//! `fallout shared/tesscript.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The unit holds `ScriptLocals` (the variables and action flags of a
//! running script instance, the engine's event list), the `Script` form, and
//! the static lists of references whose enable/disable/delete is delayed.
//!
//! Progress: the 77 mapped functions, `005a8bc0` to `005ae2e0`, are translated:
//! the first 40 (`005a8bc0` to `005aa930`) in an earlier session and the
//! last 37 (`005aaaf0` to `005ae2e0`) in the next, so there is nothing left
//! in that mapped range. The unplaced `005a8b80` constructor is also translated.
//!
//! Notes for whoever continues:
//! - The engine map names `005a8bc0` `ScriptLocals::ScriptLocals`, but the
//!   body is the destructor (it empties both lists and frees the effect
//!   data); `005aa1a0` is `Script`'s destructor although the decompiler calls
//!   it `CSettingsStore::~CSettingsStore` (a wrong library match).
//! - `BSSimpleList` nodes are used through the game's small accessors, called
//!   by address as the game does: `006815c0(node)` (the address of the node's
//!   item word), `00726070(node)` (next node), `008256d0(node)` (the list is
//!   empty), `0063f7b0(list)` (remove the first item), `005ae3d0(list,
//!   &item)` (add at the head), `00905820(list, &item)` (add at the tail),
//!   `00905330(list, &item)` (remove an item) and `005f65d0(list, &item)`
//!   (contains).
//! - The four static lists of pending references are `ReferencesToEnable`
//!   (`011caca8`), `ReferencesToDisable` (`011cac98`), `ReferencesToDelete`
//!   (`011cacb8`) and `ReferencesToFadeDisable` (`011cace8`), all guarded by
//!   the critical section `ScriptRefListCrit` (`011cacf8`). The roles follow
//!   the names of the `IsPending...` functions and what
//!   `RunDelayedScriptActionsOnReferences` does with each list; the saved
//!   order is enable, disable, delete, fade-disable.
//! - A pushed word that follows a call with no stack clean-up belongs to the
//!   next call (`005e24d0` takes none, `005e2730` takes the three pushed
//!   words).
//! - `Script::Init` takes a `ScriptCompileData` (`fn_005ab040` compares it
//!   with the script), not a `Script`; `005ab7f0` and `005ab930` are
//!   `cdecl` with a third pushed word they never read. `005aaee0(count, size)`
//!   is a `calloc` on the memory manager.
//! - `Script::ParseParameters` (`005accb0`) is `cdecl` varargs: seven fixed
//!   words, then the output pointers. Its typed form is
//!   [`script_parse_parameters`] (the outputs as a slice); it is registered
//!   through [`parse_parameters_entry`], which takes any number of words.
//!   `fn_005ac7a0` evaluates one operand (also called by it) and takes the
//!   same fixed words with the output double first.
//! - Names the decompiler or the map gets wrong here: `005ae380` is a list
//!   count (not `VATS::GetCount`), `00526ac0` returns the double of the `G`
//!   operand (not `QMultiBoundRadius`), `00500910` constructs a
//!   `SCRIPT_REFERENCED_OBJECT` (not `AddTail`), `00ec5ec0` is the stack
//!   probe of the `alloca` in `Script::Load`.
//! - Not translated: the compiler's exception-unwinding frames.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::{BSSimpleList, BSStringT};

// ---- Memory and list helpers of the game, called by address ---------------

/// `operator new(size)` and `operator delete(block)` (`cdecl`).
pub(crate) const OPERATOR_NEW: u32 = 0x0040_1000;
pub(crate) const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `memset(ptr, value, size)` (`cdecl`) and `memcpy(dest, source, size)`
/// (`cdecl`, the game's wrapper).
const MEMSET: u32 = 0x0040_3d30;
const MEMCPY: u32 = 0x0040_1460;
/// The scope guard the game keeps on its stack: `00404eb0(guard, kind, 1,
/// source file, line)` and `00404ee0(guard)`.
const SCOPE_GUARD_OPEN: u32 = 0x0040_4eb0;
const SCOPE_GUARD_CLOSE: u32 = 0x0040_4ee0;
/// `BSSimpleList` accessors (see the module notes).
const LIST_ITEM_SLOT: u32 = 0x0068_15c0;
const LIST_NEXT: u32 = 0x0072_6070;
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_POP_FIRST: u32 = 0x0063_f7b0;
const LIST_ADD_HEAD: u32 = 0x005a_e3d0;
const LIST_ADD_TAIL: u32 = 0x0090_5820;
const LIST_REMOVE: u32 = 0x0090_5330;
const LIST_CONTAINS: u32 = 0x005f_65d0;
/// The list constructor (clears item and next), its destructor and the
/// deleting destructor `004702f0(list, flags)`.
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
const LIST_DELETE: u32 = 0x0047_02f0;
/// `0044ddc0(object)`: the word at +8 (the action list of a `ScriptLocals`)
/// and `0084e3a0(locals)`: the word at +0xC (the variable list).
const ACTION_LIST_OF: u32 = 0x0044_ddc0;
const LOCAL_LIST_OF: u32 = 0x0084_e3a0;
/// `005d43c0(script)` and `005d43e0(script)`: the addresses of a script's
/// two embedded lists, `listRefObjects` (+0x44) and `listVariables` (+0x4C).
const SCRIPT_REF_OBJECT_LIST: u32 = 0x005d_43c0;
const SCRIPT_VARIABLE_LIST: u32 = 0x005d_43e0;

// ---- Logging, saving and loading ------------------------------------------

/// `Error(format, ...)` (Xbox PDB, `cdecl`, varargs) and the log
/// `005b5e40(format, ...)` (`cdecl`, varargs).
const ERROR_LOG: u32 = 0x0040_fbe0;
const SCRIPT_LOG: u32 = 0x005b_5e40;
/// The script line number that `GetVariable`'s message reports.
const SCRIPT_LINE_GLOBAL: u32 = 0x011c_ae50;
/// The save/load game object's global and the object `00408d60` turns into
/// the byte that says "log the save sizes".
const SAVE_LOAD_GAME_GLOBAL: u32 = 0x011d_e45c;
const SAVE_SIZE_LOG_OBJECT: u32 = 0x011d_e4e8;
const FLAG_BYTE_ADDRESS: u32 = 0x0040_8d60;
/// `TESSaveLoadGame` methods on the global (`this` first): the save format
/// uses blocks (`UseSaveGameBlocks`), the file version byte, the current
/// stream position (a pointer into the buffer), the header of the form being
/// loaded (`004fd3c0`) and saved (`004fd3e0`, `TES::GetWorldSpace` in the
/// engine map: form id at +0, flags at +5, version byte at +9), the load
/// test `0047c850` (a stub that returns 0), and the reading and writing of
/// raw bytes and numeric ids.
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
const SAVE_FILE_VERSION: u32 = 0x008d_f040;
const STREAM_POSITION: u32 = 0x0082_5c00;
const LOADING_FORM_HEADER: u32 = 0x004f_d3c0;
const SAVING_FORM_HEADER: u32 = 0x004f_d3e0;
const IS_LOADING_STUB: u32 = 0x0047_c850;
const STREAM_WRITE: u32 = 0x0085_79b0;
const STREAM_SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
const STREAM_READ: u32 = 0x0085_79e0;
const STREAM_LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
/// `LookupFormByID(id)` (`cdecl`).
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// `BGSSaveGameBuffer` methods (`this` the buffer): `StartVariableSizedValue`,
/// `SaveVariableSizedValue_ov2(count, position)`, `SaveFormID_ov2(form, 0)`,
/// `SaveFormID(id, 0)` and `00865e50(data, size, 0)`.
const BUFFER_START_SIZED_VALUE: u32 = 0x0086_5f20;
const BUFFER_SAVE_SIZED_VALUE: u32 = 0x0086_5ff0;
const BUFFER_SAVE_FORM_ID_OV2: u32 = 0x0086_5df0;
const BUFFER_SAVE_FORM_ID: u32 = 0x0086_5db0;
const BUFFER_SAVE_BYTES: u32 = 0x0086_5e50;
/// `BGSLoadGameBuffer` methods: `LoadVariableSizedValue`, `LoadFormID_ov2(
/// &id)` and `00864980(data, size)`.
const BUFFER_LOAD_SIZED_VALUE: u32 = 0x0086_4a60;
const BUFFER_LOAD_FORM_ID_OV2: u32 = 0x0086_48e0;
const BUFFER_LOAD_BYTES: u32 = 0x0086_4980;
/// `Script::PutNumericIDInDouble(&id, &double)` (`005acc70`) and its
/// inverse (`005acc90(&id, &double)`), both `cdecl`.
const PUT_NUMERIC_ID_IN_DOUBLE: u32 = 0x005a_cc70;
const GET_NUMERIC_ID_FROM_DOUBLE: u32 = 0x005a_cc90;
/// `ScriptRunManager::Instance()` and `InitActionList(master, count, locals)`.
const SCRIPT_RUN_MANAGER_INSTANCE: u32 = 0x005e_24d0;
const SCRIPT_RUN_MANAGER_INIT_ACTION_LIST: u32 = 0x005e_2730;
/// `005ac020(script)`: builds the variable list of a script.
const SCRIPT_BUILD_LOCALS: u32 = 0x005a_c020;
/// `RtlEnterCriticalSection` wrapper `004538a0(cs, 0)` and its leave
/// `004538c0(cs)` (`RtlLeaveCriticalSection` in the engine map).
const CRITICAL_SECTION_ENTER: u32 = 0x0045_38a0;
const CRITICAL_SECTION_LEAVE: u32 = 0x0045_38c0;
/// `__RTDynamicCast(object, 0, source type, target type, 0)`.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `TESFullName::GetFullName(form)` (`cdecl`) and the name of a reference's
/// base form (`0055d520(reference)`).
const FULL_NAME_OF_FORM: u32 = 0x0048_2720;
const FULL_NAME_OF_REFERENCE: u32 = 0x0055_d520;
/// The type descriptors the cast in `SetVariable` uses.
const RTTI_SOURCE_FORM: u32 = 0x0118_3028;
const RTTI_TARGET_REFERENCE: u32 = 0x0118_41cc;

/// `"D:\_Fallout3\Platforms\Common\Code\Fallout Shared\TESScript.cpp"`.
const SOURCE_FILE: u32 = 0x0103_6d58;
/// Formats of the messages (`Error` and `005b5e40`), by what they report.
const FORMAT_VARIABLE_NOT_FOUND: u32 = 0x0103_6d98;
const FORMAT_SET_VARIABLE_NO_SCRIPT: u32 = 0x0103_6e2c;
const FORMAT_SET_VARIABLE_FORM: u32 = 0x0103_6e68;
const FORMAT_SET_VARIABLE_NAMED: u32 = 0x0103_6f00;
const FORMAT_SET_VARIABLE_REF: u32 = 0x0103_6fa0;
const FORMAT_SET_VARIABLE_IN_SCRIPT: u32 = 0x0103_7040;
/// `"UNKNOWN"`.
const UNKNOWN_SCRIPT_NAME: u32 = 0x0101_5890;
/// `GetSaveSize()` messages: without and with the form being saved.
const FORMAT_SAVE_SIZE_PLAIN: u32 = 0x0101_2c78;
const FORMAT_SAVE_SIZE_FORM: u32 = 0x0101_2cb0;
/// `SaveGame()` messages: without and with the form being saved.
const FORMAT_SAVE_PLAIN: u32 = 0x0101_536c;
const FORMAT_SAVE_FORM: u32 = 0x0101_53a0;
/// The save block is larger than a short.
const FORMAT_BLOCK_TOO_LARGE: u32 = 0x0101_5318;
/// `LoadGame` block header errors: without and with the loading form.
const FORMAT_BLOCK_HEADER_PLAIN: u32 = 0x0101_56a8;
const FORMAT_BLOCK_HEADER_FORM: u32 = 0x0101_5718;
/// `LoadGame` buffer overrun/underrun messages, without and with the form.
const FORMAT_OVERRUN_PLAIN: u32 = 0x0101_54a0;
const FORMAT_UNDERRUN_PLAIN: u32 = 0x0101_5440;
const FORMAT_OVERRUN_FORM: u32 = 0x0101_5588;
const FORMAT_UNDERRUN_FORM: u32 = 0x0101_5500;
/// `"BLOK"` as a little-endian word: the marker that starts a save block.
const BLOCK_MARKER: u32 = 0x424c_4f4b;

/// The source line of the scope guard in `fn_005a8d20`.
const ACTION_SCOPE_LINE: u32 = 300;
/// Fields of the TLS block this unit uses: the last variable lookup of
/// `ScriptLocals` (the Xbox PDB's statics `pLastScriptLocals`,
/// `iLastVarSearchID` and `pLastVar`), `Script::pLastRefSearchScript` and the
/// byte `Script::m_bSystemOutput`.
const TLS_LAST_VARIABLE: u32 = 0x270;
const TLS_LAST_VARIABLE_ID: u32 = 0x274;
const TLS_LAST_LOCALS: u32 = 0x278;
const TLS_LAST_REF_SEARCH_SCRIPT: u32 = 0x288;
const TLS_SYSTEM_OUTPUT: u32 = 0x268;

/// The critical section that guards the pending reference lists.
const SCRIPT_REF_LIST_CRITICAL_SECTION: u32 = 0x011c_acf8;
/// The pending reference lists (module notes).
const REFERENCES_TO_ENABLE: u32 = 0x011c_aca8;
const REFERENCES_TO_DISABLE: u32 = 0x011c_ac98;
const REFERENCES_TO_DELETE: u32 = 0x011c_acb8;
const REFERENCES_TO_FADE_DISABLE: u32 = 0x011c_ace8;
/// `TESObjectREFR` methods used when the delayed actions run: enable
/// (`00573f40`) and disable (`TESObjectREFR::Disable`, `00574400`), and the
/// test of a flag in the word at +0x30 (`00456630(object, mask)`).
const REFERENCE_ENABLE: u32 = 0x0057_3f40;
const REFERENCE_DISABLE: u32 = 0x0057_4400;
const FLAGS_TEST: u32 = 0x0045_6630;
/// Byte that is non-zero while the fade-out of a disabled reference counts.
const FADE_ENABLED_BYTE: u32 = 0x011a_d7b4;
/// `0.0` as a double.
const ZERO_DOUBLE: u32 = 0x0101_2060;
/// `Script` helpers of the neighbouring functions that `CopyResultScript`
/// calls: `SetCompileData(data size, data)`, the copy of the referenced
/// objects and of the variables (`cdecl(source list, destination list,
/// script)`) and `SetText(text)`.
const SCRIPT_SET_COMPILE_DATA: u32 = 0x005a_ae70;
const SCRIPT_COPY_REF_OBJECTS: u32 = 0x005a_b7f0;
const SCRIPT_COPY_VARIABLES: u32 = 0x005a_b930;
const SCRIPT_SET_TEXT: u32 = 0x005a_be50;
/// Accessors of a `Script`: the header (`00500940`, +0x18), the compile data
/// (`00671d10`, +0x30), the text (`0055b980`, +0x2C) and the flag test
/// (`004013e0`, bit 3 of the form flags).
const SCRIPT_HEADER_OF: u32 = 0x0050_0940;
const SCRIPT_COMPILE_DATA_OF: u32 = 0x0067_1d10;
const SCRIPT_TEXT_OF: u32 = 0x0055_b980;
const FORM_FLAG_BIT_3: u32 = 0x0040_13e0;
/// `TESForm::TESForm` and `TESForm::~TESForm`, the form-type setter
/// (`004f15a0(form, type)`) and `00484ab0(script, flag)`.
const TES_FORM_CONSTRUCT: u32 = 0x0048_3370;
const TES_FORM_DESTRUCT: u32 = 0x0048_3630;
const SET_FORM_TYPE: u32 = 0x004f_15a0;
const SCRIPT_SET_QUEST_SCRIPT_FLAG: u32 = 0x0048_4ab0;
/// Deleting destructor `0061cb30(item, flags)` of the variable list's items,
/// and `007d9ec0(item)`, the body of the destructor of a
/// `SCRIPT_REFERENCED_OBJECT` (its string member).
const SCRIPT_VARIABLE_DELETE: u32 = 0x0061_cb30;
const SCRIPT_REFERENCED_OBJECT_DESTRUCT: u32 = 0x007d_9ec0;
/// `Script`'s vtable.
const SCRIPT_VTABLE: u32 = 0x0103_7094;

// ---- Second half of the unit: loading, running and parsing ------------------

/// `BGSLoadGameBuffer::LoadFormID` (`008648a0(buffer)`): the next form id.
const BUFFER_LOAD_FORM_ID: u32 = 0x0086_48a0;
/// `TESObjectREFR::TESObjectREFR` (`0055a2f0(block)`, the 0x68-byte block
/// from `operator new`).
const REFERENCE_CONSTRUCT: u32 = 0x0055_a2f0;
/// `BSSimpleList` clear (`00470470(list)`: releases every node after the
/// first and empties the first).
const LIST_CLEAR: u32 = 0x0047_0470;
/// `BSSimpleList` item count (`005ae380(list)`; the engine map calls it
/// `VATS::GetCount` through folded code).
const LIST_COUNT: u32 = 0x005a_e380;
/// `_strlen` (`cdecl`).
const STRLEN: u32 = 0x00ec_6130;
/// The memory manager: `00401020()` answers the singleton whose
/// `Allocate(size)` is `00aa3e40`.
const MEMORY_MANAGER_GET: u32 = 0x0040_1020;
const MEMORY_MANAGER_ALLOCATE: u32 = 0x00aa_3e40;
/// String helpers of the unit: the length of a `BSStringT` (`004048e0`, its
/// `this` the string), its character data (`00559450`), and
/// `00404dc0(first, second)` (`cdecl`), a string comparison that answers
/// zero when the strings are equal (`FindVariable` takes zero as a match).
const STRING_LENGTH: u32 = 0x0040_48e0;
const STRING_DATA: u32 = 0x0055_9450;
const STRING_COMPARE: u32 = 0x0040_4dc0;
/// The comparison `00469880(first, second)` of `CompareResultScripts`
/// (`cdecl`), non-zero when the two texts differ.
const TEXT_COMPARE: u32 = 0x0046_9880;
/// A test on a script (`00474cb0(script)`) that the loader of
/// `005ab040` combines with the name length; not named in the map.
const SCRIPT_IS_NAMED_CHECK: u32 = 0x0047_4cb0;
/// `005e29a0(variable, other)` (`this` a `ScriptVariable`): non-zero when
/// the two variables differ.
const VARIABLE_DIFFERS: u32 = 0x005e_29a0;
/// `005e2910(block, source)` (`this` the new block): the copy constructor
/// of `ScriptVariable` (0x20 bytes), `005ab8b0` is that of
/// `SCRIPT_REFERENCED_OBJECT` (0x10 bytes) and `005ac150` that of
/// `SCRIPT_LOCAL`.
const SCRIPT_VARIABLE_COPY_CONSTRUCT: u32 = 0x005e_2910;
/// `ScriptVariable::ScriptVariable()` (`005e28b0(block)`) and
/// `ScriptVariable::Load` (`005e2a20(variable, file)`, Xbox PDB).
const SCRIPT_VARIABLE_CONSTRUCT: u32 = 0x005e_28b0;
const SCRIPT_VARIABLE_LOAD: u32 = 0x005e_2a20;
/// The string constructor (`004037b0(this)`) and the copy
/// (`00501c10(this, source)`) that `005ab8b0` runs on the first eight bytes
/// of a referenced object, and `BSStringT::Set(text, 0)` (`004037f0`).
const STRING_CONSTRUCT: u32 = 0x0040_37b0;
const STRING_COPY: u32 = 0x0050_1c10;
const STRING_SET: u32 = 0x0040_37f0;
/// `SCRIPT_REFERENCED_OBJECT` constructor (`00500910(block)`, named
/// `BSSimpleList<..>::AddTail` in the map by folding).
const REFERENCED_OBJECT_CONSTRUCT: u32 = 0x0050_0910;
/// `ScriptLocals::ScriptLocals` (`005a8b80(block)`).
const SCRIPT_LOCALS_CONSTRUCT: u32 = 0x005a_8b80;
/// `ScriptCompileData` accessor of its variable list (`005a8080`, +0x3c).
const SCRIPT_BUFFER_VARIABLE_LIST: u32 = 0x005a_8080;
/// The form-type byte of a form (`00401170`, `this` the form: the byte
/// at +4), `TESForm::GetFile(-1)` (`00484e60`) and `SetFile` (`00484f50`),
/// `AddCompileIndex(&id, file)` (`00485d50`, `cdecl`), the test `004077c0`
/// that `InitItem` applies to the script, and `00483710` (an empty method).
const FORM_TYPE_OF: u32 = 0x0040_1170;
const FORM_GET_FILE: u32 = 0x0048_4e60;
const FORM_SET_FILE: u32 = 0x0048_4f50;
const FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
const SCRIPT_INIT_ITEM_CHECK: u32 = 0x0040_77c0;
const EMPTY_METHOD: u32 = 0x0048_3710;
/// The word at +0xC of a form (`0084e3a0(form)`): its form id.
const FORM_ID_OF: u32 = 0x0084_e3a0;
/// `TESForm` methods the script's save and load use: `StartForm`
/// (`004855a0`), `AddChunkData(type, data, size)` (`00485990`, `cdecl`),
/// `CloseForm` (`00485680`), `Save` (`00483d20`) and `LoadForm`
/// (`00485110(this, file)`).
const FORM_START: u32 = 0x0048_55a0;
const FORM_ADD_CHUNK: u32 = 0x0048_5990;
const FORM_CLOSE: u32 = 0x0048_5680;
const FORM_SAVE: u32 = 0x0048_3d20;
const FORM_LOAD: u32 = 0x0048_5110;
/// The "swap bytes on save" global test (`00401500()`, a byte at
/// `011c54ba`) and the swap of a script header (`00414220(header)`).
const ENDIAN_SWAP_ENABLED: u32 = 0x0040_1500;
const HEADER_SWAP_BYTES: u32 = 0x0041_4220;
/// `TESFile` methods (`this` the file): the type of the current record
/// (`00472660`, `GetTESForm`), the next chunk's type (`004726b0`,
/// `GetTESChunk`, zero at the end), the advance to the next chunk
/// (`004726f0`), the chunk size (`00401660`), `GetChunkData(&value)`
/// (`004727f0`), `GetChunkData(buffer, size)` (`00472890`), the
/// big-endian flag (`00401680`) and the file name (`00891170`).
const FILE_RECORD_TYPE: u32 = 0x0047_2660;
const FILE_CHUNK_TYPE: u32 = 0x0047_26b0;
const FILE_NEXT_CHUNK: u32 = 0x0047_26f0;
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_GET_CHUNK_VALUE: u32 = 0x0047_27f0;
const FILE_GET_CHUNK_DATA: u32 = 0x0047_2890;
const FILE_IS_BIG_ENDIAN: u32 = 0x0040_1680;
const FILE_NAME_OF: u32 = 0x0089_1170;
/// Chunk types of a script record, as little-endian words of their four
/// letters.
const CHUNK_RNAM: u32 = 0x4d41_4e52;
const CHUNK_OBND: u32 = 0x444e_424f;
const CHUNK_SCDA: u32 = 0x4144_4353;
const CHUNK_EDID: u32 = 0x4449_4445;
const CHUNK_SLSD: u32 = 0x4453_4c53;
const CHUNK_SCVR: u32 = 0x5256_4353;
const CHUNK_SCRO: u32 = 0x4f52_4353;
const CHUNK_SCHR: u32 = 0x5248_4353;
const CHUNK_SCRV: u32 = 0x5652_4353;
/// The form type of a script record.
const FORM_TYPE_SCRIPT: u32 = 0x11;
/// Virtual slots of a `Script` (byte offsets): the load of the object
/// bounds (`+0xE0`), the editor id setter (`+0x134`), the "set delete"
/// (`+0xC4`) and "set altered" (`+0xC8`) and the clean-up (`+0x88`).
const SLOT_LOAD_BOUNDS: u32 = 0xe0;
const SLOT_SET_EDITOR_ID: u32 = 0x134;
const SLOT_SET_DELETED: u32 = 0xc4;
const SLOT_SET_ALTERED: u32 = 0xc8;
const SLOT_INIT_ITEM: u32 = 0x88;
/// Formats of the messages of this half.
const FORMAT_REFERENCED_OBJECT_INVALID: u32 = 0x0103_7238;
const FORMAT_REFERENCED_OBJECT_NOT_FOUND: u32 = 0x0103_72a0;
const FORMAT_REFERENCE_LIST_CORRUPT: u32 = 0x0103_71d0;
const FORMAT_NOT_COMPILED: u32 = 0x0103_730c;
const FORMAT_LOCAL_VARIABLE_NOT_FOUND: u32 = 0x0103_7350;
const FORMAT_PARAMETER_TYPE_UNIMPLEMENTED: u32 = 0x0103_73a8;
/// Source lines of the scope guards.
const INIT_ITEM_SCOPE_LINE: u32 = 0xab5;
const BUILD_LOCALS_SCOPE_LINE: u32 = 0xcde;
const COPY_VARIABLES_SCOPE_LINE: u32 = 0xceb;
const COMPILE_AND_RUN_SCOPE_LINE: u32 = 0xd53;
/// TLS fields of the referenced-object cache (with
/// [`TLS_LAST_REF_SEARCH_SCRIPT`]): the last result, index and locals.
const TLS_LAST_REF_OBJECT: u32 = 0x27c;
const TLS_LAST_REF_INDEX: u32 = 0x280;
const TLS_LAST_REF_LOCALS: u32 = 0x284;
/// The setting object whose accessor `00403e20(object)` gives a pointer
/// to the float used as the default quest script delay (`fQuestScriptDelay`
/// is set from it), the table of eight halvings of it (`011cad20`) and the
/// counter of quest scripts initialized (`011cad40`).
const QUEST_DELAY_SETTING: u32 = 0x011c_ac68;
const QUEST_DELAY_ACCESSOR: u32 = 0x0040_3e20;
const QUEST_DELAY_TABLE: u32 = 0x011c_ad20;
const QUEST_DELAY_COUNTER: u32 = 0x011c_ad40;
/// The divisor of the halvings, as a double (read from the exe).
const QUEST_DELAY_DIVISOR: u32 = 0x0101_1590;
/// `00792760(quest)`: the float (in `ST0`) added to a quest script's
/// delay; `0084d030(object)` with `object` at `011f6394`: the float
/// elapsed time that `Run` takes off the delay.
const QUEST_DELAY_OF: u32 = 0x0079_2760;
const ELAPSED_TIME: u32 = 0x0084_d030;
const ELAPSED_TIME_OBJECT: u32 = 0x011f_6394;
/// `ScriptRunManager::Run(manager, script, a, b, c, flag, 0, 0, 0.0)`.
const SCRIPT_RUN_MANAGER_RUN: u32 = 0x005e_2590;
/// `ScriptCompiler::CompilePartialScript` (`005aedb0`, `this` the
/// compiler: `(script, reference, mode, 0)`).
const COMPILE_PARTIAL_SCRIPT: u32 = 0x005a_edb0;
/// `TESObjectREFR::GetScriptVariables` (`005673e0(reference)`).
const REFERENCE_SCRIPT_VARIABLES: u32 = 0x0056_73e0;
/// `ExtraDataList::GetScriptLocals` (`00418830(list)`).
const EXTRA_LIST_SCRIPT_LOCALS: u32 = 0x0041_8830;
/// `ExtraDataList::GetReferencePointer` (`0041c8d0(list)`), `00576260
/// (pointer, key, id)` (`TESObjectREFR::GetInventoryItem`), `ItemChange::
/// GetScriptLocals` (`004bdea0(item)`), `007af430(form)` (the key passed
/// to the inventory lookup), `004459e0(item, 1)` (releases the item) and
/// `0059e300(quest)` (the locals of a quest).
const EXTRA_LIST_REFERENCE_POINTER: u32 = 0x0041_c8d0;
const INVENTORY_ITEM_OF: u32 = 0x0057_6260;
const ITEM_SCRIPT_LOCALS: u32 = 0x004b_dea0;
const INVENTORY_KEY_OF: u32 = 0x007a_f430;
const RELEASE_ITEM: u32 = 0x0044_59e0;
const QUEST_SCRIPT_LOCALS: u32 = 0x0059_e300;
/// `BSMultiBoundCapsule::QMultiBoundRadius` in the map (folded): the double
/// (in `ST0`) that `fn_005ac7a0` stores for the `G` marker
/// (`00526ac0(object)`).
const GLOBAL_VALUE_OF: u32 = 0x0052_6ac0;
/// `ScriptCompiler::GetFunctionDef(id)` (`005b1120`, `cdecl`): the
/// definition whose +0x10 byte says it needs a reference, +0x14 is its
/// parameter list and +0x18 its handler.
const GET_FUNCTION_DEFINITION: u32 = 0x005b_1120;
/// `ScriptParameter` type table: 8 bytes per type, the byte at +5 is set
/// for the types given as referenced objects.
const PARAMETER_TYPE_TABLE: u32 = 0x0118_cdd5;
/// `TESContainer::ContainerCanHoldType(formType)` (`cdecl`).
const CONTAINER_CAN_HOLD_TYPE: u32 = 0x0048_1f30;
/// `_ftol2` (`00ec62c0`), the value in `ST0` as a leading double.
const FTOL2: u32 = 0x00ec_62c0;
/// Whether the scripts are processed (a byte).
const PROCESS_SCRIPTS_BYTE: u32 = 0x0118_c685;
/// Type descriptors of the casts of `ParseParameters`.
const RTTI_TARGET_FORM_7: u32 = 0x0118_3060;
const RTTI_TARGET_FORM_11: u32 = 0x0118_3140;
const RTTI_TARGET_FORM_31: u32 = 0x0118_37c4;

layout! {
    /// `ScriptCompileData` (Xbox PDB), 0x58 bytes on both builds: the
    /// output of the script compiler that `Script::Init` takes.
    pub struct ScriptCompileData: 0x58 {
        /// `cScriptName` (Xbox PDB): a `BSStringT<char>`.
        0x0C cScriptName: Inline<BSStringT>,
        /// `pOutput` (Xbox PDB): the compiled bytes.
        0x20 pOutput: Ptr,
        /// `header` (Xbox PDB).
        0x28 header: Inline<ScriptHeader>,
        /// `listVariables` (Xbox PDB): a `BSSimpleList` embedded here.
        0x3C listVariables: Inline<BSSimpleList>,
        /// `listRefObjects` (Xbox PDB): a `BSSimpleList` embedded here.
        0x44 listRefObjects: Inline<BSSimpleList>,
    }
}

layout! {
    /// `ScriptLocals` (Xbox PDB), 0x14 bytes on both builds: the variables and
    /// the action flags of one running script instance.
    pub struct ScriptLocals: 0x14 {
        /// `m_pMasterScript` (Xbox PDB): the `Script` it belongs to.
        0x00 m_pMasterScript: Ptr,
        /// `m_cFlags` (Xbox PDB).
        0x04 m_cFlags: u8,
        /// `m_pActionList` (Xbox PDB): `BSSimpleList<ACTION_OBJECT *> *`.
        0x08 m_pActionList: Ptr,
        /// `m_pLocalList` (Xbox PDB): `BSSimpleList<SCRIPT_LOCAL *> *`.
        0x0C m_pLocalList: Ptr,
        /// `m_pScriptEffectData` (Xbox PDB): `SCRIPT_EFFECT_DATA *` (8 bytes).
        0x10 m_pScriptEffectData: Ptr,
    }

    /// `ACTION_OBJECT` (Xbox PDB), 8 bytes: a form and its action flags.
    pub struct ActionObject: 0x08 {
        /// `pForm` (Xbox PDB).
        0x00 pForm: Ptr,
        /// `iFlags` (Xbox PDB).
        0x04 iFlags: u32,
    }

    /// `SCRIPT_LOCAL` (Xbox PDB): one script variable.
    pub struct ScriptLocal: 0x18 {
        /// `uiID` (Xbox PDB).
        0x00 uiID: u32,
        /// `fValue` (Xbox PDB).
        0x08 fValue: f64,
        /// `bIsInteger` (Xbox PDB).
        0x10 bIsInteger: bool,
    }

    /// `SCRIPT_REFERENCED_OBJECT` (Xbox PDB), 0x10 bytes.
    pub struct ScriptReferencedObject: 0x10 {
        /// `uiVariableID` (Xbox PDB).
        0x0C uiVariableID: u32,
    }

    /// `SCRIPT_HEADER` (Xbox PDB), 0x14 bytes.
    pub struct ScriptHeader: 0x14 {
        /// `variableCount` (Xbox PDB).
        0x00 variableCount: u32,
        /// `refObjectCount` (Xbox PDB).
        0x04 refObjectCount: u32,
        /// `dataSize` (Xbox PDB).
        0x08 dataSize: u32,
        /// `m_uiLastID` (Xbox PDB).
        0x0C m_uiLastID: u32,
        /// `bIsQuestScript` (Xbox PDB).
        0x10 bIsQuestScript: bool,
        /// `bIsMagicEffectScript` (Xbox PDB).
        0x11 bIsMagicEffectScript: bool,
        /// `bIsCompiled` (Xbox PDB).
        0x12 bIsCompiled: bool,
    }

    /// `Script` (Xbox PDB), 0x54 bytes on PC (0x64 on Xbox, whose `TESForm`
    /// is 0x10 larger): the fields are the PDB's offsets minus 0x10.
    pub struct Script: 0x54 {
        /// `m_header` (Xbox PDB).
        0x18 m_header: Inline<ScriptHeader>,
        /// `m_text` (Xbox PDB).
        0x2C m_text: Ptr,
        /// `m_data` (Xbox PDB).
        0x30 m_data: Ptr,
        /// `fProfilerTimer` (Xbox PDB).
        0x34 fProfilerTimer: f32,
        /// `fQuestScriptDelay` (Xbox PDB).
        0x38 fQuestScriptDelay: f32,
        /// `fQuestScriptGetSecondsBuffer` (Xbox PDB).
        0x3C fQuestScriptGetSecondsBuffer: f32,
        /// `pOwnerQuest` (Xbox PDB).
        0x40 pOwnerQuest: Ptr,
        /// `listRefObjects` (Xbox PDB): a `BSSimpleList` embedded here.
        0x44 listRefObjects: Inline<BSSimpleList>,
        /// `listVariables` (Xbox PDB): a `BSSimpleList` embedded here.
        0x4C listVariables: Inline<BSSimpleList>,
    }
}

// ---- Small helpers ---------------------------------------------------------

/// The item word of a list node (`006815c0` gives its address).
fn list_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
    e.mem.u32(slot)
}

/// The next node of a list node.
fn list_next(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_NEXT, &args![node]).u32()
}

/// Whether the list starting at `node` has no item (`008256d0`).
fn list_is_empty(e: &mut Engine, node: u32) -> bool {
    e.call(LIST_IS_EMPTY, &args![node]).bool()
}

fn delete_block(e: &mut Engine, block: u32) {
    e.call(OPERATOR_DELETE, &args![block]);
}

fn new_block(e: &mut Engine, size: u32) -> u32 {
    e.call(OPERATOR_NEW, &args![size]).u32()
}

/// `new BSSimpleList` (8 bytes): the block is constructed unless the
/// allocation failed (a null block gives a null list).
fn new_list(e: &mut Engine) -> u32 {
    let block = new_block(e, 8);
    if block != 0 {
        e.call(LIST_CONSTRUCT, &args![block]).u32()
    } else {
        0
    }
}

/// `Error(...)`/`005b5e40(...)` with the words the game pushes.
fn error_log(e: &mut Engine, words: &[u32]) {
    e.call(ERROR_LOG, words);
}

fn script_log(e: &mut Engine, words: &[u32]) {
    e.call(SCRIPT_LOG, words);
}

fn save_load_game(e: &Engine) -> u32 {
    e.global::<u32>(SAVE_LOAD_GAME_GLOBAL)
}

fn use_save_game_blocks(e: &mut Engine) -> bool {
    let save_load = save_load_game(e);
    e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool()
}

fn file_version(e: &mut Engine) -> u8 {
    let save_load = save_load_game(e);
    e.call(SAVE_FILE_VERSION, &args![save_load]).u8()
}

fn stream_position(e: &mut Engine) -> u32 {
    let save_load = save_load_game(e);
    e.call(STREAM_POSITION, &args![save_load]).u32()
}

/// Whether the "log the save sizes" byte is set.
fn save_size_logging(e: &mut Engine) -> bool {
    let flag = e
        .call(FLAG_BYTE_ADDRESS, &args![SAVE_SIZE_LOG_OBJECT])
        .u32();
    e.mem.u8(flag) != 0
}

fn stream_write(e: &mut Engine, data: u32, size: u32) {
    let save_load = save_load_game(e);
    e.call(STREAM_WRITE, &args![save_load, data, size]);
}

fn stream_read(e: &mut Engine, data: u32, size: u32) {
    let save_load = save_load_game(e);
    e.call(STREAM_READ, &args![save_load, data, size]);
}

/// The type name of a form (virtual slot `+0x130`, no arguments).
fn form_type_name(e: &mut Engine, form: u32) -> u32 {
    e.vcall(form, 0x130, &args![]).u32()
}

/// Runs `body` between the scope guard `00404eb0(guard, 0x15, 1, file, line)`
/// and `00404ee0(guard)` (the guard is a local of the game).
fn with_scope_guard<R>(e: &mut Engine, line: u32, body: impl FnOnce(&mut Engine) -> R) -> R {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_GUARD_OPEN,
            &args![guard, 0x15u32, 1u32, SOURCE_FILE, line],
        );
        let result = body(e);
        e.call(SCOPE_GUARD_CLOSE, &args![guard]);
        result
    })
}

/// Runs `body` with the pending-reference critical section held
/// (`004538a0(cs, 0)` ... `004538c0(cs)`).
fn with_reference_lock<R>(e: &mut Engine, body: impl FnOnce(&mut Engine) -> R) -> R {
    e.call(
        CRITICAL_SECTION_ENTER,
        &args![SCRIPT_REF_LIST_CRITICAL_SECTION, 0u32],
    );
    let result = body(e);
    e.call(
        CRITICAL_SECTION_LEAVE,
        &args![SCRIPT_REF_LIST_CRITICAL_SECTION],
    );
    result
}

/// Empties and deletes one of the two lists a `ScriptLocals` owns: deletes
/// each item as it is taken off the front, deletes the list and clears the
/// field.
fn free_item_list(e: &mut Engine, this: Ptr<ScriptLocals>, field: Field<ScriptLocals, Ptr>) {
    if e.get(this, field).is_null() {
        return;
    }
    loop {
        let list = e.get(this, field).addr();
        let item = list_item(e, list);
        if item == 0 {
            break;
        }
        e.call(LIST_POP_FIRST, &args![list]);
        delete_block(e, item);
    }
    let list = e.get(this, field);
    if !list.is_null() {
        e.call(LIST_DELETE, &args![list, 1u32]);
    }
    e.set(this, field, Ptr::NULL);
}

// Translated from 005a8b80 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Constructs `ScriptLocals` (Xbox PDB): clears its twenty bytes through the
/// game's memset, then explicitly clears the master script and effect data.
pub fn script_locals_construct(e: &mut Engine, this: Ptr<ScriptLocals>) -> Ptr<ScriptLocals> {
    e.call(MEMSET, &args![this, 0u32, 0x14u32]);
    e.set(this, ScriptLocals::m_pMasterScript, Ptr::NULL);
    e.set(this, ScriptLocals::m_pScriptEffectData, Ptr::NULL);
    this
}

// Translated from 005a8bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `ScriptLocals` (the engine map names it
/// `ScriptLocals::ScriptLocals`): empties the action list and the variable
/// list and frees the script effect data.
pub fn fn_005a8bc0(e: &mut Engine, this: Ptr<ScriptLocals>) {
    fn_005a8c00(e, this);
    fn_005a8c90(e, this);
    let effect_data = e.get(this, ScriptLocals::m_pScriptEffectData);
    if !effect_data.is_null() {
        delete_block(e, effect_data.addr());
    }
}

// Translated from 005a8c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees every `ACTION_OBJECT` and the action list.
pub fn fn_005a8c00(e: &mut Engine, this: Ptr<ScriptLocals>) {
    free_item_list(e, this, ScriptLocals::m_pActionList);
}

// Translated from 005a8c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees every `SCRIPT_LOCAL` and the variable list.
pub fn fn_005a8c90(e: &mut Engine, this: Ptr<ScriptLocals>) {
    free_item_list(e, this, ScriptLocals::m_pLocalList);
}

// Translated from 005a8d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes sure the action list has an entry for `form` (flags 0): creates the
/// list on first use and adds a new `ACTION_OBJECT` at its head when the
/// form has none. Runs inside a scope guard (kind `0x15`, line 300).
pub fn fn_005a8d20(e: &mut Engine, this: Ptr<ScriptLocals>, form: u32) {
    with_scope_guard(e, ACTION_SCOPE_LINE, |e| {
        if e.get(this, ScriptLocals::m_pActionList).is_null() {
            let list = new_list(e);
            e.set(this, ScriptLocals::m_pActionList, Ptr::new(list));
        }
        let action = fn_005a90b0(e, this, form, false);
        if action.is_null() {
            let block = new_block(e, 8);
            e.mem.set_u32(block, form);
            e.mem.set_u32(block + 4, 0);
            let list = e.get(this, ScriptLocals::m_pActionList);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), block);
                e.call(LIST_ADD_HEAD, &args![list, slot]);
            });
        }
    });
}

// Translated from 005a8e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptLocals::SetActionFlag` (Xbox PDB): ORs `flags` into the entry of
/// `form` and into the entry with no form (the "any form" entry), both
/// looked up with the create flag set. False when there is no action list
/// or neither entry exists.
pub fn script_locals_set_action_flag(
    e: &mut Engine,
    this: Ptr<ScriptLocals>,
    form: u32,
    flags: u32,
) -> bool {
    if e.get(this, ScriptLocals::m_pActionList).is_null() {
        return false;
    }
    let mut found = false;
    for target in [form, 0] {
        let action = fn_005a90b0(e, this, target, true).cast::<ActionObject>();
        if !action.is_null() {
            let old = e.get(action, ActionObject::iFlags);
            e.set(action, ActionObject::iFlags, old | flags);
            found = true;
        }
    }
    found
}

// Translated from 005a8ea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the flags of every entry of the action list.
pub fn fn_005a8ea0(e: &mut Engine, this: Ptr<ScriptLocals>) {
    let mut node = e.get(this, ScriptLocals::m_pActionList).addr();
    while node != 0 {
        let item = list_item(e, node);
        if item == 0 {
            break;
        }
        node = list_next(e, node);
        e.set(Ptr::<ActionObject>::new(item), ActionObject::iFlags, 0);
    }
}

// Translated from 005a8ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the entry of `form` has any of the flags in `mask`; a form with no
/// entry gets one (`fn_005a8d20`) and the answer is false.
pub fn fn_005a8ef0(e: &mut Engine, this: Ptr<ScriptLocals>, form: u32, mask: u32) -> bool {
    let action = fn_005a90b0(e, this, form, false).cast::<ActionObject>();
    if action.is_null() {
        fn_005a8d20(e, this, form);
        return false;
    }
    e.get(action, ActionObject::iFlags) & mask != 0
}

// Translated from 005a8f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the action list with a copy of the action list of `source` (an
/// object whose +8 word is an action list): each `ACTION_OBJECT` is copied
/// (form and flags) and appended; the new list is created on the first item.
pub fn fn_005a8f40(e: &mut Engine, this: Ptr<ScriptLocals>, source: u32) {
    if !e.get(this, ScriptLocals::m_pActionList).is_null() {
        fn_005a8c00(e, this);
    }
    if source == 0 {
        return;
    }
    let mut node = e.call(ACTION_LIST_OF, &args![source]).u32();
    if node == 0 || list_is_empty(e, node) {
        return;
    }
    let mut destination = e.get(this, ScriptLocals::m_pActionList).addr();
    while node != 0 {
        let original = list_item(e, node);
        if original == 0 {
            break;
        }
        let copy = new_block(e, 8);
        let form = e.mem.u32(original);
        let flags = e.mem.u32(original + 4);
        e.mem.set_u32(copy, form);
        e.mem.set_u32(copy + 4, flags);
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), copy);
            if destination == 0 {
                let list = new_list(e);
                e.set(this, ScriptLocals::m_pActionList, Ptr::new(list));
                e.call(LIST_ADD_HEAD, &args![list, slot]);
                destination = e.get(this, ScriptLocals::m_pActionList).addr();
            } else {
                e.call(LIST_ADD_TAIL, &args![destination, slot]);
                destination = list_next(e, destination);
            }
        });
        node = list_next(e, node);
    }
}

// Translated from 005a90b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Finds the `ACTION_OBJECT` of `form` in the action list. With `create`
/// set and no match, returns the last entry that has no form instead.
/// Null when there is no list.
pub fn fn_005a90b0(e: &mut Engine, this: Ptr<ScriptLocals>, form: u32, create: bool) -> Ptr {
    let mut node = e.get(this, ScriptLocals::m_pActionList).addr();
    if node == 0 {
        return Ptr::NULL;
    }
    let mut empty_entry = 0u32;
    while node != 0 {
        let item = list_item(e, node);
        if item == 0 {
            break;
        }
        let entry_form = e.mem.u32(item);
        if entry_form == form {
            return Ptr::new(item);
        }
        if entry_form == 0 {
            empty_entry = item;
        }
        node = list_next(e, node);
    }
    if create {
        Ptr::new(empty_entry)
    } else {
        Ptr::NULL
    }
}

// Translated from 005a9140 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptLocals::GetVariable` (Xbox PDB): the value of the variable `id`.
/// The last lookup is cached in the TLS block (locals, id, variable);
/// otherwise the variable list is searched. A missing variable is logged
/// (with the name of `script`, "UNKNOWN" when null, and the line in
/// `011cae50`) and reads 0.
pub fn script_locals_get_variable(
    e: &mut Engine,
    this: Ptr<ScriptLocals>,
    id: u32,
    script: Ptr,
) -> f64 {
    let tls = e.tls();
    let mut variable = 0u32;
    if e.mem.u32(tls + TLS_LAST_LOCALS) == this.addr()
        && e.mem.u32(tls + TLS_LAST_VARIABLE_ID) == id
    {
        variable = e.mem.u32(tls + TLS_LAST_VARIABLE);
    }
    if variable == 0 {
        let mut node = e.call(LOCAL_LIST_OF, &args![this]).u32();
        while node != 0 && !list_is_empty(e, node) {
            let candidate = list_item(e, node);
            if e.mem.u32(candidate) == id {
                variable = candidate;
                break;
            }
            node = list_next(e, node);
        }
    }
    e.mem.set_u32(tls + TLS_LAST_LOCALS, this.addr());
    e.mem.set_u32(tls + TLS_LAST_VARIABLE_ID, id);
    e.mem.set_u32(tls + TLS_LAST_VARIABLE, variable);
    if variable != 0 {
        return e.mem.f64(variable + 8);
    }
    let name = if script.is_null() {
        UNKNOWN_SCRIPT_NAME
    } else {
        form_type_name(e, script.addr())
    };
    let line: u32 = e.global(SCRIPT_LINE_GLOBAL);
    script_log(e, &args![FORMAT_VARIABLE_NOT_FOUND, id, name, line]);
    0.0
}

// Translated from 005a9290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptLocals::SetVariable` (Xbox PDB): stores `value` in the variable
/// `id`. A missing variable is logged: with the master script's name when
/// there is one; otherwise, while loading (`0047c850` on the save/load game
/// object, a stub that returns 0), with the file name and the form being
/// loaded; otherwise with the id alone.
pub fn script_locals_set_variable(e: &mut Engine, this: Ptr<ScriptLocals>, id: u32, value: f64) {
    let mut node = e.call(LOCAL_LIST_OF, &args![this]).u32();
    while node != 0 {
        if list_item(e, node) == 0 {
            break;
        }
        let variable = list_item(e, node);
        node = list_next(e, node);
        if e.mem.u32(variable) == id {
            e.mem.set_f64(variable + 8, x87_local_value(value));
            return;
        }
    }
    let mut log_generic = true;
    let master = e.get(this, ScriptLocals::m_pMasterScript);
    if !master.is_null() {
        let name = form_type_name(e, master.addr());
        script_log(e, &args![FORMAT_SET_VARIABLE_IN_SCRIPT, id, name]);
        log_generic = false;
    } else {
        let save_load = save_load_game(e);
        if e.call(IS_LOADING_STUB, &args![save_load]).bool() {
            let header = e.call(LOADING_FORM_HEADER, &args![save_load]).u32();
            if header != 0 {
                let header_id = e.mem.u32(header);
                let form = e.call(LOOKUP_FORM_BY_ID, &args![header_id]).u32();
                let reference = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![form, 0u32, RTTI_SOURCE_FORM, RTTI_TARGET_REFERENCE, 0u32],
                    )
                    .u32();
                let name = if reference != 0 {
                    e.call(FULL_NAME_OF_REFERENCE, &args![reference]).u32()
                } else {
                    e.call(FULL_NAME_OF_FORM, &args![form]).u32()
                };
                let file_name = fn_005a9460(e, Ptr::new(save_load));
                if form != 0 && name != 0 {
                    let format = if reference != 0 {
                        FORMAT_SET_VARIABLE_REF
                    } else {
                        FORMAT_SET_VARIABLE_NAMED
                    };
                    script_log(e, &args![format, file_name, id, name, header_id]);
                } else {
                    script_log(
                        e,
                        &args![FORMAT_SET_VARIABLE_FORM, file_name, id, header_id],
                    );
                }
                log_generic = false;
            }
        }
    }
    if log_generic {
        script_log(e, &args![FORMAT_SET_VARIABLE_NO_SCRIPT, id]);
    }
}

// Translated from 005a9460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The word at +0x1C4 of the save/load game object: the name of the file
/// being loaded.
pub fn fn_005a9460(e: &mut Engine, this: Ptr) -> u32 {
    e.mem.u32(this.addr() + 0x1c4)
}

// Translated from 005a9480 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether one of the master script's referenced objects has the variable id
/// `variable_id` (such a variable holds a form reference, saved as a form id).
pub fn fn_005a9480(e: &mut Engine, this: Ptr<ScriptLocals>, variable_id: u32) -> bool {
    let master = e.get(this, ScriptLocals::m_pMasterScript);
    if master.is_null() {
        return false;
    }
    let mut node = e.call(SCRIPT_REF_OBJECT_LIST, &args![master]).u32();
    while node != 0 {
        let object = Ptr::<ScriptReferencedObject>::new(list_item(e, node));
        if object.is_null() {
            break;
        }
        if e.get(object, ScriptReferencedObject::uiVariableID) == variable_id {
            return true;
        }
        node = list_next(e, node);
    }
    false
}

/// Logs a save size message with the form being saved: `size`, the form id,
/// its type name, its flags (word at +5 of the header), the line and the
/// source file; without a header only `size`, line and file.
fn log_saved_size(e: &mut Engine, header: u32, size: u32, line: u32, formats: (u32, u32)) {
    if header != 0 {
        let header_id = e.mem.u32(header);
        let form = e.call(LOOKUP_FORM_BY_ID, &args![header_id]).u32();
        let flags = e.mem.u32(header + 5);
        let name = form_type_name(e, form);
        error_log(
            e,
            &args![formats.0, size, header_id, name, flags, line, SOURCE_FILE],
        );
    } else {
        error_log(e, &args![formats.1, size, line, SOURCE_FILE]);
    }
}

// Translated from 005a94f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The size in bytes (a 16-bit count) the old save format needs for these
/// locals: 2 for the variable count (plus 6 when blocks are used), 8 or 12
/// per non-zero variable (8 when it is a form reference), 1 for the
/// effect-data flag (plus 8 when there is effect data). Logs the size when
/// size logging is on.
pub fn fn_005a94f0(e: &mut Engine, this: Ptr<ScriptLocals>) -> u16 {
    let mut size: u16 = 0;
    if use_save_game_blocks(e) {
        size = size.wrapping_add(6);
    }
    size = size.wrapping_add(2);
    let mut node = e.get(this, ScriptLocals::m_pLocalList).addr();
    while node != 0 {
        let variable = list_item(e, node);
        if variable != 0 && e.mem.f64(variable + 8) != 0.0 {
            let id = e.mem.u32(variable);
            if fn_005a9480(e, this, id) {
                size = size.wrapping_add(8);
            } else {
                size = size.wrapping_add(0xc);
            }
        }
        node = list_next(e, node);
    }
    size = size.wrapping_add(1);
    if !e.get(this, ScriptLocals::m_pScriptEffectData).is_null() {
        size = size.wrapping_add(8);
    }
    if save_size_logging(e) {
        let save_load = save_load_game(e);
        let header = e.call(SAVING_FORM_HEADER, &args![save_load]).u32();
        log_saved_size(
            e,
            header,
            size as u32,
            0x27e,
            (FORMAT_SAVE_SIZE_FORM, FORMAT_SAVE_SIZE_PLAIN),
        );
    }
    size
}

// Translated from 005a9670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the locals in the old save format straight to the save/load game
/// stream: an optional block (marker `BLOK` and a 16-bit block size patched
/// at the end), a 16-bit variable count (patched), each non-zero variable
/// (id and value, or the id with its top bit set and the form id for a form
/// reference), a flag for the effect data and the effect data.
pub fn fn_005a9670(e: &mut Engine, this: Ptr<ScriptLocals>) {
    e.with_stack(0x20, |e, frame| old_save_body(e, this, frame.addr()));
}

fn old_save_body(e: &mut Engine, this: Ptr<ScriptLocals>, frame: u32) {
    let marker = frame;
    let block_size = frame + 4;
    let count = frame + 8;
    let id_word = frame + 0xc;
    let numeric = frame + 0x10;
    let flag = frame + 0x14;
    let mut block_position = 0u32;
    let mut start = stream_position(e);
    if save_size_logging(e) {
        start = stream_position(e);
    }
    if use_save_game_blocks(e) {
        e.mem.set_u32(marker, BLOCK_MARKER);
        stream_write(e, marker, 4);
        block_position = stream_position(e);
        stream_write(e, block_size, 2);
    }
    e.mem.set_u16(count, 0);
    let count_position = stream_position(e);
    stream_write(e, count, 2);
    let mut node = e.get(this, ScriptLocals::m_pLocalList).addr();
    while node != 0 {
        let variable = list_item(e, node);
        if variable != 0 && e.mem.f64(variable + 8) != 0.0 {
            let id = e.mem.u32(variable);
            if fn_005a9480(e, this, id) {
                e.mem.set_u32(numeric, 0);
                e.call(GET_NUMERIC_ID_FROM_DOUBLE, &args![numeric, variable + 8]);
                e.mem.set_u32(id_word, id | 0x8000_0000);
                stream_write(e, id_word, 4);
                let save_load = save_load_game(e);
                e.call(STREAM_SAVE_NUMERIC_ID, &args![save_load, numeric, 4u32]);
            } else {
                stream_write(e, variable, 4);
                stream_write(e, variable + 8, 8);
            }
            let saved = e.mem.u16(count);
            e.mem.set_u16(count, saved.wrapping_add(1));
        }
        node = list_next(e, node);
    }
    let saved = e.mem.u16(count);
    e.mem.set_u16(count_position, saved);
    let effect_data = e.get(this, ScriptLocals::m_pScriptEffectData).addr();
    e.mem.set_u8(flag, (effect_data != 0) as u8);
    stream_write(e, flag, 1);
    if effect_data != 0 {
        stream_write(e, effect_data, 8);
    }
    if save_size_logging(e) {
        let end = stream_position(e);
        let save_load = save_load_game(e);
        let header = e.call(SAVING_FORM_HEADER, &args![save_load]).u32();
        log_saved_size(
            e,
            header,
            end.wrapping_sub(start),
            0x2b6,
            (FORMAT_SAVE_FORM, FORMAT_SAVE_PLAIN),
        );
    }
    if use_save_game_blocks(e) {
        let end = stream_position(e);
        if end > block_position.wrapping_add(0xffff) {
            script_log(e, &args![FORMAT_BLOCK_TOO_LARGE, SOURCE_FILE, 0x2b6u32]);
        }
        e.mem
            .set_u16(block_position, end.wrapping_sub(block_position) as u16);
    }
}

/// Logs a load error with the form being loaded: the optional leading
/// numbers, file, line, form id, type name, version byte and flags; or,
/// without a header, the leading numbers, file, line and the file version.
fn log_loading_form(
    e: &mut Engine,
    header: u32,
    form: u32,
    leading: &[u32],
    line: u32,
    formats: (u32, u32),
) {
    let mut words: Vec<u32> = Vec::new();
    if header != 0 {
        let flags = e.mem.u32(header + 5);
        let version = e.mem.u8(header + 9) as u32;
        let header_id = e.mem.u32(header);
        let name = form_type_name(e, form);
        words.push(formats.0);
        words.extend_from_slice(leading);
        words.extend_from_slice(&[SOURCE_FILE, line, header_id, name, version, flags]);
    } else {
        let version = file_version(e) as u32;
        words.push(formats.1);
        words.extend_from_slice(leading);
        words.extend_from_slice(&[SOURCE_FILE, line, version]);
    }
    script_log(e, &words);
}

/// The header of the form being loaded and the form it names (0 for both
/// when nothing is being loaded).
fn loading_form(e: &mut Engine) -> (u32, u32) {
    let save_load = save_load_game(e);
    let header = e.call(LOADING_FORM_HEADER, &args![save_load]).u32();
    let form = if header != 0 {
        let header_id = e.mem.u32(header);
        e.call(LOOKUP_FORM_BY_ID, &args![header_id]).u32()
    } else {
        0
    };
    (header, form)
}

// Translated from 005a9950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the locals in the old save format (the inverse of `fn_005a9670`),
/// calling `SetVariable` for each saved variable. Files older than version
/// 0x75 store the id and value in two plain reads and never use form
/// references. Reports a wrong block header, and a block read that over- or
/// underran its recorded size.
pub fn fn_005a9950(e: &mut Engine, this: Ptr<ScriptLocals>) {
    e.with_stack(0x20, |e, frame| old_load_body(e, this, frame.addr()));
}

fn old_load_body(e: &mut Engine, this: Ptr<ScriptLocals>, frame: u32) {
    let marker = frame;
    let block_size = frame + 4;
    let count = frame + 8;
    let id_word = frame + 0xc;
    let value = frame + 0x10;
    let numeric = frame + 0x18;
    let flag = frame + 0x1c;
    let mut block_start = 0u32;
    if use_save_game_blocks(e) {
        stream_read(e, marker, 4);
        if e.mem.u32(marker) != BLOCK_MARKER {
            let (header, form) = loading_form(e);
            log_loading_form(
                e,
                header,
                form,
                &[],
                0x2bc,
                (FORMAT_BLOCK_HEADER_FORM, FORMAT_BLOCK_HEADER_PLAIN),
            );
        }
        block_start = stream_position(e);
        stream_read(e, block_size, 2);
    }
    stream_read(e, count, 2);
    let total = e.mem.u16(count) as i32;
    let mut index = 0i32;
    while index < total {
        e.mem.set_u32(id_word, 0);
        e.mem.set_f64(value, 0.0);
        if file_version(e) >= 0x75 {
            stream_read(e, id_word, 4);
            if e.mem.u32(id_word) & 0x8000_0000 != 0 {
                let id = e.mem.u32(id_word) & 0x7fff_ffff;
                e.mem.set_u32(id_word, id);
                let save_load = save_load_game(e);
                e.call(STREAM_LOAD_NUMERIC_ID, &args![save_load, numeric, 4u32]);
                e.call(PUT_NUMERIC_ID_IN_DOUBLE, &args![numeric, value]);
            } else {
                stream_read(e, value, 8);
            }
        }
        if file_version(e) < 0x75 {
            stream_read(e, id_word, 4);
            stream_read(e, value, 8);
        }
        let id = e.mem.u32(id_word);
        let variable = e.mem.f64(value);
        script_locals_set_variable(e, this, id, variable);
        index += 1;
    }
    stream_read(e, flag, 1);
    if e.mem.u8(flag) != 0 {
        let effect_data = new_block(e, 8);
        e.set(
            this,
            ScriptLocals::m_pScriptEffectData,
            Ptr::new(effect_data),
        );
        stream_read(e, effect_data, 8);
    }
    if use_save_game_blocks(e) {
        let current = stream_position(e);
        let (header, form) = loading_form(e);
        let expected = (e.mem.u16(block_size) as u32).wrapping_add(block_start);
        if current > expected {
            log_loading_form(
                e,
                header,
                form,
                &[current - expected],
                0x2eb,
                (FORMAT_OVERRUN_FORM, FORMAT_OVERRUN_PLAIN),
            );
        } else if current < expected {
            log_loading_form(
                e,
                header,
                form,
                &[expected - current],
                0x2eb,
                (FORMAT_UNDERRUN_FORM, FORMAT_UNDERRUN_PLAIN),
            );
        }
    }
}

// Translated from 005a9d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the locals: frees the variable list, clears every action flag,
/// rebuilds the variable list from the master script (when there is one) and
/// frees the effect data.
pub fn fn_005a9d00(e: &mut Engine, this: Ptr<ScriptLocals>) {
    fn_005a8c90(e, this);
    fn_005a8ea0(e, this);
    let master = e.get(this, ScriptLocals::m_pMasterScript);
    if !master.is_null() {
        let list = e.call(SCRIPT_BUILD_LOCALS, &args![master]).ptr::<()>();
        e.set(this, ScriptLocals::m_pLocalList, list);
    }
    let effect_data = e.get(this, ScriptLocals::m_pScriptEffectData);
    if !effect_data.is_null() {
        delete_block(e, effect_data.addr());
    }
    e.set(this, ScriptLocals::m_pScriptEffectData, Ptr::NULL);
}

// Translated from 005a9d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the cache of the last variable lookup (locals 0, id `0xFFFFFFFF`,
/// variable 0).
pub fn fn_005a9d60(e: &mut Engine) {
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_LAST_LOCALS, 0);
    e.mem.set_u32(tls + TLS_LAST_VARIABLE_ID, 0xffff_ffff);
    e.mem.set_u32(tls + TLS_LAST_VARIABLE, 0);
}

// Translated from 005a9db0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptLocals::SaveGame` (Xbox PDB): writes the locals into a
/// `BGSSaveGameBuffer`: a variable-sized count, then each non-zero variable
/// (id and value, or the id with its top bit set and the saved form id for a
/// form reference), the effect-data flag and data, and a byte saying whether
/// the "any form" action entry has flag `0x1000`.
pub fn script_locals_save_game(e: &mut Engine, this: Ptr<ScriptLocals>, buffer: Ptr) {
    e.with_stack(0x10, |e, frame| {
        let id_word = frame.addr();
        let numeric = frame.addr() + 4;
        let flag = frame.addr() + 8;
        let mut count = 0u32;
        let position = e.call(BUFFER_START_SIZED_VALUE, &args![buffer]).u32();
        let mut node = e.get(this, ScriptLocals::m_pLocalList).addr();
        while node != 0 {
            let variable = list_item(e, node);
            if variable != 0 && e.mem.f64(variable + 8) != 0.0 {
                let id = e.mem.u32(variable);
                if fn_005a9480(e, this, id) {
                    e.mem.set_u32(numeric, 0);
                    e.call(GET_NUMERIC_ID_FROM_DOUBLE, &args![numeric, variable + 8]);
                    // PC 005a9e37 reloads the ID after the conversion helper.
                    e.mem.set_u32(id_word, e.mem.u32(variable) | 0x8000_0000);
                    e.call(BUFFER_SAVE_BYTES, &args![buffer, id_word, 4u32, 0u32]);
                    let form_id = e.mem.u32(numeric);
                    e.call(BUFFER_SAVE_FORM_ID, &args![buffer, form_id, 0u32]);
                } else {
                    e.call(BUFFER_SAVE_BYTES, &args![buffer, variable, 4u32, 0u32]);
                    e.call(BUFFER_SAVE_BYTES, &args![buffer, variable + 8, 8u32, 0u32]);
                }
                count += 1;
            }
            node = list_next(e, node);
        }
        e.call(BUFFER_SAVE_SIZED_VALUE, &args![buffer, count, position]);
        let effect_data = e.get(this, ScriptLocals::m_pScriptEffectData).addr();
        e.mem.set_u8(flag, (effect_data != 0) as u8);
        e.call(BUFFER_SAVE_BYTES, &args![buffer, flag, 1u32, 0u32]);
        // PC 005a9ece tests the live stack byte, then reloads the pointer.
        if e.mem.u8(flag) != 0 {
            let effect_data = e.get(this, ScriptLocals::m_pScriptEffectData).addr();
            e.call(BUFFER_SAVE_BYTES, &args![buffer, effect_data, 8u32, 0u32]);
        }
        let any_form_flag = fn_005a8ef0(e, this, 0, 0x1000);
        e.mem.set_u8(flag, any_form_flag as u8);
        e.call(BUFFER_SAVE_BYTES, &args![buffer, flag, 1u32, 0u32]);
    });
}

// Translated from 005a9f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ScriptLocals::LoadGame` (Xbox PDB): reads what `script_locals_save_game`
/// wrote from a `BGSLoadGameBuffer`: the variables (set with `SetVariable`;
/// a form reference id is read as a form id and put into the double), the
/// effect data, and, from buffer version 0x15, the "any form" action flag
/// byte. With no action list it is built by `ScriptRunManager::InitActionList`
/// for the master script, otherwise all flags are cleared; the saved flag
/// then sets `0x1000` on the "any form" entry.
pub fn script_locals_load_game(e: &mut Engine, this: Ptr<ScriptLocals>, buffer: Ptr) {
    e.with_stack(0x20, |e, frame| {
        let id_word = frame.addr();
        let numeric = frame.addr() + 4;
        let value = frame.addr() + 8;
        let effect_flag = frame.addr() + 0x10;
        let action_flag = frame.addr() + 0x11;
        let count = e.call(BUFFER_LOAD_SIZED_VALUE, &args![buffer]).u32();
        let mut index = 0u32;
        while index < count {
            e.mem.set_f64(value, 0.0);
            e.mem.set_u32(id_word, 0);
            e.call(BUFFER_LOAD_BYTES, &args![buffer, id_word, 4u32]);
            if e.mem.u32(id_word) & 0x8000_0000 != 0 {
                let id = e.mem.u32(id_word) & 0x7fff_ffff;
                e.mem.set_u32(id_word, id);
                e.mem.set_u32(numeric, 0);
                e.call(BUFFER_LOAD_FORM_ID_OV2, &args![buffer, numeric]);
                e.call(PUT_NUMERIC_ID_IN_DOUBLE, &args![numeric, value]);
            } else {
                e.call(BUFFER_LOAD_BYTES, &args![buffer, value, 8u32]);
            }
            let id = e.mem.u32(id_word);
            let variable = e.mem.f64(value);
            script_locals_set_variable(e, this, id, variable);
            index += 1;
        }
        e.mem.set_u8(effect_flag, 0);
        e.call(BUFFER_LOAD_BYTES, &args![buffer, effect_flag, 1u32]);
        if e.mem.u8(effect_flag) != 0 {
            let effect_data = new_block(e, 8);
            e.set(
                this,
                ScriptLocals::m_pScriptEffectData,
                Ptr::new(effect_data),
            );
            e.call(BUFFER_LOAD_BYTES, &args![buffer, effect_data, 8u32]);
        }
        e.mem.set_u8(action_flag, 0);
        let version = e.vcall(buffer.addr(), 0, &args![]).u8();
        if version >= 0x15 {
            e.call(BUFFER_LOAD_BYTES, &args![buffer, action_flag, 1u32]);
        }
        if e.get(this, ScriptLocals::m_pActionList).is_null() {
            let master = e.get(this, ScriptLocals::m_pMasterScript);
            let second = e.vcall(buffer.addr(), 8, &args![]).u32();
            let manager = e.call(SCRIPT_RUN_MANAGER_INSTANCE, &args![]).u32();
            e.call(
                SCRIPT_RUN_MANAGER_INIT_ACTION_LIST,
                &args![manager, master, second, this],
            );
        } else {
            fn_005a8ea0(e, this);
        }
        if e.mem.u8(action_flag) != 0 {
            script_locals_set_action_flag(e, this, 0, 0x1000);
        }
    });
}

// Translated from 005aa090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same body as `fn_005a9d00` (reset the locals), for a method that
/// takes one stack word it never reads.
pub fn fn_005aa090(e: &mut Engine, this: Ptr<ScriptLocals>, _unused_0: u32) {
    fn_005a9d00(e, this);
}

// Translated from 005aa0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::Script` (Xbox PDB): the form constructor, the `Script` vtable,
/// the two embedded lists, then the field initialization of `fn_005aa220`.
pub fn script_script(e: &mut Engine, this: Ptr<Script>) -> Ptr<Script> {
    e.call(TES_FORM_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), SCRIPT_VTABLE);
    let ref_objects = this.at(Script::listRefObjects);
    e.call(LIST_CONSTRUCT, &args![ref_objects]);
    let variables = this.at(Script::listVariables);
    e.call(LIST_CONSTRUCT, &args![variables]);
    fn_005aa220(e, this);
    this
}

// Translated from 005aa170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `Script`: destroys it and, with bit 0 of
/// `flags`, frees the block.
pub fn fn_005aa170(e: &mut Engine, this: Ptr<Script>, flags: u32) -> Ptr<Script> {
    fn_005aa1a0(e, this);
    if flags & 1 != 0 {
        delete_block(e, this.addr());
    }
    this
}

// Translated from 005aa1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of `Script` (the decompiler calls it
/// `CSettingsStore::~CSettingsStore`, a wrong library match): sets the
/// vtable, releases the text, data and lists (`fn_005aa2a0`), destroys the
/// two embedded lists and the `TESForm` base.
pub fn fn_005aa1a0(e: &mut Engine, this: Ptr<Script>) {
    e.mem.set_u32(this.addr(), SCRIPT_VTABLE);
    fn_005aa2a0(e, this);
    let variables = this.at(Script::listVariables);
    e.call(LIST_DESTRUCT, &args![variables]);
    let ref_objects = this.at(Script::listRefObjects);
    e.call(LIST_DESTRUCT, &args![ref_objects]);
    e.call(TES_FORM_DESTRUCT, &args![this]);
}

// Translated from 005aa220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the plain fields of a `Script`: the 0x14-byte header zeroed,
/// text and data null, the system-output byte cleared, the owner quest null,
/// the three float timers 0.0, and the form type set to 0x11.
pub fn fn_005aa220(e: &mut Engine, this: Ptr<Script>) {
    let header = this.at(Script::m_header);
    e.call(MEMSET, &args![header, 0u32, 0x14u32]);
    e.set(this, Script::m_data, Ptr::NULL);
    e.set(this, Script::m_text, Ptr::NULL);
    let tls = e.tls();
    e.mem.set_u8(tls + TLS_SYSTEM_OUTPUT, 0);
    e.set(this, Script::pOwnerQuest, Ptr::NULL);
    e.set(this, Script::fProfilerTimer, 0.0);
    e.set(this, Script::fQuestScriptDelay, 0.0);
    e.set(this, Script::fQuestScriptGetSecondsBuffer, 0.0);
    e.call(SET_FORM_TYPE, &args![this, 0x11u32]);
}

// Translated from 005aa2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees the data and text of a `Script` and empties its variable and
/// referenced-object lists.
pub fn fn_005aa2a0(e: &mut Engine, this: Ptr<Script>) {
    let data = e.get(this, Script::m_data);
    delete_block(e, data.addr());
    let text = e.get(this, Script::m_text);
    delete_block(e, text.addr());
    fn_005aa2e0(e, this);
    fn_005aa350(e, this);
}

// Translated from 005aa2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes every item of the script's variable list (`listVariables`) and
/// removes it from the list, until the list is empty.
pub fn fn_005aa2e0(e: &mut Engine, this: Ptr<Script>) {
    let list = e.call(SCRIPT_VARIABLE_LIST, &args![this]).u32();
    while list != 0 && !list_is_empty(e, list) {
        let item = list_item(e, list);
        if item != 0 {
            e.call(SCRIPT_VARIABLE_DELETE, &args![item, 1u32]);
        }
        e.call(LIST_POP_FIRST, &args![list]);
    }
}

// Translated from 005aa350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forgets this script as the last reference-search script (TLS +0x288),
/// then deletes every item of the referenced-object list (`listRefObjects`)
/// and removes it from the list, until the list is empty.
pub fn fn_005aa350(e: &mut Engine, this: Ptr<Script>) {
    let tls = e.tls();
    if e.mem.u32(tls + TLS_LAST_REF_SEARCH_SCRIPT) == this.addr() {
        e.mem.set_u32(tls + TLS_LAST_REF_SEARCH_SCRIPT, 0);
    }
    let list = e.call(SCRIPT_REF_OBJECT_LIST, &args![this]).u32();
    while list != 0 && !list_is_empty(e, list) {
        let item = list_item(e, list);
        if item != 0 {
            fn_005aa3f0(e, Ptr::new(item), 1);
        }
        e.call(LIST_POP_FIRST, &args![list]);
    }
}

// Translated from 005aa3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `SCRIPT_REFERENCED_OBJECT` (the body
/// `007d9ec0` destroys its editor ID string): frees the block with bit 0 of
/// `flags`.
pub fn fn_005aa3f0(e: &mut Engine, this: Ptr<ScriptReferencedObject>, flags: u32) -> Ptr {
    e.call(SCRIPT_REFERENCED_OBJECT_DESTRUCT, &args![this]);
    if flags & 1 != 0 {
        delete_block(e, this.addr());
    }
    this.cast()
}

// Translated from 005aa420 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::CopyResultScript` (Xbox PDB): makes this script a copy of
/// `other` (nothing for null): copies the 0x14-byte header and clears its
/// first word, copies the compile data (`SetCompileData(data size, data)`),
/// the referenced objects and the variables (each after emptying its list),
/// the text (`SetText`, null when `other` has none), and the quest-script
/// flag (bit 3 of the form flags).
pub fn script_copy_result_script(e: &mut Engine, this: Ptr<Script>, other: Ptr<Script>) {
    if other.is_null() {
        return;
    }
    let other_header = e.call(SCRIPT_HEADER_OF, &args![other]).u32();
    let header = this.at(Script::m_header);
    e.call(MEMCPY, &args![header, other_header, 0x14u32]);
    e.set(header, ScriptHeader::variableCount, 0);
    let compile_data = e.call(SCRIPT_COMPILE_DATA_OF, &args![other]).u32();
    let data_size = e.get(header, ScriptHeader::dataSize);
    e.call(
        SCRIPT_SET_COMPILE_DATA,
        &args![this, data_size, compile_data],
    );
    fn_005aa350(e, this);
    let own_refs = e.call(SCRIPT_REF_OBJECT_LIST, &args![this]).u32();
    let other_refs = e.call(SCRIPT_REF_OBJECT_LIST, &args![other]).u32();
    e.call(SCRIPT_COPY_REF_OBJECTS, &args![other_refs, own_refs, this]);
    fn_005aa2e0(e, this);
    let own_vars = e.call(SCRIPT_VARIABLE_LIST, &args![this]).u32();
    let other_vars = e.call(SCRIPT_VARIABLE_LIST, &args![other]).u32();
    e.call(SCRIPT_COPY_VARIABLES, &args![other_vars, own_vars, this]);
    if e.call(SCRIPT_TEXT_OF, &args![other]).u32() != 0 {
        let text = e.call(SCRIPT_TEXT_OF, &args![other]).u32();
        e.call(SCRIPT_SET_TEXT, &args![this, text]);
    } else {
        e.call(SCRIPT_SET_TEXT, &args![this, 0u32]);
    }
    let flag = e.call(FORM_FLAG_BIT_3, &args![other]).u8() as u32;
    e.call(SCRIPT_SET_QUEST_SCRIPT_FLAG, &args![this, flag]);
}

// Translated from 005aa500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::AddPendingDisabledReference` (Xbox PDB): under the lock, takes
/// `reference` off the to-enable list, then (unless null) adds it to the
/// to-disable list, or with `fade` to the fade-disable list, when it is not
/// already pending there. A null reference is only locked around.
pub fn script_add_pending_disabled_reference(e: &mut Engine, reference: u32, fade: u8) {
    with_reference_lock(e, |e| {
        if reference == 0 {
            return;
        }
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), reference);
            e.call(LIST_REMOVE, &args![REFERENCES_TO_ENABLE, slot]);
            if fade == 0 {
                if !script_is_pending_disabled_reference(e, reference) {
                    e.call(LIST_ADD_HEAD, &args![REFERENCES_TO_DISABLE, slot]);
                }
            } else if !script_is_pending_fade_disabled_reference(e, reference) {
                e.call(LIST_ADD_HEAD, &args![REFERENCES_TO_FADE_DISABLE, slot]);
            }
        });
    });
}

// Translated from 005aa580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::AddPendingEnabledReference` (Xbox PDB): under the lock, takes
/// `reference` off the to-disable list and adds it to the to-enable list
/// unless it is already pending there. Null does nothing.
pub fn script_add_pending_enabled_reference(e: &mut Engine, reference: u32) {
    with_reference_lock(e, |e| {
        if reference == 0 {
            return;
        }
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), reference);
            e.call(LIST_REMOVE, &args![REFERENCES_TO_DISABLE, slot]);
            if !script_is_pending_enabled_reference(e, reference) {
                e.call(LIST_ADD_HEAD, &args![REFERENCES_TO_ENABLE, slot]);
            }
        });
    });
}

// Translated from 005aa5d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::RemoveDelayedScriptActionReference` (Xbox PDB): under the lock,
/// takes `reference` off the to-enable, to-disable, fade-disable and
/// to-delete lists. Null does nothing (and does not lock).
pub fn script_remove_delayed_script_action_reference(e: &mut Engine, reference: u32) {
    if reference == 0 {
        return;
    }
    with_reference_lock(e, |e| {
        e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), reference);
            for list in [
                REFERENCES_TO_ENABLE,
                REFERENCES_TO_DISABLE,
                REFERENCES_TO_FADE_DISABLE,
                REFERENCES_TO_DELETE,
            ] {
                e.call(LIST_REMOVE, &args![list, slot]);
            }
        });
    });
}

/// Whether `reference` (non-null) is in `list`, under the lock.
fn is_pending_in(e: &mut Engine, list: u32, reference: u32) -> bool {
    with_reference_lock(e, |e| {
        let mut found = false;
        if reference != 0 {
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), reference);
                if e.call(LIST_CONTAINS, &args![list, slot]).bool() {
                    found = true;
                }
            });
        }
        found
    })
}

// Translated from 005aa630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPendingDisabledReference` (Xbox PDB): under the lock, whether
/// the to-disable list contains `reference`.
pub fn script_is_pending_disabled_reference(e: &mut Engine, reference: u32) -> bool {
    is_pending_in(e, REFERENCES_TO_DISABLE, reference)
}

// Translated from 005aa680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPendingEnabledReference` (Xbox PDB): under the lock, whether
/// the to-enable list contains `reference`.
pub fn script_is_pending_enabled_reference(e: &mut Engine, reference: u32) -> bool {
    is_pending_in(e, REFERENCES_TO_ENABLE, reference)
}

// Translated from 005aa6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::IsPendingFadeDisabledReference` (Xbox PDB): under the lock,
/// whether the fade-disable list contains `reference`.
pub fn script_is_pending_fade_disabled_reference(e: &mut Engine, reference: u32) -> bool {
    is_pending_in(e, REFERENCES_TO_FADE_DISABLE, reference)
}

// Translated from 005aa720 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::RunDelayedScriptActionsOnReferences` (Xbox PDB): under the lock
/// enables every reference of the to-enable list and disables every one of
/// the to-disable list (each taken off the front first); then walks the
/// fade-disable list: a reference whose virtual `+0x1D0` gives an object
/// whose virtual `+0x10` gives a target that `fn_005aa8c0` accepts stays in
/// the list, any other is disabled and removed; finally deletes (virtual
/// `+0x10` with 1) every reference of the to-delete list.
pub fn script_run_delayed_script_actions_on_references(e: &mut Engine) {
    with_reference_lock(e, |e| {
        while !list_is_empty(e, REFERENCES_TO_ENABLE) {
            let reference = list_item(e, REFERENCES_TO_ENABLE);
            e.call(LIST_POP_FIRST, &args![REFERENCES_TO_ENABLE]);
            e.call(REFERENCE_ENABLE, &args![reference]);
        }
        while !list_is_empty(e, REFERENCES_TO_DISABLE) {
            let reference = list_item(e, REFERENCES_TO_DISABLE);
            e.call(LIST_POP_FIRST, &args![REFERENCES_TO_DISABLE]);
            e.call(REFERENCE_DISABLE, &args![reference]);
        }
        let mut node = REFERENCES_TO_FADE_DISABLE;
        while node != 0 && !list_is_empty(e, node) {
            let mut following = list_next(e, node);
            let reference = list_item(e, node);
            let object = e.vcall(reference, 0x1d0, &args![]).u32();
            let target = if object != 0 {
                e.vcall(object, 0x10, &args![]).u32()
            } else {
                0
            };
            if target == 0 || !fn_005aa8c0(e, Ptr::new(target)) {
                e.call(REFERENCE_DISABLE, &args![reference]);
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), reference);
                    e.call(LIST_REMOVE, &args![REFERENCES_TO_FADE_DISABLE, slot]);
                });
                if node == REFERENCES_TO_FADE_DISABLE {
                    following = REFERENCES_TO_FADE_DISABLE;
                }
            }
            node = following;
        }
        while !list_is_empty(e, REFERENCES_TO_DELETE) {
            let reference = list_item(e, REFERENCES_TO_DELETE);
            e.call(LIST_POP_FIRST, &args![REFERENCES_TO_DELETE]);
            if reference != 0 {
                e.vcall(reference, 0x10, &args![1u32]);
            }
        }
    });
}

// Translated from 005aa8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// False when the object does not have flag `0x8000` (`fn_005aa910`) and the
/// fade byte (`011ad7b4`) is set and the float at +0xB8 is not greater than
/// zero (an unordered value counts as not greater); true otherwise.
pub fn fn_005aa8c0(e: &mut Engine, this: Ptr) -> bool {
    if !fn_005aa910(e, this) && e.global::<u8>(FADE_ENABLED_BYTE) != 0 {
        let value = e.mem.f32(this.addr() + 0xb8) as f64;
        let zero: f64 = e.global(ZERO_DOUBLE);
        if value.partial_cmp(&zero) != Some(std::cmp::Ordering::Greater) {
            return false;
        }
    }
    true
}

// Translated from 005aa910 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the object has flag `0x8000` in the word at +0x30 (`00456630`).
pub fn fn_005aa910(e: &mut Engine, this: Ptr) -> bool {
    e.call(FLAGS_TEST, &args![this, 0x8000u32]).bool()
}

// Translated from 005aa930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SavePendingReferences` (Xbox PDB): under the lock, saves into
/// the buffer, for each of the to-enable, to-disable, to-delete and
/// fade-disable lists in that order, a variable-sized count followed by the
/// form id of every non-null reference.
pub fn script_save_pending_references(e: &mut Engine, buffer: Ptr) {
    with_reference_lock(e, |e| {
        for list in [
            REFERENCES_TO_ENABLE,
            REFERENCES_TO_DISABLE,
            REFERENCES_TO_DELETE,
            REFERENCES_TO_FADE_DISABLE,
        ] {
            let mut count = 0u32;
            let position = e.call(BUFFER_START_SIZED_VALUE, &args![buffer]).u32();
            let mut node = list;
            while node != 0 {
                let reference = list_item(e, node);
                if reference != 0 {
                    e.call(BUFFER_SAVE_FORM_ID_OV2, &args![buffer, reference, 0u32]);
                    count += 1;
                }
                node = list_next(e, node);
            }
            e.call(BUFFER_SAVE_SIZED_VALUE, &args![buffer, count, position]);
        }
    });
}

// ---- Helpers of the second half -------------------------------------------

/// The `SCRIPT_HEADER` embedded in a `Script`.
fn header_of(this: Ptr<Script>) -> Ptr<ScriptHeader> {
    this.at(Script::m_header)
}

/// The form-type byte of a form (`00401170`).
fn form_type(e: &mut Engine, form: u32) -> u32 {
    e.call(FORM_TYPE_OF, &args![form]).u8() as u32
}

/// A form's id (the word at +0xC, `0084e3a0`).
fn form_id_of(e: &mut Engine, form: u32) -> u32 {
    e.call(FORM_ID_OF, &args![form]).u32()
}

/// Whether bit 3 of the form flags of `form` is set (`004013e0`).
fn form_flag_bit_3(e: &mut Engine, form: u32) -> bool {
    e.call(FORM_FLAG_BIT_3, &args![form]).bool()
}

/// `new` of `size` bytes zeroed (`005aaee0(1, size)`).
fn new_zeroed(e: &mut Engine, size: u32) -> u32 {
    fn_005aaee0(e, 1, size)
}

/// The name of a `Script` as the game shows it (virtual `+0x130`).
fn script_type_name(e: &mut Engine, this: Ptr<Script>) -> u32 {
    form_type_name(e, this.addr())
}

/// Adds `item` at the tail of `list` (the game passes the address of a
/// local that holds it).
fn list_add_tail(e: &mut Engine, list: u32, item: u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), item);
        e.call(LIST_ADD_TAIL, &args![list, slot]);
    });
}

/// The TLS block of the thread.
fn tls_block(e: &mut Engine) -> u32 {
    e.tls()
}

// Translated from 005aaaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::LoadPendingReferences` (Xbox PDB): under the lock, empties the
/// four pending lists (to-enable, to-disable, to-delete, fade-disable),
/// then for each in that order reads a variable-sized count from `buffer`
/// and, for each entry, reads a form id and adds the reference it looks up
/// at the head of the list. As in the game, a `TESObjectREFR` is constructed
/// for every entry first (0x68 bytes) and left unused.
pub fn script_load_pending_references(e: &mut Engine, buffer: Ptr) {
    with_reference_lock(e, |e| {
        let lists = [
            REFERENCES_TO_ENABLE,
            REFERENCES_TO_DISABLE,
            REFERENCES_TO_DELETE,
            REFERENCES_TO_FADE_DISABLE,
        ];
        for list in lists {
            e.call(LIST_CLEAR, &args![list]);
        }
        for list in lists {
            let count = e.call(BUFFER_LOAD_SIZED_VALUE, &args![buffer]).u32();
            for _ in 0..count {
                let block = new_block(e, 0x68);
                if block != 0 {
                    e.call(REFERENCE_CONSTRUCT, &args![block]);
                }
                let id = e.call(BUFFER_LOAD_FORM_ID, &args![buffer]).u32();
                let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
                let reference = e
                    .call(
                        RT_DYNAMIC_CAST,
                        &args![form, 0u32, RTTI_SOURCE_FORM, RTTI_TARGET_REFERENCE, 0u32],
                    )
                    .u32();
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), reference);
                    e.call(LIST_ADD_HEAD, &args![list, slot]);
                });
            }
        }
    });
}

// Translated from 005aae20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Under the lock, empties the four pending reference lists (to-enable,
/// to-disable, to-delete, fade-disable).
pub fn fn_005aae20(e: &mut Engine) {
    with_reference_lock(e, |e| {
        for list in [
            REFERENCES_TO_ENABLE,
            REFERENCES_TO_DISABLE,
            REFERENCES_TO_DELETE,
            REFERENCES_TO_FADE_DISABLE,
        ] {
            e.call(LIST_CLEAR, &args![list]);
        }
    });
}

// Translated from 005aae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetCompileData` (Xbox PDB): frees the compiled data, sets the
/// header's data size to `size` and, when it is not zero, allocates that
/// many zeroed bytes and copies `data` into them.
pub fn script_set_compile_data(e: &mut Engine, this: Ptr<Script>, size: u32, data: u32) {
    let old = e.get(this, Script::m_data);
    delete_block(e, old.addr());
    e.set(this, Script::m_data, Ptr::NULL);
    let header = header_of(this);
    e.set(header, ScriptHeader::dataSize, size);
    if size != 0 {
        let block = new_zeroed(e, size);
        e.set(this, Script::m_data, Ptr::new(block));
        e.call(MEMCPY, &args![block, data, size]);
    }
}

// Translated from 005aaee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates `count * size` bytes from the memory manager and zeroes them
/// (a `calloc`).
pub fn fn_005aaee0(e: &mut Engine, count: u32, size: u32) -> u32 {
    let total = size.wrapping_mul(count);
    let manager = e.call(MEMORY_MANAGER_GET, &args![]).u32();
    let block = e
        .call(MEMORY_MANAGER_ALLOCATE, &args![manager, total])
        .u32();
    e.call(MEMSET, &args![block, 0u32, total]);
    block
}

// Translated from 005aaf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::Init` (Xbox PDB): sets this script from the compiler's output
/// `buffer`. Runs virtual `+0x88`; a buffer with no script name only sets
/// the deleted flag (virtual `+0xC4`, 1); otherwise, when `fn_005ab040` finds
/// the buffer different from this script, sets the editor id (virtual
/// `+0x134`) from the name, copies the 0x14-byte header, sets the compile
/// data, empties and refills the referenced-object and variable lists, then
/// clears the deleted flag and sets the altered flag (virtual `+0xC8`, 1).
pub fn script_init(e: &mut Engine, this: Ptr<Script>, buffer: Ptr<ScriptCompileData>) {
    e.vcall(this.addr(), SLOT_INIT_ITEM, &args![]);
    let name = buffer.at(ScriptCompileData::cScriptName);
    if e.call(STRING_LENGTH, &args![name]).u32() == 0 {
        e.vcall(this.addr(), SLOT_SET_DELETED, &args![1u32]);
        return;
    }
    if !fn_005ab040(e, this, buffer) {
        return;
    }
    let text = e.call(STRING_DATA, &args![name]).u32();
    e.vcall(this.addr(), SLOT_SET_EDITOR_ID, &args![text]);
    let source_header = buffer.at(ScriptCompileData::header);
    e.call(MEMCPY, &args![header_of(this), source_header, 0x14u32]);
    let size = e.get(header_of(this), ScriptHeader::dataSize);
    let output = e.get(buffer, ScriptCompileData::pOutput);
    script_set_compile_data(e, this, size, output.addr());
    fn_005aa350(e, this);
    fn_005aa2e0(e, this);
    if e.get(header_of(this), ScriptHeader::dataSize) != 0 {
        let own_refs = e.call(SCRIPT_REF_OBJECT_LIST, &args![this]).u32();
        let other_refs = e.call(SCRIPT_REF_OBJECT_LIST, &args![buffer]).u32();
        fn_005ab7f0(e, other_refs, own_refs, this.addr());
        let own_vars = e.call(SCRIPT_VARIABLE_LIST, &args![this]).u32();
        let other_vars = e.call(SCRIPT_BUFFER_VARIABLE_LIST, &args![buffer]).u32();
        fn_005ab930(e, other_vars, own_vars, this.addr());
    }
    e.vcall(this.addr(), SLOT_SET_DELETED, &args![0u32]);
    e.vcall(this.addr(), SLOT_SET_ALTERED, &args![1u32]);
}

// Translated from 005ab040 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the compiler's output `buffer` differs from this script (true) or
/// not (false): true when the unnamed test `00474cb0` passes and the name
/// does not compare equal to the script's type name, when the data size,
/// the referenced-object count, or any compiled byte differs, when the
/// variable lists have different counts or a pair of variables differs
/// (`005e29a0`), or when the referenced-object lists have different counts
/// or a pair differs in the form word at +8.
pub fn fn_005ab040(e: &mut Engine, this: Ptr<Script>, buffer: Ptr<ScriptCompileData>) -> bool {
    let name = buffer.at(ScriptCompileData::cScriptName);
    if e.call(SCRIPT_IS_NAMED_CHECK, &args![this]).u32() != 0
        && e.call(STRING_LENGTH, &args![name]).u32() != 0
    {
        let text = e.call(STRING_DATA, &args![name]).u32();
        let type_name = script_type_name(e, this);
        if e.call(STRING_COMPARE, &args![type_name, text]).u32() != 0 {
            return true;
        }
    }
    let source_header = buffer.at(ScriptCompileData::header);
    let header = header_of(this);
    if e.get(header, ScriptHeader::dataSize) != e.get(source_header, ScriptHeader::dataSize) {
        return true;
    }
    if e.get(header, ScriptHeader::refObjectCount)
        != e.get(source_header, ScriptHeader::refObjectCount)
    {
        return true;
    }
    let output = e.get(buffer, ScriptCompileData::pOutput).addr();
    let data = e.get(this, Script::m_data).addr();
    for i in 0..e.get(header, ScriptHeader::dataSize) {
        if e.mem.i8(data + i) != e.mem.i8(output + i) {
            return true;
        }
    }
    let mut other = e.call(SCRIPT_BUFFER_VARIABLE_LIST, &args![buffer]).u32();
    let mut own = e.call(SCRIPT_VARIABLE_LIST, &args![this]).u32();
    if e.call(LIST_COUNT, &args![other]).u32() != e.call(LIST_COUNT, &args![own]).u32() {
        return true;
    }
    while other != 0 && own != 0 {
        let own_item = list_item(e, own);
        let other_item = list_item(e, other);
        if other_item != 0
            && own_item != 0
            && e.call(VARIABLE_DIFFERS, &args![other_item, own_item])
                .bool()
        {
            return true;
        }
        other = list_next(e, other);
        own = list_next(e, own);
    }
    let mut other = e.call(SCRIPT_REF_OBJECT_LIST, &args![buffer]).u32();
    let mut own = e.call(SCRIPT_REF_OBJECT_LIST, &args![this]).u32();
    if e.call(LIST_COUNT, &args![other]).u32() != e.call(LIST_COUNT, &args![own]).u32() {
        return true;
    }
    while other != 0 && own != 0 {
        let own_item = list_item(e, own);
        let other_item = list_item(e, other);
        // SCRIPT_REFERENCED_OBJECT::pForm-like word at +8.
        if other_item != 0 && own_item != 0 && e.mem.u32(other_item + 8) != e.mem.u32(own_item + 8)
        {
            return true;
        }
        other = list_next(e, other);
        own = list_next(e, own);
    }
    false
}

// Translated from 005ab240 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::CompareResultScripts` (Xbox PDB): whether `other` differs from
/// this script (true for a null `other`): different data size, referenced
/// object count or compiled flag, a different compiled byte, a different
/// text (one missing, a different length, or `00469880` non-zero), or a
/// pair of referenced objects whose words at +8 differ.
pub fn script_compare_result_scripts(
    e: &mut Engine,
    this: Ptr<Script>,
    other: Ptr<Script>,
) -> bool {
    if other.is_null() {
        return true;
    }
    let other_header = e.call(SCRIPT_HEADER_OF, &args![other]).u32();
    let header = header_of(this);
    // SCRIPT_HEADER::dataSize (+8), refObjectCount (+4), bIsCompiled (+0x12).
    if e.get(header, ScriptHeader::dataSize) != e.mem.u32(other_header + 8) {
        return true;
    }
    if e.get(header, ScriptHeader::refObjectCount) != e.mem.u32(other_header + 4) {
        return true;
    }
    if e.get(header, ScriptHeader::bIsCompiled) as u8 != e.mem.u8(other_header + 0x12) {
        return true;
    }
    let other_data = e.call(SCRIPT_COMPILE_DATA_OF, &args![other]).u32();
    let data = e.get(this, Script::m_data).addr();
    for i in 0..e.get(header, ScriptHeader::dataSize) {
        if e.mem.i8(data + i) != e.mem.i8(other_data + i) {
            return true;
        }
    }
    let own_text = e.call(SCRIPT_TEXT_OF, &args![this]).u32();
    let other_text = if own_text != 0 {
        e.call(SCRIPT_TEXT_OF, &args![other]).u32()
    } else {
        0
    };
    if own_text != 0 && other_text != 0 {
        let own_text = e.call(SCRIPT_TEXT_OF, &args![this]).u32();
        let own_length = e.call(STRLEN, &args![own_text]).u32();
        let other_text = e.call(SCRIPT_TEXT_OF, &args![other]).u32();
        let other_length = e.call(STRLEN, &args![other_text]).u32();
        if own_length != other_length {
            return true;
        }
        let other_text = e.call(SCRIPT_TEXT_OF, &args![other]).u32();
        let own_text = e.call(SCRIPT_TEXT_OF, &args![this]).u32();
        if e.call(TEXT_COMPARE, &args![own_text, other_text]).u32() != 0 {
            return true;
        }
    } else {
        if e.call(SCRIPT_TEXT_OF, &args![this]).u32() != 0 {
            return true;
        }
        if e.call(SCRIPT_TEXT_OF, &args![other]).u32() != 0 {
            return true;
        }
    }
    let mut other_node = e.call(SCRIPT_REF_OBJECT_LIST, &args![other]).u32();
    let mut own_node = e.call(SCRIPT_REF_OBJECT_LIST, &args![this]).u32();
    while other_node != 0 && own_node != 0 {
        let own_item = list_item(e, own_node);
        let other_item = list_item(e, other_node);
        if other_item != 0 && own_item != 0 && e.mem.u32(other_item + 8) != e.mem.u32(own_item + 8)
        {
            return true;
        }
        other_node = list_next(e, other_node);
        own_node = list_next(e, own_node);
    }
    false
}

// Translated from 005ab400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::InitItem` (Xbox PDB): does nothing when bit 3 of this script's
/// form flags is set or `source` is null. Otherwise (inside a scope guard)
/// takes the file of `source` for this script when they differ and the
/// unnamed test `004077c0` passes, then resolves each referenced object:
/// the id at +8 is rebased on the file (`AddCompileIndex`) and replaced by
/// the form it names; an object with neither a form nor a variable id, or
/// whose form is not found, is logged, removed from the list, and clears the
/// script's data size. Logs a corrupt count when the objects seen differ
/// from the header's. Quest scripts get their delay set from the setting
/// (spread over eight steps by the counter of `011cad40`, or zero when the
/// quest has its own delay), and the script takes `source` as its owner
/// quest when that is a form of type 0x47.
pub fn script_init_item(e: &mut Engine, this: Ptr<Script>, source: Ptr) {
    if form_flag_bit_3(e, this.addr()) || source.is_null() {
        return;
    }
    with_scope_guard(e, INIT_ITEM_SCOPE_LINE, |e| {
        if source.addr() != this.addr() && e.call(SCRIPT_INIT_ITEM_CHECK, &args![this]).bool() {
            let file = e.call(FORM_GET_FILE, &args![source, 0xffff_ffffu32]).u32();
            e.call(FORM_SET_FILE, &args![this, file]);
        }
        let mut index = 0u32;
        let mut node = e.call(SCRIPT_REF_OBJECT_LIST, &args![this]).u32();
        let mut previous = 0u32;
        let file = e.call(FORM_GET_FILE, &args![source, 0xffff_ffffu32]).u32();
        while node != 0 && !list_is_empty(e, node) {
            let object = list_item(e, node);
            let mut id = e.mem.u32(object + 8);
            let variable_id = e.mem.u32(object + 0xc);
            let resolved = if id != 0 {
                id = e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), id);
                    e.call(FORM_ADD_COMPILE_INDEX, &args![slot, file]);
                    e.mem.u32(slot.addr())
                });
                let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
                e.mem.set_u32(object + 8, form);
                form != 0
            } else {
                variable_id != 0
            };
            if resolved {
                previous = node;
                node = list_next(e, node);
            } else {
                if previous != 0 {
                    let slot = e.call(LIST_ITEM_SLOT, &args![node]).u32();
                    e.call(LIST_REMOVE, &args![previous, slot]);
                    node = list_next(e, previous);
                } else {
                    e.call(LIST_POP_FIRST, &args![node]);
                }
                let name = script_type_name(e, this);
                let script_id = form_id_of(e, this.addr());
                if id != 0 {
                    script_log(
                        e,
                        &args![FORMAT_REFERENCED_OBJECT_NOT_FOUND, id, script_id, name],
                    );
                } else {
                    script_log(
                        e,
                        &args![FORMAT_REFERENCED_OBJECT_INVALID, index, script_id, name],
                    );
                }
                e.set(header_of(this), ScriptHeader::dataSize, 0);
            }
            index += 1;
        }
        let expected = e.get(header_of(this), ScriptHeader::refObjectCount);
        if index != expected && e.get(header_of(this), ScriptHeader::dataSize) != 0 {
            let name = script_type_name(e, this);
            let script_id = form_id_of(e, this.addr());
            script_log(
                e,
                &args![
                    FORMAT_REFERENCE_LIST_CORRUPT,
                    script_id,
                    name,
                    expected,
                    index
                ],
            );
            e.set(header_of(this), ScriptHeader::dataSize, 0);
        }
        if !e.get(this, Script::m_data).is_null() {
            e.call(EMPTY_METHOD, &args![this]);
        }
        let header = e.call(SCRIPT_HEADER_OF, &args![this]).u32();
        // SCRIPT_HEADER::bIsQuestScript (+0x10).
        if e.mem.u8(header + 0x10) != 0 {
            let setting = e
                .call(QUEST_DELAY_ACCESSOR, &args![QUEST_DELAY_SETTING])
                .u32();
            if e.mem.f32(setting) > 0.0 {
                let setting = e
                    .call(QUEST_DELAY_ACCESSOR, &args![QUEST_DELAY_SETTING])
                    .u32();
                let mut step = e.mem.f32(setting);
                if e.global::<u32>(QUEST_DELAY_COUNTER) == 0 {
                    let divisor = e.global::<f64>(QUEST_DELAY_DIVISOR);
                    for i in 0..8u32 {
                        step = (step as f64 / divisor) as f32;
                        e.mem.set_f32(QUEST_DELAY_TABLE + i * 4, step);
                    }
                }
                let mut bits = (e.global::<u32>(QUEST_DELAY_COUNTER) & 0xff) as i32;
                let mut table_index = 0u32;
                let owner = e.get(this, Script::pOwnerQuest);
                if !owner.is_null() && e.call(QUEST_DELAY_OF, &args![owner]).f64() > 0.0 {
                    e.set(this, Script::fQuestScriptDelay, 0.0);
                } else if bits == 0 {
                    let setting = e
                        .call(QUEST_DELAY_ACCESSOR, &args![QUEST_DELAY_SETTING])
                        .u32();
                    let value = e.mem.f32(setting);
                    e.set(this, Script::fQuestScriptDelay, value);
                } else {
                    while bits != 0 {
                        if bits & 1 != 0 {
                            let delay = e.get(this, Script::fQuestScriptDelay) as f64;
                            let part = e.mem.f32(QUEST_DELAY_TABLE + table_index * 4) as f64;
                            e.set(this, Script::fQuestScriptDelay, (delay + part) as f32);
                        }
                        table_index += 1;
                        bits >>= 1;
                    }
                }
                let counter = e.global::<u32>(QUEST_DELAY_COUNTER);
                e.set_global::<u32>(QUEST_DELAY_COUNTER, counter.wrapping_add(1));
            }
        }
        if form_type(e, source.addr()) == 0x47 {
            e.set(this, Script::pOwnerQuest, source);
        }
        e.call(SCRIPT_SET_QUEST_SCRIPT_FLAG, &args![this, 1u32]);
    });
}

// Translated from 005ab7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs `Script::InitItem` with this script as its own source.
pub fn fn_005ab7d0(e: &mut Engine, this: Ptr<Script>) {
    script_init_item(e, this, this.cast());
}

// Translated from 005ab7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a list of `SCRIPT_REFERENCED_OBJECT` pointers: for each item of
/// `source` (while the node is not an empty one), makes a copy
/// (`fn_005ab8b0` into a new 0x10-byte block) and adds it at the tail of
/// `destination`. Nothing when either list is null.
pub fn fn_005ab7f0(e: &mut Engine, source: u32, destination: u32, _unused_0: u32) {
    if destination == 0 || source == 0 {
        return;
    }
    let mut node = source;
    while node != 0 && !list_is_empty(e, node) {
        let block = new_block(e, 0x10);
        let item = list_item(e, node);
        let copy = if block != 0 {
            fn_005ab8b0(e, Ptr::new(block), Ptr::new(item)).addr()
        } else {
            0
        };
        list_add_tail(e, destination, copy);
        node = list_next(e, node);
    }
}

// Translated from 005ab8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy constructor of `SCRIPT_REFERENCED_OBJECT` (0x10 bytes): constructs
/// the string at +0, copies the string of `source` into it and copies the
/// words at +8 and +0xC.
pub fn fn_005ab8b0(
    e: &mut Engine,
    this: Ptr<ScriptReferencedObject>,
    source: Ptr<ScriptReferencedObject>,
) -> Ptr<ScriptReferencedObject> {
    e.call(STRING_CONSTRUCT, &args![this]);
    e.call(STRING_COPY, &args![this, source]);
    let form = e.mem.u32(source.addr() + 8);
    e.mem.set_u32(this.addr() + 8, form);
    let variable_id = e.get(source, ScriptReferencedObject::uiVariableID);
    e.set(this, ScriptReferencedObject::uiVariableID, variable_id);
    this
}

// Translated from 005ab930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies a list of `ScriptVariable` pointers: for each item of `source`
/// (while the node is not an empty one), makes a copy (`005e2910` into a new
/// 0x20-byte block) and adds it at the tail of `destination`. Nothing when
/// either list is null.
pub fn fn_005ab930(e: &mut Engine, source: u32, destination: u32, _unused_0: u32) {
    if destination == 0 || source == 0 {
        return;
    }
    let mut node = source;
    while node != 0 && !list_is_empty(e, node) {
        let block = new_block(e, 0x20);
        let item = list_item(e, node);
        let copy = if block != 0 {
            e.call(SCRIPT_VARIABLE_COPY_CONSTRUCT, &args![block, item])
                .u32()
        } else {
            0
        };
        list_add_tail(e, destination, copy);
        node = list_next(e, node);
    }
}

// Translated from 005ab9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::Load` (Xbox PDB): reads a script record from `file`. False
/// when the record is not a script (form type 0x11). Otherwise loads the
/// form (`TESForm::LoadForm`) and then each chunk: `EDID` sets the editor
/// id (virtual `+0x134`), `OBND` runs virtual `+0xE0`, `SCHR` reads the
/// header (swapped on a big-endian file), `SCDA` reads the compiled bytes
/// into a new zeroed block, `SLSD` adds a new `ScriptVariable` loaded from
/// the file, `SCVR` sets the name of that variable, `SCRO` and `SCRV` add a
/// referenced object holding the chunk's value at +8 or +0xC, and `RNAM` is
/// read and dropped. Logs when no compiled data was read and answers true.
pub fn script_load(e: &mut Engine, this: Ptr<Script>, file: Ptr) -> bool {
    let mut current_variable = 0u32;
    if e.call(FILE_RECORD_TYPE, &args![file]).u32() != FORM_TYPE_SCRIPT {
        return false;
    }
    e.call(FORM_LOAD, &args![this, file]);
    loop {
        let chunk = e.call(FILE_CHUNK_TYPE, &args![file]).u32();
        if chunk == 0 {
            break;
        }
        match chunk {
            CHUNK_EDID => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                e.with_stack(size.max(4), |e, text| {
                    e.call(FILE_GET_CHUNK_DATA, &args![file, text, 0x200u32]);
                    e.vcall(this.addr(), SLOT_SET_EDITOR_ID, &args![text]);
                });
            }
            CHUNK_OBND => {
                e.vcall(this.addr(), SLOT_LOAD_BOUNDS, &args![file]);
            }
            CHUNK_SCHR => {
                let header = header_of(this);
                e.call(FILE_GET_CHUNK_DATA, &args![file, header, 0u32]);
                if e.call(FILE_IS_BIG_ENDIAN, &args![file]).bool() {
                    e.call(HEADER_SWAP_BYTES, &args![header]);
                }
            }
            CHUNK_SLSD => {
                let block = new_block(e, 0x20);
                let variable = if block != 0 {
                    e.call(SCRIPT_VARIABLE_CONSTRUCT, &args![block]).u32()
                } else {
                    0
                };
                current_variable = variable;
                e.call(SCRIPT_VARIABLE_LOAD, &args![variable, file]);
                let list = e.call(SCRIPT_VARIABLE_LIST, &args![this]).u32();
                list_add_tail(e, list, variable);
            }
            CHUNK_SCVR => {
                if current_variable != 0 {
                    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                    e.with_stack(size.max(4), |e, text| {
                        e.call(FILE_GET_CHUNK_DATA, &args![file, text, 0x200u32]);
                        e.call(STRING_SET, &args![current_variable + 0x18, text, 0u32]);
                    });
                }
            }
            CHUNK_SCDA => {
                let size = e.call(FILE_CHUNK_SIZE, &args![file]).u32();
                let data = new_zeroed(e, size);
                e.set(this, Script::m_data, Ptr::new(data));
                e.call(MEMSET, &args![data, 0u32, size]);
                e.call(FILE_GET_CHUNK_DATA, &args![file, data, 0u32]);
            }
            CHUNK_SCRO | CHUNK_SCRV => {
                let block = new_block(e, 0x10);
                let object = if block != 0 {
                    e.call(REFERENCED_OBJECT_CONSTRUCT, &args![block]).u32()
                } else {
                    0
                };
                let value = e.with_stack(4, |e, slot| {
                    e.call(FILE_GET_CHUNK_VALUE, &args![file, slot]);
                    e.mem.u32(slot.addr())
                });
                if chunk == CHUNK_SCRO {
                    e.mem.set_u32(object + 8, value);
                } else {
                    e.mem.set_u32(object + 0xc, value);
                }
                let list = e.call(SCRIPT_REF_OBJECT_LIST, &args![this]).u32();
                list_add_tail(e, list, object);
            }
            CHUNK_RNAM => {
                e.with_stack(4, |e, slot| {
                    e.call(FILE_GET_CHUNK_VALUE, &args![file, slot]);
                });
            }
            _ => {}
        }
        if !e.call(FILE_NEXT_CHUNK, &args![file]).bool() {
            break;
        }
    }
    if e.get(this, Script::m_data).is_null() {
        let file_name = e.call(FILE_NAME_OF, &args![file]).u32();
        let name = script_type_name(e, this);
        script_log(e, &args![FORMAT_NOT_COMPILED, name, file_name]);
    }
    true
}

// Translated from 005abd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes its argument to `TESForm::Save` (`00483d20`) and answers its byte: the exe ends on
/// that call and keeps its `AL` (the virtual slot's `bool`).
pub fn fn_005abd70(e: &mut Engine, this: Ptr<Script>, buffer: u32) -> bool {
    e.call(FORM_SAVE, &args![this, buffer]).bool()
}

/// Swaps the bytes of the script header when the save is big-endian
/// (`00401500`, then `00414220(header)`).
fn swap_header_if_needed(e: &mut Engine, this: Ptr<Script>) {
    if e.call(ENDIAN_SWAP_ENABLED, &args![]).bool() {
        e.call(HEADER_SWAP_BYTES, &args![header_of(this)]);
    }
}

// Translated from 005abd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the script record: `TESForm::StartForm`, the empty method
/// `00483710`, then the header chunk (`SCHR`, 0x14 bytes, swapped to
/// big-endian before and back after when the swap is enabled) and
/// `TESForm::CloseForm`.
pub fn fn_005abd90(e: &mut Engine, this: Ptr<Script>) {
    e.call(FORM_START, &args![this]);
    e.call(EMPTY_METHOD, &args![this]);
    swap_header_if_needed(e, this);
    e.call(FORM_ADD_CHUNK, &args![CHUNK_SCHR, header_of(this), 0x14u32]);
    swap_header_if_needed(e, this);
    e.call(FORM_CLOSE, &args![this]);
}

// Translated from 005abe00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SaveResultScript` (Xbox PDB): writes the header chunk (`SCHR`,
/// 0x14 bytes) of the script, swapped to big-endian before and back after
/// when the swap is enabled.
pub fn script_save_result_script(e: &mut Engine, this: Ptr<Script>) {
    swap_header_if_needed(e, this);
    e.call(FORM_ADD_CHUNK, &args![CHUNK_SCHR, header_of(this), 0x14u32]);
    swap_header_if_needed(e, this);
}

// Translated from 005abe50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetText` (Xbox PDB): frees the text (when set) and, for a
/// non-null `text`, keeps a zero-terminated copy of it; null clears it.
pub fn script_set_text(e: &mut Engine, this: Ptr<Script>, text: u32) {
    let old = e.get(this, Script::m_text);
    if !old.is_null() {
        delete_block(e, old.addr());
    }
    if text == 0 {
        e.set(this, Script::m_text, Ptr::NULL);
    } else {
        let length = e.call(STRLEN, &args![text]).u32();
        let block = new_zeroed(e, length + 1);
        e.set(this, Script::m_text, Ptr::new(block));
        e.call(MEMCPY, &args![block, text, length]);
    }
}

// Translated from 005abed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetVariableName` (Xbox PDB): the name string of the variable
/// `id` of the script's variable list; for an unknown id logs "variable
/// not found" with the script's name and answers null.
pub fn script_get_variable_name(e: &mut Engine, this: Ptr<Script>, id: u32) -> u32 {
    let mut node = e.call(SCRIPT_VARIABLE_LIST, &args![this]).u32();
    while node != 0 && list_item(e, node) != 0 {
        let variable = list_item(e, node);
        node = list_next(e, node);
        // ScriptVariable::data.uiID (+0) and cName (+0x18).
        if e.mem.u32(variable) == id {
            return e.call(STRING_DATA, &args![variable + 0x18]).u32();
        }
    }
    let name = script_type_name(e, this);
    script_log(e, &args![FORMAT_LOCAL_VARIABLE_NOT_FOUND, id, name]);
    0
}

// Translated from 005abf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes the `ScriptLocals` of a script instance: inside a scope guard,
/// constructs a 0x14-byte `ScriptLocals`, gives it the copy of the script's
/// variables that `fn_005ac020` builds, and its master script.
pub fn fn_005abf60(e: &mut Engine, this: Ptr<Script>) -> Ptr<ScriptLocals> {
    with_scope_guard(e, BUILD_LOCALS_SCOPE_LINE, |e| {
        let block = new_block(e, 0x14);
        let locals = if block != 0 {
            Ptr::<ScriptLocals>::new(e.call(SCRIPT_LOCALS_CONSTRUCT, &args![block]).u32())
        } else {
            Ptr::NULL
        };
        let variables = fn_005ac020(e, this);
        e.set(locals, ScriptLocals::m_pLocalList, Ptr::new(variables));
        e.set(locals, ScriptLocals::m_pMasterScript, this.cast());
        locals
    })
}

// Translated from 005ac020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Builds a new variable list that holds a copy (`fn_005ac150`, 0x18-byte
/// `SCRIPT_LOCAL`) of the data of each variable of the script's variable
/// list, until a node with no variable. Runs inside a scope guard.
pub fn fn_005ac020(e: &mut Engine, this: Ptr<Script>) -> u32 {
    with_scope_guard(e, COPY_VARIABLES_SCOPE_LINE, |e| {
        let list = new_list(e);
        let mut node = e.call(SCRIPT_VARIABLE_LIST, &args![this]).u32();
        while node != 0 && list_item(e, node) != 0 {
            let variable = list_item(e, node);
            node = list_next(e, node);
            let block = new_block(e, 0x18);
            let copy = if block != 0 {
                fn_005ac150(e, Ptr::new(block), Ptr::new(variable)).addr()
            } else {
                0
            };
            list_add_tail(e, list, copy);
        }
        list
    })
}

/// The FLD/FSTP double stores in PC 005ac150 and 005a9290 quiet signaling
/// NaNs while retaining their sign and payload (masked x87 exceptions).
fn x87_local_value(value: f64) -> f64 {
    let bits = value.to_bits();
    let nan =
        bits & 0x7ff0_0000_0000_0000 == 0x7ff0_0000_0000_0000 && bits & 0x000f_ffff_ffff_ffff != 0;
    f64::from_bits(if nan {
        bits | 0x0008_0000_0000_0000
    } else {
        bits
    })
}

// Translated from 005ac150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copy constructor of `SCRIPT_LOCAL` (0x18 bytes): copies the integer
/// flag, the id and the value.
pub fn fn_005ac150(
    e: &mut Engine,
    this: Ptr<ScriptLocal>,
    source: Ptr<ScriptLocal>,
) -> Ptr<ScriptLocal> {
    let is_integer = e.mem.u8(source.addr().wrapping_add(0x10));
    e.mem.set_u8(this.addr().wrapping_add(0x10), is_integer);
    let id = e.get(source, ScriptLocal::uiID);
    e.set(this, ScriptLocal::uiID, id);
    let value = x87_local_value(e.get(source, ScriptLocal::fValue));
    e.set(this, ScriptLocal::fValue, value);
    this
}

// Translated from 005ac190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::InitActionList` (Xbox PDB): for a reference with an extra data
/// list that has `ScriptLocals` with no action list yet, asks the script run
/// manager to build the action list (`InitActionList(master, reference,
/// locals)`).
pub fn script_init_action_list(e: &mut Engine, reference: u32, extra_list: Ptr) {
    if extra_list.is_null() {
        return;
    }
    if e.call(EXTRA_LIST_SCRIPT_LOCALS, &args![extra_list]).u32() == 0 {
        return;
    }
    let locals =
        Ptr::<ScriptLocals>::new(e.call(EXTRA_LIST_SCRIPT_LOCALS, &args![extra_list]).u32());
    if e.get(locals, ScriptLocals::m_pActionList).is_null() {
        let master = e.get(locals, ScriptLocals::m_pMasterScript);
        let manager = e.call(SCRIPT_RUN_MANAGER_INSTANCE, &args![]).u32();
        e.call(
            SCRIPT_RUN_MANAGER_INIT_ACTION_LIST,
            &args![manager, master, reference, locals],
        );
    }
}

/// Calls `ScriptRunManager::Run` (`005e2590`) on the manager with the
/// script and the eight words the callers of `Script::Run` pass.
fn run_manager(e: &mut Engine, words: &[u32]) -> Ret {
    let manager = e.call(SCRIPT_RUN_MANAGER_INSTANCE, &args![]).u32();
    let mut all = vec![manager];
    all.extend_from_slice(words);
    e.call(SCRIPT_RUN_MANAGER_RUN, &all)
}

// Translated from 005ac1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::Run` (Xbox PDB): runs the script through `ScriptRunManager::Run`
/// and answers its result. Without `force` (the last argument) a script
/// whose compiled data is 4 bytes or fewer does not run (false). A quest
/// script with a running delay (`fQuestScriptDelay` above 0) first loses
/// the elapsed time (`0084d030`) from it, adds that time to the get-seconds
/// buffer, and does not run (false) while the delay is still above 0. After a
/// run, a quest script sets the delay again: the owner quest's
/// (`00792760`) when it is above 0, or the setting's, added to what is left;
/// the get-seconds buffer is zeroed, and `fn_005ae2e0` forgets the cached
/// references.
pub fn script_run(
    e: &mut Engine,
    this: Ptr<Script>,
    first: u32,
    second: u32,
    third: u32,
    force: u8,
) -> bool {
    if force == 0 {
        let header = e.call(SCRIPT_HEADER_OF, &args![this]).u32();
        if e.mem.u32(header + 8) <= 4 {
            return false;
        }
    }
    let header = e.call(SCRIPT_HEADER_OF, &args![this]).u32();
    // SCRIPT_HEADER::bIsQuestScript (+0x10).
    if e.mem.u8(header + 0x10) != 0 && e.get(this, Script::fQuestScriptDelay) > 0.0 {
        let elapsed = e.call(ELAPSED_TIME, &args![ELAPSED_TIME_OBJECT]).f64();
        let delay = e.get(this, Script::fQuestScriptDelay) as f64;
        e.set(this, Script::fQuestScriptDelay, (delay - elapsed) as f32);
        let elapsed = e.call(ELAPSED_TIME, &args![ELAPSED_TIME_OBJECT]).f64();
        let buffer = e.get(this, Script::fQuestScriptGetSecondsBuffer) as f64;
        e.set(
            this,
            Script::fQuestScriptGetSecondsBuffer,
            (elapsed + buffer) as f32,
        );
        if e.get(this, Script::fQuestScriptDelay) > 0.0 {
            return false;
        }
    }
    let result = run_manager(
        e,
        &args![this, first, second, third, force as u32, 0u32, 0u32, 0.0f32],
    )
    .bool();
    let header = e.call(SCRIPT_HEADER_OF, &args![this]).u32();
    if e.mem.u8(header + 0x10) != 0 {
        let setting = e
            .call(QUEST_DELAY_ACCESSOR, &args![QUEST_DELAY_SETTING])
            .u32();
        if e.mem.f32(setting) > 0.0 {
            let owner = e.get(this, Script::pOwnerQuest);
            if !owner.is_null() && e.call(QUEST_DELAY_OF, &args![owner]).f64() > 0.0 {
                let owner = e.get(this, Script::pOwnerQuest);
                let quest_delay = e.call(QUEST_DELAY_OF, &args![owner]).f64();
                let delay = e.get(this, Script::fQuestScriptDelay) as f64;
                e.set(
                    this,
                    Script::fQuestScriptDelay,
                    (quest_delay + delay) as f32,
                );
            } else {
                let setting = e
                    .call(QUEST_DELAY_ACCESSOR, &args![QUEST_DELAY_SETTING])
                    .u32();
                let delay = e.get(this, Script::fQuestScriptDelay) as f64;
                let value = e.mem.f32(setting) as f64;
                e.set(this, Script::fQuestScriptDelay, (delay + value) as f32);
            }
            e.set(this, Script::fQuestScriptGetSecondsBuffer, 0.0);
        }
    }
    fn_005ae2e0(e, this);
    result
}

// Translated from 005ac340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::RunScriptEffectStart` (Xbox PDB): `ScriptRunManager::Run` with
/// the effect-start flag (the sixth word) set.
pub fn script_run_script_effect_start(e: &mut Engine, this: Ptr<Script>, first: u32, second: u32) {
    run_manager(
        e,
        &args![this, first, second, 0u32, 0u32, 1u32, 0u32, 0.0f32],
    );
}

// Translated from 005ac380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::RunScriptEffectFinish` (Xbox PDB): `ScriptRunManager::Run` with
/// the effect-finish flag (the seventh word) set.
pub fn script_run_script_effect_finish(e: &mut Engine, this: Ptr<Script>, first: u32, second: u32) {
    run_manager(
        e,
        &args![this, first, second, 0u32, 0u32, 0u32, 1u32, 0.0f32],
    );
}

// Translated from 005ac3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::RunScriptEffectUpdate` (Xbox PDB): `ScriptRunManager::Run` with
/// the elapsed time `delta` as the last word.
pub fn script_run_script_effect_update(
    e: &mut Engine,
    this: Ptr<Script>,
    first: u32,
    second: u32,
    delta: f32,
) {
    run_manager(
        e,
        &args![this, first, second, 0u32, 0u32, 0u32, 0u32, delta],
    );
}

// Translated from 005ac400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::CompileAndRun` (Xbox PDB): inside a scope guard, compiles the
/// script with `ScriptCompiler::CompilePartialScript` (`compiler` the
/// object, `mode` and `reference` its arguments) and, when that succeeds
/// and there is compiled data, runs it with `reference` and the script
/// variables of `reference` (none for a null one), forced. With `mode` 1 the
/// thread's system-output byte is set during the run.
pub fn script_compile_and_run(
    e: &mut Engine,
    this: Ptr<Script>,
    compiler: u32,
    mode: u32,
    reference: u32,
) {
    with_scope_guard(e, COMPILE_AND_RUN_SCOPE_LINE, |e| {
        let compiled = e
            .call(
                COMPILE_PARTIAL_SCRIPT,
                &args![compiler, this, reference, mode, 0u32],
            )
            .bool();
        if compiled && e.get(header_of(this), ScriptHeader::dataSize) != 0 {
            let tls = tls_block(e);
            if mode == 1 {
                e.mem.set_u8(tls + TLS_SYSTEM_OUTPUT, 1);
            }
            let mut variables = 0u32;
            if reference != 0 {
                variables = e.call(REFERENCE_SCRIPT_VARIABLES, &args![reference]).u32();
            }
            script_run(e, this, reference, variables, 0, 1);
            e.mem.set_u8(tls + TLS_SYSTEM_OUTPUT, 0);
        }
    });
}

// Translated from 005ac4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetReferencedObject` (Xbox PDB): the `index`th (1-based)
/// referenced object of the script, null for 0 or past the end. The last
/// answer is cached in the TLS block (script, index, locals, object). For a
/// new lookup, an object with a variable id and a `locals` has its form
/// reloaded from the numeric id stored in that variable
/// (`ScriptLocals::GetVariable`, `GetNumericIDFromDouble`).
pub fn script_get_referenced_object(
    e: &mut Engine,
    this: Ptr<Script>,
    index: u32,
    locals: u32,
) -> u32 {
    if index == 0 {
        return 0;
    }
    if e.get(header_of(this), ScriptHeader::refObjectCount) < index {
        return 0;
    }
    let tls = tls_block(e);
    if e.mem.u32(tls + TLS_LAST_REF_SEARCH_SCRIPT) == this.addr()
        && e.mem.u32(tls + TLS_LAST_REF_INDEX) == index
        && e.mem.u32(tls + TLS_LAST_REF_LOCALS) == locals
    {
        return e.mem.u32(tls + TLS_LAST_REF_OBJECT);
    }
    let mut position = 1u32;
    let mut node = e.call(SCRIPT_REF_OBJECT_LIST, &args![this]).u32();
    while node != 0 {
        if list_is_empty(e, node) || position >= index {
            break;
        }
        position += 1;
        node = list_next(e, node);
    }
    if node == 0 {
        return 0;
    }
    let object = list_item(e, node);
    let variable_id = e.mem.u32(object + 0xc);
    if variable_id != 0 && locals != 0 {
        let value = script_locals_get_variable(e, Ptr::new(locals), variable_id, this.cast());
        let id = e.with_stack(12, |e, buffer| {
            e.mem.set_f64(buffer.addr(), value);
            e.mem.set_u32(buffer.addr() + 8, 0);
            e.call(
                GET_NUMERIC_ID_FROM_DOUBLE,
                &args![buffer.addr() + 8, buffer],
            );
            e.mem.u32(buffer.addr() + 8)
        });
        if id != 0 {
            let form = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
            e.mem.set_u32(object + 8, form);
        }
    }
    e.mem.set_u32(tls + TLS_LAST_REF_SEARCH_SCRIPT, this.addr());
    e.mem.set_u32(tls + TLS_LAST_REF_LOCALS, locals);
    e.mem.set_u32(tls + TLS_LAST_REF_INDEX, index);
    e.mem.set_u32(tls + TLS_LAST_REF_OBJECT, object);
    object
}

// Translated from 005ac6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::FindVariable` (Xbox PDB): looks the variable called `name` up in
/// the script's variable list (`00404dc0` answering zero is a match). Found:
/// stores its id through `out` and answers `s` (0x73) for an integer
/// variable or `f` (0x66) otherwise. Not found: stores 0 and answers 0.
pub fn script_find_variable(e: &mut Engine, this: Ptr<Script>, name: u32, out: Ptr) -> u8 {
    let mut node = e.call(SCRIPT_VARIABLE_LIST, &args![this]).u32();
    while node != 0 && list_item(e, node) != 0 {
        let variable = list_item(e, node);
        node = list_next(e, node);
        let variable_name = e.call(STRING_DATA, &args![variable + 0x18]).u32();
        if e.call(STRING_COMPARE, &args![variable_name, name]).u32() == 0 {
            let id = e.mem.u32(variable);
            e.mem.set_u32(out.addr(), id);
            return if e.get(Ptr::<ScriptLocal>::new(variable), ScriptLocal::bIsInteger) {
                b's'
            } else {
                b'f'
            };
        }
    }
    e.mem.set_u32(out.addr(), 0);
    0
}

// Translated from 005ac730 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetProcessScripts` (Xbox PDB): sets the byte that says whether
/// scripts are processed.
pub fn script_set_process_scripts(e: &mut Engine, value: u8) {
    e.set_global::<u8>(PROCESS_SCRIPTS_BYTE, value);
}

// Translated from 005ac740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::GetProcessScripts` (Xbox PDB): the byte that says whether
/// scripts are processed.
pub fn script_get_process_scripts(e: &mut Engine) -> u8 {
    e.global::<u8>(PROCESS_SCRIPTS_BYTE)
}

// Translated from 005ac750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::SetActionFlag` (Xbox PDB): for a non-null `extra_list` that has
/// `ScriptLocals`, sets `flags` on the action of `form` there
/// (`ScriptLocals::SetActionFlag`) and answers its result; otherwise false.
pub fn script_set_action_flag(e: &mut Engine, form: u32, extra_list: Ptr, flags: u32) -> bool {
    let mut result = false;
    if !extra_list.is_null() && e.call(EXTRA_LIST_SCRIPT_LOCALS, &args![extra_list]).u32() != 0 {
        let locals =
            Ptr::<ScriptLocals>::new(e.call(EXTRA_LIST_SCRIPT_LOCALS, &args![extra_list]).u32());
        result = script_locals_set_action_flag(e, locals, form, flags);
    }
    result
}

/// Reads the 16-bit value at the parse offset and moves the offset past it.
fn take_i16(e: &mut Engine, code: u32, offset: u32) -> i16 {
    let at = e.mem.u32(offset);
    let value = e.mem.i16(code.wrapping_add(at));
    e.mem.set_u32(offset, at.wrapping_add(2));
    value
}

/// Reads the byte at the parse offset and moves the offset past it.
fn take_u8(e: &mut Engine, code: u32, offset: u32) -> u8 {
    let at = e.mem.u32(offset);
    let value = e.mem.u8(code.wrapping_add(at));
    e.mem.set_u32(offset, at.wrapping_add(1));
    value
}

/// Whether a form-type byte is one of the 0x3A..=0x40 and 0x69 types that
/// can carry script variables (`ParseParameters` and `fn_005ac7a0` accept
/// them as the target of a call).
fn is_scripted_reference_type(form_type: u32) -> bool {
    form_type >= 0x3a && (form_type <= 0x40 || form_type == 0x69)
}

// Translated from 005ac7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads one value of a script's compiled data at `*offset` into the double
/// at `out` (`cdecl`). The first byte is the kind of operand: `r`, `G` and
/// `Z` name a referenced object (a 16-bit index into the script's referenced
/// objects, see `Script::GetReferencedObject`): `G` stores the object's
/// global value (`00526ac0`), `Z` stores its form id as a numeric id
/// (`Script::PutNumericIDInDouble`), and `r` is followed by a call marker
/// `X` or by a variable: it redirects `locals` to the locals of the
/// referenced object (a quest, a reference, or an inventory item of one). `n`
/// is an immediate integer (4 bytes) and `z` an immediate double (8). An
/// `X` marker is a function call: a 16-bit function id (and 16 bits that are
/// skipped) looked up with `ScriptCompiler::GetFunctionDef`; with `parse`
/// set only its parameters are parsed (`ParseParameters`) and the answer is
/// false; otherwise its handler (the +0x18 word of the definition) runs
/// with the eight words of the script command signature. Any other
/// marker (`f`, `l` or `s`) is a 16-bit variable id read from `locals`
/// (`ScriptLocals::GetVariable`). False when the data cannot be read.
#[allow(clippy::too_many_arguments)]
pub fn fn_005ac7a0(
    e: &mut Engine,
    out: u32,
    code: u32,
    offset: u32,
    reference: u32,
    container: u32,
    script: u32,
    locals: u32,
    parse: u8,
) -> bool {
    let mut locals_to_use = locals;
    if locals_to_use == 0 {
        return false;
    }
    let mut object = 0u32;
    let mut parse = parse;
    if parse != 0 && form_flag_bit_3(e, script) {
        parse = 0;
    }
    let mut marker = take_u8(e, code, offset);
    match marker {
        b'G' | b'Z' | b'r' => {
            let index = take_i16(e, code, offset);
            object = script_get_referenced_object(e, Ptr::new(script), index as i32 as u32, locals);
            if object == 0 {
                return false;
            }
            let form = e.mem.u32(object + 8);
            if form == 0 && parse == 0 {
                if e.mem.u32(object + 0xc) != 0 {
                    e.mem.set_f64(out, 0.0);
                    return true;
                }
                return false;
            }
            if marker == b'G' {
                let value = e.call(GLOBAL_VALUE_OF, &args![form]).f64();
                e.mem.set_f64(out, value);
                return true;
            }
            if marker == b'Z' {
                let id = form_id_of(e, form);
                e.with_stack(4, |e, slot| {
                    e.mem.set_u32(slot.addr(), id);
                    e.call(PUT_NUMERIC_ID_IN_DOUBLE, &args![slot, out]);
                });
                return true;
            }
            marker = take_u8(e, code, offset);
            if marker != b'X' {
                let mut quest = 0u32;
                if form != 0 && form_type(e, form) == 0x47 {
                    quest = form;
                }
                if quest != 0 {
                    locals_to_use = e.call(QUEST_SCRIPT_LOCALS, &args![quest]).u32();
                } else {
                    let mut holder = 0u32;
                    if form != 0 && is_scripted_reference_type(form_type(e, form)) {
                        holder = form;
                    }
                    if holder != 0 {
                        let list = e.call(SCRIPT_REF_OBJECT_LIST, &args![holder]).u32();
                        if e.call(EXTRA_LIST_REFERENCE_POINTER, &args![list]).u32() != 0 {
                            let list = e.call(SCRIPT_REF_OBJECT_LIST, &args![holder]).u32();
                            let pointer = e.call(EXTRA_LIST_REFERENCE_POINTER, &args![list]).u32();
                            let id = form_id_of(e, holder);
                            let key = e.call(INVENTORY_KEY_OF, &args![holder]).u32();
                            let item = e.call(INVENTORY_ITEM_OF, &args![pointer, key, id]).u32();
                            if item != 0 {
                                locals_to_use = e.call(ITEM_SCRIPT_LOCALS, &args![item]).u32();
                            }
                            if item != 0 {
                                e.call(RELEASE_ITEM, &args![item, 1u32]);
                            }
                        } else {
                            locals_to_use =
                                e.call(REFERENCE_SCRIPT_VARIABLES, &args![holder]).u32();
                        }
                    }
                }
                if locals_to_use == 0 {
                    return false;
                }
            }
        }
        b'n' => {
            let at = e.mem.u32(offset);
            let value = e.mem.i32(code.wrapping_add(at));
            e.mem.set_f64(out, value as f64);
            e.mem.set_u32(offset, at.wrapping_add(4));
            return true;
        }
        b'z' => {
            let at = e.mem.u32(offset);
            e.call(MEMCPY, &args![out, code.wrapping_add(at), 8u32]);
            e.mem.set_u32(offset, at.wrapping_add(8));
            return true;
        }
        _ => {}
    }
    if marker == b'X' {
        let function_id = take_i16(e, code, offset) as i32 as u32;
        let at = e.mem.u32(offset);
        e.mem.set_u32(offset, at.wrapping_add(2));
        let definition = e.call(GET_FUNCTION_DEFINITION, &args![function_id]).u32();
        if definition == 0 {
            return false;
        }
        let mut target = reference;
        if object != 0 && e.mem.u32(object + 8) != 0 {
            target = 0;
            let form = e.mem.u32(object + 8);
            if form != 0 && is_scripted_reference_type(form_type(e, form)) {
                target = form;
            }
        }
        // The function definition: needs-reference byte (+0x10), parameters
        // (+0x14), handler (+0x18).
        let parameters = e.mem.u32(definition + 0x14);
        if parse != 0 {
            if parameters != 0 {
                script_parse_parameters(
                    e,
                    parameters,
                    code,
                    offset,
                    target,
                    container,
                    script,
                    locals_to_use,
                    &[],
                );
            }
            return false;
        }
        let needs_reference = e.mem.u8(definition + 0x10) != 0;
        if !needs_reference || target != 0 {
            let handler = e.mem.u32(definition + 0x18);
            if handler != 0
                && e.call(
                    handler,
                    &args![
                        parameters,
                        code,
                        target,
                        container,
                        script,
                        locals_to_use,
                        out,
                        offset
                    ],
                )
                .bool()
            {
                return true;
            }
            return false;
        }
        false
    } else {
        let id = take_i16(e, code, offset);
        if matches!(marker, b'f' | b'l' | b's') {
            let value = script_locals_get_variable(
                e,
                Ptr::new(locals_to_use),
                id as i32 as u32,
                Ptr::new(script),
            );
            e.mem.set_f64(out, value);
            true
        } else {
            false
        }
    }
}

// Translated from 005acc70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::PutNumericIDInDouble` (Xbox PDB): copies the 4 bytes at `id` to
/// the start of the double at `value` (`cdecl`).
pub fn script_put_numeric_id_in_double(e: &mut Engine, id: u32, value: u32) {
    e.call(MEMCPY, &args![value, id, 4u32]);
}

// Translated from 005acc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The inverse of `Script::PutNumericIDInDouble`: copies the first 4 bytes
/// of the double at `value` to `id` (`cdecl`).
pub fn fn_005acc90(e: &mut Engine, id: u32, value: u32) {
    e.call(MEMCPY, &args![id, value, 4u32]);
}

/// Stores a form through the next output pointer and says whether it is
/// not null (the common ending of the referenced-object parameters).
fn store_form(e: &mut Engine, outputs: &[u32], used: &mut usize, form: u32) -> bool {
    let target = next_output(outputs, used);
    e.mem.set_u32(target, form);
    e.mem.u32(target) != 0
}

/// The next output pointer of a `ParseParameters` call.
fn next_output(outputs: &[u32], used: &mut usize) -> u32 {
    let target = *outputs
        .get(*used)
        .unwrap_or_else(|| panic!("ParseParameters: output pointer {} missing", *used));
    *used += 1;
    target
}

/// `__RTDynamicCast(form, 0, TESForm, target, 0)`.
fn cast_form(e: &mut Engine, form: u32, target: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, RTTI_SOURCE_FORM, target, 0u32],
    )
    .u32()
}

/// The form types that a referenced-object parameter type accepts when it
/// is a plain test of the form type (the form is stored through the next
/// output pointer): `Some((low, high))` for an inclusive range.
fn exact_reference_type(parameter: u32) -> Option<(u32, u32)> {
    let exact = match parameter {
        0x9 => 0x39,
        0xc => 0xd,
        0xd => 0x45,
        0xe => 0x47,
        0xf => 0xc,
        0x10 => 0x7,
        0x11 => 0x8,
        0x13 => 0x6,
        0x1b => 0x41,
        0x1d => 0x49,
        0x1e => 0x4a,
        0x21 => 0x35,
        0x22 => 0x2a,
        0x24 => 0x4f,
        0x25 => 0x55,
        0x26 => 0x5,
        0x27 => 0x56,
        0x28 => 0x31,
        0x2a => 0x54,
        0x2b => 0x53,
        0x2f => 0x61,
        0x31 => 0x62,
        0x36 => 0x66,
        0x3a => 0x2d,
        0x3b => 0x2c,
        0x3c => 0x34,
        0x3e => 0x68,
        0x3f => 0x6d,
        0x40 => 0x6c,
        0x41 => 0x71,
        0x42 => 0x74,
        0x43 => 0x73,
        0x44 => 0x75,
        0x45 => 0x37,
        0x6 => return Some((0x3b, 0x3c)),
        _ => return None,
    };
    Some((exact, exact))
}

/// One referenced-object parameter of `ParseParameters`: `parameter` is the
/// type, `form` the (non-null) form of the referenced object. Stores the
/// form through the next output pointer when the type accepts it and says
/// whether it did; `None` for a type the game does not implement.
fn parse_reference_parameter(
    e: &mut Engine,
    parameter: u32,
    form: u32,
    outputs: &[u32],
    used: &mut usize,
) -> Option<bool> {
    if let Some((low, high)) = exact_reference_type(parameter) {
        let kind = form_type(e, form);
        if kind >= low && kind <= high {
            return Some(store_form(e, outputs, used, form));
        }
        return Some(false);
    }
    let accepted = match parameter {
        // A container-capable form (virtual +0xE8 on the form).
        3 => {
            let kind = form_type(e, form);
            if e.call(CONTAINER_CAN_HOLD_TYPE, &args![kind]).bool() {
                let target = next_output(outputs, used);
                e.mem.set_u32(target, 0);
                if form != 0 && e.vcall(form, 0xe8, &args![]).bool() {
                    e.mem.set_u32(target, form);
                }
                e.mem.u32(target) != 0
            } else {
                false
            }
        }
        4 | 0x18 | 0x1a => {
            if is_scripted_reference_type(form_type(e, form)) {
                let target = next_output(outputs, used);
                e.mem.set_u32(target, 0);
                if form != 0 && is_scripted_reference_type(form_type(e, form)) {
                    e.mem.set_u32(target, form);
                }
                e.mem.u32(target) != 0
            } else {
                false
            }
        }
        7 => {
            let target = next_output(outputs, used);
            let cast = cast_form(e, form, RTTI_TARGET_FORM_7);
            e.mem.set_u32(target, cast);
            if cast != 0 {
                true
            } else {
                let mut quest = 0u32;
                if form != 0 && form_type(e, form) == 0x19 {
                    quest = form;
                }
                e.mem.set_u32(target, quest);
                quest != 0 && list_next(e, quest + 0x74) != 0
            }
        }
        0xb | 0x1f => {
            let target = next_output(outputs, used);
            let wanted = if parameter == 0xb {
                RTTI_TARGET_FORM_11
            } else {
                RTTI_TARGET_FORM_31
            };
            let cast = cast_form(e, form, wanted);
            e.mem.set_u32(target, cast);
            cast != 0
        }
        0x14 => {
            let kind = form_type(e, form);
            if kind == 0x27 || form_type(e, form) == 0x55 {
                store_form(e, outputs, used, form)
            } else {
                false
            }
        }
        0x15 => {
            let target = next_output(outputs, used);
            e.mem.set_u32(target, 0);
            if form != 0 && e.vcall(form, 0xe8, &args![]).bool() {
                e.mem.set_u32(target, form);
            }
            e.mem.u32(target) != 0
        }
        0x19 => {
            let target = next_output(outputs, used);
            e.mem.set_u32(target, 0);
            if form != 0 {
                let kind = form_type(e, form);
                if (0x2a..=0x2b).contains(&kind) {
                    e.mem.set_u32(target, form);
                }
            }
            e.mem.u32(target) != 0
        }
        0x23 | 0x38 | 0x39 => {
            let target = next_output(outputs, used);
            e.mem.set_u32(target, 0);
            if form != 0 {
                let kind = form_type(e, form);
                let (first, second) = match parameter {
                    0x23 => (8, 0x2a),
                    0x38 => (0x2a, 0x2d),
                    _ => (0x2b, 0x2c),
                };
                if kind == first || kind == second {
                    e.mem.set_u32(target, form);
                }
            }
            e.mem.u32(target) != 0
        }
        0x32 => {
            let kind = form_type(e, form);
            if e.call(CONTAINER_CAN_HOLD_TYPE, &args![kind]).bool() || form_type(e, form) == 0x55 {
                store_form(e, outputs, used, form)
            } else {
                false
            }
        }
        0x35 => {
            let target = next_output(outputs, used);
            e.mem.set_u32(target, 0);
            if form != 0 && (e.vcall(form, 0xe8, &args![]).bool() || form_type(e, form) == 0x55) {
                e.mem.set_u32(target, form);
            }
            e.mem.u32(target) != 0
        }
        0x3d => {
            if form != 0 {
                store_form(e, outputs, used, form)
            } else {
                false
            }
        }
        _ => return None,
    };
    Some(accepted)
}

// Translated from 005accb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `Script::ParseParameters` (Xbox PDB), `cdecl`, varargs: reads the
/// parameters of a script command from the compiled data at `*offset`
/// (`code`). `parameters` is the command's parameter list (12 bytes per
/// entry, the type at +4); a 16-bit count in the data says how many follow.
/// Each parameter is stored through the next of the output pointers in
/// `outputs` (the stack words after the seventh), but only when bit 3 of
/// the script's form flags is set (`004013e0`; otherwise the data is only
/// skipped). Types the table at `0118cdd5` marks as referenced objects are
/// an `r` marker and a 16-bit index into the script's referenced objects
/// (`GetReferencedObject`) and need the object to have a form; the other
/// types are strings (type 0), integers and floats read as operands with
/// `fn_005ac7a0` (types 1, 0x17 and 2), 16-bit values stored as integers
/// (types 5, 0xA, 0x12, 0x1C, 0x29, 0x33, 0x34, 0x37), a byte (type 8) or a
/// 16-bit value stored as a byte (type 0x20). Any other type, a missing
/// `r` marker or a rejected object answers false (an unimplemented
/// referenced-object type is logged first); true when every parameter was
/// read.
#[allow(clippy::too_many_arguments)]
pub fn script_parse_parameters(
    e: &mut Engine,
    parameters: u32,
    code: u32,
    offset: u32,
    reference: u32,
    container: u32,
    script: u32,
    locals: u32,
    outputs: &[u32],
) -> bool {
    if parameters == 0 {
        return false;
    }
    let mut used = 0usize;
    let count = take_i16(e, code, offset);
    let mut index: i16 = 0;
    while index < count {
        let entry = parameters.wrapping_add((index as i32).wrapping_mul(12) as u32);
        let parameter = e.mem.u32(entry.wrapping_add(4));
        let is_reference = e
            .mem
            .u8(PARAMETER_TYPE_TABLE.wrapping_add(parameter.wrapping_mul(8)))
            != 0;
        if is_reference {
            if take_u8(e, code, offset) != b'r' {
                return false;
            }
            let object_index = take_i16(e, code, offset);
            let object = script_get_referenced_object(
                e,
                Ptr::new(script),
                object_index as i32 as u32,
                locals,
            );
            if !form_flag_bit_3(e, script) || object == 0 || e.mem.u32(object + 8) == 0 {
                return false;
            }
            let form = e.mem.u32(object + 8);
            match parse_reference_parameter(e, parameter, form, outputs, &mut used) {
                Some(true) => {}
                Some(false) => return false,
                None => {
                    script_log(e, &args![FORMAT_PARAMETER_TYPE_UNIMPLEMENTED, parameter]);
                    return false;
                }
            }
        } else {
            match parameter {
                0 => {
                    let length = take_i16(e, code, offset);
                    if form_flag_bit_3(e, script) {
                        let target = next_output(outputs, &mut used);
                        let at = e.mem.u32(offset);
                        e.call(
                            MEMCPY,
                            &args![target, code.wrapping_add(at), length as i32 as u32],
                        );
                        e.mem.set_u8(target.wrapping_add(length as i32 as u32), 0);
                    }
                    let at = e.mem.u32(offset);
                    e.mem.set_u32(offset, at.wrapping_add(length as i32 as u32));
                }
                1 | 0x17 => {
                    let value = e.with_stack(8, |e, slot| {
                        e.mem.set_f64(slot.addr(), 0.0);
                        let ok = fn_005ac7a0(
                            e,
                            slot.addr(),
                            code,
                            offset,
                            reference,
                            container,
                            script,
                            locals,
                            1,
                        );
                        ok.then(|| e.mem.f64(slot.addr()))
                    });
                    let Some(value) = value else {
                        return false;
                    };
                    if form_flag_bit_3(e, script) {
                        let target = next_output(outputs, &mut used);
                        let integer = e.call(FTOL2, &args![value]).u32();
                        e.mem.set_u32(target, integer);
                    }
                }
                2 => {
                    let value = e.with_stack(8, |e, slot| {
                        e.mem.set_f64(slot.addr(), 0.0);
                        let ok = fn_005ac7a0(
                            e,
                            slot.addr(),
                            code,
                            offset,
                            reference,
                            container,
                            script,
                            locals,
                            1,
                        );
                        ok.then(|| e.mem.f64(slot.addr()))
                    });
                    let Some(value) = value else {
                        return false;
                    };
                    if form_flag_bit_3(e, script) {
                        let target = next_output(outputs, &mut used);
                        e.mem.set_f32(target, value as f32);
                    }
                }
                5 | 0xa | 0x12 | 0x1c | 0x29 | 0x33 | 0x34 | 0x37 => {
                    let value = take_i16(e, code, offset);
                    if form_flag_bit_3(e, script) {
                        let target = next_output(outputs, &mut used);
                        e.mem.set_i32(target, value as i32);
                    }
                }
                8 => {
                    if form_flag_bit_3(e, script) {
                        let target = next_output(outputs, &mut used);
                        let at = e.mem.u32(offset);
                        let value = e.mem.u8(code.wrapping_add(at));
                        e.mem.set_u8(target, value);
                    }
                    let at = e.mem.u32(offset);
                    e.mem.set_u32(offset, at.wrapping_add(1));
                }
                0x20 => {
                    let value = take_i16(e, code, offset);
                    if form_flag_bit_3(e, script) {
                        let target = next_output(outputs, &mut used);
                        e.mem.set_u8(target, value as u8);
                    }
                }
                _ => return false,
            }
        }
        index = index.wrapping_add(1);
    }
    true
}

/// The registered form of `Script::ParseParameters`: seven fixed words and
/// then the output pointers, however many the caller pushed.
fn parse_parameters_entry(e: &mut Engine, words: &[u32]) -> Ret {
    assert!(
        words.len() >= 7,
        "ParseParameters needs 7 argument words, got {}",
        words.len()
    );
    let ok = script_parse_parameters(
        e,
        words[0],
        words[1],
        words[2],
        words[3],
        words[4],
        words[5],
        words[6],
        &words[7..],
    );
    Ret {
        eax: ok as u32,
        ..Ret::default()
    }
}

// Translated from 005ae2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forgets this script as the last referenced-object search script (TLS
/// +0x288) and clears the form (+8) of each of its referenced objects that
/// has a variable id (+0xC), so that the next lookup reloads it.
pub fn fn_005ae2e0(e: &mut Engine, this: Ptr<Script>) {
    let tls = tls_block(e);
    if e.mem.u32(tls + TLS_LAST_REF_SEARCH_SCRIPT) == this.addr() {
        e.mem.set_u32(tls + TLS_LAST_REF_SEARCH_SCRIPT, 0);
    }
    let mut node = e.call(SCRIPT_REF_OBJECT_LIST, &args![this]).u32();
    while node != 0 && !list_is_empty(e, node) {
        let object = list_item(e, node);
        if object != 0 && e.mem.u32(object + 0xc) != 0 {
            e.mem.set_u32(object + 8, 0);
        }
        node = list_next(e, node);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x005a8b80,
            script_locals_construct(Ptr<ScriptLocals>) -> Ptr<ScriptLocals>
        ),
        entry!(0x005a8bc0, fn_005a8bc0(Ptr<ScriptLocals>)),
        entry!(0x005a8c00, fn_005a8c00(Ptr<ScriptLocals>)),
        entry!(0x005a8c90, fn_005a8c90(Ptr<ScriptLocals>)),
        entry!(0x005a8d20, fn_005a8d20(Ptr<ScriptLocals>, u32)),
        entry!(
            0x005a8e20,
            script_locals_set_action_flag(Ptr<ScriptLocals>, u32, u32) -> bool
        ),
        entry!(0x005a8ea0, fn_005a8ea0(Ptr<ScriptLocals>)),
        entry!(0x005a8ef0, fn_005a8ef0(Ptr<ScriptLocals>, u32, u32) -> bool),
        entry!(0x005a8f40, fn_005a8f40(Ptr<ScriptLocals>, u32)),
        entry!(0x005a90b0, fn_005a90b0(Ptr<ScriptLocals>, u32, bool) -> Ptr),
        entry!(
            0x005a9140,
            script_locals_get_variable(Ptr<ScriptLocals>, u32, Ptr) -> f64
        ),
        entry!(
            0x005a9290,
            script_locals_set_variable(Ptr<ScriptLocals>, u32, f64)
        ),
        entry!(0x005a9460, fn_005a9460(Ptr) -> u32),
        entry!(0x005a9480, fn_005a9480(Ptr<ScriptLocals>, u32) -> bool),
        entry!(0x005a94f0, fn_005a94f0(Ptr<ScriptLocals>) -> u16),
        entry!(0x005a9670, fn_005a9670(Ptr<ScriptLocals>)),
        entry!(0x005a9950, fn_005a9950(Ptr<ScriptLocals>)),
        entry!(0x005a9d00, fn_005a9d00(Ptr<ScriptLocals>)),
        entry!(0x005a9d60, fn_005a9d60()),
        entry!(0x005a9db0, script_locals_save_game(Ptr<ScriptLocals>, Ptr)),
        entry!(0x005a9f20, script_locals_load_game(Ptr<ScriptLocals>, Ptr)),
        entry!(0x005aa090, fn_005aa090(Ptr<ScriptLocals>, u32)),
        entry!(0x005aa0f0, script_script(Ptr<Script>) -> Ptr<Script>),
        entry!(0x005aa170, fn_005aa170(Ptr<Script>, u32) -> Ptr<Script>),
        entry!(0x005aa1a0, fn_005aa1a0(Ptr<Script>)),
        entry!(0x005aa220, fn_005aa220(Ptr<Script>)),
        entry!(0x005aa2a0, fn_005aa2a0(Ptr<Script>)),
        entry!(0x005aa2e0, fn_005aa2e0(Ptr<Script>)),
        entry!(0x005aa350, fn_005aa350(Ptr<Script>)),
        entry!(
            0x005aa3f0,
            fn_005aa3f0(Ptr<ScriptReferencedObject>, u32) -> Ptr
        ),
        entry!(
            0x005aa420,
            script_copy_result_script(Ptr<Script>, Ptr<Script>)
        ),
        entry!(0x005aa500, script_add_pending_disabled_reference(u32, u8)),
        entry!(0x005aa580, script_add_pending_enabled_reference(u32)),
        entry!(
            0x005aa5d0,
            script_remove_delayed_script_action_reference(u32)
        ),
        entry!(
            0x005aa630,
            script_is_pending_disabled_reference(u32) -> bool
        ),
        entry!(
            0x005aa680,
            script_is_pending_enabled_reference(u32) -> bool
        ),
        entry!(
            0x005aa6d0,
            script_is_pending_fade_disabled_reference(u32) -> bool
        ),
        entry!(
            0x005aa720,
            script_run_delayed_script_actions_on_references()
        ),
        entry!(0x005aa8c0, fn_005aa8c0(Ptr) -> bool),
        entry!(0x005aa910, fn_005aa910(Ptr) -> bool),
        entry!(0x005aa930, script_save_pending_references(Ptr)),
        entry!(0x005aaaf0, script_load_pending_references(Ptr)),
        entry!(0x005aae20, fn_005aae20()),
        entry!(0x005aae70, script_set_compile_data(Ptr<Script>, u32, u32)),
        entry!(0x005aaee0, fn_005aaee0(u32, u32) -> u32),
        entry!(0x005aaf20, script_init(Ptr<Script>, Ptr<ScriptCompileData>)),
        entry!(
            0x005ab040,
            fn_005ab040(Ptr<Script>, Ptr<ScriptCompileData>) -> bool
        ),
        entry!(
            0x005ab240,
            script_compare_result_scripts(Ptr<Script>, Ptr<Script>) -> bool
        ),
        entry!(0x005ab400, script_init_item(Ptr<Script>, Ptr)),
        entry!(0x005ab7d0, fn_005ab7d0(Ptr<Script>)),
        entry!(0x005ab7f0, fn_005ab7f0(u32, u32, u32)),
        entry!(
            0x005ab8b0,
            fn_005ab8b0(
                Ptr<ScriptReferencedObject>,
                Ptr<ScriptReferencedObject>,
            ) -> Ptr<ScriptReferencedObject>
        ),
        entry!(0x005ab930, fn_005ab930(u32, u32, u32)),
        entry!(0x005ab9f0, script_load(Ptr<Script>, Ptr) -> bool),
        entry!(0x005abd70, fn_005abd70(Ptr<Script>, u32) -> bool),
        entry!(0x005abd90, fn_005abd90(Ptr<Script>)),
        entry!(0x005abe00, script_save_result_script(Ptr<Script>)),
        entry!(0x005abe50, script_set_text(Ptr<Script>, u32)),
        entry!(
            0x005abed0,
            script_get_variable_name(Ptr<Script>, u32) -> u32
        ),
        entry!(0x005abf60, fn_005abf60(Ptr<Script>) -> Ptr<ScriptLocals>),
        entry!(0x005ac020, fn_005ac020(Ptr<Script>) -> u32),
        entry!(
            0x005ac150,
            fn_005ac150(Ptr<ScriptLocal>, Ptr<ScriptLocal>) -> Ptr<ScriptLocal>
        ),
        entry!(0x005ac190, script_init_action_list(u32, Ptr)),
        entry!(
            0x005ac1e0,
            script_run(Ptr<Script>, u32, u32, u32, u8) -> bool
        ),
        entry!(
            0x005ac340,
            script_run_script_effect_start(Ptr<Script>, u32, u32)
        ),
        entry!(
            0x005ac380,
            script_run_script_effect_finish(Ptr<Script>, u32, u32)
        ),
        entry!(
            0x005ac3c0,
            script_run_script_effect_update(Ptr<Script>, u32, u32, f32)
        ),
        entry!(
            0x005ac400,
            script_compile_and_run(Ptr<Script>, u32, u32, u32)
        ),
        entry!(
            0x005ac4f0,
            script_get_referenced_object(Ptr<Script>, u32, u32) -> u32
        ),
        entry!(
            0x005ac6a0,
            script_find_variable(Ptr<Script>, u32, Ptr) -> u8
        ),
        entry!(0x005ac730, script_set_process_scripts(u8)),
        entry!(0x005ac740, script_get_process_scripts() -> u8),
        entry!(0x005ac750, script_set_action_flag(u32, Ptr, u32) -> bool),
        entry!(
            0x005ac7a0,
            fn_005ac7a0(u32, u32, u32, u32, u32, u32, u32, u8) -> bool
        ),
        entry!(0x005acc70, script_put_numeric_id_in_double(u32, u32)),
        entry!(0x005acc90, fn_005acc90(u32, u32)),
        (0x005accb0, parse_parameters_entry as AbiFn),
        entry!(0x005ae2e0, fn_005ae2e0(Ptr<Script>)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn locals_constructor_matches_native_guarded_vectors() {
        // PC 005a8b80, unpacked exe SHA-256
        // 19406942e48724d797300c4ea6be9ac69a32f670b8c35db09279a2258422739f.
        // Three nv-call vectors agree: twenty cleared bytes, unchanged guards,
        // original receiver returned. No function doubles in this test.
        for fill in [0xa5, 0xcc, 0xff] {
            let mut e = Engine::new();
            let block = e.mem.alloc(28);
            e.mem.write(block, &[fill; 28]);
            assert_eq!(e.call(0x005a8b80, &[block + 4]).u32(), block + 4);
            assert_eq!(e.mem.bytes(block, 4), vec![fill; 4]);
            assert_eq!(e.mem.bytes(block + 4, 20), vec![0; 20]);
            assert_eq!(e.mem.bytes(block + 24, 4), vec![fill; 4]);
        }
    }

    #[test]
    fn locals_constructor_keeps_post_memset_stores_and_receiver() {
        let mut e = Engine::new();
        let block = e.mem.alloc(20);
        e.register_double(MEMSET, move |e, a| {
            assert_eq!(a, &[block, 0, 20]);
            e.mem.write(block, &[0xa5; 20]);
            0xdeadbeefu32.into_ret()
        });
        assert_eq!(e.call(0x005a8b80, &[block]).u32(), block);
        assert_eq!(e.mem.u32(block), 0);
        assert_eq!(e.mem.bytes(block + 4, 12), vec![0xa5; 12]);
        assert_eq!(e.mem.u32(block + 16), 0);
    }

    /// The pages holding the globals this unit reads (the pending lists and
    /// their critical section, the save/load game pointer, the fade byte and
    /// the zero constant).
    const PAGES: [u32; 4] = [0x0101_2000, 0x011a_d000, 0x011c_a000, 0x011d_e000];

    /// An engine with test doubles for the callees outside this file: the
    /// allocator, the `BSSimpleList` accessors (with the behaviour the
    /// decompiled bodies have), the scope guard, the critical section, the
    /// logs and the form helpers. Doubles that only record are the ones whose
    /// behaviour the unit does not depend on (`00905330`, the logs); their
    /// calls are read from the call log.
    fn script_engine() -> Engine {
        let mut e = Engine::new();
        for page in PAGES {
            e.map(page, 0x1000);
        }
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(LIST_ITEM_SLOT, |_, a| ret(a[0]));
        e.register(LIST_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_IS_EMPTY, |e, a| {
            ret((e.mem.u32(a[0]) == 0 && e.mem.u32(a[0] + 4) == 0) as u32)
        });
        e.register(LIST_POP_FIRST, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next == 0 {
                e.mem.set_u32(a[0], 0);
            } else {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0] + 4, after);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(next + 4, 0);
            }
            Ret::default()
        });
        e.register(LIST_ADD_HEAD, |e, a| {
            let item = e.mem.u32(a[1]);
            if item == 0 {
                return Ret::default();
            }
            if e.mem.u32(a[0]) == 0 {
                e.mem.set_u32(a[0], item);
            } else {
                let node = e.mem.alloc(8);
                let old = e.mem.u32(a[0]);
                let next = e.mem.u32(a[0] + 4);
                e.mem.set_u32(node, old);
                e.mem.set_u32(node + 4, next);
                e.mem.set_u32(a[0] + 4, node);
                e.mem.set_u32(a[0], item);
            }
            Ret::default()
        });
        e.register(LIST_ADD_TAIL, |e, a| {
            let item = e.mem.u32(a[1]);
            if item == 0 {
                return Ret::default();
            }
            let mut last = a[0];
            while e.mem.u32(last + 4) != 0 {
                last = e.mem.u32(last + 4);
            }
            if e.mem.u32(last) == 0 {
                e.mem.set_u32(last, item);
            } else {
                let node = e.mem.alloc(8);
                e.mem.set_u32(node, item);
                e.mem.set_u32(last + 4, node);
            }
            Ret::default()
        });
        // Removal of the item the slot holds (the node is unlinked; the head
        // node takes the place of its successor), as the lists need in the
        // tests that walk them while removing.
        e.register(LIST_REMOVE, |e, a| {
            let wanted = e.mem.u32(a[1]);
            let mut previous = 0u32;
            let mut node = a[0];
            while node != 0 {
                let next = e.mem.u32(node + 4);
                if e.mem.u32(node) == wanted {
                    if previous != 0 {
                        e.mem.set_u32(previous + 4, next);
                    } else if next == 0 {
                        e.mem.set_u32(node, 0);
                    } else {
                        let following = e.mem.u32(next);
                        let after = e.mem.u32(next + 4);
                        e.mem.set_u32(node, following);
                        e.mem.set_u32(node + 4, after);
                    }
                    break;
                }
                previous = node;
                node = next;
            }
            Ret::default()
        });
        e.register(LIST_CONTAINS, |e, a| {
            let wanted = e.mem.u32(a[1]);
            ret(list_items(e, a[0]).contains(&wanted) as u32)
        });
        e.register(LIST_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            ret(a[0])
        });
        e.register(LIST_DESTRUCT, |_, _| Ret::default());
        e.register(LIST_DELETE, |_, _| Ret::default());
        e.register(ACTION_LIST_OF, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(LOCAL_LIST_OF, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(SCRIPT_REF_OBJECT_LIST, |_, a| ret(a[0] + 0x44));
        e.register(SCRIPT_VARIABLE_LIST, |_, a| ret(a[0] + 0x4c));
        e.register(SCOPE_GUARD_OPEN, |_, _| Ret::default());
        e.register(SCOPE_GUARD_CLOSE, |_, _| Ret::default());
        e.register(CRITICAL_SECTION_ENTER, |_, _| Ret::default());
        e.register(CRITICAL_SECTION_LEAVE, |_, _| Ret::default());
        e.register(ERROR_LOG, |_, _| Ret::default());
        e.register(SCRIPT_LOG, |_, _| Ret::default());
        e
    }

    /// The items of the list starting at `head` (an empty head node has no
    /// items).
    fn list_items(e: &Engine, head: u32) -> Vec<u32> {
        let mut items = Vec::new();
        let mut node = head;
        while node != 0 {
            let item = e.mem.u32(node);
            if item != 0 {
                items.push(item);
            }
            node = e.mem.u32(node + 4);
        }
        items
    }

    /// A list of the items: a chain of 8-byte nodes (item, next); an empty
    /// slice gives an empty head node.
    fn make_list(e: &mut Engine, items: &[u32]) -> u32 {
        let head = e.mem.alloc(8);
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            if i > 0 {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
            e.mem.set_u32(node, *item);
        }
        head
    }

    /// Fills the list node at `list` (an embedded or static list) with the
    /// items.
    fn fill_list(e: &mut Engine, list: u32, items: &[u32]) {
        let built = make_list(e, items);
        let item = e.mem.u32(built);
        let next = e.mem.u32(built + 4);
        e.mem.set_u32(list, item);
        e.mem.set_u32(list + 4, next);
    }

    fn action(e: &mut Engine, form: u32, flags: u32) -> u32 {
        let block = e.mem.alloc(8);
        e.mem.set_u32(block, form);
        e.mem.set_u32(block + 4, flags);
        block
    }

    fn variable(e: &mut Engine, id: u32, value: f64) -> u32 {
        let block = e.mem.alloc(0x18);
        e.mem.set_u32(block, id);
        e.mem.set_f64(block + 8, value);
        block
    }

    fn locals(e: &mut Engine) -> Ptr<ScriptLocals> {
        e.new_object::<ScriptLocals>()
    }

    fn set_actions(e: &mut Engine, this: Ptr<ScriptLocals>, entries: &[(u32, u32)]) -> Vec<u32> {
        let blocks: Vec<u32> = entries.iter().map(|(f, fl)| action(e, *f, *fl)).collect();
        let list = make_list(e, &blocks);
        e.set(this, ScriptLocals::m_pActionList, Ptr::new(list));
        blocks
    }

    fn set_variables(e: &mut Engine, this: Ptr<ScriptLocals>, entries: &[(u32, f64)]) -> Vec<u32> {
        let blocks: Vec<u32> = entries.iter().map(|(id, v)| variable(e, *id, *v)).collect();
        let list = make_list(e, &blocks);
        e.set(this, ScriptLocals::m_pLocalList, Ptr::new(list));
        blocks
    }

    fn action_list(e: &Engine, this: Ptr<ScriptLocals>) -> Vec<(u32, u32)> {
        let head = e.get(this, ScriptLocals::m_pActionList).addr();
        list_items(e, head)
            .iter()
            .map(|a| (e.mem.u32(*a), e.mem.u32(*a + 4)))
            .collect()
    }

    /// A master script whose referenced-object list holds objects with the
    /// given variable ids.
    fn script_with_references(e: &mut Engine, ids: &[u32]) -> Ptr<Script> {
        let script = e.new_object::<Script>();
        let objects: Vec<u32> = ids
            .iter()
            .map(|id| {
                let object = e.mem.alloc(0x10);
                e.mem.set_u32(object + 0xc, *id);
                object
            })
            .collect();
        fill_list(e, script.addr() + 0x44, &objects);
        script
    }

    /// Registers `f` at a fresh fake code address and returns it.
    fn fake_code(e: &mut Engine, f: impl FnMut(&mut Engine, &[u32]) -> Ret + 'static) -> u32 {
        let address = e.mem.alloc(4);
        e.register_double(address, f);
        address
    }

    /// An object whose virtual slots (byte offsets) call the given fake
    /// code addresses.
    fn fake_object(e: &mut Engine, slots: &[(u32, u32)]) -> u32 {
        let object = e.mem.alloc(0x20);
        let table = e.mem.alloc(0x200);
        for (slot, target) in slots {
            e.mem.set_u32(table + slot, *target);
        }
        e.mem.set_u32(object, table);
        object
    }

    /// A form whose type-name slot (`+0x130`) returns `name`.
    fn named_form(e: &mut Engine, name: u32) -> u32 {
        let target = fake_code(e, move |_, _| ret(name));
        fake_object(e, &[(0x130, target)])
    }

    /// The argument lists of the calls to `addr` in the call log.
    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The blocks passed to `operator delete`, in order.
    fn deleted(log: &[(u32, Vec<u32>)]) -> Vec<u32> {
        calls_to(log, OPERATOR_DELETE)
            .iter()
            .map(|a| a[0])
            .collect()
    }

    // ---- save/load game doubles --------------------------------------------

    /// The save/load game object (0x200 bytes at the global) with: the
    /// "use blocks" byte at +0x100, the file version at +0x101, the header
    /// of the form being loaded at +0x108, of the form being saved at
    /// +0x10C, the form `LookupFormByID` returns at +0x110, and the file name
    /// word at +0x1C4. The numeric id doubles keep the id in the low word of
    /// the double (a test convention: the real encoding is `005acc70`'s).
    fn save_load_game_object(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x200);
        e.set_global(SAVE_LOAD_GAME_GLOBAL, object);
        e.register(USE_SAVE_GAME_BLOCKS, |e, a| {
            ret(e.mem.u8(a[0] + 0x100) as u32)
        });
        e.register(SAVE_FILE_VERSION, |e, a| ret(e.mem.u8(a[0] + 0x101) as u32));
        e.register(LOADING_FORM_HEADER, |e, a| ret(e.mem.u32(a[0] + 0x108)));
        e.register(SAVING_FORM_HEADER, |e, a| ret(e.mem.u32(a[0] + 0x10c)));
        e.register(LOOKUP_FORM_BY_ID, |e, _| {
            let save_load: u32 = e.global(SAVE_LOAD_GAME_GLOBAL);
            ret(e.mem.u32(save_load + 0x110))
        });
        e.register(IS_LOADING_STUB, |_, _| ret(0));
        e.register(FLAG_BYTE_ADDRESS, |_, a| ret(a[0] + 4));
        e.register(GET_NUMERIC_ID_FROM_DOUBLE, |e, a| {
            let id = e.mem.f64(a[1]) as u32;
            e.mem.set_u32(a[0], id);
            Ret::default()
        });
        e.register(PUT_NUMERIC_ID_IN_DOUBLE, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_f64(a[1], id as f64);
            Ret::default()
        });
        object
    }

    /// A byte stream the old save format writes and reads: the position
    /// double returns an address in a buffer, the write and read doubles
    /// copy bytes and advance. Returns the buffer address and the cursor.
    fn install_stream(e: &mut Engine) -> (u32, Rc<Cell<u32>>) {
        let base = e.mem.alloc(0x400);
        let cursor = Rc::new(Cell::new(0u32));
        let c = cursor.clone();
        e.register_double(STREAM_POSITION, move |_, _| ret(base + c.get()));
        for write in [STREAM_WRITE, STREAM_SAVE_NUMERIC_ID] {
            let c = cursor.clone();
            e.register_double(write, move |e, a| {
                let bytes = e.mem.bytes(a[1], a[2]);
                e.mem.write(base + c.get(), &bytes);
                c.set(c.get() + a[2]);
                Ret::default()
            });
        }
        for read in [STREAM_READ, STREAM_LOAD_NUMERIC_ID] {
            let c = cursor.clone();
            e.register_double(read, move |e, a| {
                let bytes = e.mem.bytes(base + c.get(), a[2]);
                e.mem.write(a[1], &bytes);
                c.set(c.get() + a[2]);
                Ret::default()
            });
        }
        (base, cursor)
    }

    // ---- ScriptLocals lists --------------------------------------------------

    #[test]
    fn destructor_body_frees_both_lists_and_the_effect_data() {
        let mut e = script_engine();
        let this = locals(&mut e);
        let actions = set_actions(&mut e, this, &[(0x100, 1)]);
        let variables = set_variables(&mut e, this, &[(1, 2.0)]);
        let effect = e.mem.alloc(8);
        e.set(this, ScriptLocals::m_pScriptEffectData, Ptr::new(effect));
        e.call_log = Some(vec![]);
        e.call(0x005a_8bc0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(deleted(&log), vec![actions[0], variables[0], effect]);
        assert!(e.get(this, ScriptLocals::m_pActionList).is_null());
        assert!(e.get(this, ScriptLocals::m_pLocalList).is_null());
        assert_eq!(calls_to(&log, LIST_DELETE).len(), 2);
    }

    #[test]
    fn destructor_body_with_nothing_to_free_does_nothing() {
        let mut e = script_engine();
        let this = locals(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005a_8bc0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(deleted(&log).is_empty());
    }

    #[test]
    fn clear_action_list_leaves_the_variables() {
        let mut e = script_engine();
        let this = locals(&mut e);
        let actions = set_actions(&mut e, this, &[(1, 0), (2, 0)]);
        set_variables(&mut e, this, &[(1, 2.0)]);
        e.call_log = Some(vec![]);
        e.call(0x005a_8c00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(deleted(&log), actions);
        assert!(e.get(this, ScriptLocals::m_pActionList).is_null());
        assert!(!e.get(this, ScriptLocals::m_pLocalList).is_null());
    }

    #[test]
    fn clear_variable_list_leaves_the_actions() {
        let mut e = script_engine();
        let this = locals(&mut e);
        set_actions(&mut e, this, &[(1, 0)]);
        let variables = set_variables(&mut e, this, &[(1, 2.0), (2, 3.0)]);
        e.call_log = Some(vec![]);
        e.call(0x005a_8c90, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(deleted(&log), variables);
        assert!(e.get(this, ScriptLocals::m_pLocalList).is_null());
        assert!(!e.get(this, ScriptLocals::m_pActionList).is_null());
    }

    #[test]
    fn action_entry_is_created_once_inside_the_scope_guard() {
        let mut e = script_engine();
        let this = locals(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005a_8d20, &args![this, 0x1234u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(action_list(&e, this), vec![(0x1234, 0)]);
        // The guard opens first (kind 0x15, line 300) and closes last.
        assert_eq!(log[1].0, SCOPE_GUARD_OPEN);
        assert_eq!(&log[1].1[1..], &[0x15, 1, SOURCE_FILE, 300]);
        assert_eq!(log.last().unwrap().0, SCOPE_GUARD_CLOSE);
        // A second call for the same form adds nothing; another form does.
        e.call(0x005a_8d20, &args![this, 0x1234u32]);
        assert_eq!(action_list(&e, this).len(), 1);
        e.call(0x005a_8d20, &args![this, 0x5678u32]);
        assert_eq!(action_list(&e, this), vec![(0x5678, 0), (0x1234, 0)]);
    }

    #[test]
    fn set_action_flag_ors_into_the_form_and_the_any_form_entries() {
        let mut e = script_engine();
        let this = locals(&mut e);
        // No action list: nothing happens.
        assert!(!e.call(0x005a_8e20, &args![this, 0x10u32, 0x1000u32]).bool());
        set_actions(&mut e, this, &[(0x10, 1), (0, 2), (0x20, 4)]);
        assert!(e.call(0x005a_8e20, &args![this, 0x10u32, 0x1000u32]).bool());
        assert_eq!(
            action_list(&e, this),
            vec![(0x10, 0x1001), (0, 0x1002), (0x20, 4)]
        );
        // A form with no entry still reaches the any-form entry.
        assert!(e.call(0x005a_8e20, &args![this, 0x99u32, 0x8u32]).bool());
        assert_eq!(action_list(&e, this)[1], (0, 0x100a));
    }

    #[test]
    fn set_action_flag_reports_false_when_no_entry_exists() {
        let mut e = script_engine();
        let this = locals(&mut e);
        set_actions(&mut e, this, &[(0x20, 4)]);
        assert!(!e.call(0x005a_8e20, &args![this, 0x10u32, 0x1000u32]).bool());
        assert_eq!(action_list(&e, this), vec![(0x20, 4)]);
    }

    #[test]
    fn clear_action_flags_zeroes_every_entry() {
        let mut e = script_engine();
        let this = locals(&mut e);
        set_actions(&mut e, this, &[(0x10, 7), (0x20, 0x1000)]);
        e.call(0x005a_8ea0, &args![this]);
        assert_eq!(action_list(&e, this), vec![(0x10, 0), (0x20, 0)]);
        // No list is fine too.
        let bare = locals(&mut e);
        e.call(0x005a_8ea0, &args![bare]);
    }

    #[test]
    fn action_flag_test_creates_a_missing_entry() {
        let mut e = script_engine();
        let this = locals(&mut e);
        set_actions(&mut e, this, &[(0x10, 0x1000)]);
        assert!(e.call(0x005a_8ef0, &args![this, 0x10u32, 0x1000u32]).bool());
        assert!(!e.call(0x005a_8ef0, &args![this, 0x10u32, 0x0004u32]).bool());
        // Unknown form: false, and an entry with no flags appears.
        assert!(!e.call(0x005a_8ef0, &args![this, 0x77u32, 0x1000u32]).bool());
        assert_eq!(action_list(&e, this), vec![(0x77, 0), (0x10, 0x1000)]);
    }

    #[test]
    fn action_list_copy_duplicates_every_entry() {
        let mut e = script_engine();
        let this = locals(&mut e);
        set_actions(&mut e, this, &[(0x99, 5)]);
        let source = locals(&mut e);
        let originals = set_actions(&mut e, source, &[(0x10, 1), (0x20, 2), (0x30, 3)]);
        e.call(0x005a_8f40, &args![this, source.addr()]);
        assert_eq!(action_list(&e, this), vec![(0x10, 1), (0x20, 2), (0x30, 3)]);
        // The entries are copies, not the source's blocks.
        let head = e.get(this, ScriptLocals::m_pActionList).addr();
        for copy in list_items(&e, head) {
            assert!(!originals.contains(&copy));
        }
    }

    #[test]
    fn action_list_copy_from_nothing_only_clears() {
        let mut e = script_engine();
        let this = locals(&mut e);
        set_actions(&mut e, this, &[(0x99, 5)]);
        e.call(0x005a_8f40, &args![this, 0u32]);
        assert!(e.get(this, ScriptLocals::m_pActionList).is_null());
        // A source with no list, or an empty one, gives no list either.
        let source = locals(&mut e);
        e.call(0x005a_8f40, &args![this, source.addr()]);
        assert!(e.get(this, ScriptLocals::m_pActionList).is_null());
        set_actions(&mut e, source, &[]);
        e.call(0x005a_8f40, &args![this, source.addr()]);
        assert!(e.get(this, ScriptLocals::m_pActionList).is_null());
    }

    #[test]
    fn find_action_by_form_or_the_last_empty_entry() {
        let mut e = script_engine();
        let this = locals(&mut e);
        assert_eq!(e.call(0x005a_90b0, &args![this, 1u32, true]).u32(), 0);
        let blocks = set_actions(&mut e, this, &[(0x10, 0), (0, 0), (0x20, 0), (0, 0)]);
        let found = e.call(0x005a_90b0, &args![this, 0x20u32, false]).u32();
        assert_eq!(found, blocks[2]);
        // No match: null, or with the create flag the last entry with no form.
        assert_eq!(e.call(0x005a_90b0, &args![this, 0x30u32, false]).u32(), 0);
        assert_eq!(
            e.call(0x005a_90b0, &args![this, 0x30u32, true]).u32(),
            blocks[3]
        );
        // A form that matches wins even with the create flag.
        assert_eq!(
            e.call(0x005a_90b0, &args![this, 0x10u32, true]).u32(),
            blocks[0]
        );
    }

    // ---- variables -------------------------------------------------------------

    #[test]
    fn get_variable_searches_and_caches_the_last_lookup() {
        let mut e = script_engine();
        let this = locals(&mut e);
        let vars = set_variables(&mut e, this, &[(5, 2.5), (9, 7.0)]);
        let value = e.call(0x005a_9140, &args![this, 9u32, 0u32]).f64();
        assert_eq!(value, 7.0);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_LAST_LOCALS), this.addr());
        assert_eq!(e.mem.u32(tls + TLS_LAST_VARIABLE_ID), 9);
        assert_eq!(e.mem.u32(tls + TLS_LAST_VARIABLE), vars[1]);
        // The cache answers the same lookup even if the list is gone.
        e.mem.set_f64(vars[1] + 8, 8.0);
        e.set(this, ScriptLocals::m_pLocalList, Ptr::NULL);
        assert_eq!(e.call(0x005a_9140, &args![this, 9u32, 0u32]).f64(), 8.0);
    }

    #[test]
    fn get_variable_missing_logs_and_reads_zero() {
        let mut e = script_engine();
        let this = locals(&mut e);
        set_variables(&mut e, this, &[(5, 2.5)]);
        e.set_global(SCRIPT_LINE_GLOBAL, 42u32);
        e.call_log = Some(vec![]);
        let value = e.call(0x005a_9140, &args![this, 6u32, 0u32]).f64();
        assert_eq!(value, 0.0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![FORMAT_VARIABLE_NOT_FOUND, 6, UNKNOWN_SCRIPT_NAME, 42]]
        );
        // A named script is reported by its name.
        let script = named_form(&mut e, 0xabc0);
        e.call_log = Some(vec![]);
        e.call(0x005a_9140, &args![this, 7u32, script]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![FORMAT_VARIABLE_NOT_FOUND, 7, 0xabc0, 42]]
        );
    }

    const NATIVE_LOCAL_VALUES: [(u64, u64); 9] = [
        (0, 0),
        (0x8000_0000_0000_0000, 0x8000_0000_0000_0000),
        (1, 1),
        (0x4004_0000_0000_0000, 0x4004_0000_0000_0000),
        (0x7ff0_0000_0000_0000, 0x7ff0_0000_0000_0000),
        (0xfff0_0000_0000_0000, 0xfff0_0000_0000_0000),
        (0x7ff8_1234_5678_9abc, 0x7ff8_1234_5678_9abc),
        (0x7ff0_1234_5678_9abc, 0x7ff8_1234_5678_9abc),
        (0xfff0_1234_5678_9abc, 0xfff8_1234_5678_9abc),
    ];

    #[test]
    fn local_copy_matches_native_flag_value_and_guards() {
        // PC005ac150, generated native oracle FCW027F/MXCSR1F80.
        // All36 guarded cases independently compared byte-for-byte.
        for (input_bits, output_bits) in NATIVE_LOCAL_VALUES {
            for flag in [0, 1, 0x7f, 0xff] {
                let mut e = Engine::new();
                let block = e.mem.alloc(80);
                let mut bytes = vec![0xa5; 80];
                bytes[4..8].copy_from_slice(&0x1234_5678u32.to_le_bytes());
                bytes[12..20].copy_from_slice(&input_bits.to_le_bytes());
                bytes[20] = flag;
                e.mem.write(block, &bytes);
                bytes[40..44].copy_from_slice(&0x1234_5678u32.to_le_bytes());
                bytes[48..56].copy_from_slice(&output_bits.to_le_bytes());
                bytes[56] = flag;
                assert_eq!(
                    e.call(0x005ac150, &[block + 40, block + 4]).u32(),
                    block + 40
                );
                assert_eq!(
                    e.mem.bytes(block, 80),
                    bytes,
                    "input {input_bits:016x} flag {flag:02x}"
                );
            }
        }
    }

    #[test]
    fn local_copy_matches_native_overlapping_receivers() {
        // Native005ac150 writes flag, ID, then float, with fresh source loads.
        // These generated expected buffers were independently captured on PC.
        for (destination, hex) in [
            (4, "a5a5a5a578563412a5a5a5a5bc9a78563412f87fffa5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5"),
            (0, "7856341278563412bc9a7856ff12f87fff12f07fffa5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5"),
            (8, "a5a5a5a57856341278563412bc9a7856bc9a78563412f87fffa5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5"),
            (12, "a5a5a5a578563412a5a5a5a5785634123412f07f785634123412f87fffa5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5"),
        ] {
            let mut e = Engine::new();
            let block = e.mem.alloc(80);
            e.mem.write(block, &[0xa5; 80]);
            e.mem.set_u32(block + 4, 0x1234_5678);
            e.mem.write(block + 12, &0x7ff0_1234_5678_9abcu64.to_le_bytes());
            e.mem.set_u8(block + 20, 0xff);
            let expected = (0..hex.len()).step_by(2)
                .map(|at| u8::from_str_radix(&hex[at..at + 2], 16).unwrap())
                .collect::<Vec<_>>();
            assert_eq!(expected.len(), 80);
            assert_eq!(e.call(0x005ac150, &[block + destination, block + 4]).u32(), block + destination);
            assert_eq!(e.mem.bytes(block, 80), expected, "destination {destination}");
        }
    }

    #[test]
    fn set_variable_matches_native_first_match_value_and_guards() {
        // Native005a9290:18 generated cases, matching ID1 or the first ID2.
        for (mut input, mut output) in NATIVE_LOCAL_VALUES {
            if input == 1 {
                input = 0xf0;
                output = 0xf0;
            }
            for id in [1, 2] {
                let mut e = Engine::new();
                let block = e.mem.alloc(168);
                let mut bytes = vec![0xa5; 168];
                for (offset, word) in [
                    (16, block + 32),
                    (32, block + 64),
                    (36, block + 40),
                    (40, block + 96),
                    (44, block + 48),
                    (48, block + 128),
                    (52, 0),
                    (64, 1),
                    (96, 2),
                    (128, 2),
                ] {
                    bytes[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
                }
                e.mem.write(block, &bytes);
                let offset = if id == 1 { 72 } else { 104 };
                bytes[offset..offset + 8].copy_from_slice(&output.to_le_bytes());
                e.call(
                    0x005a9290,
                    &args![block + 4, id as u32, f64::from_bits(input)],
                );
                assert_eq!(e.mem.bytes(block, 168), bytes, "id{id} input{input:016x}");
            }
        }
    }

    #[test]
    fn set_variable_fetches_the_item_again_before_advancing() {
        let mut e = script_engine();
        let this = locals(&mut e);
        let vars = set_variables(&mut e, this, &[(5, 2.5), (9, 7.0)]);
        let head = e.mem.u32(this.addr() + 0xc);
        let replacement = vars[1];
        let calls = Rc::new(Cell::new(0));
        let observed = calls.clone();
        e.register_double(LIST_ITEM_SLOT, move |e, a| {
            let count = observed.get() + 1;
            observed.set(count);
            if count == 2 {
                e.mem.set_u32(a[0], replacement);
            }
            ret(a[0])
        });
        e.call_log = Some(Vec::new());
        e.call(0x005a9290, &args![this, 9u32, 3.25f64]);
        assert_eq!(calls.get(), 2);
        assert_eq!(e.mem.u32(head), replacement);
        assert_eq!(e.mem.f64(vars[0] + 8), 2.5);
        assert_eq!(e.mem.f64(vars[1] + 8), 3.25);
        let order = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter_map(|(address, _)| {
                [LIST_ITEM_SLOT, LIST_NEXT]
                    .contains(address)
                    .then_some(*address)
            })
            .collect::<Vec<_>>();
        assert_eq!(order, [LIST_ITEM_SLOT, LIST_ITEM_SLOT, LIST_NEXT]);
    }

    #[test]
    fn set_variable_stores_the_value() {
        let mut e = script_engine();
        let this = locals(&mut e);
        let vars = set_variables(&mut e, this, &[(5, 2.5), (9, 7.0)]);
        e.call(0x005a_9290, &args![this, 9u32, 3.25f64]);
        assert_eq!(e.mem.f64(vars[1] + 8), 3.25);
        assert_eq!(e.mem.f64(vars[0] + 8), 2.5);
    }

    #[test]
    fn set_variable_missing_in_a_script_names_the_script() {
        let mut e = script_engine();
        let this = locals(&mut e);
        set_variables(&mut e, this, &[(5, 2.5)]);
        let master = named_form(&mut e, 0xabc0);
        e.set(this, ScriptLocals::m_pMasterScript, Ptr::new(master));
        e.call_log = Some(vec![]);
        e.call(0x005a_9290, &args![this, 8u32, 1.0f64]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![FORMAT_SET_VARIABLE_IN_SCRIPT, 8, 0xabc0]]
        );
    }

    #[test]
    fn set_variable_missing_without_a_script_logs_by_load_state() {
        let mut e = script_engine();
        let save_load = save_load_game_object(&mut e);
        let this = locals(&mut e);
        // Not loading (the stub answers false): the plain message.
        e.call_log = Some(vec![]);
        e.call(0x005a_9290, &args![this, 8u32, 1.0f64]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![FORMAT_SET_VARIABLE_NO_SCRIPT, 8]]
        );

        // Loading a form that is a reference: the file name, id, name and
        // the form id are reported.
        e.register(IS_LOADING_STUB, |_, _| ret(1));
        let header = e.mem.alloc(0x10);
        e.mem.set_u32(header, 0x0001_0203);
        e.mem.set_u32(save_load + 0x108, header);
        e.mem.set_u32(save_load + 0x110, 0x7000);
        e.mem.set_u32(save_load + 0x1c4, 0xf11e);
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(FULL_NAME_OF_REFERENCE, |_, _| ret(0x4e01));
        e.register(FULL_NAME_OF_FORM, |_, _| ret(0x4e02));
        e.call_log = Some(vec![]);
        e.call(0x005a_9290, &args![this, 8u32, 1.0f64]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![
                FORMAT_SET_VARIABLE_REF,
                0xf11e,
                8,
                0x4e01,
                0x0001_0203
            ]]
        );

        // The cast fails: the plain form name.
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x005a_9290, &args![this, 8u32, 1.0f64]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![
                FORMAT_SET_VARIABLE_NAMED,
                0xf11e,
                8,
                0x4e02,
                0x0001_0203
            ]]
        );

        // No name: the message without one.
        e.register(FULL_NAME_OF_FORM, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x005a_9290, &args![this, 8u32, 1.0f64]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![FORMAT_SET_VARIABLE_FORM, 0xf11e, 8, 0x0001_0203]]
        );

        // Loading with no form header: the plain message again.
        e.mem.set_u32(save_load + 0x108, 0);
        e.call_log = Some(vec![]);
        e.call(0x005a_9290, &args![this, 8u32, 1.0f64]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![FORMAT_SET_VARIABLE_NO_SCRIPT, 8]]
        );
    }

    #[test]
    fn file_name_is_read_from_the_save_load_object() {
        let mut e = script_engine();
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object + 0x1c4, 0xfeed);
        assert_eq!(e.call(0x005a_9460, &args![object]).u32(), 0xfeed);
    }

    #[test]
    fn form_reference_variables_are_found_in_the_master_script() {
        let mut e = script_engine();
        let this = locals(&mut e);
        // No master script: no variable is a reference.
        assert!(!e.call(0x005a_9480, &args![this, 3u32]).bool());
        let master = script_with_references(&mut e, &[2, 3]);
        e.set(this, ScriptLocals::m_pMasterScript, Ptr::new(master.addr()));
        assert!(e.call(0x005a_9480, &args![this, 3u32]).bool());
        assert!(e.call(0x005a_9480, &args![this, 2u32]).bool());
        assert!(!e.call(0x005a_9480, &args![this, 4u32]).bool());
        // A master with no referenced objects has none.
        let empty = script_with_references(&mut e, &[]);
        e.set(this, ScriptLocals::m_pMasterScript, Ptr::new(empty.addr()));
        assert!(!e.call(0x005a_9480, &args![this, 3u32]).bool());
    }

    // ---- saving ------------------------------------------------------------------

    #[test]
    fn save_size_counts_the_variables_the_old_format_writes() {
        let mut e = script_engine();
        let save_load = save_load_game_object(&mut e);
        let this = locals(&mut e);
        // A zero value is not saved; a form reference takes 8 bytes, any
        // other 12; the effect-data flag takes 1.
        set_variables(&mut e, this, &[(1, 5.0), (2, 0.0), (3, 6.0)]);
        let master = script_with_references(&mut e, &[3]);
        e.set(this, ScriptLocals::m_pMasterScript, Ptr::new(master.addr()));
        assert_eq!(e.call(0x005a_94f0, &args![this]).u16(), 2 + 12 + 8 + 1);
        // Blocks add 6; effect data adds 8.
        e.mem.set_u8(save_load + 0x100, 1);
        let effect = e.mem.alloc(8);
        e.set(this, ScriptLocals::m_pScriptEffectData, Ptr::new(effect));
        assert_eq!(
            e.call(0x005a_94f0, &args![this]).u16(),
            6 + 2 + 12 + 8 + 1 + 8
        );
    }

    #[test]
    fn save_size_is_logged_when_logging_is_on() {
        let mut e = script_engine();
        let save_load = save_load_game_object(&mut e);
        let this = locals(&mut e);
        set_variables(&mut e, this, &[(1, 5.0)]);
        e.mem.set_u8(SAVE_SIZE_LOG_OBJECT + 4, 1);
        // Without a form being saved: size, line and file.
        e.call_log = Some(vec![]);
        e.call(0x005a_94f0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ERROR_LOG),
            vec![vec![FORMAT_SAVE_SIZE_PLAIN, 15, 0x27e, SOURCE_FILE]]
        );
        // With one: also its id, type name and flags.
        let header = e.mem.alloc(0x10);
        e.mem.set_u32(header, 0x00aa_bbcc);
        e.mem.set_u32(header + 5, 0x40);
        e.mem.set_u32(save_load + 0x10c, header);
        let form = named_form(&mut e, 0xabc0);
        e.mem.set_u32(save_load + 0x110, form);
        e.call_log = Some(vec![]);
        e.call(0x005a_94f0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, ERROR_LOG),
            vec![vec![
                FORMAT_SAVE_SIZE_FORM,
                15,
                0x00aa_bbcc,
                0xabc0,
                0x40,
                0x27e,
                SOURCE_FILE
            ]]
        );
    }

    #[test]
    fn old_save_writes_variables_count_and_block_size() {
        let mut e = script_engine();
        let save_load = save_load_game_object(&mut e);
        e.mem.set_u8(save_load + 0x100, 1);
        let (base, cursor) = install_stream(&mut e);
        let this = locals(&mut e);
        set_variables(&mut e, this, &[(1, 5.0), (2, 0.0), (3, 6.0)]);
        let master = script_with_references(&mut e, &[3]);
        e.set(this, ScriptLocals::m_pMasterScript, Ptr::new(master.addr()));
        e.call(0x005a_9670, &args![this]);
        // BLOK, block size, count, variable 1 (id, value), variable 3 as a
        // form reference (id with the top bit, the numeric id), no effect data.
        assert_eq!(e.mem.u32(base), BLOCK_MARKER);
        assert_eq!(e.mem.u16(base + 6), 2);
        assert_eq!(e.mem.u32(base + 8), 1);
        assert_eq!(e.mem.f64(base + 12), 5.0);
        assert_eq!(e.mem.u32(base + 20), 3 | 0x8000_0000);
        assert_eq!(e.mem.u32(base + 24), 6);
        assert_eq!(e.mem.u8(base + 28), 0);
        assert_eq!(cursor.get(), 29);
        // The block size counts from its own position to the end.
        assert_eq!(e.mem.u16(base + 4), 29 - 4);
    }

    #[test]
    fn old_save_without_blocks_writes_the_effect_data() {
        let mut e = script_engine();
        save_load_game_object(&mut e);
        let (base, cursor) = install_stream(&mut e);
        let this = locals(&mut e);
        let effect = e.mem.alloc(8);
        e.mem.set_u32(effect, 0x0101_0101);
        e.set(this, ScriptLocals::m_pScriptEffectData, Ptr::new(effect));
        e.call(0x005a_9670, &args![this]);
        assert_eq!(e.mem.u16(base), 0);
        assert_eq!(e.mem.u8(base + 2), 1);
        assert_eq!(e.mem.u32(base + 3), 0x0101_0101);
        assert_eq!(cursor.get(), 11);
    }

    #[test]
    fn old_save_logs_sizes_and_oversized_blocks() {
        let mut e = script_engine();
        let save_load = save_load_game_object(&mut e);
        e.mem.set_u8(save_load + 0x100, 1);
        e.mem.set_u8(SAVE_SIZE_LOG_OBJECT + 4, 1);
        let (_, cursor) = install_stream(&mut e);
        let this = locals(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005a_9670, &args![this]);
        let log = e.call_log.take().unwrap();
        // marker 4 + block size 2 + count 2 + flag 1.
        assert_eq!(
            calls_to(&log, ERROR_LOG),
            vec![vec![FORMAT_SAVE_PLAIN, 9, 0x2b6, SOURCE_FILE]]
        );
        assert_eq!(cursor.get(), 9);
        // A block larger than a short is reported: the position double
        // jumps far ahead once the content has been written.
        let far = e.mem.alloc(0x400);
        // The fourth position (the end of the block) is far past the start.
        let asked = Rc::new(Cell::new(0u32));
        e.register_double(STREAM_POSITION, move |_, _| {
            asked.set(asked.get() + 1);
            ret(far + if asked.get() >= 4 { 0x2_0000 } else { 0 })
        });
        e.register(STREAM_WRITE, |_, _| Ret::default());
        cursor.set(0);
        e.call_log = Some(vec![]);
        e.mem.set_u8(SAVE_SIZE_LOG_OBJECT + 4, 0);
        e.call(0x005a_9670, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![FORMAT_BLOCK_TOO_LARGE, SOURCE_FILE, 0x2b6]]
        );
    }

    #[test]
    fn old_save_and_old_load_round_trip() {
        for blocks in [0u8, 1] {
            let mut e = script_engine();
            let save_load = save_load_game_object(&mut e);
            e.mem.set_u8(save_load + 0x100, blocks);
            e.mem.set_u8(save_load + 0x101, 0x80);
            let (_, cursor) = install_stream(&mut e);
            let this = locals(&mut e);
            set_variables(&mut e, this, &[(1, 5.0), (3, 6.0), (4, -2.0)]);
            let master = script_with_references(&mut e, &[3]);
            e.set(this, ScriptLocals::m_pMasterScript, Ptr::new(master.addr()));
            let effect = e.mem.alloc(8);
            e.mem.set_u32(effect + 4, 0x2222);
            e.set(this, ScriptLocals::m_pScriptEffectData, Ptr::new(effect));
            e.call(0x005a_9670, &args![this]);
            let written = cursor.get();

            // Load into locals that have the variables with other values.
            cursor.set(0);
            let other = locals(&mut e);
            let vars = set_variables(&mut e, other, &[(1, 0.5), (3, 0.5), (4, 0.5)]);
            e.call_log = Some(vec![]);
            e.call(0x005a_9950, &args![other]);
            let log = e.call_log.take().unwrap();
            assert_eq!(cursor.get(), written);
            assert_eq!(e.mem.f64(vars[0] + 8), 5.0);
            assert_eq!(e.mem.f64(vars[1] + 8), 6.0);
            assert_eq!(e.mem.f64(vars[2] + 8), -2.0);
            let loaded = e.get(other, ScriptLocals::m_pScriptEffectData);
            assert!(!loaded.is_null());
            assert_eq!(e.mem.u32(loaded.addr() + 4), 0x2222);
            // Nothing is reported for a correct stream.
            assert!(calls_to(&log, SCRIPT_LOG).is_empty());
        }
    }

    #[test]
    fn old_load_of_an_old_file_reads_id_and_value_plainly() {
        let mut e = script_engine();
        let save_load = save_load_game_object(&mut e);
        e.mem.set_u8(save_load + 0x101, 0x70);
        let (base, cursor) = install_stream(&mut e);
        // One variable: count 1, id 7, value 4.5, no effect data.
        e.mem.set_u16(base, 1);
        e.mem.set_u32(base + 2, 7);
        e.mem.set_f64(base + 6, 4.5);
        e.mem.set_u8(base + 14, 0);
        let this = locals(&mut e);
        let vars = set_variables(&mut e, this, &[(7, 0.0)]);
        e.call(0x005a_9950, &args![this]);
        assert_eq!(e.mem.f64(vars[0] + 8), 4.5);
        assert_eq!(cursor.get(), 15);
    }

    #[test]
    fn old_load_reports_a_bad_block_header_and_size_mismatches() {
        let mut e = script_engine();
        let save_load = save_load_game_object(&mut e);
        e.mem.set_u8(save_load + 0x100, 1);
        e.mem.set_u8(save_load + 0x101, 0x80);
        let (base, cursor) = install_stream(&mut e);
        // A wrong marker; the block claims 3 bytes.
        e.mem.set_u32(base, 0x1111_1111);
        e.mem.set_u16(base + 4, 3);
        e.mem.set_u16(base + 6, 0);
        e.mem.set_u8(base + 8, 0);
        let this = locals(&mut e);
        e.call_log = Some(vec![]);
        e.call(0x005a_9950, &args![this]);
        let log = e.call_log.take().unwrap();
        let messages = calls_to(&log, SCRIPT_LOG);
        assert_eq!(
            messages[0],
            vec![FORMAT_BLOCK_HEADER_PLAIN, SOURCE_FILE, 0x2bc, 0x80]
        );
        // The block starts after the marker (4), claims 3 bytes: expected
        // end 7, the stream is at 9: an overrun of 2.
        assert_eq!(
            messages[1],
            vec![FORMAT_OVERRUN_PLAIN, 2, SOURCE_FILE, 0x2eb, 0x80]
        );
        assert_eq!(messages.len(), 2);

        // With a form being loaded, its header is reported, and a block
        // that claims too much is an underrun.
        let header = e.mem.alloc(0x10);
        e.mem.set_u32(header, 0x0000_0abc);
        e.mem.set_u32(header + 5, 0x22);
        e.mem.set_u8(header + 9, 0x33);
        e.mem.set_u32(save_load + 0x108, header);
        let form = named_form(&mut e, 0xabc0);
        e.mem.set_u32(save_load + 0x110, form);
        e.mem.set_u32(base, BLOCK_MARKER);
        e.mem.set_u16(base + 4, 100);
        cursor.set(0);
        e.call_log = Some(vec![]);
        let again = locals(&mut e);
        e.call(0x005a_9950, &args![again]);
        let log = e.call_log.take().unwrap();
        // Read to 9 bytes; the block ends at 4 + 100: an underrun of 95.
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![
                FORMAT_UNDERRUN_FORM,
                95,
                SOURCE_FILE,
                0x2eb,
                0xabc,
                0xabc0,
                0x33,
                0x22
            ]]
        );
    }

    #[test]
    fn reset_rebuilds_the_variables_and_frees_the_effect_data() {
        let mut e = script_engine();
        let this = locals(&mut e);
        set_actions(&mut e, this, &[(1, 0x1000)]);
        set_variables(&mut e, this, &[(1, 2.0)]);
        let effect = e.mem.alloc(8);
        e.set(this, ScriptLocals::m_pScriptEffectData, Ptr::new(effect));
        let master = e.mem.alloc(0x60);
        e.set(this, ScriptLocals::m_pMasterScript, Ptr::new(master));
        let rebuilt = e.mem.alloc(8);
        e.register_double(SCRIPT_BUILD_LOCALS, move |_, a| {
            assert_eq!(a[0], master);
            ret(rebuilt)
        });
        e.call_log = Some(vec![]);
        e.call(0x005a_9d00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(this, ScriptLocals::m_pLocalList).addr(), rebuilt);
        assert_eq!(action_list(&e, this), vec![(1, 0)]);
        assert!(e.get(this, ScriptLocals::m_pScriptEffectData).is_null());
        assert!(deleted(&log).contains(&effect));
        // Without a master script the list is only emptied.
        let bare = locals(&mut e);
        set_variables(&mut e, bare, &[(1, 2.0)]);
        e.call(0x005a_9d00, &args![bare]);
        assert!(e.get(bare, ScriptLocals::m_pLocalList).is_null());
    }

    #[test]
    fn the_second_reset_entry_does_the_same() {
        let mut e = script_engine();
        let this = locals(&mut e);
        set_actions(&mut e, this, &[(1, 0x1000)]);
        set_variables(&mut e, this, &[(1, 2.0)]);
        let effect = e.mem.alloc(8);
        e.set(this, ScriptLocals::m_pScriptEffectData, Ptr::new(effect));
        e.call(0x005a_a090, &args![this, 0xdead_beefu32]);
        assert_eq!(action_list(&e, this), vec![(1, 0)]);
        assert!(e.get(this, ScriptLocals::m_pLocalList).is_null());
        assert!(e.get(this, ScriptLocals::m_pScriptEffectData).is_null());
    }

    #[test]
    fn last_lookup_cache_is_cleared() {
        let mut e = script_engine();
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_LAST_LOCALS, 0x1234);
        e.mem.set_u32(tls + TLS_LAST_VARIABLE_ID, 5);
        e.mem.set_u32(tls + TLS_LAST_VARIABLE, 0x5678);
        e.call(0x005a_9d60, &args![]);
        assert_eq!(e.mem.u32(tls + TLS_LAST_LOCALS), 0);
        assert_eq!(e.mem.u32(tls + TLS_LAST_VARIABLE_ID), 0xffff_ffff);
        assert_eq!(e.mem.u32(tls + TLS_LAST_VARIABLE), 0);
    }

    /// Doubles for the `BGSSaveGameBuffer` methods: they record their
    /// arguments (through the call log) and `StartVariableSizedValue` gives
    /// `0x7000`.
    fn install_save_buffer(e: &mut Engine) -> Ptr {
        e.register(BUFFER_START_SIZED_VALUE, |_, _| ret(0x7000));
        for addr in [
            BUFFER_SAVE_SIZED_VALUE,
            BUFFER_SAVE_FORM_ID_OV2,
            BUFFER_SAVE_FORM_ID,
            BUFFER_SAVE_BYTES,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(GET_NUMERIC_ID_FROM_DOUBLE, |e, a| {
            let id = e.mem.u32(a[1]);
            e.mem.set_u32(a[0], id);
            Ret::default()
        });
        Ptr::new(e.mem.alloc(0x20))
    }

    #[test]
    fn save_game_writes_count_variables_and_flags() {
        let mut e = script_engine();
        let buffer = install_save_buffer(&mut e);
        let this = locals(&mut e);
        let vars = set_variables(&mut e, this, &[(1, 5.0), (2, 0.0), (3, 6.0)]);
        // Reference locals contain opaque form bits, not a numeric f64 ID.
        e.mem.set_f64(vars[2] + 8, f64::from_bits(6));
        let master = script_with_references(&mut e, &[3]);
        e.set(this, ScriptLocals::m_pMasterScript, Ptr::new(master.addr()));
        let effect = e.mem.alloc(8);
        e.set(this, ScriptLocals::m_pScriptEffectData, Ptr::new(effect));
        set_actions(&mut e, this, &[(0, 0x1000)]);
        e.call_log = Some(vec![]);
        e.call(0x005a_9db0, &args![this, buffer]);
        let log = e.call_log.take().unwrap();
        let buffer_calls: Vec<(u32, Vec<u32>)> = log
            .iter()
            .filter(|(a, _)| {
                [
                    BUFFER_START_SIZED_VALUE,
                    BUFFER_SAVE_SIZED_VALUE,
                    BUFFER_SAVE_FORM_ID,
                    BUFFER_SAVE_BYTES,
                ]
                .contains(a)
            })
            .cloned()
            .collect();
        let b = buffer.addr();
        // Start; variable 1 (id and value); variable 3 as a form reference
        // (id with the top bit, then the form id 6); the count 2 at the
        // position; the effect data flag and data; the action flag.
        assert_eq!(buffer_calls[0], (BUFFER_START_SIZED_VALUE, vec![b]));
        assert_eq!(buffer_calls[1], (BUFFER_SAVE_BYTES, vec![b, vars[0], 4, 0]));
        assert_eq!(
            buffer_calls[2],
            (BUFFER_SAVE_BYTES, vec![b, vars[0] + 8, 8, 0])
        );
        assert_eq!(buffer_calls[3].0, BUFFER_SAVE_BYTES);
        assert_eq!(buffer_calls[3].1[2..], [4, 0]);
        assert_eq!(buffer_calls[4], (BUFFER_SAVE_FORM_ID, vec![b, 6, 0]));
        assert_eq!(
            buffer_calls[5],
            (BUFFER_SAVE_SIZED_VALUE, vec![b, 2, 0x7000])
        );
        assert_eq!(buffer_calls[6].1[2..], [1, 0]);
        assert_eq!(buffer_calls[7], (BUFFER_SAVE_BYTES, vec![b, effect, 8, 0]));
        assert_eq!(buffer_calls[8].1[2..], [1, 0]);
        assert_eq!(buffer_calls.len(), 9);
    }

    #[test]
    fn save_game_reloads_reference_id_after_numeric_conversion() {
        let mut e = script_engine();
        let buffer = install_save_buffer(&mut e);
        let this = locals(&mut e);
        let vars = set_variables(&mut e, this, &[(3, f64::from_bits(0xff123456))]);
        let master = script_with_references(&mut e, &[3]);
        e.set(this, ScriptLocals::m_pMasterScript, Ptr::new(master.addr()));
        set_actions(&mut e, this, &[(0, 0)]);
        e.register_double(GET_NUMERIC_ID_FROM_DOUBLE, |e, a| {
            e.mem.set_u32(a[0], e.mem.u32(a[1]));
            e.mem.set_u32(a[1] - 8, 7);
            Ret::default()
        });
        let headers = Rc::new(RefCell::new(Vec::new()));
        let captured = headers.clone();
        e.register_double(BUFFER_SAVE_BYTES, move |e, a| {
            if a[2] == 4 {
                captured.borrow_mut().push(e.mem.u32(a[1]));
            }
            Ret::default()
        });
        e.call(0x005a9db0, &args![this, buffer]);
        assert_eq!(*headers.borrow(), [0x80000007]);
        assert_eq!(e.mem.u32(vars[0]), 7);
    }

    #[test]
    fn save_game_reloads_effect_flag_and_pointer_after_flag_write() {
        for (has_effect, new_flag) in [(true, 1u8), (true, 0), (false, 1)] {
            let mut e = script_engine();
            let buffer = install_save_buffer(&mut e);
            let this = locals(&mut e);
            set_actions(&mut e, this, &[(0, 0)]);
            let original = e.mem.alloc(8);
            let replacement = e.mem.alloc(8);
            e.set(
                this,
                ScriptLocals::m_pScriptEffectData,
                Ptr::new(if has_effect { original } else { 0 }),
            );
            let first_flag = Rc::new(Cell::new(true));
            let payloads = Rc::new(RefCell::new(Vec::new()));
            let captured = payloads.clone();
            e.register_double(BUFFER_SAVE_BYTES, move |e, a| {
                if a[2] == 1 && first_flag.replace(false) {
                    assert_eq!(e.mem.u8(a[1]), has_effect as u8);
                    e.mem.set_u8(a[1], new_flag);
                    e.set(
                        this,
                        ScriptLocals::m_pScriptEffectData,
                        Ptr::new(replacement),
                    );
                } else if a[2] == 8 {
                    captured.borrow_mut().push(a[1]);
                }
                Ret::default()
            });
            e.call(0x005a9db0, &args![this, buffer]);
            let expected = if new_flag != 0 {
                vec![replacement]
            } else {
                vec![]
            };
            assert_eq!(*payloads.borrow(), expected);
        }
    }

    /// Doubles for the `BGSLoadGameBuffer`: the sized value count is
    /// `count`, bytes come from `data` in order, the version slot gives
    /// `version` and the second slot `0x5ec0`.
    fn install_load_buffer(e: &mut Engine, count: u32, data: Vec<u8>, version: u32) -> Ptr {
        e.register_double(BUFFER_LOAD_SIZED_VALUE, move |_, _| ret(count));
        let position = Rc::new(Cell::new(0usize));
        e.register_double(BUFFER_LOAD_BYTES, move |e, a| {
            let start = position.get();
            let size = a[2] as usize;
            e.mem.write(a[1], &data[start..start + size]);
            position.set(start + size);
            Ret::default()
        });
        e.register(BUFFER_LOAD_FORM_ID_OV2, |e, a| {
            e.mem.set_u32(a[1], 0xf0);
            Ret::default()
        });
        let first = fake_code(e, move |_, _| ret(version));
        let second = fake_code(e, |_, _| ret(0x5ec0));
        Ptr::new(fake_object(e, &[(0, first), (8, second)]))
    }

    #[test]
    fn load_game_reads_variables_effect_data_and_the_action_flag() {
        let mut e = script_engine();
        // Use this unit's real reference-ID bit copy; double only memcpy.
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        let this = locals(&mut e);
        let vars = set_variables(&mut e, this, &[(1, 0.0), (3, 0.0)]);
        let actions = set_actions(&mut e, this, &[(0, 0)]);
        // Two variables: id 1 with a plain value 2.5, id 3 with the top bit
        // (a form reference); an effect data flag with 8 bytes; the action
        // flag.
        let mut data = Vec::new();
        data.extend_from_slice(&1u32.to_le_bytes());
        data.extend_from_slice(&2.5f64.to_le_bytes());
        data.extend_from_slice(&(3u32 | 0x8000_0000).to_le_bytes());
        data.push(1);
        data.extend_from_slice(&[9, 9, 9, 9, 8, 8, 8, 8]);
        data.push(1);
        let buffer = install_load_buffer(&mut e, 2, data, 0x20);
        e.call(0x005a_9f20, &args![this, buffer]);
        assert_eq!(e.mem.f64(vars[0] + 8), 2.5);
        // The form id the buffer gave (0xf0) is put into the double.
        assert_eq!(e.mem.f64(vars[1] + 8).to_bits(), 0xf0);
        let effect = e.get(this, ScriptLocals::m_pScriptEffectData);
        assert!(!effect.is_null());
        assert_eq!(e.mem.u32(effect.addr()), 0x0909_0909);
        // An action list exists: flags are cleared, then the any-form entry
        // gets the saved flag.
        assert_eq!(e.mem.u32(actions[0] + 4), 0x1000);
    }

    #[test]
    fn load_game_builds_the_action_list_through_the_run_manager() {
        let mut e = script_engine();
        let this = locals(&mut e);
        let master = e.mem.alloc(0x60);
        e.set(this, ScriptLocals::m_pMasterScript, Ptr::new(master));
        // An old buffer version has no action flag byte.
        let buffer = install_load_buffer(&mut e, 0, vec![0], 0x14);
        e.register(SCRIPT_RUN_MANAGER_INSTANCE, |_, _| ret(0x6a6a));
        e.register(SCRIPT_RUN_MANAGER_INIT_ACTION_LIST, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005a_9f20, &args![this, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_RUN_MANAGER_INIT_ACTION_LIST),
            vec![vec![0x6a6a, master, 0x5ec0, this.addr()]]
        );
        // Only the effect flag was read: one byte.
        assert_eq!(calls_to(&log, BUFFER_LOAD_BYTES).len(), 1);
        assert!(e.get(this, ScriptLocals::m_pScriptEffectData).is_null());
    }

    // ---- Script ------------------------------------------------------------------

    fn script_blank_engine() -> Engine {
        let mut e = script_engine();
        e.register(TES_FORM_CONSTRUCT, |_, _| Ret::default());
        e.register(TES_FORM_DESTRUCT, |_, _| Ret::default());
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        e.register(SET_FORM_TYPE, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret::default()
        });
        e
    }

    #[test]
    fn script_constructor_sets_the_vtable_and_clears_the_fields() {
        let mut e = script_blank_engine();
        let script = e.new_object::<Script>();
        // Fill the fields with junk the constructor must reset.
        for off in (0x18..0x54).step_by(4) {
            e.mem.set_u32(script.addr() + off, 0xdddd_dddd);
        }
        let tls = e.tls();
        e.mem.set_u8(tls + TLS_SYSTEM_OUTPUT, 1);
        e.call_log = Some(vec![]);
        let result = e.call(0x005a_a0f0, &args![script]).ptr::<Script>();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, script);
        assert_eq!(e.mem.u32(script.addr()), SCRIPT_VTABLE);
        // The form is constructed first, then the two embedded lists.
        assert_eq!(log[1].0, TES_FORM_CONSTRUCT);
        assert_eq!(
            calls_to(&log, LIST_CONSTRUCT),
            vec![vec![script.addr() + 0x44], vec![script.addr() + 0x4c]]
        );
        assert_eq!(e.mem.bytes(script.addr() + 0x18, 0x14), vec![0; 0x14]);
        assert_eq!(e.get(script, Script::m_text), Ptr::NULL);
        assert_eq!(e.get(script, Script::m_data), Ptr::NULL);
        assert_eq!(e.get(script, Script::pOwnerQuest), Ptr::NULL);
        assert_eq!(e.get(script, Script::fProfilerTimer), 0.0);
        assert_eq!(e.get(script, Script::fQuestScriptDelay), 0.0);
        assert_eq!(e.get(script, Script::fQuestScriptGetSecondsBuffer), 0.0);
        assert_eq!(e.mem.u8(tls + TLS_SYSTEM_OUTPUT), 0);
        assert_eq!(e.mem.u8(script.addr() + 4), 0x11);
    }

    #[test]
    fn script_field_initializer_zeroes_the_header_and_timers() {
        let mut e = script_blank_engine();
        let script = e.new_object::<Script>();
        for off in (0x18..0x54).step_by(4) {
            e.mem.set_u32(script.addr() + off, 0x4040_4040);
        }
        e.call_log = Some(vec![]);
        e.call(0x005a_a220, &args![script]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MEMSET),
            vec![vec![script.addr() + 0x18, 0, 0x14]]
        );
        assert_eq!(
            calls_to(&log, SET_FORM_TYPE),
            vec![vec![script.addr(), 0x11]]
        );
        assert_eq!(e.mem.u32(script.addr() + 0x2c), 0);
        assert_eq!(e.mem.u32(script.addr() + 0x40), 0);
        assert_eq!(e.mem.u32(script.addr() + 0x34), 0);
        // The lists are left alone.
        assert_eq!(e.mem.u32(script.addr() + 0x44), 0x4040_4040);
    }

    #[test]
    fn script_destructor_body_releases_everything() {
        let mut e = script_blank_engine();
        let script = e.new_object::<Script>();
        let text = e.mem.alloc(8);
        let data = e.mem.alloc(8);
        e.set(script, Script::m_text, Ptr::new(text));
        e.set(script, Script::m_data, Ptr::new(data));
        e.call_log = Some(vec![]);
        e.call(0x005a_a1a0, &args![script]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(script.addr()), SCRIPT_VTABLE);
        // Data first, then the text.
        assert_eq!(deleted(&log), vec![data, text]);
        // The variable list is destroyed first, then the object list, then
        // the form base.
        assert_eq!(
            calls_to(&log, LIST_DESTRUCT),
            vec![vec![script.addr() + 0x4c], vec![script.addr() + 0x44]]
        );
        assert_eq!(log.last().unwrap().0, TES_FORM_DESTRUCT);
    }

    #[test]
    fn script_deleting_destructor_frees_with_the_flag() {
        let mut e = script_blank_engine();
        let script = e.new_object::<Script>();
        e.call_log = Some(vec![]);
        let back = e.call(0x005a_a170, &args![script, 0u32]).ptr::<Script>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, script);
        assert!(!deleted(&log).contains(&script.addr()));
        e.call_log = Some(vec![]);
        e.call(0x005a_a170, &args![script, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(deleted(&log).last(), Some(&script.addr()));
    }

    #[test]
    fn script_releases_data_and_text_then_the_lists() {
        let mut e = script_blank_engine();
        let script = e.new_object::<Script>();
        let text = e.mem.alloc(8);
        let data = e.mem.alloc(8);
        e.set(script, Script::m_text, Ptr::new(text));
        e.set(script, Script::m_data, Ptr::new(data));
        let tls = e.tls();
        e.mem
            .set_u32(tls + TLS_LAST_REF_SEARCH_SCRIPT, script.addr());
        e.call(0x005a_a2a0, &args![script]);
        assert_eq!(e.mem.u32(tls + TLS_LAST_REF_SEARCH_SCRIPT), 0);
        // The text and data blocks were freed (a second free would panic).
        assert_eq!(e.mem.block_size(text), None);
        assert_eq!(e.mem.block_size(data), None);
    }

    #[test]
    fn script_variable_list_is_emptied_item_by_item() {
        let mut e = script_blank_engine();
        let script = e.new_object::<Script>();
        let items: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x20)).collect();
        fill_list(&mut e, script.addr() + 0x4c, &items);
        e.register(SCRIPT_VARIABLE_DELETE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005a_a2e0, &args![script]);
        let log = e.call_log.take().unwrap();
        let removed = calls_to(&log, SCRIPT_VARIABLE_DELETE);
        assert_eq!(
            removed,
            items.iter().map(|i| vec![*i, 1]).collect::<Vec<_>>()
        );
        assert!(list_items(&e, script.addr() + 0x4c).is_empty());
    }

    #[test]
    fn script_object_list_is_emptied_and_the_search_cache_forgotten() {
        let mut e = script_blank_engine();
        let script = e.new_object::<Script>();
        let items: Vec<u32> = (0..2).map(|_| e.mem.alloc(0x10)).collect();
        fill_list(&mut e, script.addr() + 0x44, &items);
        e.register(SCRIPT_REFERENCED_OBJECT_DESTRUCT, |_, _| Ret::default());
        let tls = e.tls();
        e.mem
            .set_u32(tls + TLS_LAST_REF_SEARCH_SCRIPT, script.addr());
        e.call_log = Some(vec![]);
        e.call(0x005a_a350, &args![script]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(tls + TLS_LAST_REF_SEARCH_SCRIPT), 0);
        assert_eq!(deleted(&log), items);
        assert!(list_items(&e, script.addr() + 0x44).is_empty());
        // Another script in the cache is left alone.
        e.mem.set_u32(tls + TLS_LAST_REF_SEARCH_SCRIPT, 0x1234);
        e.call(0x005a_a350, &args![script]);
        assert_eq!(e.mem.u32(tls + TLS_LAST_REF_SEARCH_SCRIPT), 0x1234);
    }

    #[test]
    fn referenced_object_destructor_frees_only_with_the_flag() {
        let mut e = script_engine();
        e.register(SCRIPT_REFERENCED_OBJECT_DESTRUCT, |_, _| Ret::default());
        let object = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let back = e.call(0x005a_a3f0, &args![object, 0u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, object);
        assert!(deleted(&log).is_empty());
        e.call_log = Some(vec![]);
        e.call(0x005a_a3f0, &args![object, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(deleted(&log), vec![object]);
    }

    fn copy_script_engine(text: u32, quest_flag: u32) -> Engine {
        let mut e = script_blank_engine();
        e.register(SCRIPT_HEADER_OF, |_, a| ret(a[0] + 0x18));
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(SCRIPT_COMPILE_DATA_OF, |_, _| ret(0xda7a));
        e.register(SCRIPT_SET_COMPILE_DATA, |_, _| Ret::default());
        e.register(SCRIPT_COPY_REF_OBJECTS, |_, _| Ret::default());
        e.register(SCRIPT_COPY_VARIABLES, |_, _| Ret::default());
        e.register_double(SCRIPT_TEXT_OF, move |_, _| ret(text));
        e.register(SCRIPT_SET_TEXT, |_, _| Ret::default());
        e.register_double(FORM_FLAG_BIT_3, move |_, _| ret(quest_flag));
        e.register(SCRIPT_SET_QUEST_SCRIPT_FLAG, |_, _| Ret::default());
        e
    }

    #[test]
    fn copy_result_script_copies_header_data_lists_and_text() {
        let mut e = copy_script_engine(0x7e47, 1);
        let this = e.new_object::<Script>();
        let other = e.new_object::<Script>();
        // The other script's header (+0x18) has a data size at +8.
        for i in 0..5u32 {
            e.mem.set_u32(other.addr() + 0x18 + 4 * i, 0x10 + i);
        }
        e.call_log = Some(vec![]);
        e.call(0x005a_a420, &args![this, other]);
        let log = e.call_log.take().unwrap();
        let (t, o) = (this.addr(), other.addr());
        // The header is copied except the variable count.
        assert_eq!(e.mem.u32(t + 0x18), 0);
        assert_eq!(e.mem.u32(t + 0x1c), 0x11);
        assert_eq!(
            calls_to(&log, SCRIPT_SET_COMPILE_DATA),
            vec![vec![t, 0x12, 0xda7a]]
        );
        assert_eq!(
            calls_to(&log, SCRIPT_COPY_REF_OBJECTS),
            vec![vec![o + 0x44, t + 0x44, t]]
        );
        assert_eq!(
            calls_to(&log, SCRIPT_COPY_VARIABLES),
            vec![vec![o + 0x4c, t + 0x4c, t]]
        );
        assert_eq!(calls_to(&log, SCRIPT_SET_TEXT), vec![vec![t, 0x7e47]]);
        assert_eq!(
            calls_to(&log, SCRIPT_SET_QUEST_SCRIPT_FLAG),
            vec![vec![t, 1]]
        );
    }

    #[test]
    fn copy_result_script_without_text_sets_null_text_and_ignores_null_source() {
        let mut e = copy_script_engine(0, 0);
        let this = e.new_object::<Script>();
        let other = e.new_object::<Script>();
        e.call_log = Some(vec![]);
        e.call(0x005a_a420, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SCRIPT_SET_TEXT), vec![vec![this.addr(), 0]]);
        assert_eq!(
            calls_to(&log, SCRIPT_SET_QUEST_SCRIPT_FLAG),
            vec![vec![this.addr(), 0]]
        );
        // A null source does nothing at all (only the call itself is logged).
        e.call_log = Some(vec![]);
        e.call(0x005a_a420, &args![this, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    // ---- pending references ------------------------------------------------------

    #[test]
    fn pending_checks_look_in_their_own_list_under_the_lock() {
        let mut e = script_engine();
        fill_list(&mut e, REFERENCES_TO_ENABLE, &[0x11]);
        fill_list(&mut e, REFERENCES_TO_DISABLE, &[0x22]);
        fill_list(&mut e, REFERENCES_TO_FADE_DISABLE, &[0x33]);
        for (addr, yes, no) in [
            (0x005a_a680u32, 0x11u32, 0x22u32),
            (0x005a_a630, 0x22, 0x33),
            (0x005a_a6d0, 0x33, 0x11),
        ] {
            e.call_log = Some(vec![]);
            assert!(e.call(addr, &args![yes]).bool());
            let log = e.call_log.take().unwrap();
            assert_eq!(log[1].0, CRITICAL_SECTION_ENTER);
            assert_eq!(&log[1].1, &[SCRIPT_REF_LIST_CRITICAL_SECTION, 0]);
            assert_eq!(log.last().unwrap().0, CRITICAL_SECTION_LEAVE);
            assert!(!e.call(addr, &args![no]).bool());
            // A null reference is not looked up.
            e.call_log = Some(vec![]);
            assert!(!e.call(addr, &args![0u32]).bool());
            let log = e.call_log.take().unwrap();
            assert!(calls_to(&log, LIST_CONTAINS).is_empty());
        }
    }

    #[test]
    fn add_pending_disabled_moves_the_reference_to_the_disable_list() {
        let mut e = script_engine();
        e.call_log = Some(vec![]);
        e.call(0x005a_a500, &args![0x4444u32, 0u32]);
        let log = e.call_log.take().unwrap();
        let removed = calls_to(&log, LIST_REMOVE);
        assert_eq!(removed.len(), 1);
        assert_eq!(removed[0][0], REFERENCES_TO_ENABLE);
        let added = calls_to(&log, LIST_ADD_HEAD);
        assert_eq!(added.len(), 1);
        assert_eq!(added[0][0], REFERENCES_TO_DISABLE);
        assert_eq!(list_items(&e, REFERENCES_TO_DISABLE), vec![0x4444]);
        // Already pending: not added again.
        e.call_log = Some(vec![]);
        e.call(0x005a_a500, &args![0x4444u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, LIST_ADD_HEAD).is_empty());
    }

    #[test]
    fn add_pending_disabled_with_fade_uses_the_fade_list() {
        let mut e = script_engine();
        e.call(0x005a_a500, &args![0x5555u32, 1u32]);
        assert_eq!(list_items(&e, REFERENCES_TO_FADE_DISABLE), vec![0x5555]);
        assert!(list_items(&e, REFERENCES_TO_DISABLE).is_empty());
        // A null reference only takes the lock: the call, enter, leave.
        e.call_log = Some(vec![]);
        e.call(0x005a_a500, &args![0u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, LIST_REMOVE).is_empty());
        assert_eq!(log.len(), 3);
    }

    #[test]
    fn add_pending_enabled_moves_the_reference_to_the_enable_list() {
        let mut e = script_engine();
        e.call_log = Some(vec![]);
        e.call(0x005a_a580, &args![0x6666u32]);
        let log = e.call_log.take().unwrap();
        let removed = calls_to(&log, LIST_REMOVE);
        assert_eq!(removed[0][0], REFERENCES_TO_DISABLE);
        assert_eq!(list_items(&e, REFERENCES_TO_ENABLE), vec![0x6666]);
        // Not added twice.
        e.call_log = Some(vec![]);
        e.call(0x005a_a580, &args![0x6666u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, LIST_ADD_HEAD).is_empty());
        // Null: lock only.
        e.call_log = Some(vec![]);
        e.call(0x005a_a580, &args![0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 3);
    }

    #[test]
    fn remove_delayed_reference_leaves_all_four_lists() {
        let mut e = script_engine();
        e.call_log = Some(vec![]);
        e.call(0x005a_a5d0, &args![0x7777u32]);
        let log = e.call_log.take().unwrap();
        let lists: Vec<u32> = calls_to(&log, LIST_REMOVE).iter().map(|a| a[0]).collect();
        assert_eq!(
            lists,
            vec![
                REFERENCES_TO_ENABLE,
                REFERENCES_TO_DISABLE,
                REFERENCES_TO_FADE_DISABLE,
                REFERENCES_TO_DELETE
            ]
        );
        // Null does nothing, not even lock.
        e.call_log = Some(vec![]);
        e.call(0x005a_a5d0, &args![0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn delayed_actions_enable_disable_and_delete() {
        let mut e = script_engine();
        let order = Rc::new(RefCell::new(Vec::new()));
        let seen = order.clone();
        e.register_double(REFERENCE_ENABLE, move |_, a| {
            seen.borrow_mut().push(("enable", a[0]));
            Ret::default()
        });
        let seen = order.clone();
        e.register_double(REFERENCE_DISABLE, move |_, a| {
            seen.borrow_mut().push(("disable", a[0]));
            Ret::default()
        });
        let seen = order.clone();
        let destroy = fake_code(&mut e, move |_, a| {
            seen.borrow_mut().push(("delete", a[0]));
            assert_eq!(a[1], 1);
            Ret::default()
        });
        let to_delete = fake_object(&mut e, &[(0x10, destroy)]);
        fill_list(&mut e, REFERENCES_TO_ENABLE, &[0x101, 0x102]);
        fill_list(&mut e, REFERENCES_TO_DISABLE, &[0x201]);
        fill_list(&mut e, REFERENCES_TO_DELETE, &[to_delete]);
        e.call(0x005a_a720, &args![]);
        assert_eq!(
            *order.borrow(),
            vec![
                ("enable", 0x101),
                ("enable", 0x102),
                ("disable", 0x201),
                ("delete", to_delete)
            ]
        );
        for list in [
            REFERENCES_TO_ENABLE,
            REFERENCES_TO_DISABLE,
            REFERENCES_TO_DELETE,
        ] {
            assert!(list_items(&e, list).is_empty());
        }
    }

    #[test]
    fn delayed_actions_keep_fading_references_that_are_still_visible() {
        let mut e = script_engine();
        let disabled = Rc::new(RefCell::new(Vec::new()));
        let seen = disabled.clone();
        e.register_double(REFERENCE_DISABLE, move |_, a| {
            seen.borrow_mut().push(a[0]);
            Ret::default()
        });
        // Reference A: its +0x1D0 object has no target. B: the target is
        // accepted. C: no object at all.
        let no_target = fake_code(&mut e, |_, _| ret(0));
        let object_without_target = fake_object(&mut e, &[(0x10, no_target)]);
        let give_object = fake_code(&mut e, move |_, _| ret(object_without_target));
        let a = fake_object(&mut e, &[(0x1d0, give_object)]);
        let visible_target = e.mem.alloc(0x100);
        let give_target = fake_code(&mut e, move |_, _| ret(visible_target));
        let object_with_target = fake_object(&mut e, &[(0x10, give_target)]);
        let give_object_b = fake_code(&mut e, move |_, _| ret(object_with_target));
        let b = fake_object(&mut e, &[(0x1d0, give_object_b)]);
        let give_nothing = fake_code(&mut e, |_, _| ret(0));
        let c = fake_object(&mut e, &[(0x1d0, give_nothing)]);
        // The target is accepted: flag 0x8000 set.
        e.register(FLAGS_TEST, |_, _| ret(1));
        fill_list(&mut e, REFERENCES_TO_FADE_DISABLE, &[a, b, c]);
        e.call(0x005a_a720, &args![]);
        assert_eq!(*disabled.borrow(), vec![a, c]);
    }

    #[test]
    fn fade_check_follows_flag_fade_byte_and_the_float() {
        let mut e = script_engine();
        let object = e.mem.alloc(0xc0);
        // Flag 0x8000 set: accepted whatever the rest.
        e.register(FLAGS_TEST, |_, _| ret(1));
        assert!(e.call(0x005a_a8c0, &args![object]).bool());
        // Flag clear and the fade byte clear: accepted.
        e.register(FLAGS_TEST, |_, _| ret(0));
        assert!(e.call(0x005a_a8c0, &args![object]).bool());
        // Flag clear, fade byte set: accepted only if the float is positive.
        e.set_global(FADE_ENABLED_BYTE, 1u8);
        e.mem.set_f32(object + 0xb8, 0.5);
        assert!(e.call(0x005a_a8c0, &args![object]).bool());
        e.mem.set_f32(object + 0xb8, 0.0);
        assert!(!e.call(0x005a_a8c0, &args![object]).bool());
        e.mem.set_f32(object + 0xb8, -1.0);
        assert!(!e.call(0x005a_a8c0, &args![object]).bool());
        e.mem.set_f32(object + 0xb8, f32::NAN);
        assert!(!e.call(0x005a_a8c0, &args![object]).bool());
    }

    #[test]
    fn flag_test_asks_for_bit_0x8000() {
        let mut e = script_engine();
        e.register(FLAGS_TEST, |_, a| ret((a[1] == 0x8000) as u32));
        let object = e.mem.alloc(0x40);
        assert!(e.call(0x005a_a910, &args![object]).bool());
        e.register(FLAGS_TEST, |_, _| ret(0));
        assert!(!e.call(0x005a_a910, &args![object]).bool());
    }

    #[test]
    fn pending_references_are_saved_list_by_list() {
        let mut e = script_engine();
        let buffer = install_save_buffer(&mut e);
        fill_list(&mut e, REFERENCES_TO_ENABLE, &[0x11, 0x12]);
        fill_list(&mut e, REFERENCES_TO_DELETE, &[0x31]);
        fill_list(&mut e, REFERENCES_TO_FADE_DISABLE, &[0x41]);
        e.call_log = Some(vec![]);
        e.call(0x005a_a930, &args![buffer]);
        let log = e.call_log.take().unwrap();
        let b = buffer.addr();
        let saved: Vec<(u32, Vec<u32>)> = log
            .iter()
            .filter(|(a, _)| {
                [
                    BUFFER_START_SIZED_VALUE,
                    BUFFER_SAVE_FORM_ID_OV2,
                    BUFFER_SAVE_SIZED_VALUE,
                ]
                .contains(a)
            })
            .cloned()
            .collect();
        assert_eq!(
            saved,
            vec![
                (BUFFER_START_SIZED_VALUE, vec![b]),
                (BUFFER_SAVE_FORM_ID_OV2, vec![b, 0x11, 0]),
                (BUFFER_SAVE_FORM_ID_OV2, vec![b, 0x12, 0]),
                (BUFFER_SAVE_SIZED_VALUE, vec![b, 2, 0x7000]),
                // The disable list is empty: a count of 0.
                (BUFFER_START_SIZED_VALUE, vec![b]),
                (BUFFER_SAVE_SIZED_VALUE, vec![b, 0, 0x7000]),
                (BUFFER_START_SIZED_VALUE, vec![b]),
                (BUFFER_SAVE_FORM_ID_OV2, vec![b, 0x31, 0]),
                (BUFFER_SAVE_SIZED_VALUE, vec![b, 1, 0x7000]),
                (BUFFER_START_SIZED_VALUE, vec![b]),
                (BUFFER_SAVE_FORM_ID_OV2, vec![b, 0x41, 0]),
                (BUFFER_SAVE_SIZED_VALUE, vec![b, 1, 0x7000]),
            ]
        );
        assert_eq!(log[1].0, CRITICAL_SECTION_ENTER);
        assert_eq!(log.last().unwrap().0, CRITICAL_SECTION_LEAVE);
    }

    // ============================================================================
    // Second half of the unit (005aaaf0 to 005ae2e0)
    // ============================================================================

    type Calls = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    /// An engine for the second half: the doubles of [`script_engine`] plus
    /// the C runtime and allocator functions the code reaches, and the pages
    /// of the globals it reads.
    fn tail_engine() -> Engine {
        let mut e = script_engine();
        e.map(0x0101_1000, 0x1000);
        e.map(0x0118_c000, 0x1000);
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        e.register(STRLEN, |e, a| ret(e.mem.cstr(a[0]).len() as u32));
        e.register(MEMORY_MANAGER_GET, |_, _| ret(0x011f_6238));
        e.register(MEMORY_MANAGER_ALLOCATE, |e, a| ret(e.mem.alloc(a[1])));
        e.register(SCRIPT_HEADER_OF, |_, a| ret(a[0] + 0x18));
        e.register(FORM_ID_OF, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(SCRIPT_BUFFER_VARIABLE_LIST, |_, a| ret(a[0] + 0x3c));
        e.register(STRING_LENGTH, |e, a| ret(e.mem.u16(a[0] + 4) as u32));
        e.register(STRING_DATA, |e, a| ret(e.mem.u32(a[0])));
        e
    }

    /// A script whose virtual slots record their calls (slot, words).
    fn recording_script(e: &mut Engine, slots: &[u32]) -> (Ptr<Script>, Calls) {
        let calls: Calls = Rc::new(RefCell::new(Vec::new()));
        let table = e.mem.alloc(0x200);
        for &slot in slots {
            let sink = calls.clone();
            let target = fake_code(e, move |_, a| {
                sink.borrow_mut().push((slot, a.to_vec()));
                ret(0x7000 + slot)
            });
            e.mem.set_u32(table + slot, target);
        }
        let script = e.new_object::<Script>();
        e.mem.set_u32(script.addr(), table);
        (script, calls)
    }

    /// The header of a script.
    fn script_header(script: Ptr<Script>) -> Ptr<ScriptHeader> {
        script.at(Script::m_header)
    }

    /// A C string in game memory.
    fn c_string(e: &mut Engine, text: &str) -> u32 {
        let block = e.mem.alloc(text.len() as u32 + 1);
        e.mem.set_cstr(block, text.as_bytes());
        block
    }

    /// A `BSStringT` (pointer, length) for `text` at a fresh block.
    fn bs_string(e: &mut Engine, at: u32, text: &str) {
        let data = c_string(e, text);
        e.mem.set_u32(at, data);
        e.mem.set_u16(at + 4, text.len() as u16);
    }

    /// A `ScriptVariable` block (0x20 bytes) with an id and a name.
    fn script_variable(e: &mut Engine, id: u32, name: &str) -> u32 {
        let block = e.mem.alloc(0x20);
        e.mem.set_u32(block, id);
        bs_string(e, block + 0x18, name);
        block
    }

    // ---- pending references and small helpers --------------------------------

    #[test]
    fn load_pending_references_reads_the_four_lists() {
        let mut e = tail_engine();
        e.register(LIST_CLEAR, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        e.register(REFERENCE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(LOOKUP_FORM_BY_ID, |_, a| ret(a[0] + 0x100));
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        let counts = Rc::new(RefCell::new(vec![1u32, 0, 2, 1]));
        let ids = Rc::new(RefCell::new(vec![0x10u32, 0x20, 0x21, 0x30]));
        let queue = counts.clone();
        e.register_double(BUFFER_LOAD_SIZED_VALUE, move |_, _| {
            ret(queue.borrow_mut().remove(0))
        });
        let queue = ids.clone();
        e.register_double(BUFFER_LOAD_FORM_ID, move |_, _| {
            ret(queue.borrow_mut().remove(0))
        });
        // Something already pending is dropped first.
        e.mem.set_u32(REFERENCES_TO_ENABLE, 0x99);
        e.call_log = Some(vec![]);
        e.call(0x005a_aaf0, &args![0xb0f0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LIST_CLEAR).len(), 4);
        assert_eq!(calls_to(&log, BUFFER_LOAD_SIZED_VALUE).len(), 4);
        // One reference constructed (in a 0x68-byte block) per entry.
        assert_eq!(calls_to(&log, REFERENCE_CONSTRUCT).len(), 4);
        assert_eq!(list_items(&e, REFERENCES_TO_ENABLE), vec![0x110]);
        assert_eq!(list_items(&e, REFERENCES_TO_DISABLE), Vec::<u32>::new());
        assert_eq!(list_items(&e, REFERENCES_TO_DELETE), vec![0x121, 0x120]);
        assert_eq!(list_items(&e, REFERENCES_TO_FADE_DISABLE), vec![0x130]);
        assert_eq!(calls_to(&log, CRITICAL_SECTION_ENTER).len(), 1);
        assert_eq!(calls_to(&log, CRITICAL_SECTION_LEAVE).len(), 1);
    }

    #[test]
    fn clear_pending_references_empties_the_four_lists_under_the_lock() {
        let mut e = tail_engine();
        e.register(LIST_CLEAR, |e, a| {
            e.mem.set_u32(a[0], 0);
            e.mem.set_u32(a[0] + 4, 0);
            Ret::default()
        });
        for list in [
            REFERENCES_TO_ENABLE,
            REFERENCES_TO_DISABLE,
            REFERENCES_TO_DELETE,
            REFERENCES_TO_FADE_DISABLE,
        ] {
            e.mem.set_u32(list, 0x55);
        }
        e.call_log = Some(vec![]);
        e.call(0x005a_ae20, &args![]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LIST_CLEAR).len(), 4);
        assert_eq!(calls_to(&log, CRITICAL_SECTION_ENTER).len(), 1);
        assert_eq!(e.mem.u32(REFERENCES_TO_DELETE), 0);
        assert_eq!(e.mem.u32(REFERENCES_TO_FADE_DISABLE), 0);
    }

    #[test]
    fn set_compile_data_replaces_the_data_with_a_copy() {
        let mut e = tail_engine();
        let this = e.new_object::<Script>();
        let old = e.mem.alloc(8);
        e.set(this, Script::m_data, Ptr::new(old));
        let source = e.mem.alloc(8);
        e.mem.write(source, &[1, 2, 3, 4, 5]);
        e.call_log = Some(vec![]);
        e.call(0x005a_ae70, &args![this, 5u32, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(deleted(&log), vec![old]);
        let data = e.get(this, Script::m_data).addr();
        assert_ne!(data, 0);
        assert_eq!(e.mem.bytes(data, 5), vec![1, 2, 3, 4, 5]);
        assert_eq!(e.get(script_header(this), ScriptHeader::dataSize), 5);
        // Size 0: the data is dropped and nothing is allocated.
        e.call_log = Some(vec![]);
        e.call(0x005a_ae70, &args![this, 0u32, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(deleted(&log), vec![data]);
        assert!(e.get(this, Script::m_data).is_null());
        assert_eq!(e.get(script_header(this), ScriptHeader::dataSize), 0);
        assert!(calls_to(&log, MEMORY_MANAGER_ALLOCATE).is_empty());
    }

    #[test]
    fn zeroed_allocation_multiplies_and_clears() {
        let mut e = tail_engine();
        e.call_log = Some(vec![]);
        let block = e.call(0x005a_aee0, &args![3u32, 4u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MEMORY_MANAGER_ALLOCATE),
            vec![vec![0x011f_6238, 12]]
        );
        assert_eq!(calls_to(&log, MEMSET), vec![vec![block, 0, 12]]);
        assert_eq!(e.mem.block_size(block), Some(16));
    }

    // ---- Init and comparisons ------------------------------------------------------

    /// A compiler output buffer: name `name`, `data`, header sizes and empty
    /// lists.
    fn compile_buffer(e: &mut Engine, name: &str, data: &[u8], ref_count: u32) -> u32 {
        let buffer = e.mem.alloc(0x58);
        if !name.is_empty() {
            bs_string(e, buffer + 0xc, name);
        }
        let output = e.mem.alloc(data.len() as u32 + 1);
        e.mem.write(output, data);
        e.mem.set_u32(buffer + 0x20, output);
        e.mem.set_u32(buffer + 0x28 + 4, ref_count);
        e.mem.set_u32(buffer + 0x28 + 8, data.len() as u32);
        buffer
    }

    #[test]
    fn script_differs_from_buffer_by_size_count_bytes_and_lists() {
        let mut e = tail_engine();
        e.register(SCRIPT_IS_NAMED_CHECK, |_, _| ret(0));
        e.register(LIST_COUNT, |e, a| ret(list_items(e, a[0]).len() as u32));
        e.register(VARIABLE_DIFFERS, |e, a| {
            ret((e.mem.u32(a[0]) != e.mem.u32(a[1])) as u32)
        });
        let (this, _) = recording_script(&mut e, &[0x130]);
        let data = [1u8, 2, 3];
        let set_data = |e: &mut Engine, bytes: &[u8]| {
            let block = e.mem.alloc(8);
            e.mem.write(block, bytes);
            e.set(this, Script::m_data, Ptr::new(block));
        };
        set_data(&mut e, &data);
        e.set(script_header(this), ScriptHeader::dataSize, 3);
        e.set(script_header(this), ScriptHeader::refObjectCount, 1);
        let buffer = compile_buffer(&mut e, "Name", &data, 1);
        // Same data, no lists: not different.
        assert!(!e.call(0x005a_b040, &args![this, buffer]).bool());
        // A different size, count or byte.
        e.mem.set_u32(buffer + 0x30, 4);
        assert!(e.call(0x005a_b040, &args![this, buffer]).bool());
        e.mem.set_u32(buffer + 0x30, 3);
        e.mem.set_u32(buffer + 0x2c, 2);
        assert!(e.call(0x005a_b040, &args![this, buffer]).bool());
        e.mem.set_u32(buffer + 0x2c, 1);
        let output = e.mem.u32(buffer + 0x20);
        e.mem.set_u8(output + 1, 9);
        assert!(e.call(0x005a_b040, &args![this, buffer]).bool());
        e.mem.set_u8(output + 1, 2);
        assert!(!e.call(0x005a_b040, &args![this, buffer]).bool());
        // Variable lists: a different count, then a different pair.
        let variable_a = variable(&mut e, 1, 0.0);
        let variable_b = variable(&mut e, 2, 0.0);
        fill_list(&mut e, buffer + 0x3c, &[variable_a]);
        assert!(e.call(0x005a_b040, &args![this, buffer]).bool());
        fill_list(&mut e, this.addr() + 0x4c, &[variable_b]);
        assert!(e.call(0x005a_b040, &args![this, buffer]).bool());
        e.mem.set_u32(variable_b, 1);
        assert!(!e.call(0x005a_b040, &args![this, buffer]).bool());
        // Referenced objects: the form word at +8 is compared.
        let object_a = e.mem.alloc(0x10);
        let object_b = e.mem.alloc(0x10);
        e.mem.set_u32(object_a + 8, 0x500);
        e.mem.set_u32(object_b + 8, 0x600);
        fill_list(&mut e, buffer + 0x44, &[object_a]);
        assert!(e.call(0x005a_b040, &args![this, buffer]).bool());
        fill_list(&mut e, this.addr() + 0x44, &[object_b]);
        assert!(e.call(0x005a_b040, &args![this, buffer]).bool());
        e.mem.set_u32(object_b + 8, 0x500);
        assert!(!e.call(0x005a_b040, &args![this, buffer]).bool());
    }

    #[test]
    fn script_differs_from_buffer_when_the_name_does_not_match() {
        let mut e = tail_engine();
        e.register(SCRIPT_IS_NAMED_CHECK, |_, _| ret(1));
        e.register(LIST_COUNT, |e, a| ret(list_items(e, a[0]).len() as u32));
        let table_name = c_string(&mut e, "Other");
        let target = fake_code(&mut e, move |_, _| ret(table_name));
        let table = e.mem.alloc(0x200);
        e.mem.set_u32(table + 0x130, target);
        let this = e.new_object::<Script>();
        e.mem.set_u32(this.addr(), table);
        e.register(STRING_COMPARE, |e, a| {
            ret((e.mem.cstr(a[0]) != e.mem.cstr(a[1])) as u32)
        });
        let buffer = compile_buffer(&mut e, "Name", &[], 0);
        assert!(e.call(0x005a_b040, &args![this, buffer]).bool());
        // The same name (compare is zero) and nothing else different.
        let buffer = compile_buffer(&mut e, "Other", &[], 0);
        assert!(!e.call(0x005a_b040, &args![this, buffer]).bool());
        // A buffer with no name skips the name test.
        let buffer = compile_buffer(&mut e, "", &[], 0);
        assert!(!e.call(0x005a_b040, &args![this, buffer]).bool());
    }

    #[test]
    fn init_with_an_empty_name_only_marks_the_script_deleted() {
        let mut e = tail_engine();
        let (this, calls) = recording_script(&mut e, &[0x88, 0xc4, 0xc8, 0x134]);
        let buffer = compile_buffer(&mut e, "", &[], 0);
        e.call(0x005a_af20, &args![this, buffer]);
        let calls = calls.borrow();
        assert_eq!(
            *calls,
            vec![(0x88, vec![this.addr()]), (0xc4, vec![this.addr(), 1])]
        );
    }

    #[test]
    fn init_takes_the_compiled_output_when_it_differs() {
        let mut e = tail_engine();
        e.register(SCRIPT_IS_NAMED_CHECK, |_, _| ret(0));
        e.register(LIST_COUNT, |e, a| ret(list_items(e, a[0]).len() as u32));
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_COPY, |_, _| Ret::default());
        e.register(SCRIPT_VARIABLE_COPY_CONSTRUCT, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x20);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(SCRIPT_VARIABLE_DELETE, |e, a| {
            e.mem.free(a[0]);
            Ret::default()
        });
        e.register(SCRIPT_REFERENCED_OBJECT_DESTRUCT, |_, _| Ret::default());
        let (this, calls) = recording_script(&mut e, &[0x88, 0xc4, 0xc8, 0x134]);
        let buffer = compile_buffer(&mut e, "Name", &[7, 8, 9], 1);
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 8, 0x500);
        e.mem.set_u32(object + 0xc, 3);
        fill_list(&mut e, buffer + 0x44, &[object]);
        let local = variable(&mut e, 4, 2.5);
        fill_list(&mut e, buffer + 0x3c, &[local]);
        e.call(0x005a_af20, &args![this, buffer]);
        let name_data = e.mem.u32(buffer + 0xc);
        let sequence: Vec<(u32, Vec<u32>)> = calls.borrow().clone();
        assert_eq!(
            sequence,
            vec![
                (0x88, vec![this.addr()]),
                (0x134, vec![this.addr(), name_data]),
                (0xc4, vec![this.addr(), 0]),
                (0xc8, vec![this.addr(), 1]),
            ]
        );
        assert_eq!(e.get(script_header(this), ScriptHeader::dataSize), 3);
        assert_eq!(e.get(script_header(this), ScriptHeader::refObjectCount), 1);
        let data = e.get(this, Script::m_data).addr();
        assert_eq!(e.mem.bytes(data, 3), vec![7, 8, 9]);
        // The lists are copies of the buffer's.
        let references = list_items(&e, this.addr() + 0x44);
        assert_eq!(references.len(), 1);
        assert_ne!(references[0], object);
        assert_eq!(e.mem.u32(references[0] + 8), 0x500);
        assert_eq!(e.mem.u32(references[0] + 0xc), 3);
        let variables = list_items(&e, this.addr() + 0x4c);
        assert_eq!(variables.len(), 1);
        assert_eq!(e.mem.u32(variables[0]), 4);
        assert_eq!(e.mem.f64(variables[0] + 8), 2.5);
    }

    #[test]
    fn init_leaves_an_identical_script_alone() {
        let mut e = tail_engine();
        e.register(SCRIPT_IS_NAMED_CHECK, |_, _| ret(0));
        e.register(LIST_COUNT, |e, a| ret(list_items(e, a[0]).len() as u32));
        let (this, calls) = recording_script(&mut e, &[0x88, 0xc4, 0xc8, 0x134]);
        let block = e.mem.alloc(8);
        e.mem.write(block, &[7, 8, 9]);
        e.set(this, Script::m_data, Ptr::new(block));
        e.set(script_header(this), ScriptHeader::dataSize, 3);
        let buffer = compile_buffer(&mut e, "Name", &[7, 8, 9], 0);
        e.call(0x005a_af20, &args![this, buffer]);
        assert_eq!(*calls.borrow(), vec![(0x88, vec![this.addr()])]);
    }

    /// A script with a header copy and the given text for the comparison.
    fn comparison_engine() -> Engine {
        let mut e = tail_engine();
        e.register(SCRIPT_COMPILE_DATA_OF, |e, a| ret(e.mem.u32(a[0] + 0x30)));
        e.register(TEXT_COMPARE, |e, a| {
            ret((e.mem.cstr(a[0]) != e.mem.cstr(a[1])) as u32)
        });
        e.register(SCRIPT_TEXT_OF, |e, a| ret(e.mem.u32(a[0] + 0x2c)));
        e
    }

    fn comparable_script(e: &mut Engine, data: &[u8], text: &str) -> Ptr<Script> {
        let script = e.new_object::<Script>();
        let block = e.mem.alloc(8);
        e.mem.write(block, data);
        e.set(script, Script::m_data, Ptr::new(block));
        e.set(
            script_header(script),
            ScriptHeader::dataSize,
            data.len() as u32,
        );
        e.set(script_header(script), ScriptHeader::refObjectCount, 1);
        e.set(script_header(script), ScriptHeader::bIsCompiled, true);
        if !text.is_empty() {
            let text = c_string(e, text);
            e.set(script, Script::m_text, Ptr::new(text));
        }
        script
    }

    #[test]
    fn compare_result_scripts_reports_every_difference() {
        let mut e = comparison_engine();
        let this = comparable_script(&mut e, &[1, 2], "run");
        // Null: different.
        assert!(e.call(0x005a_b240, &args![this, 0u32]).bool());
        let other = comparable_script(&mut e, &[1, 2], "run");
        assert!(!e.call(0x005a_b240, &args![this, other]).bool());
        // Size, reference count, compiled flag.
        let bigger = comparable_script(&mut e, &[1, 2, 3], "run");
        assert!(e.call(0x005a_b240, &args![this, bigger]).bool());
        e.set(script_header(other), ScriptHeader::refObjectCount, 2);
        assert!(e.call(0x005a_b240, &args![this, other]).bool());
        e.set(script_header(other), ScriptHeader::refObjectCount, 1);
        e.set(script_header(other), ScriptHeader::bIsCompiled, false);
        assert!(e.call(0x005a_b240, &args![this, other]).bool());
        e.set(script_header(other), ScriptHeader::bIsCompiled, true);
        // A compiled byte.
        let data = e.get(other, Script::m_data).addr();
        e.mem.set_u8(data, 9);
        assert!(e.call(0x005a_b240, &args![this, other]).bool());
        e.mem.set_u8(data, 1);
        // Texts: a different length, a different text, one missing.
        let longer = comparable_script(&mut e, &[1, 2], "runs");
        assert!(e.call(0x005a_b240, &args![this, longer]).bool());
        let changed = comparable_script(&mut e, &[1, 2], "rux");
        assert!(e.call(0x005a_b240, &args![this, changed]).bool());
        let none = comparable_script(&mut e, &[1, 2], "");
        assert!(e.call(0x005a_b240, &args![this, none]).bool());
        assert!(e.call(0x005a_b240, &args![none, this]).bool());
        let also_none = comparable_script(&mut e, &[1, 2], "");
        assert!(!e.call(0x005a_b240, &args![none, also_none]).bool());
    }

    #[test]
    fn compare_result_scripts_compares_referenced_objects() {
        let mut e = comparison_engine();
        let this = comparable_script(&mut e, &[1], "");
        let other = comparable_script(&mut e, &[1], "");
        let own = e.mem.alloc(0x10);
        let theirs = e.mem.alloc(0x10);
        e.mem.set_u32(own + 8, 0x500);
        e.mem.set_u32(theirs + 8, 0x500);
        fill_list(&mut e, this.addr() + 0x44, &[own]);
        fill_list(&mut e, other.addr() + 0x44, &[theirs]);
        assert!(!e.call(0x005a_b240, &args![this, other]).bool());
        e.mem.set_u32(theirs + 8, 0x600);
        assert!(e.call(0x005a_b240, &args![this, other]).bool());
        // A node with no object is skipped.
        e.mem.set_u32(other.addr() + 0x44, 0);
        assert!(!e.call(0x005a_b240, &args![this, other]).bool());
    }

    // ---- InitItem --------------------------------------------------------------------

    /// The doubles `InitItem` needs; the form of id `0x1010` exists (as
    /// `0xf0f0`), nothing else does, and rebasing adds 0x1000 to the id.
    fn init_item_engine(flag_bit_3: u32) -> Engine {
        let mut e = tail_engine();
        e.register_double(FORM_FLAG_BIT_3, move |_, _| ret(flag_bit_3));
        e.register(SCRIPT_INIT_ITEM_CHECK, |_, _| ret(1));
        e.register(FORM_GET_FILE, |_, _| ret(0xf11e));
        e.register(FORM_SET_FILE, |_, _| Ret::default());
        e.register(FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x1000);
            Ret::default()
        });
        e.register(LOOKUP_FORM_BY_ID, |_, a| {
            ret(if a[0] == 0x1010 { 0xf0f0 } else { 0 })
        });
        e.register(EMPTY_METHOD, |_, _| Ret::default());
        e.register(SCRIPT_SET_QUEST_SCRIPT_FLAG, |_, _| Ret::default());
        e.register(FORM_TYPE_OF, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e
    }

    /// A referenced object with a form id and a variable id.
    fn referenced_object(e: &mut Engine, form: u32, variable_id: u32) -> u32 {
        let object = e.mem.alloc(0x10);
        e.mem.set_u32(object + 8, form);
        e.mem.set_u32(object + 0xc, variable_id);
        object
    }

    #[test]
    fn init_item_does_nothing_when_the_flag_is_set_or_there_is_no_source() {
        let mut e = init_item_engine(1);
        let (this, _) = recording_script(&mut e, &[0x130]);
        let source = e.mem.alloc(0x20);
        e.call_log = Some(vec![]);
        e.call(0x005a_b400, &args![this, source]);
        e.call(0x005a_b7d0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SCOPE_GUARD_OPEN).is_empty());
        let mut e = init_item_engine(0);
        let (this, _) = recording_script(&mut e, &[0x130]);
        e.call_log = Some(vec![]);
        e.call(0x005a_b400, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SCOPE_GUARD_OPEN).is_empty());
    }

    #[test]
    fn init_item_resolves_objects_and_drops_the_unresolved() {
        let mut e = init_item_engine(0);
        let (this, _) = recording_script(&mut e, &[0x130]);
        let source = e.mem.alloc(0x20);
        // Three objects: resolved (id 0x10 + 0x1000), variable only, and an
        // unknown form (id 0x30).
        let good = referenced_object(&mut e, 0x10, 0);
        let by_variable = referenced_object(&mut e, 0, 7);
        let missing = referenced_object(&mut e, 0x30, 0);
        fill_list(&mut e, this.addr() + 0x44, &[good, by_variable, missing]);
        e.set(script_header(this), ScriptHeader::refObjectCount, 3);
        e.set(script_header(this), ScriptHeader::dataSize, 5);
        e.mem.set_u32(this.addr() + 0xc, 0x4242);
        e.call_log = Some(vec![]);
        e.call(0x005a_b400, &args![this, source]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SCOPE_GUARD_OPEN).len(), 1);
        assert_eq!(
            calls_to(&log, SCOPE_GUARD_OPEN)[0][1..],
            [0x15, 1, SOURCE_FILE, 0xab5]
        );
        assert_eq!(calls_to(&log, SCOPE_GUARD_CLOSE).len(), 1);
        // The file of the source is taken for the script.
        assert_eq!(
            calls_to(&log, FORM_SET_FILE),
            vec![vec![this.addr(), 0xf11e]]
        );
        // The first object has its form, the unresolved one is gone.
        assert_eq!(e.mem.u32(good + 8), 0xf0f0);
        assert_eq!(list_items(&e, this.addr() + 0x44), vec![good, by_variable]);
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![
                FORMAT_REFERENCED_OBJECT_NOT_FOUND,
                0x1030,
                0x4242,
                0x7130
            ]]
        );
        // Its data size is cleared and the script is marked.
        assert_eq!(e.get(script_header(this), ScriptHeader::dataSize), 0);
        assert_eq!(
            calls_to(&log, SCRIPT_SET_QUEST_SCRIPT_FLAG),
            vec![vec![this.addr(), 1]]
        );
    }

    #[test]
    fn init_item_logs_an_object_with_nothing_and_a_corrupt_count() {
        let mut e = init_item_engine(0);
        let (this, _) = recording_script(&mut e, &[0x130]);
        let source = this.addr();
        // The first object has neither a form nor a variable: it is removed
        // from the head (nothing before it) and logged with its index.
        let empty = referenced_object(&mut e, 0, 0);
        let kept = referenced_object(&mut e, 0, 9);
        fill_list(&mut e, this.addr() + 0x44, &[empty, kept]);
        // The header expects 3 objects but only 2 are seen.
        e.set(script_header(this), ScriptHeader::refObjectCount, 3);
        e.set(script_header(this), ScriptHeader::dataSize, 5);
        e.mem.set_u32(this.addr() + 0xc, 0x4242);
        let data = e.mem.alloc(8);
        e.set(this, Script::m_data, Ptr::new(data));
        e.call_log = Some(vec![]);
        e.call(0x005a_b400, &args![this, source]);
        let log = e.call_log.take().unwrap();
        // The source is the script itself: no file is taken.
        assert!(calls_to(&log, FORM_SET_FILE).is_empty());
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![FORMAT_REFERENCED_OBJECT_INVALID, 0, 0x4242, 0x7130]]
        );
        assert_eq!(list_items(&e, this.addr() + 0x44), vec![kept]);
        // The data size was already cleared by the first message, so the
        // corrupt count is not reported.
        assert_eq!(e.get(script_header(this), ScriptHeader::dataSize), 0);
        // A non-null data pointer runs the empty method.
        assert_eq!(calls_to(&log, EMPTY_METHOD).len(), 1);
    }

    #[test]
    fn init_item_reports_a_corrupt_object_count() {
        let mut e = init_item_engine(0);
        let (this, _) = recording_script(&mut e, &[0x130]);
        let kept = referenced_object(&mut e, 0, 9);
        fill_list(&mut e, this.addr() + 0x44, &[kept]);
        e.set(script_header(this), ScriptHeader::refObjectCount, 3);
        e.set(script_header(this), ScriptHeader::dataSize, 5);
        e.mem.set_u32(this.addr() + 0xc, 0x4242);
        e.call_log = Some(vec![]);
        e.call(0x005a_b400, &args![this, this.addr()]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![FORMAT_REFERENCE_LIST_CORRUPT, 0x4242, 0x7130, 3, 1]]
        );
        assert_eq!(e.get(script_header(this), ScriptHeader::dataSize), 0);
    }

    /// Doubles for the quest delay: the setting holds `setting`, the table
    /// is derived with a divisor of 2.
    fn quest_engine(setting: f32, counter: u32) -> (Engine, Ptr<Script>) {
        let mut e = init_item_engine(0);
        let cell = e.mem.alloc(4);
        e.mem.set_f32(cell, setting);
        e.register_double(QUEST_DELAY_ACCESSOR, move |_, _| ret(cell));
        e.mem.set_f64(QUEST_DELAY_DIVISOR, 2.0);
        e.set_global::<u32>(QUEST_DELAY_COUNTER, counter);
        let (this, _) = recording_script(&mut e, &[0x130]);
        e.set(script_header(this), ScriptHeader::bIsQuestScript, true);
        (e, this)
    }

    #[test]
    fn init_item_spreads_the_quest_delay_over_the_counter_bits() {
        let (mut e, this) = quest_engine(8.0, 0);
        e.call(0x005a_b400, &args![this, this.addr()]);
        // Counter 0: the whole setting, and the halving table is built.
        assert_eq!(e.get(this, Script::fQuestScriptDelay), 8.0);
        assert_eq!(e.mem.f32(QUEST_DELAY_TABLE), 4.0);
        assert_eq!(e.mem.f32(QUEST_DELAY_TABLE + 4 * 7), 8.0 / 256.0);
        assert_eq!(e.global::<u32>(QUEST_DELAY_COUNTER), 1);
        // Counter 5 (binary 101): table entries 0 and 2.
        e.set(this, Script::fQuestScriptDelay, 0.0);
        e.set_global::<u32>(QUEST_DELAY_COUNTER, 5);
        e.call(0x005a_b400, &args![this, this.addr()]);
        assert_eq!(e.get(this, Script::fQuestScriptDelay), 5.0);
        assert_eq!(e.global::<u32>(QUEST_DELAY_COUNTER), 6);
        // A setting of 0 or less leaves the delay and the counter alone.
        let (mut e, this) = quest_engine(0.0, 3);
        e.call(0x005a_b400, &args![this, this.addr()]);
        assert_eq!(e.get(this, Script::fQuestScriptDelay), 0.0);
        assert_eq!(e.global::<u32>(QUEST_DELAY_COUNTER), 3);
    }

    #[test]
    fn init_item_gives_a_quest_with_its_own_delay_no_delay_and_takes_it_as_owner() {
        let (mut e, this) = quest_engine(8.0, 5);
        e.register(QUEST_DELAY_OF, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        e.set(this, Script::fQuestScriptDelay, 9.0);
        let quest = e.mem.alloc(0x20);
        e.mem.set_u8(quest + 4, 0x47);
        e.set(this, Script::pOwnerQuest, Ptr::new(quest));
        e.call(0x005a_b400, &args![this, quest]);
        assert_eq!(e.get(this, Script::fQuestScriptDelay), 0.0);
        assert_eq!(e.get(this, Script::pOwnerQuest).addr(), quest);
        assert_eq!(e.global::<u32>(QUEST_DELAY_COUNTER), 6);
        // An owner whose delay is 0 gets the bit-spread delay instead.
        e.register(QUEST_DELAY_OF, |_, _| Ret::default());
        e.mem.set_f32(QUEST_DELAY_TABLE, 4.0);
        e.set_global::<u32>(QUEST_DELAY_COUNTER, 1);
        e.call(0x005a_b400, &args![this, quest]);
        assert_eq!(e.get(this, Script::fQuestScriptDelay), 4.0);
        // A source that is not a quest (type 0x20) is not taken as owner.
        let other = e.mem.alloc(0x20);
        e.mem.set_u8(other + 4, 0x20);
        e.set(this, Script::pOwnerQuest, Ptr::NULL);
        e.call(0x005a_b400, &args![this, other]);
        assert!(e.get(this, Script::pOwnerQuest).is_null());
    }

    // ---- list copies -----------------------------------------------------------------

    #[test]
    fn referenced_object_copy_constructor_copies_string_form_and_variable() {
        let mut e = tail_engine();
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_COPY, |_, _| Ret::default());
        let source = referenced_object(&mut e, 0x500, 6);
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        let result = e.call(0x005a_b8b0, &args![this, source]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this + 8), 0x500);
        assert_eq!(e.mem.u32(this + 0xc), 6);
        assert_eq!(calls_to(&log, STRING_CONSTRUCT), vec![vec![this]]);
        assert_eq!(calls_to(&log, STRING_COPY), vec![vec![this, source]]);
    }

    #[test]
    fn copying_reference_and_variable_lists_makes_new_blocks() {
        let mut e = tail_engine();
        e.register(STRING_CONSTRUCT, |_, _| Ret::default());
        e.register(STRING_COPY, |_, _| Ret::default());
        e.register(SCRIPT_VARIABLE_COPY_CONSTRUCT, |e, a| {
            let bytes = e.mem.bytes(a[1], 0x18);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        let first = referenced_object(&mut e, 0x500, 1);
        let second = referenced_object(&mut e, 0x600, 2);
        let source = make_list(&mut e, &[first, second]);
        let destination = make_list(&mut e, &[]);
        e.call(0x005a_b7f0, &args![source, destination, 0u32]);
        let copies = list_items(&e, destination);
        assert_eq!(copies.len(), 2);
        assert!(!copies.contains(&first) && !copies.contains(&second));
        assert_eq!(
            copies.iter().map(|c| e.mem.u32(c + 8)).collect::<Vec<_>>(),
            vec![0x500, 0x600]
        );
        // Variables.
        let one = variable(&mut e, 4, 1.5);
        let two = variable(&mut e, 5, 2.5);
        let source = make_list(&mut e, &[one, two]);
        let destination = make_list(&mut e, &[]);
        e.call(0x005a_b930, &args![source, destination, 0u32]);
        let copies = list_items(&e, destination);
        assert_eq!(copies.len(), 2);
        assert_eq!(e.mem.f64(copies[1] + 8), 2.5);
        // A null list on either side copies nothing.
        e.call(0x005a_b930, &args![0u32, destination, 0u32]);
        e.call(0x005a_b930, &args![source, 0u32, 0u32]);
        assert_eq!(list_items(&e, destination).len(), 2);
        // An empty source node copies nothing.
        let empty = make_list(&mut e, &[]);
        let target = make_list(&mut e, &[]);
        e.call(0x005a_b7f0, &args![empty, target, 0u32]);
        assert!(list_items(&e, target).is_empty());
    }

    // ---- Load ------------------------------------------------------------------------

    /// A chunk reader: the chunks come from a queue, `next` pops one, and
    /// the data readers fill a buffer or a value from per-chunk bytes.
    type Strings = Rc<RefCell<Vec<(u32, Vec<u8>)>>>;

    fn file_engine(chunks: Vec<(u32, Vec<u8>)>, record_type: u32) -> (Engine, u32, Strings) {
        let mut e = tail_engine();
        let queue = Rc::new(RefCell::new(chunks));
        e.register_double(FILE_RECORD_TYPE, move |_, _| ret(record_type));
        e.register(FORM_LOAD, |_, _| Ret::default());
        let q = queue.clone();
        e.register_double(FILE_CHUNK_TYPE, move |_, _| {
            ret(q.borrow().first().map(|c| c.0).unwrap_or(0))
        });
        let q = queue.clone();
        e.register_double(FILE_CHUNK_SIZE, move |_, _| {
            ret(q.borrow().first().map(|c| c.1.len() as u32).unwrap_or(0))
        });
        let q = queue.clone();
        e.register_double(FILE_NEXT_CHUNK, move |_, _| {
            q.borrow_mut().remove(0);
            ret(!q.borrow().is_empty() as u32)
        });
        let q = queue.clone();
        e.register_double(FILE_GET_CHUNK_DATA, move |e, a| {
            let bytes = q.borrow()[0].1.clone();
            e.mem.write(a[1], &bytes);
            Ret::default()
        });
        let q = queue.clone();
        e.register_double(FILE_GET_CHUNK_VALUE, move |e, a| {
            let bytes = q.borrow()[0].1.clone();
            e.mem.write(a[1], &bytes);
            Ret::default()
        });
        e.register(FILE_IS_BIG_ENDIAN, |_, _| ret(1));
        e.register(HEADER_SWAP_BYTES, |_, _| Ret::default());
        e.register(FILE_NAME_OF, |_, a| ret(a[0] + 0x20));
        e.register(SCRIPT_VARIABLE_CONSTRUCT, |_, a| ret(a[0]));
        e.register(SCRIPT_VARIABLE_LOAD, |e, a| {
            e.mem.set_u32(a[0], 0x77);
            Ret::default()
        });
        let strings: Strings = Rc::new(RefCell::new(Vec::new()));
        let sink = strings.clone();
        e.register_double(STRING_SET, move |e, a| {
            sink.borrow_mut().push((a[0], e.mem.cstr(a[1])));
            Ret::default()
        });
        e.register(REFERENCED_OBJECT_CONSTRUCT, |_, a| ret(a[0]));
        let file = e.mem.alloc(0x40);
        (e, file, strings)
    }

    #[test]
    fn load_reads_every_chunk_kind() {
        let header = (0..0x14u8).collect::<Vec<_>>();
        let (mut e, file, strings) = file_engine(
            vec![
                (CHUNK_EDID, b"MyScript\0".to_vec()),
                (CHUNK_SCHR, header.clone()),
                (CHUNK_SCDA, vec![0xaa, 0xbb, 0xcc]),
                (CHUNK_SLSD, vec![0; 4]),
                (CHUNK_SCVR, b"fVar\0".to_vec()),
                (CHUNK_SCRO, 0x1234u32.to_le_bytes().to_vec()),
                (CHUNK_SCRV, 7u32.to_le_bytes().to_vec()),
                (CHUNK_RNAM, vec![1, 2, 3, 4]),
                (CHUNK_OBND, vec![]),
                (0x1234_5678, vec![]),
            ],
            0x11,
        );
        let (this, calls) = recording_script(&mut e, &[0x134, 0xe0]);
        // The editor id slot keeps the text it was given (the buffer is gone later).
        let names: Rc<RefCell<Vec<Vec<u8>>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = names.clone();
        let capture = fake_code(&mut e, move |e, a| {
            sink.borrow_mut().push(e.mem.cstr(a[1]));
            Ret::default()
        });
        let table = e.mem.u32(this.addr());
        e.mem.set_u32(table + 0x134, capture);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005a_b9f0, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        // EDID went to the editor id slot as a string.
        assert_eq!(*names.borrow(), vec![b"MyScript".to_vec()]);
        // OBND went to its slot with the file.
        assert_eq!(*calls.borrow(), vec![(0xe0, vec![this.addr(), file])]);
        // The header chunk was read into the script and swapped (big-endian).
        assert_eq!(e.mem.bytes(this.addr() + 0x18, 0x14), header);
        assert_eq!(
            calls_to(&log, HEADER_SWAP_BYTES),
            vec![vec![this.addr() + 0x18]]
        );
        // Compiled data was read into a new block.
        let data = e.get(this, Script::m_data).addr();
        assert_eq!(e.mem.bytes(data, 3), vec![0xaa, 0xbb, 0xcc]);
        assert_eq!(calls_to(&log, MEMSET).len(), 2);
        // The variable was loaded, named and added.
        let variables = list_items(&e, this.addr() + 0x4c);
        assert_eq!(variables.len(), 1);
        assert_eq!(e.mem.u32(variables[0]), 0x77);
        assert_eq!(
            *strings.borrow(),
            vec![(variables[0] + 0x18, b"fVar".to_vec())]
        );
        // Two referenced objects: one with a form word, one with a variable id.
        let objects = list_items(&e, this.addr() + 0x44);
        assert_eq!(objects.len(), 2);
        assert_eq!(e.mem.u32(objects[0] + 8), 0x1234);
        assert_eq!(e.mem.u32(objects[1] + 0xc), 7);
        // Compiled: no "not compiled" message.
        assert!(calls_to(&log, SCRIPT_LOG).is_empty());
    }

    #[test]
    fn load_rejects_other_records_and_logs_a_script_without_data() {
        let (mut e, file, _) = file_engine(vec![(CHUNK_SCVR, b"x\0".to_vec())], 0x12);
        let (this, _) = recording_script(&mut e, &[0x130]);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x005a_b9f0, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FORM_LOAD).is_empty());
        // A script record with only a name chunk for a variable that was never
        // declared: no variable to name, no data, a message and true.
        let (mut e, file, strings) = file_engine(vec![(CHUNK_SCVR, b"x\0".to_vec())], 0x11);
        let (this, _) = recording_script(&mut e, &[0x130]);
        e.call_log = Some(vec![]);
        assert!(e.call(0x005a_b9f0, &args![this, file]).bool());
        let log = e.call_log.take().unwrap();
        assert!(strings.borrow().is_empty());
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![FORMAT_NOT_COMPILED, 0x7130, file + 0x20]]
        );
        // A little-endian file does not swap the header.
        let (mut e, file, _) = file_engine(vec![(CHUNK_SCHR, vec![0; 0x14])], 0x11);
        e.register(FILE_IS_BIG_ENDIAN, |_, _| ret(0));
        let (this, _) = recording_script(&mut e, &[0x130]);
        e.call_log = Some(vec![]);
        e.call(0x005a_b9f0, &args![this, file]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, HEADER_SWAP_BYTES).is_empty());
    }

    // ---- saving, text, variable names -------------------------------------------------

    #[test]
    fn save_forwards_to_the_form_save() {
        let mut e = tail_engine();
        e.register(FORM_SAVE, |_, _| Ret::default());
        let this = e.new_object::<Script>();
        e.call_log = Some(vec![]);
        e.call(0x005a_bd70, &args![this, 0xb0f0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FORM_SAVE), vec![vec![this.addr(), 0xb0f0]]);
    }

    #[test]
    fn save_answers_the_form_save() {
        let mut e = tail_engine();
        e.register(FORM_SAVE, |_, _| 1u32.into_ret());
        let this = e.new_object::<Script>();
        assert!(e.call(0x005a_bd70, &args![this, 0xb0f0u32]).bool());
        e.register(FORM_SAVE, |_, _| 0u32.into_ret());
        assert!(!e.call(0x005a_bd70, &args![this, 0xb0f0u32]).bool());
    }

    #[test]
    fn save_writes_the_header_chunk_swapping_around_it() {
        let mut e = tail_engine();
        for address in [
            FORM_START,
            EMPTY_METHOD,
            FORM_CLOSE,
            FORM_ADD_CHUNK,
            HEADER_SWAP_BYTES,
        ] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(ENDIAN_SWAP_ENABLED, |_, _| ret(1));
        let this = e.new_object::<Script>();
        let header = this.addr() + 0x18;
        e.call_log = Some(vec![]);
        e.call(0x005a_bd90, &args![this]);
        let log = e.call_log.take().unwrap();
        let order: Vec<u32> = log
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| *a != ENDIAN_SWAP_ENABLED)
            .collect();
        assert_eq!(
            order,
            vec![
                0x005a_bd90,
                FORM_START,
                EMPTY_METHOD,
                HEADER_SWAP_BYTES,
                FORM_ADD_CHUNK,
                HEADER_SWAP_BYTES,
                FORM_CLOSE
            ]
        );
        assert_eq!(
            calls_to(&log, FORM_ADD_CHUNK),
            vec![vec![CHUNK_SCHR, header, 0x14]]
        );
        // Without the swap only the chunk is written.
        e.register(ENDIAN_SWAP_ENABLED, |_, _| ret(0));
        e.call_log = Some(vec![]);
        e.call(0x005a_be00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, HEADER_SWAP_BYTES).is_empty());
        assert_eq!(calls_to(&log, FORM_ADD_CHUNK).len(), 1);
        // With it, the result-script save swaps before and after.
        e.register(ENDIAN_SWAP_ENABLED, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x005a_be00, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, HEADER_SWAP_BYTES).len(), 2);
    }

    #[test]
    fn set_text_keeps_a_zero_terminated_copy() {
        let mut e = tail_engine();
        let this = e.new_object::<Script>();
        let text = c_string(&mut e, "Begin GameMode");
        e.call(0x005a_be50, &args![this, text]);
        let copy = e.get(this, Script::m_text).addr();
        assert_ne!(copy, text);
        assert_eq!(e.mem.cstr(copy), b"Begin GameMode".to_vec());
        // Replacing frees the old copy; null clears.
        e.call_log = Some(vec![]);
        e.call(0x005a_be50, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(deleted(&log), vec![copy]);
        assert!(e.get(this, Script::m_text).is_null());
        // With no text there is nothing to free.
        e.call_log = Some(vec![]);
        e.call(0x005a_be50, &args![this, text]);
        let log = e.call_log.take().unwrap();
        assert!(deleted(&log).is_empty());
    }

    #[test]
    fn variable_name_is_found_by_id_or_logged() {
        let mut e = tail_engine();
        let (this, _) = recording_script(&mut e, &[0x130]);
        let first = script_variable(&mut e, 1, "fOne");
        let second = script_variable(&mut e, 2, "fSpeed");
        let name = e.mem.u32(second + 0x18);
        fill_list(&mut e, this.addr() + 0x4c, &[first, second]);
        assert_eq!(e.call(0x005a_bed0, &args![this, 2u32]).u32(), name);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x005a_bed0, &args![this, 9u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![FORMAT_LOCAL_VARIABLE_NOT_FOUND, 9, 0x7130]]
        );
    }

    // ---- ScriptLocals of a script -------------------------------------------------------

    #[test]
    fn locals_are_built_from_copies_of_the_script_variables() {
        let mut e = tail_engine();
        e.register(SCRIPT_LOCALS_CONSTRUCT, |_, a| ret(a[0]));
        let this = e.new_object::<Script>();
        let first = variable(&mut e, 1, 1.5);
        let second = variable(&mut e, 2, 2.5);
        e.mem.set_u8(second + 0x10, 1);
        fill_list(&mut e, this.addr() + 0x4c, &[first, second]);
        e.call_log = Some(vec![]);
        let locals = e.call(0x005a_bf60, &args![this]).ptr::<ScriptLocals>();
        let log = e.call_log.take().unwrap();
        assert_eq!(
            e.get(locals, ScriptLocals::m_pMasterScript).addr(),
            this.addr()
        );
        let list = e.get(locals, ScriptLocals::m_pLocalList).addr();
        let copies = list_items(&e, list);
        assert_eq!(copies.len(), 2);
        assert!(!copies.contains(&first) && !copies.contains(&second));
        assert_eq!(e.mem.u32(copies[0]), 1);
        assert_eq!(e.mem.f64(copies[0] + 8), 1.5);
        assert_eq!(e.mem.u8(copies[0] + 0x10), 0);
        assert_eq!(e.mem.f64(copies[1] + 8), 2.5);
        assert_eq!(e.mem.u8(copies[1] + 0x10), 1);
        // The locals block is 0x14 bytes; two scope guards, lines 0xcde and
        // 0xceb, are opened.
        assert_eq!(calls_to(&log, OPERATOR_NEW)[0], vec![0x14]);
        let guards = calls_to(&log, SCOPE_GUARD_OPEN);
        assert_eq!(guards.len(), 2);
        assert_eq!(guards[0][4], 0xcde);
        assert_eq!(guards[1][4], 0xceb);
        assert_eq!(calls_to(&log, SCOPE_GUARD_CLOSE).len(), 2);
    }

    #[test]
    fn init_item_with_itself_as_the_source_takes_no_file() {
        let mut e = init_item_engine(0);
        let (this, _) = recording_script(&mut e, &[0x130]);
        e.call_log = Some(vec![]);
        e.call(0x005a_b7d0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SCOPE_GUARD_OPEN).len(), 1);
        assert!(calls_to(&log, FORM_SET_FILE).is_empty());
        assert_eq!(calls_to(&log, SCRIPT_SET_QUEST_SCRIPT_FLAG).len(), 1);
    }

    #[test]
    fn variable_list_copy_makes_one_copy_per_variable() {
        let mut e = tail_engine();
        let this = e.new_object::<Script>();
        let first = variable(&mut e, 1, 1.5);
        let second = variable(&mut e, 2, 2.5);
        fill_list(&mut e, this.addr() + 0x4c, &[first, second]);
        e.call_log = Some(vec![]);
        let list = e.call(0x005a_c020, &args![this]).u32();
        let log = e.call_log.take().unwrap();
        let copies = list_items(&e, list);
        assert_eq!(copies.len(), 2);
        assert_eq!(e.mem.f64(copies[1] + 8), 2.5);
        assert_eq!(calls_to(&log, SCOPE_GUARD_OPEN)[0][4], 0xceb);
    }

    #[test]
    fn variable_data_copy_constructor_copies_id_value_and_flag() {
        let mut e = tail_engine();
        let source = variable(&mut e, 9, 4.25);
        e.mem.set_u8(source + 0x10, 1);
        let this = e.mem.alloc(0x18);
        let result = e.call(0x005a_c150, &args![this, source]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), 9);
        assert_eq!(e.mem.f64(this + 8), 4.25);
        assert_eq!(e.mem.u8(this + 0x10), 1);
    }

    #[test]
    fn init_action_list_asks_the_run_manager_once() {
        let mut e = tail_engine();
        let locals = locals(&mut e);
        let master = e.mem.alloc(0x20);
        e.set(locals, ScriptLocals::m_pMasterScript, Ptr::new(master));
        let extra = e.mem.alloc(0x20);
        e.register_double(EXTRA_LIST_SCRIPT_LOCALS, move |_, _| ret(locals.addr()));
        e.register(SCRIPT_RUN_MANAGER_INSTANCE, |_, _| ret(0x3ab));
        e.register(SCRIPT_RUN_MANAGER_INIT_ACTION_LIST, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x005a_c190, &args![0xf00du32, extra]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_RUN_MANAGER_INIT_ACTION_LIST),
            vec![vec![0x3ab, master, 0xf00d, locals.addr()]]
        );
        // It already has an action list: nothing.
        set_actions(&mut e, locals, &[(1, 0)]);
        e.call_log = Some(vec![]);
        e.call(0x005a_c190, &args![0xf00du32, extra]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SCRIPT_RUN_MANAGER_INIT_ACTION_LIST).is_empty());
        // No extra data list, or one with no locals: nothing.
        e.call_log = Some(vec![]);
        e.call(0x005a_c190, &args![0xf00du32, 0u32]);
        e.register(EXTRA_LIST_SCRIPT_LOCALS, |_, _| ret(0));
        e.call(0x005a_c190, &args![0xf00du32, extra]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SCRIPT_RUN_MANAGER_INIT_ACTION_LIST).is_empty());
    }

    // ---- Run -------------------------------------------------------------------------

    type Runs = Rc<RefCell<Vec<Vec<u32>>>>;

    /// Doubles for running: the manager (answering `answer`), the elapsed
    /// time 0.5 and the quest delay setting `setting`.
    fn run_engine(answer: u32, setting: f32) -> (Engine, Runs) {
        let mut e = tail_engine();
        e.register(SCRIPT_RUN_MANAGER_INSTANCE, |_, _| ret(0x3ab));
        let runs: Runs = Rc::new(RefCell::new(Vec::new()));
        let sink = runs.clone();
        e.register_double(SCRIPT_RUN_MANAGER_RUN, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(answer)
        });
        e.register(ELAPSED_TIME, |_, _| Ret {
            st0: 0.5,
            ..Ret::default()
        });
        let cell = e.mem.alloc(4);
        e.mem.set_f32(cell, setting);
        e.register_double(QUEST_DELAY_ACCESSOR, move |_, _| ret(cell));
        (e, runs)
    }

    fn runnable_script(e: &mut Engine, data_size: u32, quest: bool) -> Ptr<Script> {
        let script = e.new_object::<Script>();
        e.set(script_header(script), ScriptHeader::dataSize, data_size);
        e.set(script_header(script), ScriptHeader::bIsQuestScript, quest);
        script
    }

    #[test]
    fn run_skips_tiny_scripts_unless_forced() {
        let (mut e, runs) = run_engine(1, 0.0);
        let this = runnable_script(&mut e, 4, false);
        assert!(!e
            .call(0x005a_c1e0, &args![this, 1u32, 2u32, 3u32, 0u8])
            .bool());
        assert!(runs.borrow().is_empty());
        // Forced, or big enough: the manager runs it with the 8 words.
        assert!(e
            .call(0x005a_c1e0, &args![this, 1u32, 2u32, 3u32, 1u8])
            .bool());
        assert_eq!(
            runs.borrow()[0],
            vec![0x3ab, this.addr(), 1, 2, 3, 1, 0, 0, 0]
        );
        let big = runnable_script(&mut e, 5, false);
        e.call(0x005a_c1e0, &args![big, 1u32, 2u32, 3u32, 0u8]);
        assert_eq!(
            runs.borrow()[1],
            vec![0x3ab, big.addr(), 1, 2, 3, 0, 0, 0, 0]
        );
    }

    #[test]
    fn run_answers_what_the_manager_answers_and_forgets_the_reference_cache() {
        let (mut e, _) = run_engine(0, 0.0);
        let this = runnable_script(&mut e, 8, false);
        let object = referenced_object(&mut e, 0x500, 3);
        let plain = referenced_object(&mut e, 0x600, 0);
        fill_list(&mut e, this.addr() + 0x44, &[object, plain]);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_LAST_REF_SEARCH_SCRIPT, this.addr());
        assert!(!e
            .call(0x005a_c1e0, &args![this, 0u32, 0u32, 0u32, 0u8])
            .bool());
        // The cache is dropped and the object with a variable id loses its form.
        assert_eq!(e.mem.u32(tls + TLS_LAST_REF_SEARCH_SCRIPT), 0);
        assert_eq!(e.mem.u32(object + 8), 0);
        assert_eq!(e.mem.u32(plain + 8), 0x600);
    }

    #[test]
    fn run_counts_down_a_quest_scripts_delay() {
        let (mut e, runs) = run_engine(1, 0.0);
        let this = runnable_script(&mut e, 8, true);
        e.set(this, Script::fQuestScriptDelay, 2.0);
        e.set(this, Script::fQuestScriptGetSecondsBuffer, 0.25);
        // 2.0 - 0.5 is still above 0: no run, and the time is banked.
        assert!(!e
            .call(0x005a_c1e0, &args![this, 0u32, 0u32, 0u32, 0u8])
            .bool());
        assert!(runs.borrow().is_empty());
        assert_eq!(e.get(this, Script::fQuestScriptDelay), 1.5);
        assert_eq!(e.get(this, Script::fQuestScriptGetSecondsBuffer), 0.75);
    }

    #[test]
    fn run_resets_a_quest_scripts_delay_after_running() {
        let (mut e, runs) = run_engine(1, 4.0);
        let this = runnable_script(&mut e, 8, true);
        e.set(this, Script::fQuestScriptDelay, 0.5);
        e.set(this, Script::fQuestScriptGetSecondsBuffer, 0.25);
        // 0.5 - 0.5 reaches 0: it runs, then the setting is added.
        assert!(e
            .call(0x005a_c1e0, &args![this, 0u32, 0u32, 0u32, 0u8])
            .bool());
        assert_eq!(runs.borrow().len(), 1);
        assert_eq!(e.get(this, Script::fQuestScriptDelay), 4.0);
        assert_eq!(e.get(this, Script::fQuestScriptGetSecondsBuffer), 0.0);
        // With an owner quest that has its own delay, that is added instead.
        e.register(QUEST_DELAY_OF, |_, _| Ret {
            st0: 3.0,
            ..Ret::default()
        });
        let quest = e.mem.alloc(0x20);
        e.set(this, Script::pOwnerQuest, Ptr::new(quest));
        e.set(this, Script::fQuestScriptDelay, 0.0);
        e.call(0x005a_c1e0, &args![this, 0u32, 0u32, 0u32, 0u8]);
        assert_eq!(e.get(this, Script::fQuestScriptDelay), 3.0);
        // A setting of 0 leaves the delay alone.
        let (mut e, _) = run_engine(1, 0.0);
        let this = runnable_script(&mut e, 8, true);
        e.set(this, Script::fQuestScriptDelay, 0.0);
        e.set(this, Script::fQuestScriptGetSecondsBuffer, 0.25);
        e.call(0x005a_c1e0, &args![this, 0u32, 0u32, 0u32, 0u8]);
        assert_eq!(e.get(this, Script::fQuestScriptDelay), 0.0);
        assert_eq!(e.get(this, Script::fQuestScriptGetSecondsBuffer), 0.25);
    }

    #[test]
    fn script_effect_runs_pass_their_flags_to_the_manager() {
        let (mut e, runs) = run_engine(1, 0.0);
        let this = e.new_object::<Script>();
        e.call(0x005a_c340, &args![this, 7u32, 8u32]);
        e.call(0x005a_c380, &args![this, 7u32, 8u32]);
        e.call(0x005a_c3c0, &args![this, 7u32, 8u32, 1.5f32]);
        let runs = runs.borrow();
        let s = this.addr();
        assert_eq!(runs[0], vec![0x3ab, s, 7, 8, 0, 0, 1, 0, 0]);
        assert_eq!(runs[1], vec![0x3ab, s, 7, 8, 0, 0, 0, 1, 0]);
        assert_eq!(runs[2], vec![0x3ab, s, 7, 8, 0, 0, 0, 0, 1.5f32.to_bits()]);
    }

    #[test]
    fn compile_and_run_runs_what_compiled() {
        let (mut e, runs) = run_engine(1, 0.0);
        let compiled = Rc::new(Cell::new(0u32));
        let flag = compiled.clone();
        let seen_output: Rc<RefCell<Vec<u8>>> = Rc::new(RefCell::new(Vec::new()));
        let seen = seen_output.clone();
        e.register_double(COMPILE_PARTIAL_SCRIPT, move |_, _| ret(flag.get()));
        e.register(REFERENCE_SCRIPT_VARIABLES, |_, _| ret(0x5ca1));
        let this = runnable_script(&mut e, 8, false);
        let tls = e.tls();
        // Nothing compiled: no run.
        e.call_log = Some(vec![]);
        e.call(0x005a_c400, &args![this, 0xc0deu32, 1u32, 0xf00du32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, COMPILE_PARTIAL_SCRIPT),
            vec![vec![0xc0de, this.addr(), 0xf00d, 1, 0]]
        );
        assert!(runs.borrow().is_empty());
        assert_eq!(calls_to(&log, SCOPE_GUARD_OPEN)[0][4], 0xd53);
        // Compiled, mode 1: runs with the reference's variables and the
        // system-output byte set while it does.
        compiled.set(1);
        let sink = runs.clone();
        e.register_double(SCRIPT_RUN_MANAGER_RUN, move |e, a| {
            sink.borrow_mut().push(a.to_vec());
            let tls = e.tls();
            seen.borrow_mut().push(e.mem.u8(tls + TLS_SYSTEM_OUTPUT));
            ret(1)
        });
        e.call(0x005a_c400, &args![this, 0xc0deu32, 1u32, 0xf00du32]);
        assert_eq!(
            runs.borrow()[0],
            vec![0x3ab, this.addr(), 0xf00d, 0x5ca1, 0, 1, 0, 0, 0]
        );
        assert_eq!(*seen_output.borrow(), vec![1]);
        assert_eq!(e.mem.u8(tls + TLS_SYSTEM_OUTPUT), 0);
        // Another mode, no reference: no variables and the byte stays clear.
        e.call(0x005a_c400, &args![this, 0xc0deu32, 0u32, 0u32]);
        assert_eq!(
            runs.borrow()[1],
            vec![0x3ab, this.addr(), 0, 0, 0, 1, 0, 0, 0]
        );
        assert_eq!(*seen_output.borrow(), vec![1, 0]);
        // Compiled to nothing: no run.
        e.set(script_header(this), ScriptHeader::dataSize, 0);
        e.call(0x005a_c400, &args![this, 0xc0deu32, 0u32, 0u32]);
        assert_eq!(runs.borrow().len(), 2);
    }

    // ---- referenced objects and variables ----------------------------------------------

    #[test]
    fn referenced_object_lookup_is_one_based_and_cached() {
        let mut e = tail_engine();
        let this = e.new_object::<Script>();
        e.set(script_header(this), ScriptHeader::refObjectCount, 2);
        let first = referenced_object(&mut e, 0x500, 0);
        let second = referenced_object(&mut e, 0x600, 0);
        fill_list(&mut e, this.addr() + 0x44, &[first, second]);
        assert_eq!(e.call(0x005a_c4f0, &args![this, 0u32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x005a_c4f0, &args![this, 3u32, 0u32]).u32(), 0);
        assert_eq!(e.call(0x005a_c4f0, &args![this, 1u32, 0u32]).u32(), first);
        assert_eq!(e.call(0x005a_c4f0, &args![this, 2u32, 0u32]).u32(), second);
        let tls = e.tls();
        assert_eq!(e.mem.u32(tls + TLS_LAST_REF_SEARCH_SCRIPT), this.addr());
        assert_eq!(e.mem.u32(tls + TLS_LAST_REF_INDEX), 2);
        assert_eq!(e.mem.u32(tls + TLS_LAST_REF_OBJECT), second);
        // The cache answers without walking: break the list to prove it.
        e.mem.set_u32(this.addr() + 0x44 + 4, 0);
        assert_eq!(e.call(0x005a_c4f0, &args![this, 2u32, 0u32]).u32(), second);
        // A different locals argument is a new lookup, which now falls off
        // the end of the list.
        assert_eq!(e.call(0x005a_c4f0, &args![this, 2u32, 0x1111u32]).u32(), 0);
    }

    #[test]
    fn referenced_object_with_a_variable_takes_its_form_from_the_locals() {
        let mut e = tail_engine();
        e.register(GET_NUMERIC_ID_FROM_DOUBLE, |e, a| {
            let id = e.mem.f64(a[1]) as u32;
            e.mem.set_u32(a[0], id);
            Ret::default()
        });
        e.register(LOOKUP_FORM_BY_ID, |_, a| ret(a[0] + 0x100));
        let this = e.new_object::<Script>();
        e.set(script_header(this), ScriptHeader::refObjectCount, 1);
        let object = referenced_object(&mut e, 0x500, 5);
        fill_list(&mut e, this.addr() + 0x44, &[object]);
        let locals = locals(&mut e);
        set_variables(&mut e, locals, &[(5, 4660.0)]);
        assert_eq!(
            e.call(0x005a_c4f0, &args![this, 1u32, locals]).u32(),
            object
        );
        assert_eq!(e.mem.u32(object + 8), 4660 + 0x100);
        // A zero numeric id leaves the form as it was.
        set_variables(&mut e, locals, &[(5, 0.0)]);
        e.mem.set_u32(object + 8, 0x777);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_LAST_LOCALS, 0);
        e.mem.set_u32(tls + TLS_LAST_REF_SEARCH_SCRIPT, 0);
        e.call(0x005a_c4f0, &args![this, 1u32, locals]);
        assert_eq!(e.mem.u32(object + 8), 0x777);
    }

    #[test]
    fn find_variable_answers_the_type_letter_and_stores_the_id() {
        let mut e = tail_engine();
        e.register(STRING_COMPARE, |e, a| {
            ret((e.mem.cstr(a[0]) != e.mem.cstr(a[1])) as u32)
        });
        let this = e.new_object::<Script>();
        let float = script_variable(&mut e, 3, "fSpeed");
        let integer = script_variable(&mut e, 4, "iCount");
        e.mem.set_u8(integer + 0x10, 1);
        fill_list(&mut e, this.addr() + 0x4c, &[float, integer]);
        let out = e.mem.alloc(4);
        let wanted = c_string(&mut e, "iCount");
        assert_eq!(e.call(0x005a_c6a0, &args![this, wanted, out]).u8(), b's');
        assert_eq!(e.mem.u32(out), 4);
        let wanted = c_string(&mut e, "fSpeed");
        assert_eq!(e.call(0x005a_c6a0, &args![this, wanted, out]).u8(), b'f');
        assert_eq!(e.mem.u32(out), 3);
        let wanted = c_string(&mut e, "missing");
        assert_eq!(e.call(0x005a_c6a0, &args![this, wanted, out]).u8(), 0);
        assert_eq!(e.mem.u32(out), 0);
    }

    #[test]
    fn process_scripts_flag_is_a_byte() {
        let mut e = tail_engine();
        e.call(0x005a_c730, &args![1u8]);
        assert_eq!(e.call(0x005a_c740, &args![]).u8(), 1);
        e.call(0x005a_c730, &args![0u8]);
        assert_eq!(e.call(0x005a_c740, &args![]).u8(), 0);
    }

    #[test]
    fn set_action_flag_goes_through_the_extra_list_locals() {
        let mut e = tail_engine();
        let locals = locals(&mut e);
        set_actions(&mut e, locals, &[(0x10, 0), (0, 0)]);
        e.register_double(EXTRA_LIST_SCRIPT_LOCALS, move |_, _| ret(locals.addr()));
        let extra = e.mem.alloc(0x20);
        assert!(e
            .call(0x005a_c750, &args![0x10u32, extra, 0x1000u32])
            .bool());
        assert_eq!(action_list(&e, locals), vec![(0x10, 0x1000), (0, 0x1000)]);
        // No extra list, or one with no locals: false.
        assert!(!e.call(0x005a_c750, &args![0x10u32, 0u32, 0x1000u32]).bool());
        e.register(EXTRA_LIST_SCRIPT_LOCALS, |_, _| ret(0));
        assert!(!e
            .call(0x005a_c750, &args![0x10u32, extra, 0x1000u32])
            .bool());
    }

    #[test]
    fn numeric_ids_travel_in_the_low_word_of_a_double() {
        let mut e = tail_engine();
        let id = e.mem.alloc(4);
        let value = e.mem.alloc(8);
        e.mem.set_u32(id, 0x0001_0203);
        e.mem.set_f64(value, 0.0);
        e.call(0x005a_cc70, &args![id, value]);
        assert_eq!(e.mem.u32(value), 0x0001_0203);
        assert_eq!(e.mem.u32(value + 4), 0);
        let back = e.mem.alloc(4);
        e.call(0x005a_cc90, &args![back, value]);
        assert_eq!(e.mem.u32(back), 0x0001_0203);
    }

    #[test]
    fn clear_reference_cache_forgets_forms_that_have_variables() {
        let mut e = tail_engine();
        let this = e.new_object::<Script>();
        let with_variable = referenced_object(&mut e, 0x500, 4);
        let plain = referenced_object(&mut e, 0x600, 0);
        fill_list(&mut e, this.addr() + 0x44, &[with_variable, plain]);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_LAST_REF_SEARCH_SCRIPT, this.addr());
        e.call(0x005a_e2e0, &args![this]);
        assert_eq!(e.mem.u32(tls + TLS_LAST_REF_SEARCH_SCRIPT), 0);
        assert_eq!(e.mem.u32(with_variable + 8), 0);
        assert_eq!(e.mem.u32(plain + 8), 0x600);
        // Another script in the cache stays.
        e.mem.set_u32(tls + TLS_LAST_REF_SEARCH_SCRIPT, 0x1234);
        e.call(0x005a_e2e0, &args![this]);
        assert_eq!(e.mem.u32(tls + TLS_LAST_REF_SEARCH_SCRIPT), 0x1234);
    }

    // ---- operands (005ac7a0) -------------------------------------------------------------

    /// Compiled data in memory and the cell that holds the read offset.
    fn code_with(e: &mut Engine, bytes: &[u8]) -> (u32, u32) {
        let code = e.mem.alloc(bytes.len() as u32 + 8);
        e.mem.write(code, bytes);
        let offset = e.mem.alloc(4);
        (code, offset)
    }

    /// Doubles for operands and parameters: bit 3 of the form flags answers
    /// `flag`, the form type is the byte at +4, numeric ids travel in the low
    /// word of a double.
    fn operand_engine(flag: u32) -> Engine {
        let mut e = tail_engine();
        e.register_double(FORM_FLAG_BIT_3, move |_, _| ret(flag));
        e.register(FORM_TYPE_OF, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(PUT_NUMERIC_ID_IN_DOUBLE, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_f64(a[1], id as f64);
            Ret::default()
        });
        e
    }

    /// Calls `fn_005ac7a0(out, code, &offset, reference, container, script,
    /// locals, parse)`.
    fn operand(
        e: &mut Engine,
        code: u32,
        offset: u32,
        script: u32,
        locals: u32,
        parse: u8,
    ) -> (bool, f64) {
        let out = e.mem.alloc(8);
        let ok = e
            .call(
                0x005a_c7a0,
                &args![out, code, offset, 0xaaa0u32, 0xbbb0u32, script, locals, parse],
            )
            .bool();
        (ok, e.mem.f64(out))
    }

    #[test]
    fn operand_immediates_and_variables() {
        let mut e = operand_engine(0);
        let mut bytes = vec![b'n'];
        bytes.extend_from_slice(&(-42i32).to_le_bytes());
        bytes.push(b'z');
        bytes.extend_from_slice(&1.25f64.to_le_bytes());
        bytes.extend_from_slice(&[b'f', 5, 0, b's', 5, 0, b'q', 5, 0]);
        let (code, offset) = code_with(&mut e, &bytes);
        let this = e.new_object::<Script>();
        let locals = locals(&mut e);
        set_variables(&mut e, locals, &[(5, 9.5)]);
        assert_eq!(
            operand(&mut e, code, offset, this.addr(), locals.addr(), 0),
            (true, -42.0)
        );
        assert_eq!(e.mem.u32(offset), 5);
        assert_eq!(
            operand(&mut e, code, offset, this.addr(), locals.addr(), 0),
            (true, 1.25)
        );
        assert_eq!(e.mem.u32(offset), 14);
        assert_eq!(
            operand(&mut e, code, offset, this.addr(), locals.addr(), 0),
            (true, 9.5)
        );
        assert_eq!(
            operand(&mut e, code, offset, this.addr(), locals.addr(), 0),
            (true, 9.5)
        );
        // An unknown marker still consumes its id, and answers false.
        assert!(!operand(&mut e, code, offset, this.addr(), locals.addr(), 0).0);
        assert_eq!(e.mem.u32(offset), 23);
        // No locals: nothing is read.
        e.mem.set_u32(offset, 0);
        assert!(!operand(&mut e, code, offset, this.addr(), 0, 0).0);
        assert_eq!(e.mem.u32(offset), 0);
    }

    /// A script with one referenced object (form `form`, variable id
    /// `variable_id`).
    fn script_with_object(e: &mut Engine, form: u32, variable_id: u32) -> (Ptr<Script>, u32) {
        let script = e.new_object::<Script>();
        e.set(script_header(script), ScriptHeader::refObjectCount, 1);
        let object = referenced_object(e, form, variable_id);
        fill_list(e, script.addr() + 0x44, &[object]);
        (script, object)
    }

    #[test]
    fn operand_global_and_form_id_markers() {
        let mut e = operand_engine(0);
        e.register(GLOBAL_VALUE_OF, |_, _| Ret {
            st0: 6.5,
            ..Ret::default()
        });
        let form = e.mem.alloc(0x20);
        e.mem.set_u32(form + 0xc, 0x1234);
        let (script, object) = script_with_object(&mut e, form, 0);
        let locals = locals(&mut e);
        let (code, offset) = code_with(&mut e, &[b'G', 1, 0, b'Z', 1, 0]);
        assert_eq!(
            operand(&mut e, code, offset, script.addr(), locals.addr(), 0),
            (true, 6.5)
        );
        assert_eq!(
            operand(&mut e, code, offset, script.addr(), locals.addr(), 0),
            (true, 4660.0)
        );
        // No form: false, or 0.0 when the object has a variable id (and the
        // call is not a parse).
        e.mem.set_u32(object + 8, 0);
        e.mem.set_u32(offset, 0);
        assert!(!operand(&mut e, code, offset, script.addr(), locals.addr(), 0).0);
        e.mem.set_u32(object + 0xc, 3);
        e.mem.set_u32(offset, 0);
        assert_eq!(
            operand(&mut e, code, offset, script.addr(), locals.addr(), 0),
            (true, 0.0)
        );
        // An index past the objects: false.
        let (code, offset) = code_with(&mut e, &[b'G', 2, 0]);
        assert!(!operand(&mut e, code, offset, script.addr(), locals.addr(), 0).0);
    }

    #[test]
    fn operand_reference_redirects_the_locals() {
        let mut e = operand_engine(0);
        // A quest (type 0x47): its locals answer the variable.
        let quest = e.mem.alloc(0x20);
        e.mem.set_u8(quest + 4, 0x47);
        let quest_locals = locals(&mut e);
        set_variables(&mut e, quest_locals, &[(7, 3.5)]);
        e.register_double(QUEST_SCRIPT_LOCALS, move |_, _| ret(quest_locals.addr()));
        let (script, _) = script_with_object(&mut e, quest, 0);
        let own = locals(&mut e);
        let (code, offset) = code_with(&mut e, &[b'r', 1, 0, b'f', 7, 0]);
        assert_eq!(
            operand(&mut e, code, offset, script.addr(), own.addr(), 0),
            (true, 3.5)
        );
        assert_eq!(e.mem.u32(offset), 6);
        // A reference (type 0x3a) with no extra reference pointer: the
        // reference's script variables.
        let reference = e.mem.alloc(0x20);
        e.mem.set_u8(reference + 4, 0x3a);
        let reference_locals = locals(&mut e);
        set_variables(&mut e, reference_locals, &[(7, 1.5)]);
        e.register_double(REFERENCE_SCRIPT_VARIABLES, move |_, _| {
            ret(reference_locals.addr())
        });
        e.register(EXTRA_LIST_REFERENCE_POINTER, |_, _| ret(0));
        let (script, _) = script_with_object(&mut e, reference, 0);
        e.mem.set_u32(offset, 0);
        assert_eq!(
            operand(&mut e, code, offset, script.addr(), own.addr(), 0),
            (true, 1.5)
        );
        // With an extra reference pointer: the inventory item's locals, and
        // the item is released.
        let item_locals = locals(&mut e);
        set_variables(&mut e, item_locals, &[(7, 2.5)]);
        e.register(EXTRA_LIST_REFERENCE_POINTER, |_, _| ret(0x9999));
        e.register(INVENTORY_KEY_OF, |_, _| ret(0x4e));
        e.register(INVENTORY_ITEM_OF, |_, _| ret(0x17e));
        e.register_double(ITEM_SCRIPT_LOCALS, move |_, _| ret(item_locals.addr()));
        e.register(RELEASE_ITEM, |_, _| Ret::default());
        let (script, _) = script_with_object(&mut e, reference, 0);
        e.mem.set_u32(reference + 0xc, 0xabc);
        e.mem.set_u32(offset, 0);
        e.call_log = Some(vec![]);
        assert_eq!(
            operand(&mut e, code, offset, script.addr(), own.addr(), 0),
            (true, 2.5)
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, INVENTORY_ITEM_OF),
            vec![vec![0x9999, 0x4e, 0xabc]]
        );
        assert_eq!(calls_to(&log, RELEASE_ITEM), vec![vec![0x17e, 1]]);
        // No locals found for the referenced object: false.
        e.register(ITEM_SCRIPT_LOCALS, |_, _| ret(0));
        e.mem.set_u32(offset, 0);
        assert!(!operand(&mut e, code, offset, script.addr(), own.addr(), 0).0);
        // A form of another type keeps the caller's locals.
        let other = e.mem.alloc(0x20);
        e.mem.set_u8(other + 4, 0x20);
        let (script, _) = script_with_object(&mut e, other, 0);
        set_variables(&mut e, own, &[(7, 8.5)]);
        e.mem.set_u32(offset, 0);
        assert_eq!(
            operand(&mut e, code, offset, script.addr(), own.addr(), 0),
            (true, 8.5)
        );
    }

    /// A function definition: needs-reference byte, parameters and handler.
    fn function_definition(
        e: &mut Engine,
        needs_reference: u8,
        parameters: u32,
        handler: u32,
    ) -> u32 {
        let definition = e.mem.alloc(0x20);
        e.mem.set_u8(definition + 0x10, needs_reference);
        e.mem.set_u32(definition + 0x14, parameters);
        e.mem.set_u32(definition + 0x18, handler);
        definition
    }

    #[test]
    fn operand_function_call_runs_the_handler() {
        let mut e = operand_engine(0);
        let words: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = words.clone();
        let answer = Rc::new(Cell::new(1u32));
        let reply = answer.clone();
        let handler = fake_code(&mut e, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(reply.get())
        });
        let definition = function_definition(&mut e, 0, 0x6a6a, handler);
        e.register_double(GET_FUNCTION_DEFINITION, move |_, _| ret(definition));
        let this = e.new_object::<Script>();
        let locals = locals(&mut e);
        let (code, offset) = code_with(&mut e, &[b'X', 0x34, 0x12, 0, 0]);
        let (ok, _) = operand(&mut e, code, offset, this.addr(), locals.addr(), 0);
        assert!(ok);
        // Function id and two skipped words consumed after the marker.
        assert_eq!(e.mem.u32(offset), 5);
        let out_words = words.borrow()[0].clone();
        assert_eq!(out_words[0], 0x6a6a); // parameters
        assert_eq!(out_words[1], code);
        assert_eq!(out_words[2], 0xaaa0); // reference
        assert_eq!(out_words[3], 0xbbb0); // container
        assert_eq!(out_words[4], this.addr());
        assert_eq!(out_words[5], locals.addr());
        assert_eq!(out_words[7], offset);
        // A handler that answers false: false.
        answer.set(0);
        e.mem.set_u32(offset, 0);
        assert!(!operand(&mut e, code, offset, this.addr(), locals.addr(), 0).0);
        // A definition that needs a reference with none: false, no call.
        let needy = function_definition(&mut e, 1, 0, handler);
        e.register_double(GET_FUNCTION_DEFINITION, move |_, _| ret(needy));
        answer.set(1);
        let before = words.borrow().len();
        e.mem.set_u32(offset, 0);
        let out = e.mem.alloc(8);
        let ok = e
            .call(
                0x005a_c7a0,
                &args![
                    out,
                    code,
                    offset,
                    0u32,
                    0u32,
                    this.addr(),
                    locals.addr(),
                    0u8
                ],
            )
            .bool();
        assert!(!ok);
        assert_eq!(words.borrow().len(), before);
        // An unknown function: false.
        e.register(GET_FUNCTION_DEFINITION, |_, _| ret(0));
        e.mem.set_u32(offset, 0);
        assert!(!operand(&mut e, code, offset, this.addr(), locals.addr(), 0).0);
    }

    #[test]
    fn operand_function_call_uses_the_referenced_object_as_target() {
        let mut e = operand_engine(0);
        let words: Rc<RefCell<Vec<Vec<u32>>>> = Rc::new(RefCell::new(Vec::new()));
        let sink = words.clone();
        let handler = fake_code(&mut e, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(1)
        });
        let definition = function_definition(&mut e, 1, 0, handler);
        e.register_double(GET_FUNCTION_DEFINITION, move |_, _| ret(definition));
        // A scripted reference (type 0x3a) named by `r` is the target.
        let reference = e.mem.alloc(0x20);
        e.mem.set_u8(reference + 4, 0x3a);
        let (script, _) = script_with_object(&mut e, reference, 0);
        let own = locals(&mut e);
        // 'r' then 'X': the locals are not redirected, the target is the form.
        let (code, offset) = code_with(&mut e, &[b'r', 1, 0, b'X', 1, 0, 0, 0]);
        let (ok, _) = operand(&mut e, code, offset, script.addr(), own.addr(), 0);
        assert!(ok);
        assert_eq!(words.borrow()[0][2], reference);
        // An object of another type clears the target.
        let other = e.mem.alloc(0x20);
        e.mem.set_u8(other + 4, 0x20);
        let (script, _) = script_with_object(&mut e, other, 0);
        e.mem.set_u32(offset, 0);
        let (ok, _) = operand(&mut e, code, offset, script.addr(), own.addr(), 0);
        assert!(!ok);
        assert_eq!(words.borrow().len(), 1);
    }

    #[test]
    fn operand_function_call_while_parsing_only_parses_the_parameters() {
        let mut e = operand_engine(0);
        let handler = fake_code(&mut e, |_, _| ret(1));
        // A parameter list that says there are no parameters: count 0.
        let parameters = e.mem.alloc(16);
        let definition = function_definition(&mut e, 0, parameters, handler);
        e.register_double(GET_FUNCTION_DEFINITION, move |_, _| ret(definition));
        let this = e.new_object::<Script>();
        let locals = locals(&mut e);
        // The count (0) of the parameters follows the call marker.
        let (code, offset) = code_with(&mut e, &[b'X', 1, 0, 0, 0, 0, 0]);
        let (ok, _) = operand(&mut e, code, offset, this.addr(), locals.addr(), 1);
        assert!(!ok);
        // marker, id, skipped word, then the count of 0 was read.
        assert_eq!(e.mem.u32(offset), 7);
        // With bit 3 of the flags set the parse flag is dropped: the handler
        // runs instead.
        e.register(FORM_FLAG_BIT_3, |_, _| ret(1));
        e.mem.set_u32(offset, 0);
        let (ok, _) = operand(&mut e, code, offset, this.addr(), locals.addr(), 1);
        assert!(ok);
    }

    // ---- ParseParameters -------------------------------------------------------------------

    /// A parameter list: 12 bytes per entry with the type at +4.
    fn parameter_list(e: &mut Engine, types: &[u32]) -> u32 {
        let list = e.mem.alloc(types.len() as u32 * 12 + 4);
        for (i, kind) in types.iter().enumerate() {
            e.mem.set_u32(list + i as u32 * 12 + 4, *kind);
        }
        list
    }

    /// Calls `ParseParameters` with `outputs`.
    fn parse(
        e: &mut Engine,
        parameters: u32,
        code: u32,
        offset: u32,
        script: u32,
        locals: u32,
        outputs: &[u32],
    ) -> bool {
        let mut words = args![parameters, code, offset, 0xaaa0u32, 0xbbb0u32, script, locals];
        words.extend_from_slice(outputs);
        e.call(0x005a_ccb0, &words).bool()
    }

    #[test]
    fn parse_parameters_stores_plain_values_when_the_flag_is_set() {
        let mut e = operand_engine(1);
        let types = [0, 8, 5, 2, 1, 0x20, 0xa];
        let parameters = parameter_list(&mut e, &types);
        let mut bytes = vec![7u8, 0]; // count
        bytes.extend_from_slice(&[3, 0, b'a', b'b', b'c']); // string "abc"
        bytes.push(0x2a); // byte
        bytes.extend_from_slice(&(-2i16).to_le_bytes()); // short
        bytes.push(b'n');
        bytes.extend_from_slice(&3i32.to_le_bytes()); // float operand 3
        bytes.push(b'n');
        bytes.extend_from_slice(&(-5i32).to_le_bytes()); // int operand -5
        bytes.extend_from_slice(&0x1234u16.to_le_bytes()); // u16 -> byte
        bytes.extend_from_slice(&(-9i16).to_le_bytes());
        let (code, offset) = code_with(&mut e, &bytes);
        let this = e.new_object::<Script>();
        let locals = locals(&mut e);
        let cells: Vec<u32> = (0..7).map(|_| e.mem.alloc(8)).collect();
        assert!(parse(
            &mut e,
            parameters,
            code,
            offset,
            this.addr(),
            locals.addr(),
            &cells
        ));
        assert_eq!(e.mem.cstr(cells[0]), b"abc".to_vec());
        assert_eq!(e.mem.u8(cells[1]), 0x2a);
        assert_eq!(e.mem.i32(cells[2]), -2);
        assert_eq!(e.mem.f32(cells[3]), 3.0);
        assert_eq!(e.mem.i32(cells[4]), -5);
        assert_eq!(e.mem.u8(cells[5]), 0x34);
        assert_eq!(e.mem.i32(cells[6]), -9);
        assert_eq!(e.mem.u32(offset) as usize, bytes.len());
    }

    #[test]
    fn parse_parameters_only_skips_without_the_flag() {
        let mut e = operand_engine(0);
        let parameters = parameter_list(&mut e, &[0, 8, 5, 1]);
        let mut bytes = vec![4u8, 0, 2, 0, b'h', b'i', 9];
        bytes.extend_from_slice(&7i16.to_le_bytes());
        bytes.push(b'n');
        bytes.extend_from_slice(&1i32.to_le_bytes());
        let (code, offset) = code_with(&mut e, &bytes);
        let this = e.new_object::<Script>();
        let locals = locals(&mut e);
        // No output pointers at all: nothing is stored, the data is skipped.
        assert!(parse(
            &mut e,
            parameters,
            code,
            offset,
            this.addr(),
            locals.addr(),
            &[]
        ));
        assert_eq!(e.mem.u32(offset) as usize, bytes.len());
        // A null parameter list: false and nothing read.
        e.mem.set_u32(offset, 0);
        assert!(!parse(
            &mut e,
            0,
            code,
            offset,
            this.addr(),
            locals.addr(),
            &[]
        ));
        assert_eq!(e.mem.u32(offset), 0);
    }

    #[test]
    fn parse_parameters_fails_on_unsupported_types_and_failed_operands() {
        let mut e = operand_engine(1);
        let this = e.new_object::<Script>();
        let locals = locals(&mut e);
        // Type 3 is not a plain type.
        let parameters = parameter_list(&mut e, &[3]);
        let (code, offset) = code_with(&mut e, &[1, 0]);
        assert!(!parse(
            &mut e,
            parameters,
            code,
            offset,
            this.addr(),
            locals.addr(),
            &[0]
        ));
        // An operand that cannot be read ('q' is not an operand).
        let parameters = parameter_list(&mut e, &[2]);
        let (code, offset) = code_with(&mut e, &[1, 0, b'q', 0, 0]);
        let cell = e.mem.alloc(4);
        assert!(!parse(
            &mut e,
            parameters,
            code,
            offset,
            this.addr(),
            locals.addr(),
            &[cell]
        ));
    }

    /// Marks `kinds` as referenced-object parameter types.
    fn mark_reference_types(e: &mut Engine, kinds: &[u32]) {
        for kind in kinds {
            e.mem.set_u8(PARAMETER_TYPE_TABLE + kind * 8, 1);
        }
    }

    #[test]
    fn parse_parameters_takes_referenced_objects_by_form_type() {
        let mut e = operand_engine(1);
        mark_reference_types(&mut e, &[0xe, 0x6, 0x16]);
        let locals = locals(&mut e);
        let form = e.mem.alloc(0x20);
        e.mem.set_u8(form + 4, 0x47);
        let (script, _) = script_with_object(&mut e, form, 0);
        let (code, offset) = code_with(&mut e, &[1, 0, b'r', 1, 0]);
        let cell = e.mem.alloc(4);
        let parameters = parameter_list(&mut e, &[0xe]);
        assert!(parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        assert_eq!(e.mem.u32(cell), form);
        // The wrong form type is rejected, and nothing is stored.
        e.mem.set_u8(form + 4, 0x20);
        e.mem.set_u32(cell, 0);
        e.mem.set_u32(offset, 0);
        assert!(!parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        assert_eq!(e.mem.u32(cell), 0);
        // Type 6 takes the two types 0x3b and 0x3c.
        let parameters = parameter_list(&mut e, &[0x6]);
        e.mem.set_u8(form + 4, 0x3c);
        e.mem.set_u32(offset, 0);
        assert!(parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        assert_eq!(e.mem.u32(cell), form);
        e.mem.set_u8(form + 4, 0x3d);
        e.mem.set_u32(offset, 0);
        assert!(!parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        // An unimplemented referenced-object type is logged.
        let parameters = parameter_list(&mut e, &[0x16]);
        e.mem.set_u32(offset, 0);
        e.call_log = Some(vec![]);
        assert!(!parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SCRIPT_LOG),
            vec![vec![FORMAT_PARAMETER_TYPE_UNIMPLEMENTED, 0x16]]
        );
    }

    #[test]
    fn parse_parameters_rejects_bad_referenced_objects() {
        let mut e = operand_engine(1);
        mark_reference_types(&mut e, &[0xe]);
        let locals = locals(&mut e);
        let parameters = parameter_list(&mut e, &[0xe]);
        let cell = e.mem.alloc(4);
        let form = e.mem.alloc(0x20);
        e.mem.set_u8(form + 4, 0x47);
        let (script, object) = script_with_object(&mut e, form, 0);
        // No 'r' marker.
        let (code, offset) = code_with(&mut e, &[1, 0, b'x', 1, 0]);
        assert!(!parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        // An index past the objects.
        let (code, offset) = code_with(&mut e, &[1, 0, b'r', 2, 0]);
        assert!(!parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        // An object with no form.
        let (code, offset) = code_with(&mut e, &[1, 0, b'r', 1, 0]);
        e.mem.set_u32(object + 8, 0);
        assert!(!parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        // Without the flag, a referenced object always fails.
        e.mem.set_u32(object + 8, form);
        e.register(FORM_FLAG_BIT_3, |_, _| ret(0));
        e.mem.set_u32(offset, 0);
        assert!(!parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
    }

    #[test]
    fn parse_parameters_container_and_cast_types() {
        let mut e = operand_engine(1);
        mark_reference_types(&mut e, &[0x3, 0x7, 0x15, 0x3d, 0x23]);
        let locals = locals(&mut e);
        let can_hold = fake_code(&mut e, |_, _| ret(1));
        let form = fake_object(&mut e, &[(0xe8, can_hold)]);
        e.mem.set_u8(form + 4, 0x2a);
        let (script, _) = script_with_object(&mut e, form, 0);
        let (code, offset) = code_with(&mut e, &[1, 0, b'r', 1, 0]);
        let cell = e.mem.alloc(4);
        // Type 3: a container type, and the form answers its virtual +0xE8.
        e.register(CONTAINER_CAN_HOLD_TYPE, |_, _| ret(1));
        let parameters = parameter_list(&mut e, &[0x3]);
        assert!(parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        assert_eq!(e.mem.u32(cell), form);
        // Type 3 with a type that cannot hold anything: false.
        e.register(CONTAINER_CAN_HOLD_TYPE, |_, _| ret(0));
        e.mem.set_u32(offset, 0);
        assert!(!parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        // Type 0x15: the virtual +0xE8 alone decides.
        let parameters = parameter_list(&mut e, &[0x15]);
        e.mem.set_u32(offset, 0);
        e.mem.set_u32(cell, 0);
        assert!(parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        assert_eq!(e.mem.u32(cell), form);
        // Type 0x3d takes any form.
        let parameters = parameter_list(&mut e, &[0x3d]);
        e.mem.set_u32(offset, 0);
        assert!(parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        // Type 0x23 takes types 8 and 0x2a.
        let parameters = parameter_list(&mut e, &[0x23]);
        e.mem.set_u32(offset, 0);
        assert!(parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        e.mem.set_u8(form + 4, 0x30);
        e.mem.set_u32(offset, 0);
        assert!(!parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        // Type 7 casts to its class; a failed cast falls back to a type 0x19
        // form whose list at +0x74 has a next node.
        let parameters = parameter_list(&mut e, &[0x7]);
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0] + 1));
        e.mem.set_u32(offset, 0);
        assert!(parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        assert_eq!(e.mem.u32(cell), form + 1);
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        e.mem.set_u8(form + 4, 0x19);
        e.mem.set_u32(offset, 0);
        assert!(!parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
        assert_eq!(e.mem.u32(cell), form);
        let node = e.mem.alloc(8);
        let next = e.mem.alloc(8);
        e.mem.set_u32(node + 4, next);
        e.mem.set_u32(form + 0x74, 0);
        // The list node is embedded at +0x74 of the form: give it a next.
        e.mem.set_u32(form + 0x74 + 4, next);
        e.mem.set_u32(offset, 0);
        assert!(parse(
            &mut e,
            parameters,
            code,
            offset,
            script.addr(),
            locals.addr(),
            &[cell]
        ));
    }

    #[test]
    fn parse_parameters_entry_needs_its_fixed_words() {
        let mut e = operand_engine(0);
        let parameters = parameter_list(&mut e, &[]);
        let (code, offset) = code_with(&mut e, &[0, 0]);
        let this = e.new_object::<Script>();
        let locals = locals(&mut e);
        // Count 0: true with no outputs at all.
        assert!(parse(
            &mut e,
            parameters,
            code,
            offset,
            this.addr(),
            locals.addr(),
            &[]
        ));
    }

    // PC 005a9db0: eighteen native FCW 027F / MXCSR 1F80 vectors.
    #[test]
    fn save_game_matches_native_numeric_reference_and_guard_vectors() {
        for bits in [
            0u64,
            0x8000000000000000,
            42.5f64.to_bits(),
            0x7ff0000000000000,
            0xfff0000000000000,
            1,
            0x7ff8000000001234,
            0x7ff0000000001234,
            0xfff0000000002345,
        ] {
            for flags in [0u32, 0x1000] {
                let mut e = Engine::new();
                for global in [0x011de134, 0x011ddf38, 0x011c3f2c] {
                    e.map(global, 4);
                }
                let base = e.mem.alloc(0x200);
                e.mem.set_u32(0x011de134, base + 0x130);
                e.mem.set_u32(0x011ddf38, base + 0x140);
                e.mem.set_u32(0x011c3f2c, 0);
                let mut bytes = vec![0xa5; 0x200];
                for (offset, word) in [
                    (0x10, base + 0x50),
                    (0x18, base + 0xd0),
                    (0x1c, base + 0xc0),
                    (0x20, 0),
                    (0x30, 0x01082324),
                    (0x34, base + 0x180),
                    (0x38, 64),
                    (0x3c, 0),
                    (0x40, 0),
                    (0x94, base + 0xb0),
                    (0x98, 0),
                    (0xbc, 2),
                    (0xc0, base + 0xf0),
                    (0xc4, base + 0xc8),
                    (0xc8, base + 0x110),
                    (0xcc, 0),
                    (0xd0, base + 0xe0),
                    (0xd4, 0),
                    (0xe0, 0),
                    (0xe4, flags),
                    (0xf0, 1),
                    (0x110, 2),
                    (0x148, 0),
                ] {
                    bytes[offset..offset + 4].copy_from_slice(&word.to_le_bytes());
                }
                bytes[0x13c] = 0;
                bytes[0xf8..0x100].copy_from_slice(&bits.to_le_bytes());
                bytes[0x118..0x120].copy_from_slice(&0xff123456u64.to_le_bytes());
                e.mem.write(base, &bytes);
                // Ordered positive/negative zero are omitted; unordered NaNs
                // are included with the source bits unchanged, including sNaNs.
                let zero = matches!(bits, 0 | 0x8000000000000000);
                let mut stream = if zero {
                    vec![4, 0x7c]
                } else {
                    let mut data = vec![8, 0x7c, 1, 0, 0, 0, 0x7c];
                    data.extend_from_slice(&bits.to_le_bytes());
                    data.push(0x7c);
                    data
                };
                stream.extend_from_slice(&[
                    2,
                    0,
                    0,
                    0x80,
                    0x7c,
                    0x92,
                    0x34,
                    0x56,
                    0x7c,
                    0,
                    0x7c,
                    (flags != 0) as u8,
                    0x7c,
                ]);
                let mut expected = bytes;
                expected[0x180..0x180 + stream.len()].copy_from_slice(&stream);
                expected[0x3c..0x40].copy_from_slice(&(stream.len() as u32).to_le_bytes());
                expected[0x40..0x44].copy_from_slice(&(if zero { 5u32 } else { 7 }).to_le_bytes());
                e.call(0x005a9db0, &[base + 0x10, base + 0x30]);
                assert_eq!(
                    e.mem.bytes(base, 0x200),
                    expected,
                    "{bits:016x}/{flags:04x}"
                );
            }
        }
    }

    // PC 005a9f20: nine native FCW 027F / MXCSR 1F80 generated-buffer vectors.

    #[test]
    fn load_game_matches_native_numeric_reference_and_guard_vectors() {
        for (input, output) in [
            (42.5f64.to_bits(), 42.5f64.to_bits()),
            (0x7ff0000000001234, 0x7ff8000000001234),
            (0xfff0000000002345, 0xfff8000000002345),
        ] {
            for fill in [0xa5, 0xcc, 0xff] {
                let mut e = Engine::new();
                e.map(0x01082000, 0x1000);
                e.mem.set_u32(0x01082028, 0x008a81c0);
                let base = e.mem.alloc(0x140);
                let mut bytes = vec![fill; 0x140];
                for (offset, value) in [
                    (0x10, 0),
                    (0x18, base + 0x90),
                    (0x1c, base + 0x70),
                    (0x20, base + 0x120),
                    (0x30, 0x01082028),
                    (0x34, base + 0xf0),
                    (0x38, 29),
                    (0x3c, 0),
                    (0x70, base + 0xa0),
                    (0x74, base + 0x78),
                    (0x78, base + 0xc0),
                    (0x7c, 0),
                    (0x90, base + 0x110),
                    (0x94, 0),
                    (0xa0, 1),
                    (0xc0, 2),
                    (0x110, 0x76543210),
                    (0x114, u32::MAX),
                ] {
                    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
                }
                bytes[0x4c] = 0x1b;
                bytes[0xb0] = 0x7f;
                bytes[0xd0] = 0xff;
                bytes[0xa8..0xb0].copy_from_slice(&(-1f64).to_le_bytes());
                bytes[0xc8..0xd0].copy_from_slice(&(-2f64).to_le_bytes());
                let mut stream = vec![8, 0x7c, 1, 0, 0, 0, 0x7c];
                stream.extend_from_slice(&input.to_le_bytes());
                stream.extend_from_slice(&[
                    0x7c, 2, 0, 0, 0x80, 0x7c, 0x92, 0x34, 0x56, 0x7c, 0, 0x7c, 0, 0x7c,
                ]);
                assert_eq!(stream.len(), 29);
                bytes[0xf0..0x10d].copy_from_slice(&stream);
                e.mem.write(base, &bytes);
                let mut expected = bytes.clone();
                expected[0x3c..0x40].copy_from_slice(&29u32.to_le_bytes());
                expected[0xa8..0xb0].copy_from_slice(&output.to_le_bytes());
                expected[0xc8..0xd0].copy_from_slice(&0xff123456u64.to_le_bytes());
                expected[0x114..0x118].copy_from_slice(&0u32.to_le_bytes());

                e.call(0x005a9f20, &[base + 0x10, base + 0x30]);
                let actual = e.mem.bytes(base, 0x140);
                assert_eq!(
                    actual, expected,
                    "unexpected mismatch for {input:016x}, fill{fill:02x}"
                );
            }
        }
    }
}
