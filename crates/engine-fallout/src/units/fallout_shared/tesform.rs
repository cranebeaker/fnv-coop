//! `fallout shared/tesform.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESForm` itself (vtable, `cFormType`, `iFormFlags`, `iFormID`, the source
//! file list) is [`crate::types::TESForm`]. The unit also owns the form
//! registries the constructor creates on first use and the destructor
//! destroys with the last form:
//!
//! - the form map (`FORM_MAP`, `NiTPointerMap<unsigned int, TESForm *>`):
//!   form ID to form;
//! - the editor ID map (`EDITOR_ID_MAP`): editor ID string to form;
//! - the form array (`FORM_LIST`, `BSSimpleArray`-shaped: items at +4, count
//!   at +0x0C) of the forms that carry flag 2 (`fn_00484730`).
//!
//! Done: every function of the unit, `00483370` to `00486eb0`. The third
//! session added `AddCompileIndex`, the component `DATA` chunk code
//! (`__SaveData`, `LoadData`, `fn_00486620`; the eight component kinds are
//! `Components`), the form copier `fn_004867a0`, the form-type string lookup
//! and the destructors of the registries' map classes.
//!
//! The record writer (`StartForm`, `CloseForm`, the `AddChunk` family) works
//! on the buffer in `SAVE_BUFFER` / `SAVE_BUFFER_SIZE`: a 0x18 byte record
//! header (type tag at +0, data size at +4, flags at +8, form ID at +0x0C,
//! version-control word at +0x10, version at +0x14 as a `u16`, one more
//! `u16` at +0x16), followed by chunks of a 4 byte tag, a `u16` size and the
//! data. All chunk writers end in [`tes_form_add_chunk_data`].
//!
//! The form's source file list (`BSSimpleList` at +0x10) is walked by node:
//! a node is an item word at +0 and a next pointer at +4, and the list's
//! first node is embedded in the form; [`list_first`], [`list_item`] and
//! [`list_next`] wrap the game's own accessors.
//!
//! The flag setters share two helpers, [`set_form_flag`] and
//! [`notify_changed`] (the form's virtual at +0x48 called with 1).
//!
//! Not translated: the compiler's exception-unwinding frames and SEH
//! registration in the functions that have them (`00483370`, `00483630`,
//! `00483720`, `00483870`, `00483a50`).

use crate::prelude::*;
use crate::types::TESForm;

// ---- globals of this unit ----
/// Number of live `TESForm` objects (the registries exist while it is not 0).
const FORM_COUNT: u32 = 0x011c_54bc;
/// Byte: the form-type string table has been checked and packed.
const FORM_TABLE_CHECKED: u32 = 0x011c_54b8;
/// Form ID to form map (pointer).
const FORM_MAP: u32 = 0x011c_54c0;
/// Editor ID to form map (pointer).
const EDITOR_ID_MAP: u32 = 0x011c_54c8;
/// Array of forms (pointer; items at +4, count at +0x0C).
const FORM_LIST: u32 = 0x011c_54c4;
/// The save writer's current record buffer (pointer) and its size in bytes.
const SAVE_BUFFER: u32 = 0x011c_54cc;
const SAVE_BUFFER_SIZE: u32 = 0x011c_54d0;
/// `TESDataHandler` instance (pointer).
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `TESSaveLoadGame` instance (pointer).
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// `BGSSaveLoadGame` instance (pointer).
const BGS_SAVE_LOAD_GAME: u32 = 0x011d_df38;

/// The form-type table: 0x79 entries of 12 bytes, each a form type byte at
/// +0, the type's 4-character string (pointer) at +4 and that string packed
/// into a word at +8 (the constructor fills the packed word in).
const FORM_ENUM_TABLE: u32 = 0x0118_7000;
const FORM_ENUM_COUNT: u32 = 0x79;
const FORM_ENUM_STRIDE: u32 = 12;
/// The packed string of the table's third entry, the tag of a group record
/// (`fn_00484150` compares a record's first word against it).
const GROUP_TAG: u32 = 0x0118_7020;

/// `TESForm`'s vtable.
const TES_FORM_VTABLE: u32 = 0x0101_c794;
/// Messages: `"FORMS: formEnumString[ %d ].cFormID in TESForm.cpp is out of
/// order."`, the duplicate-string message, the fatal error text, the zlib
/// version `"1.2.1"` and the two zlib error messages.
const FORM_ENUM_ERROR_ORDER: u32 = 0x0101_c748;
const FORM_ENUM_ERROR_DUPLICATE: u32 = 0x0101_c6e0;
const FORM_ENUM_FIX_MESSAGE: u32 = 0x0101_c6a0;
const ZLIB_VERSION: u32 = 0x0101_a29c;
const ZLIB_INIT_ERROR: u32 = 0x0101_c8f4;
const ZLIB_DEFLATE_ERROR: u32 = 0x0101_c8cc;

// ---- callees outside this file ----
/// `BSSimpleList` constructor (empties the list).
const SIMPLE_LIST_CONSTRUCT: u32 = 0x0096_a2d0;
/// `BSSimpleList` destructor (frees every node).
const SIMPLE_LIST_DESTROY: u32 = 0x0047_0470;
/// Destructor wrapper of the same list (`0046ffb0` calls `00470470`).
const SIMPLE_LIST_DESTROY_WRAPPER: u32 = 0x0046_ffb0;
/// Base class constructor step: stores the base vtable (`00402d30`).
const BASE_CONSTRUCT: u32 = 0x0040_2d30;
/// `operator new(size)` and `operator delete(block)`.
const OPERATOR_NEW: u32 = 0x0040_1000;
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `memcpy(dst, src, size)` (the game's wrapper).
const MEMCPY: u32 = 0x0040_1460;
/// `Swap32(pointer, 0)`: byte-swaps the word at `pointer`.
const SWAP_WORD: u32 = 0x0040_1080;
/// The big-endian target flag (`MOV AL,[011c54ba]`).
const IS_BIG_ENDIAN: u32 = 0x0040_1500;
/// `FORM::Endian(header)`: swaps a record header in place.
const FORM_ENDIAN: u32 = 0x0047_0650;
/// `TESForm::cFormType` getter (`MOVZX EAX,byte [ECX+4]`).
const FORM_TYPE: u32 = 0x0040_1170;
/// Whether `iFormFlags & 0x4000` (temporary) is set.
const IS_TEMPORARY: u32 = 0x0040_77c0;
/// Whether `iFormFlags & 2` is set.
const IS_FLAG_2: u32 = 0x0046_0340;
/// Whether `iFormFlags & 0x20` (deleted) is set.
const IS_DELETED: u32 = 0x0044_0d80;
/// `MOV EAX,[ECX+0x0C]`: the form ID of a form, or the item count of the
/// form array.
const WORD_AT_0C: u32 = 0x0084_e3a0;
/// Address of the item at an index of the form array (`data + 4 * index`).
const LIST_ITEM_ADDRESS: u32 = 0x0087_7a30;
/// Form array: remove (clear) the item at an index.
const FORM_LIST_REMOVE_AT: u32 = 0x0048_6b80;
/// Form array: add an item (argument: address of the item pointer).
const FORM_LIST_ADD: u32 = 0x0047_0140;
/// Form array constructor (capacity, grow flag).
const FORM_LIST_CONSTRUCT: u32 = 0x0047_01d0;
/// Form array: clear (sets every item to null; `00863db0`).
const FORM_LIST_CLEAR: u32 = 0x0086_3db0;
/// Form map constructor (bucket count).
const FORM_MAP_CONSTRUCT: u32 = 0x0048_6930;
/// Editor ID map constructor (bucket count, copy-key flag).
const EDITOR_ID_MAP_CONSTRUCT: u32 = 0x0048_6ab0;
/// Map: remove every entry (`00438af0`).
const MAP_CLEAR: u32 = 0x0043_8af0;
/// Map: look up a key (`MAP_LOOKUP(map, key, &result)`, returns found).
const MAP_LOOKUP: u32 = 0x0085_3130;
/// Map: remove a key (`NiTMapBase::RemoveAt`).
const MAP_REMOVE_AT: u32 = 0x0040_5430;
/// Map: insert (`NiTMapBase::SetAt(key, value)`).
const MAP_SET_AT: u32 = 0x0084_4700;
/// `TESDataHandler::GetNextID`.
const DATA_HANDLER_GET_NEXT_ID: u32 = 0x0046_9800;
/// `TESDataHandler::RemoveIDFromDataHandler(id)`.
const DATA_HANDLER_REMOVE_ID: u32 = 0x0046_96f0;
/// Returns the byte at +0x620 of the data handler (`00482f20`).
const DATA_HANDLER_FLAG: u32 = 0x0048_2f20;
/// `TESSaveLoadGame::RemoveChanges(form, flags)` and `00856c70`'s partner.
const SAVE_LOAD_REMOVE_CHANGES: u32 = 0x0085_6c70;
const SAVE_LOAD_REMOVE_FORM: u32 = 0x0085_a410;
/// `BGSSaveLoadGame::RemoveChanges(form)`.
const BGS_REMOVE_CHANGES: u32 = 0x0084_a7c0;
/// `BGSSaveLoadGame` add change (form, change flags, force) and its sibling.
const BGS_ADD_CHANGE: u32 = 0x0084_a690;
const BGS_CHANGE_SIBLING: u32 = 0x0084_a780;
/// Stores its argument at `*this` (a one-word value's constructor).
const STORE_WORD: u32 = 0x008c_71b0;
/// `TESFile::StartForm(file, form)`, `TESFile::AddTESForm(file, form)` and
/// the call the file makes after a deleted form's start (`00473090`).
const FILE_START_FORM: u32 = 0x0047_2e60;
const FILE_ADD_FORM: u32 = 0x0047_2fe0;
const FILE_AFTER_START: u32 = 0x0047_3090;
/// `TESForm::SetFile(form, file)`, `TESForm::SetFormID(form, id, flag)` and
/// `TESForm::GetFormTypeFromFormString(string)` (all in this unit, later).
const SET_FILE: u32 = 0x0048_4f50;
const SET_FORM_ID: u32 = 0x0048_5c10;
const GET_FORM_TYPE_FROM_FORM_STRING: u32 = 0x0048_6890;
/// `FormComponentCollection` constructor and `InitFormComponentArray(form)`.
const COMPONENTS_CONSTRUCT: u32 = 0x0047_c050;
const COMPONENTS_INIT: u32 = 0x0047_c070;
/// Calls virtual slot 0 of every component, and virtual slot 1.
const COMPONENTS_VISIT_FIRST: u32 = 0x0047_c560;
const COMPONENTS_VISIT_SECOND: u32 = 0x0047_c5b0;
/// A `FormComponentCollection` is 0x26 pointers.
const COMPONENT_COLLECTION_SIZE: u32 = 0x98;
/// Clears a sub-object at +4 of the argument (`0048ce80`).
const CLEAR_SUB_OBJECT: u32 = 0x0048_ce80;
/// The save/load game stub that returns false (`0047c850`).
const SAVE_LOAD_STUB: u32 = 0x0047_c850;
/// Game messages: printf-style log (cdecl) and the fatal error.
const LOG_MESSAGE: u32 = 0x005b_5e40;
const FATAL_ERROR: u32 = 0x0040_fbe0;
const DISABLE_WARNING_COUNT: u32 = 0x0043_b2b0;
/// zlib 1.2.1: `deflateInit_(stream, level, version, stream_size)`,
/// `deflate(stream, flush)`, `deflateEnd(stream)`.
const DEFLATE_INIT: u32 = 0x00b4_6360;
const DEFLATE: u32 = 0x00b4_6b70;
const DEFLATE_END: u32 = 0x00b4_7370;

// ---- callees outside this file, second batch ----
/// `__RTDynamicCast(object, vfdelta, source type, target type, is_reference)`
/// (cdecl) and the type descriptors the forms are cast between.
const RT_DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// Type descriptor `.?AVTESForm@@`.
const TYPE_TES_FORM: u32 = 0x0118_3028;
/// Type descriptor `.?AVBaseFormComponent@@`.
const TYPE_BASE_FORM_COMPONENT: u32 = 0x0118_3040;
/// Type descriptor `.?AVTESObjectREFR@@`.
const TYPE_TES_OBJECT_REFR: u32 = 0x0118_41cc;
/// `TESSaveLoadGame` methods the form's old save/load helpers forward to:
/// `SaveGameDataOLD(buffer, size)`, `LoadGameDataOLD(buffer, size)`,
/// `SaveNumericID(a, b)` and `LoadNumericID(a, b)`.
const SAVE_LOAD_SAVE_DATA: u32 = 0x0085_79b0;
const SAVE_LOAD_LOAD_DATA: u32 = 0x0085_79e0;
const SAVE_LOAD_SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
const SAVE_LOAD_LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
/// On a save/load buffer object: stores the word at +0x17 (its save kind)
/// in the out parameter; and `bits_set(word pointer, mask)`.
const BUFFER_SAVE_KIND: u32 = 0x0042_8110;
const WORD_HAS_BITS: u32 = 0x0042_80f0;
/// Save buffer: `Write(data, size, 0)`. Load buffer: `Read(data, size)`.
const SAVE_BUFFER_WRITE: u32 = 0x0086_5e50;
const LOAD_BUFFER_READ: u32 = 0x0086_4980;
/// `MOV EAX,[ECX+8]`: the form's flags.
const FORM_FLAGS: u32 = 0x0044_ddc0;
/// The form type's string: `formEnumString[cFormType].string` (`+4` of the
/// form-type table entry).
const FORM_TYPE_NAME: u32 = 0x0044_0e30;
/// What `00474cb0` computes from the form's virtual at +0x130 (a string): 0
/// while it runs re-entrantly or when the string is null, else `0044a670` of
/// the string.
const FORM_NAME_KEY: u32 = 0x0047_4cb0;
/// `strcmp`-like comparison of two strings (cdecl, 0 when equal).
const STRING_COMPARE: u32 = 0x0040_4dc0;
/// `TESFile::LoadForm(file, form)`.
const FILE_LOAD_FORM: u32 = 0x0047_2f60;
/// `file + 0x3ec`: the address of a file's list of masters.
const FILE_MASTER_LIST: u32 = 0x0046_4df0;
/// The end of a file's chain (`TESFile` +4 repeatedly, to the last link whose
/// +4 is null); `SetFile` replaces the file it is given by this one.
const FILE_ROOT: u32 = 0x0047_3c70;
/// `TESFile::GetMaster` (Xbox PDB name).
const FILE_IS_MASTER: u32 = 0x0047_1c20;
/// `BSSimpleList` accessors: the first node of the form's list (`form +
/// 0x10`), a node's item address (the node itself), a node's next pointer
/// (+4) and whether a node is empty (no item and no next).
const LIST_FIRST_NODE: u32 = 0x0046_0140;
const LIST_NODE_ITEM: u32 = 0x0068_15c0;
const LIST_NODE_NEXT: u32 = 0x0072_6070;
const LIST_NODE_EMPTY: u32 = 0x0082_56d0;
/// `BSSimpleList` operations on a node: remove the node's own item (the next
/// node moves up; the last node is just cleared), remove the first node whose
/// item equals the argument item, append an item at the end, and push an item
/// at the front.
const LIST_POP_NODE: u32 = 0x0063_f7b0;
const LIST_REMOVE_ITEM: u32 = 0x0090_5330;
const LIST_APPEND: u32 = 0x0090_5820;
const LIST_PUSH_FRONT: u32 = 0x005a_e3d0;
/// `FormComponentCollection` copy and compare (the argument is the other
/// collection).
const COMPONENTS_COPY: u32 = 0x0047_c600;
const COMPONENTS_COMPARE: u32 = 0x0047_c670;
/// Returns the record version (0xf) the form writer stamps in a record.
const RECORD_VERSION: u32 = 0x0047_0bb0;
/// Byte-swaps the `u16` at the pointer (second argument 0).
const SWAP_HALF: u32 = 0x0040_7a90;
/// Swaps a chunk header in place (the word at +0, the `u16` at +4).
const SWAP_CHUNK_HEADER: u32 = 0x0041_41e0;
/// `realloc(block, size)`: the memory manager's resize.
const BUFFER_REALLOC: u32 = 0x0042_f5d0;
/// The log texts of `Copy` and `Compare` for forms that have no override.
const COPY_MESSAGE: u32 = 0x0101_c930;
const COMPARE_MESSAGE: u32 = 0x0101_c978;
/// The tag `"XXXX"` of the chunk written when a chunk is larger than 0xffff.
const OVERSIZE_CHUNK_TAG: u32 = 0x5858_5858;
/// `iFormFlags` bits `StartForm` keeps for a form of a type other than 1.
const START_FORM_FLAGS_MASK: u32 = 0x3003_2fe0;
/// `iFormFlags` bits `LoadGame` takes from the save: for a reference
/// (`TESObjectREFR`) and for any other form.
const LOAD_GAME_REFERENCE_FLAGS: u32 = 0x0091_2860;
const LOAD_GAME_FORM_FLAGS: u32 = 0x4000_0c20;
/// Flags `fn_004853e0` ignores when comparing two forms.
const COMPARE_FLAGS_MASK: u32 = 0xffff_bff4;
/// The form's virtual at +0xc8 (takes the other form's "flag 2" state when it
/// differs), at +0xdc (called by `StartForm` last), at +0xf0 (a boolean:
/// true selects the reference mask when flags are loaded) and at +0x130
/// (returns a string, used as the form's name).
const SLOT_SET_FLAG_2: u32 = 0xc8;
const SLOT_AFTER_START: u32 = 0xdc;
const SLOT_IS_REFERENCE: u32 = 0xf0;
const SLOT_NAME: u32 = 0x130;

// ---- third batch: the component save/load data, the form creator, the maps ----
/// The form components that `__SaveData` / `LoadData` / `fn_00486620` cast the
/// form to (RTTI type descriptors, `.?AV<name>@@`), in the order the code
/// lists them: `TESUsesForm`, `TESValueForm`, `TESHealthForm`,
/// `TESWeightForm`, `TESQualityForm`, `TESAttackDamageForm`, `TESAttributes`
/// and `BGSClipRoundsForm`.
const TYPE_USES_FORM: u32 = 0x0118_6b88;
const TYPE_VALUE_FORM: u32 = 0x0118_6b6c;
const TYPE_HEALTH_FORM: u32 = 0x0118_6c3c;
const TYPE_WEIGHT_FORM: u32 = 0x0118_3274;
const TYPE_QUALITY_FORM: u32 = 0x0118_6bdc;
const TYPE_ATTACK_DAMAGE_FORM: u32 = 0x0118_6c90;
const TYPE_ATTRIBUTES: u32 = 0x0118_6c74;
const TYPE_CLIP_ROUNDS_FORM: u32 = 0x0118_68d4;
/// The chunk tag `"DATA"` of the form's component data.
const DATA_CHUNK_TAG: u32 = 0x4154_4144;
/// `"MASTERFILE: Trying to SaveData that's too large."` and the format
/// `"%s Form '%s' (%08X)"` of `GetFormDetailedString`.
const SAVE_DATA_TOO_LARGE_MESSAGE: u32 = 0x0101_ca2c;
const DETAILED_STRING_FORMAT: u32 = 0x0101_ca60;
/// The record buffer's current size (`011c54d0`) and address (`011c54cc`) as
/// two getters (the receiver is ignored).
const SAVE_BUFFER_SIZE_GETTER: u32 = 0x0047_3080;
const SAVE_BUFFER_GETTER: u32 = 0x0047_3070;
/// The one-byte component accessors: the byte at +4 of the component
/// (`TESUsesForm`, `BGSClipRoundsForm`; the same body as [`FORM_TYPE`]) and
/// its setter.
const COMPONENT_BYTE_GET: u32 = FORM_TYPE;
const COMPONENT_BYTE_SET: u32 = 0x004f_15a0;
/// The word at +4 of `TESValueForm` / `TESHealthForm`, and their setters.
const COMPONENT_WORD_GET: u32 = 0x0072_6070;
const VALUE_SET: u32 = 0x0048_e960;
const HEALTH_SET: u32 = 0x006e_cd40;
/// `TESWeightForm`: the weight (a float returned in `ST0`) and its setter
/// (the float is one stack word).
const WEIGHT_GET: u32 = 0x006b_9130;
const WEIGHT_SET: u32 = 0x004f_5d90;
/// `TESQualityForm`: the quality (a getter returning a byte).
const QUALITY_GET: u32 = 0x004f_1540;
/// `TESAttackDamageForm`: the setter (a `u16`); the getter is the
/// component's virtual at +0x10.
const ATTACK_DAMAGE_SET: u32 = 0x0047_ff50;
const SLOT_ATTACK_DAMAGE_GET: u32 = 0x10;
/// `TESAttributes`: the size in bytes of its save data, its writer
/// `(attributes, destination)` and its reader `(attributes, source)` (both
/// return the number of bytes used).
const ATTRIBUTES_SIZE: u32 = 0x009d_2510;
const ATTRIBUTES_SAVE: u32 = 0x0048_0100;
const ATTRIBUTES_LOAD: u32 = 0x0048_0130;
/// On a `TESFile`: the size of the current chunk (`file + 0x25c`), reading
/// the current chunk's data `(file, buffer, size)` and the big-endian flag
/// (the byte at `file + 0x299`).
const FILE_CHUNK_SIZE: u32 = 0x0040_1660;
const FILE_READ_CHUNK: u32 = 0x0047_2890;
const FILE_IS_BIG_ENDIAN: u32 = 0x0040_1680;
/// `TESFile::GetIndexFile(file, index)` (Xbox PDB name) and a file's compile
/// index (a byte).
const FILE_GET_INDEX_FILE: u32 = 0x0047_1a10;
const FILE_COMPILE_INDEX: u32 = 0x0047_3250;
/// On `TESSaveLoadGame`: the remapping of a form ID that
/// `AddCompileIndex` applies while the save/load stub (`0047c850`) is true.
const SAVE_LOAD_REMAP_ID: u32 = 0x0085_7bf0;
/// `TESDataHandler::CreateFormOfType(type)` (Xbox PDB name, cdecl).
const CREATE_FORM_OF_TYPE: u32 = 0x0046_5110;
/// The getter `00401280` returns the constant string at `DEFAULT_NAME`,
/// which is also what `fn_004867a0` first hands the form's virtual at +0x134
/// (a virtual that takes a string).
const FORM_NAME_GETTER: u32 = 0x0040_1280;
const DEFAULT_NAME: u32 = 0x0101_1584;
const SLOT_TAKE_NAME: u32 = 0x134;
/// The new form's virtual at +0x108, called with the original form.
const SLOT_COPY_FROM: u32 = 0x108;
/// A string local of 8 bytes (a pointer and two `u16`s): its constructor
/// from a C string, the C string pointer it holds and its destructor.
const STRING_CONSTRUCT: u32 = 0x0040_c0e0;
const STRING_DATA: u32 = 0x0055_9450;
const STRING_DESTROY: u32 = 0x0040_37d0;
/// `sprintf(buffer, format, ...)` (cdecl).
const FORMAT_STRING: u32 = 0x0040_6f60;
/// `memset(buffer, value, size)` (cdecl) and the map bucket array's
/// allocator (cdecl, byte size).
const MEMSET: u32 = 0x0040_3d30;
const ALLOCATE_BUCKETS: u32 = 0x00aa_1070;
/// The base destructor steps of the map classes: of the form map and its
/// base (`00486a60`), of the case-insensitive string map (`00486c80`), of
/// `NiTMapBase<const char *, TESForm *>` (`00486dc0`), of
/// `NiTMap<const char *, TESForm *>` (`00486d60`) and of
/// `NiTStringTemplateMap` (`00486ca0`).
const MAP_BASE_DESTROY: u32 = 0x0048_6a60;
const STRING_MAP_BASE_DESTROY: u32 = 0x0048_6c80;
const CHAR_KEY_MAP_BASE_DESTROY: u32 = 0x0048_6dc0;
const NI_T_MAP_DESTROY: u32 = 0x0048_6d60;
const STRING_TEMPLATE_MAP_DESTROY: u32 = 0x0048_6ca0;
/// Map vtables: the map base, the `NiTPointerMap<unsigned int, TESForm *>`
/// and the case-insensitive string map.
const MAP_BASE_VTABLE: u32 = 0x0101_ca98;
const POINTER_MAP_VTABLE: u32 = 0x0101_ca78;
const STRING_MAP_VTABLE: u32 = 0x0101_cab8;
/// The cache of `GetFormTypeFromFormString`: the last result and the last
/// string looked up.
const FORM_TYPE_CACHE_RESULT: u32 = 0x011c_54f0;
const FORM_TYPE_CACHE_KEY: u32 = 0x011c_54f4;

// `iFormFlags` bits set or cleared by the setters in this file that have a
// named role.
const FLAG_TEMPORARY: u32 = 0x4000;
const FLAG_DELETED: u32 = 0x20;

/// Sets or clears `mask` in the form's flags.
fn set_form_flag(e: &mut Engine, this: Ptr<TESForm>, mask: u32, on: bool) {
    let flags = e.get(this, TESForm::iFormFlags);
    let flags = if on { flags | mask } else { flags & !mask };
    e.set(this, TESForm::iFormFlags, flags);
}

/// The form's virtual at +0x48 with argument 1 (the "form changed" hook the
/// setters call after changing a flag that is saved).
fn notify_changed(e: &mut Engine, this: Ptr<TESForm>) {
    e.vcall(this.addr(), 0x48, &args![1u32]);
}

/// Sort key of a form type, as the compiler inlined it into several
/// functions: types `0x67`, `0x6c`, `0x73`, `0x74` rank `0x137` to `0x13a`,
/// every other type ranks `10 * type`.
fn form_type_rank(form_type: u32) -> u32 {
    match form_type {
        0x67 => 0x137,
        0x6c => 0x138,
        0x73 => 0x139,
        0x74 => 0x13a,
        other => other.wrapping_mul(10),
    }
}

/// The rank of the form's own type.
fn form_rank(e: &mut Engine, form: Ptr<TESForm>) -> u32 {
    let form_type = e.call(FORM_TYPE, &args![form]).u32();
    form_type_rank(form_type)
}

/// Index of `form` in the form array, if the array exists and holds it.
fn find_in_form_list(e: &mut Engine, form: Ptr<TESForm>) -> Option<u32> {
    let mut index = 0u32;
    loop {
        let list = e.global::<u32>(FORM_LIST);
        if list == 0 {
            return None;
        }
        let count = e.call(WORD_AT_0C, &args![list]).u32();
        if index >= count {
            return None;
        }
        let item = e.call(LIST_ITEM_ADDRESS, &args![list, index]).u32();
        if e.mem.u32(item) == form.addr() {
            return Some(index);
        }
        index += 1;
    }
}

// Translated from 00483370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::TESForm` (Xbox PDB): sets up the base class and vtable, creates
/// the registries if this is the first form, checks and packs the form-type
/// string table once, gives the form flags 8 and, when the data handler
/// exists and its flag byte is clear, takes the next form ID (and the
/// handler's current file), then registers the form in the form map.
// Translation incomplete: 00483370 - compiler exception-unwind cleanup remains omitted.
pub fn tes_form_tes_form(e: &mut Engine, this: Ptr<TESForm>) -> Ptr<TESForm> {
    e.call(BASE_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TES_FORM_VTABLE);
    e.call(SIMPLE_LIST_CONSTRUCT, &args![this.byte_add(0x10)]);

    if e.global::<u32>(FORM_COUNT) == 0 {
        fn_00483a50(e);
    }
    let count = e.global::<u32>(FORM_COUNT);
    e.set_global(FORM_COUNT, count.wrapping_add(1));

    if e.global::<u8>(FORM_TABLE_CHECKED) == 0 {
        check_form_enum_table(e);
        e.set_global(FORM_TABLE_CHECKED, 1u8);
    }

    e.set(this, TESForm::cFormType, 0);
    e.set(this, TESForm::iFormFlags, 8);
    e.set(this, TESForm::iFormID, 0);

    let handler = e.global::<u32>(DATA_HANDLER);
    if handler != 0 && !e.call(DATA_HANDLER_FLAG, &args![handler]).bool() {
        // PC 00483555 reloads the singleton after the flag callback.
        let handler = e.global::<u32>(DATA_HANDLER);
        let id = e.call(DATA_HANDLER_GET_NEXT_ID, &args![handler]).u32();
        e.set(this, TESForm::iFormID, id);
        // PC 00483566 reloads again after GetNextID.
        let handler = e.global::<u32>(DATA_HANDLER);
        if fn_004835e0(e, handler) != 0 {
            let handler = e.global::<u32>(DATA_HANDLER);
            let file = fn_004835e0(e, handler);
            e.call(SET_FILE, &args![this, file]);
        }
        if e.get(this, TESForm::iFormID) < 0x800 {
            e.call(SET_FORM_ID, &args![this, 0x800u32, 1u32]);
        }
    }

    let id = e.get(this, TESForm::iFormID);
    if id != 0 {
        let map = e.global::<u32>(FORM_MAP);
        e.call(MAP_SET_AT, &args![map, id, this]);
    }
    this
}

/// The constructor's one-time check of the form-type table: packs each type's
/// four characters (sign-extended bytes, first character lowest) into the
/// entry's word at +8, reports an entry whose type byte is not its index and
/// entries with equal packed strings, and stops the game if any was reported.
fn check_form_enum_table(e: &mut Engine) {
    let mut failed = false;
    e.call(DISABLE_WARNING_COUNT, &args![1u32]);
    for i in 0..FORM_ENUM_COUNT {
        let entry = FORM_ENUM_TABLE + i * FORM_ENUM_STRIDE;
        let text = e.mem.u32(entry + 4);
        let packed = (e.mem.i8(text) as i32)
            | (e.mem.i8(text + 1) as i32) << 8
            | (e.mem.i8(text + 2) as i32) << 16
            | (e.mem.i8(text + 3) as i32) << 24;
        e.mem.set_i32(entry + 8, packed);
        if e.mem.u8(entry) as u32 != i {
            e.call(LOG_MESSAGE, &args![FORM_ENUM_ERROR_ORDER, i]);
            failed = true;
        }
        for j in 0..FORM_ENUM_COUNT {
            if i != j
                && e.mem.u32(entry + 8) == e.mem.u32(FORM_ENUM_TABLE + j * FORM_ENUM_STRIDE + 8)
            {
                e.call(LOG_MESSAGE, &args![FORM_ENUM_ERROR_DUPLICATE, i, j, text]);
                failed = true;
            }
        }
    }
    e.call(DISABLE_WARNING_COUNT, &args![0u32]);
    if failed {
        e.call(FATAL_ERROR, &args![FORM_ENUM_FIX_MESSAGE]);
    }
}

// Translated from 004835e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A `TESDataHandler` getter: the word at +0x20C (the file the handler is
/// currently loading, passed to `SetFile` by the form constructor).
pub fn fn_004835e0(e: &mut Engine, this: u32) -> u32 {
    e.mem.u32(this + 0x20c)
}

// Translated from 00483600 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::scalar deleting destructor`: runs the destructor and, when bit 0
/// of `flags` is set, frees the object.
pub fn tes_form_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<TESForm>,
    flags: u32,
) -> Ptr<TESForm> {
    tes_form_dtor(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00483630 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::~TESForm` (Xbox PDB): takes the form out of the registries and
/// the save/load change tables (unless it is temporary), destroys the source
/// file list and, with the last form, the registries.
pub fn tes_form_dtor(e: &mut Engine, this: Ptr<TESForm>) {
    e.mem.set_u32(this.addr(), TES_FORM_VTABLE);
    tes_form_remove_from_data_structures(e, this);
    e.call(SIMPLE_LIST_DESTROY, &args![this.byte_add(0x10)]);
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if !e.call(IS_TEMPORARY, &args![this]).bool() && save_load != 0 {
        e.call(SAVE_LOAD_REMOVE_CHANGES, &args![save_load, this, 0u32]);
        e.call(SAVE_LOAD_REMOVE_FORM, &args![save_load, this]);
    }
    let bgs = e.global::<u32>(BGS_SAVE_LOAD_GAME);
    if !e.call(IS_TEMPORARY, &args![this]).bool() && bgs != 0 {
        e.call(BGS_REMOVE_CHANGES, &args![bgs, this]);
    }
    let count = e.global::<u32>(FORM_COUNT).wrapping_sub(1);
    e.set_global(FORM_COUNT, count);
    if count == 0 {
        fn_00483b80(e);
    }
    e.call(SIMPLE_LIST_DESTROY_WRAPPER, &args![this.byte_add(0x10)]);
}

// Translated from 00483710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Does nothing (an empty base-class destructor the compiler left a call to).
pub fn fn_00483710(_e: &mut Engine, _this: u32) {}

// Translated from 00483720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Visits a form's components. Forms of types `0x11`, `0x3a` to `0x40`,
/// `0x42`, `0x43`, `0x49` and `0x69` have none; type `0x39` clears its
/// sub-object at +0x18; every other form builds its
/// `FormComponentCollection` on the stack and calls virtual slot 0 of each
/// component (`0047c560`). What the slot does is not named in the Xbox PDB.
pub fn fn_00483720(e: &mut Engine, this: Ptr<TESForm>) {
    let form_type = e.call(FORM_TYPE, &args![this]).u32();
    match form_type {
        0x11 | 0x3a..=0x40 | 0x42 | 0x43 | 0x49 | 0x69 => fn_00483710(e, this.addr()),
        0x39 => {
            e.call(CLEAR_SUB_OBJECT, &args![this.byte_add(0x18)]);
            fn_00483710(e, this.addr());
        }
        _ => e.with_stack(COMPONENT_COLLECTION_SIZE, |e, collection| {
            e.call(COMPONENTS_CONSTRUCT, &args![collection]);
            e.call(COMPONENTS_INIT, &args![collection, this]);
            e.call(COMPONENTS_VISIT_FIRST, &args![collection]);
            fn_00483710(e, collection.addr());
        }),
    }
}

// Translated from 00483870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like [`fn_00483720`] but calls virtual slot 1 of each component
/// (`0047c5b0`), and for type `0x39` only calls the empty `00483710`.
pub fn fn_00483870(e: &mut Engine, this: Ptr<TESForm>) {
    let form_type = e.call(FORM_TYPE, &args![this]).u32();
    match form_type {
        0x11 | 0x3a..=0x40 | 0x42 | 0x43 | 0x49 | 0x69 => fn_00483710(e, this.addr()),
        0x39 => {
            fn_00483710(e, this.byte_add(0x18).addr());
            fn_00483710(e, this.addr());
        }
        _ => e.with_stack(COMPONENT_COLLECTION_SIZE, |e, collection| {
            e.call(COMPONENTS_CONSTRUCT, &args![collection]);
            e.call(COMPONENTS_INIT, &args![collection, this]);
            e.call(COMPONENTS_VISIT_SECOND, &args![collection]);
            fn_00483710(e, collection.addr());
        }),
    }
}

// Translated from 004839c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Looks a form up by form ID in the form map; null when there is no map or
/// no such form.
pub fn fn_004839c0(e: &mut Engine, form_id: u32) -> Ptr<TESForm> {
    let map = e.global::<u32>(FORM_MAP);
    if map == 0 {
        return Ptr::NULL;
    }
    e.with_stack(4, |e, result| {
        e.mem.set_u32(result.addr(), 0);
        if e.call(MAP_LOOKUP, &args![map, form_id, result]).bool() {
            Ptr::new(e.mem.u32(result.addr()))
        } else {
            Ptr::NULL
        }
    })
}

// Translated from 00483a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::GetFormByEditorID` (Xbox PDB): looks a form up by editor ID in
/// the editor ID map; null for a null or empty name, no map, or no match.
pub fn tes_form_get_form_by_editor_id(e: &mut Engine, editor_id: Ptr) -> Ptr<TESForm> {
    if editor_id.is_null() || e.mem.i8(editor_id.addr()) == 0 {
        return Ptr::NULL;
    }
    let map = e.global::<u32>(EDITOR_ID_MAP);
    if map == 0 {
        return Ptr::NULL;
    }
    e.with_stack(4, |e, result| {
        e.mem.set_u32(result.addr(), 0);
        if e.call(MAP_LOOKUP, &args![map, editor_id, result]).bool() {
            Ptr::new(e.mem.u32(result.addr()))
        } else {
            Ptr::NULL
        }
    })
}

// Translated from 00483a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Creates the form map (0x2008d buckets), the editor ID map (0xfa1 buckets)
/// and the form array (capacity 0x20), each only if it does not exist yet.
// Translation incomplete: 00483a50 - compiler exception-unwind cleanup remains omitted.
pub fn fn_00483a50(e: &mut Engine) {
    if e.global::<u32>(FORM_MAP) == 0 {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let map = if block != 0 {
            e.call(FORM_MAP_CONSTRUCT, &args![block, 0x2008du32]).u32()
        } else {
            0
        };
        e.set_global(FORM_MAP, map);
    }
    if e.global::<u32>(EDITOR_ID_MAP) == 0 {
        let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
        let map = if block != 0 {
            e.call(EDITOR_ID_MAP_CONSTRUCT, &args![block, 0xfa1u32, 1u32])
                .u32()
        } else {
            0
        };
        e.set_global(EDITOR_ID_MAP, map);
    }
    if e.global::<u32>(FORM_LIST) == 0 {
        let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
        let list = if block != 0 {
            e.call(FORM_LIST_CONSTRUCT, &args![block, 0x20u32, 1u32])
                .u32()
        } else {
            0
        };
        e.set_global(FORM_LIST, list);
    }
}

// Translated from 00483b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the form map, the editor ID map and the form array (clear, then
/// the object's scalar deleting destructor, virtual slot 0 with argument 1)
/// and nulls the globals.
pub fn fn_00483b80(e: &mut Engine) {
    for (global, clear) in [
        (FORM_MAP, MAP_CLEAR),
        (EDITOR_ID_MAP, MAP_CLEAR),
        (FORM_LIST, FORM_LIST_CLEAR),
    ] {
        let object = e.global::<u32>(global);
        if object != 0 {
            e.call(clear, &args![object]);
            let object = e.global::<u32>(global);
            if object != 0 {
                e.vcall(object, 0, &args![1u32]);
            }
            e.set_global(global, 0u32);
        }
    }
}

// Translated from 00483c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::RemoveFromDataStructures` (Xbox PDB): unless the form is
/// temporary, removes its ID from the form map, its entry from the form
/// array and its ID from the data handler.
pub fn tes_form_remove_from_data_structures(e: &mut Engine, this: Ptr<TESForm>) {
    if e.call(IS_TEMPORARY, &args![this]).bool() {
        return;
    }
    let map = e.global::<u32>(FORM_MAP);
    if map != 0 {
        let id = e.get(this, TESForm::iFormID);
        e.call(MAP_REMOVE_AT, &args![map, id]);
    }
    if let Some(index) = find_in_form_list(e, this) {
        let list = e.global::<u32>(FORM_LIST);
        e.call(FORM_LIST_REMOVE_AT, &args![list, index]);
    }
    let handler = e.global::<u32>(DATA_HANDLER);
    if handler != 0 {
        let id = e.get(this, TESForm::iFormID);
        e.call(DATA_HANDLER_REMOVE_ID, &args![handler, id]);
    }
}

// Translated from 00483d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::Save` (Xbox PDB): false for a temporary form; otherwise calls
/// virtual slot 0x2c and adds the form to the file, true when the file's
/// `AddTESForm` returns 0.
pub fn tes_form_save(e: &mut Engine, this: Ptr<TESForm>, file: Ptr) -> bool {
    if e.call(IS_TEMPORARY, &args![this]).bool() {
        return false;
    }
    e.vcall(this.addr(), 0x2c, &[]);
    e.call(FILE_ADD_FORM, &args![file, this]).u32() == 0
}

// Translated from 00483d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::CompressSaveBuffer` (Xbox PDB): compresses the save writer's
/// current record body with zlib. The buffer is a 0x18-byte record header
/// followed by the body; when it is larger than the header and not already
/// compressed (flag 0x40000 in the header's flags at +8), it is replaced by
/// a new buffer holding the header (flagged, size = compressed size + 4),
/// the uncompressed size as a word and the deflate stream. On a big-endian
/// target the header is swapped around the changes and the size word is
/// swapped. A zlib failure is logged and leaves the buffer as it was.
pub fn tes_form_compress_save_buffer(e: &mut Engine) {
    let buffer = e.global::<u32>(SAVE_BUFFER);
    let size = e.global::<u32>(SAVE_BUFFER_SIZE);
    if buffer == 0 || size <= 0x18 {
        return;
    }
    if e.call(IS_BIG_ENDIAN, &[]).bool() {
        e.call(FORM_ENDIAN, &args![buffer]);
    }
    if e.mem.u32(buffer + 8) & 0x40000 != 0 {
        if e.call(IS_BIG_ENDIAN, &[]).bool() {
            e.call(FORM_ENDIAN, &args![buffer]);
        }
        return;
    }
    let body_size = size - 0x18;
    let body = buffer + 0x18;
    e.with_stack(0x38, |e, stream| {
        let stream = stream.addr();
        // z_stream: next_in +0, avail_in +4, next_out +0x0c, avail_out +0x10,
        // state +0x1c, zalloc +0x20, zfree +0x24, opaque +0x28.
        e.mem.set_u32(stream + 0x20, 0);
        e.mem.set_u32(stream + 0x24, 0);
        e.mem.set_u32(stream + 0x28, 0);
        e.mem.set_u32(stream + 0x1c, 0);
        let status = e
            .call(
                DEFLATE_INIT,
                &args![stream, 0xffff_ffffu32, ZLIB_VERSION, 0x38u32],
            )
            .i32();
        if status != 0 {
            e.call(LOG_MESSAGE, &args![ZLIB_INIT_ERROR]);
            return;
        }
        let capacity = body_size << 1;
        let packed = e.call(OPERATOR_NEW, &args![capacity]).u32();
        e.mem.set_u32(stream + 4, body_size);
        e.mem.set_u32(stream, body);
        e.mem.set_u32(stream + 0x10, capacity);
        e.mem.set_u32(stream + 0x0c, packed);
        let status = e.call(DEFLATE, &args![stream, 4u32]).i32();
        if status == -2 {
            e.call(LOG_MESSAGE, &args![ZLIB_DEFLATE_ERROR]);
            e.call(OPERATOR_DELETE, &args![packed]);
            return;
        }
        let packed_size = capacity.wrapping_sub(e.mem.u32(stream + 0x10));
        let total = packed_size + 0x1c;
        let new_buffer = e.call(OPERATOR_NEW, &args![total]).u32();
        let flags = e.mem.u32(buffer + 8) | 0x40000;
        e.mem.set_u32(buffer + 8, flags);
        e.mem.set_u32(buffer + 4, total - 0x18);
        if e.call(IS_BIG_ENDIAN, &[]).bool() {
            e.call(FORM_ENDIAN, &args![buffer]);
        }
        e.call(MEMCPY, &args![new_buffer, buffer, 0x18u32]);
        let mut cursor = new_buffer + 0x18;
        e.with_stack(4, |e, original_size| {
            e.mem.set_u32(original_size.addr(), body_size);
            if e.call(IS_BIG_ENDIAN, &[]).bool() {
                e.call(SWAP_WORD, &args![original_size, 0u32]);
            }
            e.call(MEMCPY, &args![cursor, original_size, 4u32]);
        });
        cursor += 4;
        e.call(MEMCPY, &args![cursor, packed, packed_size]);
        let old_buffer = e.global::<u32>(SAVE_BUFFER);
        e.call(OPERATOR_DELETE, &args![old_buffer]);
        e.set_global(SAVE_BUFFER, new_buffer);
        e.set_global(SAVE_BUFFER_SIZE, total);
        e.call(DEFLATE_END, &args![stream]);
        e.call(OPERATOR_DELETE, &args![packed]);
    });
}

// Translated from 00483fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Writes the form to `file`: false for a temporary form; for a deleted form
/// starts the form record in the file (`TESFile::StartForm`), calls
/// `00473090` on the file and returns true; otherwise returns what virtual
/// slot 0x28 returns for `file`.
pub fn fn_00483fc0(e: &mut Engine, this: Ptr<TESForm>, file: Ptr) -> bool {
    if e.call(IS_TEMPORARY, &args![this]).bool() {
        return false;
    }
    if e.call(IS_DELETED, &args![this]).bool() {
        e.call(FILE_START_FORM, &args![file, this]);
        e.call(FILE_AFTER_START, &args![file]);
        return true;
    }
    e.vcall(this.addr(), 0x28, &args![file]).bool()
}

// Translated from 00484020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether this form's type ranks before `other`'s in the file order
/// ([`form_type_rank`]).
pub fn fn_00484020(e: &mut Engine, this: Ptr<TESForm>, other: Ptr<TESForm>) -> bool {
    let other_rank = form_rank(e, other);
    let this_rank = form_rank(e, this);
    this_rank < other_rank
}

// Translated from 00484150 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether this form goes before the group record `group` (a null pointer or
/// a record whose first word is not the group tag gives false). The record's
/// word at +0x0C is the group type: 0 compares the form's rank with the rank
/// of the form type named by the label string at +8; types 1, 4 and 5 accept
/// ranks below 0x28a, types 2, 3 and 6 below 0x23a and type 7 below 0x2b2;
/// other group types give false.
pub fn fn_00484150(e: &mut Engine, this: Ptr<TESForm>, group: Ptr) -> bool {
    if group.is_null() || e.mem.u32(group.addr()) != e.global::<u32>(GROUP_TAG) {
        return false;
    }
    match e.mem.u32(group.addr() + 0xc) {
        0 => {
            let label = e.mem.u32(group.addr() + 8);
            let label_type = e.call(GET_FORM_TYPE_FROM_FORM_STRING, &args![label]).u32();
            let label_rank = form_type_rank(label_type);
            form_rank(e, this) < label_rank
        }
        1 | 4 | 5 => form_rank(e, this) < 0x28a,
        2 | 3 | 6 => form_rank(e, this) < 0x23a,
        7 => form_rank(e, this) < 0x2b2,
        _ => false,
    }
}

// Translated from 00484490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::SetTemporary` (Xbox PDB): removes the form from the registries
/// and the save/load change tables, then sets the temporary flag (0x4000).
pub fn tes_form_set_temporary(e: &mut Engine, this: Ptr<TESForm>) {
    tes_form_remove_from_data_structures(e, this);
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if save_load != 0 {
        e.call(SAVE_LOAD_REMOVE_CHANGES, &args![save_load, this, 0u32]);
    }
    let bgs = e.global::<u32>(BGS_SAVE_LOAD_GAME);
    if bgs != 0 {
        e.call(BGS_REMOVE_CHANGES, &args![bgs, this]);
    }
    set_form_flag(e, this, FLAG_TEMPORARY, true);
}

// Translated from 004844f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears form flag 0x1 (no notification).
pub fn fn_004844f0(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x1, on);
}

// Translated from 00484530 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::SetDelete` (Xbox PDB): sets or clears the deleted flag (0x20)
/// and calls the changed hook.
pub fn tes_form_set_delete(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, FLAG_DELETED, on);
    notify_changed(e, this);
}

// Translated from 00484580 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::SetEmpty` (Xbox PDB): sets or clears flag 0x2000. Unless the
/// save/load stub (`0047c850`, which returns false) says otherwise, it first
/// calls virtual slot 0x48 (set) or 0x4c (clear) with 0x200000.
pub fn tes_form_set_empty(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if !e.call(SAVE_LOAD_STUB, &args![save_load]).bool() {
        let slot = if on { 0x48 } else { 0x4c };
        e.vcall(this.addr(), slot, &args![0x20_0000u32]);
    }
    set_form_flag(e, this, 0x2000, on);
}

// Translated from 00484610 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears form flag 0x2000 (no notification).
pub fn fn_00484610(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x2000, on);
}

// Translated from 00484650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears form flag 0x800000 and calls the changed hook.
pub fn fn_00484650(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x80_0000, on);
    notify_changed(e, this);
}

// Translated from 004846a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::SetDestructible` (Xbox PDB): sets or clears flag 0x1000000.
pub fn tes_form_set_destructible(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x100_0000, on);
}

// Translated from 004846e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears form flag 0x4000000 and calls the changed hook.
pub fn fn_004846e0(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x400_0000, on);
    notify_changed(e, this);
}

// Translated from 00484730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears form flag 2 and keeps the form array in step. Clearing: if
/// the flag was set, the form's entry is removed from the form array. Setting:
/// if the flag was not set, the form is not temporary and the form array
/// exists, the form is added to it unless already listed.
pub fn fn_00484730(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    if on {
        if !e.call(IS_FLAG_2, &args![this]).bool()
            && !e.call(IS_TEMPORARY, &args![this]).bool()
            && e.global::<u32>(FORM_LIST) != 0
            && find_in_form_list(e, this).is_none()
        {
            let list = e.global::<u32>(FORM_LIST);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), this.addr());
                e.call(FORM_LIST_ADD, &args![list, slot]);
            });
        }
        set_form_flag(e, this, 2, true);
    } else {
        if e.call(IS_FLAG_2, &args![this]).bool() && e.global::<u32>(FORM_LIST) != 0 {
            if let Some(index) = find_in_form_list(e, this) {
                let list = e.global::<u32>(FORM_LIST);
                e.call(FORM_LIST_REMOVE_AT, &args![list, index]);
            }
        }
        set_form_flag(e, this, 2, false);
    }
}

// Translated from 00484860 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears form flag 0x400 and calls the changed hook.
pub fn fn_00484860(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x400, on);
    notify_changed(e, this);
}

// Translated from 004848b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears form flag 0x40 and calls the changed hook.
pub fn fn_004848b0(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x40, on);
    notify_changed(e, this);
}

// Translated from 00484900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears form flag 0x10000 and calls the changed hook.
pub fn fn_00484900(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x1_0000, on);
    notify_changed(e, this);
}

// Translated from 00484950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears form flag 0x20000 (no notification).
pub fn fn_00484950(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x2_0000, on);
}

// Translated from 00484990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears form flag 0x100000 (no notification).
pub fn fn_00484990(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x10_0000, on);
}

// Translated from 004849d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears form flag 0x200 (no notification).
pub fn fn_004849d0(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x200, on);
}

// Translated from 00484a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Same as [`fn_004848b0`] (flag 0x40, changed hook); the compiler kept a
/// copy of the new flags in a local that the hook call does not use.
pub fn fn_00484a10(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x40, on);
    notify_changed(e, this);
}

// Translated from 00484a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::SetFireOff` (Xbox PDB): sets or clears flag 0x80.
pub fn tes_form_set_fire_off(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x80, on);
}

// Translated from 00484ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears form flag 0x8 (no notification).
pub fn fn_00484ab0(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x8, on);
}

// Translated from 00484af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::SetDisabled` (Xbox PDB): sets or clears flag 0x800 and calls the
/// changed hook.
pub fn tes_form_set_disabled(e: &mut Engine, this: Ptr<TESForm>, on: bool) {
    set_form_flag(e, this, 0x800, on);
    notify_changed(e, this);
}

// Translated from 00484b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a form ID is in the range `1..=0x7ff` (cdecl, one argument).
pub fn fn_00484b40(_e: &mut Engine, form_id: u32) -> bool {
    form_id != 0 && form_id <= 0x7ff
}

/// The two words the change functions build on the stack: `00484b60` pushes
/// `extra`, makes room for a word, and has `008c71b0` store `changes` there.
fn change_words(e: &mut Engine, changes: u32, extra: u32) -> (u32, u32) {
    e.with_stack(8, |e, words| {
        e.mem.set_u32(words.addr() + 4, extra);
        e.call(STORE_WORD, &args![words, changes]);
        (e.mem.u32(words.addr()), e.mem.u32(words.addr() + 4))
    })
}

// Translated from 00484b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records a change of `changes` (change flags) on the form with the
/// save/load game's add-change function, with its last argument 0.
pub fn fn_00484b60(e: &mut Engine, this: Ptr<TESForm>, changes: u32) {
    let (changes, extra) = change_words(e, changes, 0);
    let bgs = e.global::<u32>(BGS_SAVE_LOAD_GAME);
    e.call(BGS_ADD_CHANGE, &args![bgs, this, changes, extra]);
}

// Translated from 00484b90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::ForceChange` (Xbox PDB): like [`fn_00484b60`] with the last
/// argument 1.
pub fn tes_form_force_change(e: &mut Engine, this: Ptr<TESForm>, changes: u32) {
    let (changes, extra) = change_words(e, changes, 1);
    let bgs = e.global::<u32>(BGS_SAVE_LOAD_GAME);
    e.call(BGS_ADD_CHANGE, &args![bgs, this, changes, extra]);
}

// Translated from 00484bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes the form and `changes` to the save/load game's sibling of the
/// add-change function (`0084a780`).
pub fn fn_00484bc0(e: &mut Engine, this: Ptr<TESForm>, changes: u32) {
    let value = e.with_stack(4, |e, word| {
        e.call(STORE_WORD, &args![word, changes]);
        e.mem.u32(word.addr())
    });
    let bgs = e.global::<u32>(BGS_SAVE_LOAD_GAME);
    e.call(BGS_CHANGE_SIBLING, &args![bgs, this, value]);
}

/// The first node of the form's source file list (the node embedded at +0x10).
fn list_first(e: &mut Engine, form: Ptr<TESForm>) -> u32 {
    e.call(LIST_FIRST_NODE, &args![form]).u32()
}

/// The item (a file pointer) of a list node: the word at the address the
/// game's accessor gives.
fn list_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(LIST_NODE_ITEM, &args![node]).u32();
    e.mem.u32(slot)
}

/// The node after `node` (null at the end).
fn list_next(e: &mut Engine, node: u32) -> u32 {
    e.call(LIST_NODE_NEXT, &args![node]).u32()
}

/// Runs `f` with a stack word holding `value`, the way the game passes the
/// address of a local item to the list operations.
fn with_item<R>(e: &mut Engine, value: u32, f: impl FnOnce(&mut Engine, Ptr) -> R) -> R {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), value);
        f(e, slot)
    })
}

/// Whether the target stores multi-byte values big-endian (`00401500`).
fn is_big_endian(e: &mut Engine) -> bool {
    e.call(IS_BIG_ENDIAN, &args![]).bool()
}

/// The packed string word of a form type's entry in the form-type table
/// (what a record header carries as its type).
fn form_type_tag(e: &mut Engine, form: Ptr<TESForm>) -> u32 {
    let form_type = e.call(FORM_TYPE, &args![form]).u32();
    e.mem
        .u32(FORM_ENUM_TABLE.wrapping_add(form_type.wrapping_mul(FORM_ENUM_STRIDE)) + 8)
}

/// `form.cast<TESForm>()` of a `BaseFormComponent` pointer (null when the
/// object is not a form).
fn cast_component_to_form(e: &mut Engine, component: Ptr) -> Ptr<TESForm> {
    e.call(
        RT_DYNAMIC_CAST,
        &args![
            component,
            0u32,
            TYPE_BASE_FORM_COMPONENT,
            TYPE_TES_FORM,
            0u32
        ],
    )
    .ptr()
}

// Translated from 00484bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual of `TESForm` (no Xbox PDB name): 4 when bit 0 of `flags` is set,
/// else 0. `fn_00484c20` and `fn_00484da0` save and load these 4 bytes (the
/// form's flags) under the same bit.
pub fn fn_00484bf0(_e: &mut Engine, _this: Ptr<TESForm>, flags: u32) -> u16 {
    if flags & 1 != 0 {
        4
    } else {
        0
    }
}

// Translated from 00484c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual of `TESForm` (no Xbox PDB name): when bit 0 of `flags` is set,
/// saves the form's flags word (+8, 4 bytes) with `SaveGameDataOLD`.
pub fn fn_00484c20(e: &mut Engine, this: Ptr<TESForm>, flags: u32) {
    if flags & 1 != 0 {
        tes_form_save_game_data_old(e, this, this.byte_add(8), 4);
    }
}

// Translated from 00484c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::LoadGame` (Xbox PDB): when bit 0 of `flags` is set, reads 4
/// bytes with `LoadGameDataOLD` and merges them into the form's flags: a form
/// that casts to `TESObjectREFR` takes the bits `0x912860`, any other form
/// the bits `0x40000c20`.
/// The second stack word (`RET 8`) is not read.
pub fn tes_form_load_game(e: &mut Engine, this: Ptr<TESForm>, flags: u32, _unused_2: u32) {
    if flags & 1 == 0 {
        return;
    }
    let saved = e.with_stack(4, |e, word| {
        tes_form_load_game_data_old(e, this, word, 4);
        e.mem.u32(word.addr())
    });
    let reference = e
        .call(
            RT_DYNAMIC_CAST,
            &args![this, 0u32, TYPE_TES_FORM, TYPE_TES_OBJECT_REFR, 0u32],
        )
        .u32();
    let current = e.get(this, TESForm::iFormFlags);
    let merged = if reference != 0 {
        current & !LOAD_GAME_REFERENCE_FLAGS | saved & LOAD_GAME_REFERENCE_FLAGS
    } else {
        current & !LOAD_GAME_FORM_FLAGS | saved & LOAD_GAME_FORM_FLAGS
    };
    e.set(this, TESForm::iFormFlags, merged);
}

// Translated from 00484ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::SaveGameDataOLD` (Xbox PDB): forwards `(buffer, size)` to the
/// `TESSaveLoadGame` instance's `SaveGameDataOLD`.
pub fn tes_form_save_game_data_old(e: &mut Engine, _this: Ptr<TESForm>, buffer: Ptr, size: u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_LOAD_SAVE_DATA, &args![save_load, buffer, size]);
}

// Translated from 00484d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::LoadGameDataOLD` (Xbox PDB): forwards `(buffer, size)` to the
/// `TESSaveLoadGame` instance's `LoadGameDataOLD`.
pub fn tes_form_load_game_data_old(e: &mut Engine, _this: Ptr<TESForm>, buffer: Ptr, size: u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_LOAD_LOAD_DATA, &args![save_load, buffer, size]);
}

// Translated from 00484d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::SaveNumericID` (Xbox PDB): forwards its two words to the
/// `TESSaveLoadGame` instance's `SaveNumericID`.
pub fn tes_form_save_numeric_id(e: &mut Engine, _this: Ptr<TESForm>, first: u32, second: u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_LOAD_SAVE_NUMERIC_ID, &args![save_load, first, second]);
}

// Translated from 00484d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::LoadNumericID` (Xbox PDB): forwards its two words to the
/// `TESSaveLoadGame` instance's `LoadNumericID`.
pub fn tes_form_load_numeric_id(e: &mut Engine, _this: Ptr<TESForm>, first: u32, second: u32) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    e.call(SAVE_LOAD_LOAD_NUMERIC_ID, &args![save_load, first, second]);
}

// Translated from 00484d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual of `TESForm` (no Xbox PDB name): when bit 0 of the save kind
/// word of `buffer` (+0x17) is set, writes the form's flags word (+8, 4
/// bytes) to `buffer`.
pub fn fn_00484d60(e: &mut Engine, this: Ptr<TESForm>, buffer: Ptr) {
    let wanted = e.with_stack(4, |e, kind| {
        let kind = e.call(BUFFER_SAVE_KIND, &args![buffer, kind]).u32();
        e.call(WORD_HAS_BITS, &args![kind, 1u32]).bool()
    });
    if wanted {
        e.call(
            SAVE_BUFFER_WRITE,
            &args![buffer, this.byte_add(8), 4u32, 0u32],
        );
    }
}

// Translated from 00484da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual of `TESForm` (no Xbox PDB name): the load counterpart of
/// `fn_00484d60`. When bit 0 of `buffer`'s save kind is set, reads 4 bytes
/// and merges them into the form's flags with the same masks as
/// [`tes_form_load_game`], choosing by the form's virtual at +0xf0.
pub fn fn_00484da0(e: &mut Engine, this: Ptr<TESForm>, buffer: Ptr) {
    let wanted = e.with_stack(4, |e, kind| {
        let kind = e.call(BUFFER_SAVE_KIND, &args![buffer, kind]).u32();
        e.call(WORD_HAS_BITS, &args![kind, 1u32]).bool()
    });
    if !wanted {
        return;
    }
    let saved = e.with_stack(4, |e, word| {
        e.call(LOAD_BUFFER_READ, &args![buffer, word, 4u32]);
        e.mem.u32(word.addr())
    });
    let current = e.get(this, TESForm::iFormFlags);
    let merged = if e.vcall(this.addr(), SLOT_IS_REFERENCE, &[]).bool() {
        current & !LOAD_GAME_REFERENCE_FLAGS | saved & LOAD_GAME_REFERENCE_FLAGS
    } else {
        current & !LOAD_GAME_FORM_FLAGS | saved & LOAD_GAME_FORM_FLAGS
    };
    e.set(this, TESForm::iFormFlags, merged);
}

// Translated from 00484e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the form's ID is in the range `1..=0x7ff` (`fn_00484b40` of the
/// word at +0x0C).
pub fn fn_00484e40(e: &mut Engine, this: Ptr<TESForm>) -> bool {
    let id = e.call(WORD_AT_0C, &args![this]).u32();
    fn_00484b40(e, id)
}

// Translated from 00484e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::GetFile` (Xbox PDB): the file at position `index` among the
/// form's non-null source files (position 0 is the first); the last non-null
/// file when there are fewer; with `index` -1, the last non-null file;
/// null without any.
pub fn tes_form_get_file(e: &mut Engine, this: Ptr<TESForm>, index: i32) -> Ptr {
    let mut node = list_first(e, this);
    let mut position = 0i32;
    let mut found = 0u32;
    while node != 0 {
        let file = list_item(e, node);
        node = list_next(e, node);
        if file == 0 {
            continue;
        }
        found = file;
        if index == -1 {
            continue;
        }
        position = position.wrapping_add(1);
        if position > index {
            break;
        }
    }
    Ptr::new(found)
}

// Translated from 00484ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::GetOwnerMaster` (Xbox PDB): the last file of the form's list
/// that `TESFile::GetMaster` accepts, walking until the list's empty node;
/// null when none.
pub fn tes_form_get_owner_master(e: &mut Engine, this: Ptr<TESForm>) -> Ptr {
    let mut node = list_first(e, this);
    let mut master = 0u32;
    while node != 0 && !e.call(LIST_NODE_EMPTY, &args![node]).bool() {
        let file = list_item(e, node);
        node = list_next(e, node);
        if e.call(FILE_IS_MASTER, &args![file]).bool() {
            master = file;
        }
    }
    Ptr::new(master)
}

// Translated from 00484f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::SetFile` (Xbox PDB): records `file` (replaced by the end of its
/// chain, `00473c70`, when that is not null) in the form's source file list.
///
/// - No file: the last list node holding a file is popped (the list's first
///   node when none holds one).
/// - A file that `GetMaster` accepts: non-master entries are moved out from
///   in front of it (each one is removed after the last master node seen, or
///   popped while no master has been seen), then the file is appended.
/// - Another file: nothing happens when the list already holds it; otherwise
///   it is appended after the last non-null node, or pushed at the front of
///   a list without files.
pub fn tes_form_set_file(e: &mut Engine, this: Ptr<TESForm>, file: Ptr) {
    let mut file = file.addr();
    let root = if file == 0 {
        0
    } else {
        e.call(FILE_ROOT, &args![file]).u32()
    };
    if root != 0 {
        file = root;
    }
    if file == 0 {
        let mut node = list_first(e, this);
        let mut last = 0u32;
        while node != 0 {
            if list_item(e, node) != 0 {
                last = node;
            }
            node = list_next(e, node);
        }
        if last != 0 {
            e.call(LIST_POP_NODE, &args![last]);
        } else {
            let first = list_first(e, this);
            e.call(LIST_POP_NODE, &args![first]);
        }
    } else if e.call(FILE_IS_MASTER, &args![file]).bool() {
        let mut node = list_first(e, this);
        let mut last_master = 0u32;
        while node != 0 && !e.call(LIST_NODE_EMPTY, &args![node]).bool() {
            let current = list_item(e, node);
            if e.call(FILE_IS_MASTER, &args![current]).bool() {
                last_master = node;
                node = list_next(e, node);
            } else if last_master != 0 {
                with_item(e, current, |e, slot| {
                    e.call(LIST_REMOVE_ITEM, &args![last_master, slot]);
                });
                node = list_next(e, last_master);
            } else {
                e.call(LIST_POP_NODE, &args![node]);
            }
        }
        let first = list_first(e, this);
        with_item(e, file, |e, slot| {
            e.call(LIST_APPEND, &args![first, slot]);
        });
    } else {
        let mut node = list_first(e, this);
        let mut last = 0u32;
        while node != 0 {
            let current = list_item(e, node);
            if current != 0 {
                last = node;
                if current == file {
                    return;
                }
            }
            node = list_next(e, node);
        }
        if last != 0 {
            with_item(e, file, |e, slot| {
                e.call(LIST_APPEND, &args![last, slot]);
            });
        } else {
            let first = list_first(e, this);
            with_item(e, file, |e, slot| {
                e.call(LIST_PUSH_FRONT, &args![first, slot]);
            });
        }
    }
}

// Translated from 00485110 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::LoadForm` (Xbox PDB): `file.LoadForm(this)`.
pub fn tes_form_load_form(e: &mut Engine, this: Ptr<TESForm>, file: Ptr) {
    e.call(FILE_LOAD_FORM, &args![file, this]);
}

// Translated from 00485130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::Copy` (Xbox PDB), the base version: logs that the form `other`
/// (named by its virtual at +0x130) has no copy function for its form type.
pub fn tes_form_copy(e: &mut Engine, _this: Ptr<TESForm>, other: Ptr<TESForm>) {
    let type_name = e.call(FORM_TYPE_NAME, &args![other]).u32();
    let name = e.vcall(other.addr(), SLOT_NAME, &[]).u32();
    e.call(LOG_MESSAGE, &args![COPY_MESSAGE, name, type_name]);
}

// Translated from 00485170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::Compare` (Xbox PDB), the base version: logs that the form
/// `other` has no compare function for its form type; returns false.
pub fn tes_form_compare(e: &mut Engine, _this: Ptr<TESForm>, other: Ptr<TESForm>) -> bool {
    let type_name = e.call(FORM_TYPE_NAME, &args![other]).u32();
    let name = e.vcall(other.addr(), SLOT_NAME, &[]).u32();
    e.call(LOG_MESSAGE, &args![COMPARE_MESSAGE, name, type_name]);
    false
}

// Translated from 004851b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::CopyAllComponents` (Xbox PDB): builds the
/// `FormComponentCollection` of this form and of `other` on the stack and
/// calls `collection(this).Copy(collection(other))` (`0047c600`). The
/// exception-unwinding frame is not translated.
pub fn tes_form_copy_all_components(e: &mut Engine, this: Ptr<TESForm>, other: Ptr<TESForm>) {
    e.with_stack(COMPONENT_COLLECTION_SIZE, |e, mine| {
        e.call(COMPONENTS_CONSTRUCT, &args![mine]);
        e.with_stack(COMPONENT_COLLECTION_SIZE, |e, theirs| {
            e.call(COMPONENTS_CONSTRUCT, &args![theirs]);
            e.call(COMPONENTS_INIT, &args![mine, this]);
            e.call(COMPONENTS_INIT, &args![theirs, other]);
            e.call(COMPONENTS_COPY, &args![mine, theirs]);
            fn_00483710(e, theirs.addr());
        });
        fn_00483710(e, mine.addr());
    });
}

// Translated from 00485270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::CompareAllComponents` (Xbox PDB): true when `other` is null;
/// otherwise builds the component collections of both forms on the stack and
/// returns `collection(this).Compare(collection(other))` (`0047c670`). The
/// exception-unwinding frame is not translated.
pub fn tes_form_compare_all_components(
    e: &mut Engine,
    this: Ptr<TESForm>,
    other: Ptr<TESForm>,
) -> bool {
    if other.is_null() {
        return true;
    }
    e.with_stack(COMPONENT_COLLECTION_SIZE, |e, mine| {
        e.call(COMPONENTS_CONSTRUCT, &args![mine]);
        e.with_stack(COMPONENT_COLLECTION_SIZE, |e, theirs| {
            e.call(COMPONENTS_CONSTRUCT, &args![theirs]);
            e.call(COMPONENTS_INIT, &args![mine, this]);
            e.call(COMPONENTS_INIT, &args![theirs, other]);
            let result = e.call(COMPONENTS_COMPARE, &args![mine, theirs]).u8();
            fn_00483710(e, theirs.addr());
            fn_00483710(e, mine.addr());
            result != 0
        })
    })
}

// Translated from 00485340 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual of `TESForm` (no Xbox PDB name) that copies the state of
/// another object from a base component pointer: when `other` casts to
/// `TESForm`, calls the virtual at +0xc8 with the other form's flag 2 state
/// if it differs from this form's, copies the form type and takes the other
/// form's flags except bit 0x4000, which stays as this form has it.
pub fn fn_00485340(e: &mut Engine, this: Ptr<TESForm>, other: Ptr) {
    let source = cast_component_to_form(e, other);
    if source.is_null() {
        return;
    }
    let mine = e.call(IS_FLAG_2, &args![this]).u8();
    let theirs = e.call(IS_FLAG_2, &args![source]).u8();
    if mine != theirs {
        let theirs = e.call(IS_FLAG_2, &args![source]).u8();
        e.vcall(this.addr(), SLOT_SET_FLAG_2, &args![theirs as u32]);
    }
    let form_type = e.call(FORM_TYPE, &args![source]).u8();
    e.set(this, TESForm::cFormType, form_type);
    let flags = e.call(FORM_FLAGS, &args![source]).u32();
    let kept = e.get(this, TESForm::iFormFlags) & FLAG_TEMPORARY;
    e.set(this, TESForm::iFormFlags, flags & !FLAG_TEMPORARY | kept);
}

// Translated from 004853e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A virtual of `TESForm` (no Xbox PDB name): whether this form differs from
/// `other` (a base component pointer). True when `other` does not cast to
/// `TESForm`, when the form types differ, when either form has a name key
/// (`00474cb0`) and the keys or the names of virtual +0x130 differ, or when
/// the flags differ outside the bits `0x4000` and `0xb`.
pub fn fn_004853e0(e: &mut Engine, this: Ptr<TESForm>, other: Ptr) -> bool {
    let source = cast_component_to_form(e, other);
    if source.is_null() {
        return true;
    }
    let form_type = e.get(this, TESForm::cFormType) as u32;
    if form_type != e.call(FORM_TYPE, &args![source]).u32() {
        return true;
    }
    let mine_key = e.call(FORM_NAME_KEY, &args![this]).u32();
    if mine_key != 0 || e.call(FORM_NAME_KEY, &args![source]).u32() != 0 {
        let mine_key = e.call(FORM_NAME_KEY, &args![this]).u32();
        let theirs_key = e.call(FORM_NAME_KEY, &args![source]).u32();
        if mine_key != theirs_key {
            return true;
        }
        let theirs_name = e.vcall(source.addr(), SLOT_NAME, &[]).u32();
        let mine_name = e.vcall(this.addr(), SLOT_NAME, &[]).u32();
        if e.call(STRING_COMPARE, &args![mine_name, theirs_name]).u32() != 0 {
            return true;
        }
    }
    let mine = e.call(FORM_FLAGS, &args![this]).u32() & COMPARE_FLAGS_MASK;
    let theirs = e.call(FORM_FLAGS, &args![source]).u32() & COMPARE_FLAGS_MASK;
    mine != theirs
}

// Translated from 004854e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `record` is the top-level group record (group tag, group type 0)
/// whose label is this form's type string. `RET 0xc`: two further words are
/// taken and ignored.
pub fn fn_004854e0(
    e: &mut Engine,
    this: Ptr<TESForm>,
    record: Ptr,
    _unused_1: u32,
    _unused_2: u32,
) -> bool {
    if record.is_null() || e.mem.u32(record.addr()) != e.global::<u32>(GROUP_TAG) {
        return false;
    }
    if e.mem.u32(record.addr() + 0xc) != 0 {
        return false;
    }
    let tag = form_type_tag(e, this);
    e.mem.u32(record.addr() + 8) == tag
}

// Translated from 00485530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Fills `record` as the top-level group record of this form's type (group
/// tag, label = the type string, group type 0, nothing else), unless
/// `record` is null or `flag` is not 0.
pub fn fn_00485530(e: &mut Engine, this: Ptr<TESForm>, record: Ptr, flag: u32) {
    if record.is_null() || flag != 0 {
        return;
    }
    let at = record.addr();
    let group_tag = e.global::<u32>(GROUP_TAG);
    e.mem.set_u32(at, group_tag);
    e.mem.set_u32(at + 0xc, 0);
    let tag = form_type_tag(e, this);
    e.mem.set_u32(at + 8, tag);
    e.mem.set_u32(at + 4, 0);
    e.mem.set_u32(at + 0x10, 0);
    e.mem.set_u16(at + 0x14, 0);
    e.mem.set_u16(at + 0x16, 0);
}

// Translated from 004855a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::StartForm` (Xbox PDB): unless the form is temporary, allocates
/// a new 0x18 byte record header in the save buffer: the form's flags (only
/// the bits `0x30032fe0` when the type is not 1), the type string as tag, the
/// form ID, size 0, the record version (`00470bb0`) and no
/// version-control data; then calls the form's virtual at +0xdc.
pub fn tes_form_start_form(e: &mut Engine, this: Ptr<TESForm>) {
    if e.call(IS_TEMPORARY, &args![this]).bool() {
        return;
    }
    e.set_global(SAVE_BUFFER_SIZE, 0x18u32);
    let size = e.global::<u32>(SAVE_BUFFER_SIZE);
    let allocated = e.call(OPERATOR_NEW, &args![size]).u32();
    e.set_global(SAVE_BUFFER, allocated);
    let header = e.global::<u32>(SAVE_BUFFER);
    let mut flags = e.call(FORM_FLAGS, &args![this]).u32();
    e.mem.set_u32(header + 8, flags);
    if e.call(FORM_TYPE, &args![this]).u32() != 1 {
        flags &= START_FORM_FLAGS_MASK;
        e.mem.set_u32(header + 8, flags);
    }
    let tag = form_type_tag(e, this);
    e.mem.set_u32(header, tag);
    let id = e.call(WORD_AT_0C, &args![this]).u32();
    e.mem.set_u32(header + 0xc, id);
    e.mem.set_u32(header + 4, 0);
    let version = e.call(RECORD_VERSION, &args![]).u16();
    e.mem.set_u16(header + 0x14, version);
    e.mem.set_u32(header + 0x10, 0);
    e.mem.set_u16(header + 0x16, 0);
    e.vcall(this.addr(), SLOT_AFTER_START, &[]);
}

// Translated from 00485680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::CloseForm` (Xbox PDB): unless the form is temporary, stores the
/// record's data size (the buffer size minus the 0x18 byte header) in the
/// header and, on a big-endian target, swaps the header (`FORM::Endian`).
pub fn tes_form_close_form(e: &mut Engine, this: Ptr<TESForm>) {
    if e.call(IS_TEMPORARY, &args![this]).bool() {
        return;
    }
    let header = e.global::<u32>(SAVE_BUFFER);
    let size = e.global::<u32>(SAVE_BUFFER_SIZE);
    e.mem.set_u32(header + 4, size.wrapping_sub(0x18));
    if is_big_endian(e) {
        e.call(FORM_ENDIAN, &args![header]);
    }
}

// Translated from 004856d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::AddChunk` (Xbox PDB), the overload without data: a chunk with
/// this tag and size 0 (cdecl).
pub fn tes_form_add_chunk(e: &mut Engine, tag: u32) {
    tes_form_add_chunk_data(e, tag, Ptr::NULL, 0);
}

// Translated from 004856f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::AddChunkArray` (Xbox PDB): a chunk with `size` bytes of `data`
/// (cdecl).
pub fn tes_form_add_chunk_array(e: &mut Engine, tag: u32, data: Ptr, size: u32) {
    tes_form_add_chunk_data(e, tag, data, size);
}

// Translated from 00485710 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::AddChunkArray_ov2` (Xbox PDB): a chunk of `count` 32-bit values
/// (cdecl); see [`tes_form_add_chunk_array32`].
pub fn tes_form_add_chunk_array_ov2(e: &mut Engine, tag: u32, data: Ptr, count: u32) {
    tes_form_add_chunk_array32(e, tag, data, count);
}

// Translated from 00485730 (decompiled, FalloutNV.exe 1.4.0.525)
/// A chunk of `count` 16-bit values (cdecl); see [`fn_00485820`].
pub fn fn_00485730(e: &mut Engine, tag: u32, data: Ptr, count: u32) {
    fn_00485820(e, tag, data, count);
}

// Translated from 00485750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::AddChunkArray32` (Xbox PDB): a chunk of `count` 32-bit values
/// (`4 * count` bytes). On a big-endian target the values are copied to a
/// temporary block and each is byte-swapped first (cdecl).
pub fn tes_form_add_chunk_array32(e: &mut Engine, tag: u32, data: Ptr, count: u32) {
    let size = count << 2;
    if !is_big_endian(e) {
        tes_form_add_chunk_data(e, tag, data, size);
        return;
    }
    let request = count.saturating_mul(4);
    let copy = e.call(OPERATOR_NEW, &args![request]).u32();
    e.call(MEMCPY, &args![copy, data, size]);
    for i in 0..count {
        e.call(SWAP_WORD, &args![copy.wrapping_add(i << 2), 0u32]);
    }
    tes_form_add_chunk_data(e, tag, Ptr::new(copy), size);
    e.call(OPERATOR_DELETE, &args![copy]);
}

// Translated from 00485820 (decompiled, FalloutNV.exe 1.4.0.525)
/// A chunk of `count` 16-bit values (`2 * count` bytes); like
/// [`tes_form_add_chunk_array32`] with half-word swaps (cdecl).
pub fn fn_00485820(e: &mut Engine, tag: u32, data: Ptr, count: u32) {
    let size = count << 1;
    if !is_big_endian(e) {
        tes_form_add_chunk_data(e, tag, data, size);
        return;
    }
    let request = count.saturating_mul(2);
    let copy = e.call(OPERATOR_NEW, &args![request]).u32();
    e.call(MEMCPY, &args![copy, data, size]);
    for i in 0..count {
        e.call(SWAP_HALF, &args![copy.wrapping_add(i << 1), 0u32]);
    }
    tes_form_add_chunk_data(e, tag, Ptr::new(copy), size);
    e.call(OPERATOR_DELETE, &args![copy]);
}

// Translated from 004858f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A one-byte chunk (cdecl): the data is the low byte of `value`, passed by
/// the address of the stack word that holds it.
pub fn fn_004858f0(e: &mut Engine, tag: u32, value: u32) {
    with_item(e, value, |e, slot| {
        tes_form_add_chunk_data(e, tag, slot, 1);
    });
}

// Translated from 00485910 (decompiled, FalloutNV.exe 1.4.0.525)
/// A chunk holding one 32-bit value (cdecl), swapped first on a big-endian
/// target.
pub fn fn_00485910(e: &mut Engine, tag: u32, value: u32) {
    with_item(e, value, |e, slot| {
        if is_big_endian(e) {
            e.call(SWAP_WORD, &args![slot, 0u32]);
        }
        tes_form_add_chunk_data(e, tag, slot, 4);
    });
}

// Translated from 00485950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::AddChunk_ov2` (Xbox PDB): a chunk holding one 16-bit value
/// (cdecl), swapped first on a big-endian target. `value` is the stack word;
/// its low half is the data.
pub fn tes_form_add_chunk_ov2(e: &mut Engine, tag: u32, value: u32) {
    with_item(e, value, |e, slot| {
        if is_big_endian(e) {
            e.call(SWAP_HALF, &args![slot, 0u32]);
        }
        tes_form_add_chunk_data(e, tag, slot, 2);
    });
}

// Translated from 00485990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::__AddChunkData` (Xbox PDB): appends a chunk to the save buffer:
/// the 4 byte tag, a `u16` size (0 for a chunk larger than 0xffff, which
/// writes an extra `"XXXX"` chunk holding the real size first) and `size`
/// bytes of `data`. The buffer grows by `size + 6` through `0042f5d0`; the
/// header is swapped on a big-endian target (cdecl).
pub fn tes_form_add_chunk_data(e: &mut Engine, tag: u32, data: Ptr, size: u32) {
    let mut size16 = size as u16;
    if size > 0xffff {
        fn_00485910(e, OVERSIZE_CHUNK_TAG, size);
        size16 = 0;
    }
    let old = e.global::<u32>(SAVE_BUFFER_SIZE);
    e.set_global(SAVE_BUFFER_SIZE, old.wrapping_add(size).wrapping_add(6));
    let buffer = e.global::<u32>(SAVE_BUFFER);
    let new_size = e.global::<u32>(SAVE_BUFFER_SIZE);
    let grown = e.call(BUFFER_REALLOC, &args![buffer, new_size]).u32();
    e.set_global(SAVE_BUFFER, grown);
    let header = e.global::<u32>(SAVE_BUFFER).wrapping_add(old);
    e.mem.set_u32(header, tag);
    e.mem.set_u16(header + 4, size16);
    if is_big_endian(e) {
        e.call(SWAP_CHUNK_HEADER, &args![header]);
    }
    e.call(MEMCPY, &args![header, header, 6u32]);
    let body = e
        .global::<u32>(SAVE_BUFFER)
        .wrapping_add(old)
        .wrapping_add(6);
    e.call(MEMCPY, &args![body, data, size]);
}

// Translated from 00485a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Grows the size field of the chunk header `chunk` by `amount` (and the
/// save buffer by the same, through `0042f5d0`). Fails (false, nothing
/// changed) when both the current size and `amount` are 0 or the new size
/// would pass 0xffff. The header is swapped to host order around the change
/// on a big-endian target.
pub fn fn_00485a70(e: &mut Engine, _this: Ptr<TESForm>, chunk: Ptr, amount: u16) -> bool {
    let swap = is_big_endian(e);
    if swap {
        e.call(SWAP_CHUNK_HEADER, &args![chunk]);
    }
    let current = e.mem.u16(chunk.addr() + 4) as u32;
    let amount = amount as u32;
    if (current == 0 && amount == 0) || current + amount > 0xffff {
        if swap {
            e.call(SWAP_CHUNK_HEADER, &args![chunk]);
        }
        return false;
    }
    e.mem.set_u16(chunk.addr() + 4, (current + amount) as u16);
    if swap {
        e.call(SWAP_CHUNK_HEADER, &args![chunk]);
    }
    let size = e.global::<u32>(SAVE_BUFFER_SIZE).wrapping_add(amount);
    e.set_global(SAVE_BUFFER_SIZE, size);
    let buffer = e.global::<u32>(SAVE_BUFFER);
    let grown = e.call(BUFFER_REALLOC, &args![buffer, size]).u32();
    e.set_global(SAVE_BUFFER, grown);
    true
}

// Translated from 00485b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::FreeFormBuffer` (Xbox PDB): frees the save buffer and clears the
/// pointer.
pub fn tes_form_free_form_buffer(e: &mut Engine, _this: Ptr<TESForm>) {
    let buffer = e.global::<u32>(SAVE_BUFFER);
    e.call(OPERATOR_DELETE, &args![buffer]);
    e.set_global(SAVE_BUFFER, 0u32);
}

// Translated from 00485b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// A packed word for the form: the low 24 bits of the form ID
/// (`fn_00485bc0`), with bit 24 set when the master list (file +0x3ec) of
/// the form's first file is not empty. A form without a file reads the
/// "file" at null.
pub fn fn_00485b60(e: &mut Engine, this: Ptr<TESForm>) -> u32 {
    let id = fn_00485bc0(e, this);
    let file = tes_form_get_file(e, this, 0);
    let masters = e.call(FILE_MASTER_LIST, &args![file]).u32();
    let empty = e.call(LIST_NODE_EMPTY, &args![masters]).bool();
    ((!empty) as u32) << 24 | id
}

// Translated from 00485bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The low 24 bits of the form's ID (+0x0C).
pub fn fn_00485bc0(e: &mut Engine, this: Ptr<TESForm>) -> u32 {
    e.get(this, TESForm::iFormID) & 0x00ff_ffff
}

// Translated from 00485be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the low 24 bits of `form_id` equal those of the form's ID.
pub fn fn_00485be0(e: &mut Engine, this: Ptr<TESForm>, form_id: u32) -> bool {
    form_id & 0x00ff_ffff == fn_00485bc0(e, this)
}

// Translated from 00485c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::SetFormID` (Xbox PDB): when `new_id` differs from the form's ID
/// and the form is not temporary, removes the old ID from the form map,
/// removes it from the data handler when `remove_from_handler` is set, and
/// maps the new ID (if not 0) to the form; the ID is stored in every case.
/// The compiler's dead block after the map removal (a log message and the
/// byte at `011c54b9`, guarded by a local that is always 0) and the
/// exception-unwinding frame are not translated.
pub fn tes_form_set_form_id(
    e: &mut Engine,
    this: Ptr<TESForm>,
    new_id: u32,
    remove_from_handler: bool,
) {
    let old_id = e.get(this, TESForm::iFormID);
    if new_id == old_id {
        return;
    }
    if !e.call(IS_TEMPORARY, &args![this]).bool() {
        if old_id != 0 {
            let map = e.global::<u32>(FORM_MAP);
            e.call(MAP_REMOVE_AT, &args![map, old_id]);
        }
        if remove_from_handler && e.get(this, TESForm::iFormID) != 0 {
            let id = e.get(this, TESForm::iFormID);
            let handler = e.global::<u32>(DATA_HANDLER);
            e.call(DATA_HANDLER_REMOVE_ID, &args![handler, id]);
        }
        if new_id != 0 {
            let map = e.global::<u32>(FORM_MAP);
            e.call(MAP_SET_AT, &args![map, new_id, this]);
        }
    }
    e.set(this, TESForm::iFormID, new_id);
}

// Translated from 00485d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::AddCompileIndex` (Xbox PDB), a cdecl function `(id pointer,
/// file)`: while the save/load stub is true the ID is remapped by
/// `TESSaveLoadGame` (`00857bf0`); otherwise, unless the ID is the one
/// `fn_00484b40` selects and when `file` is not null, the top byte of the ID
/// becomes the compile index of the file the ID's top byte plus one names in
/// `file` (`GetIndexFile`), or of `file` itself when there is none.
pub fn tes_form_add_compile_index(e: &mut Engine, id: Ptr, file: Ptr) {
    let save_load = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(SAVE_LOAD_STUB, &args![save_load]).bool() {
        let old = e.mem.u32(id.addr());
        let remapped = e.call(SAVE_LOAD_REMAP_ID, &args![save_load, old]).u32();
        e.mem.set_u32(id.addr(), remapped);
        return;
    }
    let old = e.mem.u32(id.addr());
    if fn_00484b40(e, old) || file.addr() == 0 {
        return;
    }
    let indexed = e
        .call(
            FILE_GET_INDEX_FILE,
            &args![file, (old >> 24).wrapping_add(1)],
        )
        .u32();
    let source = if indexed != 0 { indexed } else { file.addr() };
    let index = e.call(FILE_COMPILE_INDEX, &args![source]).u8() as u32;
    let current = e.mem.u32(id.addr());
    e.mem
        .set_u32(id.addr(), index << 24 | current & 0x00ff_ffff);
}

/// The eight form components `__SaveData`, `LoadData` and `fn_00486620`
/// cast a form to (each 0 when the form is not of that kind).
struct Components {
    uses: u32,
    value: u32,
    health: u32,
    weight: u32,
    quality: u32,
    attack_damage: u32,
    attributes: u32,
    clip_rounds: u32,
}

/// `__RTDynamicCast(form, 0, TESForm, target, 0)`: the component of
/// `target`'s kind inside the form, or 0.
fn cast_form_to(e: &mut Engine, form: Ptr<TESForm>, target: u32) -> u32 {
    e.call(
        RT_DYNAMIC_CAST,
        &args![form, 0u32, TYPE_TES_FORM, target, 0u32],
    )
    .u32()
}

/// All eight casts, in the order of the game's code.
fn cast_components(e: &mut Engine, form: Ptr<TESForm>) -> Components {
    Components {
        uses: cast_form_to(e, form, TYPE_USES_FORM),
        value: cast_form_to(e, form, TYPE_VALUE_FORM),
        health: cast_form_to(e, form, TYPE_HEALTH_FORM),
        weight: cast_form_to(e, form, TYPE_WEIGHT_FORM),
        quality: cast_form_to(e, form, TYPE_QUALITY_FORM),
        attack_damage: cast_form_to(e, form, TYPE_ATTACK_DAMAGE_FORM),
        attributes: cast_form_to(e, form, TYPE_ATTRIBUTES),
        clip_rounds: cast_form_to(e, form, TYPE_CLIP_ROUNDS_FORM),
    }
}

/// Bytes the components add to the form's `DATA` chunk: 1 for the uses and
/// clip rounds components, 4 for value, health, weight and quality, 2 for
/// the attack damage and the attributes' own size (`009d2510`).
fn components_size(e: &mut Engine, parts: &Components) -> u32 {
    let mut size = (parts.uses != 0) as u32;
    for part in [parts.value, parts.health, parts.weight, parts.quality] {
        if part != 0 {
            size += 4;
        }
    }
    if parts.attack_damage != 0 {
        size += 2;
    }
    if parts.attributes != 0 {
        size = size.wrapping_add(e.call(ATTRIBUTES_SIZE, &args![parts.attributes]).u32());
    }
    if parts.clip_rounds != 0 {
        size += 1;
    }
    size
}

/// Byte-swaps the word at `addr` in place (`Swap32(addr, 0)`).
fn swap_word_in_place(e: &mut Engine, addr: u32) {
    e.call(SWAP_WORD, &args![addr, 0u32]);
}

/// Byte-swaps the `u16` at `addr` in place.
fn swap_half_in_place(e: &mut Engine, addr: u32) {
    e.call(SWAP_HALF, &args![addr, 0u32]);
}

/// x87 `FISTP` with the rounding mode set to truncation: values that do not
/// fit a 32 bit integer give the "integer indefinite" `0x80000000`.
fn x87_truncate(value: f32) -> i32 {
    if value.is_nan() || value >= 2_147_483_648.0 || value < -2_147_483_648.0 {
        i32::MIN
    } else {
        value as i32
    }
}

// Translated from 00485e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::SaveData` (Xbox PDB): `__SaveData` with no data.
pub fn tes_form_save_data(e: &mut Engine, this: Ptr<TESForm>) {
    tes_form_save_data_inner(e, this, Ptr::new(0), 0);
}

// Translated from 00485e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::__SaveData` (Xbox PDB): writes the `DATA` chunk of `size` bytes
/// of `data` (a log message and nothing else when `size` is over 0xffff),
/// then makes room in that chunk for what the form's components add and
/// appends it: the uses byte, the value and health words, the weight and the
/// quality as floats, the attack damage `u16`, the attributes' own data and
/// the clip rounds byte, each swapped to big endian when the target is.
pub fn tes_form_save_data_inner(e: &mut Engine, this: Ptr<TESForm>, data: Ptr, size: u32) {
    if size > 0xffff {
        e.call(LOG_MESSAGE, &args![SAVE_DATA_TOO_LARGE_MESSAGE]);
        return;
    }
    let size_before = e.call(SAVE_BUFFER_SIZE_GETTER, &args![this]).u32();
    tes_form_add_chunk_data(e, DATA_CHUNK_TAG, data, size);
    let chunk = e
        .call(SAVE_BUFFER_GETTER, &args![this])
        .u32()
        .wrapping_add(size_before);
    let parts = cast_components(e, this);
    let extra = components_size(e, &parts) as u16;
    if extra == 0 {
        return;
    }
    let size_before = e.call(SAVE_BUFFER_SIZE_GETTER, &args![this]).u32();
    if !fn_00485a70(e, this, Ptr::new(chunk), extra) {
        return;
    }
    let mut out = e
        .call(SAVE_BUFFER_GETTER, &args![this])
        .u32()
        .wrapping_add(size_before);
    if parts.uses != 0 {
        let byte = e.call(COMPONENT_BYTE_GET, &args![parts.uses]).u8();
        e.mem.set_u8(out, byte);
        out += 1;
    }
    for part in [parts.value, parts.health] {
        if part != 0 {
            let word = e.call(COMPONENT_WORD_GET, &args![part]).u32();
            e.mem.set_u32(out, word);
            if is_big_endian(e) {
                swap_word_in_place(e, out);
            }
            out += 4;
        }
    }
    if parts.weight != 0 {
        let weight = e.call(WEIGHT_GET, &args![parts.weight]).f32();
        out = write_float(e, out, weight);
    }
    if parts.quality != 0 {
        let quality = e.call(QUALITY_GET, &args![parts.quality]).u8();
        out = write_float(e, out, quality as f32);
    }
    if parts.attack_damage != 0 {
        let damage = e
            .vcall(parts.attack_damage, SLOT_ATTACK_DAMAGE_GET, &args![])
            .u16();
        e.mem.set_u16(out, damage);
        if is_big_endian(e) {
            swap_half_in_place(e, out);
        }
        out += 2;
    }
    if parts.attributes != 0 {
        let used = e.call(ATTRIBUTES_SAVE, &args![parts.attributes, out]).u32();
        out = out.wrapping_add(used);
    }
    if parts.clip_rounds != 0 {
        let byte = e.call(COMPONENT_BYTE_GET, &args![parts.clip_rounds]).u8();
        e.mem.set_u8(out, byte);
    }
}

/// Stores `value` as a float at `out` (through a stack local and `memcpy`, as
/// the game does), swapped when the target is big endian; returns the
/// address after it.
fn write_float(e: &mut Engine, out: u32, value: f32) -> u32 {
    e.with_stack(4, |e, local| {
        e.mem.set_f32(local.addr(), value);
        if is_big_endian(e) {
            swap_word_in_place(e, local.addr());
        }
        e.call(MEMCPY, &args![out, local.addr(), 4u32]);
    });
    out.wrapping_add(4)
}

// Translated from 004861f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::LoadData` (Xbox PDB), `(file, destination, start)`: reads the
/// current chunk of `file` into a local buffer, copies its first
/// `min(chunk size, start)` bytes to `destination`, then hands the
/// components, from offset `start` of the chunk on, their values in the order
/// `__SaveData` wrote them (each only while the offset is inside the chunk,
/// the attributes always): the uses byte, the value and health words, the
/// weight and the quality as floats (the quality is truncated to an integer
/// and set as a byte), the attack damage `u16`, the attributes' own data and
/// the clip rounds byte. Words are swapped when the file is big endian.
pub fn tes_form_load_data(
    e: &mut Engine,
    this: Ptr<TESForm>,
    file: Ptr,
    destination: Ptr,
    start: u16,
) {
    let size = e.call(FILE_CHUNK_SIZE, &args![file]).u16();
    e.with_stack((size as u32).max(1), |e, buffer| {
        let buffer = buffer.addr();
        e.call(FILE_READ_CHUNK, &args![file, buffer, size as u32]);
        let copied = fn_004865f0(e, size, start);
        e.call(MEMCPY, &args![destination, buffer, copied as u32]);
        let parts = cast_components(e, this);
        let mut offset = start;
        let swapped = |e: &mut Engine| e.call(FILE_IS_BIG_ENDIAN, &args![file]).bool();
        if parts.uses != 0 && offset < size {
            let byte = e.mem.u8(buffer + offset as u32) as u32;
            e.call(COMPONENT_BYTE_SET, &args![parts.uses, byte]);
            offset = offset.wrapping_add(1);
        }
        for (part, setter) in [(parts.value, VALUE_SET), (parts.health, HEALTH_SET)] {
            if part != 0 && offset < size {
                let mut word = e.mem.u32(buffer + offset as u32);
                offset = offset.wrapping_add(4);
                if swapped(e) {
                    word = byte_swapped_word(e, word);
                }
                e.call(setter, &args![part, word]);
            }
        }
        if parts.weight != 0 && offset < size {
            let (weight, next) = read_float(e, file, buffer, offset);
            offset = next;
            e.call(WEIGHT_SET, &args![parts.weight, weight]);
        }
        if parts.quality != 0 && offset < size {
            let (quality, next) = read_float(e, file, buffer, offset);
            offset = next;
            let truncated = x87_truncate(quality) as u32 & 0xff;
            e.call(COMPONENT_BYTE_SET, &args![parts.quality, truncated]);
        }
        if parts.attack_damage != 0 && offset < size {
            let mut half = e.mem.u16(buffer + offset as u32);
            offset = offset.wrapping_add(2);
            if swapped(e) {
                half = e.with_stack(2, |e, local| {
                    e.mem.set_u16(local.addr(), half);
                    swap_half_in_place(e, local.addr());
                    e.mem.u16(local.addr())
                });
            }
            e.call(ATTACK_DAMAGE_SET, &args![parts.attack_damage, half as u32]);
        }
        if parts.attributes != 0 {
            let used = e
                .call(
                    ATTRIBUTES_LOAD,
                    &args![parts.attributes, buffer + offset as u32],
                )
                .u32();
            offset = (offset as u32).wrapping_add(used) as u16;
        }
        if parts.clip_rounds != 0 && offset < size {
            let byte = e.mem.u8(buffer + offset as u32) as u32;
            e.call(COMPONENT_BYTE_SET, &args![parts.clip_rounds, byte]);
        }
    });
}

/// A word read from the game's stack local after a byte swap.
fn byte_swapped_word(e: &mut Engine, value: u32) -> u32 {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), value);
        swap_word_in_place(e, local.addr());
        e.mem.u32(local.addr())
    })
}

/// Reads the float at `buffer + offset` (`memcpy` into a local, then into a
/// second local that is swapped when `file` is big endian); returns it and
/// the offset after it.
fn read_float(e: &mut Engine, file: Ptr, buffer: u32, offset: u16) -> (f32, u16) {
    let value = e.with_stack(8, |e, locals| {
        let copy = locals.addr();
        let work = copy + 4;
        e.call(MEMCPY, &args![copy, buffer + offset as u32, 4u32]);
        let value = e.mem.f32(copy);
        e.mem.set_f32(work, value);
        if e.call(FILE_IS_BIG_ENDIAN, &args![file]).bool() {
            swap_word_in_place(e, work);
        }
        e.mem.f32(work)
    });
    (value, offset.wrapping_add(4))
}

// Translated from 004865f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The smaller of two `u16` values (cdecl).
pub fn fn_004865f0(_e: &mut Engine, first: u16, second: u16) -> u16 {
    first.min(second)
}

// Translated from 00486620 (decompiled, FalloutNV.exe 1.4.0.525)
/// A fastcall function `(form)`: the number of bytes the form's components
/// add to its `DATA` chunk (see [`components_size`]).
pub fn fn_00486620(e: &mut Engine, this: Ptr<TESForm>) -> u32 {
    let parts = cast_components(e, this);
    components_size(e, &parts)
}

// Translated from 004867a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A form copier `(form, _unused_1, map)`: creates a form of the same type
/// (`CreateFormOfType`), and when that works: saves the name the form's name
/// getter returns in a string local, gives the original form's virtual at
/// +0x134 the constant string `DEFAULT_NAME`, calls the new form's virtual at
/// +0x108 with the original form, and then, if the saved string holds
/// anything, gives it back to the original through the virtual at +0x134.
/// When `map` is not null the new form is added to it under the original
/// form's address (`SetAt`). Returns the new form or null. The exception
/// frame is not translated.
pub fn fn_004867a0(e: &mut Engine, this: Ptr<TESForm>, _unused_1: u32, map: Ptr) -> Ptr<TESForm> {
    let form_type = e.call(FORM_TYPE, &args![this]).u32();
    let created = e
        .call(CREATE_FORM_OF_TYPE, &args![form_type])
        .ptr::<TESForm>();
    if created.addr() != 0 {
        let name = e.call(FORM_NAME_GETTER, &args![this]).u32();
        e.with_stack(8, |e, saved| {
            e.call(STRING_CONSTRUCT, &args![saved, name]);
            e.vcall(this.addr(), SLOT_TAKE_NAME, &args![DEFAULT_NAME]);
            e.vcall(created.addr(), SLOT_COPY_FROM, &args![this]);
            if e.call(STRING_DATA, &args![saved]).u32() != 0 {
                let text = e.call(STRING_DATA, &args![saved]).u32();
                e.vcall(this.addr(), SLOT_TAKE_NAME, &args![text]);
            }
            e.call(STRING_DESTROY, &args![saved]);
        });
        if map.addr() != 0 {
            e.call(MAP_SET_AT, &args![map, this, created]);
        }
    }
    created
}

// Translated from 00486890 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::GetFormTypeFromFormString` (Xbox PDB): the index of the entry of
/// the form-type table whose packed string equals `packed` (0 when none),
/// remembering the last string and index it found.
pub fn tes_form_get_form_type_from_form_string(e: &mut Engine, packed: u32) -> u32 {
    if packed == e.global::<u32>(FORM_TYPE_CACHE_KEY) {
        return e.global::<u32>(FORM_TYPE_CACHE_RESULT);
    }
    for index in 0..FORM_ENUM_COUNT {
        let entry = FORM_ENUM_TABLE + index * FORM_ENUM_STRIDE;
        // Form-type table entry +8: the packed string.
        if e.mem.u32(entry + 8) == packed {
            e.set_global(FORM_TYPE_CACHE_RESULT, index);
            e.set_global(FORM_TYPE_CACHE_KEY, packed);
            return index;
        }
    }
    0
}

// Translated from 004868f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESForm::GetFormDetailedString` (Xbox PDB): prints `"<type> Form '<name>'
/// (<form ID>)"` into `buffer` (`sprintf`), the type being the form-type
/// table's string and the name what `00401280` returns.
pub fn tes_form_get_form_detailed_string(e: &mut Engine, this: Ptr<TESForm>, buffer: Ptr) {
    let id = e.call(WORD_AT_0C, &args![this]).u32();
    let name = e.call(FORM_NAME_GETTER, &args![this]).u32();
    let type_name = e.call(FORM_TYPE_NAME, &args![this]).u32();
    e.call(
        FORMAT_STRING,
        &args![buffer, DETAILED_STRING_FORMAT, type_name, name, id],
    );
}

// Translated from 00486960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, TESForm *>::scalar deleting destructor`
/// (Xbox PDB): the map's destructor and, when bit 0 of `flags` is set, the
/// release of the object. Returns the map.
pub fn ni_t_pointer_map_form_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00486a00(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00486930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The constructor of the form map `(buckets)`: the map base's constructor
/// (`fn_00486990`) and then the `NiTPointerMap` vtable.
pub fn fn_00486930(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    fn_00486990(e, this, buckets);
    e.mem.set_u32(this.addr(), POINTER_MAP_VTABLE);
    this
}

// Translated from 00486990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The map base's constructor `(buckets)`: stores the map base vtable, the
/// bucket count (+4), an item count of 0 (+0x0C) and a zeroed array of
/// `buckets` words (+8). Returns the map.
pub fn fn_00486990(e: &mut Engine, this: Ptr, buckets: u32) -> Ptr {
    let base = this.addr();
    e.mem.set_u32(base, MAP_BASE_VTABLE);
    e.mem.set_u32(base + 4, buckets);
    e.mem.set_u32(base + 0xc, 0);
    let array = e.call(ALLOCATE_BUCKETS, &args![buckets << 2]).u32();
    e.mem.set_u32(base + 8, array);
    // PC 004869d6 reloads the count after the allocation callback.
    let bytes = e.mem.u32(base + 4).wrapping_shl(2);
    e.call(MEMSET, &args![array, 0u32, bytes]);
    this
}

// Translated from 00486a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of the form map: the `NiTPointerMap` vtable, removal
/// of every entry (`00438af0`) and the map base's destruction (`00486a60`).
/// The exception frame is not translated.
pub fn fn_00486a00(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), POINTER_MAP_VTABLE);
    e.call(MAP_CLEAR, &args![this]);
    e.call(MAP_BASE_DESTROY, &args![this]);
}

/// The shared tail of the scalar deleting destructors below: runs
/// `destroy(this)` and releases the object when bit 0 of `flags` is set.
fn scalar_deleting(e: &mut Engine, this: Ptr, flags: u32, destroy: u32) -> Ptr {
    e.call(destroy, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00486c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, TESForm *>::
/// scalar deleting destructor` (Xbox PDB): `00486a60`, then the release when
/// bit 0 of `flags` is set. Returns the object.
pub fn ni_t_map_base_form_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    scalar_deleting(e, this, flags, MAP_BASE_DESTROY)
}

// Translated from 00486c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSTCaseInsensitiveStringMap<TESForm *>::scalar deleting destructor`
/// (Xbox PDB): the destructor `fn_00486c60`, then the release when bit 0 of
/// `flags` is set. Returns the object.
pub fn bst_case_insensitive_string_map_form_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    fn_00486c60(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00486c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the case-insensitive string map: its vtable, then the
/// base destruction (`00486c80`).
pub fn fn_00486c60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), STRING_MAP_VTABLE);
    e.call(STRING_MAP_BASE_DESTROY, &args![this]);
}

// Translated from 00486e50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<DFALL<NiTMapItem<const char *, TESForm *>>, const char *,
/// TESForm *>::scalar deleting destructor` (Xbox PDB): `00486dc0`, then the
/// release when bit 0 of `flags` is set. Returns the object.
pub fn ni_t_map_base_char_key_form_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, CHAR_KEY_MAP_BASE_DESTROY)
}

// Translated from 00486e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMap<const char *, TESForm *>::scalar deleting destructor` (Xbox PDB):
/// `00486d60`, then the release when bit 0 of `flags` is set. Returns the
/// object.
pub fn ni_t_map_char_key_form_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, NI_T_MAP_DESTROY)
}

// Translated from 00486eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTStringTemplateMap<NiTMap<const char *, TESForm *>, TESForm *>::scalar
/// deleting destructor` (Xbox PDB): `00486ca0`, then the release when bit 0
/// of `flags` is set. Returns the object.
pub fn ni_t_string_template_map_form_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr,
    flags: u32,
) -> Ptr {
    scalar_deleting(e, this, flags, STRING_TEMPLATE_MAP_DESTROY)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00483370, tes_form_tes_form(Ptr<TESForm>) -> Ptr<TESForm>),
        entry!(0x004835e0, fn_004835e0(u32) -> u32),
        entry!(
            0x00483600,
            tes_form_scalar_deleting_destructor(Ptr<TESForm>, u32) -> Ptr<TESForm>
        ),
        entry!(0x00483630, tes_form_dtor(Ptr<TESForm>)),
        entry!(0x00483710, fn_00483710(u32)),
        entry!(0x00483720, fn_00483720(Ptr<TESForm>)),
        entry!(0x00483870, fn_00483870(Ptr<TESForm>)),
        entry!(0x004839c0, fn_004839c0(u32) -> Ptr<TESForm>),
        entry!(
            0x00483a00,
            tes_form_get_form_by_editor_id(Ptr) -> Ptr<TESForm>
        ),
        entry!(0x00483a50, fn_00483a50()),
        entry!(0x00483b80, fn_00483b80()),
        entry!(
            0x00483c70,
            tes_form_remove_from_data_structures(Ptr<TESForm>)
        ),
        entry!(0x00483d20, tes_form_save(Ptr<TESForm>, Ptr) -> bool),
        entry!(0x00483d70, tes_form_compress_save_buffer()),
        entry!(0x00483fc0, fn_00483fc0(Ptr<TESForm>, Ptr) -> bool),
        entry!(0x00484020, fn_00484020(Ptr<TESForm>, Ptr<TESForm>) -> bool),
        entry!(0x00484150, fn_00484150(Ptr<TESForm>, Ptr) -> bool),
        entry!(0x00484490, tes_form_set_temporary(Ptr<TESForm>)),
        entry!(0x004844f0, fn_004844f0(Ptr<TESForm>, bool)),
        entry!(0x00484530, tes_form_set_delete(Ptr<TESForm>, bool)),
        entry!(0x00484580, tes_form_set_empty(Ptr<TESForm>, bool)),
        entry!(0x00484610, fn_00484610(Ptr<TESForm>, bool)),
        entry!(0x00484650, fn_00484650(Ptr<TESForm>, bool)),
        entry!(0x004846a0, tes_form_set_destructible(Ptr<TESForm>, bool)),
        entry!(0x004846e0, fn_004846e0(Ptr<TESForm>, bool)),
        entry!(0x00484730, fn_00484730(Ptr<TESForm>, bool)),
        entry!(0x00484860, fn_00484860(Ptr<TESForm>, bool)),
        entry!(0x004848b0, fn_004848b0(Ptr<TESForm>, bool)),
        entry!(0x00484900, fn_00484900(Ptr<TESForm>, bool)),
        entry!(0x00484950, fn_00484950(Ptr<TESForm>, bool)),
        entry!(0x00484990, fn_00484990(Ptr<TESForm>, bool)),
        entry!(0x004849d0, fn_004849d0(Ptr<TESForm>, bool)),
        entry!(0x00484a10, fn_00484a10(Ptr<TESForm>, bool)),
        entry!(0x00484a70, tes_form_set_fire_off(Ptr<TESForm>, bool)),
        entry!(0x00484ab0, fn_00484ab0(Ptr<TESForm>, bool)),
        entry!(0x00484af0, tes_form_set_disabled(Ptr<TESForm>, bool)),
        entry!(0x00484b40, fn_00484b40(u32) -> bool),
        entry!(0x00484b60, fn_00484b60(Ptr<TESForm>, u32)),
        entry!(0x00484b90, tes_form_force_change(Ptr<TESForm>, u32)),
        entry!(0x00484bc0, fn_00484bc0(Ptr<TESForm>, u32)),
        entry!(0x00484bf0, fn_00484bf0(Ptr<TESForm>, u32) -> u16),
        entry!(0x00484c20, fn_00484c20(Ptr<TESForm>, u32)),
        entry!(0x00484c50, tes_form_load_game(Ptr<TESForm>, u32, u32)),
        entry!(
            0x00484ce0,
            tes_form_save_game_data_old(Ptr<TESForm>, Ptr, u32)
        ),
        entry!(
            0x00484d00,
            tes_form_load_game_data_old(Ptr<TESForm>, Ptr, u32)
        ),
        entry!(0x00484d20, tes_form_save_numeric_id(Ptr<TESForm>, u32, u32)),
        entry!(0x00484d40, tes_form_load_numeric_id(Ptr<TESForm>, u32, u32)),
        entry!(0x00484d60, fn_00484d60(Ptr<TESForm>, Ptr)),
        entry!(0x00484da0, fn_00484da0(Ptr<TESForm>, Ptr)),
        entry!(0x00484e40, fn_00484e40(Ptr<TESForm>) -> bool),
        entry!(0x00484e60, tes_form_get_file(Ptr<TESForm>, i32) -> Ptr),
        entry!(0x00484ee0, tes_form_get_owner_master(Ptr<TESForm>) -> Ptr),
        entry!(0x00484f50, tes_form_set_file(Ptr<TESForm>, Ptr)),
        entry!(0x00485110, tes_form_load_form(Ptr<TESForm>, Ptr)),
        entry!(0x00485130, tes_form_copy(Ptr<TESForm>, Ptr<TESForm>)),
        entry!(
            0x00485170,
            tes_form_compare(Ptr<TESForm>, Ptr<TESForm>) -> bool
        ),
        entry!(
            0x004851b0,
            tes_form_copy_all_components(Ptr<TESForm>, Ptr<TESForm>)
        ),
        entry!(
            0x00485270,
            tes_form_compare_all_components(Ptr<TESForm>, Ptr<TESForm>) -> bool
        ),
        entry!(0x00485340, fn_00485340(Ptr<TESForm>, Ptr)),
        entry!(0x004853e0, fn_004853e0(Ptr<TESForm>, Ptr) -> bool),
        entry!(0x004854e0, fn_004854e0(Ptr<TESForm>, Ptr, u32, u32) -> bool),
        entry!(0x00485530, fn_00485530(Ptr<TESForm>, Ptr, u32)),
        entry!(0x004855a0, tes_form_start_form(Ptr<TESForm>)),
        entry!(0x00485680, tes_form_close_form(Ptr<TESForm>)),
        entry!(0x004856d0, tes_form_add_chunk(u32)),
        entry!(0x004856f0, tes_form_add_chunk_array(u32, Ptr, u32)),
        entry!(0x00485710, tes_form_add_chunk_array_ov2(u32, Ptr, u32)),
        entry!(0x00485730, fn_00485730(u32, Ptr, u32)),
        entry!(0x00485750, tes_form_add_chunk_array32(u32, Ptr, u32)),
        entry!(0x00485820, fn_00485820(u32, Ptr, u32)),
        entry!(0x004858f0, fn_004858f0(u32, u32)),
        entry!(0x00485910, fn_00485910(u32, u32)),
        entry!(0x00485950, tes_form_add_chunk_ov2(u32, u32)),
        entry!(0x00485990, tes_form_add_chunk_data(u32, Ptr, u32)),
        entry!(0x00485a70, fn_00485a70(Ptr<TESForm>, Ptr, u16) -> bool),
        entry!(0x00485b30, tes_form_free_form_buffer(Ptr<TESForm>)),
        entry!(0x00485b60, fn_00485b60(Ptr<TESForm>) -> u32),
        entry!(0x00485bc0, fn_00485bc0(Ptr<TESForm>) -> u32),
        entry!(0x00485be0, fn_00485be0(Ptr<TESForm>, u32) -> bool),
        entry!(0x00485c10, tes_form_set_form_id(Ptr<TESForm>, u32, bool)),
        entry!(0x00485d50, tes_form_add_compile_index(Ptr, Ptr)),
        entry!(0x00485e00, tes_form_save_data(Ptr<TESForm>)),
        entry!(0x00485e20, tes_form_save_data_inner(Ptr<TESForm>, Ptr, u32)),
        entry!(0x004861f0, tes_form_load_data(Ptr<TESForm>, Ptr, Ptr, u16)),
        entry!(0x004865f0, fn_004865f0(u16, u16) -> u16),
        entry!(0x00486620, fn_00486620(Ptr<TESForm>) -> u32),
        entry!(
            0x004867a0,
            fn_004867a0(Ptr<TESForm>, u32, Ptr) -> Ptr<TESForm>
        ),
        entry!(
            0x00486890,
            tes_form_get_form_type_from_form_string(u32) -> u32
        ),
        entry!(
            0x004868f0,
            tes_form_get_form_detailed_string(Ptr<TESForm>, Ptr)
        ),
        entry!(0x00486930, fn_00486930(Ptr, u32) -> Ptr),
        entry!(
            0x00486960,
            ni_t_pointer_map_form_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00486990, fn_00486990(Ptr, u32) -> Ptr),
        entry!(0x00486a00, fn_00486a00(Ptr)),
        entry!(
            0x00486c00,
            ni_t_map_base_form_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00486c30,
            bst_case_insensitive_string_map_form_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(0x00486c60, fn_00486c60(Ptr)),
        entry!(
            0x00486e50,
            ni_t_map_base_char_key_form_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00486e80,
            ni_t_map_char_key_form_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x00486eb0,
            ni_t_string_template_map_form_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test vtable for forms, and doubles at the addresses of its slots.
    const VTABLE: u32 = 0x0200_0000;
    const SLOT_DESTRUCTOR: u32 = 0x0f00_0000;
    const SLOT_0X28: u32 = 0x0f00_0028;
    const SLOT_0X2C: u32 = 0x0f00_002c;
    const SLOT_0X48: u32 = 0x0f00_0048;
    const SLOT_0X4C: u32 = 0x0f00_004c;
    const BIG_ENDIAN_FLAG: u32 = 0x011c_54ba;
    const GROUP_TAG_VALUE: u32 = 0x5055_5247;

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// An engine with the globals' pages mapped and doubles for the trivial
    /// getters and the form's virtual slots.
    fn form_engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x011c_5000,
            0x011c_3000,
            0x011d_d000,
            0x011d_e000,
            0x0118_7000,
        ] {
            e.map(page, 0x1000);
        }
        let mut slots = vec![0u32; 20];
        slots[0] = SLOT_DESTRUCTOR;
        slots[0x28 / 4] = SLOT_0X28;
        slots[0x2c / 4] = SLOT_0X2C;
        slots[0x48 / 4] = SLOT_0X48;
        slots[0x4c / 4] = SLOT_0X4C;
        e.put_vtable(VTABLE, &slots);
        e.register(IS_TEMPORARY, |e, a| {
            ret((e.mem.u32(a[0] + 8) & 0x4000 != 0) as u32)
        });
        e.register(IS_FLAG_2, |e, a| ret((e.mem.u32(a[0] + 8) & 2 != 0) as u32));
        e.register(IS_DELETED, |e, a| {
            ret((e.mem.u32(a[0] + 8) & 0x20 != 0) as u32)
        });
        e.register(FORM_TYPE, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(WORD_AT_0C, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        e.register(LIST_ITEM_ADDRESS, |e, a| {
            ret(e.mem.u32(a[0] + 4).wrapping_add(4 * a[1]))
        });
        e.register(IS_BIG_ENDIAN, |e, _| ret(e.mem.u8(BIG_ENDIAN_FLAG) as u32));
        for slot in [SLOT_DESTRUCTOR, SLOT_0X28, SLOT_0X2C, SLOT_0X48, SLOT_0X4C] {
            e.register(slot, |_, _| Ret::default());
        }
        e
    }

    /// A form with the test vtable, the given flags and type.
    fn form(e: &mut Engine, flags: u32, form_type: u8) -> Ptr<TESForm> {
        let form: Ptr<TESForm> = e.new_object();
        e.mem.set_u32(form.addr(), VTABLE);
        e.set(form, TESForm::iFormFlags, flags);
        e.set(form, TESForm::cFormType, form_type);
        e.set(form, TESForm::iFormID, 0x0100_0abc);
        form
    }

    /// The recorded calls to `addr`, with their argument words.
    fn calls_to(log: &[(u32, Vec<u32>)], addr: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// A form array holding the given forms (items at +4, count at +0x0C).
    fn form_array(e: &mut Engine, forms: &[Ptr<TESForm>]) -> u32 {
        let list = e.mem.alloc(0x18);
        let data = e.mem.alloc(4 * 8);
        e.mem.set_u32(list + 4, data);
        e.mem.set_u32(list + 0xc, forms.len() as u32);
        for (i, f) in forms.iter().enumerate() {
            e.mem.set_u32(data + 4 * i as u32, f.addr());
        }
        list
    }

    /// An object whose first word is the test vtable (slot 0 is the
    /// destructor double).
    fn object_with_vtable(e: &mut Engine) -> u32 {
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, VTABLE);
        object
    }

    /// `form_engine` plus doubles for what the constructor calls.
    fn constructor_engine() -> Engine {
        let mut e = form_engine();
        e.register(BASE_CONSTRUCT, |_, _| Ret::default());
        e.register(SIMPLE_LIST_CONSTRUCT, |_, _| Ret::default());
        e.register(FORM_MAP_CONSTRUCT, |_, a| ret(a[0]));
        e.register(EDITOR_ID_MAP_CONSTRUCT, |_, a| ret(a[0]));
        e.register(FORM_LIST_CONSTRUCT, |_, a| ret(a[0]));
        e.register(MAP_SET_AT, |_, _| Ret::default());
        e.register(SET_FILE, |_, _| Ret::default());
        e.register(SET_FORM_ID, |_, _| Ret::default());
        e.register(DISABLE_WARNING_COUNT, |_, _| Ret::default());
        e.register(LOG_MESSAGE, |_, _| Ret::default());
        e.register(FATAL_ERROR, |_, _| Ret::default());
        e.register(DATA_HANDLER_FLAG, |e, a| ret(e.mem.u8(a[0] + 0x620) as u32));
        e.register(DATA_HANDLER_GET_NEXT_ID, |_, _| ret(0x10));
        e
    }

    #[test]
    fn constructor_creates_the_registries_with_the_first_form() {
        let mut e = constructor_engine();
        e.set_global(FORM_TABLE_CHECKED, 1u8);
        let first: Ptr<TESForm> = e.new_object();
        e.call_log = Some(vec![]);
        let back = e.call(0x0048_3370, &args![first]).ptr::<TESForm>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, first);
        assert_eq!(e.mem.u32(first.addr()), TES_FORM_VTABLE);
        assert_eq!(e.get(first, TESForm::iFormFlags), 8);
        assert_eq!(e.get(first, TESForm::iFormID), 0);
        assert_eq!(e.global::<u32>(FORM_COUNT), 1);
        assert_ne!(e.global::<u32>(FORM_MAP), 0);
        assert_ne!(e.global::<u32>(EDITOR_ID_MAP), 0);
        assert_ne!(e.global::<u32>(FORM_LIST), 0);
        // Without a form ID the form is not put in the map.
        assert!(calls_to(&log, MAP_SET_AT).is_empty());
        // A second form finds the registries and only counts itself.
        let map = e.global::<u32>(FORM_MAP);
        let second: Ptr<TESForm> = e.new_object();
        e.call(0x0048_3370, &args![second]);
        assert_eq!(e.global::<u32>(FORM_MAP), map);
        assert_eq!(e.global::<u32>(FORM_COUNT), 2);
    }

    #[test]
    fn constructor_takes_an_id_and_file_from_the_data_handler() {
        let mut e = constructor_engine();
        e.set_global(FORM_TABLE_CHECKED, 1u8);
        e.set_global(FORM_COUNT, 1u32);
        e.set_global(FORM_MAP, 0x7700_0000u32);
        let handler = e.mem.alloc(0x640);
        e.mem.set_u32(handler + 0x20c, 0x5555);
        e.set_global(DATA_HANDLER, handler);
        let form: Ptr<TESForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0048_3370, &args![form]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(form, TESForm::iFormID), 0x10);
        assert_eq!(calls_to(&log, SET_FILE), vec![vec![form.addr(), 0x5555]]);
        // The ID is below 0x800: the form is given the first free ID.
        assert_eq!(
            calls_to(&log, SET_FORM_ID),
            vec![vec![form.addr(), 0x800, 1]]
        );
        assert_eq!(
            calls_to(&log, MAP_SET_AT),
            vec![vec![0x7700_0000, 0x10, form.addr()]]
        );

        // A handler whose flag byte is set leaves the ID alone.
        e.mem.set_u8(handler + 0x620, 1);
        let other: Ptr<TESForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0048_3370, &args![other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(other, TESForm::iFormID), 0);
        assert!(calls_to(&log, SET_FILE).is_empty());
        assert!(calls_to(&log, MAP_SET_AT).is_empty());
    }

    #[test]
    fn constructor_reloads_data_handler_after_flag_and_id_callbacks() {
        let mut e = constructor_engine();
        e.set_global(FORM_TABLE_CHECKED, 1u8);
        e.set_global(FORM_COUNT, 1u32);
        e.set_global(FORM_MAP, 0x7700_0000u32);
        let first = e.mem.alloc(0x640);
        let second = e.mem.alloc(0x640);
        let third = e.mem.alloc(0x640);
        e.mem.set_u32(first + 0x20c, 0x1111);
        e.mem.set_u32(second + 0x20c, 0x2222);
        e.mem.set_u32(third + 0x20c, 0x3333);
        e.set_global(DATA_HANDLER, first);
        e.register_double(DATA_HANDLER_FLAG, move |e, args| {
            assert_eq!(args, &[first]);
            e.set_global(DATA_HANDLER, second);
            ret(0)
        });
        e.register_double(DATA_HANDLER_GET_NEXT_ID, move |e, args| {
            assert_eq!(args, &[second]);
            e.set_global(DATA_HANDLER, third);
            ret(0x1000)
        });
        let form: Ptr<TESForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x00483370, &args![form]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(form, TESForm::iFormID), 0x1000);
        assert_eq!(calls_to(&log, SET_FILE), vec![vec![form.addr(), 0x3333]]);
        assert_eq!(
            calls_to(&log, MAP_SET_AT),
            vec![vec![0x77000000, 0x1000, form.addr()]]
        );
    }

    #[test]
    fn constructor_skips_set_file_without_a_current_file_and_keeps_high_ids() {
        let mut e = constructor_engine();
        e.set_global(FORM_TABLE_CHECKED, 1u8);
        e.set_global(FORM_COUNT, 1u32);
        e.set_global(FORM_MAP, 0x7700_0000u32);
        e.register(DATA_HANDLER_GET_NEXT_ID, |_, _| ret(0x0100_0000));
        let handler = e.mem.alloc(0x640);
        e.set_global(DATA_HANDLER, handler);
        let form: Ptr<TESForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0048_3370, &args![form]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SET_FILE).is_empty());
        assert!(calls_to(&log, SET_FORM_ID).is_empty());
        assert_eq!(calls_to(&log, MAP_SET_AT).len(), 1);
    }

    /// Fills the form-type table with `0x79` entries: type byte = index and a
    /// distinct four-character string each.
    fn fill_form_enum_table(e: &mut Engine) {
        for i in 0..FORM_ENUM_COUNT {
            let entry = FORM_ENUM_TABLE + i * FORM_ENUM_STRIDE;
            let text = 0x0118_7800 + 4 * i;
            e.mem.write(
                text,
                &[b'T', b'A' + (i / 26) as u8, b'A' + (i % 26) as u8, b'_'],
            );
            e.mem.set_u8(entry, i as u8);
            e.mem.set_u32(entry + 4, text);
        }
    }

    #[test]
    fn constructor_packs_the_form_type_table_once() {
        let mut e = constructor_engine();
        fill_form_enum_table(&mut e);
        // A string with a byte above 0x7f packs sign-extended.
        e.mem.set_u8(0x0118_7800 + 4 * 10, 0xe9);
        let form: Ptr<TESForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0048_3370, &args![form]);
        let log = e.call_log.take().unwrap();
        let packed = |e: &Engine, i: u32| e.mem.u32(FORM_ENUM_TABLE + i * FORM_ENUM_STRIDE + 8);
        assert_eq!(packed(&e, 0), u32::from_le_bytes(*b"TAA_"));
        let expected = (-23i32) | (b'A' as i32) << 8 | (b'K' as i32) << 16 | (b'_' as i32) << 24;
        assert_eq!(packed(&e, 10), expected as u32);
        assert!(calls_to(&log, LOG_MESSAGE).is_empty());
        assert!(calls_to(&log, FATAL_ERROR).is_empty());
        assert_eq!(
            calls_to(&log, DISABLE_WARNING_COUNT),
            vec![vec![1], vec![0]]
        );
        assert_eq!(e.global::<u8>(FORM_TABLE_CHECKED), 1);

        // Already checked: the table is not looked at again.
        let again: Ptr<TESForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0048_3370, &args![again]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, DISABLE_WARNING_COUNT).is_empty());
    }

    #[test]
    fn constructor_reports_a_misnumbered_and_a_duplicated_form_type() {
        let mut e = constructor_engine();
        fill_form_enum_table(&mut e);
        // Entry 5 has the wrong type byte; entry 4 shares entry 3's string.
        e.mem.set_u8(FORM_ENUM_TABLE + 5 * FORM_ENUM_STRIDE, 7);
        let shared = e.mem.u32(FORM_ENUM_TABLE + 3 * FORM_ENUM_STRIDE + 4);
        e.mem
            .set_u32(FORM_ENUM_TABLE + 4 * FORM_ENUM_STRIDE + 4, shared);
        let form: Ptr<TESForm> = e.new_object();
        e.call_log = Some(vec![]);
        e.call(0x0048_3370, &args![form]);
        let log = e.call_log.take().unwrap();
        let messages = calls_to(&log, LOG_MESSAGE);
        assert!(messages.contains(&vec![FORM_ENUM_ERROR_ORDER, 5]));
        // Entry 3 is packed before entry 4, so the duplicate shows when entry
        // 4 is reached and compared with the already packed entry 3.
        assert!(messages.contains(&vec![FORM_ENUM_ERROR_DUPLICATE, 4, 3, shared]));
        assert_eq!(
            calls_to(&log, FATAL_ERROR),
            vec![vec![FORM_ENUM_FIX_MESSAGE]]
        );
        assert_eq!(e.global::<u8>(FORM_TABLE_CHECKED), 1);
    }

    #[test]
    fn data_handler_getter_reads_the_current_file() {
        let mut e = Engine::new();
        let handler = e.mem.alloc(0x640);
        e.mem.set_u32(handler + 0x20c, 0xabcd);
        assert_eq!(e.call(0x0048_35e0, &args![handler]).u32(), 0xabcd);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_when_asked() {
        let mut e = form_engine();
        e.set_global(FORM_COUNT, 5u32);
        e.register(SIMPLE_LIST_DESTROY, |_, _| Ret::default());
        e.register(SIMPLE_LIST_DESTROY_WRAPPER, |_, _| Ret::default());
        let kept = form(&mut e, 8, 0x20);
        let back = e.call(0x0048_3600, &args![kept, 0u32]).ptr::<TESForm>();
        assert_eq!(back, kept);
        assert!(e.mem.block_size(kept.addr()).is_some());
        let freed = form(&mut e, 8, 0x20);
        let back = e.call(0x0048_3600, &args![freed, 1u32]).ptr::<TESForm>();
        assert_eq!(back, freed);
        assert_eq!(e.mem.block_size(freed.addr()), None);
        assert_eq!(e.global::<u32>(FORM_COUNT), 3);
    }

    #[test]
    fn destructor_unregisters_a_form_and_the_last_one_destroys_the_registries() {
        let mut e = form_engine();
        e.register(SIMPLE_LIST_DESTROY, |_, _| Ret::default());
        e.register(SIMPLE_LIST_DESTROY_WRAPPER, |_, _| Ret::default());
        e.register(SAVE_LOAD_REMOVE_CHANGES, |_, _| Ret::default());
        e.register(SAVE_LOAD_REMOVE_FORM, |_, _| Ret::default());
        e.register(BGS_REMOVE_CHANGES, |_, _| Ret::default());
        e.register(MAP_REMOVE_AT, |_, _| Ret::default());
        e.register(MAP_CLEAR, |_, _| Ret::default());
        e.register(FORM_LIST_CLEAR, |_, _| Ret::default());
        e.set_global(SAVE_LOAD_GAME, 0x6600_0000u32);
        e.set_global(BGS_SAVE_LOAD_GAME, 0x6700_0000u32);
        let map = object_with_vtable(&mut e);
        e.set_global(FORM_MAP, map);
        e.set_global(FORM_COUNT, 1u32);
        let victim = form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_3630, &args![victim]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(victim.addr()), TES_FORM_VTABLE);
        assert_eq!(
            calls_to(&log, SAVE_LOAD_REMOVE_CHANGES),
            vec![vec![0x6600_0000, victim.addr(), 0]]
        );
        assert_eq!(
            calls_to(&log, SAVE_LOAD_REMOVE_FORM),
            vec![vec![0x6600_0000, victim.addr()]]
        );
        assert_eq!(
            calls_to(&log, BGS_REMOVE_CHANGES),
            vec![vec![0x6700_0000, victim.addr()]]
        );
        assert_eq!(calls_to(&log, MAP_REMOVE_AT).len(), 1);
        assert_eq!(calls_to(&log, SIMPLE_LIST_DESTROY).len(), 1);
        assert_eq!(calls_to(&log, SIMPLE_LIST_DESTROY_WRAPPER).len(), 1);
        // The last form: the registries go (the map's destructor got 1).
        assert_eq!(e.global::<u32>(FORM_COUNT), 0);
        assert_eq!(e.global::<u32>(FORM_MAP), 0);
        assert_eq!(calls_to(&log, SLOT_DESTRUCTOR), vec![vec![map, 1]]);
    }

    #[test]
    fn destructor_leaves_a_temporary_form_out_of_the_change_tables() {
        let mut e = form_engine();
        e.register(SIMPLE_LIST_DESTROY, |_, _| Ret::default());
        e.register(SIMPLE_LIST_DESTROY_WRAPPER, |_, _| Ret::default());
        e.set_global(SAVE_LOAD_GAME, 0x6600_0000u32);
        e.set_global(BGS_SAVE_LOAD_GAME, 0x6700_0000u32);
        e.set_global(FORM_COUNT, 3u32);
        let temporary = form(&mut e, 0x4000, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_3630, &args![temporary]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SAVE_LOAD_REMOVE_CHANGES).is_empty());
        assert!(calls_to(&log, BGS_REMOVE_CHANGES).is_empty());
        assert_eq!(e.global::<u32>(FORM_COUNT), 2);
    }

    #[test]
    fn empty_destructor_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x0048_3710, &args![0x1234u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    /// Doubles for the component collection calls.
    fn component_engine() -> Engine {
        let mut e = form_engine();
        for addr in [
            COMPONENTS_CONSTRUCT,
            COMPONENTS_INIT,
            COMPONENTS_VISIT_FIRST,
            COMPONENTS_VISIT_SECOND,
            CLEAR_SUB_OBJECT,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn component_visit_depends_on_the_form_type() {
        let mut e = component_engine();
        for form_type in [0x11u8, 0x3a, 0x40, 0x42, 0x43, 0x49, 0x69] {
            let f = form(&mut e, 8, form_type);
            e.call_log = Some(vec![]);
            e.call(0x0048_3720, &args![f]);
            assert_eq!(e.call_log.take().unwrap().len(), 2, "type {form_type:#x}");
        }
        let special = form(&mut e, 8, 0x39);
        e.call_log = Some(vec![]);
        e.call(0x0048_3720, &args![special]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, CLEAR_SUB_OBJECT),
            vec![vec![special.addr() + 0x18]]
        );
        let ordinary = form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_3720, &args![ordinary]);
        let log = e.call_log.take().unwrap();
        let built = calls_to(&log, COMPONENTS_CONSTRUCT);
        assert_eq!(built.len(), 1);
        let collection = built[0][0];
        assert_eq!(
            calls_to(&log, COMPONENTS_INIT),
            vec![vec![collection, ordinary.addr()]]
        );
        assert_eq!(
            calls_to(&log, COMPONENTS_VISIT_FIRST),
            vec![vec![collection]]
        );
        assert!(calls_to(&log, COMPONENTS_VISIT_SECOND).is_empty());
        // Types just outside the groups take the ordinary path.
        for form_type in [0x10u8, 0x41, 0x6a] {
            let f = form(&mut e, 8, form_type);
            e.call_log = Some(vec![]);
            e.call(0x0048_3720, &args![f]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, COMPONENTS_CONSTRUCT).len(), 1);
        }
    }

    #[test]
    fn second_component_visit_uses_the_second_slot_and_clears_nothing() {
        let mut e = component_engine();
        let skipped = form(&mut e, 8, 0x3c);
        e.call_log = Some(vec![]);
        e.call(0x0048_3870, &args![skipped]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        let special = form(&mut e, 8, 0x39);
        e.call_log = Some(vec![]);
        e.call(0x0048_3870, &args![special]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, CLEAR_SUB_OBJECT).is_empty());
        assert_eq!(log.len(), 2);
        let ordinary = form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_3870, &args![ordinary]);
        let log = e.call_log.take().unwrap();
        let collection = calls_to(&log, COMPONENTS_CONSTRUCT)[0][0];
        assert_eq!(
            calls_to(&log, COMPONENTS_VISIT_SECOND),
            vec![vec![collection]]
        );
        assert!(calls_to(&log, COMPONENTS_VISIT_FIRST).is_empty());
    }

    #[test]
    fn form_lookup_by_id() {
        let mut e = form_engine();
        // No map: null without a lookup.
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0048_39c0, &args![0x1234u32])
            .ptr::<TESForm>()
            .is_null());
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // A hit stores the form through the out pointer.
        e.set_global(FORM_MAP, 0x7700_0000u32);
        e.register(MAP_LOOKUP, |e, a| {
            e.mem.set_u32(a[2], 0xf0f0);
            ret(1)
        });
        e.call_log = Some(vec![]);
        let hit = e.call(0x0048_39c0, &args![0x1234u32]).ptr::<TESForm>();
        let log = e.call_log.take().unwrap();
        assert_eq!(hit.addr(), 0xf0f0);
        let lookups = calls_to(&log, MAP_LOOKUP);
        assert_eq!((lookups[0][0], lookups[0][1]), (0x7700_0000, 0x1234));
        // A miss gives null even when the lookup scribbled on the out word.
        e.register(MAP_LOOKUP, |e, a| {
            e.mem.set_u32(a[2], 0xf0f0);
            ret(0)
        });
        assert!(e
            .call(0x0048_39c0, &args![0x1234u32])
            .ptr::<TESForm>()
            .is_null());
    }

    #[test]
    fn form_lookup_by_editor_id() {
        let mut e = form_engine();
        let name = Ptr::<()>::new(e.mem.alloc(16));
        e.mem.set_cstr(name.addr(), b"MyForm");
        let empty = Ptr::<()>::new(e.mem.alloc(16));
        e.register(MAP_LOOKUP, |e, a| {
            e.mem.set_u32(a[2], 0xabc0);
            ret(1)
        });
        // No map yet.
        assert!(e.call(0x0048_3a00, &args![name]).ptr::<TESForm>().is_null());
        e.set_global(EDITOR_ID_MAP, 0x7800_0000u32);
        // Null and empty names never reach the map.
        e.call_log = Some(vec![]);
        assert!(e
            .call(0x0048_3a00, &args![Ptr::<()>::NULL])
            .ptr::<TESForm>()
            .is_null());
        assert!(e
            .call(0x0048_3a00, &args![empty])
            .ptr::<TESForm>()
            .is_null());
        assert!(calls_to(&e.call_log.take().unwrap(), MAP_LOOKUP).is_empty());
        // A hit.
        e.call_log = Some(vec![]);
        let hit = e.call(0x0048_3a00, &args![name]).ptr::<TESForm>();
        assert_eq!(hit.addr(), 0xabc0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MAP_LOOKUP)[0][..2],
            [0x7800_0000, name.addr()]
        );
        // A miss.
        e.register(MAP_LOOKUP, |_, _| ret(0));
        assert!(e.call(0x0048_3a00, &args![name]).ptr::<TESForm>().is_null());
    }

    #[test]
    fn registries_are_created_once() {
        let mut e = form_engine();
        e.register(FORM_MAP_CONSTRUCT, |_, a| ret(a[0]));
        e.register(EDITOR_ID_MAP_CONSTRUCT, |_, a| ret(a[0]));
        e.register(FORM_LIST_CONSTRUCT, |_, a| ret(a[0]));
        e.call_log = Some(vec![]);
        e.call(0x0048_3a50, &[]);
        let log = e.call_log.take().unwrap();
        let map = e.global::<u32>(FORM_MAP);
        let editor_ids = e.global::<u32>(EDITOR_ID_MAP);
        let list = e.global::<u32>(FORM_LIST);
        assert!(map != 0 && editor_ids != 0 && list != 0);
        assert_eq!(calls_to(&log, FORM_MAP_CONSTRUCT), vec![vec![map, 0x2008d]]);
        assert_eq!(
            calls_to(&log, EDITOR_ID_MAP_CONSTRUCT),
            vec![vec![editor_ids, 0xfa1, 1]]
        );
        assert_eq!(
            calls_to(&log, FORM_LIST_CONSTRUCT),
            vec![vec![list, 0x20, 1]]
        );
        // The three blocks are 0x10, 0x14 and 0x18 bytes.
        assert_eq!(e.mem.block_size(map), Some(0x10));
        assert!(e.mem.block_size(editor_ids).unwrap() >= 0x14);
        assert_eq!(e.mem.block_size(list), Some(0x18));
        // A second call changes nothing.
        e.call_log = Some(vec![]);
        e.call(0x0048_3a50, &[]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.global::<u32>(FORM_MAP), map);
    }

    #[test]
    fn registries_are_destroyed_and_cleared_from_the_globals() {
        let mut e = form_engine();
        e.register(MAP_CLEAR, |_, _| Ret::default());
        e.register(FORM_LIST_CLEAR, |_, _| Ret::default());
        // Nothing to do without registries.
        e.call_log = Some(vec![]);
        e.call(0x0048_3b80, &[]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        let map = object_with_vtable(&mut e);
        let editor_ids = object_with_vtable(&mut e);
        let list = object_with_vtable(&mut e);
        e.set_global(FORM_MAP, map);
        e.set_global(EDITOR_ID_MAP, editor_ids);
        e.set_global(FORM_LIST, list);
        e.call_log = Some(vec![]);
        e.call(0x0048_3b80, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MAP_CLEAR), vec![vec![map], vec![editor_ids]]);
        assert_eq!(calls_to(&log, FORM_LIST_CLEAR), vec![vec![list]]);
        assert_eq!(
            calls_to(&log, SLOT_DESTRUCTOR),
            vec![vec![map, 1], vec![editor_ids, 1], vec![list, 1]]
        );
        assert_eq!(e.global::<u32>(FORM_MAP), 0);
        assert_eq!(e.global::<u32>(EDITOR_ID_MAP), 0);
        assert_eq!(e.global::<u32>(FORM_LIST), 0);
    }

    #[test]
    fn remove_from_data_structures_skips_temporary_forms() {
        let mut e = form_engine();
        e.set_global(FORM_MAP, 0x7700_0000u32);
        e.set_global(DATA_HANDLER, 0x7900_0000u32);
        let temporary = form(&mut e, 0x4000, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_3c70, &args![temporary]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    #[test]
    fn remove_from_data_structures_clears_map_list_and_handler() {
        let mut e = form_engine();
        e.register(MAP_REMOVE_AT, |_, _| Ret::default());
        e.register(FORM_LIST_REMOVE_AT, |_, _| Ret::default());
        e.register(DATA_HANDLER_REMOVE_ID, |_, _| Ret::default());
        let other = form(&mut e, 8, 0x20);
        let target = form(&mut e, 8, 0x20);
        let list = form_array(&mut e, &[other, target]);
        e.set_global(FORM_MAP, 0x7700_0000u32);
        e.set_global(FORM_LIST, list);
        e.set_global(DATA_HANDLER, 0x7900_0000u32);
        e.call_log = Some(vec![]);
        e.call(0x0048_3c70, &args![target]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MAP_REMOVE_AT),
            vec![vec![0x7700_0000, 0x0100_0abc]]
        );
        assert_eq!(calls_to(&log, FORM_LIST_REMOVE_AT), vec![vec![list, 1]]);
        assert_eq!(
            calls_to(&log, DATA_HANDLER_REMOVE_ID),
            vec![vec![0x7900_0000, 0x0100_0abc]]
        );

        // Not in the array, and no map or handler: nothing is called.
        let stranger = form(&mut e, 8, 0x20);
        e.set_global(FORM_MAP, 0u32);
        e.set_global(DATA_HANDLER, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x0048_3c70, &args![stranger]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, FORM_LIST_REMOVE_AT).is_empty());
        assert!(calls_to(&log, MAP_REMOVE_AT).is_empty());
    }

    #[test]
    fn save_adds_the_form_to_the_file_unless_temporary() {
        let mut e = form_engine();
        e.register(FILE_ADD_FORM, |_, _| ret(0));
        let file = Ptr::<()>::new(0x5100_0000);
        let normal = form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_3d20, &args![normal, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SLOT_0X2C), vec![vec![normal.addr()]]);
        assert_eq!(
            calls_to(&log, FILE_ADD_FORM),
            vec![vec![file.addr(), normal.addr()]]
        );
        // A nonzero result from the file is a failure.
        e.register(FILE_ADD_FORM, |_, _| ret(3));
        assert!(!e.call(0x0048_3d20, &args![normal, file]).bool());
        // Temporary forms are not saved.
        let temporary = form(&mut e, 0x4000, 0x20);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0048_3d20, &args![temporary, file]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 2);
    }

    /// Doubles for zlib and the byte-swapping helpers; the deflate double
    /// checks its input and writes `ABCDE`.
    fn compress_engine() -> Engine {
        let mut e = form_engine();
        e.register(DEFLATE_INIT, |_, a| {
            assert_eq!(a[1..], [0xffff_ffff, ZLIB_VERSION, 0x38]);
            ret(0)
        });
        e.register(DEFLATE, |e, a| {
            let stream = a[0];
            assert_eq!(a[1], 4);
            // avail_in = 10, avail_out = twice that.
            assert_eq!(e.mem.u32(stream + 4), 10);
            assert_eq!(e.mem.u32(stream + 0x10), 20);
            let out = e.mem.u32(stream + 0x0c);
            e.mem.write(out, b"ABCDE");
            e.mem.set_u32(stream + 0x10, 15);
            ret(1)
        });
        e.register(DEFLATE_END, |_, _| Ret::default());
        e.register(FORM_ENDIAN, |_, _| Ret::default());
        e.register(SWAP_WORD, |e, a| {
            let value = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], value.swap_bytes());
            Ret::default()
        });
        e.register(LOG_MESSAGE, |_, _| Ret::default());
        e
    }

    /// A save buffer of a 0x18-byte header and ten body bytes.
    fn install_save_buffer(e: &mut Engine, flags: u32) -> u32 {
        let buffer = e.mem.alloc(0x22);
        e.mem.set_u32(buffer, 0x4d41_4e59);
        e.mem.set_u32(buffer + 4, 10);
        e.mem.set_u32(buffer + 8, flags);
        e.mem.write(buffer + 0x18, b"0123456789");
        e.set_global(SAVE_BUFFER, buffer);
        e.set_global(SAVE_BUFFER_SIZE, 0x22u32);
        buffer
    }

    #[test]
    fn compress_replaces_the_buffer_with_header_size_and_stream() {
        let mut e = compress_engine();
        let old = install_save_buffer(&mut e, 0x10);
        e.call_log = Some(vec![]);
        e.call(0x0048_3d70, &[]);
        let log = e.call_log.take().unwrap();
        let new = e.global::<u32>(SAVE_BUFFER);
        assert_ne!(new, old);
        // 5 compressed bytes + header (0x18) + the size word (4).
        assert_eq!(e.global::<u32>(SAVE_BUFFER_SIZE), 0x21);
        assert_eq!(e.mem.u32(new), 0x4d41_4e59);
        assert_eq!(e.mem.u32(new + 4), 9);
        assert_eq!(e.mem.u32(new + 8), 0x10 | 0x40000);
        assert_eq!(e.mem.u32(new + 0x18), 10);
        assert_eq!(e.mem.bytes(new + 0x1c, 5), b"ABCDE");
        assert_eq!(e.mem.block_size(old), None);
        assert!(calls_to(&log, FORM_ENDIAN).is_empty());
        assert_eq!(calls_to(&log, DEFLATE_END).len(), 1);
    }

    #[test]
    fn compress_swaps_the_header_and_size_on_a_big_endian_target() {
        let mut e = compress_engine();
        e.set_global(BIG_ENDIAN_FLAG, 1u8);
        install_save_buffer(&mut e, 0);
        e.call_log = Some(vec![]);
        e.call(0x0048_3d70, &[]);
        let log = e.call_log.take().unwrap();
        let new = e.global::<u32>(SAVE_BUFFER);
        assert_eq!(calls_to(&log, FORM_ENDIAN).len(), 2);
        assert_eq!(e.mem.u32(new + 0x18), 10u32.swap_bytes());
    }

    #[test]
    fn compress_leaves_small_missing_and_compressed_buffers_alone() {
        let mut e = compress_engine();
        // No buffer.
        e.call_log = Some(vec![]);
        e.call(0x0048_3d70, &[]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // A header only.
        let buffer = install_save_buffer(&mut e, 0);
        e.set_global(SAVE_BUFFER_SIZE, 0x18u32);
        e.call_log = Some(vec![]);
        e.call(0x0048_3d70, &[]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // Already compressed: only the endian swap of the header, twice on a
        // big-endian target.
        e.set_global(SAVE_BUFFER_SIZE, 0x22u32);
        e.mem.set_u32(buffer + 8, 0x40000);
        e.set_global(BIG_ENDIAN_FLAG, 1u8);
        e.call_log = Some(vec![]);
        e.call(0x0048_3d70, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FORM_ENDIAN).len(), 2);
        assert!(calls_to(&log, DEFLATE_INIT).is_empty());
        assert_eq!(e.global::<u32>(SAVE_BUFFER), buffer);
    }

    #[test]
    fn compress_logs_zlib_failures_and_keeps_the_buffer() {
        let mut e = compress_engine();
        let buffer = install_save_buffer(&mut e, 0);
        e.register(DEFLATE_INIT, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0048_3d70, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOG_MESSAGE), vec![vec![ZLIB_INIT_ERROR]]);
        assert_eq!(e.global::<u32>(SAVE_BUFFER), buffer);
        assert_eq!(e.mem.u32(buffer + 8), 0);

        e.register(DEFLATE_INIT, |_, _| ret(0));
        e.register(DEFLATE, |_, _| ret(-2i32 as u32));
        e.call_log = Some(vec![]);
        e.call(0x0048_3d70, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LOG_MESSAGE), vec![vec![ZLIB_DEFLATE_ERROR]]);
        assert!(calls_to(&log, DEFLATE_END).is_empty());
        assert_eq!(e.global::<u32>(SAVE_BUFFER), buffer);
        assert_eq!(e.mem.u32(buffer + 8), 0);
    }

    #[test]
    fn write_form_starts_deleted_forms_and_defers_others_to_the_virtual() {
        let mut e = form_engine();
        e.register(FILE_START_FORM, |_, _| Ret::default());
        e.register(FILE_AFTER_START, |_, _| Ret::default());
        let file = Ptr::<()>::new(0x5100_0000);
        let temporary = form(&mut e, 0x4000, 0x20);
        assert!(!e.call(0x0048_3fc0, &args![temporary, file]).bool());
        let deleted = form(&mut e, 0x20, 0x20);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_3fc0, &args![deleted, file]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FILE_START_FORM),
            vec![vec![file.addr(), deleted.addr()]]
        );
        assert_eq!(calls_to(&log, FILE_AFTER_START), vec![vec![file.addr()]]);
        assert!(calls_to(&log, SLOT_0X28).is_empty());
        // Otherwise the virtual's result decides.
        let normal = form(&mut e, 8, 0x20);
        e.register(SLOT_0X28, |_, _| ret(1));
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_3fc0, &args![normal, file]).bool());
        assert_eq!(
            calls_to(&e.call_log.take().unwrap(), SLOT_0X28),
            vec![vec![normal.addr(), file.addr()]]
        );
        e.register(SLOT_0X28, |_, _| ret(0x100));
        assert!(!e.call(0x0048_3fc0, &args![normal, file]).bool());
    }

    #[test]
    fn type_order_ranks_the_special_types_by_table() {
        let mut e = form_engine();
        let low = form(&mut e, 8, 0x20); // rank 320
        let special = form(&mut e, 8, 0x67); // rank 0x137 = 311
        let other_special = form(&mut e, 8, 0x74); // rank 0x13a = 314
        assert!(e.call(0x0048_4020, &args![special, low]).bool());
        assert!(!e.call(0x0048_4020, &args![low, special]).bool());
        assert!(e.call(0x0048_4020, &args![special, other_special]).bool());
        assert!(!e.call(0x0048_4020, &args![low, low]).bool());
        let t6c = form(&mut e, 8, 0x6c);
        let t73 = form(&mut e, 8, 0x73);
        assert!(e.call(0x0048_4020, &args![t6c, t73]).bool());
        assert!(e.call(0x0048_4020, &args![other_special, low]).bool());
    }

    #[test]
    fn group_order_depends_on_the_group_type() {
        let mut e = form_engine();
        e.set_global(GROUP_TAG, GROUP_TAG_VALUE);
        e.register(GET_FORM_TYPE_FROM_FORM_STRING, |_, _| ret(5));
        let group = e.mem.alloc(0x14);
        e.mem.set_u32(group, GROUP_TAG_VALUE);
        let ask = |e: &mut Engine, group_type: u32, form_type: u8| {
            e.mem.set_u32(group + 0xc, group_type);
            let f = form(e, 8, form_type);
            e.call(0x0048_4150, &args![f, Ptr::<()>::new(group)]).bool()
        };
        // Group type 0: rank of the label's form type (5 -> 50).
        assert!(ask(&mut e, 0, 4));
        assert!(!ask(&mut e, 0, 5));
        // Types 1, 4, 5: below 0x28a (650).
        for group_type in [1, 4, 5] {
            assert!(ask(&mut e, group_type, 64));
            assert!(!ask(&mut e, group_type, 65));
        }
        // Types 2, 3, 6: below 0x23a (570).
        for group_type in [2, 3, 6] {
            assert!(ask(&mut e, group_type, 56));
            assert!(!ask(&mut e, group_type, 57));
        }
        // Type 7: below 0x2b2 (690).
        assert!(ask(&mut e, 7, 68));
        assert!(!ask(&mut e, 7, 69));
        // The special ranks count too (0x67 ranks 311).
        assert!(ask(&mut e, 2, 0x67));
        // Unknown group types, a null group and a wrong tag are false.
        assert!(!ask(&mut e, 8, 1));
        let f = form(&mut e, 8, 1);
        assert!(!e.call(0x0048_4150, &args![f, Ptr::<()>::NULL]).bool());
        e.mem.set_u32(group, 0x1234);
        e.mem.set_u32(group + 0xc, 1);
        assert!(!e.call(0x0048_4150, &args![f, Ptr::<()>::new(group)]).bool());
    }

    #[test]
    fn set_temporary_removes_the_form_from_the_tables_then_flags_it() {
        let mut e = form_engine();
        e.register(SAVE_LOAD_REMOVE_CHANGES, |_, _| Ret::default());
        e.register(BGS_REMOVE_CHANGES, |_, _| Ret::default());
        let f = form(&mut e, 8, 0x20);
        // No save/load objects: only the flag changes.
        e.call_log = Some(vec![]);
        e.call(0x0048_4490, &args![f]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SAVE_LOAD_REMOVE_CHANGES).is_empty());
        assert!(calls_to(&log, BGS_REMOVE_CHANGES).is_empty());
        assert_eq!(e.get(f, TESForm::iFormFlags), 8 | 0x4000);
        // With them.
        e.set_global(SAVE_LOAD_GAME, 0x6600_0000u32);
        e.set_global(BGS_SAVE_LOAD_GAME, 0x6700_0000u32);
        let g = form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_4490, &args![g]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SAVE_LOAD_REMOVE_CHANGES),
            vec![vec![0x6600_0000, g.addr(), 0]]
        );
        assert_eq!(
            calls_to(&log, BGS_REMOVE_CHANGES),
            vec![vec![0x6700_0000, g.addr()]]
        );
    }

    /// Checks a flag setter: it sets and clears exactly `mask` and calls the
    /// changed hook (virtual +0x48 with 1) when `notifies`.
    fn check_flag_setter(addr: u32, mask: u32, notifies: bool) {
        let mut e = form_engine();
        let f = form(&mut e, 0xa000_0000, 0x20);
        e.call_log = Some(vec![]);
        e.call(addr, &args![f, true]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(f, TESForm::iFormFlags), 0xa000_0000 | mask);
        let hooks = calls_to(&log, SLOT_0X48);
        if notifies {
            assert_eq!(hooks, vec![vec![f.addr(), 1]]);
        } else {
            assert!(hooks.is_empty());
        }
        e.set(f, TESForm::iFormFlags, 0xffff_ffff);
        e.call_log = Some(vec![]);
        e.call(addr, &args![f, false]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.get(f, TESForm::iFormFlags), !mask);
        assert_eq!(calls_to(&log, SLOT_0X48).len(), notifies as usize);
    }

    #[test]
    fn flag_0x1_setter() {
        check_flag_setter(0x0048_44f0, 0x1, false);
    }

    #[test]
    fn set_delete_sets_0x20_and_notifies() {
        check_flag_setter(0x0048_4530, 0x20, true);
    }

    #[test]
    fn flag_0x2000_setter() {
        check_flag_setter(0x0048_4610, 0x2000, false);
    }

    #[test]
    fn flag_0x800000_setter() {
        check_flag_setter(0x0048_4650, 0x80_0000, true);
    }

    #[test]
    fn set_destructible_sets_0x1000000() {
        check_flag_setter(0x0048_46a0, 0x100_0000, false);
    }

    #[test]
    fn flag_0x4000000_setter() {
        check_flag_setter(0x0048_46e0, 0x400_0000, true);
    }

    #[test]
    fn flag_0x400_setter() {
        check_flag_setter(0x0048_4860, 0x400, true);
    }

    #[test]
    fn flag_0x40_setter() {
        check_flag_setter(0x0048_48b0, 0x40, true);
    }

    #[test]
    fn flag_0x10000_setter() {
        check_flag_setter(0x0048_4900, 0x1_0000, true);
    }

    #[test]
    fn flag_0x20000_setter() {
        check_flag_setter(0x0048_4950, 0x2_0000, false);
    }

    #[test]
    fn flag_0x100000_setter() {
        check_flag_setter(0x0048_4990, 0x10_0000, false);
    }

    #[test]
    fn flag_0x200_setter() {
        check_flag_setter(0x0048_49d0, 0x200, false);
    }

    #[test]
    fn second_flag_0x40_setter() {
        check_flag_setter(0x0048_4a10, 0x40, true);
    }

    #[test]
    fn set_fire_off_sets_0x80() {
        check_flag_setter(0x0048_4a70, 0x80, false);
    }

    #[test]
    fn flag_0x8_setter() {
        check_flag_setter(0x0048_4ab0, 0x8, false);
    }

    #[test]
    fn set_disabled_sets_0x800_and_notifies() {
        check_flag_setter(0x0048_4af0, 0x800, true);
    }

    #[test]
    fn set_empty_calls_the_slot_unless_the_stub_says_otherwise() {
        let mut e = form_engine();
        e.register(SAVE_LOAD_STUB, |_, _| ret(0));
        e.set_global(SAVE_LOAD_GAME, 0x6600_0000u32);
        let f = form(&mut e, 0, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_4580, &args![f, true]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SAVE_LOAD_STUB), vec![vec![0x6600_0000]]);
        assert_eq!(calls_to(&log, SLOT_0X48), vec![vec![f.addr(), 0x20_0000]]);
        assert!(calls_to(&log, SLOT_0X4C).is_empty());
        assert_eq!(e.get(f, TESForm::iFormFlags), 0x2000);
        e.call_log = Some(vec![]);
        e.call(0x0048_4580, &args![f, false]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, SLOT_0X4C), vec![vec![f.addr(), 0x20_0000]]);
        assert!(calls_to(&log, SLOT_0X48).is_empty());
        assert_eq!(e.get(f, TESForm::iFormFlags), 0);
        // The stub returning true suppresses the slot call.
        e.register(SAVE_LOAD_STUB, |_, _| ret(1));
        e.call_log = Some(vec![]);
        e.call(0x0048_4580, &args![f, true]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, SLOT_0X48).is_empty());
        assert_eq!(e.get(f, TESForm::iFormFlags), 0x2000);
    }

    #[test]
    fn flag_2_set_adds_the_form_to_the_array_once() {
        let mut e = form_engine();
        e.register(FORM_LIST_ADD, |_, _| ret(0));
        let f = form(&mut e, 0, 0x20);
        // No array: only the flag.
        e.call_log = Some(vec![]);
        e.call(0x0048_4730, &args![f, true]);
        assert!(calls_to(&e.call_log.take().unwrap(), FORM_LIST_ADD).is_empty());
        assert_eq!(e.get(f, TESForm::iFormFlags), 2);
        // With an array that lacks the form: added through a pointer slot.
        let g = form(&mut e, 0, 0x20);
        let list = form_array(&mut e, &[]);
        e.set_global(FORM_LIST, list);
        e.call_log = Some(vec![]);
        e.call(0x0048_4730, &args![g, true]);
        let log = e.call_log.take().unwrap();
        let adds = calls_to(&log, FORM_LIST_ADD);
        assert_eq!(adds.len(), 1);
        assert_eq!(adds[0][0], list);
        assert_eq!(e.get(g, TESForm::iFormFlags), 2);
        // Already listed: not added again.
        let listed = form(&mut e, 0, 0x20);
        let list = form_array(&mut e, &[listed]);
        e.set_global(FORM_LIST, list);
        e.call_log = Some(vec![]);
        e.call(0x0048_4730, &args![listed, true]);
        assert!(calls_to(&e.call_log.take().unwrap(), FORM_LIST_ADD).is_empty());
        // Flag already set, or temporary: not added either.
        let flagged = form(&mut e, 2, 0x20);
        let temporary = form(&mut e, 0x4000, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_4730, &args![flagged, true]);
        e.call(0x0048_4730, &args![temporary, true]);
        assert!(calls_to(&e.call_log.take().unwrap(), FORM_LIST_ADD).is_empty());
        assert_eq!(e.get(temporary, TESForm::iFormFlags), 0x4002);
    }

    #[test]
    fn flag_2_clear_removes_the_form_from_the_array() {
        let mut e = form_engine();
        e.register(FORM_LIST_REMOVE_AT, |_, _| Ret::default());
        let other = form(&mut e, 2, 0x20);
        let f = form(&mut e, 0x12, 0x20);
        let list = form_array(&mut e, &[other, f]);
        e.set_global(FORM_LIST, list);
        e.call_log = Some(vec![]);
        e.call(0x0048_4730, &args![f, false]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FORM_LIST_REMOVE_AT), vec![vec![list, 1]]);
        assert_eq!(e.get(f, TESForm::iFormFlags), 0x10);
        // A form without the flag is not searched for.
        let plain = form(&mut e, 0x10, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_4730, &args![plain, false]);
        assert!(calls_to(&e.call_log.take().unwrap(), FORM_LIST_REMOVE_AT).is_empty());
        assert_eq!(e.get(plain, TESForm::iFormFlags), 0x10);
    }

    #[test]
    fn form_id_range_check() {
        let mut e = Engine::new();
        for (id, expected) in [(0u32, false), (1, true), (0x7ff, true), (0x800, false)] {
            assert_eq!(e.call(0x0048_4b40, &args![id]).bool(), expected, "{id:#x}");
        }
    }

    #[test]
    fn change_recorders_pass_the_form_flags_and_force_word() {
        let mut e = form_engine();
        e.register(STORE_WORD, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(BGS_ADD_CHANGE, |_, _| Ret::default());
        e.register(BGS_CHANGE_SIBLING, |_, _| Ret::default());
        e.set_global(BGS_SAVE_LOAD_GAME, 0x6700_0000u32);
        let f = form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_4b60, &args![f, 0x0300_0001u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, BGS_ADD_CHANGE),
            vec![vec![0x6700_0000, f.addr(), 0x0300_0001, 0]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0048_4b90, &args![f, 0x20u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, BGS_ADD_CHANGE),
            vec![vec![0x6700_0000, f.addr(), 0x20, 1]]
        );
        e.call_log = Some(vec![]);
        e.call(0x0048_4bc0, &args![f, 0x44u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, BGS_CHANGE_SIBLING),
            vec![vec![0x6700_0000, f.addr(), 0x44]]
        );
    }

    // ---- second batch: record writer, source file list, save/load helpers ----

    /// Test vtable with the slots the second batch calls, and the doubles that
    /// stand behind them.
    const WIDE_VTABLE: u32 = 0x0200_1000;
    const WIDE_SET_FLAG_2: u32 = 0x0f01_00c8;
    const WIDE_AFTER_START: u32 = 0x0f01_00dc;
    const WIDE_IS_REFERENCE: u32 = 0x0f01_00f0;
    const WIDE_NAME: u32 = 0x0f01_0130;
    /// First word of a test file that `FILE_IS_MASTER` accepts.
    const MASTER_MARK: u32 = 0x4d41_5354;
    const SAVE_LOAD_OBJECT: u32 = 0x6800_0000;
    /// Where `load_game_engine` keeps the word the load double hands out.
    const SAVED_WORD_ADDRESS: u32 = 0x0200_2000;

    fn wide_engine() -> Engine {
        let mut e = form_engine();
        let mut slots = vec![0u32; 0x134 / 4];
        slots[0xc8 / 4] = WIDE_SET_FLAG_2;
        slots[0xdc / 4] = WIDE_AFTER_START;
        slots[0xf0 / 4] = WIDE_IS_REFERENCE;
        slots[0x130 / 4] = WIDE_NAME;
        e.put_vtable(WIDE_VTABLE, &slots);
        for slot in [WIDE_SET_FLAG_2, WIDE_AFTER_START, WIDE_IS_REFERENCE] {
            e.register(slot, |_, _| Ret::default());
        }
        e.register(WIDE_NAME, |_, a| ret(a[0]));
        e.register(FORM_FLAGS, |e, a| ret(e.mem.u32(a[0] + 8)));
        e.register(FORM_TYPE_NAME, |e, a| {
            ret(0x7000_0000 + e.mem.u8(a[0] + 4) as u32)
        });
        e
    }

    /// A form with the wide test vtable.
    fn wide_form(e: &mut Engine, flags: u32, form_type: u8) -> Ptr<TESForm> {
        let f = form(e, flags, form_type);
        e.mem.set_u32(f.addr(), WIDE_VTABLE);
        f
    }

    fn list_pop_double(e: &mut Engine, a: &[u32]) -> Ret {
        let node = a[0];
        let next = e.mem.u32(node + 4);
        if next == 0 {
            e.mem.set_u32(node, 0);
        } else {
            let item = e.mem.u32(next);
            let after = e.mem.u32(next + 4);
            e.mem.set_u32(node, item);
            e.mem.set_u32(node + 4, after);
        }
        Ret::default()
    }

    fn list_remove_item_double(e: &mut Engine, a: &[u32]) -> Ret {
        let head = a[0];
        let item = e.mem.u32(a[1]);
        if item == 0 || (e.mem.u32(head + 4) == 0 && e.mem.u32(head) == 0) {
            return Ret::default();
        }
        let mut previous = head;
        let mut node = head;
        while node != 0 && e.mem.u32(node) != item {
            previous = node;
            node = e.mem.u32(node + 4);
        }
        if node == 0 {
            return Ret::default();
        }
        if node == head {
            list_pop_double(e, &[head]);
        } else {
            let after = e.mem.u32(node + 4);
            e.mem.set_u32(previous + 4, after);
        }
        Ret::default()
    }

    fn list_append_double(e: &mut Engine, a: &[u32]) -> Ret {
        let item = e.mem.u32(a[1]);
        if item == 0 {
            return Ret::default();
        }
        let mut tail = a[0];
        while e.mem.u32(tail + 4) != 0 {
            tail = e.mem.u32(tail + 4);
        }
        if e.mem.u32(tail) == 0 {
            e.mem.set_u32(tail, item);
        } else {
            let node = e.mem.alloc(8);
            e.mem.set_u32(node, item);
            e.mem.set_u32(tail + 4, node);
        }
        Ret::default()
    }

    fn list_push_front_double(e: &mut Engine, a: &[u32]) -> Ret {
        let head = a[0];
        let item = e.mem.u32(a[1]);
        if item == 0 {
            return Ret::default();
        }
        if e.mem.u32(head) != 0 {
            let node = e.mem.alloc(8);
            let old_item = e.mem.u32(head);
            let old_next = e.mem.u32(head + 4);
            e.mem.set_u32(node, old_item);
            e.mem.set_u32(node + 4, old_next);
            e.mem.set_u32(head + 4, node);
        }
        e.mem.set_u32(head, item);
        Ret::default()
    }

    /// `wide_engine` plus doubles that behave like the game's list nodes
    /// (item at +0, next at +4) and its files.
    fn list_engine() -> Engine {
        let mut e = wide_engine();
        e.register(LIST_FIRST_NODE, |_, a| ret(a[0] + 0x10));
        e.register(LIST_NODE_ITEM, |_, a| ret(a[0]));
        e.register(LIST_NODE_NEXT, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(LIST_NODE_EMPTY, |e, a| {
            ret((e.mem.u32(a[0] + 4) == 0 && e.mem.u32(a[0]) == 0) as u32)
        });
        e.register(FILE_IS_MASTER, |e, a| {
            ret((a[0] != 0 && e.mem.u32(a[0]) == MASTER_MARK) as u32)
        });
        e.register(FILE_ROOT, |_, _| ret(0));
        e.register(LIST_POP_NODE, list_pop_double);
        e.register(LIST_REMOVE_ITEM, list_remove_item_double);
        e.register(LIST_APPEND, list_append_double);
        e.register(LIST_PUSH_FRONT, list_push_front_double);
        e
    }

    /// A test file (0x400 bytes, so its master list at +0x3ec fits).
    fn test_file(e: &mut Engine, master: bool) -> u32 {
        let file = e.mem.alloc(0x400);
        if master {
            e.mem.set_u32(file, MASTER_MARK);
        }
        file
    }

    /// Makes the form's source file list hold exactly `items`.
    fn set_list(e: &mut Engine, form: Ptr<TESForm>, items: &[u32]) {
        let mut node = form.addr() + 0x10;
        e.mem.set_u32(node, items.first().copied().unwrap_or(0));
        e.mem.set_u32(node + 4, 0);
        for item in items.iter().skip(1) {
            let next = e.mem.alloc(8);
            e.mem.set_u32(next, *item);
            e.mem.set_u32(next + 4, 0);
            e.mem.set_u32(node + 4, next);
            node = next;
        }
    }

    /// The items of the form's source file list, in order.
    fn list_items(e: &Engine, form: Ptr<TESForm>) -> Vec<u32> {
        let mut items = vec![];
        let mut node = form.addr() + 0x10;
        while node != 0 {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        items
    }

    #[test]
    fn change_flag_size_is_four_when_bit_0_is_set() {
        let mut e = wide_engine();
        let f = wide_form(&mut e, 8, 0x20);
        assert_eq!(e.call(0x0048_4bf0, &args![f, 1u32]).u16(), 4);
        assert_eq!(e.call(0x0048_4bf0, &args![f, 3u32]).u16(), 4);
        assert_eq!(e.call(0x0048_4bf0, &args![f, 2u32]).u16(), 0);
        assert_eq!(e.call(0x0048_4bf0, &args![f, 0u32]).u16(), 0);
    }

    #[test]
    fn save_of_flags_forwards_the_flags_word_only_for_bit_0() {
        let mut e = wide_engine();
        e.set_global(SAVE_LOAD_GAME, SAVE_LOAD_OBJECT);
        e.register(SAVE_LOAD_SAVE_DATA, |_, _| Ret::default());
        let f = wide_form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_4c20, &args![f, 1u32]);
        e.call(0x0048_4c20, &args![f, 2u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SAVE_LOAD_SAVE_DATA),
            vec![vec![SAVE_LOAD_OBJECT, f.addr() + 8, 4]]
        );
    }

    fn load_game_engine(saved: u32) -> Engine {
        let mut e = wide_engine();
        e.set_global(SAVE_LOAD_GAME, SAVE_LOAD_OBJECT);
        e.map(SAVED_WORD_ADDRESS, 0x1000);
        e.mem.set_u32(SAVED_WORD_ADDRESS, saved);
        e.register(SAVE_LOAD_LOAD_DATA, |e, a| {
            let saved = e.mem.u32(SAVED_WORD_ADDRESS);
            e.mem.set_u32(a[1], saved);
            Ret::default()
        });
        e
    }

    #[test]
    fn load_game_merges_the_saved_flags_by_form_kind() {
        // A reference takes the bits 0x912860 of the saved word.
        let mut e = load_game_engine(0x1234_5678);
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        let f = wide_form(&mut e, 0x8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_4c50, &args![f, 1u32, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![f.addr(), 0, 0x0118_3028, 0x0118_41cc, 0]]
        );
        let loads = calls_to(&log, SAVE_LOAD_LOAD_DATA);
        assert_eq!(loads.len(), 1);
        assert_eq!((loads[0][0], loads[0][2]), (SAVE_LOAD_OBJECT, 4));
        assert_eq!(
            e.get(f, TESForm::iFormFlags),
            8 | (0x1234_5678 & 0x0091_2860)
        );
        // Any other form takes the bits 0x40000c20.
        let mut e = load_game_engine(0x1234_5678);
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        let g = wide_form(&mut e, 0x8, 0x20);
        e.call(0x0048_4c50, &args![g, 1u32, 0u32]);
        assert_eq!(
            e.get(g, TESForm::iFormFlags),
            8 | (0x1234_5678 & 0x4000_0c20)
        );
        // Without bit 0 nothing is read.
        let h = wide_form(&mut e, 0x8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_4c50, &args![h, 2u32, 0u32]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.get(h, TESForm::iFormFlags), 8);
    }

    /// Forwarding helpers: the call reaches the `TESSaveLoadGame` object's
    /// method with its own two words.
    fn check_forward(addr: u32, target: u32) {
        let mut e = wide_engine();
        e.set_global(SAVE_LOAD_GAME, SAVE_LOAD_OBJECT);
        e.register(target, |_, _| Ret::default());
        let f = wide_form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(addr, &args![f, 0x1111u32, 0x2222u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, target),
            vec![vec![SAVE_LOAD_OBJECT, 0x1111, 0x2222]]
        );
    }

    #[test]
    fn save_game_data_old_forwards_to_the_save_load_object() {
        check_forward(0x0048_4ce0, SAVE_LOAD_SAVE_DATA);
    }

    #[test]
    fn load_game_data_old_forwards_to_the_save_load_object() {
        check_forward(0x0048_4d00, SAVE_LOAD_LOAD_DATA);
    }

    #[test]
    fn save_numeric_id_forwards_to_the_save_load_object() {
        check_forward(0x0048_4d20, SAVE_LOAD_SAVE_NUMERIC_ID);
    }

    #[test]
    fn load_numeric_id_forwards_to_the_save_load_object() {
        check_forward(0x0048_4d40, SAVE_LOAD_LOAD_NUMERIC_ID);
    }

    /// Doubles for the save/load buffer object calls of `00484d60` and
    /// `00484da0`: the buffer's save kind is the word at +0x17.
    fn buffer_engine() -> (Engine, u32) {
        let mut e = wide_engine();
        e.register(BUFFER_SAVE_KIND, |e, a| {
            let kind = e.mem.u32(a[0] + 0x17);
            e.mem.set_u32(a[1], kind);
            ret(a[1])
        });
        e.register(WORD_HAS_BITS, |e, a| {
            ret((e.mem.u32(a[0]) & a[1] != 0) as u32)
        });
        e.register(SAVE_BUFFER_WRITE, |_, _| Ret::default());
        e.register(LOAD_BUFFER_READ, |e, a| {
            e.mem.set_u32(a[1], 0x1234_5678);
            Ret::default()
        });
        let buffer = e.mem.alloc(0x20);
        (e, buffer)
    }

    #[test]
    fn save_to_buffer_writes_the_flags_when_the_kind_has_bit_0() {
        let (mut e, buffer) = buffer_engine();
        let f = wide_form(&mut e, 8, 0x20);
        e.mem.set_u32(buffer + 0x17, 1);
        e.call_log = Some(vec![]);
        e.call(0x0048_4d60, &args![f, buffer]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, SAVE_BUFFER_WRITE),
            vec![vec![buffer, f.addr() + 8, 4, 0]]
        );
        e.mem.set_u32(buffer + 0x17, 2);
        e.call_log = Some(vec![]);
        e.call(0x0048_4d60, &args![f, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), SAVE_BUFFER_WRITE).is_empty());
    }

    #[test]
    fn load_from_buffer_merges_the_flags_by_the_reference_virtual() {
        let (mut e, buffer) = buffer_engine();
        e.mem.set_u32(buffer + 0x17, 1);
        let plain = wide_form(&mut e, 8, 0x20);
        e.call(0x0048_4da0, &args![plain, buffer]);
        assert_eq!(
            e.get(plain, TESForm::iFormFlags),
            8 | (0x1234_5678 & 0x4000_0c20)
        );
        e.register(WIDE_IS_REFERENCE, |_, _| ret(1));
        let reference = wide_form(&mut e, 8, 0x20);
        e.call(0x0048_4da0, &args![reference, buffer]);
        assert_eq!(
            e.get(reference, TESForm::iFormFlags),
            8 | (0x1234_5678 & 0x0091_2860)
        );
        // Kind without bit 0: no read, flags untouched.
        e.mem.set_u32(buffer + 0x17, 0);
        let untouched = wide_form(&mut e, 0x77, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_4da0, &args![untouched, buffer]);
        assert!(calls_to(&e.call_log.take().unwrap(), LOAD_BUFFER_READ).is_empty());
        assert_eq!(e.get(untouched, TESForm::iFormFlags), 0x77);
    }

    #[test]
    fn form_id_in_reserved_range_is_checked_on_the_forms_own_id() {
        let mut e = form_engine();
        let f = form(&mut e, 8, 0x20);
        for (id, expected) in [(0u32, false), (0x10, true), (0x7ff, true), (0x800, false)] {
            e.set(f, TESForm::iFormID, id);
            assert_eq!(e.call(0x0048_4e40, &args![f]).bool(), expected, "{id:#x}");
        }
    }

    #[test]
    fn get_file_returns_the_indexth_non_null_file() {
        let mut e = list_engine();
        let f = wide_form(&mut e, 8, 0x20);
        let (a, b, c) = (
            test_file(&mut e, false),
            test_file(&mut e, false),
            test_file(&mut e, false),
        );
        set_list(&mut e, f, &[0, a, 0, b, c]);
        assert_eq!(e.call(0x0048_4e60, &args![f, 0u32]).u32(), a);
        assert_eq!(e.call(0x0048_4e60, &args![f, 1u32]).u32(), b);
        assert_eq!(e.call(0x0048_4e60, &args![f, 2u32]).u32(), c);
        // Past the end, and with -1, it is the last non-null file.
        assert_eq!(e.call(0x0048_4e60, &args![f, 9u32]).u32(), c);
        assert_eq!(e.call(0x0048_4e60, &args![f, -1i32]).u32(), c);
        set_list(&mut e, f, &[]);
        assert_eq!(e.call(0x0048_4e60, &args![f, 0u32]).u32(), 0);
    }

    #[test]
    fn owner_master_is_the_last_master_in_the_list() {
        let mut e = list_engine();
        let f = wide_form(&mut e, 8, 0x20);
        let plain = test_file(&mut e, false);
        let first = test_file(&mut e, true);
        let second = test_file(&mut e, true);
        set_list(&mut e, f, &[plain, first, 0, second, plain]);
        assert_eq!(e.call(0x0048_4ee0, &args![f]).u32(), second);
        set_list(&mut e, f, &[plain]);
        assert_eq!(e.call(0x0048_4ee0, &args![f]).u32(), 0);
        set_list(&mut e, f, &[]);
        assert_eq!(e.call(0x0048_4ee0, &args![f]).u32(), 0);
    }

    #[test]
    fn set_file_with_no_file_pops_the_last_node_with_a_file() {
        let mut e = list_engine();
        let f = wide_form(&mut e, 8, 0x20);
        let (a, b) = (test_file(&mut e, false), test_file(&mut e, false));
        set_list(&mut e, f, &[a, b]);
        e.call(0x0048_4f50, &args![f, 0u32]);
        assert_eq!(list_items(&e, f), vec![a, 0]);
        // Without any file the first node is the one popped.
        set_list(&mut e, f, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0048_4f50, &args![f, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LIST_POP_NODE), vec![vec![f.addr() + 0x10]]);
    }

    #[test]
    fn set_file_appends_a_plain_file_once() {
        let mut e = list_engine();
        let f = wide_form(&mut e, 8, 0x20);
        let (a, b, c) = (
            test_file(&mut e, false),
            test_file(&mut e, false),
            test_file(&mut e, false),
        );
        set_list(&mut e, f, &[a, b]);
        e.call(0x0048_4f50, &args![f, c]);
        assert_eq!(list_items(&e, f), vec![a, b, c]);
        // A file already in the list is left alone.
        e.call(0x0048_4f50, &args![f, b]);
        assert_eq!(list_items(&e, f), vec![a, b, c]);
        // A list without files gets it at the front.
        set_list(&mut e, f, &[]);
        e.call_log = Some(vec![]);
        e.call(0x0048_4f50, &args![f, a]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, LIST_PUSH_FRONT).len(), 1);
        assert_eq!(list_items(&e, f), vec![a]);
    }

    #[test]
    fn set_file_puts_a_master_after_the_other_masters() {
        let mut e = list_engine();
        let f = wide_form(&mut e, 8, 0x20);
        let plain = test_file(&mut e, false);
        let other_plain = test_file(&mut e, false);
        let master = test_file(&mut e, true);
        let new_master = test_file(&mut e, true);
        set_list(&mut e, f, &[plain, master, other_plain]);
        e.call(0x0048_4f50, &args![f, new_master]);
        // The plain files are moved out of the way; the new master ends up
        // last.
        assert_eq!(list_items(&e, f), vec![master, new_master]);
    }

    #[test]
    fn set_file_replaces_the_file_by_its_chain_end() {
        let mut e = list_engine();
        let f = wide_form(&mut e, 8, 0x20);
        let chain_end = test_file(&mut e, false);
        let start = test_file(&mut e, false);
        e.mem.set_u32(start + 4, chain_end);
        e.register(FILE_ROOT, |e, a| ret(e.mem.u32(a[0] + 4)));
        set_list(&mut e, f, &[]);
        e.call(0x0048_4f50, &args![f, start]);
        assert_eq!(list_items(&e, f), vec![chain_end]);
    }

    #[test]
    fn load_form_asks_the_file_to_load_the_form() {
        let mut e = wide_engine();
        e.register(FILE_LOAD_FORM, |_, _| Ret::default());
        let f = wide_form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_5110, &args![f, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, FILE_LOAD_FORM), vec![vec![0x5000, f.addr()]]);
    }

    #[test]
    fn copy_and_compare_log_the_missing_override() {
        let mut e = wide_engine();
        e.register(LOG_MESSAGE, |_, _| Ret::default());
        let this = wide_form(&mut e, 8, 0x20);
        let other = wide_form(&mut e, 8, 0x21);
        e.call_log = Some(vec![]);
        e.call(0x0048_5130, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![0x0101_c930, other.addr(), 0x7000_0021]]
        );
        e.call_log = Some(vec![]);
        let result = e.call(0x0048_5170, &args![this, other]).bool();
        let log = e.call_log.take().unwrap();
        assert!(!result);
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![0x0101_c978, other.addr(), 0x7000_0021]]
        );
    }

    fn components_engine() -> Engine {
        let mut e = wide_engine();
        e.register(COMPONENTS_CONSTRUCT, |_, _| Ret::default());
        e.register(COMPONENTS_INIT, |_, _| Ret::default());
        e.register(COMPONENTS_COPY, |_, _| Ret::default());
        e.register(COMPONENTS_COMPARE, |_, _| ret(1));
        e
    }

    #[test]
    fn copy_all_components_copies_between_two_collections() {
        let mut e = components_engine();
        let this = wide_form(&mut e, 8, 0x20);
        let other = wide_form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_51b0, &args![this, other]);
        let log = e.call_log.take().unwrap();
        let built = calls_to(&log, COMPONENTS_CONSTRUCT);
        assert_eq!(built.len(), 2);
        let (mine, theirs) = (built[0][0], built[1][0]);
        assert_ne!(mine, theirs);
        assert_eq!(
            calls_to(&log, COMPONENTS_INIT),
            vec![vec![mine, this.addr()], vec![theirs, other.addr()]]
        );
        assert_eq!(calls_to(&log, COMPONENTS_COPY), vec![vec![mine, theirs]]);
    }

    #[test]
    fn compare_all_components_is_true_without_another_form() {
        let mut e = components_engine();
        let this = wide_form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_5270, &args![this, 0u32]).bool());
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        let other = wide_form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_5270, &args![this, other]).bool());
        let log = e.call_log.take().unwrap();
        let built = calls_to(&log, COMPONENTS_CONSTRUCT);
        assert_eq!(built.len(), 2);
        assert_eq!(
            calls_to(&log, COMPONENTS_INIT),
            vec![
                vec![built[0][0], this.addr()],
                vec![built[1][0], other.addr()]
            ]
        );
        assert_eq!(
            calls_to(&log, COMPONENTS_COMPARE),
            vec![vec![built[0][0], built[1][0]]]
        );
        // Only the low byte of the result counts.
        e.register(COMPONENTS_COMPARE, |_, _| ret(0x100));
        assert!(!e.call(0x0048_5270, &args![this, other]).bool());
    }

    #[test]
    fn copy_from_component_takes_type_and_flags_of_the_other_form() {
        let mut e = wide_engine();
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        let this = wide_form(&mut e, 0x4008, 0x05);
        let other = wide_form(&mut e, 0x0102, 0x2a);
        e.call_log = Some(vec![]);
        e.call(0x0048_5340, &args![this, other]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, RT_DYNAMIC_CAST),
            vec![vec![other.addr(), 0, 0x0118_3040, 0x0118_3028, 0]]
        );
        // Flag 2 differs: the virtual at +0xc8 gets the other form's state.
        assert_eq!(calls_to(&log, WIDE_SET_FLAG_2), vec![vec![this.addr(), 1]]);
        assert_eq!(e.get(this, TESForm::cFormType), 0x2a);
        assert_eq!(e.get(this, TESForm::iFormFlags), 0x4102);
        // Same flag 2 state: no virtual call.
        let same = wide_form(&mut e, 0x0, 0x05);
        let same_other = wide_form(&mut e, 0x0100, 0x06);
        e.call_log = Some(vec![]);
        e.call(0x0048_5340, &args![same, same_other]);
        assert!(calls_to(&e.call_log.take().unwrap(), WIDE_SET_FLAG_2).is_empty());
        assert_eq!(e.get(same, TESForm::iFormFlags), 0x0100);
        // Not a form: nothing changes.
        let untouched = wide_form(&mut e, 0x0, 0x07);
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        e.call(0x0048_5340, &args![untouched, same_other]);
        assert_eq!(e.get(untouched, TESForm::cFormType), 0x07);
    }

    fn difference_engine() -> Engine {
        let mut e = wide_engine();
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        e.register(FORM_NAME_KEY, |e, a| ret(e.mem.u32(a[0] + 0x10)));
        e.register(STRING_COMPARE, |_, _| ret(0));
        e
    }

    #[test]
    fn difference_check_stops_at_the_first_difference() {
        let mut e = difference_engine();
        let this = wide_form(&mut e, 0x8, 0x20);
        let same = wide_form(&mut e, 0x4001, 0x20);
        // Not a form: different.
        e.register(RT_DYNAMIC_CAST, |_, _| ret(0));
        assert!(e.call(0x0048_53e0, &args![this, same]).bool());
        e.register(RT_DYNAMIC_CAST, |_, a| ret(a[0]));
        // Same type, no name keys, flags equal outside the masked bits.
        assert!(!e.call(0x0048_53e0, &args![this, same]).bool());
        // Different type.
        let other_type = wide_form(&mut e, 0x8, 0x21);
        assert!(e.call(0x0048_53e0, &args![this, other_type]).bool());
        // A flag outside the mask.
        let other_flags = wide_form(&mut e, 0x10, 0x20);
        assert!(e.call(0x0048_53e0, &args![this, other_flags]).bool());
    }

    #[test]
    fn difference_check_compares_name_keys_and_names() {
        let mut e = difference_engine();
        let this = wide_form(&mut e, 0x8, 0x20);
        let other = wide_form(&mut e, 0x8, 0x20);
        // Keys are the words at +0x10: both non-zero and equal, names equal
        // by the string comparison double.
        e.mem.set_u32(this.addr() + 0x10, 5);
        e.mem.set_u32(other.addr() + 0x10, 5);
        e.call_log = Some(vec![]);
        assert!(!e.call(0x0048_53e0, &args![this, other]).bool());
        let log = e.call_log.take().unwrap();
        // The names (virtual +0x130) are compared this form first.
        assert_eq!(
            calls_to(&log, STRING_COMPARE),
            vec![vec![this.addr(), other.addr()]]
        );
        // Keys differ.
        e.mem.set_u32(other.addr() + 0x10, 6);
        assert!(e.call(0x0048_53e0, &args![this, other]).bool());
        // Only the other form has a key.
        e.mem.set_u32(this.addr() + 0x10, 0);
        assert!(e.call(0x0048_53e0, &args![this, other]).bool());
        // Equal keys, different names.
        e.mem.set_u32(this.addr() + 0x10, 5);
        e.mem.set_u32(other.addr() + 0x10, 5);
        e.register(STRING_COMPARE, |_, _| ret(1));
        assert!(e.call(0x0048_53e0, &args![this, other]).bool());
    }

    const GROUP_WORD: u32 = 0x5055_5247;

    fn group_engine() -> (Engine, Ptr<TESForm>, u32) {
        let mut e = wide_engine();
        e.set_global(GROUP_TAG, GROUP_WORD);
        e.mem
            .set_u32(FORM_ENUM_TABLE + 0x20 * FORM_ENUM_STRIDE + 8, 0x4c41_5645);
        let f = wide_form(&mut e, 8, 0x20);
        let record = e.mem.alloc(0x18);
        (e, f, record)
    }

    #[test]
    fn group_check_matches_the_top_level_group_of_the_forms_type() {
        let (mut e, f, record) = group_engine();
        e.mem.set_u32(record, GROUP_WORD);
        e.mem.set_u32(record + 8, 0x4c41_5645);
        e.mem.set_u32(record + 0xc, 0);
        assert!(e.call(0x0048_54e0, &args![f, record, 0u32, 0u32]).bool());
        // Null record, another tag, another group type, another label.
        assert!(!e.call(0x0048_54e0, &args![f, 0u32, 0u32, 0u32]).bool());
        e.mem.set_u32(record, GROUP_WORD + 1);
        assert!(!e.call(0x0048_54e0, &args![f, record, 0u32, 0u32]).bool());
        e.mem.set_u32(record, GROUP_WORD);
        e.mem.set_u32(record + 0xc, 1);
        assert!(!e.call(0x0048_54e0, &args![f, record, 0u32, 0u32]).bool());
        e.mem.set_u32(record + 0xc, 0);
        e.mem.set_u32(record + 8, 0x4c41_5646);
        assert!(!e.call(0x0048_54e0, &args![f, record, 0u32, 0u32]).bool());
    }

    #[test]
    fn group_fill_writes_the_top_level_group_header() {
        let (mut e, f, record) = group_engine();
        for offset in (0..0x18).step_by(4) {
            e.mem.set_u32(record + offset, 0xaaaa_aaaa);
        }
        e.call(0x0048_5530, &args![f, record, 0u32]);
        assert_eq!(e.mem.u32(record), GROUP_WORD);
        assert_eq!(e.mem.u32(record + 4), 0);
        assert_eq!(e.mem.u32(record + 8), 0x4c41_5645);
        assert_eq!(e.mem.u32(record + 0xc), 0);
        assert_eq!(e.mem.u32(record + 0x10), 0);
        assert_eq!(e.mem.u16(record + 0x14), 0);
        assert_eq!(e.mem.u16(record + 0x16), 0);
        // A non-zero flag word or a null record leaves everything alone.
        e.mem.set_u32(record, 0xaaaa_aaaa);
        e.call(0x0048_5530, &args![f, record, 1u32]);
        assert_eq!(e.mem.u32(record), 0xaaaa_aaaa);
        e.call(0x0048_5530, &args![f, 0u32, 0u32]);
    }

    fn start_engine() -> Engine {
        let mut e = wide_engine();
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(RECORD_VERSION, |_, _| ret(0xf));
        e.mem
            .set_u32(FORM_ENUM_TABLE + 0x20 * FORM_ENUM_STRIDE + 8, 0x4c41_5645);
        e.mem
            .set_u32(FORM_ENUM_TABLE + FORM_ENUM_STRIDE + 8, 0x3254_4553);
        e
    }

    #[test]
    fn start_form_writes_a_record_header() {
        let mut e = start_engine();
        let f = wide_form(&mut e, 0xffff_bff8, 0x20);
        e.set(f, TESForm::iFormID, 0x0100_0abc);
        e.call_log = Some(vec![]);
        e.call(0x0048_55a0, &args![f]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.global::<u32>(SAVE_BUFFER_SIZE), 0x18);
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x18]]);
        let header = e.global::<u32>(SAVE_BUFFER);
        assert_eq!(e.mem.u32(header), 0x4c41_5645);
        assert_eq!(e.mem.u32(header + 4), 0);
        assert_eq!(e.mem.u32(header + 8), 0xffff_bff8 & 0x3003_2fe0);
        assert_eq!(e.mem.u32(header + 0xc), 0x0100_0abc);
        assert_eq!(e.mem.u32(header + 0x10), 0);
        assert_eq!(e.mem.u16(header + 0x14), 0xf);
        assert_eq!(e.mem.u16(header + 0x16), 0);
        assert_eq!(calls_to(&log, WIDE_AFTER_START), vec![vec![f.addr()]]);
    }

    #[test]
    fn start_form_keeps_all_flags_of_type_1_and_skips_temporary_forms() {
        let mut e = start_engine();
        let first = wide_form(&mut e, 0xffff_bff8, 0x01);
        e.call(0x0048_55a0, &args![first]);
        let header = e.global::<u32>(SAVE_BUFFER);
        assert_eq!(e.mem.u32(header + 8), 0xffff_bff8);
        assert_eq!(e.mem.u32(header), 0x3254_4553);
        let temporary = wide_form(&mut e, 0x4000, 0x20);
        e.set_global(SAVE_BUFFER, 0u32);
        e.call_log = Some(vec![]);
        e.call(0x0048_55a0, &args![temporary]);
        assert!(calls_to(&e.call_log.take().unwrap(), OPERATOR_NEW).is_empty());
        assert_eq!(e.global::<u32>(SAVE_BUFFER), 0);
    }

    #[test]
    fn close_form_stores_the_data_size_and_swaps_on_big_endian() {
        let mut e = wide_engine();
        e.register(FORM_ENDIAN, |_, _| Ret::default());
        let header = e.mem.alloc(0x40);
        e.set_global(SAVE_BUFFER, header);
        e.set_global(SAVE_BUFFER_SIZE, 0x30u32);
        let f = wide_form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_5680, &args![f]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(header + 4), 0x18);
        assert!(calls_to(&log, FORM_ENDIAN).is_empty());
        e.set_global(BIG_ENDIAN_FLAG, 1u8);
        e.set_global(SAVE_BUFFER_SIZE, 0x20u32);
        e.call_log = Some(vec![]);
        e.call(0x0048_5680, &args![f]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(header + 4), 8);
        assert_eq!(calls_to(&log, FORM_ENDIAN), vec![vec![header]]);
        // A temporary form leaves the header alone.
        let temporary = wide_form(&mut e, 0x4000, 0x20);
        e.set_global(SAVE_BUFFER_SIZE, 0x38u32);
        e.call(0x0048_5680, &args![temporary]);
        assert_eq!(e.mem.u32(header + 4), 8);
    }

    /// An engine with a record buffer of `capacity` bytes (header already
    /// written, `0x18` bytes used), a pass-through realloc and byte-swap
    /// doubles. Returns the engine and the buffer address.
    fn chunk_engine(big_endian: bool, capacity: u32) -> (Engine, u32) {
        let mut e = form_engine();
        let buffer = e.mem.alloc(capacity);
        e.set_global(SAVE_BUFFER, buffer);
        e.set_global(SAVE_BUFFER_SIZE, 0x18u32);
        e.set_global(BIG_ENDIAN_FLAG, big_endian as u8);
        e.register(BUFFER_REALLOC, |_, a| ret(a[0]));
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        e.register(SWAP_WORD, |e, a| {
            let value = e.mem.u32(a[0]).swap_bytes();
            e.mem.set_u32(a[0], value);
            Ret::default()
        });
        e.register(SWAP_HALF, |e, a| {
            let value = e.mem.u16(a[0]).swap_bytes();
            e.mem.set_u16(a[0], value);
            Ret::default()
        });
        (e, buffer)
    }

    /// The bytes of the record buffer after the header.
    fn chunks(e: &Engine, buffer: u32) -> Vec<u8> {
        let size = e.global::<u32>(SAVE_BUFFER_SIZE);
        e.mem.bytes(buffer + 0x18, size - 0x18)
    }

    #[test]
    fn add_chunk_data_appends_a_tagged_chunk() {
        let (mut e, buffer) = chunk_engine(false, 0x100);
        let data = e.mem.alloc(8);
        e.mem.write(data, &[1, 2, 3]);
        e.call_log = Some(vec![]);
        e.call(0x0048_5990, &args![0x4443_4241u32, data, 3u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.global::<u32>(SAVE_BUFFER_SIZE), 0x21);
        assert_eq!(chunks(&e, buffer), b"ABCD\x03\x00\x01\x02\x03");
        assert_eq!(calls_to(&log, BUFFER_REALLOC), vec![vec![buffer, 0x21]]);
        // A second chunk follows the first.
        e.call(0x0048_5990, &args![0x4443_4241u32, data, 1u32]);
        assert_eq!(e.global::<u32>(SAVE_BUFFER_SIZE), 0x21 + 7);
    }

    #[test]
    fn add_chunk_data_swaps_the_header_on_big_endian() {
        let (mut e, buffer) = chunk_engine(true, 0x100);
        let data = e.mem.alloc(8);
        e.mem.write(data, &[1, 2, 3]);
        e.register(SWAP_CHUNK_HEADER, |e, a| {
            let tag = e.mem.u32(a[0]).swap_bytes();
            e.mem.set_u32(a[0], tag);
            let size = e.mem.u16(a[0] + 4).swap_bytes();
            e.mem.set_u16(a[0] + 4, size);
            Ret::default()
        });
        e.call(0x0048_5990, &args![0x4443_4241u32, data, 3u32]);
        assert_eq!(chunks(&e, buffer), b"DCBA\x00\x03\x01\x02\x03");
    }

    #[test]
    fn add_chunk_data_writes_the_real_size_of_a_huge_chunk_first() {
        let (mut e, buffer) = chunk_engine(false, 0x10100);
        let data = e.mem.alloc(0x10000);
        e.mem.set_u8(data, 0x5a);
        e.call(0x0048_5990, &args![0x4443_4241u32, data, 0x1_0000u32]);
        assert_eq!(e.global::<u32>(SAVE_BUFFER_SIZE), 0x18 + 10 + 6 + 0x1_0000);
        // "XXXX", size 4, the real size; then the tag with size 0 and the data.
        assert_eq!(
            e.mem.bytes(buffer + 0x18, 10),
            b"XXXX\x04\x00\x00\x00\x01\x00"
        );
        assert_eq!(e.mem.bytes(buffer + 0x22, 6), b"ABCD\x00\x00");
        assert_eq!(e.mem.u8(buffer + 0x28), 0x5a);
    }

    #[test]
    fn add_chunk_without_data_has_size_zero() {
        let (mut e, buffer) = chunk_engine(false, 0x100);
        e.call(0x0048_56d0, &args![0x4443_4241u32]);
        assert_eq!(chunks(&e, buffer), b"ABCD\x00\x00");
    }

    #[test]
    fn add_chunk_array_adds_the_given_bytes() {
        let (mut e, buffer) = chunk_engine(false, 0x100);
        let data = e.mem.alloc(8);
        e.mem.write(data, &[9, 8]);
        e.call(0x0048_56f0, &args![0x4443_4241u32, data, 2u32]);
        assert_eq!(chunks(&e, buffer), b"ABCD\x02\x00\x09\x08");
    }

    fn word_array(e: &mut Engine) -> u32 {
        let data = e.mem.alloc(8);
        e.mem.set_u32(data, 0x0102_0304);
        e.mem.set_u32(data + 4, 0x0506_0708);
        data
    }

    #[test]
    fn add_chunk_array32_copies_and_swaps_on_big_endian() {
        for (address, name) in [(0x0048_5750u32, "array32"), (0x0048_5710, "array_ov2")] {
            let (mut e, buffer) = chunk_engine(false, 0x100);
            let data = word_array(&mut e);
            e.call(address, &args![0x4443_4241u32, data, 2u32]);
            assert_eq!(
                chunks(&e, buffer),
                b"ABCD\x08\x00\x04\x03\x02\x01\x08\x07\x06\x05",
                "{name}"
            );
            let (mut e, buffer) = chunk_engine(true, 0x100);
            e.register(SWAP_CHUNK_HEADER, |_, _| Ret::default());
            let data = word_array(&mut e);
            e.call_log = Some(vec![]);
            e.call(address, &args![0x4443_4241u32, data, 2u32]);
            let log = e.call_log.take().unwrap();
            // The values are swapped in a copy, not in the caller's array.
            assert_eq!(e.mem.u32(data), 0x0102_0304);
            assert_eq!(
                chunks(&e, buffer),
                b"ABCD\x08\x00\x01\x02\x03\x04\x05\x06\x07\x08",
                "{name}"
            );
            assert_eq!(calls_to(&log, SWAP_WORD).len(), 2);
        }
    }

    #[test]
    fn add_chunk_array16_copies_and_swaps_on_big_endian() {
        for address in [0x0048_5820u32, 0x0048_5730] {
            let (mut e, buffer) = chunk_engine(false, 0x100);
            let data = word_array(&mut e);
            e.call(address, &args![0x4443_4241u32, data, 3u32]);
            assert_eq!(chunks(&e, buffer), b"ABCD\x06\x00\x04\x03\x02\x01\x08\x07");
            let (mut e, buffer) = chunk_engine(true, 0x100);
            e.register(SWAP_CHUNK_HEADER, |_, _| Ret::default());
            let data = word_array(&mut e);
            e.call_log = Some(vec![]);
            e.call(address, &args![0x4443_4241u32, data, 3u32]);
            let log = e.call_log.take().unwrap();
            assert_eq!(calls_to(&log, SWAP_HALF).len(), 3);
            assert_eq!(chunks(&e, buffer), b"ABCD\x06\x00\x03\x04\x01\x02\x07\x08");
        }
    }

    #[test]
    fn add_byte_chunk_writes_the_low_byte() {
        let (mut e, buffer) = chunk_engine(false, 0x100);
        e.call(0x0048_58f0, &args![0x4443_4241u32, 0x1234_5677u32]);
        assert_eq!(chunks(&e, buffer), b"ABCD\x01\x00\x77");
    }

    #[test]
    fn add_word_chunk_swaps_on_big_endian() {
        let (mut e, buffer) = chunk_engine(false, 0x100);
        e.call(0x0048_5910, &args![0x4443_4241u32, 0x0102_0304u32]);
        assert_eq!(chunks(&e, buffer), b"ABCD\x04\x00\x04\x03\x02\x01");
        let (mut e, buffer) = chunk_engine(true, 0x100);
        e.register(SWAP_CHUNK_HEADER, |_, _| Ret::default());
        e.call(0x0048_5910, &args![0x4443_4241u32, 0x0102_0304u32]);
        assert_eq!(chunks(&e, buffer), b"ABCD\x04\x00\x01\x02\x03\x04");
    }

    #[test]
    fn add_half_chunk_swaps_on_big_endian() {
        let (mut e, buffer) = chunk_engine(false, 0x100);
        e.call(0x0048_5950, &args![0x4443_4241u32, 0x0102u32]);
        assert_eq!(chunks(&e, buffer), b"ABCD\x02\x00\x02\x01");
        let (mut e, buffer) = chunk_engine(true, 0x100);
        e.register(SWAP_CHUNK_HEADER, |_, _| Ret::default());
        e.call(0x0048_5950, &args![0x4443_4241u32, 0x0102u32]);
        assert_eq!(chunks(&e, buffer), b"ABCD\x02\x00\x01\x02");
    }

    #[test]
    fn grow_chunk_adds_to_the_size_field_and_the_buffer() {
        let (mut e, buffer) = chunk_engine(false, 0x100);
        let f = form(&mut e, 8, 0x20);
        let data = e.mem.alloc(8);
        e.call(0x0048_5990, &args![0x4443_4241u32, data, 5u32]);
        let chunk = buffer + 0x18;
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_5a70, &args![f, chunk, 3u32]).bool());
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u16(chunk + 4), 8);
        assert_eq!(e.global::<u32>(SAVE_BUFFER_SIZE), 0x18 + 11 + 3);
        assert_eq!(calls_to(&log, BUFFER_REALLOC), vec![vec![buffer, 0x26]]);
        // Past 0xffff, or growing an empty chunk by nothing, fails.
        e.mem.set_u16(chunk + 4, 0xfffe);
        assert!(!e.call(0x0048_5a70, &args![f, chunk, 2u32]).bool());
        assert_eq!(e.mem.u16(chunk + 4), 0xfffe);
        e.mem.set_u16(chunk + 4, 0);
        assert!(!e.call(0x0048_5a70, &args![f, chunk, 0u32]).bool());
        assert_eq!(e.global::<u32>(SAVE_BUFFER_SIZE), 0x26);
        // Exactly 0xffff is allowed.
        e.mem.set_u16(chunk + 4, 0xfff0);
        assert!(e.call(0x0048_5a70, &args![f, chunk, 0xfu32]).bool());
        assert_eq!(e.mem.u16(chunk + 4), 0xffff);
    }

    #[test]
    fn grow_chunk_swaps_the_header_around_the_change_on_big_endian() {
        let (mut e, buffer) = chunk_engine(true, 0x100);
        e.register(SWAP_CHUNK_HEADER, |e, a| {
            let size = e.mem.u16(a[0] + 4).swap_bytes();
            e.mem.set_u16(a[0] + 4, size);
            Ret::default()
        });
        let f = form(&mut e, 8, 0x20);
        let chunk = buffer + 0x18;
        e.mem.set_u16(chunk + 4, 0x0500);
        assert!(e.call(0x0048_5a70, &args![f, chunk, 3u32]).bool());
        assert_eq!(e.mem.u16(chunk + 4), 0x0800);
        // A failure puts the original order back.
        e.mem.set_u16(chunk + 4, 0xfeff);
        assert!(!e.call(0x0048_5a70, &args![f, chunk, 2u32]).bool());
        assert_eq!(e.mem.u16(chunk + 4), 0xfeff);
    }

    #[test]
    fn free_form_buffer_frees_and_clears_the_pointer() {
        let mut e = form_engine();
        let buffer = e.mem.alloc(0x20);
        e.set_global(SAVE_BUFFER, buffer);
        let f = form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_5b30, &args![f]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![buffer]]);
        assert_eq!(e.global::<u32>(SAVE_BUFFER), 0);
    }

    #[test]
    fn id_and_master_word_of_a_form() {
        let mut e = list_engine();
        let f = wide_form(&mut e, 8, 0x20);
        e.set(f, TESForm::iFormID, 0x0512_3456);
        e.register(FILE_MASTER_LIST, |_, a| ret(a[0] + 0x3ec));
        let file = test_file(&mut e, false);
        set_list(&mut e, f, &[file]);
        // An empty master list: just the 24 bit ID.
        assert_eq!(e.call(0x0048_5b60, &args![f]).u32(), 0x0012_3456);
        // A master listed on the file sets bit 24.
        e.mem.set_u32(file + 0x3ec, 0x1234);
        assert_eq!(e.call(0x0048_5b60, &args![f]).u32(), 0x0112_3456);
    }

    #[test]
    fn low_24_bits_of_the_id() {
        let mut e = form_engine();
        let f = form(&mut e, 8, 0x20);
        e.set(f, TESForm::iFormID, 0x0512_3456);
        assert_eq!(e.call(0x0048_5bc0, &args![f]).u32(), 0x0012_3456);
    }

    #[test]
    fn id_comparison_ignores_the_top_byte() {
        let mut e = form_engine();
        let f = form(&mut e, 8, 0x20);
        e.set(f, TESForm::iFormID, 0x0512_3456);
        assert!(e.call(0x0048_5be0, &args![f, 0xff12_3456u32]).bool());
        assert!(!e.call(0x0048_5be0, &args![f, 0x0512_3457u32]).bool());
    }

    fn set_id_engine() -> Engine {
        let mut e = form_engine();
        e.set_global(FORM_MAP, 0x7100_0000u32);
        e.set_global(DATA_HANDLER, 0x7200_0000u32);
        e.register(MAP_REMOVE_AT, |_, _| Ret::default());
        e.register(MAP_SET_AT, |_, _| Ret::default());
        e.register(DATA_HANDLER_REMOVE_ID, |_, _| Ret::default());
        e
    }

    #[test]
    fn set_form_id_moves_the_form_in_the_map() {
        let mut e = set_id_engine();
        let f = form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_5c10, &args![f, 0x0200_0001u32, true]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, MAP_REMOVE_AT),
            vec![vec![0x7100_0000, 0x0100_0abc]]
        );
        assert_eq!(
            calls_to(&log, DATA_HANDLER_REMOVE_ID),
            vec![vec![0x7200_0000, 0x0100_0abc]]
        );
        assert_eq!(
            calls_to(&log, MAP_SET_AT),
            vec![vec![0x7100_0000, 0x0200_0001, f.addr()]]
        );
        assert_eq!(e.get(f, TESForm::iFormID), 0x0200_0001);
    }

    #[test]
    fn set_form_id_edge_cases() {
        let mut e = set_id_engine();
        // Same ID: nothing at all.
        let f = form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_5c10, &args![f, 0x0100_0abcu32, true]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        // Without the handler flag, and a new ID of 0: only the map removal.
        e.call_log = Some(vec![]);
        e.call(0x0048_5c10, &args![f, 0u32, false]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls_to(&log, MAP_REMOVE_AT).len(), 1);
        assert!(calls_to(&log, DATA_HANDLER_REMOVE_ID).is_empty());
        assert!(calls_to(&log, MAP_SET_AT).is_empty());
        assert_eq!(e.get(f, TESForm::iFormID), 0);
        // A form without an ID has nothing to remove.
        e.call_log = Some(vec![]);
        e.call(0x0048_5c10, &args![f, 0x30u32, true]);
        let log = e.call_log.take().unwrap();
        assert!(calls_to(&log, MAP_REMOVE_AT).is_empty());
        assert!(calls_to(&log, DATA_HANDLER_REMOVE_ID).is_empty());
        assert_eq!(calls_to(&log, MAP_SET_AT).len(), 1);
        // A temporary form only stores the ID.
        let temporary = form(&mut e, 0x4000, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_5c10, &args![temporary, 0x31u32, true]);
        assert_eq!(e.call_log.take().unwrap().len(), 2);
        assert_eq!(e.get(temporary, TESForm::iFormID), 0x31);
    }

    // ---- third batch: components, copier, maps ----

    /// The component objects of a test form: each has its value at +4.
    struct Parts {
        uses: u32,
        value: u32,
        health: u32,
        weight: u32,
        quality: u32,
        attack_damage: u32,
        attributes: u32,
        clip_rounds: u32,
    }

    const SLOT_ATTACK_DAMAGE: u32 = 0x0f00_0010;

    fn make_parts(e: &mut Engine) -> Parts {
        let mut slots = vec![0u32; 5];
        slots[4] = SLOT_ATTACK_DAMAGE;
        e.put_vtable(0x0200_1000, &slots);
        let new = |e: &mut Engine| e.mem.alloc(0x20);
        let parts = Parts {
            uses: new(e),
            value: new(e),
            health: new(e),
            weight: new(e),
            quality: new(e),
            attack_damage: new(e),
            attributes: new(e),
            clip_rounds: new(e),
        };
        e.mem.set_u32(parts.attack_damage, 0x0200_1000);
        parts
    }

    /// Makes `RT_DYNAMIC_CAST` find the given components (target type to
    /// object).
    fn cast_to(e: &mut Engine, parts: &Parts, kinds: &[u32]) {
        let table = [
            (TYPE_USES_FORM, parts.uses),
            (TYPE_VALUE_FORM, parts.value),
            (TYPE_HEALTH_FORM, parts.health),
            (TYPE_WEIGHT_FORM, parts.weight),
            (TYPE_QUALITY_FORM, parts.quality),
            (TYPE_ATTACK_DAMAGE_FORM, parts.attack_damage),
            (TYPE_ATTRIBUTES, parts.attributes),
            (TYPE_CLIP_ROUNDS_FORM, parts.clip_rounds),
        ];
        let found: Vec<(u32, u32)> = table
            .iter()
            .copied()
            .filter(|(kind, _)| kinds.contains(kind))
            .collect();
        e.register_double(RT_DYNAMIC_CAST, move |_, a| {
            assert_eq!(a[2], TYPE_TES_FORM);
            ret(found
                .iter()
                .find(|(kind, _)| *kind == a[3])
                .map_or(0, |(_, object)| *object))
        });
    }

    const ALL_KINDS: [u32; 8] = [
        TYPE_USES_FORM,
        TYPE_VALUE_FORM,
        TYPE_HEALTH_FORM,
        TYPE_WEIGHT_FORM,
        TYPE_QUALITY_FORM,
        TYPE_ATTACK_DAMAGE_FORM,
        TYPE_ATTRIBUTES,
        TYPE_CLIP_ROUNDS_FORM,
    ];

    /// A record-buffer engine with the doubles `__SaveData` needs.
    fn save_data_engine(big_endian: bool) -> (Engine, u32, Parts) {
        let (mut e, buffer) = chunk_engine(big_endian, 0x200);
        e.register(SWAP_CHUNK_HEADER, |_, _| Ret::default());
        e.register(SAVE_BUFFER_SIZE_GETTER, |e, _| {
            ret(e.global::<u32>(SAVE_BUFFER_SIZE))
        });
        e.register(SAVE_BUFFER_GETTER, |e, _| ret(e.global::<u32>(SAVE_BUFFER)));
        e.register(LOG_MESSAGE, |_, _| Ret::default());
        e.register(COMPONENT_WORD_GET, |e, a| ret(e.mem.u32(a[0] + 4)));
        e.register(WEIGHT_GET, |e, a| Ret {
            st0: e.mem.f32(a[0] + 4) as f64,
            ..Ret::default()
        });
        e.register(QUALITY_GET, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register(ATTRIBUTES_SIZE, |_, _| ret(3));
        e.register(ATTRIBUTES_SAVE, |e, a| {
            e.mem.write(a[1], &[9, 9, 9]);
            ret(3)
        });
        e.register(SLOT_ATTACK_DAMAGE, |e, a| ret(e.mem.u16(a[0] + 4) as u32));
        let parts = make_parts(&mut e);
        e.mem.set_u8(parts.uses + 4, 7);
        e.mem.set_u32(parts.value + 4, 0x1122_3344);
        e.mem.set_u32(parts.health + 4, 0x5566_7788);
        e.mem.set_f32(parts.weight + 4, 1.5);
        e.mem.set_u8(parts.quality + 4, 200);
        e.mem.set_u16(parts.attack_damage + 4, 0x0abc);
        e.mem.set_u8(parts.clip_rounds + 4, 2);
        (e, buffer, parts)
    }

    #[test]
    fn save_data_appends_every_component_to_the_data_chunk() {
        let (mut e, buffer, parts) = save_data_engine(false);
        cast_to(&mut e, &parts, &ALL_KINDS);
        let f = form(&mut e, 8, 0x20);
        let data = e.mem.alloc(4);
        e.mem.write(data, &[1, 2, 3]);
        e.call(0x0048_5e20, &args![f, data, 3u32]);
        let mut expected = b"DATA".to_vec();
        expected.extend_from_slice(&26u16.to_le_bytes());
        expected.extend_from_slice(&[1, 2, 3, 7]);
        expected.extend_from_slice(&0x1122_3344u32.to_le_bytes());
        expected.extend_from_slice(&0x5566_7788u32.to_le_bytes());
        expected.extend_from_slice(&1.5f32.to_le_bytes());
        expected.extend_from_slice(&200.0f32.to_le_bytes());
        expected.extend_from_slice(&[0xbc, 0x0a, 9, 9, 9, 2]);
        assert_eq!(chunks(&e, buffer), expected);
    }

    #[test]
    fn save_data_swaps_words_floats_and_halves_on_big_endian() {
        let (mut e, buffer, parts) = save_data_engine(true);
        cast_to(
            &mut e,
            &parts,
            &[TYPE_VALUE_FORM, TYPE_WEIGHT_FORM, TYPE_ATTACK_DAMAGE_FORM],
        );
        let f = form(&mut e, 8, 0x20);
        e.call(0x0048_5e00, &args![f]);
        let mut expected = b"DATA".to_vec();
        expected.extend_from_slice(&10u16.to_le_bytes());
        expected.extend_from_slice(&0x1122_3344u32.to_be_bytes());
        expected.extend_from_slice(&1.5f32.to_be_bytes());
        expected.extend_from_slice(&0x0abcu16.to_be_bytes());
        assert_eq!(chunks(&e, buffer), expected);
    }

    #[test]
    fn save_data_without_components_writes_only_the_chunk() {
        let (mut e, buffer, parts) = save_data_engine(false);
        cast_to(&mut e, &parts, &[]);
        let f = form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_5e00, &args![f]);
        let log = e.call_log.take().unwrap();
        assert_eq!(chunks(&e, buffer), b"DATA\x00\x00");
        assert_eq!(calls_to(&log, ATTRIBUTES_SIZE).len(), 0);
        assert_eq!(e.global::<u32>(SAVE_BUFFER_SIZE), 0x18 + 6);
    }

    #[test]
    fn save_data_refuses_more_than_65535_bytes() {
        let (mut e, _, parts) = save_data_engine(false);
        cast_to(&mut e, &parts, &ALL_KINDS);
        let f = form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_5e20, &args![f, 0u32, 0x1_0000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![SAVE_DATA_TOO_LARGE_MESSAGE]]
        );
        assert!(calls_to(&log, SAVE_BUFFER_SIZE_GETTER).is_empty());
        assert_eq!(e.global::<u32>(SAVE_BUFFER_SIZE), 0x18);
    }

    #[test]
    fn component_size_counts_each_kind() {
        let (mut e, _, parts) = save_data_engine(false);
        let f = form(&mut e, 8, 0x20);
        cast_to(&mut e, &parts, &ALL_KINDS);
        assert_eq!(e.call(0x0048_6620, &args![f]).u32(), 1 + 16 + 2 + 3 + 1);
        cast_to(&mut e, &parts, &[TYPE_QUALITY_FORM, TYPE_CLIP_ROUNDS_FORM]);
        assert_eq!(e.call(0x0048_6620, &args![f]).u32(), 5);
        cast_to(&mut e, &parts, &[]);
        assert_eq!(e.call(0x0048_6620, &args![f]).u32(), 0);
    }

    #[test]
    fn smaller_of_two_halves() {
        let mut e = form_engine();
        assert_eq!(e.call(0x0048_65f0, &args![3u16, 9u16]).u16(), 3);
        assert_eq!(e.call(0x0048_65f0, &args![9u16, 3u16]).u16(), 3);
        assert_eq!(e.call(0x0048_65f0, &args![0xffffu16, 1u16]).u16(), 1);
    }

    #[test]
    fn x87_truncation_matches_fistp() {
        assert_eq!(x87_truncate(7.9), 7);
        assert_eq!(x87_truncate(-1.5), -1);
        assert_eq!(x87_truncate(3.0e10), i32::MIN);
        assert_eq!(x87_truncate(f32::NAN), i32::MIN);
    }

    const FILE_CHUNK_AT: u32 = 0x100;

    /// An engine with a file whose current chunk is `chunk`, and doubles for
    /// the components' setters that store what they are given at +4.
    fn load_data_engine(chunk: &[u8], big_endian: bool) -> (Engine, u32, Parts) {
        let (mut e, _) = chunk_engine(big_endian, 0x40);
        let file = e.mem.alloc(0x400);
        e.mem.set_u32(file + 0x25c, chunk.len() as u32);
        e.mem.set_u8(file + 0x299, big_endian as u8);
        e.mem.write(file + FILE_CHUNK_AT, chunk);
        e.register(FILE_CHUNK_SIZE, |e, a| ret(e.mem.u32(a[0] + 0x25c)));
        e.register(
            FILE_IS_BIG_ENDIAN,
            |e, a| ret(e.mem.u8(a[0] + 0x299) as u32),
        );
        e.register(FILE_READ_CHUNK, |e, a| {
            let bytes = e.mem.bytes(a[0] + FILE_CHUNK_AT, a[2]);
            e.mem.write(a[1], &bytes);
            ret(1)
        });
        e.register(COMPONENT_BYTE_SET, |e, a| {
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            Ret::default()
        });
        for setter in [VALUE_SET, HEALTH_SET, WEIGHT_SET] {
            e.register(setter, |e, a| {
                e.mem.set_u32(a[0] + 4, a[1]);
                Ret::default()
            });
        }
        e.register(ATTACK_DAMAGE_SET, |e, a| {
            e.mem.set_u16(a[0] + 4, a[1] as u16);
            Ret::default()
        });
        e.register(ATTRIBUTES_LOAD, |_, _| ret(3));
        let parts = make_parts(&mut e);
        (e, file, parts)
    }

    #[test]
    fn load_data_hands_every_component_its_value() {
        let mut chunk = vec![0xaa, 0xbb, 5];
        chunk.extend_from_slice(&0x0102_0304u32.to_le_bytes());
        chunk.extend_from_slice(&0x0a0b_0c0du32.to_le_bytes());
        chunk.extend_from_slice(&2.5f32.to_le_bytes());
        chunk.extend_from_slice(&7.9f32.to_le_bytes());
        chunk.extend_from_slice(&0x1234u16.to_le_bytes());
        chunk.extend_from_slice(&[0, 0, 0, 9]);
        let (mut e, file, parts) = load_data_engine(&chunk, false);
        cast_to(&mut e, &parts, &ALL_KINDS);
        let f = form(&mut e, 8, 0x20);
        let destination = e.mem.alloc(8);
        e.call_log = Some(vec![]);
        e.call(0x0048_61f0, &args![f, file, destination, 2u16]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.bytes(destination, 2), vec![0xaa, 0xbb]);
        assert_eq!(e.mem.u8(parts.uses + 4), 5);
        assert_eq!(e.mem.u32(parts.value + 4), 0x0102_0304);
        assert_eq!(e.mem.u32(parts.health + 4), 0x0a0b_0c0d);
        assert_eq!(e.mem.f32(parts.weight + 4), 2.5);
        assert_eq!(e.mem.u8(parts.quality + 4), 7);
        assert_eq!(e.mem.u16(parts.attack_damage + 4), 0x1234);
        let loads = calls_to(&log, ATTRIBUTES_LOAD);
        assert_eq!(loads.len(), 1);
        assert_eq!(loads[0][0], parts.attributes);
        // The clip rounds byte is read after the attributes' 3 bytes.
        assert_eq!(e.mem.u8(parts.clip_rounds + 4), 9);
    }

    #[test]
    fn load_data_swaps_when_the_file_is_big_endian() {
        let mut chunk = vec![];
        chunk.extend_from_slice(&0x0102_0304u32.to_be_bytes());
        chunk.extend_from_slice(&2.5f32.to_be_bytes());
        chunk.extend_from_slice(&0x1234u16.to_be_bytes());
        let (mut e, file, parts) = load_data_engine(&chunk, true);
        cast_to(
            &mut e,
            &parts,
            &[TYPE_VALUE_FORM, TYPE_WEIGHT_FORM, TYPE_ATTACK_DAMAGE_FORM],
        );
        let f = form(&mut e, 8, 0x20);
        let destination = e.mem.alloc(8);
        e.call(0x0048_61f0, &args![f, file, destination, 0u16]);
        assert_eq!(e.mem.u32(parts.value + 4), 0x0102_0304);
        assert_eq!(e.mem.f32(parts.weight + 4), 2.5);
        assert_eq!(e.mem.u16(parts.attack_damage + 4), 0x1234);
    }

    #[test]
    fn load_data_stops_at_the_end_of_the_chunk_but_always_loads_attributes() {
        let (mut e, file, parts) = load_data_engine(&[5], false);
        cast_to(&mut e, &parts, &ALL_KINDS);
        let f = form(&mut e, 8, 0x20);
        let destination = e.mem.alloc(8);
        e.mem.set_u32(parts.value + 4, 0xdead);
        e.mem.set_u8(parts.clip_rounds + 4, 0x55);
        e.call_log = Some(vec![]);
        e.call(0x0048_61f0, &args![f, file, destination, 0u16]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u8(parts.uses + 4), 5);
        assert_eq!(e.mem.u32(parts.value + 4), 0xdead);
        assert_eq!(e.mem.u8(parts.clip_rounds + 4), 0x55);
        assert!(calls_to(&log, VALUE_SET).is_empty());
        assert_eq!(calls_to(&log, ATTRIBUTES_LOAD).len(), 1);
    }

    #[test]
    fn add_compile_index_remaps_while_the_save_load_stub_is_true() {
        let mut e = form_engine();
        e.set_global(SAVE_LOAD_GAME, 0x7300_0000u32);
        e.register(SAVE_LOAD_STUB, |_, _| ret(1));
        e.register(SAVE_LOAD_REMAP_ID, |_, a| ret(a[1] + 1));
        let id = e.mem.alloc(4);
        e.mem.set_u32(id, 0x0100_0800);
        e.call(0x0048_5d50, &args![id, 0u32]);
        assert_eq!(e.mem.u32(id), 0x0100_0801);
    }

    #[test]
    fn add_compile_index_replaces_the_top_byte_by_the_files_index() {
        let mut e = form_engine();
        e.set_global(SAVE_LOAD_GAME, 0x7300_0000u32);
        e.register(SAVE_LOAD_STUB, |_, _| ret(0));
        e.register(FILE_GET_INDEX_FILE, |_, a| {
            ret(if a[1] == 2 { 0x7400_0000 } else { 0 })
        });
        e.register(FILE_COMPILE_INDEX, |_, a| {
            ret(if a[0] == 0x7400_0000 { 5 } else { 3 })
        });
        let file = 0x7500_0000u32;
        let id = e.mem.alloc(4);
        // The file with index 2 is found: its compile index is used.
        e.mem.set_u32(id, 0x0112_3456);
        e.call(0x0048_5d50, &args![id, file]);
        assert_eq!(e.mem.u32(id), 0x0512_3456);
        // No such file: the file's own index.
        e.mem.set_u32(id, 0x0712_3456);
        e.call(0x0048_5d50, &args![id, file]);
        assert_eq!(e.mem.u32(id), 0x0312_3456);
        // IDs 1..=0x7ff and a null file leave the ID alone.
        e.mem.set_u32(id, 0x0000_0123);
        e.call(0x0048_5d50, &args![id, file]);
        assert_eq!(e.mem.u32(id), 0x0000_0123);
        e.mem.set_u32(id, 0x0112_3456);
        e.call(0x0048_5d50, &args![id, 0u32]);
        assert_eq!(e.mem.u32(id), 0x0112_3456);
    }

    const COPIER_VTABLE: u32 = 0x0200_2000;
    const SLOT_TAKE_NAME_DOUBLE: u32 = 0x0f00_0134;
    const SLOT_COPY_FROM_DOUBLE: u32 = 0x0f00_0108;

    fn copier_engine(name: u32) -> (Engine, Ptr<TESForm>, Ptr<TESForm>) {
        let mut e = form_engine();
        let mut slots = vec![0u32; 0x138 / 4];
        slots[0x134 / 4] = SLOT_TAKE_NAME_DOUBLE;
        slots[0x108 / 4] = SLOT_COPY_FROM_DOUBLE;
        e.put_vtable(COPIER_VTABLE, &slots);
        e.register(SLOT_TAKE_NAME_DOUBLE, |_, _| Ret::default());
        e.register(SLOT_COPY_FROM_DOUBLE, |_, _| Ret::default());
        e.register_double(FORM_NAME_GETTER, move |_, _| ret(name));
        e.register(STRING_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        e.register(STRING_DATA, |e, a| ret(e.mem.u32(a[0])));
        e.register(STRING_DESTROY, |_, _| Ret::default());
        e.register(MAP_SET_AT, |_, _| Ret::default());
        let original = form(&mut e, 8, 0x2a);
        e.mem.set_u32(original.addr(), COPIER_VTABLE);
        let created = form(&mut e, 8, 0x2a);
        e.mem.set_u32(created.addr(), COPIER_VTABLE);
        let created_addr = created.addr();
        e.register_double(CREATE_FORM_OF_TYPE, move |_, a| {
            ret(if a[0] == 0x2a { created_addr } else { 0 })
        });
        (e, original, created)
    }

    #[test]
    fn copier_creates_a_form_copies_into_it_and_restores_the_name() {
        let (mut e, original, created) = copier_engine(0x7600_0000);
        e.call_log = Some(vec![]);
        let map = 0x7700_0000u32;
        let back = e
            .call(0x0048_67a0, &args![original, 0xeeeeu32, map])
            .ptr::<TESForm>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, created);
        let order: Vec<u32> = log
            .iter()
            .map(|(a, _)| *a)
            .filter(|a| {
                [
                    CREATE_FORM_OF_TYPE,
                    SLOT_TAKE_NAME_DOUBLE,
                    SLOT_COPY_FROM_DOUBLE,
                    MAP_SET_AT,
                ]
                .contains(a)
            })
            .collect();
        assert_eq!(
            order,
            vec![
                CREATE_FORM_OF_TYPE,
                SLOT_TAKE_NAME_DOUBLE,
                SLOT_COPY_FROM_DOUBLE,
                SLOT_TAKE_NAME_DOUBLE,
                MAP_SET_AT
            ]
        );
        assert_eq!(
            calls_to(&log, SLOT_TAKE_NAME_DOUBLE),
            vec![
                vec![original.addr(), DEFAULT_NAME],
                vec![original.addr(), 0x7600_0000]
            ]
        );
        assert_eq!(
            calls_to(&log, SLOT_COPY_FROM_DOUBLE),
            vec![vec![created.addr(), original.addr()]]
        );
        assert_eq!(
            calls_to(&log, MAP_SET_AT),
            vec![vec![map, original.addr(), created.addr()]]
        );
    }

    #[test]
    fn copier_without_a_saved_name_or_map_skips_those_steps() {
        let (mut e, original, created) = copier_engine(0);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x0048_67a0, &args![original, 0u32, 0u32])
            .ptr::<TESForm>();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, created);
        assert_eq!(calls_to(&log, SLOT_TAKE_NAME_DOUBLE).len(), 1);
        assert!(calls_to(&log, MAP_SET_AT).is_empty());
    }

    #[test]
    fn copier_returns_null_when_no_form_can_be_created() {
        let (mut e, _, _) = copier_engine(0x7600_0000);
        let other = form(&mut e, 8, 0x30);
        e.mem.set_u32(other.addr(), COPIER_VTABLE);
        e.call_log = Some(vec![]);
        let back = e
            .call(0x0048_67a0, &args![other, 0u32, 0x7700_0000u32])
            .u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, 0);
        assert!(calls_to(&log, MAP_SET_AT).is_empty());
        assert!(calls_to(&log, SLOT_COPY_FROM_DOUBLE).is_empty());
    }

    #[test]
    fn form_type_lookup_finds_the_table_entry_and_caches_it() {
        let mut e = form_engine();
        let entry = |index: u32| FORM_ENUM_TABLE + index * FORM_ENUM_STRIDE + 8;
        e.mem.set_u32(entry(3), 0x1111);
        e.mem.set_u32(entry(5), 0x2222);
        e.set_global(FORM_TYPE_CACHE_KEY, 0xffff_ffffu32);
        assert_eq!(e.call(0x0048_6890, &args![0x2222u32]).u32(), 5);
        assert_eq!(e.global::<u32>(FORM_TYPE_CACHE_RESULT), 5);
        assert_eq!(e.global::<u32>(FORM_TYPE_CACHE_KEY), 0x2222);
        // The cache answers without looking at the table.
        e.mem.set_u32(entry(5), 0);
        assert_eq!(e.call(0x0048_6890, &args![0x2222u32]).u32(), 5);
        // A string not in the table gives 0 and keeps the cache.
        assert_eq!(e.call(0x0048_6890, &args![0x3333u32]).u32(), 0);
        assert_eq!(e.global::<u32>(FORM_TYPE_CACHE_KEY), 0x2222);
        assert_eq!(e.call(0x0048_6890, &args![0x1111u32]).u32(), 3);
    }

    #[test]
    fn detailed_string_prints_type_name_and_id() {
        let mut e = form_engine();
        e.register(FORM_NAME_GETTER, |_, _| ret(0x7800_0000));
        e.register(FORM_TYPE_NAME, |_, _| ret(0x7900_0000));
        e.register(FORMAT_STRING, |_, _| Ret::default());
        let f = form(&mut e, 8, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x0048_68f0, &args![f, 0x7a00_0000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls_to(&log, FORMAT_STRING),
            vec![vec![
                0x7a00_0000,
                DETAILED_STRING_FORMAT,
                0x7900_0000,
                0x7800_0000,
                0x0100_0abc
            ]]
        );
    }

    fn map_engine() -> Engine {
        let mut e = form_engine();
        e.register(ALLOCATE_BUCKETS, |e, a| ret(e.mem.alloc(a[0])));
        e.register(MEMSET, |e, a| {
            for i in 0..a[2] {
                e.mem.set_u8(a[0] + i, a[1] as u8);
            }
            ret(a[0])
        });
        for addr in [
            MAP_CLEAR,
            MAP_BASE_DESTROY,
            STRING_MAP_BASE_DESTROY,
            CHAR_KEY_MAP_BASE_DESTROY,
            NI_T_MAP_DESTROY,
            STRING_TEMPLATE_MAP_DESTROY,
            OPERATOR_DELETE,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        e
    }

    #[test]
    fn map_base_constructor_allocates_and_clears_the_buckets() {
        let mut e = map_engine();
        let map = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        let back = e.call(0x0048_6990, &args![map, 5u32]).u32();
        let log = e.call_log.take().unwrap();
        assert_eq!(back, map);
        assert_eq!(e.mem.u32(map), MAP_BASE_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 5);
        assert_eq!(e.mem.u32(map + 0xc), 0);
        let buckets = e.mem.u32(map + 8);
        assert_eq!(calls_to(&log, ALLOCATE_BUCKETS), vec![vec![20]]);
        assert_eq!(calls_to(&log, MEMSET), vec![vec![buckets, 0, 20]]);
    }

    #[test]
    fn map_base_constructor_reloads_count_after_allocation() {
        let mut e = Engine::new();
        let map = e.mem.alloc(0x14);
        let buffer = e.mem.alloc(16);
        e.mem.write(buffer, &[0xcc; 16]);
        e.register_double(ALLOCATE_BUCKETS, move |e, args| {
            assert_eq!(args, &[4]); // Native 32-bit SHL wraps.
            e.mem.set_u32(map + 4, 3);
            ret(buffer)
        });
        assert_eq!(e.call(0x00486990, &[map, 0x40000001]).u32(), map);
        assert_eq!(e.mem.u32(map + 8), buffer);
        assert_eq!(e.mem.bytes(buffer, 12), [0; 12]);
        assert_eq!(e.mem.bytes(buffer + 12, 4), [0xcc; 4]);
    }

    #[test]
    fn form_map_constructor_sets_the_pointer_map_vtable() {
        let mut e = map_engine();
        let map = e.mem.alloc(0x14);
        assert_eq!(e.call(0x0048_6930, &args![map, 7u32]).u32(), map);
        assert_eq!(e.mem.u32(map), POINTER_MAP_VTABLE);
        assert_eq!(e.mem.u32(map + 4), 7);
    }

    #[test]
    fn form_map_destructor_clears_then_destroys_the_base() {
        let mut e = map_engine();
        let map = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x0048_6a00, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(map), POINTER_MAP_VTABLE);
        let order: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(order, vec![0x0048_6a00, MAP_CLEAR, MAP_BASE_DESTROY]);
    }

    #[test]
    fn string_map_destructor_sets_its_vtable_and_destroys_the_base() {
        let mut e = map_engine();
        let map = e.mem.alloc(0x14);
        e.call_log = Some(vec![]);
        e.call(0x0048_6c60, &args![map]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(map), STRING_MAP_VTABLE);
        assert_eq!(calls_to(&log, STRING_MAP_BASE_DESTROY), vec![vec![map]]);
    }

    #[test]
    fn scalar_deleting_destructors_release_only_when_asked() {
        let cases: [(u32, &[u32]); 6] = [
            (0x0048_6960, &[MAP_CLEAR, MAP_BASE_DESTROY]),
            (0x0048_6c00, &[MAP_BASE_DESTROY]),
            (0x0048_6c30, &[STRING_MAP_BASE_DESTROY]),
            (0x0048_6e50, &[CHAR_KEY_MAP_BASE_DESTROY]),
            (0x0048_6e80, &[NI_T_MAP_DESTROY]),
            (0x0048_6eb0, &[STRING_TEMPLATE_MAP_DESTROY]),
        ];
        for (addr, callees) in cases {
            let mut e = map_engine();
            let object = e.mem.alloc(0x14);
            for (flags, deleted) in [(0u32, false), (1, true), (2, false), (3, true)] {
                e.call_log = Some(vec![]);
                assert_eq!(e.call(addr, &args![object, flags]).u32(), object);
                let log = e.call_log.take().unwrap();
                for callee in callees {
                    assert_eq!(calls_to(&log, *callee).len(), 1, "{addr:08x}");
                }
                let expected: Vec<Vec<u32>> = if deleted { vec![vec![object]] } else { vec![] };
                assert_eq!(
                    calls_to(&log, OPERATOR_DELETE),
                    expected,
                    "{addr:08x} flags {flags}"
                );
            }
        }
    }
}
