//! `fallout/misc/saveload/bgssaveloadmanager.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The save/load manager keeps the list of save files found on disk, the
//! number of the next manual save, the name of the most recent save, and
//! drives saving and loading: it opens (or simulates) the save file, hands it
//! to the `BGSSaveLoadGame` object (`[0x011ddf38]`), and reports problems
//! through message boxes and on-screen notices.
//!
//! Translation notes
//!
//! - This build is not optimized: every function keeps `this` in a stack
//!   slot, so the decompiler's argument lists are unreliable; the arguments
//!   below were read from the disassembly and checked against each callee's
//!   `RET n`. A callee entered without a `this` set up (a static function)
//!   gets no leading word.
//! - The stack-protector cookie check (`00ec408c`) and the C++ exception
//!   frames of `00851330` and `00851680` are not translated.
//! - Session 1 covers the first 40 functions in address order, `008467e0` to
//!   `00851bf0`. Session 2 covers the remaining 18, `00851cb0` to `00852080`
//!   (the unit is complete).
//! - Shared helpers and constants at the top are `pub(crate)` where a later
//!   part file would need them.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSStringT;

// ---- Globals -------------------------------------------------------------

/// `BGSSaveLoadManager` singleton pointer.
pub(crate) const MANAGER_INSTANCE: u32 = 0x011d_e134;
/// The `BGSSaveLoadGame` object pointer (the object that writes and reads
/// the save file).
pub(crate) const SAVE_LOAD_GAME: u32 = 0x011d_df38;
/// The player character pointer.
pub(crate) const PLAYER: u32 = 0x011d_ea3c;
/// `Main` pointer (`Main::UpdateNonRenderSafeAITasks`,
/// `Main::KillMenuBGTexture` run on it).
const MAIN: u32 = 0x011d_ea0c;
/// The `TES` world object pointer.
const WORLD: u32 = 0x011d_ea10;
/// The interface manager pointer (an argument to `00714d70`).
const INTERFACE_MANAGER: u32 = 0x011d_ea24;
/// The model loader pointer (its functions are in `modelloader.cpp`).
const MODEL_LOADER: u32 = 0x011c_3b3c;
/// `ExteriorCellLoader` pointer.
const EXTERIOR_CELL_LOADER: u32 = 0x011c_9618;
/// `FaderManager` pointer.
const FADER_MANAGER: u32 = 0x011d_8804;
/// The `VATS` object (its address, not a pointer to it).
const VATS: u32 = 0x011f_2250;
/// The `BSTimer` object (its address).
const TIMER: u32 = 0x011f_6394;
/// The `ProcessLists` object (its address).
const PROCESS_LISTS: u32 = 0x011e_0e80;
/// A float game setting (its value is read through `00403e20`) that the
/// post-load code uses as the fade time.
const FADE_SETTING: u32 = 0x011c_3e7c;
/// Pointer to the open list-box menu (null when none); its list sits at
/// menu +0x40.
const LIST_MENU: u32 = 0x011d_a4f0;
/// An integer game setting (its address); `CloseSaveFile` uses its value as
/// the number of `.bak` generations to keep.
const BACKUP_COUNT_SETTING: u32 = 0x011d_e2c8;
/// Pointer to the I/O manager the save routines pause and resume.
const IO_MANAGER: u32 = 0x0120_2d98;
/// An object whose first byte, read through `00408d60`, `LoadGame` tests
/// (when it is zero the load is made quiet).
const QUIET_LOAD_SETTING: u32 = 0x011d_e248;
/// What the "load failed, try again?" message box needs to retry: the save
/// name (260 bytes), its device id and its statistics flag.
const RETRY_NAME: u32 = 0x011d_e138;
const RETRY_DEVICE_ID: u32 = 0x011a_224c;
const RETRY_FLAG: u32 = 0x011d_e23c;

// ---- Settings and strings --------------------------------------------------

/// `Setting` value getter: `this == 0 ? 0 : *(this + 4)` (the text or number
/// of a game setting). `004c69f0` is a second copy of the same code.
const SETTING_VALUE: u32 = 0x0040_3df0;
const SETTING_VALUE_COPY: u32 = 0x004c_69f0;
/// `00403e20`: the same for a setting whose value is a `float`; returns a
/// pointer to the value. `0043d4d0` does that for the integer setting.
const SETTING_FLOAT: u32 = 0x0040_3e20;
const SETTING_INT: u32 = 0x0043_d4d0;
/// Message texts (addresses of `Setting` objects).
const MESSAGE_SETTING_CANNOT_SAVE: u32 = 0x011d_2364;
const MESSAGE_SETTING_SAVING: u32 = 0x011d_4930;
const MESSAGE_SETTING_SAVE_FAILED: u32 = 0x011d_4f0c;
const MESSAGE_SETTING_QUICKSAVING: u32 = 0x011d_3600;
const MESSAGE_SETTING_QUICKLOADING: u32 = 0x011d_35ac;
/// Text of the message box that opens when a save cannot be loaded, and its
/// two buttons.
const MESSAGE_SETTING_LOAD_FAILED: u32 = 0x011d_2100;
const MESSAGE_SETTING_BUTTON_YES: u32 = 0x011d_34f8;
const MESSAGE_SETTING_BUTTON_NO: u32 = 0x011d_3684;
/// The button of the version error boxes.
const MESSAGE_SETTING_BUTTON_OK: u32 = 0x011d_38b8;
/// Text for a save without a readable version.
const MESSAGE_SETTING_CORRUPT_SAVE: u32 = 0x011d_3d0c;
/// The two words a generated save name can start with.
const NAME_PREFIX_ALTERNATE: u32 = 0x011d_3ed4;
const NAME_PREFIX_GENERATED: u32 = 0x011d_4060;

/// `"Interface\Icons\Message Icons\glow_message_vaultboy_sad.dds"`.
const ICON_SAD: u32 = 0x0102_08a0;
/// `"Interface\Icons\Message Icons\glow_message_vaultboy_neutral.dds"`.
const ICON_NEUTRAL: u32 = 0x0102_08e0;
/// `2.0f`: how long an on-screen notice stays.
const NOTICE_SECONDS: u32 = 0x0101_62c0;

/// `".bak"`.
const BACKUP_EXTENSION: u32 = 0x0107_facc;
/// `".fos"`.
const SAVE_EXTENSION: u32 = 0x0107_6e74;
/// `"*"`.
const WILDCARD: u32 = 0x0101_ffbc;
/// `" "`.
const SPACE: u32 = 0x0102_0770;
/// `""` (the first byte of an empty string constant).
const EMPTY_STRING: u32 = 0x0101_1584;
/// `"quicksave"`.
const QUICKSAVE_NAME: u32 = 0x0107_fad4;
/// `"autosave"`.
const AUTOSAVE_NAME: u32 = 0x0107_fae0;
/// `"systemsave"`.
const SYSTEMSAVE_NAME: u32 = 0x0107_faec;
/// `"The savegame cannot be loaded because it comes from a newer
/// executable.  Its version is %i and the current version is %i."`.
const NEWER_VERSION_MESSAGE: u32 = 0x0107_faf8;
/// `"The savegame cannot be loaded because it is outdated.  Its version is
/// %i and the current version is %i."`.
const OUTDATED_VERSION_MESSAGE: u32 = 0x0107_fb78;
/// `"%s - %s, EXE Version: %s, Save Version %i.%i"`.
const VERSION_INFO_FORMAT: u32 = 0x0107_fbe0;
/// `"%d/%d/%02d %02d:%02d:%02d"`.
const LOCAL_TIME_FORMAT: u32 = 0x0107_fc10;
/// `"%s %i - %s, %s, %s"`.
const SAVE_NAME_FORMAT: u32 = 0x0107_fc2c;
/// The characters a file name may not contain (`"\t\\/:*<>?|\"+=@^[]`;"`).
const FORBIDDEN_NAME_CHARACTERS: u32 = 0x0107_fc40;
/// `"%02i.%02i.%02i"` and `"%03i.%02i.%02i"` (hours, minutes, seconds).
const PLAY_TIME_FORMAT: u32 = 0x0107_fc54;
const PLAY_TIME_FORMAT_LONG: u32 = 0x0107_fc64;
/// `"%s %s"`.
const TWO_STRINGS_FORMAT: u32 = 0x0101_2058;
/// `"1.4.0.525"`.
const EXE_VERSION: u32 = 0x0107_6e68;

/// The save version the game writes (`0x30`); loads accept `0x2f` and `0x30`.
const SAVE_VERSION: i32 = 0x30;
const OLDEST_LOADABLE_VERSION: i32 = 0x2f;
/// The minor version `00851110` returns.
const SAVE_MINOR_VERSION: u32 = 0x1b;
/// A system save name is cut at this many characters.
const SYSTEM_NAME_LIMIT: u32 = 0x1b;
/// Size of every path and name buffer.
const PATH_SIZE: u32 = 0x104;

// ---- Callees outside this file -------------------------------------------

/// `strcpy_s` and `strcat_s` wrappers `(destination, size, source)`,
/// `strlen`, `sprintf_s` `(buffer, size, format, ...)`, allocation, free.
const STRING_COPY: u32 = 0x0040_6d30;
const STRING_APPEND: u32 = 0x0040_6d50;
const STRING_LENGTH: u32 = 0x0044_a670;
const FORMAT: u32 = 0x0040_6d00;
const ALLOCATE: u32 = 0x0040_1000;
const FREE: u32 = 0x0040_1030;
/// String comparison (0 when equal).
const STRING_COMPARE: u32 = 0x0040_8b20;
const STRING_COMPARE_IGNORE_CASE: u32 = 0x0040_4dc0;
/// Compares the first `n` characters, `(a, b, n)` (0 when equal).
const STRING_COMPARE_PREFIX: u32 = 0x0045_64f0;
const STRING_FIND_CHARACTER: u32 = 0x00ec_7690;
const MEMORY_SET: u32 = 0x00ec_61c0;
const FILE_ACCESS: u32 = 0x00ec_bf05;
const FILE_RENAME: u32 = 0x00ec_862c;
/// Returns its `this` unchanged.
const THIS_IDENTITY: u32 = 0x0068_15c0;
/// `BSString` constructor, destructor, assignment `(this, text, 0)`, length
/// and character-data accessors.
const BSSTRING_CONSTRUCT: u32 = 0x0040_37b0;
const BSSTRING_DESTRUCT: u32 = 0x0040_37d0;
const BSSTRING_SET: u32 = 0x0040_37f0;
const BSSTRING_LENGTH: u32 = 0x0040_48e0;
const BSSTRING_DATA: u32 = 0x0055_9450;
/// Windows imports, by their import slot.
const DELETE_FILE: u32 = 0x00fd_f0d4;
const GET_USER_NAME: u32 = 0x00fd_f014;
const GET_LOCAL_TIME: u32 = 0x00fd_f0d8;
const FIND_FIRST_FILE: u32 = 0x00fd_f070;
const FIND_NEXT_FILE: u32 = 0x00fd_f068;
const FIND_CLOSE: u32 = 0x00fd_f064;

/// `XContentClose` (Xbox PDB), `(this, name, out_path, add_suffix)`: builds
/// the path of a save file (with a 4-character suffix when asked).
const CONTENT_CLOSE: u32 = 0x0084_ff90;
/// Folder of the saves, `(this, out_path)`.
const SAVE_FOLDER_PATH: u32 = 0x0084_ff30;
/// Opens or simulates the save file, `(this, name, for_save, mode, device)`.
const OPEN_SAVE_FILE: u32 = 0x0085_0030;
/// `BGSSaveLoadFile` destructor body, and its "is open" test.
const SAVE_FILE_DESTRUCT: u32 = 0x0084_62c0;
const SAVE_FILE_IS_OPEN: u32 = 0x0084_63c0;
/// `BGSSaveLoadFileEntry`: constructor `(this, name, device_id)`,
/// destructor body, loader, three tests (the middle one is
/// `BGSSaveLoadFileEntry::IsAutosave`), and the save game number.
const FILE_ENTRY_CONSTRUCT: u32 = 0x0084_65b0;
const FILE_ENTRY_DESTRUCT: u32 = 0x0084_6690;
const FILE_ENTRY_LOAD: u32 = 0x0084_6a20;
const FILE_ENTRY_TEST_A: u32 = 0x0084_6ad0;
const FILE_ENTRY_IS_AUTOSAVE: u32 = 0x0084_6af0;
const FILE_ENTRY_TEST_B: u32 = 0x0084_6b10;
const FILE_ENTRY_SAVE_NUMBER: u32 = 0x007d_6950;
/// The comparison function that orders the entries.
const FILE_ENTRY_COMPARE: u32 = 0x0084_fbe0;
/// List helpers: sort, next node, clear, delete, construct, remove, add at
/// the head, count.
const LIST_SORT: u32 = 0x0083_fd60;
const LIST_NEXT: u32 = 0x0072_6070;
const LIST_CLEAR: u32 = 0x0047_0470;
const LIST_DELETE: u32 = 0x0047_02f0;
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_REMOVE: u32 = 0x0090_5330;
const LIST_ADD_HEAD: u32 = 0x005a_e3d0;
const LIST_COUNT: u32 = 0x005a_e380;
/// `BGSSaveLoadGame`: save `(this, file)`, load `(this, file, quiet)`, clean
/// up expired data, set or clear the flag mask 4 of `[+0x244]` `(this, on)`,
/// test that mask.
const GAME_SAVE: u32 = 0x0084_7850;
const GAME_LOAD: u32 = 0x0084_7df0;
const GAME_CLEANUP_EXPIRED_DATA: u32 = 0x0084_ab20;
const GAME_SET_FLAG: u32 = 0x0084_7d00;
const GAME_FLAG_IS_SET: u32 = 0x0056_21d0;
/// The history object inside the game object (+0x1c), its size, and
/// `BGSSaveLoadHistory::Rewind`.
const GAME_HISTORY: u32 = 0x0044_1110;
const HISTORY_SIZE: u32 = 0x0062_0b80;
const HISTORY_REWIND: u32 = 0x0084_e250;
/// `BGSSaveLoadHeader::SaveHeader(file)` and `LoadHeader(file)` (returns the
/// save version).
const SAVE_HEADER: u32 = 0x0084_d4b0;
const LOAD_HEADER: u32 = 0x0084_d8c0;
/// Writes the statistics text file of a save and opens it.
const OUTPUT_STATISTICS: u32 = 0x0085_3a30;
/// On-screen notice `(text, 0, icon, 0, seconds, flag)` (cdecl).
const SHOW_NOTICE: u32 = 0x0070_52f0;
/// Message box `(text, 0, 0, callback, 1, 0x17, 0.0, 0.0, button.., 0)`
/// (cdecl; the buttons are a null terminated list).
const SHOW_MESSAGE_BOX: u32 = 0x0070_3e80;
const MESSAGE_BOX_RESULT: u32 = 0x0070_3fa0;
const CONSOLE_IS_VISIBLE: u32 = 0x0070_3d50;
const CONSOLE_OPEN: u32 = 0x0070_3e10;
const CONSOLE_REFRESH: u32 = 0x0071_4d70;
const INTERFACE_GET: u32 = 0x004b_7210;
const KILL_MENU_BG_TEXTURE: u32 = 0x0087_7430;
const MENU_COUNT: u32 = 0x0070_2440;
const MENU_ID_AT: u32 = 0x0070_2400;
const LIST_MENU_COUNT: u32 = 0x0076_5340;
/// Player tests: a state getter (1 and 2 forbid saving), a "cannot save"
/// test.
const PLAYER_STATE: u32 = 0x004f_8960;
const PLAYER_CANNOT_SAVE: u32 = 0x0093_a740;
const VATS_ACTIVE: u32 = 0x0052_5430;
const PLAYER_NAME: u32 = 0x0055_d520;
const PLAYER_LOCATION_NAME: u32 = 0x0057_8870;
/// The player's play time in milliseconds.
const PLAY_TIME_MS: u32 = 0x0085_1cb0;
/// Elapsed seconds of the last frame (`float` in ST0).
const FRAME_SECONDS: u32 = 0x0045_3850;
const SKY_UPDATE: u32 = 0x0063_ac70;
/// Returns the object `Sky::Update` runs on, for the world object.
const SKY_OWNER: u32 = 0x008d_8520;
/// The singleton accessors of the path manager and the audio system.
const PATH_MANAGER: u32 = 0x0047_d0b0;
const AUDIO: u32 = 0x0045_3a70;

// ---- Callees of the second part (`00851cb0` onwards) -----------------------

/// `GetTickCount` through `00457fe0` (a wrapper around the import).
const TICK_COUNT: u32 = 0x0045_7fe0;
/// Fader test `(this, kind)` (kinds 1 and 2 block a queued save) and
/// `Interface::IsInGameLoadingMenuOpen`.
const FADER_ACTIVE: u32 = 0x0070_1450;
const LOADING_MENU_OPEN: u32 = 0x0070_5ea0;
/// `BGSSaveLoadFileEntry::LoadData` (Xbox PDB): reads the entry's header.
const FILE_ENTRY_LOAD_DATA: u32 = 0x0084_6900;
/// Returns the static object that `00851f50` hands back.
const GAME_DATA_UTILITY: u32 = 0x00af_2e50;
/// `BSSystemUtility::QInstance` (Xbox PDB), and
/// `BSMsgDialogSystemUtility::QInstance` (Xbox PDB) called on its result.
const SYSTEM_UTILITY_INSTANCE: u32 = 0x00af_2640;
const MSG_DIALOG_INSTANCE: u32 = 0x0085_26c0;
/// Directory walker: construct `(this, path)`, next `(this, &finished)`,
/// destruct, and the "is a plain file" test on the current entry.
const DIRECTORY_CONSTRUCT: u32 = 0x00b0_0820;
const DIRECTORY_NEXT: u32 = 0x00b0_0720;
const DIRECTORY_DESTRUCT: u32 = 0x00b0_0860;
const DIRECTORY_IS_FILE: u32 = 0x0085_2700;
/// The current entry's file name sits at `walker + 0x30` (`00436aa0`).
const DIRECTORY_FILE_NAME: u32 = 0x0043_6aa0;
/// A file object: construct `(this, path, 0, 0, 0)`, destruct, the open
/// result (its first word, 0 when the open worked), `BSSystemFile::DoGetSize`
/// (Xbox PDB) `(this, &size)` and `BSSystemFile::DoRead` (Xbox PDB)
/// `(this, buffer, size_low, size_high, &bytes_read)`.
const FILE_CONSTRUCT: u32 = 0x00b0_0900;
const FILE_DESTRUCT: u32 = 0x00b0_0950;
const FILE_OPEN_RESULT: u32 = 0x0055_9450;
const FILE_GET_SIZE: u32 = 0x0065_c290;
const FILE_READ: u32 = 0x0085_26d0;
/// `BGSSaveLoadFile::BGSSaveLoadFile` (Xbox PDB) `(this, name, 0, size)`.
const SAVE_LOAD_FILE_CONSTRUCT: u32 = 0x0084_61e0;
/// The save-data buffer object of a `BGSSaveLoadFile` (its word at +0x104),
/// and that object's data pointer (its word at +0x15c).
const SAVE_FILE_BUFFER: u32 = 0x0068_efb0;
const BUFFER_DATA: u32 = 0x008d_85e0;
/// `(save file, entry)` (cdecl): fills a list entry from the file.
const FILE_ENTRY_FILL: u32 = 0x0084_dab0;
/// Sets the byte at entry +4.
const FILE_ENTRY_MARK: u32 = 0x006e_a330;
/// Entry getters that run `LoadData` first: +0x14, +0x18, +0x10, +0x0c, +0x1c.
const ENTRY_LOCATION: u32 = 0x007d_6970;
const ENTRY_PLAYER_NAME: u32 = 0x007d_5180;
const ENTRY_LEVEL_NAME: u32 = 0x007d_5140;
const ENTRY_CELL_NAME: u32 = 0x0075_f8c0;
const ENTRY_LEVEL: u32 = 0x007d_5160;
/// The three entry getters of this file and the buffer setters.
const ENTRY_DATA_A: u32 = 0x0085_1ef0;
const ENTRY_DATA_B: u32 = 0x0085_1f10;
const ENTRY_DATA_C: u32 = 0x0085_1f30;
const BUFFER_SET_TITLE: u32 = 0x0085_1f60;
const BUFFER_SET_LOCATION: u32 = 0x0085_1f90;
const BUFFER_SET_TEXT: u32 = 0x0085_1fc0;
const BUFFER_SET_SIZES: u32 = 0x0085_1ff0;
const BUFFER_SET_IMAGE: u32 = 0x0085_2030;
const BUFFER_SET_CALLBACK: u32 = 0x0085_2060;
/// The routine the buffer calls when the copy is done.
const COPY_DONE_CALLBACK: u32 = 0x0085_2740;
/// Sets the word at +0x20 (here the manager's open save file).
const SET_WORD_AT_20: u32 = 0x0050_f9c0;
/// Reads the byte at +4 of the game-data utility: 1 while a save-data
/// operation runs.
const GAME_DATA_BUSY: u32 = 0x004f_1540;
/// `Sleep` wrapper `(milliseconds)` (cdecl).
const SLEEP: u32 = 0x0040_fca0;
/// Debug print `(format, ...)` (cdecl).
const DEBUG_PRINT: u32 = 0x0084_cbd0;
/// The object `CopySaveGames` quiets: its data byte is saved, cleared and
/// restored (`00408d60` returns the byte's address, `004de2d0` sets it).
const COPY_GUARD_OBJECT: u32 = 0x011d_e308;
const GUARD_DATA: u32 = 0x0040_8d60;
const GUARD_SET: u32 = 0x004d_e2d0;
/// Settings read for the description text.
const SETTING_PLAYER_NAME: u32 = 0x011d_3954;
const SETTING_CELL_NAME: u32 = 0x011d_4f78;
const SETTING_LEVEL_LABEL: u32 = 0x011d_3b2c;
/// Texts: `"Saves"`, `"/"`, `"%s%s"`, `"SaveData"`, `"-SAVE-"`,
/// `"Location"`, `"-"`, the description format `"%s\n%s %d: %s\n%s: %s"`,
/// the image path, the progress text and the two failure texts.
const COPY_FOLDER: u32 = 0x0107_fd90;
const COPY_SLASH: u32 = 0x0106_3cbc;
const COPY_PATH_FORMAT: u32 = 0x0101_996c;
const COPY_BUFFER_NAME: u32 = 0x0107_fca4;
const COPY_TITLE: u32 = 0x0107_fabc;
const COPY_LOCATION: u32 = 0x0104_47cc;
const COPY_DASH: u32 = 0x0103_45e0;
const COPY_TEXT_FORMAT: u32 = 0x0107_fc90;
const COPY_IMAGE: u32 = 0x0107_fc74;
const COPY_PROGRESS: u32 = 0x0107_fd60;
const COPY_READ_FAILED: u32 = 0x0107_fd08;
const COPY_OPEN_FAILED: u32 = 0x0107_fcb0;
/// Sizes: the directory walker, and the file object.
const DIRECTORY_SIZE: u32 = 0x258;
const FILE_OBJECT_SIZE: u32 = 0x20;
/// Play-time fields of the player object (read through `00851cb0`): the tick
/// count at the last mark and the accumulated milliseconds.
const PLAY_TIME_MARK: u32 = 0x78c;
const PLAY_TIME_ACCUMULATED: u32 = 0x790;

// ---- Layouts -------------------------------------------------------------

layout! {
    /// `BGSSaveLoadManager` (Xbox PDB), 0x30 bytes on the Xbox. The PC build
    /// adds a `BSString` at +0x30 and a byte at +0x11; the size here is the
    /// extent this unit uses, not a confirmed allocation size.
    pub struct BGSSaveLoadManager: 0x38 {
        /// `pSaveGameList` (Xbox PDB): `BSSimpleList<BGSSaveLoadFileEntry *>*`.
        0x00 pSaveGameList: Ptr,
        /// `iSaveGameCount` (Xbox PDB).
        0x04 iSaveGameCount: u32,
        /// `iCurrentSaveGameNumber` (Xbox PDB).
        0x08 iCurrentSaveGameNumber: u32,
        /// `bSimulatedMode` (Xbox PDB).
        0x0C bSimulatedMode: bool,
        /// `bAutosaveDisabledForDiskspace` (Xbox PDB).
        0x0D bAutosaveDisabledForDiskspace: bool,
        /// `bhandledCorrupt` (Xbox PDB).
        0x0E bhandledCorrupt: bool,
        /// `cQueuedAutosave` (Xbox PDB).
        0x0F cQueuedAutosave: u8,
        /// `cQueuedForceSave` (Xbox PDB).
        0x10 cQueuedForceSave: u8,
        /// PC only (the Xbox PDB has padding here): cleared by the
        /// system-save routine `00850a90`, as `cQueuedAutosave` is by the
        /// autosave routine.
        0x11 cQueuedSystemSave: u8,
        /// `pMostRecentSaveGame` (Xbox PDB): `char*`, a heap copy.
        0x14 pMostRecentSaveGame: Ptr,
        /// `iMostRecentSaveGameDeviceID` (Xbox PDB).
        0x18 iMostRecentSaveGameDeviceID: i32,
        /// `pSaveLoadFile` (Xbox PDB): `BGSSaveLoadFile*` kept open for a
        /// simulated save.
        0x20 pSaveLoadFile: Ptr,
        /// `bStartMenuLoading` (Xbox PDB).
        0x26 bStartMenuLoading: bool,
        /// PC only: the system save's name (`BSString`).
        0x30 SystemSaveName: Inline<BSStringT>,
    }
}

// `BGSSaveLoadFileEntry` (Xbox PDB; its own unit is `bgssaveloadfile.cpp`),
// read at its offsets: `pFileName` +0x00, `iDeviceID` +0x34; 0x7c bytes.
const FILE_ENTRY_FILE_NAME: u32 = 0x00;
const FILE_ENTRY_DEVICE_ID: u32 = 0x34;
const FILE_ENTRY_SIZE: u32 = 0x7c;
/// `WIN32_FIND_DATAA`: size, `nFileSizeLow` +0x20, `cFileName` +0x2c.
const FIND_DATA_SIZE: u32 = 0x140;
const FIND_DATA_FILE_SIZE_LOW: u32 = 0x20;
const FIND_DATA_FILE_NAME: u32 = 0x2c;

// ---- Small helpers -------------------------------------------------------

fn copy_string(e: &mut Engine, destination: u32, size: u32, source: u32) {
    e.call(STRING_COPY, &args![destination, size, source]);
}

fn append_string(e: &mut Engine, destination: u32, size: u32, source: u32) {
    e.call(STRING_APPEND, &args![destination, size, source]);
}

fn string_length(e: &mut Engine, text: u32) -> u32 {
    e.call(STRING_LENGTH, &args![text]).u32()
}

/// Shows an on-screen notice: the text of `setting` (read with `getter`, one
/// of the two copies of the setting getter), the icon and the final flag.
fn show_notice(e: &mut Engine, getter: u32, setting: u32, icon: u32, flag: u32) {
    let seconds: f32 = e.global(NOTICE_SECONDS);
    let text = e.call(getter, &args![setting]).u32();
    e.call(SHOW_NOTICE, &args![text, 0u32, icon, 0u32, seconds, flag]);
}

/// Opens a message box with `text` and a single button (the version errors).
fn show_error_box(e: &mut Engine, text: u32) {
    let ok = e
        .call(SETTING_VALUE, &args![MESSAGE_SETTING_BUTTON_OK])
        .u32();
    e.call(
        SHOW_MESSAGE_BOX,
        &args![text, 0u32, 0u32, 0u32, 0u32, 0x17u32, 0.0f32, 0.0f32, ok, 0u32],
    );
}

// Translated from 008467e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::RemoveSaveGame` (Xbox PDB): deletes the save file of
/// a list entry and takes the entry out of the manager's list.
///
/// The body treats `this` (ECX) as the `BGSSaveLoadFileEntry`: it reads
/// `pFileName` (+0) and `iDeviceID` (+0x34), and passes the singleton
/// manager (`[0x011de134]`) as `this` to `00850360` and `00851620`.
pub fn bgssaveloadmanager_remove_save_game(e: &mut Engine, entry: Ptr) {
    let manager = Ptr::new(e.global::<u32>(MANAGER_INSTANCE));
    let file_name = Ptr::new(e.mem.u32(entry.addr() + FILE_ENTRY_FILE_NAME));
    let device_id = e.mem.u32(entry.addr() + FILE_ENTRY_DEVICE_ID);
    fn_00850360(e, manager, file_name, device_id);
    fn_00851620(e, manager.cast(), entry);
}

// Translated from 00847dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The engine map names this `BGSSaveLoadManager::GetMinorVersion`, but the
/// body only runs `BGSSaveGameBuffer::~BGSSaveGameBuffer` (`00865c10`) on its
/// `this` (identical-code folding).
pub fn fn_00847dd0(e: &mut Engine, this: Ptr) {
    e.call(0x0086_5c10, &args![this]);
}

// Translated from 00850100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::CloseSaveFile` (Xbox PDB): deletes `file` (a
/// `BGSSaveLoadFile`, whose first member is its path) and, when `rotate` is
/// set, moves the temporary file it wrote into place: `XContentClose` gives
/// the temporary name, the final name is that without its 4-character
/// suffix, the older copies are shifted to `.bak` names first (as many
/// generations as the integer setting at `0x011de2c8` says), and the
/// temporary file is renamed to the final name.
pub fn bgssaveloadmanager_close_save_file(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadManager>,
    file: Ptr,
    rotate: bool,
) {
    e.with_stack(5 * PATH_SIZE, |e, base| {
        let file_name = base.addr();
        let temp_name = file_name + PATH_SIZE;
        let final_name = temp_name + PATH_SIZE;
        let older = final_name + PATH_SIZE;
        let newer = older + PATH_SIZE;

        let path = e.call(THIS_IDENTITY, &args![file]).u32();
        copy_string(e, file_name, PATH_SIZE, path);
        if !file.is_null() {
            fn_00850330(e, file, 1);
        }
        if rotate {
            e.call(CONTENT_CLOSE, &args![this, file_name, temp_name, rotate]);
            copy_string(e, final_name, PATH_SIZE, temp_name);
            let length = string_length(e, final_name);
            e.mem
                .set_u8(final_name.wrapping_add(length).wrapping_sub(4), 0);
            let counter = e.call(SETTING_INT, &args![BACKUP_COUNT_SETTING]).u32();
            let mut generation = e.mem.i32(counter).wrapping_sub(1);
            while generation >= 0 {
                copy_string(e, older, PATH_SIZE, final_name);
                for _ in 0..generation {
                    append_string(e, older, PATH_SIZE, BACKUP_EXTENSION);
                }
                copy_string(e, newer, PATH_SIZE, older);
                append_string(e, newer, PATH_SIZE, BACKUP_EXTENSION);
                if e.call(FILE_ACCESS, &args![older, 0u32]).i32() != -1 {
                    if e.call(FILE_ACCESS, &args![newer, 0u32]).i32() != -1 {
                        e.call(DELETE_FILE, &args![newer]);
                    }
                    e.call(FILE_RENAME, &args![older, newer]);
                }
                generation -= 1;
            }
            e.call(FILE_RENAME, &args![temp_name, final_name]);
            copy_string(e, temp_name, PATH_SIZE, final_name);
        }
    });
}

// Translated from 00850330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFile` scalar deleting destructor: runs the destructor body
/// (`008462c0`), and frees the object when bit 0 of `flags` is set. Returns
/// `this`.
pub fn fn_00850330(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(SAVE_FILE_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00850360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the save file called `name` from disk: builds its path with
/// `XContentClose` and calls `DeleteFileA`. The second stack word is never
/// read.
pub fn fn_00850360(e: &mut Engine, this: Ptr, name: Ptr, _unused_1: u32) {
    e.with_stack(PATH_SIZE, |e, path| {
        e.call(CONTENT_CLOSE, &args![this, name, path, 0u32]);
        e.call(DELETE_FILE, &args![path]);
    });
}

// Translated from 008503b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::SaveGame` (Xbox PDB): saves the game under `name`
/// (a generated name when null). Returns whether a save was written.
///
/// Autosaves, forced saves and system saves report nothing when they cannot
/// run. A failed save raises the sad-face notice and leaves the counter
/// alone; a good one rotates the files (`CloseSaveFile`), trims the history
/// back to its size at the start, adds the entry to the list, counts a
/// manual save, and remembers the name. `dump_statistics` also writes and
/// opens the statistics file.
pub fn bgssaveloadmanager_save_game(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadManager>,
    name: Ptr,
    device_id: u32,
    dump_statistics: bool,
) -> bool {
    let is_autosave = fn_00851a20(e, this, name);
    let is_quicksave = fn_00851a60(e, this, name);
    let is_system_save = fn_00851b00(e, this, name);
    let quiet =
        is_autosave || e.get(this, BGSSaveLoadManager::cQueuedForceSave) != 0 || is_system_save;

    if !fn_00850fe0(e, this, is_autosave) {
        if !quiet {
            show_notice(e, SETTING_VALUE, MESSAGE_SETTING_CANNOT_SAVE, ICON_SAD, 0);
        }
        return false;
    }

    if e.get(this, BGSSaveLoadManager::iCurrentSaveGameNumber) == 0 {
        fn_00851330(e, this);
        fn_00851540(e, this);
    }

    e.with_stack(3 * PATH_SIZE + 0x40, |e, base| {
        let generated = base.addr();
        let name_copy = generated + PATH_SIZE + 0x10;
        let statistics = name_copy + PATH_SIZE + 0x10;

        let mut name = name;
        if name.is_null() {
            bgssaveloadmanager_generate_save_file_name(e, this, Ptr::new(generated), false, false);
            name = Ptr::new(generated);
        }
        copy_string(e, name_copy, PATH_SIZE, name.addr());
        let game = e.global::<u32>(SAVE_LOAD_GAME);
        e.call(GAME_CLEANUP_EXPIRED_DATA, &args![game]);
        let history = e.call(GAME_HISTORY, &args![game]).u32();
        let history_size = e.call(HISTORY_SIZE, &args![history]).u32();
        fn_00850ba0(e, this);

        let open_file = e.get(this, BGSSaveLoadManager::pSaveLoadFile);
        let file = if !open_file.is_null() {
            open_file
        } else {
            e.call(OPEN_SAVE_FILE, &args![this, name, 1u32, 2u32, device_id])
                .ptr::<()>()
        };
        if file.is_null() {
            fn_00850bf0(e, this);
            return false;
        }

        if quiet {
            e.set(
                this,
                BGSSaveLoadManager::bAutosaveDisabledForDiskspace,
                false,
            );
            show_notice(
                e,
                SETTING_VALUE_COPY,
                MESSAGE_SETTING_SAVING,
                ICON_NEUTRAL,
                1,
            );
        }
        fn_00850ea0(e, this, file);
        e.call(GAME_SAVE, &args![game, file]);
        if e.call(GAME_FLAG_IS_SET, &args![game]).bool() {
            e.call(GAME_SET_FLAG, &args![game, 0u32]);
            show_notice(
                e,
                SETTING_VALUE_COPY,
                MESSAGE_SETTING_SAVE_FAILED,
                ICON_SAD,
                0,
            );
            fn_00850bf0(e, this);
            return false;
        }
        fn_00850bf0(e, this);

        let open_file = e.get(this, BGSSaveLoadManager::pSaveLoadFile);
        if file != open_file {
            bgssaveloadmanager_close_save_file(e, this, file, true);
        }
        if dump_statistics {
            e.call(CONTENT_CLOSE, &args![this, name, statistics, 0u32]);
            e.call(OUTPUT_STATISTICS, &args![statistics]);
        }
        let history = e.call(GAME_HISTORY, &args![game]).u32();
        e.call(HISTORY_REWIND, &args![history, history_size]);

        let open_file = e.get(this, BGSSaveLoadManager::pSaveLoadFile);
        if file != open_file && !e.get(this, BGSSaveLoadManager::pSaveGameList).is_null() {
            let number = e.get(this, BGSSaveLoadManager::iCurrentSaveGameNumber);
            fn_00851680(e, this, Ptr::new(name_copy), device_id, number);
        }
        if !is_quicksave && !is_autosave {
            let number = e.get(this, BGSSaveLoadManager::iCurrentSaveGameNumber);
            e.set(
                this,
                BGSSaveLoadManager::iCurrentSaveGameNumber,
                number.wrapping_add(1),
            );
        }
        fn_00851250(e, this, Ptr::new(name_copy), device_id as i32);
        true
    })
}

// Translated from 00850720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Callback of the "load failed, try again?" message box: when the player
/// answered yes (result 1), loads the remembered save again.
pub fn fn_00850720(e: &mut Engine) {
    let answer = e.call(MESSAGE_BOX_RESULT, &args![]).u8() as i8;
    if answer == 1 {
        let manager: Ptr<BGSSaveLoadManager> = Ptr::new(e.global::<u32>(MANAGER_INSTANCE));
        let device_id: u32 = e.global(RETRY_DEVICE_ID);
        let flag: u8 = e.global(RETRY_FLAG);
        fn_00850760(e, manager, Ptr::new(RETRY_NAME), device_id, flag != 0, 1);
    }
}

// Translated from 00850760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Loads the save called `name` (the manager's `LoadGame`). Returns whether
/// it loaded.
///
/// Opens the file (or uses the open simulated one), checks the header, and
/// lets `BGSSaveLoadGame::LoadGame` read it. On success the world is brought
/// back up (`00850d60`). On failure the name is kept for the retry message
/// box. `do_stats` writes and opens the statistics file after a good load.
/// `quiet_load` is forced to 1 when the byte behind `0x011de248` is zero.
pub fn fn_00850760(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadManager>,
    name: Ptr,
    device_id: u32,
    do_stats: bool,
    quiet_load: u8,
) -> bool {
    e.with_stack(2 * PATH_SIZE + 0x10, |e, base| {
        let name_copy = base.addr();
        let statistics = name_copy + PATH_SIZE + 0x10;
        let mut quiet_load = quiet_load;
        if !name.is_null() {
            copy_string(e, name_copy, PATH_SIZE, name.addr());
        }
        let open_file = e.get(this, BGSSaveLoadManager::pSaveLoadFile);
        let file = if !open_file.is_null() {
            open_file
        } else {
            e.call(OPEN_SAVE_FILE, &args![this, name, 0u32, 1u32, device_id])
                .ptr::<()>()
        };
        let mut loaded = false;
        if !file.is_null() {
            let console_was_visible = e.call(CONSOLE_IS_VISIBLE, &args![]).bool();
            let setting = e.call(0x0040_8d60, &args![QUIET_LOAD_SETTING]).u32();
            if e.mem.u8(setting) == 0 {
                quiet_load = 1;
            }
            if e.call(SAVE_FILE_IS_OPEN, &args![file]).bool()
                && bgssaveloadmanager_load_game_header(e, this, file)
            {
                e.set(this, BGSSaveLoadManager::bStartMenuLoading, true);
                let game = e.global::<u32>(SAVE_LOAD_GAME);
                loaded = e.call(GAME_LOAD, &args![game, file, quiet_load]).bool();
                e.set(this, BGSSaveLoadManager::bStartMenuLoading, false);
                if loaded {
                    fn_00850d60(e, this);
                } else {
                    copy_string(e, RETRY_NAME, PATH_SIZE, name.addr());
                    e.set_global(RETRY_DEVICE_ID, device_id);
                    e.set_global(RETRY_FLAG, do_stats as u8);
                    let text = e
                        .call(SETTING_VALUE, &args![MESSAGE_SETTING_LOAD_FAILED])
                        .u32();
                    let yes = e
                        .call(SETTING_VALUE, &args![MESSAGE_SETTING_BUTTON_YES])
                        .u32();
                    let no = e
                        .call(SETTING_VALUE, &args![MESSAGE_SETTING_BUTTON_NO])
                        .u32();
                    e.call(
                        SHOW_MESSAGE_BOX,
                        &args![
                            text,
                            0u32,
                            0u32,
                            0x0085_0720u32,
                            1u32,
                            0x17u32,
                            0.0f32,
                            0.0f32,
                            yes,
                            no,
                            0u32
                        ],
                    );
                }
            }
            let open_file = e.get(this, BGSSaveLoadManager::pSaveLoadFile);
            if file != open_file {
                bgssaveloadmanager_close_save_file(e, this, file, false);
            }
            if loaded && do_stats {
                e.call(CONTENT_CLOSE, &args![this, name, statistics, 0u32]);
                e.call(OUTPUT_STATISTICS, &args![statistics]);
            }
            if console_was_visible {
                e.call(CONSOLE_OPEN, &args![]);
                let interface = e.global::<u32>(INTERFACE_MANAGER);
                let manager = e.call(INTERFACE_GET, &args![]).u32();
                e.call(CONSOLE_REFRESH, &args![manager, interface]);
            }
            if loaded {
                fn_00851250(e, this, Ptr::new(name_copy), device_id as i32);
            }
        }
        let main = e.global::<u32>(MAIN);
        e.call(KILL_MENU_BG_TEXTURE, &args![main]);
        loaded
    })
}

// Translated from 008509a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Quick save: shows the "quick saving" notice and saves as `quicksave`.
pub fn fn_008509a0(e: &mut Engine, this: Ptr<BGSSaveLoadManager>) {
    show_notice(
        e,
        SETTING_VALUE,
        MESSAGE_SETTING_QUICKSAVING,
        ICON_NEUTRAL,
        0,
    );
    bgssaveloadmanager_save_game(e, this, Ptr::new(QUICKSAVE_NAME), u32::MAX, false);
}

// Translated from 008509f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Quick load: shows the "quick loading" notice and loads `quicksave`,
/// answering whether the load succeeded (the exe ends on the `00850760` call
/// and leaves its `EAX`; the caller tests `AL`).
pub fn fn_008509f0(e: &mut Engine, this: Ptr<BGSSaveLoadManager>) -> bool {
    show_notice(
        e,
        SETTING_VALUE,
        MESSAGE_SETTING_QUICKLOADING,
        ICON_NEUTRAL,
        0,
    );
    fn_00850760(e, this, Ptr::new(QUICKSAVE_NAME), u32::MAX, false, 1)
}

/// Hands the frame time (`00453850`, in ST0) to `Sky::Update`, running on the
/// object `008d8520` returns for the world object.
fn update_sky_for_save(e: &mut Engine) {
    let seconds = e.call(FRAME_SECONDS, &args![]).f32();
    let world = e.global::<u32>(WORLD);
    let owner = e.call(SKY_OWNER, &args![world]).u32();
    e.call(SKY_UPDATE, &args![owner, seconds]);
}

// Translated from 00850a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs a queued autosave: clears the autosave queue byte and the "handled
/// corrupt" flag, updates the sky for the frame time, saves as `autosave`.
pub fn fn_00850a40(e: &mut Engine, this: Ptr<BGSSaveLoadManager>) {
    e.set(this, BGSSaveLoadManager::cQueuedAutosave, 0);
    e.set(this, BGSSaveLoadManager::bhandledCorrupt, false);
    update_sky_for_save(e);
    bgssaveloadmanager_save_game(e, this, Ptr::new(AUTOSAVE_NAME), u32::MAX, false);
}

// Translated from 00850a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs a queued system save: clears its queue byte (+0x11) and the "handled
/// corrupt" flag, updates the sky, makes the system save name (stored in the
/// manager's `BSString` at +0x30 when that is still empty) and saves under
/// it.
pub fn fn_00850a90(e: &mut Engine, this: Ptr<BGSSaveLoadManager>) {
    e.set(this, BGSSaveLoadManager::cQueuedSystemSave, 0);
    e.set(this, BGSSaveLoadManager::bhandledCorrupt, false);
    update_sky_for_save(e);
    e.with_stack(PATH_SIZE, |e, name| {
        fn_00850b40(e, name);
        let stored = this.at(BGSSaveLoadManager::SystemSaveName);
        if e.call(BSSTRING_LENGTH, &args![stored]).u32() == 0 {
            e.call(BSSTRING_SET, &args![stored, name, 0u32]);
        }
        bgssaveloadmanager_save_game(e, this, name, u32::MAX, false);
    });
}

// Translated from 00850b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the system save name into `buffer`: `"systemsave <player name>"`
/// cut to 0x1b characters and scrubbed of characters a file name cannot hold
/// (`ScrubFileName` on the singleton manager).
pub fn fn_00850b40(e: &mut Engine, buffer: Ptr) {
    let player = e.global::<u32>(PLAYER);
    let player_name = e.call(PLAYER_NAME, &args![player]).u32();
    e.call(
        FORMAT,
        &args![
            buffer,
            PATH_SIZE,
            TWO_STRINGS_FORMAT,
            SYSTEMSAVE_NAME,
            player_name
        ],
    );
    if string_length(e, buffer.addr()) > SYSTEM_NAME_LIMIT {
        e.mem.set_u8(buffer.addr() + SYSTEM_NAME_LIMIT, 0);
    }
    let manager = e.global::<u32>(MANAGER_INSTANCE);
    bgssaveloadmanager_scrub_file_name(e, Ptr::new(manager), buffer);
}

// Translated from 00850ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Pauses background work before a save: asks the path manager to stop
/// (`006ebc70`), pauses the I/O manager (`00c3e310`), and then waits until the
/// path manager reports it has stopped (`006ebc90`), nudging it (`006ebc50`)
/// each time round.
pub fn fn_00850ba0(e: &mut Engine, _unused_this: Ptr<BGSSaveLoadManager>) {
    let path_manager = e.call(PATH_MANAGER, &args![]).u32();
    e.call(0x006e_bc70, &args![path_manager]);
    let io_manager = e.global::<u32>(IO_MANAGER);
    e.call(0x00c3_e310, &args![io_manager]);
    loop {
        let path_manager = e.call(PATH_MANAGER, &args![]).u32();
        if e.call(0x006e_bc90, &args![path_manager]).bool() {
            break;
        }
        let path_manager = e.call(PATH_MANAGER, &args![]).u32();
        e.call(0x006e_bc50, &args![path_manager]);
    }
}

// Translated from 00850bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Undoes `00850ba0`: resumes the I/O manager (`00c3e340`) and the path
/// manager (`006ebcd0`).
pub fn fn_00850bf0(e: &mut Engine, _unused_this: Ptr<BGSSaveLoadManager>) {
    let io_manager = e.global::<u32>(IO_MANAGER);
    e.call(0x00c3_e340, &args![io_manager]);
    let path_manager = e.call(PATH_MANAGER, &args![]).u32();
    e.call(0x006e_bcd0, &args![path_manager]);
}

// Translated from 00850c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tears the running game down before a load: clears the exterior cell
/// loader and the world's caches, silences and flushes the audio, ends VATS,
/// resets the global time multiplier to 1.0, makes the player's world update,
/// and then runs the path manager until it is idle. `this` is not used.
pub fn fn_00850c20(e: &mut Engine, _unused_this: Ptr) {
    let loader = e.global::<u32>(EXTERIOR_CELL_LOADER);
    e.call(0x0052_7fc0, &args![loader]);
    let model_loader = e.global::<u32>(MODEL_LOADER);
    e.call(0x0044_8420, &args![model_loader]);
    e.call(0x005b_4940, &args![]);
    e.call(0x005a_e270, &args![]);
    e.call(0x005a_9d60, &args![]);
    // `BSAudio::KillAllOfType(-1, 0)`.
    let audio = e.call(AUDIO, &args![]).u32();
    e.call(0x00ad_84e0, &args![audio, u32::MAX, 0u32]);
    let audio = e.call(AUDIO, &args![]).u32();
    let multithreaded = e.call(0x005b_b4d0, &args![audio]).u8();
    // `BSAudio::SetMultiThreaded(0)`, ten flushes, then the old setting back.
    let audio = e.call(AUDIO, &args![]).u32();
    e.call(0x00ad_7230, &args![audio, 0u32]);
    for _ in 0..10u32 {
        let audio = e.call(AUDIO, &args![]).u32();
        e.call(0x00ad_7740, &args![audio, 0u32]);
    }
    let audio = e.call(AUDIO, &args![]).u32();
    e.call(0x00ad_7230, &args![audio, multithreaded]);
    if e.call(VATS_ACTIVE, &args![VATS]).bool() {
        // `VATS::QuitVATSPlayback(1, 0)` and `VATS::SetMode(0, 0)`.
        e.call(0x009c_8950, &args![VATS, 1u32, 0u32]);
        e.call(0x009c_6c30, &args![VATS, 0u32, 0u32]);
    }
    // `BSTimer::SetGlobalTimeMultiplier(1.0, true)`.
    e.call(0x00aa_4db0, &args![TIMER, 1.0f32, 1u32]);
    let player = e.global::<u32>(PLAYER);
    if player != 0 {
        e.call(0x0093_e770, &args![player, 2u32, 1u32]);
    }
    let world = e.global::<u32>(WORLD);
    e.call(0x0045_7d70, &args![world, 1u32, 0u32, 0u32]);
    loop {
        let path_manager = e.call(PATH_MANAGER, &args![]).u32();
        if e.call(0x006e_bcb0, &args![path_manager]).bool() {
            break;
        }
        let path_manager = e.call(PATH_MANAGER, &args![]).u32();
        e.call(0x006e_bc50, &args![path_manager]);
        let obstacles = e.call(0x006c_0720, &args![]).u32();
        e.call(0x006c_3640, &args![obstacles]);
    }
    let path_manager = e.call(PATH_MANAGER, &args![]).u32();
    e.call(0x006e_bcf0, &args![path_manager]);
}

// Translated from 00850d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Brings the world back up after a successful load: rebuilds AI tasks and
/// caches, updates the terrain around the player, fades the screen in,
/// resumes the loading thread, and lets the path manager run again. `this`
/// is not used.
pub fn fn_00850d60(e: &mut Engine, _unused_this: Ptr<BGSSaveLoadManager>) {
    e.call(0x005f_5880, &args![]);
    let main = e.global::<u32>(MAIN);
    e.call(0x0086_f6a0, &args![main]);
    e.call(0x0086_f670, &args![main]);
    let model_loader = e.global::<u32>(MODEL_LOADER);
    e.call(0x0044_8620, &args![model_loader, 0u32]);
    let world = e.global::<u32>(WORLD);
    e.call(0x0045_9a00, &args![world]);
    e.call(0x0048_3710, &args![main]);
    let grid = e.call(0x0044_ddc0, &args![world]).u32();
    e.call(0x004b_a8d0, &args![grid]);
    e.call(0x009c_a410, &args![VATS]);

    let player = e.global::<u32>(PLAYER);
    let world_space = e.call(0x0057_5d70, &args![player]).u32();
    if world_space != 0 {
        // Virtual slot 0x1f4 of the player (`Actor::GetLocationOnReference`
        // in the player's vtable) returns a position.
        let position = e.vcall(player, 0x1f4, &args![]).u32();
        let terrain = e.call(0x0058_6170, &args![world_space]).u32();
        e.call(0x006f_ca90, &args![terrain, position, 0xfu32]);
    }
    let start_menu = e.call(0x00a2_5920, &args![]).u32();
    if start_menu != 0 {
        let start_menu = e.call(0x00a2_5920, &args![]).u32();
        e.call(0x007d_0bd0, &args![start_menu, 1u32]);
        let start_menu = e.call(0x00a2_5920, &args![]).u32();
        e.call(0x007d_0bb0, &args![start_menu]);
    }
    let fade = e.call(SETTING_FLOAT, &args![FADE_SETTING]).u32();
    let fade_seconds = e.mem.f32(fade);
    let faders = e.global::<u32>(FADER_MANAGER);
    e.call(0x0070_0960, &args![faders, 1u32, fade_seconds, 1u32]);
    e.call(0x0078_cfc0, &args![]);
    e.call(0x0045_7d70, &args![world, 0u32, 0u32, 0u32]);
    let audio = e.call(AUDIO, &args![]).u32();
    e.call(0x00ad_8740, &args![audio]);
    e.call(0x0083_0660, &args![]);
    e.call(0x0097_7660, &args![PROCESS_LISTS]);
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(0x0084_cc40, &args![game, 0u32]);
    let obstacles = e.call(0x006c_0720, &args![]).u32();
    e.call(0x006c_39c0, &args![obstacles]);
    let path_manager = e.call(PATH_MANAGER, &args![]).u32();
    e.call(0x006e_bcd0, &args![path_manager]);
}

// Translated from 00850ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the save header of `file`: sets flag mask 4 of the game object
/// (`00847d00(on)`; the save is treated as failed when it is still set
/// afterwards, see `SaveGame`) and calls `BGSSaveLoadHeader::SaveHeader`.
pub fn fn_00850ea0(e: &mut Engine, _unused_this: Ptr<BGSSaveLoadManager>, file: Ptr) {
    let game = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(GAME_SET_FLAG, &args![game, 1u32]);
    e.call(SAVE_HEADER, &args![file]);
}

// Translated from 00850ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::LoadGameHeader` (Xbox PDB): reads the header of
/// `file` and accepts versions 0x2f and 0x30. Anything else raises a message
/// box (no version for 0 or less, "outdated" below 0x2f, "newer" above 0x30)
/// and returns false.
pub fn bgssaveloadmanager_load_game_header(
    e: &mut Engine,
    _unused_this: Ptr<BGSSaveLoadManager>,
    file: Ptr,
) -> bool {
    let version = e.call(LOAD_HEADER, &args![file]).i32();
    if (OLDEST_LOADABLE_VERSION..=SAVE_VERSION).contains(&version) {
        return true;
    }
    if version <= 0 {
        let text = e
            .call(SETTING_VALUE_COPY, &args![MESSAGE_SETTING_CORRUPT_SAVE])
            .u32();
        show_error_box(e, text);
    } else {
        e.with_stack(PATH_SIZE, |e, message| {
            let format = if version < SAVE_VERSION {
                OUTDATED_VERSION_MESSAGE
            } else {
                NEWER_VERSION_MESSAGE
            };
            e.call(
                FORMAT,
                &args![message, PATH_SIZE, format, version, SAVE_VERSION],
            );
            show_error_box(e, message.addr());
        });
    }
    false
}

// Translated from 00850fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the game may save now. No when the player's state is 1 or 2 or
/// the player cannot save, or when VATS is active. Unless `is_autosave`, also
/// no when a menu other than these is open: ids 0, 3 and 0x3f5 are fine, 0x3e9
/// only while the open list-box menu has no entries.
pub fn fn_00850fe0(
    e: &mut Engine,
    _unused_this: Ptr<BGSSaveLoadManager>,
    is_autosave: bool,
) -> bool {
    let player = e.global::<u32>(PLAYER);
    if player != 0 {
        if e.call(PLAYER_STATE, &args![player]).i32() == 2
            || e.call(PLAYER_STATE, &args![player]).i32() == 1
        {
            return false;
        }
        if e.call(PLAYER_CANNOT_SAVE, &args![player]).bool() {
            return false;
        }
    }
    if e.call(VATS_ACTIVE, &args![VATS]).bool() {
        return false;
    }
    if !is_autosave {
        let count = e.call(MENU_COUNT, &args![]).u32();
        for index in 0..count {
            let menu = e.call(MENU_ID_AT, &args![index]).i32();
            if menu > 0x3e9 {
                if menu != 0x3f5 {
                    return false;
                }
            } else if menu == 0x3e9 {
                if fn_008510e0(e) > 0 {
                    return false;
                }
            } else if menu != 0 && menu != 3 {
                return false;
            }
        }
    }
    true
}

// Translated from 008510e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The number of unfiltered entries of the open list-box menu
/// (`ListBox<int>::NumUnfiltered` on the list at menu +0x40), 0 when there is
/// no such menu.
pub fn fn_008510e0(e: &mut Engine) -> u32 {
    let menu = e.global::<u32>(LIST_MENU);
    if menu == 0 {
        return 0;
    }
    e.call(LIST_MENU_COUNT, &args![menu + 0x40]).u32()
}

// Translated from 00851110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the constant 0x1b (the save's minor version). Several unrelated
/// classes share this body, so `this` is whatever the caller had in ECX.
pub fn fn_00851110(_e: &mut Engine, _unused_this: u32) -> u32 {
    SAVE_MINOR_VERSION
}

// Translated from 00851120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::GetVersionInfo` (Xbox PDB): writes
/// `"<user> - <m/d/yy h:m:s>, EXE Version: 1.4.0.525, Save Version 48.27"`
/// into `buffer` (`size` characters).
pub fn bgssaveloadmanager_get_version_info(
    e: &mut Engine,
    _unused_this: Ptr<BGSSaveLoadManager>,
    buffer: Ptr,
    size: u32,
) {
    e.with_stack(2 * PATH_SIZE + 4 + 16, |e, base| {
        let user = base.addr();
        let time_text = user + PATH_SIZE;
        let name_size = time_text + PATH_SIZE;
        let system_time = name_size + 4;
        let empty = e.mem.u8(EMPTY_STRING);
        e.mem.set_u8(user, empty);
        e.call(MEMORY_SET, &args![user + 1, 0u32, 0x103u32]);
        e.mem.set_u8(time_text, empty);
        e.call(MEMORY_SET, &args![time_text + 1, 0u32, 0x103u32]);
        e.mem.set_u32(name_size, PATH_SIZE);
        e.call(GET_USER_NAME, &args![user, name_size]);
        e.call(GET_LOCAL_TIME, &args![system_time]);
        // SYSTEMTIME: wYear +0, wMonth +2, wDay +6, wHour +8, wMinute +10,
        // wSecond +12.
        let year = e.mem.u16(system_time) as u32;
        let month = e.mem.u16(system_time + 2) as u32;
        let day = e.mem.u16(system_time + 6) as u32;
        let hour = e.mem.u16(system_time + 8) as u32;
        let minute = e.mem.u16(system_time + 10) as u32;
        let second = e.mem.u16(system_time + 12) as u32;
        e.call(
            FORMAT,
            &args![
                time_text,
                PATH_SIZE,
                LOCAL_TIME_FORMAT,
                month,
                day,
                year,
                hour,
                minute,
                second
            ],
        );
        e.call(
            FORMAT,
            &args![
                buffer,
                size,
                VERSION_INFO_FORMAT,
                user,
                time_text,
                EXE_VERSION,
                SAVE_VERSION,
                SAVE_MINOR_VERSION
            ],
        );
    });
}

// Translated from 00851230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a most-recent save name is remembered.
pub fn fn_00851230(e: &mut Engine, this: Ptr<BGSSaveLoadManager>) -> bool {
    !e.get(this, BGSSaveLoadManager::pMostRecentSaveGame)
        .is_null()
}

// Translated from 00851250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Remembers `name` and `device_id` as the most recent save. Does nothing for
/// a null name or when the same name is already remembered.
pub fn fn_00851250(e: &mut Engine, this: Ptr<BGSSaveLoadManager>, name: Ptr, device_id: i32) {
    if name.is_null() {
        return;
    }
    let current = e.get(this, BGSSaveLoadManager::pMostRecentSaveGame);
    if !current.is_null() && e.call(STRING_COMPARE, &args![current, name]).i32() == 0 {
        return;
    }
    e.call(FREE, &args![current]);
    let size = string_length(e, name.addr()).wrapping_add(1);
    let copy = e.call(ALLOCATE, &args![size]).ptr::<()>();
    e.set(this, BGSSaveLoadManager::pMostRecentSaveGame, copy);
    copy_string(e, copy.addr(), size, name.addr());
    e.set(
        this,
        BGSSaveLoadManager::iMostRecentSaveGameDeviceID,
        device_id,
    );
}

// Translated from 008512f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::LoadMostRecentSaveGame` (Xbox PDB): loads the
/// remembered most recent save (name and device id), quietly and without the
/// statistics file. Returns false when no save is remembered.
pub fn bgssaveloadmanager_load_most_recent_save_game(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadManager>,
) -> bool {
    if !fn_00851230(e, this) {
        return false;
    }
    let name = e.get(this, BGSSaveLoadManager::pMostRecentSaveGame);
    let device_id = e.get(this, BGSSaveLoadManager::iMostRecentSaveGameDeviceID);
    fn_00850760(e, this, name, device_id as u32, false, 1)
}

// Translated from 00851330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the save folder: rebuilds the list of save files from every `*.fos`
/// file with a size (`00851680` for each), sorts it, and sets the next save
/// number from the first entry (its number, plus 1 unless that entry is an
/// autosave or one of the two other kinds `00846ad0` and `00846b10` test).
/// The C++ exception frame is not translated.
pub fn fn_00851330(e: &mut Engine, this: Ptr<BGSSaveLoadManager>) {
    if !e.get(this, BGSSaveLoadManager::pSaveGameList).is_null() {
        fn_00851540(e, this);
    }
    let node = e.call(ALLOCATE, &args![8u32]).u32();
    let list = if node != 0 {
        e.call(LIST_CONSTRUCT, &args![node]).u32()
    } else {
        0
    };
    e.set(this, BGSSaveLoadManager::pSaveGameList, Ptr::new(list));
    e.set(this, BGSSaveLoadManager::iCurrentSaveGameNumber, 1);

    e.with_stack(PATH_SIZE + FIND_DATA_SIZE, |e, base| {
        let pattern = base.addr();
        let find_data = pattern + PATH_SIZE;
        e.call(SAVE_FOLDER_PATH, &args![this, pattern]);
        append_string(e, pattern, PATH_SIZE, WILDCARD);
        append_string(e, pattern, PATH_SIZE, SAVE_EXTENSION);
        let handle = e.call(FIND_FIRST_FILE, &args![pattern, find_data]).u32();
        if handle == u32::MAX {
            return;
        }
        loop {
            if e.mem.u32(find_data + FIND_DATA_FILE_SIZE_LOW) != 0 {
                fn_00851680(
                    e,
                    this,
                    Ptr::new(find_data + FIND_DATA_FILE_NAME),
                    u32::MAX,
                    0,
                );
            }
            if e.call(FIND_NEXT_FILE, &args![handle, find_data]).u32() == 0 {
                break;
            }
        }
        e.call(FIND_CLOSE, &args![handle]);
        let list = e.get(this, BGSSaveLoadManager::pSaveGameList);
        e.call(LIST_SORT, &args![list, FILE_ENTRY_COMPARE]);
        let head = e.call(THIS_IDENTITY, &args![list]).u32();
        let first = e.mem.u32(head);
        if first != 0 && e.get(this, BGSSaveLoadManager::iCurrentSaveGameNumber) <= 1 {
            let number = e.call(FILE_ENTRY_SAVE_NUMBER, &args![first]).u32();
            e.set(this, BGSSaveLoadManager::iCurrentSaveGameNumber, number);
            if !e.call(FILE_ENTRY_TEST_A, &args![first]).bool()
                && !e.call(FILE_ENTRY_IS_AUTOSAVE, &args![first]).bool()
                && !e.call(FILE_ENTRY_TEST_B, &args![first]).bool()
            {
                e.set(
                    this,
                    BGSSaveLoadManager::iCurrentSaveGameNumber,
                    number.wrapping_add(1),
                );
            }
        }
    });
}

// Translated from 00851540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empties the save file list: deletes every entry, then the list itself, and
/// zeroes the count. The next-save number is left alone.
pub fn fn_00851540(e: &mut Engine, this: Ptr<BGSSaveLoadManager>) {
    let mut node = e.get(this, BGSSaveLoadManager::pSaveGameList).addr();
    while node != 0 {
        let head = e.call(THIS_IDENTITY, &args![node]).u32();
        let entry = e.mem.u32(head);
        if entry != 0 {
            fn_008515f0(e, Ptr::new(entry), 1);
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
    }
    let list = e.get(this, BGSSaveLoadManager::pSaveGameList);
    if !list.is_null() {
        e.call(LIST_CLEAR, &args![list]);
        let list = e.get(this, BGSSaveLoadManager::pSaveGameList);
        if !list.is_null() {
            e.call(LIST_DELETE, &args![list, 1u32]);
        }
        e.set(this, BGSSaveLoadManager::pSaveGameList, Ptr::NULL);
    }
    e.set(this, BGSSaveLoadManager::iSaveGameCount, 0);
}

// Translated from 008515f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFileEntry` scalar deleting destructor: runs the destructor
/// body (`00846690`), then frees the object when bit 0 of `flags` is set.
/// Returns `this`.
pub fn fn_008515f0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(FILE_ENTRY_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00851620 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes `entry` from the list and deletes it, then refreshes the count.
/// The list's remove routine (`00905330`) takes the entry by address and may
/// replace it, so the entry deleted is what it leaves there.
pub fn fn_00851620(e: &mut Engine, this: Ptr<BGSSaveLoadManager>, entry: Ptr) {
    let list = e.get(this, BGSSaveLoadManager::pSaveGameList);
    if list.is_null() {
        return;
    }
    let removed = e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry.addr());
        e.call(LIST_REMOVE, &args![list, slot]);
        e.mem.u32(slot.addr())
    });
    if removed != 0 {
        fn_008515f0(e, Ptr::new(removed), 1);
    }
    let list = e.get(this, BGSSaveLoadManager::pSaveGameList);
    let count = e.call(LIST_COUNT, &args![list]).u32();
    e.set(this, BGSSaveLoadManager::iSaveGameCount, count);
}

// Translated from 00851680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a list entry for the save file `name` (without its `.fos` suffix):
/// builds the `BGSSaveLoadFileEntry`, loads its details, puts it at the head
/// of the list and counts it. The third word is never read. The C++
/// exception frame is not translated.
pub fn fn_00851680(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadManager>,
    name: Ptr,
    device_id: u32,
    _unused_2: u32,
) {
    e.with_stack(PATH_SIZE + 4, |e, base| {
        let buffer = base.addr();
        let entry_slot = buffer + PATH_SIZE;
        copy_string(e, buffer, PATH_SIZE, name.addr());
        let length = string_length(e, buffer);
        if length > 4 {
            let suffix = buffer + length - 4;
            if e.call(STRING_COMPARE_IGNORE_CASE, &args![suffix, SAVE_EXTENSION])
                .i32()
                == 0
            {
                e.mem.set_u8(suffix, 0);
            }
        }
        let block = e.call(ALLOCATE, &args![FILE_ENTRY_SIZE]).u32();
        let entry = if block != 0 {
            e.call(FILE_ENTRY_CONSTRUCT, &args![block, buffer, device_id])
                .u32()
        } else {
            0
        };
        e.mem.set_u32(entry_slot, entry);
        e.call(FILE_ENTRY_LOAD, &args![entry]);
        let list = e.get(this, BGSSaveLoadManager::pSaveGameList);
        e.call(LIST_ADD_HEAD, &args![list, entry_slot]);
        let count = e.get(this, BGSSaveLoadManager::iSaveGameCount);
        e.set(
            this,
            BGSSaveLoadManager::iSaveGameCount,
            count.wrapping_add(1),
        );
    });
}

// Translated from 008517c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::GenerateSaveFileName` (Xbox PDB): writes
/// `"<prefix> <save number> - <player>, <location>, <play time>"` into
/// `buffer`. `long_hours` gives the play time three-digit hours; unless it is
/// set the name is cut to 0xff characters and scrubbed of characters a file
/// name cannot hold. `alternate_prefix` picks the second prefix text.
pub fn bgssaveloadmanager_generate_save_file_name(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadManager>,
    buffer: Ptr,
    long_hours: bool,
    alternate_prefix: bool,
) {
    e.with_stack(2 * PATH_SIZE, |e, base| {
        let location = base.addr();
        let play_time = location + PATH_SIZE;
        let player = e.global::<u32>(PLAYER);
        let player_name = e.call(PLAYER_NAME, &args![player]).u32();
        bgssaveloadmanager_get_player_location_name(e, this, Ptr::new(location));
        bgssaveloadmanager_get_play_time_string(e, this, Ptr::new(play_time), long_hours);
        let prefix_setting = if alternate_prefix {
            NAME_PREFIX_ALTERNATE
        } else {
            NAME_PREFIX_GENERATED
        };
        let prefix = e.call(SETTING_VALUE, &args![prefix_setting]).u32();
        let number = e.get(this, BGSSaveLoadManager::iCurrentSaveGameNumber);
        e.call(
            FORMAT,
            &args![
                buffer,
                PATH_SIZE,
                SAVE_NAME_FORMAT,
                prefix,
                number,
                player_name,
                location,
                play_time
            ],
        );
        if !long_hours {
            if string_length(e, buffer.addr()) > PATH_SIZE - 5 {
                e.mem.set_u8(buffer.addr() + PATH_SIZE - 5, 0);
            }
            bgssaveloadmanager_scrub_file_name(e, this, buffer);
        }
    });
}

// Translated from 008518d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::ScrubFileName` (Xbox PDB): replaces characters a file
/// name cannot hold by spaces, in place. Characters below `'0'` (compared as
/// signed bytes, so every byte of 0x80 and up too) and above `'z'` become
/// spaces; a character found in the forbidden set (`\t \ / : * < > ? | " + =
/// @ ^ [ ] \` ;`) becomes a space as well, except a double quote, which
/// would become `'`. The quote test runs after the first replacement, and
/// `'"'` is below `'0'`, so that case never happens; this is what the code
/// does.
pub fn bgssaveloadmanager_scrub_file_name(
    e: &mut Engine,
    _unused_this: Ptr<BGSSaveLoadManager>,
    text: Ptr,
) {
    let length = string_length(e, text.addr());
    for index in 0..length {
        let at = text.addr() + index;
        if (e.mem.u8(at) as i8) < 0x30 {
            e.mem.set_u8(at, 0x20);
        }
        let character = e.mem.u8(at) as i8;
        if character > 0x7a {
            e.mem.set_u8(at, 0x20);
        } else if e
            .call(
                STRING_FIND_CHARACTER,
                &args![FORBIDDEN_NAME_CHARACTERS, character as i32],
            )
            .u32()
            != 0
        {
            if character == 0x22 {
                e.mem.set_u8(at, 0x27);
            } else {
                e.mem.set_u8(at, 0x20);
            }
        }
    }
}

// Translated from 00851980 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::IsSaveFileNameGenerated` (Xbox PDB): whether `name`
/// starts with the generated-name prefix text followed by a space.
pub fn bgssaveloadmanager_is_save_file_name_generated(
    e: &mut Engine,
    _unused_this: Ptr<BGSSaveLoadManager>,
    name: Ptr,
) -> bool {
    e.with_stack(PATH_SIZE, |e, prefix| {
        let text = e.call(SETTING_VALUE, &args![NAME_PREFIX_GENERATED]).u32();
        copy_string(e, prefix.addr(), PATH_SIZE, text);
        append_string(e, prefix.addr(), PATH_SIZE, SPACE);
        if name.is_null() {
            return false;
        }
        let length = string_length(e, prefix.addr());
        e.call(STRING_COMPARE_PREFIX, &args![name, prefix, length])
            .u32()
            == 0
    })
}

/// Whether `name` is non-null and starts with the text at `constant`.
fn name_starts_with(e: &mut Engine, name: Ptr, constant: u32) -> bool {
    if name.is_null() {
        return false;
    }
    let length = string_length(e, constant);
    e.call(STRING_COMPARE_PREFIX, &args![name, constant, length])
        .u32()
        == 0
}

// Translated from 00851a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::IsSaveFileNameAutosave` (Xbox PDB): whether `name`
/// starts with `"autosave"`.
pub fn fn_00851a20(e: &mut Engine, _unused_this: Ptr<BGSSaveLoadManager>, name: Ptr) -> bool {
    name_starts_with(e, name, AUTOSAVE_NAME)
}

// Translated from 00851a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `name` starts with `"quicksave"`.
pub fn fn_00851a60(e: &mut Engine, _unused_this: Ptr<BGSSaveLoadManager>, name: Ptr) -> bool {
    name_starts_with(e, name, QUICKSAVE_NAME)
}

// Translated from 00851aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `name` is a name the player chose: not a generated name and not an
/// autosave, quicksave or system save name.
pub fn fn_00851aa0(e: &mut Engine, this: Ptr<BGSSaveLoadManager>, name: Ptr) -> bool {
    !(bgssaveloadmanager_is_save_file_name_generated(e, this, name)
        || fn_00851a20(e, this, name)
        || fn_00851a60(e, this, name)
        || fn_00851b00(e, this, name))
}

// Translated from 00851b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `name` starts with `"systemsave"`.
pub fn fn_00851b00(e: &mut Engine, _unused_this: Ptr<BGSSaveLoadManager>, name: Ptr) -> bool {
    name_starts_with(e, name, SYSTEMSAVE_NAME)
}

// Translated from 00851b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::GetPlayerLocationName` (Xbox PDB): copies the
/// player's current location name into `buffer`, or an empty string when
/// there is none.
pub fn bgssaveloadmanager_get_player_location_name(
    e: &mut Engine,
    _unused_this: Ptr<BGSSaveLoadManager>,
    buffer: Ptr,
) {
    e.with_stack(8, |e, text| {
        e.call(BSSTRING_CONSTRUCT, &args![text]);
        let player = e.global::<u32>(PLAYER);
        e.call(PLAYER_LOCATION_NAME, &args![player, text]);
        let data = e.call(BSSTRING_DATA, &args![text]).u32();
        if data != 0 {
            let data = e.call(BSSTRING_DATA, &args![text]).u32();
            copy_string(e, buffer.addr(), PATH_SIZE, data);
        } else {
            copy_string(e, buffer.addr(), PATH_SIZE, EMPTY_STRING);
        }
        e.call(BSSTRING_DESTRUCT, &args![text]);
    });
}

// Translated from 00851bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::GetPlayTimeString` (Xbox PDB): writes the player's
/// play time (milliseconds, from `00851cb0`) as `HH.MM.SS` into `buffer`;
/// with `long_hours` the hours get three digits.
pub fn bgssaveloadmanager_get_play_time_string(
    e: &mut Engine,
    _unused_this: Ptr<BGSSaveLoadManager>,
    buffer: Ptr,
    long_hours: bool,
) {
    let player = e.global::<u32>(PLAYER);
    let mut time = e.call(PLAY_TIME_MS, &args![player]).u32();
    let hours = time / 3_600_000;
    time -= hours * 3_600_000;
    let minutes = time / 60_000;
    time -= minutes * 60_000;
    let seconds = time / 1000;
    let format = if long_hours {
        PLAY_TIME_FORMAT_LONG
    } else {
        PLAY_TIME_FORMAT
    };
    e.call(
        FORMAT,
        &args![buffer, PATH_SIZE, format, hours, minutes, seconds],
    );
}

// Translated from 00851cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player's play time: brings the accumulated milliseconds (`+0x790`) up
/// to date with `00851cd0` and returns them.
pub fn fn_00851cb0(e: &mut Engine, this: Ptr) -> u32 {
    fn_00851cd0(e, this);
    e.mem.u32(this.addr() + PLAY_TIME_ACCUMULATED)
}

// Translated from 00851cd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the time since the last mark (`+0x78c`) to the accumulated play time
/// (`+0x790`), then marks the current tick (`00851d10`).
pub fn fn_00851cd0(e: &mut Engine, this: Ptr) {
    let now = e.call(TICK_COUNT, &args![]).u32();
    let since = now.wrapping_sub(e.mem.u32(this.addr() + PLAY_TIME_MARK));
    let total = since.wrapping_add(e.mem.u32(this.addr() + PLAY_TIME_ACCUMULATED));
    e.mem.set_u32(this.addr() + PLAY_TIME_ACCUMULATED, total);
    fn_00851d10(e, this);
}

// Translated from 00851d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the current tick count at `+0x78c`.
pub fn fn_00851d10(e: &mut Engine, this: Ptr) {
    let now = e.call(TICK_COUNT, &args![]).u32();
    e.mem.set_u32(this.addr() + PLAY_TIME_MARK, now);
}

// Translated from 00851d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues an autosave: sets `cQueuedAutosave` to 1.
pub fn fn_00851d30(e: &mut Engine, this: Ptr<BGSSaveLoadManager>) {
    e.set(this, BGSSaveLoadManager::cQueuedAutosave, 1);
}

// Translated from 00851d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues a forced save: sets `cQueuedForceSave` to 1.
pub fn fn_00851d50(e: &mut Engine, this: Ptr<BGSSaveLoadManager>) {
    e.set(this, BGSSaveLoadManager::cQueuedForceSave, 1);
}

// Translated from 00851d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Queues a system save: sets the PC-only byte at +0x11 to 1.
pub fn fn_00851d70(e: &mut Engine, this: Ptr<BGSSaveLoadManager>) {
    e.set(this, BGSSaveLoadManager::cQueuedSystemSave, 1);
}

/// True while the fader (kinds 1 and 2) or the in-game loading menu stops a
/// queued save from running.
fn queued_save_blocked(e: &mut Engine) -> bool {
    let faders = e.global::<u32>(FADER_MANAGER);
    if e.call(FADER_ACTIVE, &args![faders, 1u32]).bool() {
        return true;
    }
    let faders = e.global::<u32>(FADER_MANAGER);
    if e.call(FADER_ACTIVE, &args![faders, 2u32]).bool() {
        return true;
    }
    e.call(LOADING_MENU_OPEN, &args![]).bool()
}

// Translated from 00851d90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::UpdateQueuedSaves` (Xbox PDB): each frame, counts a
/// queued autosave, forced save or system save down; at 1 it runs once
/// neither fader kind nor the in-game loading menu is active (an autosave
/// through `00850a40`, a forced save as `SaveGame(0, -1, 0)` followed by
/// clearing its byte, a system save through `00850a90`). Only the first
/// queue that is set is looked at. The saves run on the singleton manager.
pub fn bgssaveloadmanager_update_queued_saves(e: &mut Engine, this: Ptr<BGSSaveLoadManager>) {
    let autosave = e.get(this, BGSSaveLoadManager::cQueuedAutosave);
    if autosave != 0 {
        if autosave > 1 {
            e.set(this, BGSSaveLoadManager::cQueuedAutosave, autosave - 1);
        } else if !queued_save_blocked(e) {
            let manager = Ptr::new(e.global::<u32>(MANAGER_INSTANCE));
            fn_00850a40(e, manager);
        }
        return;
    }
    let forced = e.get(this, BGSSaveLoadManager::cQueuedForceSave);
    if forced != 0 {
        if forced > 1 {
            e.set(this, BGSSaveLoadManager::cQueuedForceSave, forced - 1);
        } else if !queued_save_blocked(e) {
            let manager = Ptr::new(e.global::<u32>(MANAGER_INSTANCE));
            bgssaveloadmanager_save_game(e, manager, Ptr::new(0), u32::MAX, false);
            e.set(this, BGSSaveLoadManager::cQueuedForceSave, 0);
        }
        return;
    }
    let system = e.get(this, BGSSaveLoadManager::cQueuedSystemSave);
    if system != 0 {
        if system > 1 {
            e.set(this, BGSSaveLoadManager::cQueuedSystemSave, system - 1);
        } else if !queued_save_blocked(e) {
            let manager = Ptr::new(e.global::<u32>(MANAGER_INSTANCE));
            fn_00850a90(e, manager);
        }
    }
}

// Translated from 00851ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadFileEntry` getter: loads the entry's data
/// (`BGSSaveLoadFileEntry::LoadData`, `00846900`) and returns the word at
/// +0x20.
pub fn fn_00851ef0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FILE_ENTRY_LOAD_DATA, &args![this]);
    e.mem.u32(this.addr() + 0x20)
}

// Translated from 00851f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00851ef0`, for the word at +0x24.
pub fn fn_00851f10(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FILE_ENTRY_LOAD_DATA, &args![this]);
    e.mem.u32(this.addr() + 0x24)
}

// Translated from 00851f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `00851ef0`, for the word at +0x28.
pub fn fn_00851f30(e: &mut Engine, this: Ptr) -> u32 {
    e.call(FILE_ENTRY_LOAD_DATA, &args![this]);
    e.mem.u32(this.addr() + 0x28)
}

// Translated from 00851f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the static object `00af2e50` creates on first use (the engine map
/// shows it as an `Immortalize` template; the callee takes no arguments and
/// `this` is left unread).
pub fn fn_00851f50(e: &mut Engine, _unused_this: u32) -> u32 {
    e.call(GAME_DATA_UTILITY, &args![]).u32()
}

// Translated from 00851f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies `text` into the 0x100-byte field at +0x164 of the save-data
/// buffer object `this`.
pub fn fn_00851f60(e: &mut Engine, this: Ptr, text: Ptr) {
    copy_string(e, this.addr() + 0x164, 0x100, text.addr());
}

// Translated from 00851f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies `text` into the 0x80-byte field at +0x265.
pub fn fn_00851f90(e: &mut Engine, this: Ptr, text: Ptr) {
    copy_string(e, this.addr() + 0x265, 0x80, text.addr());
}

// Translated from 00851fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies `text` into the 0x400-byte field at +0x2e5.
pub fn fn_00851fc0(e: &mut Engine, this: Ptr, text: Ptr) {
    copy_string(e, this.addr() + 0x2e5, 0x400, text.addr());
}

// Translated from 00851ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores three words at +0x7e8, +0x7ec and +0x7f0.
pub fn fn_00851ff0(e: &mut Engine, this: Ptr, first: u32, second: u32, third: u32) {
    e.mem.set_u32(this.addr() + 0x7e8, first);
    e.mem.set_u32(this.addr() + 0x7ec, second);
    e.mem.set_u32(this.addr() + 0x7f0, third);
}

// Translated from 00852030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies `text` into the 0x100-byte field at +0x8f4.
pub fn fn_00852030(e: &mut Engine, this: Ptr, text: Ptr) {
    copy_string(e, this.addr() + 0x8f4, 0x100, text.addr());
}

// Translated from 00852060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores `callback` at +0x9f4.
pub fn fn_00852060(e: &mut Engine, this: Ptr, callback: u32) {
    e.mem.set_u32(this.addr() + 0x9f4, callback);
}

// Translated from 00852080 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSSaveLoadManager::CopySaveGames` (Xbox PDB): the Xbox "copy saves from
/// the host" routine. It saves the data byte of the guard object
/// (`0x011de308`) and clears it, then walks the `Saves/` folder; for every
/// plain file ending in `.fos` it opens it, reads it whole into a new
/// `BGSSaveLoadFile`, builds a list entry for it, fills the save-data buffer
/// object (title `-SAVE-`, location, a description made of the player name,
/// level and cell texts, sizes, the `SAVEBACK.PNG` image and the completion
/// callback `00852740`), parks the file in the manager (`+0x20`), starts the
/// operation through the game-data utility (`00851f50`, slot 0x20) and polls
/// until its busy byte clears, sleeping 10 ms between polls. A file that
/// cannot be opened or read is reported with the debug print and skipped.
/// At the end the guard byte is restored.
///
/// The path buffer is never initialised by the game before the two appends
/// (its stack contents are whatever was there; here it starts empty). The
/// stack-protector check and the exception frame are not translated, and
/// the stack parameter is never read.
pub fn bgssaveloadmanager_copy_save_games(
    e: &mut Engine,
    this: Ptr<BGSSaveLoadManager>,
    _unused_1: u32,
) {
    let guard_data = e.call(GUARD_DATA, &args![COPY_GUARD_OBJECT]).u32();
    let saved_byte = e.mem.u8(guard_data);
    e.call(GUARD_SET, &args![COPY_GUARD_OBJECT, 0u32]);

    e.with_stack(0x6a0, |e, block| {
        let walker = block.addr();
        let folder = walker + DIRECTORY_SIZE;
        let path = folder + PATH_SIZE;
        let message = path + PATH_SIZE;
        let description = message + PATH_SIZE;
        let file = description + 0x100;
        let size = file + FILE_OBJECT_SIZE;
        let bytes_read = size + 8;
        let finished = bytes_read + 8;

        append_string(e, folder, PATH_SIZE, COPY_FOLDER);
        append_string(e, folder, PATH_SIZE, COPY_SLASH);
        e.call(DIRECTORY_CONSTRUCT, &args![walker, folder]);
        e.mem.set_u8(finished, 0);
        loop {
            e.call(DIRECTORY_NEXT, &args![walker, finished]);
            if e.mem.u8(finished) != 0 {
                break;
            }
            if e.call(DIRECTORY_IS_FILE, &args![walker]).u32() & 0xff != 1 {
                continue;
            }
            let name = e.call(DIRECTORY_FILE_NAME, &args![walker]).u32();
            let length = string_length(e, name);
            let name_end = name.wrapping_add(length).wrapping_sub(4);
            let differs = e
                .call(
                    STRING_COMPARE_PREFIX,
                    &args![name_end, SAVE_EXTENSION, 4u32],
                )
                .u32();
            if differs != 0 {
                continue;
            }
            let name = e.call(DIRECTORY_FILE_NAME, &args![walker]).u32();
            e.call(
                FORMAT,
                &args![path, PATH_SIZE, COPY_PATH_FORMAT, folder, name],
            );
            e.call(FILE_CONSTRUCT, &args![file, path, 0u32, 0u32, 0u32]);
            if e.call(FILE_OPEN_RESULT, &args![file]).u32() != 0 {
                e.call(DEBUG_PRINT, &args![COPY_OPEN_FAILED, path]);
            } else {
                let places = CopyPlaces {
                    file,
                    path,
                    message,
                    description,
                    size,
                    bytes_read,
                };
                copy_one_save(e, this, &places);
            }
            e.call(FILE_DESTRUCT, &args![file]);
        }
        e.call(GUARD_SET, &args![COPY_GUARD_OBJECT, saved_byte as u32]);
        e.call(DIRECTORY_DESTRUCT, &args![walker]);
    });
}

/// The stack locals `CopySaveGames` hands to the per-file part.
struct CopyPlaces {
    file: u32,
    path: u32,
    message: u32,
    description: u32,
    size: u32,
    bytes_read: u32,
}

/// The body of `CopySaveGames` for one opened file: read it, make the entry
/// and the buffer, and start the copy.
fn copy_one_save(e: &mut Engine, this: Ptr<BGSSaveLoadManager>, at: &CopyPlaces) {
    let CopyPlaces {
        file,
        path,
        message,
        description,
        size,
        bytes_read,
    } = *at;
    e.mem.set_u32(size, 0);
    e.mem.set_u32(size + 4, 0);
    e.call(FILE_GET_SIZE, &args![file, size]);
    let size_low = e.mem.u32(size);
    let size_high = e.mem.u32(size + 4);
    let memory = e.call(ALLOCATE, &args![0x110u32]).u32();
    let save_file = if memory != 0 {
        e.call(
            SAVE_LOAD_FILE_CONSTRUCT,
            &args![memory, COPY_BUFFER_NAME, 0u32, size_low],
        )
        .u32()
    } else {
        0
    };
    let buffer = e.call(SAVE_FILE_BUFFER, &args![save_file]).u32();
    e.mem.set_u32(bytes_read, 0);
    e.mem.set_u32(bytes_read + 4, 0);
    let data = e.call(BUFFER_DATA, &args![buffer]).u32();
    e.call(
        FILE_READ,
        &args![file, data, size_low, size_high, bytes_read],
    );
    if e.call(FILE_OPEN_RESULT, &args![file]).u32() != 0 {
        e.call(DEBUG_PRINT, &args![COPY_READ_FAILED, path]);
        return;
    }

    e.call(FORMAT, &args![message, PATH_SIZE, COPY_PROGRESS]);
    append_string(e, message, PATH_SIZE, path);
    let utility = e.call(SYSTEM_UTILITY_INSTANCE, &args![]).u32();
    let dialog = e.call(MSG_DIALOG_INSTANCE, &args![utility]).u32();
    e.vcall(dialog, 8, &args![message, 1000u32]);

    let memory = e.call(ALLOCATE, &args![FILE_ENTRY_SIZE]).u32();
    let entry = if memory != 0 {
        e.call(FILE_ENTRY_CONSTRUCT, &args![memory, path, 0u32])
            .u32()
    } else {
        0
    };
    e.call(FILE_ENTRY_FILL, &args![save_file, entry]);
    e.call(FILE_ENTRY_MARK, &args![entry]);
    e.call(BUFFER_SET_TITLE, &args![buffer, COPY_TITLE]);

    let location = e.call(ENTRY_LOCATION, &args![entry]).u32();
    let location_length = if location != 0 {
        let location = e.call(ENTRY_LOCATION, &args![entry]).u32();
        string_length(e, location)
    } else {
        0
    };
    if location != 0 && location_length != 0 {
        let location = e.call(ENTRY_LOCATION, &args![entry]).u32();
        e.call(BUFFER_SET_LOCATION, &args![buffer, location]);
    } else {
        e.call(BUFFER_SET_LOCATION, &args![buffer, COPY_LOCATION]);
    }

    let player_name = if e.call(ENTRY_PLAYER_NAME, &args![entry]).u32() != 0 {
        e.call(ENTRY_PLAYER_NAME, &args![entry]).u32()
    } else {
        e.call(SETTING_VALUE, &args![SETTING_PLAYER_NAME]).u32()
    };
    let level_name = if e.call(ENTRY_LEVEL_NAME, &args![entry]).u32() != 0 {
        e.call(ENTRY_LEVEL_NAME, &args![entry]).u32()
    } else {
        COPY_DASH
    };
    let cell_name = if e.call(ENTRY_CELL_NAME, &args![entry]).u32() != 0 {
        e.call(ENTRY_CELL_NAME, &args![entry]).u32()
    } else {
        e.call(SETTING_VALUE, &args![SETTING_CELL_NAME]).u32()
    };
    let player_label = e.call(SETTING_VALUE, &args![SETTING_PLAYER_NAME]).u32();
    let level = e.call(ENTRY_LEVEL, &args![entry]).u32();
    let level_label = e.call(SETTING_VALUE, &args![SETTING_LEVEL_LABEL]).u32();
    e.call(
        FORMAT,
        &args![
            description,
            0x100u32,
            COPY_TEXT_FORMAT,
            cell_name,
            level_label,
            level,
            level_name,
            player_label,
            player_name
        ],
    );
    e.call(BUFFER_SET_TEXT, &args![buffer, description]);

    let data = e.call(BUFFER_DATA, &args![buffer]).u32();
    let third = data.wrapping_add(e.call(ENTRY_DATA_C, &args![entry]).u32());
    let second = e.call(ENTRY_DATA_B, &args![entry]).u32();
    let first = e.call(ENTRY_DATA_A, &args![entry]).u32();
    e.call(BUFFER_SET_SIZES, &args![buffer, first, second, third]);
    e.call(BUFFER_SET_IMAGE, &args![buffer, COPY_IMAGE]);
    e.call(BUFFER_SET_CALLBACK, &args![buffer, COPY_DONE_CALLBACK]);

    if entry != 0 {
        fn_008515f0(e, Ptr::new(entry), 1);
    }
    e.call(SET_WORD_AT_20, &args![this, save_file]);
    let utility = e.call(SYSTEM_UTILITY_INSTANCE, &args![]).u32();
    let operations = fn_00851f50(e, utility);
    e.vcall(operations, 0x20, &args![buffer]);
    loop {
        let utility = e.call(SYSTEM_UTILITY_INSTANCE, &args![]).u32();
        let operations = fn_00851f50(e, utility);
        if e.call(GAME_DATA_BUSY, &args![operations]).u32() & 0xff != 1 {
            break;
        }
        e.call(SLEEP, &args![10u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008467e0, bgssaveloadmanager_remove_save_game(Ptr)),
        entry!(0x00847dd0, fn_00847dd0(Ptr)),
        entry!(
            0x00850100,
            bgssaveloadmanager_close_save_file(Ptr<BGSSaveLoadManager>, Ptr, bool)
        ),
        entry!(0x00850330, fn_00850330(Ptr, u32) -> Ptr),
        entry!(0x00850360, fn_00850360(Ptr, Ptr, u32)),
        entry!(
            0x008503b0,
            bgssaveloadmanager_save_game(Ptr<BGSSaveLoadManager>, Ptr, u32, bool) -> bool
        ),
        entry!(0x00850720, fn_00850720()),
        entry!(
            0x00850760,
            fn_00850760(Ptr<BGSSaveLoadManager>, Ptr, u32, bool, u8) -> bool
        ),
        entry!(0x008509a0, fn_008509a0(Ptr<BGSSaveLoadManager>)),
        entry!(0x008509f0, fn_008509f0(Ptr<BGSSaveLoadManager>) -> bool),
        entry!(0x00850a40, fn_00850a40(Ptr<BGSSaveLoadManager>)),
        entry!(0x00850a90, fn_00850a90(Ptr<BGSSaveLoadManager>)),
        entry!(0x00850b40, fn_00850b40(Ptr)),
        entry!(0x00850ba0, fn_00850ba0(Ptr<BGSSaveLoadManager>)),
        entry!(0x00850bf0, fn_00850bf0(Ptr<BGSSaveLoadManager>)),
        entry!(0x00850c20, fn_00850c20(Ptr)),
        entry!(0x00850d60, fn_00850d60(Ptr<BGSSaveLoadManager>)),
        entry!(0x00850ea0, fn_00850ea0(Ptr<BGSSaveLoadManager>, Ptr)),
        entry!(
            0x00850ed0,
            bgssaveloadmanager_load_game_header(Ptr<BGSSaveLoadManager>, Ptr) -> bool
        ),
        entry!(
            0x00850fe0,
            fn_00850fe0(Ptr<BGSSaveLoadManager>, bool) -> bool
        ),
        entry!(0x008510e0, fn_008510e0() -> u32),
        entry!(0x00851110, fn_00851110(u32) -> u32),
        entry!(
            0x00851120,
            bgssaveloadmanager_get_version_info(Ptr<BGSSaveLoadManager>, Ptr, u32)
        ),
        entry!(0x00851230, fn_00851230(Ptr<BGSSaveLoadManager>) -> bool),
        entry!(0x00851250, fn_00851250(Ptr<BGSSaveLoadManager>, Ptr, i32)),
        entry!(
            0x008512f0,
            bgssaveloadmanager_load_most_recent_save_game(Ptr<BGSSaveLoadManager>) -> bool
        ),
        entry!(0x00851330, fn_00851330(Ptr<BGSSaveLoadManager>)),
        entry!(0x00851540, fn_00851540(Ptr<BGSSaveLoadManager>)),
        entry!(0x008515f0, fn_008515f0(Ptr, u32) -> Ptr),
        entry!(0x00851620, fn_00851620(Ptr<BGSSaveLoadManager>, Ptr)),
        entry!(
            0x00851680,
            fn_00851680(Ptr<BGSSaveLoadManager>, Ptr, u32, u32)
        ),
        entry!(
            0x008517c0,
            bgssaveloadmanager_generate_save_file_name(Ptr<BGSSaveLoadManager>, Ptr, bool, bool)
        ),
        entry!(
            0x008518d0,
            bgssaveloadmanager_scrub_file_name(Ptr<BGSSaveLoadManager>, Ptr)
        ),
        entry!(
            0x00851980,
            bgssaveloadmanager_is_save_file_name_generated(Ptr<BGSSaveLoadManager>, Ptr) -> bool
        ),
        entry!(
            0x00851a20,
            fn_00851a20(Ptr<BGSSaveLoadManager>, Ptr) -> bool
        ),
        entry!(
            0x00851a60,
            fn_00851a60(Ptr<BGSSaveLoadManager>, Ptr) -> bool
        ),
        entry!(
            0x00851aa0,
            fn_00851aa0(Ptr<BGSSaveLoadManager>, Ptr) -> bool
        ),
        entry!(
            0x00851b00,
            fn_00851b00(Ptr<BGSSaveLoadManager>, Ptr) -> bool
        ),
        entry!(
            0x00851b40,
            bgssaveloadmanager_get_player_location_name(Ptr<BGSSaveLoadManager>, Ptr)
        ),
        entry!(
            0x00851bf0,
            bgssaveloadmanager_get_play_time_string(Ptr<BGSSaveLoadManager>, Ptr, bool)
        ),
        entry!(0x00851cb0, fn_00851cb0(Ptr) -> u32),
        entry!(0x00851cd0, fn_00851cd0(Ptr)),
        entry!(0x00851d10, fn_00851d10(Ptr)),
        entry!(0x00851d30, fn_00851d30(Ptr<BGSSaveLoadManager>)),
        entry!(0x00851d50, fn_00851d50(Ptr<BGSSaveLoadManager>)),
        entry!(0x00851d70, fn_00851d70(Ptr<BGSSaveLoadManager>)),
        entry!(
            0x00851d90,
            bgssaveloadmanager_update_queued_saves(Ptr<BGSSaveLoadManager>)
        ),
        entry!(0x00851ef0, fn_00851ef0(Ptr) -> u32),
        entry!(0x00851f10, fn_00851f10(Ptr) -> u32),
        entry!(0x00851f30, fn_00851f30(Ptr) -> u32),
        entry!(0x00851f50, fn_00851f50(u32) -> u32),
        entry!(0x00851f60, fn_00851f60(Ptr, Ptr)),
        entry!(0x00851f90, fn_00851f90(Ptr, Ptr)),
        entry!(0x00851fc0, fn_00851fc0(Ptr, Ptr)),
        entry!(0x00851ff0, fn_00851ff0(Ptr, u32, u32, u32)),
        entry!(0x00852030, fn_00852030(Ptr, Ptr)),
        entry!(0x00852060, fn_00852060(Ptr, u32)),
        entry!(
            0x00852080,
            bgssaveloadmanager_copy_save_games(Ptr<BGSSaveLoadManager>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::BTreeSet;
    use std::rc::Rc;

    /// Every function outside this file that the translations call: each gets a
    /// do-nothing double first, so no other unit's translation runs for real.
    const EXTERNAL: [u32; 131] = [
        0x0040_1000,
        0x0040_1030,
        0x0040_37b0,
        0x0040_37d0,
        0x0040_37f0,
        0x0040_3df0,
        0x0040_3e20,
        0x0040_48e0,
        0x0040_4dc0,
        0x0040_6d00,
        0x0040_6d30,
        0x0040_6d50,
        0x0040_8b20,
        0x0040_8d60,
        0x0043_d4d0,
        0x0044_1110,
        0x0044_8420,
        0x0044_8620,
        0x0044_a670,
        0x0044_ddc0,
        0x0045_3850,
        0x0045_3a70,
        0x0045_64f0,
        0x0045_7d70,
        0x0045_9a00,
        0x0047_02f0,
        0x0047_0470,
        0x0047_d0b0,
        0x0048_3710,
        0x004b_7210,
        0x004b_a8d0,
        0x004c_69f0,
        0x004f_8960,
        0x0052_5430,
        0x0052_7fc0,
        0x0055_9450,
        0x0055_d520,
        0x0056_21d0,
        0x0057_5d70,
        0x0057_8870,
        0x0058_6170,
        0x005a_9d60,
        0x005a_e270,
        0x005a_e380,
        0x005a_e3d0,
        0x005b_4940,
        0x005b_b4d0,
        0x005f_5880,
        0x0062_0b80,
        0x0063_ac70,
        0x0068_15c0,
        0x006c_0720,
        0x006c_3640,
        0x006c_39c0,
        0x006e_bc50,
        0x006e_bc70,
        0x006e_bc90,
        0x006e_bcb0,
        0x006e_bcd0,
        0x006e_bcf0,
        0x006f_ca90,
        0x0070_0960,
        0x0070_2400,
        0x0070_2440,
        0x0070_3d50,
        0x0070_3e10,
        0x0070_3e80,
        0x0070_3fa0,
        0x0070_52f0,
        0x0071_4d70,
        0x0072_6070,
        0x0076_5340,
        0x0078_cfc0,
        0x007d_0bb0,
        0x007d_0bd0,
        0x007d_6950,
        0x0083_0660,
        0x0083_fd60,
        0x0084_62c0,
        0x0084_63c0,
        0x0084_65b0,
        0x0084_6690,
        0x0084_6a20,
        0x0084_6ad0,
        0x0084_6af0,
        0x0084_6b10,
        0x0084_7850,
        0x0084_7d00,
        0x0084_7df0,
        0x0084_ab20,
        0x0084_cc40,
        0x0084_d4b0,
        0x0084_d8c0,
        0x0084_e250,
        0x0084_fbe0,
        0x0084_ff30,
        0x0084_ff90,
        0x0085_0030,
        0x0085_1cb0,
        0x0085_3a30,
        0x0086_5c10,
        0x0086_f670,
        0x0086_f6a0,
        0x0087_7430,
        0x008d_8520,
        0x0090_5330,
        0x0093_a740,
        0x0093_e770,
        0x0096_a2d0,
        0x0097_7660,
        0x009c_6c30,
        0x009c_8950,
        0x009c_a410,
        0x00a2_5920,
        0x00aa_4db0,
        0x00ad_7230,
        0x00ad_7740,
        0x00ad_84e0,
        0x00ad_8740,
        0x00c3_e310,
        0x00c3_e340,
        0x00ec_61c0,
        0x00ec_7690,
        0x00ec_862c,
        0x00ec_bf05,
        0x00fd_f014,
        0x00fd_f064,
        0x00fd_f068,
        0x00fd_f070,
        0x00fd_f0d4,
        0x00fd_f0d8,
    ];

    /// Callees of the second part (`00851cb0` onwards) outside this file.
    const EXTERNAL_PART_TWO: [u32; 33] = [
        0x0040_8d60,
        0x0040_fca0,
        0x0043_6aa0,
        0x0045_7fe0,
        0x004d_e2d0,
        0x004f_1540,
        0x0050_f9c0,
        0x0055_9450,
        0x0065_c290,
        0x0068_efb0,
        0x006e_a330,
        0x0070_1450,
        0x0070_5ea0,
        0x0075_f8c0,
        0x007d_5140,
        0x007d_5160,
        0x007d_5180,
        0x007d_6970,
        0x0084_61e0,
        0x0084_6900,
        0x0084_cbd0,
        0x0084_dab0,
        0x0085_26c0,
        0x0085_26d0,
        0x0085_2700,
        0x008d_85e0,
        0x00af_2640,
        0x00af_2e50,
        0x00b0_0720,
        0x00b0_0820,
        0x00b0_0860,
        0x00b0_0900,
        0x00b0_0950,
    ];

    fn rv(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn put(e: &mut Engine, addr: u32, text: &str) {
        e.mem.set_cstr(addr, text.as_bytes());
    }

    fn text(e: &Engine, addr: u32) -> String {
        String::from_utf8_lossy(&e.mem.cstr(addr)).into_owned()
    }

    /// A heap block holding `text`.
    fn string(e: &mut Engine, text: &str) -> u32 {
        let block = e.mem.alloc(text.len() as u32 + 1);
        put(e, block, text);
        block
    }

    /// A `Setting` object (value at +4) holding the pointer `value`.
    fn set_setting(e: &mut Engine, setting: u32, value: u32) {
        e.mem.set_u32(setting + 4, value);
    }

    /// The `printf` subset the manager uses: `%s`, `%i`, `%d`, `%0Ni`.
    fn c_format(e: &Engine, format: u32, args: &[u32]) -> String {
        let format = e.mem.cstr(format);
        let mut out = String::new();
        let (mut i, mut next) = (0, 0);
        while i < format.len() {
            if format[i] != b'%' {
                out.push(format[i] as char);
                i += 1;
                continue;
            }
            i += 1;
            let zero = format[i] == b'0';
            let mut width = 0;
            while format[i].is_ascii_digit() {
                width = width * 10 + (format[i] - b'0') as usize;
                i += 1;
            }
            let value = args[next];
            next += 1;
            match format[i] {
                b's' => out.push_str(&text(e, value)),
                b'i' | b'd' => {
                    let digits = (value as i32).to_string();
                    if zero && digits.len() < width {
                        out.push_str(&"0".repeat(width - digits.len()));
                    }
                    out.push_str(&digits);
                }
                other => panic!("unsupported format directive {}", other as char),
            }
            i += 1;
        }
        out
    }

    /// Writes `s` into the buffer at `destination` of `size` bytes.
    fn store(e: &mut Engine, destination: u32, size: u32, s: &str) {
        assert!(s.len() < size as usize, "{s:?} does not fit {size}");
        e.mem.set_cstr(destination, s.as_bytes());
    }

    /// The C library, allocator and setting getters, as small doubles.
    fn library_doubles(e: &mut Engine) {
        e.register(STRING_COPY, |e, a| {
            let s = text(e, a[2]);
            store(e, a[0], a[1], &s);
            Ret::default()
        });
        e.register(STRING_APPEND, |e, a| {
            let s = format!("{}{}", text(e, a[0]), text(e, a[2]));
            store(e, a[0], a[1], &s);
            Ret::default()
        });
        e.register(STRING_LENGTH, |e, a| rv(e.mem.cstr(a[0]).len() as u32));
        e.register(FORMAT, |e, a| {
            let s = c_format(e, a[2], &a[3..]);
            store(e, a[0], a[1], &s);
            rv(s.len() as u32)
        });
        e.register(ALLOCATE, |e, a| rv(e.mem.alloc(a[0])));
        e.register(FREE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(STRING_COMPARE, |e, a| {
            rv((text(e, a[0]) != text(e, a[1])) as u32)
        });
        e.register(STRING_COMPARE_IGNORE_CASE, |e, a| {
            rv((!text(e, a[0]).eq_ignore_ascii_case(&text(e, a[1]))) as u32)
        });
        e.register(STRING_COMPARE_PREFIX, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            let n = a[2] as usize;
            let same = x.len() >= n && y.len() >= n && x[..n] == y[..n];
            rv(!same as u32)
        });
        e.register(STRING_FIND_CHARACTER, |e, a| {
            let set = e.mem.cstr(a[0]);
            rv(set.contains(&(a[1] as u8)) as u32)
        });
        e.register(MEMORY_SET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            rv(a[0])
        });
        for getter in [SETTING_VALUE, SETTING_VALUE_COPY] {
            e.register(getter, |e, a| {
                rv(if a[0] == 0 { 0 } else { e.mem.u32(a[0] + 4) })
            });
        }
        e.register(THIS_IDENTITY, |_, a| rv(a[0]));
    }

    /// The constant strings the manager reads from the exe's data.
    fn constant_strings(e: &mut Engine) {
        for (addr, s) in [
            (BACKUP_EXTENSION, ".bak"),
            (SAVE_EXTENSION, ".fos"),
            (WILDCARD, "*"),
            (SPACE, " "),
            (QUICKSAVE_NAME, "quicksave"),
            (AUTOSAVE_NAME, "autosave"),
            (SYSTEMSAVE_NAME, "systemsave"),
            (NEWER_VERSION_MESSAGE, "newer %i %i"),
            (OUTDATED_VERSION_MESSAGE, "outdated %i %i"),
            (
                VERSION_INFO_FORMAT,
                "%s - %s, EXE Version: %s, Save Version %i.%i",
            ),
            (LOCAL_TIME_FORMAT, "%d/%d/%02d %02d:%02d:%02d"),
            (SAVE_NAME_FORMAT, "%s %i - %s, %s, %s"),
            (FORBIDDEN_NAME_CHARACTERS, "\t\\/:*<>?|\"+=@^[]`;"),
            (PLAY_TIME_FORMAT, "%02i.%02i.%02i"),
            (PLAY_TIME_FORMAT_LONG, "%03i.%02i.%02i"),
            (TWO_STRINGS_FORMAT, "%s %s"),
            (EXE_VERSION, "1.4.0.525"),
        ] {
            put(e, addr, s);
        }
        e.set_global(NOTICE_SECONDS, 2.0f32);
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        for (addr, len) in [
            (0x0101_0000, 0x7_0000),
            (0x011a_0000, 0x5_0000),
            (0x011f_0000, 0x8000),
            (0x0120_0000, 0x4000),
        ] {
            e.map(addr, len);
        }
        for addr in EXTERNAL {
            e.register(addr, |_, _| Ret::default());
        }
        for addr in EXTERNAL_PART_TWO {
            e.register(addr, |_, _| Ret::default());
        }
        library_doubles(&mut e);
        constant_strings(&mut e);
        e
    }

    fn manager(e: &mut Engine) -> Ptr<BGSSaveLoadManager> {
        let manager: Ptr<BGSSaveLoadManager> = e.new_object();
        e.set_global(MANAGER_INSTANCE, manager.addr());
        manager
    }

    /// The `BGSSaveLoadGame` object (a zeroed block).
    fn game(e: &mut Engine) -> u32 {
        let game = e.mem.alloc(0x250);
        e.set_global(SAVE_LOAD_GAME, game);
        game
    }

    /// Calls to `addr` in the log, with their argument words.
    fn calls(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    fn start_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    fn take_log(e: &mut Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log.take().unwrap()
    }

    /// A tiny file system for `XContentClose`, `_access`, `rename` and
    /// `DeleteFileA`. Paths are `saves\<name>`; `XContentClose` adds `.tmp`
    /// when asked.
    #[derive(Clone, Default)]
    struct Files {
        present: Rc<RefCell<BTreeSet<String>>>,
        events: Rc<RefCell<Vec<String>>>,
    }

    fn fake_files(e: &mut Engine, existing: &[&str]) -> Files {
        let files = Files::default();
        files
            .present
            .borrow_mut()
            .extend(existing.iter().map(|s| s.to_string()));
        e.register(CONTENT_CLOSE, |e, a| {
            let suffix = if a[3] != 0 { ".tmp" } else { "" };
            let path = format!("saves\\{}{}", text(e, a[1]), suffix);
            store(e, a[2], PATH_SIZE, &path);
            Ret::default()
        });
        let state = files.clone();
        e.register_double(FILE_ACCESS, move |e, a| {
            let there = state.present.borrow().contains(&text(e, a[0]));
            rv(if there { 0 } else { u32::MAX })
        });
        let state = files.clone();
        e.register_double(FILE_RENAME, move |e, a| {
            let (from, to) = (text(e, a[0]), text(e, a[1]));
            let mut present = state.present.borrow_mut();
            assert!(present.remove(&from), "rename of a missing file {from}");
            assert!(
                present.insert(to.clone()),
                "rename onto an existing file {to}"
            );
            state
                .events
                .borrow_mut()
                .push(format!("rename {from} {to}"));
            rv(0)
        });
        let state = files.clone();
        e.register_double(DELETE_FILE, move |e, a| {
            let path = text(e, a[0]);
            state.present.borrow_mut().remove(&path);
            state.events.borrow_mut().push(format!("delete {path}"));
            rv(1)
        });
        files
    }

    /// A save file object (`BGSSaveLoadFile`) whose path is `path`.
    fn save_file(e: &mut Engine, path: &str) -> Ptr {
        let file = e.mem.alloc(0x110);
        put(e, file, path);
        Ptr::new(file)
    }

    // ---- Files and small functions --------------------------------------

    #[test]
    fn remove_save_game_deletes_the_file_and_the_list_entry() {
        let mut e = engine();
        let manager = manager(&mut e);
        let list = e.mem.alloc(8);
        e.set(manager, BGSSaveLoadManager::pSaveGameList, Ptr::new(list));
        let files = fake_files(&mut e, &["saves\\slot1.fos"]);
        let entry = e.mem.alloc(FILE_ENTRY_SIZE);
        let name = string(&mut e, "slot1.fos");
        e.mem.set_u32(entry + FILE_ENTRY_FILE_NAME, name);
        e.mem.set_u32(entry + FILE_ENTRY_DEVICE_ID, 7);
        e.register(LIST_COUNT, |_, _| rv(4));
        start_log(&mut e);
        e.call(0x0084_67e0, &args![entry]);
        let log = take_log(&mut e);
        assert_eq!(*files.events.borrow(), vec!["delete saves\\slot1.fos"]);
        // The manager singleton is `this` of the file deletion; the entry is
        // destroyed, freed, and the count refreshed from the list.
        assert_eq!(calls(&log, CONTENT_CLOSE)[0][0], manager.addr());
        assert_eq!(calls(&log, FILE_ENTRY_DESTRUCT), vec![vec![entry]]);
        assert!(e.mem.block_size(entry).is_none());
        assert_eq!(calls(&log, LIST_REMOVE)[0][0], list);
        assert_eq!(e.get(manager, BGSSaveLoadManager::iSaveGameCount), 4);
    }

    #[test]
    fn destructor_thunk_runs_the_save_game_buffer_destructor() {
        let mut e = engine();
        e.register(0x0086_5c10, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0084_7dd0, &args![0x1234u32]);
        let log = take_log(&mut e);
        assert_eq!(calls(&log, 0x0086_5c10), vec![vec![0x1234]]);
    }

    #[test]
    fn close_save_file_without_rotation_only_deletes_the_file_object() {
        let mut e = engine();
        let manager = manager(&mut e);
        let file = save_file(&mut e, "slot.fos");
        start_log(&mut e);
        e.call(0x0085_0100, &args![manager, file, false]);
        let log = take_log(&mut e);
        assert_eq!(calls(&log, SAVE_FILE_DESTRUCT), vec![vec![file.addr()]]);
        assert!(calls(&log, CONTENT_CLOSE).is_empty());
        assert!(calls(&log, FILE_RENAME).is_empty());
        assert!(e.mem.block_size(file.addr()).is_none());
    }

    #[test]
    fn close_save_file_rotates_the_backup_generations() {
        let mut e = engine();
        let manager = manager(&mut e);
        // Two generations kept.
        e.register(SETTING_INT, |_, a| rv(a[0] + 4));
        e.set_global(BACKUP_COUNT_SETTING + 4, 2u32);
        let files = fake_files(
            &mut e,
            &[
                "saves\\slot.fos",
                "saves\\slot.fos.bak",
                "saves\\slot.fos.tmp",
            ],
        );
        let file = save_file(&mut e, "slot.fos");
        e.call(0x0085_0100, &args![manager, file, true]);
        // Generation 1 first: the old backup is dropped for the older save,
        // then the newest save becomes the backup, then the temp file takes
        // its place.
        assert_eq!(
            *files.events.borrow(),
            vec![
                "rename saves\\slot.fos.bak saves\\slot.fos.bak.bak",
                "rename saves\\slot.fos saves\\slot.fos.bak",
                "rename saves\\slot.fos.tmp saves\\slot.fos",
            ]
        );
        let present: Vec<String> = files.present.borrow().iter().cloned().collect();
        assert_eq!(
            present,
            vec![
                "saves\\slot.fos",
                "saves\\slot.fos.bak",
                "saves\\slot.fos.bak.bak"
            ]
        );
    }

    #[test]
    fn close_save_file_replaces_an_existing_older_backup() {
        let mut e = engine();
        let manager = manager(&mut e);
        e.register(SETTING_INT, |_, a| rv(a[0] + 4));
        e.set_global(BACKUP_COUNT_SETTING + 4, 1u32);
        let files = fake_files(
            &mut e,
            &[
                "saves\\slot.fos",
                "saves\\slot.fos.bak",
                "saves\\slot.fos.tmp",
            ],
        );
        let file = save_file(&mut e, "slot.fos");
        e.call(0x0085_0100, &args![manager, file, true]);
        // One generation: the existing backup is deleted before the rename.
        assert_eq!(
            *files.events.borrow(),
            vec![
                "delete saves\\slot.fos.bak",
                "rename saves\\slot.fos saves\\slot.fos.bak",
                "rename saves\\slot.fos.tmp saves\\slot.fos",
            ]
        );
    }

    #[test]
    fn close_save_file_keeps_no_backups_when_the_setting_is_zero() {
        let mut e = engine();
        let manager = manager(&mut e);
        e.register(SETTING_INT, |_, a| rv(a[0] + 4));
        e.set_global(BACKUP_COUNT_SETTING + 4, 0u32);
        let files = fake_files(&mut e, &["saves\\slot.fos.tmp"]);
        let file = save_file(&mut e, "slot.fos");
        e.call(0x0085_0100, &args![manager, file, true]);
        assert_eq!(
            *files.events.borrow(),
            vec!["rename saves\\slot.fos.tmp saves\\slot.fos"]
        );
    }

    #[test]
    fn save_file_deleting_destructor_frees_only_when_asked() {
        let mut e = engine();
        let kept = e.mem.alloc(0x110);
        let freed = e.mem.alloc(0x110);
        start_log(&mut e);
        assert_eq!(e.call(0x0085_0330, &args![kept, 0u32]).u32(), kept);
        assert_eq!(e.call(0x0085_0330, &args![freed, 1u32]).u32(), freed);
        let log = take_log(&mut e);
        assert_eq!(calls(&log, SAVE_FILE_DESTRUCT).len(), 2);
        assert!(e.mem.block_size(kept).is_some());
        assert!(e.mem.block_size(freed).is_none());
    }

    #[test]
    fn delete_file_builds_the_path_and_deletes_it() {
        let mut e = engine();
        let manager = manager(&mut e);
        let files = fake_files(&mut e, &["saves\\gone.fos"]);
        let name = string(&mut e, "gone.fos");
        e.call(0x0085_0360, &args![manager, name, 0x99u32]);
        assert_eq!(*files.events.borrow(), vec!["delete saves\\gone.fos"]);
        assert!(files.present.borrow().is_empty());
    }

    #[test]
    fn minor_version_is_a_constant() {
        let mut e = engine();
        assert_eq!(e.call(0x0085_1110, &args![0x1111u32]).u32(), 0x1b);
    }

    #[test]
    fn most_recent_save_presence() {
        let mut e = engine();
        let manager = manager(&mut e);
        assert!(!e.call(0x0085_1230, &args![manager]).bool());
        let name = string(&mut e, "slot");
        e.set(
            manager,
            BGSSaveLoadManager::pMostRecentSaveGame,
            Ptr::new(name),
        );
        assert!(e.call(0x0085_1230, &args![manager]).bool());
    }

    #[test]
    fn remember_most_recent_save_copies_the_name() {
        let mut e = engine();
        let manager = manager(&mut e);
        let first = string(&mut e, "first");
        e.call(0x0085_1250, &args![manager, first, 3i32]);
        let kept = e.get(manager, BGSSaveLoadManager::pMostRecentSaveGame);
        assert_ne!(kept.addr(), first);
        assert_eq!(text(&e, kept.addr()), "first");
        assert_eq!(
            e.get(manager, BGSSaveLoadManager::iMostRecentSaveGameDeviceID),
            3
        );

        // The same name again changes nothing, not even the device id.
        let again = string(&mut e, "first");
        e.call(0x0085_1250, &args![manager, again, 9i32]);
        assert_eq!(
            e.get(manager, BGSSaveLoadManager::pMostRecentSaveGame),
            kept
        );
        assert_eq!(
            e.get(manager, BGSSaveLoadManager::iMostRecentSaveGameDeviceID),
            3
        );

        // A new name replaces the copy (the old one is freed).
        let second = string(&mut e, "second one");
        e.call(0x0085_1250, &args![manager, second, -1i32]);
        let latest = e.get(manager, BGSSaveLoadManager::pMostRecentSaveGame);
        assert_eq!(text(&e, latest.addr()), "second one");
        assert_eq!(
            e.get(manager, BGSSaveLoadManager::iMostRecentSaveGameDeviceID),
            -1
        );
        assert!(e.mem.block_size(kept.addr()).is_none());

        // A null name is ignored.
        e.call(0x0085_1250, &args![manager, 0u32, 5i32]);
        assert_eq!(
            e.get(manager, BGSSaveLoadManager::pMostRecentSaveGame),
            latest
        );
    }

    // ---- Save names --------------------------------------------------------

    #[test]
    fn name_kind_tests_match_by_prefix() {
        let mut e = engine();
        let manager = manager(&mut e);
        set_setting(&mut e, NAME_PREFIX_GENERATED, 0);
        let auto = string(&mut e, "autosave 3");
        let quick = string(&mut e, "quicksave");
        let system = string(&mut e, "systemsave Courier");
        let other = string(&mut e, "my own save");
        for (name, autosave, quicksave, systemsave) in [
            (auto, true, false, false),
            (quick, false, true, false),
            (system, false, false, true),
            (other, false, false, false),
            (0, false, false, false),
        ] {
            assert_eq!(e.call(0x0085_1a20, &args![manager, name]).bool(), autosave);
            assert_eq!(e.call(0x0085_1a60, &args![manager, name]).bool(), quicksave);
            assert_eq!(
                e.call(0x0085_1b00, &args![manager, name]).bool(),
                systemsave
            );
        }
    }

    #[test]
    fn generated_name_check_uses_the_prefix_setting_and_a_space() {
        let mut e = engine();
        let manager = manager(&mut e);
        let prefix = string(&mut e, "Save");
        set_setting(&mut e, NAME_PREFIX_GENERATED, prefix);
        let generated = string(&mut e, "Save 4 - Courier");
        let no_space = string(&mut e, "Saved game");
        assert!(e.call(0x0085_1980, &args![manager, generated]).bool());
        assert!(!e.call(0x0085_1980, &args![manager, no_space]).bool());
        assert!(!e.call(0x0085_1980, &args![manager, 0u32]).bool());
    }

    #[test]
    fn player_chosen_name_is_none_of_the_other_kinds() {
        let mut e = engine();
        let manager = manager(&mut e);
        let prefix = string(&mut e, "Save");
        set_setting(&mut e, NAME_PREFIX_GENERATED, prefix);
        let names = [
            ("Save 4 - Courier", false),
            ("autosave", false),
            ("quicksave", false),
            ("systemsave x", false),
            ("Mojave run", true),
        ];
        for (name, chosen) in names {
            let name = string(&mut e, name);
            assert_eq!(e.call(0x0085_1aa0, &args![manager, name]).bool(), chosen);
        }
    }

    #[test]
    fn scrub_replaces_characters_a_file_name_cannot_hold() {
        let mut e = engine();
        let manager = manager(&mut e);
        let name = e.mem.alloc(0x40);
        // Below '0', above 'z', the forbidden set, a quote and a byte >= 0x80.
        e.mem.set_cstr(
            name,
            b"A:b\"c|d~e{f=g@h[i]j`k;l^m<n>o?p\\q+r s/t.u-v\x80w0z9",
        );
        e.call(0x0085_18d0, &args![manager, name]);
        assert_eq!(
            text(&e, name),
            "A b c d e f g h i j k l m n o p q r s t u v w0z9"
        );
        // Letters, digits and ' ' stay.
        e.mem.set_cstr(name, b"Quick Save 12 zZ");
        e.call(0x0085_18d0, &args![manager, name]);
        assert_eq!(text(&e, name), "Quick Save 12 zZ");
    }

    #[test]
    fn play_time_string_has_two_or_three_hour_digits() {
        let mut e = engine();
        let manager = manager(&mut e);
        let player = e.mem.alloc(16);
        e.set_global(PLAYER, player);
        // 1 h 2 min 3 s 456 ms.
        e.register(PLAY_TIME_MS, |_, _| {
            rv(3_600_000 + 2 * 60_000 + 3_000 + 456)
        });
        let out = e.mem.alloc(0x40);
        e.call(0x0085_1bf0, &args![manager, out, false]);
        assert_eq!(text(&e, out), "01.02.03");
        e.call(0x0085_1bf0, &args![manager, out, true]);
        assert_eq!(text(&e, out), "001.02.03");
    }

    #[test]
    fn location_name_is_copied_or_empty() {
        let mut e = engine();
        let manager = manager(&mut e);
        let player = e.mem.alloc(16);
        e.set_global(PLAYER, player);
        // `BSString` stand-in: the first word is the character pointer.
        e.register(BSSTRING_DATA, |e, a| rv(e.mem.u32(a[0])));
        let name = string(&mut e, "Goodsprings");
        e.register_double(PLAYER_LOCATION_NAME, move |e, a| {
            e.mem.set_u32(a[1], name);
            Ret::default()
        });
        let out = e.mem.alloc(0x40);
        start_log(&mut e);
        e.call(0x0085_1b40, &args![manager, out]);
        let log = take_log(&mut e);
        assert_eq!(text(&e, out), "Goodsprings");
        assert_eq!(calls(&log, BSSTRING_CONSTRUCT).len(), 1);
        assert_eq!(calls(&log, BSSTRING_DESTRUCT).len(), 1);

        // No location: the empty string.
        e.register(PLAYER_LOCATION_NAME, |_, _| Ret::default());
        put(&mut e, out, "stale");
        e.call(0x0085_1b40, &args![manager, out]);
        assert_eq!(text(&e, out), "");
    }

    #[test]
    fn generated_name_joins_prefix_number_player_location_and_play_time() {
        let mut e = engine();
        let manager = manager(&mut e);
        let player = e.mem.alloc(16);
        e.set_global(PLAYER, player);
        let prefix = string(&mut e, "Save");
        let other_prefix = string(&mut e, "Quest");
        set_setting(&mut e, NAME_PREFIX_GENERATED, prefix);
        set_setting(&mut e, NAME_PREFIX_ALTERNATE, other_prefix);
        let courier = string(&mut e, "Courier");
        e.register_double(PLAYER_NAME, move |_, _| rv(courier));
        e.register(BSSTRING_DATA, |e, a| rv(e.mem.u32(a[0])));
        let place = string(&mut e, "Goodsprings");
        e.register_double(PLAYER_LOCATION_NAME, move |e, a| {
            e.mem.set_u32(a[1], place);
            Ret::default()
        });
        e.register(PLAY_TIME_MS, |_, _| rv(3_723_000));
        e.set(manager, BGSSaveLoadManager::iCurrentSaveGameNumber, 5);
        let out = e.mem.alloc(0x200);

        // Without the long play time the name is scrubbed: '-', ',' and '.'
        // are below '0'.
        e.call(0x0085_17c0, &args![manager, out, false, false]);
        assert_eq!(text(&e, out), "Save 5   Courier  Goodsprings  01 02 03");
        // The long play time keeps the name as it is.
        e.call(0x0085_17c0, &args![manager, out, true, true]);
        assert_eq!(text(&e, out), "Quest 5 - Courier, Goodsprings, 001.02.03");
    }

    #[test]
    fn generated_name_is_cut_at_255_characters() {
        let mut e = engine();
        let manager = manager(&mut e);
        let player = e.mem.alloc(16);
        e.set_global(PLAYER, player);
        let prefix = string(&mut e, "S");
        set_setting(&mut e, NAME_PREFIX_GENERATED, prefix);
        let long_name = string(&mut e, &"N".repeat(220));
        e.register_double(PLAYER_NAME, move |_, _| rv(long_name));
        e.register(BSSTRING_DATA, |e, a| rv(e.mem.u32(a[0])));
        let place = string(&mut e, &"L".repeat(20));
        e.register_double(PLAYER_LOCATION_NAME, move |e, a| {
            e.mem.set_u32(a[1], place);
            Ret::default()
        });
        let out = e.mem.alloc(0x200);
        e.call(0x0085_17c0, &args![manager, out, false, false]);
        assert_eq!(text(&e, out).len(), 0xff);
    }

    #[test]
    fn system_save_name_is_cut_and_scrubbed() {
        let mut e = engine();
        manager(&mut e);
        let player = e.mem.alloc(16);
        e.set_global(PLAYER, player);
        let long = string(&mut e, "Courier: of the Mojave Wasteland");
        e.register_double(PLAYER_NAME, move |_, _| rv(long));
        let out = e.mem.alloc(0x100);
        e.call(0x0085_0b40, &args![out]);
        // "systemsave Courier: of the Mojave..." cut at 0x1b characters, the
        // colon turned into a space.
        assert_eq!(text(&e, out), "systemsave Courier  of the ");
        assert_eq!(text(&e, out).len(), 0x1b);
        // A short name is kept whole.
        let short = string(&mut e, "Ed");
        e.register_double(PLAYER_NAME, move |_, _| rv(short));
        e.call(0x0085_0b40, &args![out]);
        assert_eq!(text(&e, out), "systemsave Ed");
    }

    #[test]
    fn version_info_names_user_time_and_versions() {
        let mut e = engine();
        let manager = manager(&mut e);
        e.register(GET_USER_NAME, |e, a| {
            put(e, a[0], "alexa");
            rv(1)
        });
        e.register(GET_LOCAL_TIME, |e, a| {
            // 2026-10-09 (a Friday) 14:05:09.
            for (i, v) in [2026u16, 10, 5, 9, 14, 5, 9, 0].iter().enumerate() {
                e.mem.set_u16(a[0] + 2 * i as u32, *v);
            }
            rv(0)
        });
        let out = e.mem.alloc(0x200);
        e.call(0x0085_1120, &args![manager, out, 0x200u32]);
        assert_eq!(
            text(&e, out),
            "alexa - 10/9/2026 14:05:09, EXE Version: 1.4.0.525, Save Version 48.27"
        );
    }

    // ---- Save list ---------------------------------------------------------

    /// A list node: item, next.
    fn node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    #[test]
    fn deleting_destructor_of_an_entry_frees_only_when_asked() {
        let mut e = engine();
        let kept = e.mem.alloc(FILE_ENTRY_SIZE);
        let freed = e.mem.alloc(FILE_ENTRY_SIZE);
        start_log(&mut e);
        assert_eq!(e.call(0x0085_15f0, &args![kept, 0u32]).u32(), kept);
        assert_eq!(e.call(0x0085_15f0, &args![freed, 1u32]).u32(), freed);
        let log = take_log(&mut e);
        assert_eq!(calls(&log, FILE_ENTRY_DESTRUCT).len(), 2);
        assert!(e.mem.block_size(kept).is_some());
        assert!(e.mem.block_size(freed).is_none());
    }

    #[test]
    fn clearing_the_list_deletes_every_entry_and_the_list() {
        let mut e = engine();
        let manager = manager(&mut e);
        let first = e.mem.alloc(FILE_ENTRY_SIZE);
        let second = e.mem.alloc(FILE_ENTRY_SIZE);
        let tail = node(&mut e, second, 0);
        let empty = node(&mut e, 0, tail);
        let list = node(&mut e, first, empty);
        e.set(manager, BGSSaveLoadManager::pSaveGameList, Ptr::new(list));
        e.set(manager, BGSSaveLoadManager::iSaveGameCount, 3);
        e.set(manager, BGSSaveLoadManager::iCurrentSaveGameNumber, 9);
        e.register(LIST_NEXT, |e, a| rv(e.mem.u32(a[0] + 4)));
        start_log(&mut e);
        e.call(0x0085_1540, &args![manager]);
        let log = take_log(&mut e);
        // Both entries are destroyed (the empty node is skipped), then the
        // list is cleared and deleted.
        assert_eq!(
            calls(&log, FILE_ENTRY_DESTRUCT),
            vec![vec![first], vec![second]]
        );
        assert!(e.mem.block_size(first).is_none() && e.mem.block_size(second).is_none());
        assert_eq!(calls(&log, LIST_CLEAR), vec![vec![list]]);
        assert_eq!(calls(&log, LIST_DELETE), vec![vec![list, 1]]);
        assert!(e.get(manager, BGSSaveLoadManager::pSaveGameList).is_null());
        assert_eq!(e.get(manager, BGSSaveLoadManager::iSaveGameCount), 0);
        assert_eq!(
            e.get(manager, BGSSaveLoadManager::iCurrentSaveGameNumber),
            9
        );
    }

    #[test]
    fn clearing_without_a_list_only_zeroes_the_count() {
        let mut e = engine();
        let manager = manager(&mut e);
        e.set(manager, BGSSaveLoadManager::iSaveGameCount, 3);
        start_log(&mut e);
        e.call(0x0085_1540, &args![manager]);
        let log = take_log(&mut e);
        assert!(calls(&log, LIST_CLEAR).is_empty());
        assert_eq!(e.get(manager, BGSSaveLoadManager::iSaveGameCount), 0);
    }

    #[test]
    fn removing_an_entry_deletes_what_the_list_hands_back() {
        let mut e = engine();
        let manager = manager(&mut e);
        let list = e.mem.alloc(8);
        e.set(manager, BGSSaveLoadManager::pSaveGameList, Ptr::new(list));
        e.register(LIST_COUNT, |_, _| rv(2));
        let entry = e.mem.alloc(FILE_ENTRY_SIZE);
        start_log(&mut e);
        e.call(0x0085_1620, &args![manager, entry]);
        let log = take_log(&mut e);
        assert_eq!(calls(&log, FILE_ENTRY_DESTRUCT), vec![vec![entry]]);
        assert!(e.mem.block_size(entry).is_none());
        assert_eq!(e.get(manager, BGSSaveLoadManager::iSaveGameCount), 2);

        // The remove routine reports that nothing was removed: no deletion.
        let other = e.mem.alloc(FILE_ENTRY_SIZE);
        e.register(LIST_REMOVE, |e, a| {
            e.mem.set_u32(a[1], 0);
            Ret::default()
        });
        start_log(&mut e);
        e.call(0x0085_1620, &args![manager, other]);
        let log = take_log(&mut e);
        assert!(calls(&log, FILE_ENTRY_DESTRUCT).is_empty());
        assert!(e.mem.block_size(other).is_some());
    }

    #[test]
    fn removing_without_a_list_does_nothing() {
        let mut e = engine();
        let manager = manager(&mut e);
        e.set(manager, BGSSaveLoadManager::iSaveGameCount, 6);
        start_log(&mut e);
        e.call(0x0085_1620, &args![manager, 0x1234u32]);
        assert!(take_log(&mut e).iter().all(|(a, _)| *a == 0x0085_1620));
        assert_eq!(e.get(manager, BGSSaveLoadManager::iSaveGameCount), 6);
    }

    #[test]
    fn adding_a_save_strips_the_extension_and_puts_it_at_the_head() {
        let mut e = engine();
        let manager = manager(&mut e);
        let list = e.mem.alloc(8);
        e.set(manager, BGSSaveLoadManager::pSaveGameList, Ptr::new(list));
        let built = Rc::new(RefCell::new(Vec::new()));
        let seen = built.clone();
        e.register_double(FILE_ENTRY_CONSTRUCT, move |e, a| {
            seen.borrow_mut().push((text(e, a[1]), a[2]));
            rv(a[0])
        });
        e.register(LIST_ADD_HEAD, |e, a| {
            let entry = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], entry);
            Ret::default()
        });
        let name = string(&mut e, "Quicksave 3.FOS");
        start_log(&mut e);
        e.call(0x0085_1680, &args![manager, name, 5u32, 77u32]);
        let log = take_log(&mut e);
        assert_eq!(*built.borrow(), vec![("Quicksave 3".to_string(), 5)]);
        let entry = e.mem.u32(list);
        assert!(e
            .mem
            .block_size(entry)
            .is_some_and(|size| size >= FILE_ENTRY_SIZE));
        assert_eq!(calls(&log, FILE_ENTRY_LOAD), vec![vec![entry]]);
        assert_eq!(e.get(manager, BGSSaveLoadManager::iSaveGameCount), 1);

        // A name that does not end in the extension, and a short one, are kept.
        for kept in ["notes.txt", "abcd"] {
            let name = string(&mut e, kept);
            e.call(0x0085_1680, &args![manager, name, 0u32, 0u32]);
            assert_eq!(built.borrow().last().unwrap().0, kept);
        }
        assert_eq!(e.get(manager, BGSSaveLoadManager::iSaveGameCount), 3);
    }

    /// The directory scan: `entries` are (file name, size); the first one is
    /// returned by `FindFirstFileA`, the rest by `FindNextFileA`.
    fn fake_folder(e: &mut Engine, entries: &[(&str, u32)]) -> Rc<RefCell<Vec<String>>> {
        let patterns = Rc::new(RefCell::new(Vec::new()));
        let queue = Rc::new(RefCell::new(
            entries
                .iter()
                .map(|(n, s)| (n.to_string(), *s))
                .collect::<Vec<_>>(),
        ));
        e.register(SAVE_FOLDER_PATH, |e, a| {
            put(e, a[1], "saves\\");
            Ret::default()
        });
        fn deliver(e: &mut Engine, find_data: u32, entry: &(String, u32)) {
            e.mem.set_u32(find_data + FIND_DATA_FILE_SIZE_LOW, entry.1);
            put(e, find_data + FIND_DATA_FILE_NAME, &entry.0);
        }
        let pending = queue.clone();
        let seen = patterns.clone();
        e.register_double(FIND_FIRST_FILE, move |e, a| {
            seen.borrow_mut().push(text(e, a[0]));
            if pending.borrow().is_empty() {
                return rv(u32::MAX);
            }
            let first = pending.borrow_mut().remove(0);
            deliver(e, a[1], &first);
            rv(0x77)
        });
        let pending = queue;
        e.register_double(FIND_NEXT_FILE, move |e, a| {
            if pending.borrow().is_empty() {
                return rv(0);
            }
            let next = pending.borrow_mut().remove(0);
            deliver(e, a[1], &next);
            rv(1)
        });
        patterns
    }

    /// Records the (name, device) of every entry the scan builds, and makes
    /// the list's head item the most recently added entry.
    fn entry_recorder(e: &mut Engine) -> Rc<RefCell<Vec<(String, u32)>>> {
        let built = Rc::new(RefCell::new(Vec::new()));
        let seen = built.clone();
        e.register_double(FILE_ENTRY_CONSTRUCT, move |e, a| {
            seen.borrow_mut().push((text(e, a[1]), a[2]));
            rv(a[0])
        });
        e.register(LIST_ADD_HEAD, |e, a| {
            let entry = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], entry);
            Ret::default()
        });
        e.register(LIST_CONSTRUCT, |_, a| rv(a[0]));
        built
    }

    #[test]
    fn scanning_the_folder_lists_the_saves_and_picks_the_next_number() {
        let mut e = engine();
        let manager = manager(&mut e);
        let patterns = fake_folder(
            &mut e,
            &[("Save 1.fos", 100), ("empty.fos", 0), ("Save 2.fos", 200)],
        );
        let built = entry_recorder(&mut e);
        e.register(FILE_ENTRY_SAVE_NUMBER, |_, _| rv(12));
        start_log(&mut e);
        e.call(0x0085_1330, &args![manager]);
        let log = take_log(&mut e);
        assert_eq!(*patterns.borrow(), vec!["saves\\*.fos"]);
        // The empty file is skipped; the others are added with the default
        // device.
        assert_eq!(
            *built.borrow(),
            vec![
                ("Save 1".to_string(), u32::MAX),
                ("Save 2".to_string(), u32::MAX)
            ]
        );
        let list = e.get(manager, BGSSaveLoadManager::pSaveGameList);
        assert!(!list.is_null());
        assert_eq!(e.get(manager, BGSSaveLoadManager::iSaveGameCount), 2);
        assert_eq!(
            calls(&log, LIST_SORT),
            vec![vec![list.addr(), FILE_ENTRY_COMPARE]]
        );
        assert_eq!(calls(&log, FIND_CLOSE), vec![vec![0x77]]);
        // The head entry is not an autosave or one of the other kinds: its
        // number plus one.
        assert_eq!(
            e.get(manager, BGSSaveLoadManager::iCurrentSaveGameNumber),
            13
        );
    }

    #[test]
    fn scanning_keeps_the_head_number_for_autosave_style_entries() {
        for kind in [FILE_ENTRY_TEST_A, FILE_ENTRY_IS_AUTOSAVE, FILE_ENTRY_TEST_B] {
            let mut e = engine();
            let manager = manager(&mut e);
            fake_folder(&mut e, &[("autosave.fos", 10)]);
            entry_recorder(&mut e);
            e.register(FILE_ENTRY_SAVE_NUMBER, |_, _| rv(12));
            e.register(kind, |_, _| rv(1));
            e.call(0x0085_1330, &args![manager]);
            assert_eq!(
                e.get(manager, BGSSaveLoadManager::iCurrentSaveGameNumber),
                12
            );
        }
    }

    #[test]
    fn scanning_an_unreadable_folder_leaves_an_empty_list() {
        let mut e = engine();
        let manager = manager(&mut e);
        fake_folder(&mut e, &[]);
        entry_recorder(&mut e);
        // A previous list is thrown away first.
        let old = e.mem.alloc(8);
        e.set(manager, BGSSaveLoadManager::pSaveGameList, Ptr::new(old));
        start_log(&mut e);
        e.call(0x0085_1330, &args![manager]);
        let log = take_log(&mut e);
        assert_eq!(calls(&log, LIST_CLEAR), vec![vec![old]]);
        assert!(calls(&log, LIST_SORT).is_empty());
        assert!(!e.get(manager, BGSSaveLoadManager::pSaveGameList).is_null());
        assert_eq!(
            e.get(manager, BGSSaveLoadManager::iCurrentSaveGameNumber),
            1
        );
    }

    // ---- Gates ---------------------------------------------------------------

    /// `00850fe0` with the given player state, "cannot save" flag, VATS flag
    /// and open menus (ids), and the unfiltered count of the list menu.
    fn may_save(
        player_state: u32,
        cannot_save: bool,
        vats: bool,
        menus: &[u32],
        list_entries: u32,
        autosave: bool,
    ) -> bool {
        let mut e = engine();
        let manager = manager(&mut e);
        let player = e.mem.alloc(16);
        e.mem.set_u32(player, player_state);
        e.set_global(PLAYER, player);
        e.register(PLAYER_STATE, |e, a| rv(e.mem.u32(a[0])));
        e.register_double(PLAYER_CANNOT_SAVE, move |_, _| rv(cannot_save as u32));
        e.register_double(VATS_ACTIVE, move |_, _| rv(vats as u32));
        let ids = menus.to_vec();
        e.register_double(MENU_COUNT, move |_, _| rv(ids.len() as u32));
        let ids = menus.to_vec();
        e.register_double(MENU_ID_AT, move |_, a| rv(ids[a[0] as usize]));
        let menu = e.mem.alloc(0x100);
        e.set_global(LIST_MENU, menu);
        e.register_double(LIST_MENU_COUNT, move |_, _| rv(list_entries));
        e.call(0x0085_0fe0, &args![manager, autosave]).bool()
    }

    #[test]
    fn saving_is_refused_for_player_states_vats_and_menus() {
        assert!(may_save(0, false, false, &[], 0, false));
        assert!(!may_save(1, false, false, &[], 0, false));
        assert!(!may_save(2, false, false, &[], 0, false));
        assert!(may_save(3, false, false, &[], 0, false));
        assert!(!may_save(0, true, false, &[], 0, false));
        assert!(!may_save(0, false, true, &[], 0, false));
    }

    #[test]
    fn open_menus_decide_whether_a_manual_save_may_run() {
        // The ids 0, 3 and 0x3f5 are harmless.
        assert!(may_save(0, false, false, &[0, 3, 0x3f5], 0, false));
        // 0x3e9 is harmless while its list has no entries.
        assert!(may_save(0, false, false, &[0x3e9], 0, false));
        assert!(!may_save(0, false, false, &[0x3e9], 2, false));
        // Anything else blocks, below or above.
        assert!(!may_save(0, false, false, &[5], 0, false));
        assert!(!may_save(0, false, false, &[0, 0x3f6], 0, false));
        assert!(!may_save(0, false, false, &[0x400], 0, false));
        // An autosave ignores the menus altogether.
        assert!(may_save(0, false, false, &[5, 0x400], 2, true));
        // But not the player's state.
        assert!(!may_save(2, false, false, &[], 0, true));
    }

    #[test]
    fn list_menu_count_is_zero_without_a_menu() {
        let mut e = engine();
        assert_eq!(e.call(0x0085_10e0, &args![]).u32(), 0);
        let menu = e.mem.alloc(0x100);
        e.set_global(LIST_MENU, menu);
        e.register(LIST_MENU_COUNT, |_, a| rv(a[0]));
        assert_eq!(e.call(0x0085_10e0, &args![]).u32(), menu + 0x40);
    }

    // ---- Versions ----------------------------------------------------------

    /// Message boxes: (text, argument words).
    type Boxes = Rc<RefCell<Vec<(String, Vec<u32>)>>>;

    fn record_boxes(e: &mut Engine) -> Boxes {
        let boxes: Boxes = Rc::default();
        let seen = boxes.clone();
        e.register_double(SHOW_MESSAGE_BOX, move |e, a| {
            seen.borrow_mut().push((text(e, a[0]), a.to_vec()));
            Ret::default()
        });
        boxes
    }

    fn header_check(version: i32) -> (bool, Vec<(String, Vec<u32>)>, u32) {
        let mut e = engine();
        let manager = manager(&mut e);
        let boxes = record_boxes(&mut e);
        e.register_double(LOAD_HEADER, move |_, _| rv(version as u32));
        let corrupt = string(&mut e, "damaged save");
        set_setting(&mut e, MESSAGE_SETTING_CORRUPT_SAVE, corrupt);
        let ok = string(&mut e, "OK");
        set_setting(&mut e, MESSAGE_SETTING_BUTTON_OK, ok);
        let file = e.mem.alloc(0x110);
        let accepted = e.call(0x0085_0ed0, &args![manager, file]).bool();
        let shown = boxes.borrow().clone();
        (accepted, shown, ok)
    }

    #[test]
    fn versions_2f_and_30_load_silently() {
        for version in [0x2f, 0x30] {
            let (accepted, shown, _) = header_check(version);
            assert!(accepted);
            assert!(shown.is_empty());
        }
    }

    #[test]
    fn other_versions_raise_a_message_box_and_are_refused() {
        let (accepted, shown, ok) = header_check(0x31);
        assert!(!accepted);
        assert_eq!(shown[0].0, "newer 49 48");
        let (accepted, shown, _) = header_check(0x2e);
        assert!(!accepted);
        assert_eq!(shown[0].0, "outdated 46 48");
        // The box: no callback, type 0x17, the OK button, terminated by 0.
        let words = &shown[0].1;
        assert_eq!(words.len(), 10);
        assert_eq!(&words[1..5], &[0, 0, 0, 0]);
        assert_eq!(words[5], 0x17);
        assert_eq!(words[8], ok);
        assert_eq!(words[9], 0);
        // No usable version at all.
        for version in [0, -3] {
            let (accepted, shown, _) = header_check(version);
            assert!(!accepted);
            assert_eq!(shown[0].0, "damaged save");
        }
    }

    // ---- Pausing and tearing down -------------------------------------------

    /// The addresses called, in order, leaving out the top-level call.
    fn order(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        log.iter().skip(1).map(|(a, _)| *a).collect()
    }

    #[test]
    fn pausing_waits_until_the_path_manager_has_stopped() {
        let mut e = engine();
        let manager = manager(&mut e);
        e.set_global(IO_MANAGER, 0x5555u32);
        e.register(PATH_MANAGER, |_, _| rv(0x7777));
        let polls = Rc::new(RefCell::new(0));
        let counter = polls.clone();
        e.register_double(0x006e_bc90, move |_, _| {
            *counter.borrow_mut() += 1;
            rv((*counter.borrow() == 3) as u32)
        });
        start_log(&mut e);
        e.call(0x0085_0ba0, &args![manager]);
        let log = take_log(&mut e);
        let mut expected = vec![PATH_MANAGER, 0x006e_bc70, 0x00c3_e310];
        for _ in 0..2 {
            expected.extend([PATH_MANAGER, 0x006e_bc90, PATH_MANAGER, 0x006e_bc50]);
        }
        expected.extend([PATH_MANAGER, 0x006e_bc90]);
        assert_eq!(order(&log), expected);
        assert_eq!(calls(&log, 0x00c3_e310), vec![vec![0x5555]]);
        assert_eq!(calls(&log, 0x006e_bc70), vec![vec![0x7777]]);
    }

    #[test]
    fn resuming_restarts_the_io_manager_and_the_path_manager() {
        let mut e = engine();
        let manager = manager(&mut e);
        e.set_global(IO_MANAGER, 0x5555u32);
        e.register(PATH_MANAGER, |_, _| rv(0x7777));
        start_log(&mut e);
        e.call(0x0085_0bf0, &args![manager]);
        let log = take_log(&mut e);
        assert_eq!(order(&log), vec![0x00c3_e340, PATH_MANAGER, 0x006e_bcd0]);
        assert_eq!(calls(&log, 0x00c3_e340), vec![vec![0x5555]]);
        assert_eq!(calls(&log, 0x006e_bcd0), vec![vec![0x7777]]);
    }

    #[test]
    fn tearing_down_for_a_load_with_vats_and_a_busy_path_manager() {
        let mut e = engine();
        let player = e.mem.alloc(16);
        e.set_global(PLAYER, player);
        e.set_global(WORLD, 0x1010u32);
        e.set_global(MODEL_LOADER, 0x2020u32);
        e.set_global(EXTERIOR_CELL_LOADER, 0x3030u32);
        e.register(AUDIO, |_, _| rv(0xa0a0));
        e.register(PATH_MANAGER, |_, _| rv(0x7777));
        e.register(0x005b_b4d0, |_, _| rv(1));
        e.register(VATS_ACTIVE, |_, _| rv(1));
        let polls = Rc::new(RefCell::new(0));
        let counter = polls.clone();
        e.register_double(0x006e_bcb0, move |_, _| {
            *counter.borrow_mut() += 1;
            rv((*counter.borrow() == 3) as u32)
        });
        start_log(&mut e);
        e.call(0x0085_0c20, &args![0x9999u32]);
        let log = take_log(&mut e);

        let mut expected = vec![
            0x0052_7fc0,
            0x0044_8420,
            0x005b_4940,
            0x005a_e270,
            0x005a_9d60,
            AUDIO,
            0x00ad_84e0,
            AUDIO,
            0x005b_b4d0,
            AUDIO,
            0x00ad_7230,
        ];
        for _ in 0..10 {
            expected.extend([AUDIO, 0x00ad_7740]);
        }
        expected.extend([AUDIO, 0x00ad_7230, VATS_ACTIVE, 0x009c_8950, 0x009c_6c30]);
        expected.extend([0x00aa_4db0, 0x0093_e770, 0x0045_7d70]);
        for _ in 0..2 {
            expected.extend([
                PATH_MANAGER,
                0x006e_bcb0,
                PATH_MANAGER,
                0x006e_bc50,
                0x006c_0720,
                0x006c_3640,
            ]);
        }
        expected.extend([PATH_MANAGER, 0x006e_bcb0, PATH_MANAGER, 0x006e_bcf0]);
        assert_eq!(order(&log), expected);

        assert_eq!(calls(&log, 0x0052_7fc0), vec![vec![0x3030]]);
        assert_eq!(calls(&log, 0x00ad_84e0), vec![vec![0xa0a0, u32::MAX, 0]]);
        // Multithreading off, then back to what it was (1).
        assert_eq!(
            calls(&log, 0x00ad_7230),
            vec![vec![0xa0a0, 0], vec![0xa0a0, 1]]
        );
        assert_eq!(calls(&log, 0x009c_8950), vec![vec![VATS, 1, 0]]);
        assert_eq!(calls(&log, 0x009c_6c30), vec![vec![VATS, 0, 0]]);
        assert_eq!(
            calls(&log, 0x00aa_4db0),
            vec![vec![TIMER, 1.0f32.to_bits(), 1]]
        );
        assert_eq!(calls(&log, 0x0093_e770), vec![vec![player, 2, 1]]);
        assert_eq!(calls(&log, 0x0045_7d70), vec![vec![0x1010, 1, 0, 0]]);
        assert_eq!(calls(&log, 0x006e_bcf0), vec![vec![0x7777]]);
    }

    #[test]
    fn tearing_down_without_vats_or_a_player() {
        let mut e = engine();
        e.register(AUDIO, |_, _| rv(0xa0a0));
        e.register(PATH_MANAGER, |_, _| rv(0x7777));
        e.register(0x006e_bcb0, |_, _| rv(1));
        start_log(&mut e);
        e.call(0x0085_0c20, &args![0u32]);
        let log = take_log(&mut e);
        assert!(calls(&log, 0x009c_8950).is_empty());
        assert!(calls(&log, 0x0093_e770).is_empty());
        assert!(calls(&log, 0x006c_0720).is_empty());
        assert_eq!(calls(&log, 0x00aa_4db0).len(), 1);
    }

    fn world_up_rig(e: &mut Engine) -> u32 {
        let player = e.mem.alloc(16);
        let vtable = e.mem.alloc(0x200);
        e.mem.set_u32(vtable + 0x1f4, 0x008a_e4c0);
        e.mem.set_u32(player, vtable);
        e.set_global(PLAYER, player);
        e.set_global(MAIN, 0x4040u32);
        e.set_global(WORLD, 0x1010u32);
        e.set_global(MODEL_LOADER, 0x2020u32);
        e.set_global(FADER_MANAGER, 0x6060u32);
        e.set_global(SAVE_LOAD_GAME, 0x8080u32);
        e.register(AUDIO, |_, _| rv(0xa0a0));
        e.register(PATH_MANAGER, |_, _| rv(0x7777));
        e.register(0x006c_0720, |_, _| rv(0xb0b0));
        player
    }

    #[test]
    fn bringing_the_world_back_up_after_a_load() {
        let mut e = engine();
        let player = world_up_rig(&mut e);
        e.register(0x0057_5d70, |_, _| rv(0x5151));
        e.register(0x0058_6170, |_, a| rv(a[0] + 1));
        e.register(0x008a_e4c0, |_, _| rv(0x9191));
        e.register(0x00a2_5920, |_, _| rv(0xc0c0));
        e.register(0x0044_ddc0, |_, a| rv(a[0] + 2));
        let fade = e.mem.alloc(4);
        e.mem.set_f32(fade, 0.75);
        set_fade(&mut e, fade);
        start_log(&mut e);
        e.call(0x0085_0d60, &args![0x9999u32]);
        let log = take_log(&mut e);
        // Terrain update: the world space's terrain manager, the player's
        // position (virtual slot 0x1f4) and 0xf.
        assert_eq!(calls(&log, 0x0057_5d70), vec![vec![player]]);
        assert_eq!(calls(&log, 0x006f_ca90), vec![vec![0x5152, 0x9191, 0xf]]);
        // The start menu, when there is one.
        assert_eq!(calls(&log, 0x007d_0bd0), vec![vec![0xc0c0, 1]]);
        assert_eq!(calls(&log, 0x007d_0bb0), vec![vec![0xc0c0]]);
        // Fade in with the setting's time.
        assert_eq!(
            calls(&log, 0x0070_0960),
            vec![vec![0x6060, 1, 0.75f32.to_bits(), 1]]
        );
        assert_eq!(calls(&log, 0x0045_7d70), vec![vec![0x1010, 0, 0, 0]]);
        assert_eq!(calls(&log, 0x0044_8620), vec![vec![0x2020, 0]]);
        assert_eq!(calls(&log, 0x004b_a8d0), vec![vec![0x1012]]);
        assert_eq!(calls(&log, 0x0084_cc40), vec![vec![0x8080, 0]]);
        assert_eq!(calls(&log, 0x006c_39c0), vec![vec![0xb0b0]]);
        assert_eq!(calls(&log, 0x006e_bcd0), vec![vec![0x7777]]);
        assert_eq!(calls(&log, 0x0086_f6a0), vec![vec![0x4040]]);
        assert_eq!(calls(&log, 0x0097_7660), vec![vec![PROCESS_LISTS]]);
    }

    /// Makes the fade setting object at `FADE_SETTING` hand out `value`.
    fn set_fade(e: &mut Engine, value: u32) {
        e.register_double(SETTING_FLOAT, move |_, _| rv(value));
    }

    #[test]
    fn bringing_the_world_back_up_without_terrain_or_start_menu() {
        let mut e = engine();
        world_up_rig(&mut e);
        let fade = e.mem.alloc(4);
        set_fade(&mut e, fade);
        start_log(&mut e);
        e.call(0x0085_0d60, &args![0x9999u32]);
        let log = take_log(&mut e);
        assert!(calls(&log, 0x006f_ca90).is_empty());
        assert!(calls(&log, 0x0058_6170).is_empty());
        assert!(calls(&log, 0x007d_0bd0).is_empty());
        assert_eq!(calls(&log, 0x0070_0960).len(), 1);
    }

    #[test]
    fn writing_the_header_sets_the_game_flag_first() {
        let mut e = engine();
        let manager = manager(&mut e);
        let game = game(&mut e);
        let file = e.mem.alloc(0x110);
        start_log(&mut e);
        e.call(0x0085_0ea0, &args![manager, file]);
        let log = take_log(&mut e);
        assert_eq!(order(&log), vec![GAME_SET_FLAG, SAVE_HEADER]);
        assert_eq!(calls(&log, GAME_SET_FLAG), vec![vec![game, 1]]);
        assert_eq!(calls(&log, SAVE_HEADER), vec![vec![file]]);
    }

    // ---- Saving and loading ---------------------------------------------------

    /// (name, for_save, mode, device) of every `OpenSaveFile`.
    type Opens = Rc<RefCell<Vec<(String, u32, u32, u32)>>>;
    /// (text, icon, flag) of every notice.
    type Notices = Rc<RefCell<Vec<(String, u32, u32)>>>;
    /// (file, quiet, loading from the start menu) of every game load.
    type Loads = Rc<RefCell<Vec<(u32, u32, bool)>>>;

    /// What the save/load tests share: a manager with a list and the number
    /// 5, the game object, one save file object, a fake file system, and
    /// doubles for everything the flows call.
    struct Rig {
        manager: Ptr<BGSSaveLoadManager>,
        game: u32,
        file: u32,
        files: Files,
        /// (name, for_save, mode, device) of every `OpenSaveFile`.
        opens: Opens,
        /// (text, icon, flag) of every notice.
        notices: Notices,
        boxes: Boxes,
    }

    fn rig(e: &mut Engine) -> Rig {
        let manager = manager(e);
        e.set(manager, BGSSaveLoadManager::iCurrentSaveGameNumber, 5);
        let list = e.mem.alloc(8);
        e.set(manager, BGSSaveLoadManager::pSaveGameList, Ptr::new(list));
        let game = game(e);
        let file = e.mem.alloc(0x110);
        put(e, file, "slot.fos");
        let files = fake_files(e, &["saves\\slot.fos.tmp"]);
        let opens: Opens = Rc::default();
        let seen = opens.clone();
        e.register_double(OPEN_SAVE_FILE, move |e, a| {
            let name = if a[1] == 0 {
                String::new()
            } else {
                text(e, a[1])
            };
            seen.borrow_mut().push((name, a[2], a[3], a[4]));
            rv(file)
        });
        e.set_global(IO_MANAGER, 0x5555u32);
        e.register(PATH_MANAGER, |_, _| rv(0x7777));
        e.register(0x006e_bc90, |_, _| rv(1));
        e.register(GAME_HISTORY, |_, a| rv(a[0] + 0x1c));
        e.register(HISTORY_SIZE, |_, _| rv(42));
        e.register(FILE_ENTRY_CONSTRUCT, |_, a| rv(a[0]));
        e.register(SETTING_INT, |_, a| rv(a[0] + 4));
        e.set_global(BACKUP_COUNT_SETTING + 4, 0u32);
        for (setting, message) in [
            (MESSAGE_SETTING_CANNOT_SAVE, "cannot save"),
            (MESSAGE_SETTING_SAVING, "saving"),
            (MESSAGE_SETTING_SAVE_FAILED, "save failed"),
            (MESSAGE_SETTING_QUICKSAVING, "quick saving"),
            (MESSAGE_SETTING_QUICKLOADING, "quick loading"),
            (MESSAGE_SETTING_LOAD_FAILED, "load failed"),
            (MESSAGE_SETTING_BUTTON_YES, "Yes"),
            (MESSAGE_SETTING_BUTTON_NO, "No"),
        ] {
            let message = string(e, message);
            set_setting(e, setting, message);
        }
        let notices: Notices = Rc::default();
        let seen = notices.clone();
        e.register_double(SHOW_NOTICE, move |e, a| {
            assert_eq!(a[4], 2.0f32.to_bits());
            seen.borrow_mut().push((text(e, a[0]), a[2], a[5]));
            Ret::default()
        });
        let boxes = record_boxes(e);
        Rig {
            manager,
            game,
            file,
            files,
            opens,
            notices,
            boxes,
        }
    }

    fn save(e: &mut Engine, rig: &Rig, name: &str, device: u32, stats: bool) -> bool {
        let name = string(e, name);
        e.call(0x0085_03b0, &args![rig.manager, name, device, stats])
            .bool()
    }

    #[test]
    fn a_named_save_writes_the_file_and_counts_it() {
        let mut e = engine();
        let rig = rig(&mut e);
        start_log(&mut e);
        assert!(save(&mut e, &rig, "Mojave 1", 3, false));
        let log = take_log(&mut e);
        assert_eq!(*rig.opens.borrow(), vec![("Mojave 1".to_string(), 1, 2, 3)]);
        // Order: expired data, pause, open, header, save, resume, close,
        // history rewind.
        let steps = [
            GAME_CLEANUP_EXPIRED_DATA,
            0x006e_bc70,
            OPEN_SAVE_FILE,
            GAME_SET_FLAG,
            SAVE_HEADER,
            GAME_SAVE,
            0x006e_bcd0,
            SAVE_FILE_DESTRUCT,
            HISTORY_REWIND,
        ];
        let mut seen = log.iter().map(|(a, _)| *a);
        for step in steps {
            assert!(
                seen.any(|a| a == step),
                "step {step:08x} missing or out of order"
            );
        }
        assert_eq!(calls(&log, SAVE_HEADER), vec![vec![rig.file]]);
        assert_eq!(calls(&log, GAME_SAVE), vec![vec![rig.game, rig.file]]);
        assert_eq!(calls(&log, HISTORY_REWIND), vec![vec![rig.game + 0x1c, 42]]);
        // The temporary file is moved into place.
        assert_eq!(
            *rig.files.events.borrow(),
            vec!["rename saves\\slot.fos.tmp saves\\slot.fos"]
        );
        assert!(rig.notices.borrow().is_empty());
        // Counted: number, list, most recent name.
        assert_eq!(
            e.get(rig.manager, BGSSaveLoadManager::iCurrentSaveGameNumber),
            6
        );
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::iSaveGameCount), 1);
        let recent = e.get(rig.manager, BGSSaveLoadManager::pMostRecentSaveGame);
        assert_eq!(text(&e, recent.addr()), "Mojave 1");
        assert_eq!(
            e.get(rig.manager, BGSSaveLoadManager::iMostRecentSaveGameDeviceID),
            3
        );
    }

    #[test]
    fn quicksaves_and_autosaves_do_not_advance_the_save_number() {
        for (name, quiet) in [("quicksave", false), ("autosave", true)] {
            let mut e = engine();
            let rig = rig(&mut e);
            e.set(
                rig.manager,
                BGSSaveLoadManager::bAutosaveDisabledForDiskspace,
                true,
            );
            assert!(save(&mut e, &rig, name, u32::MAX, false));
            assert_eq!(
                e.get(rig.manager, BGSSaveLoadManager::iCurrentSaveGameNumber),
                5
            );
            // The entry is still listed and remembered.
            assert_eq!(e.get(rig.manager, BGSSaveLoadManager::iSaveGameCount), 1);
            let recent = e.get(rig.manager, BGSSaveLoadManager::pMostRecentSaveGame);
            assert_eq!(text(&e, recent.addr()), name);
            // An autosave (quiet) announces itself with the neutral icon and
            // re-enables itself; a quicksave says nothing here.
            let notices = rig.notices.borrow();
            if quiet {
                assert_eq!(*notices, vec![("saving".to_string(), ICON_NEUTRAL, 1)]);
                assert!(!e.get(
                    rig.manager,
                    BGSSaveLoadManager::bAutosaveDisabledForDiskspace
                ));
            } else {
                assert!(notices.is_empty());
            }
        }
    }

    #[test]
    fn a_forced_save_is_quiet_too() {
        let mut e = engine();
        let rig = rig(&mut e);
        e.set(rig.manager, BGSSaveLoadManager::cQueuedForceSave, 1);
        assert!(save(&mut e, &rig, "Mojave 2", u32::MAX, false));
        assert_eq!(rig.notices.borrow().len(), 1);
        assert_eq!(
            e.get(rig.manager, BGSSaveLoadManager::iCurrentSaveGameNumber),
            6
        );
    }

    #[test]
    fn a_save_that_may_not_run_is_refused_with_a_notice() {
        let mut e = engine();
        let rig = rig(&mut e);
        let player = e.mem.alloc(16);
        e.mem.set_u32(player, 2);
        e.set_global(PLAYER, player);
        e.register(PLAYER_STATE, |e, a| rv(e.mem.u32(a[0])));
        start_log(&mut e);
        assert!(!save(&mut e, &rig, "Mojave 1", 3, false));
        let log = take_log(&mut e);
        assert_eq!(
            *rig.notices.borrow(),
            vec![("cannot save".to_string(), ICON_SAD, 0)]
        );
        assert!(calls(&log, OPEN_SAVE_FILE).is_empty());
        // Autosaves and system saves stay silent.
        assert!(!save(&mut e, &rig, "autosave", 3, false));
        assert!(!save(&mut e, &rig, "systemsave Courier", 3, false));
        assert_eq!(rig.notices.borrow().len(), 1);
    }

    #[test]
    fn a_save_without_a_file_gives_up_and_resumes() {
        let mut e = engine();
        let rig = rig(&mut e);
        e.register(OPEN_SAVE_FILE, |_, _| Ret::default());
        start_log(&mut e);
        assert!(!save(&mut e, &rig, "Mojave 1", 3, false));
        let log = take_log(&mut e);
        assert_eq!(calls(&log, 0x006e_bcd0).len(), 1);
        assert!(calls(&log, SAVE_HEADER).is_empty());
        assert_eq!(
            e.get(rig.manager, BGSSaveLoadManager::iCurrentSaveGameNumber),
            5
        );
    }

    #[test]
    fn a_failed_save_clears_the_flag_and_reports() {
        let mut e = engine();
        let rig = rig(&mut e);
        e.register(GAME_FLAG_IS_SET, |_, _| rv(1));
        start_log(&mut e);
        assert!(!save(&mut e, &rig, "Mojave 1", 3, false));
        let log = take_log(&mut e);
        assert_eq!(
            calls(&log, GAME_SET_FLAG),
            vec![vec![rig.game, 1], vec![rig.game, 0]]
        );
        assert_eq!(
            *rig.notices.borrow(),
            vec![("save failed".to_string(), ICON_SAD, 0)]
        );
        // Nothing is rotated or counted.
        assert!(calls(&log, SAVE_FILE_DESTRUCT).is_empty());
        assert!(rig.files.events.borrow().is_empty());
        assert_eq!(
            e.get(rig.manager, BGSSaveLoadManager::iCurrentSaveGameNumber),
            5
        );
        assert_eq!(calls(&log, 0x006e_bcd0).len(), 1);
    }

    #[test]
    fn a_simulated_file_is_reused_and_not_closed() {
        let mut e = engine();
        let rig = rig(&mut e);
        e.set(
            rig.manager,
            BGSSaveLoadManager::pSaveLoadFile,
            Ptr::new(rig.file),
        );
        start_log(&mut e);
        assert!(save(&mut e, &rig, "Mojave 1", 3, false));
        let log = take_log(&mut e);
        assert!(calls(&log, OPEN_SAVE_FILE).is_empty());
        assert!(calls(&log, SAVE_FILE_DESTRUCT).is_empty());
        // No list entry either; the number still counts.
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::iSaveGameCount), 0);
        assert_eq!(
            e.get(rig.manager, BGSSaveLoadManager::iCurrentSaveGameNumber),
            6
        );
    }

    #[test]
    fn a_save_can_write_the_statistics_file() {
        let mut e = engine();
        let rig = rig(&mut e);
        start_log(&mut e);
        assert!(save(&mut e, &rig, "Mojave 1", 3, true));
        let log = take_log(&mut e);
        let stats = calls(&log, OUTPUT_STATISTICS);
        assert_eq!(stats.len(), 1);
        // The statistics path was built by a second `XContentClose` without
        // the temporary suffix.
        let builds = calls(&log, CONTENT_CLOSE);
        assert_eq!(builds.len(), 2);
        assert_eq!(builds[1][3], 0);
        assert_eq!(builds[1][2], stats[0][0]);
    }

    #[test]
    fn saving_without_a_name_generates_one() {
        let mut e = engine();
        let rig = rig(&mut e);
        let player = e.mem.alloc(16);
        e.set_global(PLAYER, player);
        let prefix = string(&mut e, "Save");
        set_setting(&mut e, NAME_PREFIX_GENERATED, prefix);
        let courier = string(&mut e, "Ed");
        e.register_double(PLAYER_NAME, move |_, _| rv(courier));
        e.register(BSSTRING_DATA, |e, a| rv(e.mem.u32(a[0])));
        e.register(PLAY_TIME_MS, |_, _| rv(61_000));
        let name = 0u32;
        assert!(e
            .call(0x0085_03b0, &args![rig.manager, name, 7u32, false])
            .bool());
        assert_eq!(rig.opens.borrow()[0].0, "Save 5   Ed    00 01 01");
    }

    #[test]
    fn the_first_save_reads_the_folder_for_the_next_number() {
        let mut e = engine();
        let rig = rig(&mut e);
        e.set(rig.manager, BGSSaveLoadManager::iCurrentSaveGameNumber, 0);
        fake_folder(&mut e, &[("Save 9.fos", 10)]);
        entry_recorder(&mut e);
        e.register(FILE_ENTRY_SAVE_NUMBER, |_, _| rv(9));
        start_log(&mut e);
        assert!(save(&mut e, &rig, "Mojave 1", 3, false));
        let log = take_log(&mut e);
        // The folder scan ran, and its list was cleared again at once; the
        // number from it (9, plus 1) then counts this save.
        assert_eq!(calls(&log, FIND_FIRST_FILE).len(), 1);
        assert_eq!(calls(&log, LIST_CLEAR).len(), 2);
        assert_eq!(
            e.get(rig.manager, BGSSaveLoadManager::iCurrentSaveGameNumber),
            11
        );
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::iSaveGameCount), 0);
    }

    #[test]
    fn quick_save_announces_itself_and_saves_as_quicksave() {
        let mut e = engine();
        let rig = rig(&mut e);
        e.call(0x0085_09a0, &args![rig.manager]);
        assert_eq!(
            *rig.notices.borrow(),
            vec![("quick saving".to_string(), ICON_NEUTRAL, 0)]
        );
        assert_eq!(
            *rig.opens.borrow(),
            vec![("quicksave".to_string(), 1, 2, u32::MAX)]
        );
    }

    #[test]
    fn auto_save_clears_its_flags_and_updates_the_sky() {
        let mut e = engine();
        let rig = rig(&mut e);
        e.set(rig.manager, BGSSaveLoadManager::cQueuedAutosave, 1);
        e.set(rig.manager, BGSSaveLoadManager::bhandledCorrupt, true);
        e.set_global(WORLD, 0x1010u32);
        e.register(FRAME_SECONDS, |_, _| Ret {
            st0: 0.25,
            ..Ret::default()
        });
        e.register(SKY_OWNER, |_, a| rv(a[0] + 1));
        start_log(&mut e);
        e.call(0x0085_0a40, &args![rig.manager]);
        let log = take_log(&mut e);
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedAutosave), 0);
        assert!(!e.get(rig.manager, BGSSaveLoadManager::bhandledCorrupt));
        assert_eq!(
            calls(&log, SKY_UPDATE),
            vec![vec![0x1011, 0.25f32.to_bits()]]
        );
        assert_eq!(
            rig.opens.borrow()[0],
            ("autosave".to_string(), 1, 2, u32::MAX)
        );
    }

    #[test]
    fn system_save_names_itself_and_remembers_the_name_once() {
        let mut e = engine();
        let rig = rig(&mut e);
        e.set(rig.manager, BGSSaveLoadManager::cQueuedSystemSave, 1);
        let player = e.mem.alloc(16);
        e.set_global(PLAYER, player);
        let name = string(&mut e, "Ed");
        e.register_double(PLAYER_NAME, move |_, _| rv(name));
        e.register(FRAME_SECONDS, |_, _| Ret::default());
        start_log(&mut e);
        e.call(0x0085_0a90, &args![rig.manager]);
        let log = take_log(&mut e);
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedSystemSave), 0);
        assert_eq!(
            rig.opens.borrow()[0],
            ("systemsave Ed".to_string(), 1, 2, u32::MAX)
        );
        // The stored name was empty, so it is set from the made-up one.
        let stored = rig.manager.addr() + 0x30;
        let sets = calls(&log, BSSTRING_SET);
        assert_eq!(sets.len(), 1);
        assert_eq!(sets[0][0], stored);
        assert_eq!(sets[0][2], 0);

        // A name that is already stored is kept.
        e.register(BSSTRING_LENGTH, |_, _| rv(5));
        // (The first save deleted its file object, so the next open is a new one.)
        let next_file = e.mem.alloc(0x110);
        put(&mut e, next_file, "slot.fos");
        rig.files.present.borrow_mut().clear();
        rig.files
            .present
            .borrow_mut()
            .insert("saves\\slot.fos.tmp".to_string());
        e.register_double(OPEN_SAVE_FILE, move |_, _| rv(next_file));
        start_log(&mut e);
        e.call(0x0085_0a90, &args![rig.manager]);
        assert!(calls(&take_log(&mut e), BSSTRING_SET).is_empty());
    }

    // ---- Loading ------------------------------------------------------------

    /// The load flow's doubles on top of the shared rig; returns the
    /// recorded `BGSSaveLoadGame::LoadGame` calls: (file, quiet, whether the
    /// manager said it was loading from the start menu at the time).
    fn load_rig(e: &mut Engine, rig: &Rig) -> Loads {
        e.register(LOAD_HEADER, |_, _| rv(0x30));
        e.register(SAVE_FILE_IS_OPEN, |_, _| rv(1));
        let byte = e.mem.alloc(8);
        e.mem.set_u8(byte, 1);
        e.register_double(0x0040_8d60, move |_, _| rv(byte));
        let loads: Loads = Rc::default();
        let seen = loads.clone();
        let manager = rig.manager;
        e.register_double(GAME_LOAD, move |e, a| {
            let loading = e.get(manager, BGSSaveLoadManager::bStartMenuLoading);
            seen.borrow_mut().push((a[1], a[2], loading));
            rv(1)
        });
        e.set_global(MAIN, 0x4040u32);
        let fade = e.mem.alloc(4);
        set_fade(e, fade);
        loads
    }

    fn load(e: &mut Engine, rig: &Rig, name: &str, device: u32, stats: bool, quiet: u8) -> bool {
        let name = string(e, name);
        e.call(0x0085_0760, &args![rig.manager, name, device, stats, quiet])
            .bool()
    }

    #[test]
    fn a_good_load_rebuilds_the_world_and_remembers_the_save() {
        let mut e = engine();
        let rig = rig(&mut e);
        let loads = load_rig(&mut e, &rig);
        start_log(&mut e);
        assert!(load(&mut e, &rig, "Mojave 1", 3, false, 0));
        let log = take_log(&mut e);
        assert_eq!(*rig.opens.borrow(), vec![("Mojave 1".to_string(), 0, 1, 3)]);
        // The game loaded the file with the manager in "start menu loading".
        assert_eq!(*loads.borrow(), vec![(rig.file, 0, true)]);
        assert!(!e.get(rig.manager, BGSSaveLoadManager::bStartMenuLoading));
        assert_eq!(calls(&log, 0x005f_5880).len(), 1);
        // The file object is deleted without rotation; the menu background
        // texture killed; the name remembered.
        assert_eq!(calls(&log, SAVE_FILE_DESTRUCT), vec![vec![rig.file]]);
        assert!(calls(&log, CONTENT_CLOSE).is_empty());
        assert_eq!(calls(&log, KILL_MENU_BG_TEXTURE), vec![vec![0x4040]]);
        let recent = e.get(rig.manager, BGSSaveLoadManager::pMostRecentSaveGame);
        assert_eq!(text(&e, recent.addr()), "Mojave 1");
        assert_eq!(
            e.get(rig.manager, BGSSaveLoadManager::iMostRecentSaveGameDeviceID),
            3
        );
    }

    #[test]
    fn a_load_is_quiet_when_the_setting_byte_is_zero() {
        let mut e = engine();
        let rig = rig(&mut e);
        let loads = load_rig(&mut e, &rig);
        let byte = e.mem.alloc(8);
        e.register_double(0x0040_8d60, move |_, _| rv(byte));
        assert!(load(&mut e, &rig, "Mojave 1", 3, false, 0));
        assert_eq!(loads.borrow()[0].1, 1);
    }

    #[test]
    fn a_load_can_write_the_statistics_file() {
        let mut e = engine();
        let rig = rig(&mut e);
        load_rig(&mut e, &rig);
        start_log(&mut e);
        assert!(load(&mut e, &rig, "Mojave 1", 3, true, 0));
        let log = take_log(&mut e);
        assert_eq!(calls(&log, OUTPUT_STATISTICS).len(), 1);
        assert_eq!(calls(&log, CONTENT_CLOSE).len(), 1);
    }

    #[test]
    fn a_failed_load_keeps_the_name_and_offers_a_retry() {
        let mut e = engine();
        let rig = rig(&mut e);
        load_rig(&mut e, &rig);
        e.register(GAME_LOAD, |_, _| Ret::default());
        start_log(&mut e);
        assert!(!load(&mut e, &rig, "Mojave 1", 3, true, 0));
        let log = take_log(&mut e);
        assert_eq!(text(&e, RETRY_NAME), "Mojave 1");
        assert_eq!(e.global::<u32>(RETRY_DEVICE_ID), 3);
        assert_eq!(e.global::<u8>(RETRY_FLAG), 1);
        let boxes = rig.boxes.borrow();
        assert_eq!(boxes.len(), 1);
        let words = &boxes[0].1;
        assert_eq!(boxes[0].0, "load failed");
        // Callback, one, type 0x17, no size, "Yes", "No", end of buttons.
        assert_eq!(words[3], 0x0085_0720);
        assert_eq!(words[4], 1);
        assert_eq!(words[5], 0x17);
        assert_eq!(text(&e, words[8]), "Yes");
        assert_eq!(text(&e, words[9]), "No");
        assert_eq!(words[10], 0);
        assert_eq!(words.len(), 11);
        // The world is not rebuilt, the name not remembered, the file closed.
        assert!(calls(&log, 0x005f_5880).is_empty());
        assert!(calls(&log, OUTPUT_STATISTICS).is_empty());
        assert!(e
            .get(rig.manager, BGSSaveLoadManager::pMostRecentSaveGame)
            .is_null());
        assert_eq!(calls(&log, SAVE_FILE_DESTRUCT).len(), 1);
    }

    #[test]
    fn a_load_with_a_bad_header_stops_there() {
        let mut e = engine();
        let rig = rig(&mut e);
        let loads = load_rig(&mut e, &rig);
        e.register(LOAD_HEADER, |_, _| rv(0x20));
        let ok = string(&mut e, "OK");
        set_setting(&mut e, MESSAGE_SETTING_BUTTON_OK, ok);
        assert!(!load(&mut e, &rig, "Mojave 1", 3, false, 0));
        assert!(loads.borrow().is_empty());
        // The header check raised its own box, not the retry one.
        assert_eq!(rig.boxes.borrow().len(), 1);
        assert!(rig.boxes.borrow()[0].0.starts_with("outdated"));
        assert_eq!(e.global::<u32>(RETRY_DEVICE_ID), 0);
    }

    #[test]
    fn a_load_of_a_file_that_is_not_open_does_not_read_it() {
        let mut e = engine();
        let rig = rig(&mut e);
        let loads = load_rig(&mut e, &rig);
        e.register(SAVE_FILE_IS_OPEN, |_, _| Ret::default());
        start_log(&mut e);
        assert!(!load(&mut e, &rig, "Mojave 1", 3, false, 0));
        let log = take_log(&mut e);
        assert!(loads.borrow().is_empty());
        assert!(calls(&log, LOAD_HEADER).is_empty());
        assert_eq!(calls(&log, SAVE_FILE_DESTRUCT).len(), 1);
    }

    #[test]
    fn a_load_without_a_file_only_kills_the_menu_texture() {
        let mut e = engine();
        let rig = rig(&mut e);
        load_rig(&mut e, &rig);
        e.register(OPEN_SAVE_FILE, |_, _| Ret::default());
        start_log(&mut e);
        assert!(!load(&mut e, &rig, "Mojave 1", 3, false, 0));
        let log = take_log(&mut e);
        let others: Vec<u32> = order(&log);
        assert_eq!(
            others,
            vec![STRING_COPY, OPEN_SAVE_FILE, KILL_MENU_BG_TEXTURE]
        );
    }

    #[test]
    fn a_load_reuses_the_simulated_file_and_leaves_it_open() {
        let mut e = engine();
        let rig = rig(&mut e);
        load_rig(&mut e, &rig);
        e.set(
            rig.manager,
            BGSSaveLoadManager::pSaveLoadFile,
            Ptr::new(rig.file),
        );
        start_log(&mut e);
        assert!(load(&mut e, &rig, "Mojave 1", 3, false, 0));
        let log = take_log(&mut e);
        assert!(calls(&log, OPEN_SAVE_FILE).is_empty());
        assert!(calls(&log, SAVE_FILE_DESTRUCT).is_empty());
    }

    #[test]
    fn a_load_brings_the_console_back_if_it_was_open() {
        let mut e = engine();
        let rig = rig(&mut e);
        load_rig(&mut e, &rig);
        e.register(CONSOLE_IS_VISIBLE, |_, _| rv(1));
        e.register(INTERFACE_GET, |_, _| rv(0x4444));
        e.set_global(INTERFACE_MANAGER, 0x3333u32);
        start_log(&mut e);
        assert!(load(&mut e, &rig, "Mojave 1", 3, false, 0));
        let log = take_log(&mut e);
        assert_eq!(calls(&log, CONSOLE_OPEN).len(), 1);
        assert_eq!(calls(&log, CONSOLE_REFRESH), vec![vec![0x4444, 0x3333]]);
    }

    #[test]
    fn loading_the_most_recent_save_needs_a_remembered_name() {
        let mut e = engine();
        let rig = rig(&mut e);
        let loads = load_rig(&mut e, &rig);
        assert!(!e.call(0x0085_12f0, &args![rig.manager]).bool());
        assert!(rig.opens.borrow().is_empty());

        let name = string(&mut e, "Mojave 1");
        e.set(
            rig.manager,
            BGSSaveLoadManager::pMostRecentSaveGame,
            Ptr::new(name),
        );
        e.set(
            rig.manager,
            BGSSaveLoadManager::iMostRecentSaveGameDeviceID,
            6,
        );
        assert!(e.call(0x0085_12f0, &args![rig.manager]).bool());
        assert_eq!(*rig.opens.borrow(), vec![("Mojave 1".to_string(), 0, 1, 6)]);
        // Quiet (1), and no statistics file.
        assert_eq!(loads.borrow()[0].1, 1);
    }

    #[test]
    fn quick_load_announces_itself_and_loads_quicksave() {
        let mut e = engine();
        let rig = rig(&mut e);
        let loads = load_rig(&mut e, &rig);
        e.call(0x0085_09f0, &args![rig.manager]);
        assert_eq!(
            *rig.notices.borrow(),
            vec![("quick loading".to_string(), ICON_NEUTRAL, 0)]
        );
        assert_eq!(
            *rig.opens.borrow(),
            vec![("quicksave".to_string(), 0, 1, u32::MAX)]
        );
        // The quiet flag passed is 1.
        assert_eq!(loads.borrow()[0].1, 1);
    }

    #[test]
    fn the_retry_box_callback_loads_again_only_on_yes() {
        let mut e = engine();
        let rig = rig(&mut e);
        let loads = load_rig(&mut e, &rig);
        put(&mut e, RETRY_NAME, "retry me");
        e.set_global(RETRY_DEVICE_ID, 4u32);
        e.set_global(RETRY_FLAG, 1u8);
        e.register(MESSAGE_BOX_RESULT, |_, _| rv(0));
        e.call(0x0085_0720, &args![]);
        assert!(rig.opens.borrow().is_empty());
        e.register(MESSAGE_BOX_RESULT, |_, _| rv(1));
        e.call(0x0085_0720, &args![]);
        assert_eq!(*rig.opens.borrow(), vec![("retry me".to_string(), 0, 1, 4)]);
        assert_eq!(loads.borrow().len(), 1);
        // Loading again writes the statistics (flag 1) and passes quiet = 1.
        assert_eq!(loads.borrow()[0].1, 1);
    }

    // ---- Second part: queued saves, entry getters and CopySaveGames ----------

    #[test]
    fn play_time_adds_the_time_since_the_mark_and_moves_the_mark() {
        let mut e = engine();
        let player = e.mem.alloc(0x800);
        e.mem.set_u32(player + 0x78c, 400);
        e.mem.set_u32(player + 0x790, 50);
        e.register(TICK_COUNT, |_, _| rv(1000));
        assert_eq!(fn_00851cb0(&mut e, Ptr::new(player)), 650);
        assert_eq!(e.mem.u32(player + 0x790), 650);
        assert_eq!(e.mem.u32(player + 0x78c), 1000);
        // The mark alone is the current tick.
        e.mem.set_u32(player + 0x78c, 0);
        fn_00851d10(&mut e, Ptr::new(player));
        assert_eq!(e.mem.u32(player + 0x78c), 1000);
        assert_eq!(e.mem.u32(player + 0x790), 650);
    }

    #[test]
    fn the_queue_setters_each_set_their_own_byte() {
        let mut e = engine();
        let manager = manager(&mut e);
        e.call(0x0085_1d30, &args![manager]);
        assert_eq!(e.get(manager, BGSSaveLoadManager::cQueuedAutosave), 1);
        assert_eq!(e.get(manager, BGSSaveLoadManager::cQueuedForceSave), 0);
        e.call(0x0085_1d50, &args![manager]);
        assert_eq!(e.get(manager, BGSSaveLoadManager::cQueuedForceSave), 1);
        assert_eq!(e.get(manager, BGSSaveLoadManager::cQueuedSystemSave), 0);
        e.call(0x0085_1d70, &args![manager]);
        assert_eq!(e.get(manager, BGSSaveLoadManager::cQueuedSystemSave), 1);
    }

    type Blockers = (Rc<std::cell::Cell<u32>>, Rc<std::cell::Cell<bool>>);

    /// Blockers for the queued saves: the fader kind that is active (0 for
    /// none) and whether the loading menu is open.
    fn blockers(e: &mut Engine) -> Blockers {
        let kind = Rc::new(std::cell::Cell::new(0u32));
        let menu = Rc::new(std::cell::Cell::new(false));
        e.set_global(FADER_MANAGER, 0x6060u32);
        let seen = kind.clone();
        e.register_double(FADER_ACTIVE, move |_, a| {
            assert_eq!(a[0], 0x6060);
            rv((a[1] == seen.get()) as u32)
        });
        let seen = menu.clone();
        e.register_double(LOADING_MENU_OPEN, move |_, _| rv(seen.get() as u32));
        (kind, menu)
    }

    #[test]
    fn a_queued_autosave_counts_down_and_waits_for_the_blockers() {
        let mut e = engine();
        let rig = rig(&mut e);
        let (fader, menu) = blockers(&mut e);
        e.register(FRAME_SECONDS, |_, _| Ret::default());
        e.register(SKY_OWNER, |_, a| rv(a[0]));
        e.set(rig.manager, BGSSaveLoadManager::cQueuedAutosave, 3);
        e.call(0x0085_1d90, &args![rig.manager]);
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedAutosave), 2);
        e.call(0x0085_1d90, &args![rig.manager]);
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedAutosave), 1);
        // At 1 it stays queued while either fader kind or the menu is active.
        for (kind, open) in [(1, false), (2, false), (0, true)] {
            fader.set(kind);
            menu.set(open);
            e.call(0x0085_1d90, &args![rig.manager]);
            assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedAutosave), 1);
            assert!(rig.opens.borrow().is_empty());
        }
        fader.set(0);
        menu.set(false);
        e.call(0x0085_1d90, &args![rig.manager]);
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedAutosave), 0);
        assert_eq!(
            rig.opens.borrow()[0],
            ("autosave".to_string(), 1, 2, u32::MAX)
        );
    }

    #[test]
    fn a_queued_forced_save_saves_with_no_name_and_clears_its_byte() {
        let mut e = engine();
        let rig = rig(&mut e);
        let (_fader, menu) = blockers(&mut e);
        let player = e.mem.alloc(16);
        e.set_global(PLAYER, player);
        let prefix = string(&mut e, "Save");
        set_setting(&mut e, NAME_PREFIX_GENERATED, prefix);
        let courier = string(&mut e, "Ed");
        e.register_double(PLAYER_NAME, move |_, _| rv(courier));
        e.register(BSSTRING_DATA, |e, a| rv(e.mem.u32(a[0])));
        e.register(PLAY_TIME_MS, |_, _| rv(61_000));
        e.set(rig.manager, BGSSaveLoadManager::cQueuedForceSave, 2);
        e.call(0x0085_1d90, &args![rig.manager]);
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedForceSave), 1);
        menu.set(true);
        e.call(0x0085_1d90, &args![rig.manager]);
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedForceSave), 1);
        assert!(rig.opens.borrow().is_empty());
        menu.set(false);
        e.call(0x0085_1d90, &args![rig.manager]);
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedForceSave), 0);
        assert_eq!(rig.opens.borrow()[0].0, "Save 5   Ed    00 01 01");
    }

    #[test]
    fn a_queued_system_save_runs_and_only_the_first_queue_counts() {
        let mut e = engine();
        let rig = rig(&mut e);
        blockers(&mut e);
        let player = e.mem.alloc(16);
        e.set_global(PLAYER, player);
        let name = string(&mut e, "Ed");
        e.register_double(PLAYER_NAME, move |_, _| rv(name));
        e.register(FRAME_SECONDS, |_, _| Ret::default());
        e.set(rig.manager, BGSSaveLoadManager::cQueuedSystemSave, 2);
        e.call(0x0085_1d90, &args![rig.manager]);
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedSystemSave), 1);
        e.call(0x0085_1d90, &args![rig.manager]);
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedSystemSave), 0);
        assert_eq!(rig.opens.borrow().len(), 1);

        // With an autosave queued as well, only the autosave counter moves.
        e.set(rig.manager, BGSSaveLoadManager::cQueuedAutosave, 5);
        e.set(rig.manager, BGSSaveLoadManager::cQueuedForceSave, 5);
        e.set(rig.manager, BGSSaveLoadManager::cQueuedSystemSave, 5);
        e.call(0x0085_1d90, &args![rig.manager]);
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedAutosave), 4);
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedForceSave), 5);
        assert_eq!(e.get(rig.manager, BGSSaveLoadManager::cQueuedSystemSave), 5);
    }

    #[test]
    fn the_entry_getters_load_the_entry_first() {
        let mut e = engine();
        let entry = e.mem.alloc(0x7c);
        e.mem.set_u32(entry + 0x20, 11);
        e.mem.set_u32(entry + 0x24, 22);
        e.mem.set_u32(entry + 0x28, 33);
        start_log(&mut e);
        assert_eq!(e.call(0x0085_1ef0, &args![entry]).u32(), 11);
        assert_eq!(e.call(0x0085_1f10, &args![entry]).u32(), 22);
        assert_eq!(e.call(0x0085_1f30, &args![entry]).u32(), 33);
        let log = take_log(&mut e);
        assert_eq!(calls(&log, FILE_ENTRY_LOAD_DATA), vec![vec![entry]; 3]);
    }

    #[test]
    fn the_game_data_utility_getter_returns_the_static_object() {
        let mut e = engine();
        e.register(GAME_DATA_UTILITY, |_, _| rv(0x011f_72a0));
        assert_eq!(e.call(0x0085_1f50, &args![0x1234u32]).u32(), 0x011f_72a0);
    }

    #[test]
    fn the_buffer_setters_fill_their_fields() {
        let mut e = engine();
        let buffer = e.mem.alloc(0xa00);
        for (setter, offset, size, written) in [
            (0x0085_1f60u32, 0x164u32, 0x100u32, "-SAVE-"),
            (0x0085_1f90, 0x265, 0x80, "Location"),
            (0x0085_1fc0, 0x2e5, 0x400, "a description"),
            (0x0085_2030, 0x8f4, 0x100, "DATA/SYSUTIL/SAVEBACK.PNG"),
        ] {
            let source = string(&mut e, written);
            start_log(&mut e);
            e.call(setter, &args![buffer, source]);
            let log = take_log(&mut e);
            assert_eq!(
                calls(&log, STRING_COPY),
                vec![vec![buffer + offset, size, source]]
            );
            assert_eq!(text(&e, buffer + offset), written);
        }
        e.call(0x0085_1ff0, &args![buffer, 1u32, 2u32, 3u32]);
        assert_eq!(
            [0x7e8, 0x7ec, 0x7f0].map(|o| e.mem.u32(buffer + o)),
            [1, 2, 3]
        );
        e.call(0x0085_2060, &args![buffer, 0x0085_2740u32]);
        assert_eq!(e.mem.u32(buffer + 0x9f4), 0x0085_2740);
    }

    /// Everything `CopySaveGames` observably does, recorded by doubles.
    #[derive(Default)]
    struct CopyLog {
        /// The guard object's byte as set, and 0xdead when the walker is
        /// destroyed.
        guard_sets: Vec<u32>,
        folder: String,
        paths: Vec<String>,
        failures: Vec<(u32, String)>,
        destructs: u32,
        reads: Vec<Vec<u32>>,
        dialog: Vec<(String, u32)>,
        started: Vec<u32>,
        sleeps: u32,
        busy_polls: u32,
    }

    const COPY_UTILITY: u32 = 0x00ab_0000;
    const COPY_DIALOG: u32 = 0x00ab_0100;
    const COPY_GAME_DATA: u32 = 0x00ab_0200;
    const COPY_BUFFER_OBJECT: u32 = 0x00ab_1000;
    const COPY_DATA_BLOCK: u32 = 0x00ab_3000;

    /// A heap string (for doubles that cannot capture).
    fn string_at(e: &mut Engine, s: &str) -> u32 {
        string(e, s)
    }

    /// Doubles for the whole of `CopySaveGames`: a folder holding `files`
    /// (name, is a plain file), the file layer (a file whose path contains
    /// `unreadable` fails to open; with `read_fails` every read fails), and
    /// the dialog and game-data singletons. `busy` is how many polls report
    /// the operation still running.
    fn copy_rig(
        e: &mut Engine,
        files: &[(&str, bool)],
        unreadable: &'static str,
        read_fails: bool,
        busy: u32,
    ) -> Rc<RefCell<CopyLog>> {
        let log = Rc::new(RefCell::new(CopyLog::default()));
        e.map(0x00ab_0000, 0x4000);
        e.map(0x00ac_0000, 0x3000);
        e.map(COPY_GUARD_OBJECT, 0x100);
        let guard_byte = e.mem.alloc(8);
        e.mem.set_u8(guard_byte, 7);
        e.register_double(GUARD_DATA, move |_, a| {
            assert_eq!(a[0], COPY_GUARD_OBJECT);
            rv(guard_byte)
        });
        let seen = log.clone();
        e.register_double(GUARD_SET, move |_, a| {
            assert_eq!(a[0], COPY_GUARD_OBJECT);
            seen.borrow_mut().guard_sets.push(a[1]);
            Ret::default()
        });

        // The folder walker.
        let seen = log.clone();
        e.register_double(DIRECTORY_CONSTRUCT, move |e, a| {
            seen.borrow_mut().folder = text(e, a[1]);
            Ret::default()
        });
        let pending: Rc<RefCell<Vec<(String, bool)>>> = Rc::new(RefCell::new(
            files
                .iter()
                .rev()
                .map(|(n, f)| (n.to_string(), *f))
                .collect(),
        ));
        e.register_double(DIRECTORY_NEXT, move |e, a| {
            match pending.borrow_mut().pop() {
                None => e.mem.set_u8(a[1], 1),
                Some((name, is_file)) => {
                    e.mem.set_u32(a[0] + 4, if is_file { 0x20 } else { 0x10 });
                    put(e, a[0] + 0x30, &name);
                }
            }
            Ret::default()
        });
        e.register(DIRECTORY_IS_FILE, |e, a| {
            rv((e.mem.u32(a[0] + 4) & 0x20 != 0) as u32)
        });
        e.register(DIRECTORY_FILE_NAME, |_, a| rv(a[0] + 0x30));
        let seen = log.clone();
        e.register_double(DIRECTORY_DESTRUCT, move |_, _| {
            seen.borrow_mut().guard_sets.push(0xdead);
            Ret::default()
        });

        // The file layer.
        let seen = log.clone();
        e.register_double(FILE_CONSTRUCT, move |e, a| {
            let path = text(e, a[1]);
            e.mem
                .set_u32(a[0], if path.contains(unreadable) { 0x80 } else { 0 });
            seen.borrow_mut().paths.push(path);
            Ret::default()
        });
        let reading = Rc::new(std::cell::Cell::new(false));
        let flag = reading.clone();
        e.register_double(FILE_OPEN_RESULT, move |e, a| {
            let code = e.mem.u32(a[0]);
            rv(if code == 0 && read_fails && flag.get() {
                5
            } else {
                code
            })
        });
        e.register_double(FILE_GET_SIZE, |e, a| {
            e.mem.set_u32(a[1], 0x40);
            e.mem.set_u32(a[1] + 4, 0);
            Ret::default()
        });
        let seen = log.clone();
        e.register_double(FILE_READ, move |_, a| {
            seen.borrow_mut().reads.push(a.to_vec());
            reading.set(true);
            Ret::default()
        });
        let seen = log.clone();
        e.register_double(FILE_DESTRUCT, move |_, _| {
            seen.borrow_mut().destructs += 1;
            Ret::default()
        });
        let seen = log.clone();
        e.register_double(DEBUG_PRINT, move |e, a| {
            seen.borrow_mut().failures.push((a[0], text(e, a[1])));
            Ret::default()
        });

        // The save file and its buffer object.
        e.map(COPY_BUFFER_OBJECT, 0x2000);
        e.mem.set_u32(COPY_BUFFER_OBJECT + 0x15c, COPY_DATA_BLOCK);
        e.map(COPY_DATA_BLOCK, 0x1000);
        e.register(SAVE_LOAD_FILE_CONSTRUCT, |e, a| {
            assert_eq!(text(e, a[1]), "SaveData");
            assert_eq!((a[2], a[3]), (0, 0x40));
            e.mem.set_u32(a[0] + 0x104, COPY_BUFFER_OBJECT);
            rv(a[0])
        });
        e.register(SAVE_FILE_BUFFER, |e, a| rv(e.mem.u32(a[0] + 0x104)));
        e.register(BUFFER_DATA, |e, a| rv(e.mem.u32(a[0] + 0x15c)));
        e.register(FILE_ENTRY_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0x20, 1);
            e.mem.set_u32(a[0] + 0x24, 2);
            e.mem.set_u32(a[0] + 0x28, 3);
            rv(a[0])
        });

        // The entry getters and the settings behind the description.
        let location = string(e, "Novac");
        e.register_double(ENTRY_LOCATION, move |_, _| rv(location));
        e.register(ENTRY_LEVEL, |_, _| rv(12));
        for (setting, s) in [
            (SETTING_PLAYER_NAME, "Courier"),
            (SETTING_CELL_NAME, "Cottonwood"),
            (SETTING_LEVEL_LABEL, "Level"),
        ] {
            let s = string(e, s);
            set_setting(e, setting, s);
        }
        for (addr, s) in [
            (COPY_FOLDER, "Saves"),
            (COPY_SLASH, "/"),
            (COPY_PATH_FORMAT, "%s%s"),
            (COPY_BUFFER_NAME, "SaveData"),
            (COPY_TITLE, "-SAVE-"),
            (COPY_LOCATION, "Location"),
            (COPY_DASH, "-"),
            (COPY_TEXT_FORMAT, "%s\n%s %d: %s\n%s: %s"),
            (COPY_IMAGE, "DATA/SYSUTIL/SAVEBACK.PNG"),
            (COPY_PROGRESS, "Copying...\n"),
            (COPY_READ_FAILED, "read failed %s"),
            (COPY_OPEN_FAILED, "open failed %s"),
        ] {
            put(e, addr, s);
        }

        // The dialog and the game-data utility.
        e.register(SYSTEM_UTILITY_INSTANCE, |_, _| rv(COPY_UTILITY));
        e.register(MSG_DIALOG_INSTANCE, |_, a| {
            assert_eq!(a[0], COPY_UTILITY);
            rv(COPY_DIALOG)
        });
        e.put_vtable(0x00ac_0000, &[0, 0, 0x00ac_1000]);
        e.mem.set_u32(COPY_DIALOG, 0x00ac_0000);
        let seen = log.clone();
        e.register_double(0x00ac_1000, move |e, a| {
            assert_eq!(a[0], COPY_DIALOG);
            seen.borrow_mut().dialog.push((text(e, a[1]), a[2]));
            Ret::default()
        });
        e.register(GAME_DATA_UTILITY, |_, _| rv(COPY_GAME_DATA));
        let mut table = vec![0u32; 9];
        table[8] = 0x00ac_2000;
        e.put_vtable(0x00ac_0100, &table);
        e.mem.set_u32(COPY_GAME_DATA, 0x00ac_0100);
        let seen = log.clone();
        e.register_double(0x00ac_2000, move |_, a| {
            assert_eq!(a[0], COPY_GAME_DATA);
            seen.borrow_mut().started.push(a[1]);
            Ret::default()
        });
        let seen = log.clone();
        e.register_double(GAME_DATA_BUSY, move |_, a| {
            assert_eq!(a[0], COPY_GAME_DATA);
            let mut log = seen.borrow_mut();
            log.busy_polls += 1;
            rv((log.busy_polls <= busy) as u32)
        });
        let seen = log.clone();
        e.register_double(SLEEP, move |_, a| {
            assert_eq!(a[0], 10);
            seen.borrow_mut().sleeps += 1;
            Ret::default()
        });
        e.register(SET_WORD_AT_20, |e, a| {
            e.mem.set_u32(a[0] + 0x20, a[1]);
            Ret::default()
        });
        log
    }

    #[test]
    fn copy_save_games_copies_every_readable_fos_file() {
        let mut e = engine();
        let manager = manager(&mut e);
        let log = copy_rig(
            &mut e,
            &[
                ("Quick.fos", true),
                ("Saves", false),
                ("notes.txt", true),
                ("Bad.fos", true),
            ],
            "Bad",
            false,
            2,
        );
        e.call(0x0085_2080, &args![manager, 0u32]);
        let log = log.borrow();

        // The guard byte is cleared first and restored (7) at the end; the
        // walker is destroyed last.
        assert_eq!(log.guard_sets, vec![0, 7, 0xdead]);
        assert_eq!(log.folder, "Saves/");
        // Only the `.fos` plain files are opened.
        assert_eq!(log.paths, vec!["Saves/Quick.fos", "Saves/Bad.fos"]);
        assert_eq!(
            log.failures,
            vec![(COPY_OPEN_FAILED, "Saves/Bad.fos".to_string())]
        );
        assert_eq!(log.destructs, 2);

        // The good file is read whole into the buffer's data block.
        assert_eq!(log.reads.len(), 1);
        assert_eq!(log.reads[0][1..4], [COPY_DATA_BLOCK, 0x40, 0]);
        assert_eq!(
            log.dialog,
            vec![("Copying...\nSaves/Quick.fos".to_string(), 1000)]
        );

        // The buffer object got its texts, sizes, image and callback.
        let at = COPY_BUFFER_OBJECT;
        assert_eq!(text(&e, at + 0x164), "-SAVE-");
        assert_eq!(text(&e, at + 0x265), "Novac");
        assert_eq!(
            text(&e, at + 0x2e5),
            "Cottonwood\nLevel 12: -\nCourier: Courier"
        );
        assert_eq!(e.mem.u32(at + 0x7e8), 1);
        assert_eq!(e.mem.u32(at + 0x7ec), 2);
        assert_eq!(e.mem.u32(at + 0x7f0), COPY_DATA_BLOCK + 3);
        assert_eq!(text(&e, at + 0x8f4), "DATA/SYSUTIL/SAVEBACK.PNG");
        assert_eq!(e.mem.u32(at + 0x9f4), 0x0085_2740);

        // The save file is parked in the manager; the operation started once
        // and was polled until it stopped (2 busy polls, then free).
        let parked = e.get(manager, BGSSaveLoadManager::pSaveLoadFile);
        assert!(!parked.is_null());
        assert_eq!(log.started, vec![COPY_BUFFER_OBJECT]);
        assert_eq!(log.busy_polls, 3);
        assert_eq!(log.sleeps, 2);
    }

    #[test]
    fn copy_save_games_uses_the_entry_texts_when_it_has_them() {
        let mut e = engine();
        let manager = manager(&mut e);
        let log = copy_rig(&mut e, &[("Quick.fos", true)], "none", false, 0);
        e.register(ENTRY_LOCATION, |_, _| Ret::default());
        e.register(ENTRY_PLAYER_NAME, |e, _| rv(string_at(e, "Boone")));
        e.register(ENTRY_LEVEL_NAME, |e, _| rv(string_at(e, "Novac")));
        e.register(ENTRY_CELL_NAME, |e, _| rv(string_at(e, "Strip")));
        e.call(0x0085_2080, &args![manager, 0u32]);
        let at = COPY_BUFFER_OBJECT;
        // No location: the placeholder; the entry's own names fill the text.
        assert_eq!(text(&e, at + 0x265), "Location");
        assert_eq!(
            text(&e, at + 0x2e5),
            "Strip\nLevel 12: Novac\nCourier: Boone"
        );
        assert_eq!(log.borrow().sleeps, 0);
    }

    #[test]
    fn copy_save_games_reports_a_failed_read_and_goes_on() {
        let mut e = engine();
        let manager = manager(&mut e);
        let log = copy_rig(&mut e, &[("Quick.fos", true)], "none", true, 0);
        e.call(0x0085_2080, &args![manager, 0u32]);
        let log = log.borrow();
        assert_eq!(
            log.failures,
            vec![(COPY_READ_FAILED, "Saves/Quick.fos".to_string())]
        );
        // No dialog, no copy, but the file object is still destroyed and the
        // guard byte restored.
        assert!(log.dialog.is_empty());
        assert!(log.started.is_empty());
        assert_eq!(log.destructs, 1);
        assert_eq!(log.guard_sets, vec![0, 7, 0xdead]);
        assert!(e.get(manager, BGSSaveLoadManager::pSaveLoadFile).is_null());
    }

    #[test]
    fn quick_load_answers_the_loads_result() {
        let mut e = engine();
        let rig = rig(&mut e);
        let _loads = load_rig(&mut e, &rig);
        assert!(e.call(0x0085_09f0, &args![rig.manager]).bool());
    }
}
