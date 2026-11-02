//! `fallout shared/tesactorbasedata.cpp` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESActorBaseData` is a form component (`BaseFormComponent` plus the
//! actor's base statistics). On PC it sits at `+0x30` of `TESNPC` and
//! `TESCreature`, so `component - 0x30` is the owning form: the functions that
//! mark a change call the form's virtual slots `+0x48` / `+0x4c` on it.
//!
//! Layout (PC offsets; the Xbox PDB `ACTOR_BASE_DATA` starts at `+0x04`):
//! the base flags `+0x04`, then the statistics up to the template-use flags
//! `+0x1a`, the death item `+0x1c`, the voice type `+0x20`, the template form
//! `+0x24`, the change flags `+0x28` and the faction list `+0x2c`.
//!
//! Progress: the queue (`ledger queue "fallout shared/tesactorbasedata.cpp"`)
//! is translated in address order: the last two, `0047e6f0` (`LoadGame`) and
//! `004a8ae0` (`GetFatigue`), are done. The unit is finished except for the
//! functions carrying a `Translation incomplete` marker (`0047c700`,
//! `0047c880`, `0047ce10`, `0047d0b0`: C++ exception unwinding).
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` where the
//! code stores a `float`. Conversions to an integer use the truncating
//! rounding mode the code sets around `FISTP`, so an out-of-range or NaN
//! value gives `0x80000000` there.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::units::fallout_shared::extradataobjects_p2::FACTION_RANK;
use std::cmp::Ordering;

/// `TESForm` flag-word getter shared by the one-bit predicates here: whether
/// the word at `+4` has any of the bits of the mask (`00461580`).
const FLAG_WORD_HAS_BITS: u32 = 0x0046_1580;
/// Same, with the mask `0x80` (`00461560`).
const FLAG_WORD_HAS_BIT_80: u32 = 0x0046_1560;
/// The byte at `+4` of an object: for a form, its form type (`00401170`).
const FORM_TYPE_GETTER: u32 = 0x0040_1170;
/// `operator new` (`00401000`).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete` (`00401030`).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// The game's `memcpy` wrapper (`00401460`).
const MEMCPY: u32 = 0x0040_1460;
/// Swaps the 32-bit word the pointer argument points at (`00401080`); the
/// second stack word is a byte the callee never reads.
const SWAP_U32: u32 = 0x0040_1080;
/// Swaps the 16-bit word the pointer argument points at (`00407a90`).
const SWAP_U16: u32 = 0x0040_7a90;
/// Whether the save file is big-endian: the byte at `011c54ba` (`00401500`).
const IS_BIG_ENDIAN: u32 = 0x0040_1500;
/// A setting's value address: the object's `+4` (a zeroed global for null).
const SETTING_VALUE_POINTER: u32 = 0x0040_3e20;
/// A `Setting`'s word at `+4` (the string pointer of a string setting).
const SETTING_STRING: u32 = 0x0040_3df0;
/// `Setting` value address as a `u16` (`0043d4d0`).
const SETTING_INT_POINTER: u32 = 0x0043_d4d0;
/// The address of the byte at `+4` of an object (`00408d60`).
const BYTE_FIELD_POINTER: u32 = 0x0040_8d60;
/// Whether `a` and `b` differ by at most `tolerance` (`0040eb10`, three
/// `float` stack arguments `a`, `b`, `tolerance`).
const FLOATS_NEAR: u32 = 0x0040_eb10;
/// The log function, one `char *` format string followed by its arguments.
const ERROR_LOG: u32 = 0x0040_fbe0;
/// A second logging function with the same convention (`005b5e40`).
const MESSAGE_LOG: u32 = 0x005b_5e40;
/// `ExtraDataList::GetContainerChanges` (Xbox PDB map name): the list's
/// extra of type `0x15`, returning its word at `+0xc` or null.
const GET_CONTAINER_CHANGES: u32 = 0x0041_8520;
const EXTRA_LIST_CLEAR_A: u32 = 0x0041_aeb0;
const EXTRA_LIST_CLEAR_B: u32 = 0x0041_af50;
/// Whether bit 2 of the word at `+0x294` of the TLS block is set (`004121b0`).
const TLS_FLAG_FOUR: u32 = 0x0041_21b0;
/// The word at `+0x1c` of `this` (`00441110`, map name `PathingLocation::GetWorldspace`).
const WORD_AT_1C: u32 = 0x0044_1110;
/// Whether bit 2 of the TLS flags is set; also sets/clears bit 0 (`004623f0`).
const TLS_FLAG_ONE_SET: u32 = 0x0046_23f0;
/// `TESDataHandler::CreateFormOfType` (Xbox PDB map name).
const CREATE_FORM_OF_TYPE: u32 = 0x0046_5110;
/// Destructor body of the chained container (`0046ffb0`).
const CHAINED_CONTAINER_DESTRUCT: u32 = 0x0046_ffb0;
const BASE_COMPONENT_CONSTRUCT: u32 = 0x0040_2d30;
const LIST_NODE_CONSTRUCT: u32 = 0x0096_a2d0;
/// `TESContainer::TESContainer` / its destructor (map names).
const TES_CONTAINER_CONSTRUCT: u32 = 0x0048_1610;
const TES_CONTAINER_DESTRUCT: u32 = 0x0048_1680;
/// `TESForm::GetFile` (Xbox PDB): `(form, position)`.
const TES_FORM_GET_FILE: u32 = 0x0048_4e60;
/// `TESForm::AddCompileIndex` (Xbox PDB): `(id pointer, file)`.
const TES_FORM_ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// `TESForm::__AddChunkData` (Xbox PDB): `(tag, data, size)`.
const TES_FORM_ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// A chunk holding one 32-bit value: `(tag, value)`.
const ADD_CHUNK_VALUE: u32 = 0x0048_5910;
/// Looks a form up by its form ID.
const LOOKUP_FORM_BY_ID: u32 = 0x0048_39c0;
/// The word at `+0xc` of a form: its form ID.
const FORM_ID_GETTER: u32 = 0x0084_e3a0;
/// `__RTDynamicCast` (replaced by the platform layer).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `_atexit`.
const ATEXIT: u32 = 0x00ec_658f;

/// Linked-list helpers of the faction list (a `BSSimpleList`: item, next).
/// `this + 0x2c`.
const FACTION_LIST_HEAD: u32 = 0x005d_8a70;
/// Returns the node itself: the address of its item word.
const NODE_ITEM_SLOT: u32 = 0x0068_15c0;
/// The node's next-node word at `+4`.
const NODE_NEXT: u32 = 0x0072_6070;
/// Whether both words of a list head node are zero.
const LIST_IS_EMPTY: u32 = 0x0082_56d0;
/// Removes the first node of a chain (the head takes the next node's item).
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
/// Number of nodes whose item is not null.
const LIST_COUNT_ITEMS: u32 = 0x005a_e380;
/// Takes over the head item of another chain (`this`, `&other`).
const LIST_TAKE_HEAD: u32 = 0x005a_e3d0;
/// `BSSimpleList` append: `(this head node, &item)`.
const LIST_APPEND: u32 = 0x0090_5820;

/// The save/load object pointer (`011de45c`) and its methods.
const SAVE_LOAD_GAME_GLOBAL: u32 = 0x011d_e45c;
/// The save buffer's current write position, a word at `+0x14` (`00825c00`).
const BUFFER_POSITION: u32 = 0x0082_5c00;
/// Whether the save-game blocks (`KOBL` headers) are in use: the version byte
/// at `+0x80` of the save/load object is in `0x1f..0x5a` (`00862110`).
const USE_SAVE_GAME_BLOCKS: u32 = 0x0086_2110;
/// Write `(source, size)` into the save buffer (`008579b0`).
const SAVE_BYTES: u32 = 0x0085_79b0;
/// Write a form ID with `(source, size)`, converting to the numeric ID
/// (`00857a10`).
const SAVE_NUMERIC_ID: u32 = 0x0085_7a10;
/// The save/load object's current form record or null (`004fd3e0`).
const CURRENT_SAVE_ENTRY: u32 = 0x004f_d3e0;
/// The debug switch byte (`011de4e8`).
const SAVE_DEBUG_SWITCH: u32 = 0x011d_e4e8;
/// `"D:\\_Fallout3\\Platforms\\Common\\Code\\Fallout Shared\\TESActorBaseData.cpp"`.
const SOURCE_FILE_PATH: u32 = 0x0101_b210;

/// The save/load object's load-side methods (`this` is the global at
/// `011de45c`): read `(destination, size)` bytes from the buffer and advance
/// (`008579e0`); read a numeric form ID, remapping it (`00857aa0`, returns a
/// byte flag the caller ignores); the current load entry, the word at
/// `+0x84` (`004fd3c0`); the version byte at `+0x80` (`008df040`).
const LOAD_BYTES: u32 = 0x0085_79e0;
const LOAD_NUMERIC_ID: u32 = 0x0085_7aa0;
const CURRENT_LOAD_ENTRY: u32 = 0x004f_d3c0;
const SAVE_VERSION: u32 = 0x008d_f040;
/// Messages `LoadGame` logs (`005b5e40`): the block header is wrong, then the
/// buffer overran / underran its size, each without and with a current load
/// entry naming the form being loaded.
const TEXT_LOAD_HEADER_PLAIN: u32 = 0x0101_56a8;
const TEXT_LOAD_HEADER_FORM: u32 = 0x0101_5718;
const TEXT_LOAD_OVERRUN_PLAIN: u32 = 0x0101_54a0;
const TEXT_LOAD_OVERRUN_FORM: u32 = 0x0101_5588;
const TEXT_LOAD_UNDERRUN_PLAIN: u32 = 0x0101_5440;
const TEXT_LOAD_UNDERRUN_FORM: u32 = 0x0101_5500;

/// The object with the `+0x20` word the level calculation reads (`011dea3c`).
const LEVEL_SOURCE_OBJECT: u32 = 0x011d_ea3c;
/// The word at `+0x20` of `this` (`007af430`).
const WORD_AT_20: u32 = 0x007a_f430;

/// `TESContainer` is 0xc bytes (Xbox PDB).
const TES_CONTAINER_SIZE: u32 = 0x0c;
/// Callees of `fn_0047ce10` outside this file, unnamed unless noted:
/// `0056afc0` (true when the actor's saved form passes a test that stops the
/// template pass), `0059bb30` (the word at `+0x24`: the template form),
/// `00488350` (builds the container from the template component for the
/// actor), `00717e50` (the address `this + 4`), `008d8520` (the word at
/// `+0x68`), `00559450` (the word at `+0`), `005d43c0` (the address
/// `this + 0x44`), `00884f80` (clears flags `0x2000` and `0x1000` of the
/// object at `+0x68`), `0056b020`, `00575690` (map name
/// `TESObjectREFR::SetObjectReference`), `00565730` (map name
/// `TESObjectREFR::InitScript`), `004bf220` (map name
/// `InventoryChanges::GetInventoryChanges`), `004d1440`, `004d1610`,
/// `004d1960` and `0042cde0` (whether a list head is empty).
const ACTOR_TEMPLATE_BLOCKED: u32 = 0x0056_afc0;
const TEMPLATE_FORM_GETTER: u32 = 0x0059_bb30;
const BUILD_TEMPLATE_CONTAINER: u32 = 0x0048_8350;
const ADDRESS_PLUS_4: u32 = 0x0071_7e50;
const WORD_AT_68: u32 = 0x008d_8520;
const WORD_AT_0: u32 = 0x0055_9450;
const ADDRESS_PLUS_44: u32 = 0x005d_43c0;
const CLEAR_ACTOR_PROCESS_FLAGS: u32 = 0x0088_4f80;
const APPLY_TEMPLATE_FORM: u32 = 0x0056_b020;
const SET_OBJECT_REFERENCE: u32 = 0x0057_5690;
const INIT_SCRIPT: u32 = 0x0056_5730;
const GET_INVENTORY_CHANGES: u32 = 0x004b_f220;
const INVENTORY_CHANGES_STEP_A: u32 = 0x004d_1440;
const INVENTORY_CHANGES_STEP_B: u32 = 0x004d_1610;
const INVENTORY_CHANGES_STEP_C: u32 = 0x004d_1960;
const LIST_IS_EMPTY_CHECK: u32 = 0x0042_cde0;
/// Setters used by `CopyComponent`: `0050f9a0` stores the word at `+0x1c`,
/// `0050f9c0` the word at `+0x20`, `007037c0` the word at `+0x24` (the map
/// names that shared body `BGSMenuPacker::RecomputePacking`); `008f21d0`
/// reads the 16-bit word at `+0x12`; `0087f9f0` is the actor's level.
const STORE_DEATH_ITEM: u32 = 0x0050_f9a0;
const STORE_VOICE_TYPE: u32 = 0x0050_f9c0;
const STORE_TEMPLATE_FORM: u32 = 0x0070_37c0;
const SPEED_MULT_GETTER: u32 = 0x008f_21d0;
const ACTOR_LEVEL: u32 = 0x0087_f9f0;
/// `BGSSaveLoadGame::GetChange` (map name; called with the word at
/// `011ddf38` as `this`, then the form and the by-value change flags) and the
/// `BGSChangeFlags` constructor.
const GET_CHANGE: u32 = 0x0084_a6d0;
const CHANGE_FLAGS_CONSTRUCT: u32 = 0x008c_71b0;
/// `Actor` level getter results: the actor value index `0x17` is read for
/// the karma.
const KARMA_ACTOR_VALUE: u32 = 0x17;

/// The vtable `TESActorBaseData` writes into its first word.
const TES_ACTOR_BASE_DATA_VTABLE: u32 = 0x0101_b0dc;

/// Settings read as `float`s through the value pointer getter. Names are
/// described by role; the Xbox PDB has `fDefaultFatigue`, `fDefaultKarma`
/// and `iBaseDisposition` among its statics but the addresses are not
/// confirmed against them.
const FATIGUE_DEFAULT_SETTING: u32 = 0x011c_4fa4;
const KARMA_DEFAULT_SETTING: u32 = 0x011c_5074;
const DISPOSITION_DEFAULT_SETTING: u32 = 0x011c_50d8;
/// The four karma alignment thresholds `GetAlignmentForKarma` compares with
/// (the Xbox PDB statics `fAlign...Karma`).
const KARMA_THRESHOLD_011C5144: u32 = 0x011c_5144;
const KARMA_THRESHOLD_011C4F64: u32 = 0x011c_4f64;
const KARMA_THRESHOLD_011C4E54: u32 = 0x011c_4e54;
const KARMA_THRESHOLD_011C5090: u32 = 0x011c_5090;
/// The karma limits `SetKarma` clamps to: upper, then lower.
const KARMA_MAX_SETTING: u32 = 0x011c_5108;
const KARMA_MIN_SETTING: u32 = 0x011c_5160;
/// The five default alignment karma values (floats) and the
/// "table consistent" state (`-1` unknown, `0` fine, `1` inconsistent).
const DEFAULT_ALIGNMENT_VALUES: u32 = 0x0118_6d08;
const ALIGNMENT_TABLE_STATE: u32 = 0x0118_6f78;
/// Three derived differences kept between calls (floats).
const ALIGN_DIFF_011C52A4: u32 = 0x011c_52a4;
const ALIGN_DIFF_011C52A0: u32 = 0x011c_52a0;
const ALIGN_DIFF_011C529C: u32 = 0x011c_529c;
/// A `float` tolerance (about `0.001`; the call passes `0x3a83126f` in the
/// decompiler's view) and the doubles `0.5` and `100.0`.
const KARMA_TOLERANCE: u32 = 0x0101_7d00;
const HALF: u32 = 0x0101_1588;
const HUNDRED: u32 = 0x0101_7a40;
/// `1000.0` (`double`).
const THOUSAND: u32 = 0x0101_7b70;
/// The empty string the karmic title falls back to.
const EMPTY_STRING: u32 = 0x0101_1584;
/// The karmic title `Setting *` table: 5 alignments of 50 levels each
/// (`0xc8` bytes per row).
const KARMIC_TITLE_TABLE: u32 = 0x0118_6d20;

/// `TESForm` class descriptor, `TESActorBaseData` descriptor and friends for
/// `__RTDynamicCast`.
const RTTI_TES_FORM: u32 = 0x0118_3028;
const RTTI_TES_ACTOR_BASE_DATA: u32 = 0x0118_6cb4;
/// The complete type descriptors the cast in `CopyComponent` converts
/// between.
const RTTI_BASE_FORM_COMPONENT_SOURCE: u32 = 0x0118_3040;
const RTTI_FACTION: u32 = 0x0118_4704;
const RTTI_VOICE_TYPE: u32 = 0x0118_6318;
const RTTI_LEVELED_ITEM: u32 = 0x0118_6f7c;

/// Message texts logged by `InitItem` and the save functions.
const TEXT_DEATH_ITEM_MISSING: u32 = 0x0101_b1c8;
const TEXT_TEMPLATE_MISSING: u32 = 0x0101_b178;
const TEXT_INVALID_KARMA: u32 = 0x0101_b148;
const TEXT_SAVE_SIZE_FORM: u32 = 0x0101_2cb0;
const TEXT_SAVE_SIZE_PLAIN: u32 = 0x0101_2c78;
const TEXT_SAVE_FORM: u32 = 0x0101_53a0;
const TEXT_SAVE_PLAIN: u32 = 0x0101_536c;
const TEXT_SAVE_BLOCK_TOO_BIG: u32 = 0x0101_5318;

/// Chunk tags written by `Save`: `ACBS`, `INAM`, `VTCK`, `TPLT`.
const TAG_ACBS: u32 = 0x5342_4341;
const TAG_INAM: u32 = 0x4d41_4e49;
const TAG_VTCK: u32 = 0x4b43_5456;
const TAG_TPLT: u32 = 0x544c_5054;
/// `"KOBL"`: the save-game block header.
const TAG_KOBL: u32 = 0x424c_4f4b;

/// The tool-global pointer the template creation and `ce10` use (a data
/// handler object): the word at `011ddf38`.
const DATA_HANDLER_GLOBAL: u32 = 0x011d_df38;

/// The function-local `static` of `path_manager_q_instance`: the guard bit
/// word and the object.
const PATH_MANAGER_GUARD: u32 = 0x011c_5298;
const PATH_MANAGER_OBJECT: u32 = 0x011c_5290;
/// The `atexit` destructor registered for it.
const PATH_MANAGER_ATEXIT: u32 = 0x00fc_8480;
const PATH_MANAGER_CONSTRUCT: u32 = 0x006e_b820;
/// `PathManager::ClearInventoryPointersFromCurrentRequests` (map name).
const PATH_MANAGER_CLEAR_INVENTORY_POINTERS: u32 = 0x006e_bd10;

layout! {
    /// `TESActorBaseData` (Xbox PDB), 0x34 bytes on both builds. The
    /// `ACTOR_BASE_DATA` struct (flags, statistics) starts at `+0x04`.
    pub struct TESActorBaseData: 0x34 {
        /// `actorData.iActorBaseFlags` (Xbox PDB).
        0x04 iActorBaseFlags: u32,
        /// `actorData.usFatigue` (Xbox PDB).
        0x08 usFatigue: u16,
        /// `actorData.usBartergold` (Xbox PDB).
        0x0A usBartergold: u16,
        /// `actorData.sLevel` (Xbox PDB).
        0x0C sLevel: u16,
        /// `actorData.usCalcLevelMin` (Xbox PDB).
        0x0E usCalcLevelMin: u16,
        /// `actorData.usCalcLevelMax` (Xbox PDB).
        0x10 usCalcLevelMax: u16,
        /// `actorData.usSpeedMult` (Xbox PDB).
        0x12 usSpeedMult: u16,
        /// `actorData.fKarma` (Xbox PDB).
        0x14 fKarma: f32,
        /// `actorData.usBaseDisposition` (Xbox PDB).
        0x18 usBaseDisposition: u16,
        /// `actorData.sTemplateUseFlags` (Xbox PDB).
        0x1A sTemplateUseFlags: u16,
        /// `pDeathItem` (Xbox PDB): `TESLevItem *`.
        0x1C pDeathItem: Ptr,
        /// `pVoiceType` (Xbox PDB): `BGSVoiceType *`.
        0x20 pVoiceType: Ptr,
        /// `pTemplateForm` (Xbox PDB): `TESForm *`.
        0x24 pTemplateForm: Ptr,
        /// `iChangeFlags` (Xbox PDB).
        0x28 iChangeFlags: u32,
        /// `listFactions` (Xbox PDB): the head node of a
        /// `BSSimpleList<FACTION_RANK *>`.
        0x2C listFactions: Inline<crate::types::BSSimpleList>,
    }
}

/// `FISTP` with the truncating rounding mode: the integer part, or
/// `0x80000000` ("integer indefinite") for NaN and out-of-range values.
fn float_to_int_truncated(value: f64) -> i32 {
    let whole = value.trunc();
    if whole.is_nan() || !(-2147483648.0..=2147483647.0).contains(&whole) {
        i32::MIN
    } else {
        whole as i32
    }
}

/// The value of a `float` setting, read through its value pointer.
fn setting_float(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE_POINTER, &args![setting]).u32();
    e.mem.f32(value)
}

/// `this + 0x2c`, the head node of the faction list.
fn faction_list_head(e: &mut Engine, this: Ptr<TESActorBaseData>) -> u32 {
    e.call(FACTION_LIST_HEAD, &args![this]).u32()
}

/// The item (a `FACTION_RANK *`) stored in a list node.
fn node_item(e: &mut Engine, node: u32) -> u32 {
    let slot = e.call(NODE_ITEM_SLOT, &args![node]).u32();
    e.mem.u32(slot)
}

/// The node after `node`.
fn node_next(e: &mut Engine, node: u32) -> u32 {
    e.call(NODE_NEXT, &args![node]).u32()
}

// Translated from 0047c700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::TESActorBaseData` (Xbox PDB): constructs the component
/// base, writes this class's vtable, constructs the faction list's head node
/// and sets the defaults (`InitializeDataComponent`). Returns `this`.
// Translation incomplete: 0047c700 - C++ exception unwinding (the frame registered at FS:[0]) is not translated.
pub fn tes_actor_base_data_tes_actor_base_data(
    e: &mut Engine,
    this: Ptr<TESActorBaseData>,
) -> Ptr<TESActorBaseData> {
    e.call(BASE_COMPONENT_CONSTRUCT, &args![this]);
    e.mem.set_u32(this.addr(), TES_ACTOR_BASE_DATA_VTABLE);
    e.call(LIST_NODE_CONSTRUCT, &args![this.addr() + 0x2c]);
    tes_actor_base_data_initialize_data_component(e, this);
    this
}

/// A one-bit predicate on the base flags: `00461580` with `mask`.
fn base_flag(e: &mut Engine, this: Ptr<TESActorBaseData>, mask: u32) -> bool {
    e.call(FLAG_WORD_HAS_BITS, &args![this, mask]).bool()
}

// Translated from 0047c770 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::GetNoVATSMelee` (Xbox PDB): base flag `0x100000`.
pub fn tes_actor_base_data_get_no_vats_melee(e: &mut Engine, this: Ptr<TESActorBaseData>) -> bool {
    base_flag(e, this, 0x0010_0000)
}

// Translated from 0047c790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::GetAllowPCDialogue` (Xbox PDB): base flag `0x200000`.
pub fn tes_actor_base_data_get_allow_pc_dialogue(
    e: &mut Engine,
    this: Ptr<TESActorBaseData>,
) -> bool {
    base_flag(e, this, 0x0020_0000)
}

// Translated from 0047c7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::GetAllowPickPocket` (Xbox PDB): base flag
/// `0x10000000`.
pub fn tes_actor_base_data_get_allow_pick_pocket(
    e: &mut Engine,
    this: Ptr<TESActorBaseData>,
) -> bool {
    base_flag(e, this, 0x1000_0000)
}

// Translated from 0047c7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::GetIsGhost` (Xbox PDB): base flag `0x20000000`.
pub fn tes_actor_base_data_get_is_ghost(e: &mut Engine, this: Ptr<TESActorBaseData>) -> bool {
    base_flag(e, this, 0x2000_0000)
}

// Translated from 0047c7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::GetInvulnerable` (Xbox PDB): base flag `0x80000000`.
pub fn tes_actor_base_data_get_invulnerable(e: &mut Engine, this: Ptr<TESActorBaseData>) -> bool {
    base_flag(e, this, 0x8000_0000)
}

// Translated from 0047c810 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::GetCantOpenDoors` (Xbox PDB): base flag `0x400000`.
pub fn tes_actor_base_data_get_cant_open_doors(
    e: &mut Engine,
    this: Ptr<TESActorBaseData>,
) -> bool {
    base_flag(e, this, 0x0040_0000)
}

// Translated from 0047c830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::GetAutoCalcServiceFlags` (Xbox PDB): base flag
/// `0x800000`.
pub fn tes_actor_base_data_get_auto_calc_service_flags(
    e: &mut Engine,
    this: Ptr<TESActorBaseData>,
) -> bool {
    base_flag(e, this, 0x0080_0000)
}

// Translated from 0047c850 (PC disassembly, FalloutNV.exe 1.4.0.525).
/// Returns false in AL without reading the receiver. This folded body is
/// shared by several vtables and predicates; no Xbox PDB name is asserted.
pub fn fn_0047c850(_e: &mut Engine, _this: Ptr) -> bool {
    false
}

// Translated from 0047c860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::GetKarma` (Xbox PDB): `fKarma`, returned in `ST0`.
pub fn tes_actor_base_data_get_karma(e: &mut Engine, this: Ptr<TESActorBaseData>) -> f32 {
    e.get(this, TESActorBaseData::fKarma)
}

// Translated from 0047c880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor body of `TESActorBaseData` (unnamed in the map): restores this
/// class's vtable, clears the component (`ClearDataComponent`, called
/// directly, not through the vtable) and destroys the faction list's chain.
// Translation incomplete: 0047c880 - C++ exception unwinding (the frame registered at FS:[0]) is not translated.
pub fn fn_0047c880(e: &mut Engine, this: Ptr<TESActorBaseData>) {
    e.mem.set_u32(this.addr(), TES_ACTOR_BASE_DATA_VTABLE);
    tes_actor_base_data_clear_data_component(e, this);
    e.call(CHAINED_CONTAINER_DESTRUCT, &args![this.addr() + 0x2c]);
}

// Translated from 0047c8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::InitializeDataComponent` (Xbox PDB): the defaults. The
/// flags are cleared; the fatigue is a `float` setting truncated to an
/// integer; the bartergold, calculated level range and template-use flags
/// are zero; the level and speed multiplier are 1 and 100; the karma and base
/// disposition come from settings; the death item, voice type, template form
/// and change flags are cleared.
pub fn tes_actor_base_data_initialize_data_component(e: &mut Engine, this: Ptr<TESActorBaseData>) {
    e.set(this, TESActorBaseData::iActorBaseFlags, 0);
    let fatigue = setting_float(e, FATIGUE_DEFAULT_SETTING);
    e.set(
        this,
        TESActorBaseData::usFatigue,
        float_to_int_truncated(fatigue as f64) as u16,
    );
    e.set(this, TESActorBaseData::usBartergold, 0);
    e.set(this, TESActorBaseData::sLevel, 1);
    e.set(this, TESActorBaseData::usCalcLevelMin, 0);
    e.set(this, TESActorBaseData::usCalcLevelMax, 0);
    e.set(this, TESActorBaseData::usSpeedMult, 100);
    let karma = setting_float(e, KARMA_DEFAULT_SETTING);
    e.set(this, TESActorBaseData::fKarma, karma);
    let disposition_slot = e.call(SETTING_INT_POINTER, &args![DISPOSITION_DEFAULT_SETTING]);
    let disposition = e.mem.u16(disposition_slot.u32());
    e.set(this, TESActorBaseData::usBaseDisposition, disposition);
    e.set(this, TESActorBaseData::pVoiceType, Ptr::NULL);
    e.set(this, TESActorBaseData::pDeathItem, Ptr::NULL);
    e.set(this, TESActorBaseData::pTemplateForm, Ptr::NULL);
    e.set(this, TESActorBaseData::sTemplateUseFlags, 0);
    e.set(this, TESActorBaseData::iChangeFlags, 0);
}

// Translated from 0047c9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::ClearDataComponent` (Xbox PDB): clears the faction list.
pub fn tes_actor_base_data_clear_data_component(e: &mut Engine, this: Ptr<TESActorBaseData>) {
    tes_actor_base_data_clear_faction_list(e, this);
}

// Translated from 0047c9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::ClearFactionList` (Xbox PDB): while the list is not
/// empty, deletes the first `FACTION_RANK` and removes the first node.
pub fn tes_actor_base_data_clear_faction_list(e: &mut Engine, this: Ptr<TESActorBaseData>) {
    let head = faction_list_head(e, this);
    while head != 0 && !e.call(LIST_IS_EMPTY, &args![head]).bool() {
        let item = node_item(e, head);
        e.call(OPERATOR_DELETE, &args![item]);
        e.call(LIST_REMOVE_HEAD, &args![head]);
    }
}

// Translated from 0047ca20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::CopyFactionList` (Xbox PDB): clears this list, then
/// appends an 8-byte copy of every non-null `FACTION_RANK` of `source`'s list
/// to it.
pub fn tes_actor_base_data_copy_faction_list(
    e: &mut Engine,
    this: Ptr<TESActorBaseData>,
    source: Ptr<TESActorBaseData>,
) {
    tes_actor_base_data_clear_faction_list(e, this);
    let mut tail = faction_list_head(e, this);
    let mut node = faction_list_head(e, source);
    while node != 0 {
        let rank = node_item(e, node);
        if rank != 0 {
            let copy = e.call(OPERATOR_NEW, &args![8u32]).u32();
            e.call(MEMCPY, &args![copy, rank, 8u32]);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), copy);
                e.call(LIST_APPEND, &args![tail, slot]);
            });
        }
        let next = node_next(e, tail);
        if next != 0 {
            tail = next;
        }
        node = node_next(e, node);
    }
}

// Translated from 0047cac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::Save` (Xbox PDB): writes the `ACBS` chunk (the `0x18`
/// bytes of `ACTOR_BASE_DATA`, byte-swapped around the write on a big-endian
/// target), then the death item (`INAM`), voice type (`VTCK`) and template
/// form (`TPLT`) as form IDs when set.
pub fn tes_actor_base_data_save(e: &mut Engine, this: Ptr<TESActorBaseData>) {
    let data = this.addr() + 4;
    if e.call(IS_BIG_ENDIAN, &args![]).bool() {
        fn_0047cb90(e, Ptr::new(data));
    }
    e.call(TES_FORM_ADD_CHUNK_DATA, &args![TAG_ACBS, data, 0x18u32]);
    if e.call(IS_BIG_ENDIAN, &args![]).bool() {
        fn_0047cb90(e, Ptr::new(data));
    }
    for (tag, field) in [
        (TAG_INAM, TESActorBaseData::pDeathItem),
        (TAG_VTCK, TESActorBaseData::pVoiceType),
        (TAG_TPLT, TESActorBaseData::pTemplateForm),
    ] {
        let form = e.get(this, field);
        if !form.is_null() {
            let id = e.call(FORM_ID_GETTER, &args![form]).u32();
            e.call(ADD_CHUNK_VALUE, &args![tag, id]);
        }
    }
}

// Translated from 0047cb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Byte-swaps the `0x18` bytes of `ACTOR_BASE_DATA` at `data` in place (the
/// flags and the karma as 32-bit words, the six statistics and the last two
/// fields as 16-bit words). Unnamed in the map.
pub fn fn_0047cb90(e: &mut Engine, data: Ptr) {
    let base = data.addr();
    for (offset, wide) in [
        (0x00u32, true),
        (0x04, false),
        (0x06, false),
        (0x08, false),
        (0x0a, false),
        (0x0c, false),
        (0x0e, false),
        (0x10, true),
        (0x14, false),
        (0x16, false),
    ] {
        let function = if wide { SWAP_U32 } else { SWAP_U16 };
        e.call(function, &args![base + offset, 0u32]);
    }
}

// Translated from 0047cca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 15 of the template-use flags is set (`0047cd40` with 15).
/// Unnamed in the map.
pub fn fn_0047cca0(e: &mut Engine, this: Ptr<TESActorBaseData>) -> bool {
    fn_0047cd40(e, this, 0xf)
}

// Translated from 0047ccc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets or clears bit 15 of the template-use flags (`0047cce0` with 15).
/// Unnamed in the map.
pub fn fn_0047ccc0(e: &mut Engine, this: Ptr<TESActorBaseData>, value: u8) {
    fn_0047cce0(e, this, 0xf, value);
}

// Translated from 0047cce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets (`value` not zero) or clears bit `index` (0 to 15) of the
/// template-use flags; other indices do nothing. Unnamed in the map.
pub fn fn_0047cce0(e: &mut Engine, this: Ptr<TESActorBaseData>, index: i32, value: u8) {
    if (0..0x10).contains(&index) {
        let mask = (1u32 << index) as u16;
        let flags = e.get(this, TESActorBaseData::sTemplateUseFlags);
        let flags = if value != 0 {
            flags | mask
        } else {
            flags & !mask
        };
        e.set(this, TESActorBaseData::sTemplateUseFlags, flags);
    }
}

// Translated from 0047cd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit `index` (0 to 15) of the template-use flags is set; true for
/// an index outside that range. Unnamed in the map.
pub fn fn_0047cd40(e: &mut Engine, this: Ptr<TESActorBaseData>, index: i32) -> bool {
    if (0..0x10).contains(&index) {
        let mask = (1u32 << index) as u16;
        e.get(this, TESActorBaseData::sTemplateUseFlags) & mask != 0
    } else {
        true
    }
}

// Translated from 0047cd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the template-use flags. Unnamed in the map.
pub fn fn_0047cd90(e: &mut Engine, this: Ptr<TESActorBaseData>, flags: u16) {
    e.set(this, TESActorBaseData::sTemplateUseFlags, flags);
}

// Translated from 0047cdb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `fn_0047cdd0` (the compiler emitted a second copy that forwards to it).
pub fn fn_0047cdb0(e: &mut Engine, this: Ptr<TESActorBaseData>) -> bool {
    fn_0047cdd0(e, this)
}

// Translated from 0047cdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the template form is set and its form type (the byte at `+4`) is
/// `0x2c` or `0x2d`. Unnamed in the map.
pub fn fn_0047cdd0(e: &mut Engine, this: Ptr<TESActorBaseData>) -> bool {
    let template = e.get(this, TESActorBaseData::pTemplateForm);
    if template.is_null() {
        return false;
    }
    let form_type = e.call(FORM_TYPE_GETTER, &args![template]).i32();
    (0x2c..=0x2d).contains(&form_type)
}

// Translated from 0047ce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies the template to a loaded actor (`actor`): when this component
/// has a template whose form type is `0x2c` or `0x2d` and the actor passes
/// the `0056afc0` test, the template form's own `TESActorBaseData` (at
/// `+0x30`) builds a container, the actor's inventory pointers are cleared
/// out of the path manager's requests, a new template base form is created
/// for the actor's saved form (`create_template_actor_base`), and the actor's
/// inventory is rebuilt through its container changes. Unnamed in the map.
// Translation incomplete: 0047ce10 - C++ exception unwinding (the frame registered at FS:[0] and the container local's unwind state) is not translated.
pub fn fn_0047ce10(e: &mut Engine, this: Ptr<TESActorBaseData>, actor: Ptr) {
    if actor.is_null()
        || !fn_0047cdb0(e, this)
        || e.call(ACTOR_TEMPLATE_BLOCKED, &args![actor]).bool()
    {
        return;
    }
    let mut component = 0u32;
    let template = e.call(TEMPLATE_FORM_GETTER, &args![this]).u32();
    if e.call(FORM_TYPE_GETTER, &args![template]).i32() == 0x2d {
        let template = e.call(TEMPLATE_FORM_GETTER, &args![this]).u32();
        component = if template != 0 { template + 0x30 } else { 0 };
    } else {
        let template = e.call(TEMPLATE_FORM_GETTER, &args![this]).u32();
        if e.call(FORM_TYPE_GETTER, &args![template]).i32() == 0x2c {
            let template = e.call(TEMPLATE_FORM_GETTER, &args![this]).u32();
            component = if template != 0 { template + 0x30 } else { 0 };
        }
    }
    if component == 0 {
        return;
    }
    e.with_stack(TES_CONTAINER_SIZE, |e, container| {
        e.call(TES_CONTAINER_CONSTRUCT, &args![container]);
        e.call(
            BUILD_TEMPLATE_CONTAINER,
            &args![component, actor, container],
        );
        // `TESContainer::objectList` (+4) is the container's list: its first
        // entry's second word is passed on as the template argument.
        let list = e.call(ADDRESS_PLUS_4, &args![container]).u32();
        let first = node_item(e, list);
        let first_entry_word = e.mem.u32(first + 4);
        let handler = e.mem.u32(DATA_HANDLER_GLOBAL);
        if handler == 0 || !e.call(TLS_FLAG_FOUR, &args![handler]).bool() {
            if e.vcall(actor.addr(), 0xfc, &args![]).bool()
                && e.call(WORD_AT_68, &args![actor]).u32() != 0
            {
                e.vcall(actor.addr(), 0x244, &args![]);
            }
            e.vcall(actor.addr(), 0x1cc, &args![0u32, 1u32]);
        }
        let manager = path_manager_q_instance(e);
        if e.call(WORD_AT_0, &args![manager]).u32() != 0 {
            let extra_list = e.call(ADDRESS_PLUS_44, &args![actor]).u32();
            let changes = e.call(GET_CONTAINER_CHANGES, &args![extra_list]).u32();
            let manager = path_manager_q_instance(e);
            e.call(
                PATH_MANAGER_CLEAR_INVENTORY_POINTERS,
                &args![manager, changes],
            );
        }
        let extra_list = e.call(ADDRESS_PLUS_44, &args![actor]).u32();
        e.call(EXTRA_LIST_CLEAR_A, &args![extra_list]);
        let extra_list = e.call(ADDRESS_PLUS_44, &args![actor]).u32();
        e.call(EXTRA_LIST_CLEAR_B, &args![extra_list]);
        if e.vcall(actor.addr(), 0x100, &args![]).bool() {
            e.call(CLEAR_ACTOR_PROCESS_FLAGS, &args![actor]);
        }
        let saved_form = e.call(WORD_AT_20, &args![actor]).u32();
        let template_base = create_template_actor_base(e, saved_form, first_entry_word);
        e.call(
            APPLY_TEMPLATE_FORM,
            &args![actor, saved_form, first_entry_word],
        );
        e.call(SET_OBJECT_REFERENCE, &args![actor, template_base]);
        e.call(INIT_SCRIPT, &args![actor]);
        let changes = e.call(GET_INVENTORY_CHANGES, &args![actor]).u32();
        e.call(INVENTORY_CHANGES_STEP_A, &args![changes]);
        e.call(INVENTORY_CHANGES_STEP_B, &args![changes]);
        e.call(INVENTORY_CHANGES_STEP_C, &args![changes]);
        if e.call(LIST_IS_EMPTY_CHECK, &args![changes]).bool() {
            let extra_list = e.call(ADDRESS_PLUS_44, &args![actor]).u32();
            e.call(EXTRA_LIST_CLEAR_A, &args![extra_list]);
        }
        e.call(TES_CONTAINER_DESTRUCT, &args![container]);
    });
}

// Translated from 0047d0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `PathManager::QInstance` (Xbox PDB map name): the function-local `static
/// PathManager` at `011c5290`, constructed on the first call (guard bit 0 of
/// `011c5298`) with its destructor registered through `atexit`.
// Translation incomplete: 0047d0b0 - C++ exception unwinding (the frame registered at FS:[0]) is not translated.
pub fn path_manager_q_instance(e: &mut Engine) -> u32 {
    let guard: u32 = e.global(PATH_MANAGER_GUARD);
    if guard & 1 == 0 {
        e.set_global(PATH_MANAGER_GUARD, guard | 1);
        e.call(PATH_MANAGER_CONSTRUCT, &args![PATH_MANAGER_OBJECT]);
        e.call(ATEXIT, &args![PATH_MANAGER_ATEXIT]);
    }
    PATH_MANAGER_OBJECT
}

// Translated from 0047d130 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::CreateTemplateActorBase` (Xbox PDB): with the data
/// handler's flag cleared, creates a form of the same type as `source`,
/// copies `source` into it (form virtual `+0x108`), has its component copy
/// from the template argument (the component's virtual `+0x10`,
/// `CopyFromTemplateForm`), restores the flag and returns the new form.
pub fn create_template_actor_base(e: &mut Engine, source: u32, template: u32) -> u32 {
    let handler = e.mem.u32(DATA_HANDLER_GLOBAL);
    let saved = e.call(TLS_FLAG_ONE_SET, &args![handler, 0u32]).u8();
    let form_type = e.call(FORM_TYPE_GETTER, &args![source]).u32();
    let form = e.call(CREATE_FORM_OF_TYPE, &args![form_type]).u32();
    e.vcall(form, 0x108, &args![source]);
    e.vcall(form + 0x30, 0x10, &args![template]);
    let handler = e.mem.u32(DATA_HANDLER_GLOBAL);
    e.call(TLS_FLAG_ONE_SET, &args![handler, saved]);
    form
}

// Translated from 0047d1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the chain of template forms from `form`: while the form's virtual
/// `+0xf8` says true, ANDs the 16-bit word at `+0x4a` of the form into
/// `*mask` (starting from `0xffff`) and moves to the form's template (the
/// template pointer of its component at `+0x30`). Returns the first form
/// where the virtual says false, or null at the end of the chain. Unnamed in
/// the map.
pub fn fn_0047d1a0(e: &mut Engine, form: Ptr, mask: Ptr) -> Ptr {
    let mut found = Ptr::NULL;
    e.mem.set_u16(mask.addr(), 0xffff);
    let mut current = form.addr();
    while current != 0 {
        if e.vcall(current, 0xf8, &args![]).bool() {
            let bits = e.mem.u16(current + 0x4a);
            let combined = e.mem.u16(mask.addr()) & bits;
            e.mem.set_u16(mask.addr(), combined);
            current = e.call(TEMPLATE_FORM_GETTER, &args![current + 0x30]).u32();
        } else {
            found = Ptr::new(current);
            current = 0;
        }
    }
    found
}

// Translated from 0047d220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::CopyComponent` (Xbox PDB): when `source` casts to a
/// `TESActorBaseData`, copies its base flags, fatigue (virtual `+0x60`),
/// karma (virtual `+0x64`), bartergold, level, calculated level range, speed
/// multiplier and base disposition through this class's setters (each marks
/// the form changed), then the faction list, death item, voice type
/// (virtual `+0x68`), template form and template-use flags.
pub fn tes_actor_base_data_copy_component(
    e: &mut Engine,
    this: Ptr<TESActorBaseData>,
    source: Ptr,
) {
    let other = e
        .call(
            DYNAMIC_CAST,
            &args![
                source,
                0u32,
                RTTI_BASE_FORM_COMPONENT_SOURCE,
                RTTI_TES_ACTOR_BASE_DATA,
                0u32
            ],
        )
        .u32();
    if other == 0 {
        return;
    }
    let source: Ptr<TESActorBaseData> = Ptr::new(other);
    let flags_slot = e.call(ADDRESS_PLUS_4, &args![source]).u32();
    let flags = e.mem.u32(flags_slot);
    fn_0047dd30(e, this, flags);
    let fatigue = e.vcall(other, 0x60, &args![]).u16();
    fn_0047e010(e, this, fatigue);
    let karma = e.vcall(other, 0x64, &args![]).f32();
    fn_0047e1c0(e, this, karma);
    let bartergold = fn_0047d3f0(e, source);
    fn_0047e240(e, this, bartergold);
    let level = fn_0047d370(e, source);
    fn_0047dfe0(e, this, level);
    let calc_min = fn_0047d390(e, source);
    fn_0047de40(e, this, calc_min);
    let calc_max = fn_0047d3b0(e, source);
    fn_0047de70(e, this, calc_max);
    let speed = e.call(SPEED_MULT_GETTER, &args![source]).u16();
    fn_0047e270(e, this, speed);
    let disposition = fn_0047d3d0(e, source);
    fn_0047dea0(e, this, disposition);
    tes_actor_base_data_copy_faction_list(e, this, source);
    let death_item = e.call(WORD_AT_1C, &args![source]).u32();
    e.call(STORE_DEATH_ITEM, &args![this, death_item]);
    let voice = e.vcall(other, 0x68, &args![]).u32();
    e.call(STORE_VOICE_TYPE, &args![this, voice]);
    let template = e.call(TEMPLATE_FORM_GETTER, &args![source]).u32();
    e.call(STORE_TEMPLATE_FORM, &args![this, template]);
    let use_flags = e.get(source, TESActorBaseData::sTemplateUseFlags);
    e.set(this, TESActorBaseData::sTemplateUseFlags, use_flags);
}

// Translated from 0047d370 (decompiled, FalloutNV.exe 1.4.0.525)
/// `actorData.sLevel`, the word at `+0xc`. Unnamed in the map.
pub fn fn_0047d370(e: &mut Engine, this: Ptr<TESActorBaseData>) -> u16 {
    e.get(this, TESActorBaseData::sLevel)
}

// Translated from 0047d390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `actorData.usCalcLevelMin`, the word at `+0xe`. Unnamed in the map.
pub fn fn_0047d390(e: &mut Engine, this: Ptr<TESActorBaseData>) -> u16 {
    e.get(this, TESActorBaseData::usCalcLevelMin)
}

// Translated from 0047d3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `actorData.usCalcLevelMax`, the word at `+0x10`. Unnamed in the map.
pub fn fn_0047d3b0(e: &mut Engine, this: Ptr<TESActorBaseData>) -> u16 {
    e.get(this, TESActorBaseData::usCalcLevelMax)
}

// Translated from 0047d3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `actorData.usBaseDisposition`, the word at `+0x18`. Unnamed in the map.
pub fn fn_0047d3d0(e: &mut Engine, this: Ptr<TESActorBaseData>) -> u16 {
    e.get(this, TESActorBaseData::usBaseDisposition)
}

// Translated from 0047d3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `actorData.usBartergold`, the word at `+0xa`. Unnamed in the map.
pub fn fn_0047d3f0(e: &mut Engine, this: Ptr<TESActorBaseData>) -> u16 {
    e.get(this, TESActorBaseData::usBartergold)
}

// Translated from 0047d410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::CompareComponent` (Xbox PDB): returns true (different)
/// unless `other` casts to a `TESActorBaseData` with equal base flags,
/// fatigue (virtual `+0x60`), karma (virtual `+0x64`, `float` compare, NaN
/// is different), bartergold, level, calculated level range, death item,
/// speed multiplier, base disposition, equal `GetFactionRank` for every
/// faction of this list, equal voice type (virtual `+0x68`), template form
/// and template-use flags. The factions of `other` are only counted.
pub fn tes_actor_base_data_compare_component(
    e: &mut Engine,
    this: Ptr<TESActorBaseData>,
    other: Ptr,
) -> bool {
    let cast = e
        .call(
            DYNAMIC_CAST,
            &args![
                other,
                0u32,
                RTTI_BASE_FORM_COMPONENT_SOURCE,
                RTTI_TES_ACTOR_BASE_DATA,
                0u32
            ],
        )
        .u32();
    if cast == 0 {
        return true;
    }
    let other: Ptr<TESActorBaseData> = Ptr::new(cast);
    let mine = e.call(ADDRESS_PLUS_4, &args![this]).u32();
    let theirs = e.call(ADDRESS_PLUS_4, &args![other]).u32();
    if e.mem.u32(mine) != e.mem.u32(theirs) {
        return true;
    }
    if e.vcall(this.addr(), 0x60, &args![]).u16() != e.vcall(cast, 0x60, &args![]).u16() {
        return true;
    }
    let my_karma = e.vcall(this.addr(), 0x64, &args![]).f32() as f64;
    let their_karma = e.vcall(cast, 0x64, &args![]).f32() as f64;
    if my_karma != their_karma {
        return true;
    }
    if fn_0047d3f0(e, this) != fn_0047d3f0(e, other)
        || fn_0047d370(e, this) != fn_0047d370(e, other)
        || fn_0047d390(e, this) != fn_0047d390(e, other)
        || fn_0047d3b0(e, this) != fn_0047d3b0(e, other)
    {
        return true;
    }
    let my_death_item = e.call(WORD_AT_1C, &args![this]).u32();
    let their_death_item = e.call(WORD_AT_1C, &args![other]).u32();
    if my_death_item != their_death_item {
        return true;
    }
    if e.call(SPEED_MULT_GETTER, &args![this]).u16()
        != e.call(SPEED_MULT_GETTER, &args![other]).u16()
    {
        return true;
    }
    if fn_0047d3d0(e, this) != fn_0047d3d0(e, other) {
        return true;
    }
    let my_list = faction_list_head(e, this);
    let my_count = e.call(LIST_COUNT_ITEMS, &args![my_list]).u32();
    let their_list = faction_list_head(e, other);
    let their_count = e.call(LIST_COUNT_ITEMS, &args![their_list]).u32();
    if my_count != their_count {
        return true;
    }
    let mut node = faction_list_head(e, this);
    while node != 0 {
        let entry = node_item(e, node);
        if entry != 0 {
            let faction = e.get(Ptr::<FACTION_RANK>::new(entry), FACTION_RANK::pFaction);
            let rank = tes_actor_base_data_get_faction_rank(e, other, faction.addr(), 0);
            let own_rank = e.get(Ptr::<FACTION_RANK>::new(entry), FACTION_RANK::cRank) as i8;
            if rank != own_rank as i32 {
                return true;
            }
        }
        node = node_next(e, node);
    }
    if e.vcall(this.addr(), 0x68, &args![]).u32() != e.vcall(cast, 0x68, &args![]).u32() {
        return true;
    }
    let my_template = e.call(TEMPLATE_FORM_GETTER, &args![this]).u32();
    let their_template = e.call(TEMPLATE_FORM_GETTER, &args![other]).u32();
    if my_template != their_template {
        return true;
    }
    e.get(this, TESActorBaseData::sTemplateUseFlags)
        != e.get(other, TESActorBaseData::sTemplateUseFlags)
}

// Translated from 0047d680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::GetFactionRank` (Xbox PDB): the rank (sign-extended
/// byte) of `faction` in the list, or -1. The search ends at the first node
/// with a null item. When `check_faction_flag` is not zero, factions whose
/// flag word has bit 3 set (`fn_0047d720`) never match.
pub fn tes_actor_base_data_get_faction_rank(
    e: &mut Engine,
    this: Ptr<TESActorBaseData>,
    faction: u32,
    check_faction_flag: u8,
) -> i32 {
    let mut node = faction_list_head(e, this);
    loop {
        if node == 0 || node_item(e, node) == 0 {
            return -1;
        }
        let entry = node_item(e, node);
        let entry_faction = e.mem.u32(entry);
        if check_faction_flag != 0 {
            if entry != 0 && !fn_0047d720(e, Ptr::new(entry_faction)) && entry_faction == faction {
                return e.mem.i8(entry + 4) as i32;
            }
        } else if entry != 0 && entry_faction == faction {
            return e.mem.i8(entry + 4) as i32;
        }
        node = node_next(e, node);
    }
}

// Translated from 0047d720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 3 (`8`) of the word at `+0x34` of a faction form is set.
/// Unnamed in the map.
pub fn fn_0047d720(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x34) & 8 != 0
}

// Translated from 0047d740 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::IsInEvilFactionsOnly` (Xbox PDB): true when the list
/// is not empty and every non-null entry's faction has flag bit 1
/// (`fn_0047d7c0`).
pub fn tes_actor_base_data_is_in_evil_factions_only(
    e: &mut Engine,
    this: Ptr<TESActorBaseData>,
) -> bool {
    let mut result = true;
    let mut node = faction_list_head(e, this);
    if e.call(LIST_IS_EMPTY, &args![node]).bool() {
        result = false;
    }
    while node != 0 && result {
        let entry = node_item(e, node);
        if entry != 0 {
            let faction = e.mem.u32(entry);
            if !fn_0047d7c0(e, Ptr::new(faction)) {
                result = false;
            }
        }
        node = node_next(e, node);
    }
    result
}

// Translated from 0047d7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 1 (`2`) of the word at `+0x34` of a faction form is set.
/// Unnamed in the map.
pub fn fn_0047d7c0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x34) & 2 != 0
}

// Translated from 0047d7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 2 (`4`) of the word at `+0x34` of a faction form is set.
/// Unnamed in the map.
pub fn fn_0047d7e0(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u32(this.addr() + 0x34) & 4 != 0
}

// Translated from 0047d800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::SetFactionRank` (Xbox PDB): does nothing for a null
/// faction. If the faction is in the list, rank `-1` removes its node (the
/// removal takes the node's successor into its place, `0063f7b0`) and any
/// other rank is stored; otherwise a new 8-byte `FACTION_RANK` is added to
/// the list (`005ae3d0`).
pub fn tes_actor_base_data_set_faction_rank(
    e: &mut Engine,
    this: Ptr<TESActorBaseData>,
    faction: u32,
    rank: i8,
) {
    if faction == 0 {
        return;
    }
    let mut node = faction_list_head(e, this);
    while node != 0 {
        let entry = node_item(e, node);
        if entry != 0 && e.mem.u32(entry) == faction {
            if rank == -1 {
                e.call(LIST_REMOVE_HEAD, &args![node]);
            } else {
                e.mem.set_i8(entry + 4, rank);
            }
            return;
        }
        node = node_next(e, node);
    }
    let entry = e.call(OPERATOR_NEW, &args![8u32]).u32();
    e.mem.set_u32(entry, faction);
    e.mem.set_i8(entry + 4, rank);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry);
        let head = faction_list_head(e, this);
        e.call(LIST_TAKE_HEAD, &args![head, slot]);
    });
}

// Translated from 0047d8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::InitItem` (Xbox PDB): after load, resolves the stored
/// form IDs. Each faction entry's ID is remapped to `form`'s last file
/// (`TESForm::AddCompileIndex`) and replaced by the faction form (a cast of
/// the looked-up form; an entry that does not resolve is removed from the
/// list); the voice type, the death item (logging when it cannot be found)
/// and the template form (logging likewise) are resolved the same way. Then,
/// unless the template-use flag 0 is set on a template, a stale karma value
/// is repaired: the first call checks that `GetAlignmentForKarma` maps each
/// of the five default alignment values back to its index and caches
/// (`011c52a0`..`011c52a4`) the gaps between the thresholds; if the table is
/// inconsistent and the karma equals one of the five default values within
/// the tolerance, it is moved to the corresponding value derived from the
/// thresholds, else `Invalid karma value found. Unable to fix.` is logged.
pub fn tes_actor_base_data_init_item(e: &mut Engine, this: Ptr<TESActorBaseData>, form: Ptr) {
    let mut node = faction_list_head(e, this);
    let file = if form.is_null() {
        0
    } else {
        e.call(TES_FORM_GET_FILE, &args![form, 0xffff_ffffu32])
            .u32()
    };
    while node != 0 {
        let entry = node_item(e, node);
        if entry == 0 {
            node = node_next(e, node);
            continue;
        }
        if e.mem.u32(entry) != 0 {
            let id = e.mem.u32(entry);
            let (_, resolved) = resolve_form_id(e, id, file, Some(RTTI_FACTION));
            e.mem.set_u32(entry, resolved);
        }
        if e.mem.u32(entry) != 0 {
            node = node_next(e, node);
        } else {
            e.call(LIST_REMOVE_HEAD, &args![node]);
        }
    }

    // Voice type.
    let id = e.get(this, TESActorBaseData::pVoiceType).addr();
    if id != 0 {
        let (_, resolved) = resolve_form_id(e, id, file, Some(RTTI_VOICE_TYPE));
        e.set(this, TESActorBaseData::pVoiceType, Ptr::new(resolved));
    }
    // Death item.
    let id = e.get(this, TESActorBaseData::pDeathItem).addr();
    if id != 0 {
        let (remapped, resolved) = resolve_form_id(e, id, file, Some(RTTI_LEVELED_ITEM));
        e.set(this, TESActorBaseData::pDeathItem, Ptr::new(resolved));
        if resolved == 0 {
            log_missing_form(e, TEXT_DEATH_ITEM_MISSING, remapped, form);
        }
    }
    // Template form (not cast).
    let id = e.get(this, TESActorBaseData::pTemplateForm).addr();
    if id != 0 {
        let (remapped, resolved) = resolve_form_id(e, id, file, None);
        e.set(this, TESActorBaseData::pTemplateForm, Ptr::new(resolved));
        if resolved == 0 {
            log_missing_form(e, TEXT_TEMPLATE_MISSING, remapped, form);
        }
    }

    if !e.get(this, TESActorBaseData::pTemplateForm).is_null() && fn_0047cd40(e, this, 0) {
        return;
    }
    if e.global::<i32>(ALIGNMENT_TABLE_STATE) == -1 {
        for index in 0..5i32 {
            let value: f32 = e.global(DEFAULT_ALIGNMENT_VALUES + 4 * index as u32);
            if tes_actor_base_data_get_alignment_for_karma(e, value) != index {
                e.set_global(ALIGNMENT_TABLE_STATE, 1i32);
                let a = setting_float(e, KARMA_THRESHOLD_011C5090);
                let b = setting_float(e, KARMA_THRESHOLD_011C4E54);
                e.set_global(ALIGN_DIFF_011C52A4, (a as f64 - b as f64) as f32);
                let a = setting_float(e, KARMA_THRESHOLD_011C4E54);
                let b = setting_float(e, KARMA_THRESHOLD_011C5144);
                e.set_global(ALIGN_DIFF_011C52A0, (a as f64 - b as f64) as f32);
                let a = setting_float(e, KARMA_THRESHOLD_011C4F64);
                let b = setting_float(e, KARMA_THRESHOLD_011C5144);
                e.set_global(ALIGN_DIFF_011C529C, (a as f64 - b as f64) as f32);
                break;
            }
        }
        if e.global::<i32>(ALIGNMENT_TABLE_STATE) != 1 {
            e.set_global(ALIGNMENT_TABLE_STATE, 0i32);
        }
    }
    if e.global::<i32>(ALIGNMENT_TABLE_STATE) != 1 {
        return;
    }
    let tolerance: f32 = e.global(KARMA_TOLERANCE);
    let half: f64 = e.global(HALF);
    let hundred: f64 = e.global(HUNDRED);
    let near_default = |e: &mut Engine, index: u32| -> bool {
        let karma = e.get(this, TESActorBaseData::fKarma);
        let default: f32 = e.global(DEFAULT_ALIGNMENT_VALUES + 4 * index);
        e.call(FLOATS_NEAR, &args![karma, default, tolerance])
            .bool()
    };
    let new_karma = if near_default(e, 1) {
        let base = setting_float(e, KARMA_THRESHOLD_011C4E54);
        let diff: f32 = e.global(ALIGN_DIFF_011C52A0);
        Some(base as f64 - diff as f64 * half)
    } else if near_default(e, 0) {
        let base = setting_float(e, KARMA_THRESHOLD_011C4E54);
        let diff: f32 = e.global(ALIGN_DIFF_011C52A4);
        Some(diff as f64 * half + base as f64)
    } else if near_default(e, 2) {
        let base = setting_float(e, KARMA_THRESHOLD_011C5144);
        let diff: f32 = e.global(ALIGN_DIFF_011C529C);
        Some(diff as f64 * half + base as f64)
    } else if near_default(e, 3) {
        let base = setting_float(e, KARMA_THRESHOLD_011C5090);
        Some(base as f64 + hundred)
    } else if near_default(e, 4) {
        let base = setting_float(e, KARMA_THRESHOLD_011C4F64);
        Some(base as f64 - hundred)
    } else {
        e.call(MESSAGE_LOG, &args![TEXT_INVALID_KARMA]);
        None
    };
    if let Some(karma) = new_karma {
        e.set(this, TESActorBaseData::fKarma, karma as f32);
    }
}

/// Remaps a stored form ID to the load order (`AddCompileIndex`), looks the
/// form up and, with a type descriptor, casts it (`TESForm` to that type).
/// Returns the remapped ID and the result.
fn resolve_form_id(e: &mut Engine, id: u32, file: u32, cast_to: Option<u32>) -> (u32, u32) {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), id);
        e.call(TES_FORM_ADD_COMPILE_INDEX, &args![slot, file]);
        let remapped = e.mem.u32(slot.addr());
        let found = e.call(LOOKUP_FORM_BY_ID, &args![remapped]).u32();
        let result = match cast_to {
            Some(target) => e
                .call(
                    DYNAMIC_CAST,
                    &args![found, 0u32, RTTI_TES_FORM, target, 0u32],
                )
                .u32(),
            None => found,
        };
        (remapped, result)
    })
}

/// The "could not find form" log of `InitItem`: the message, the missing
/// ID, the form ID (`0084e3a0`) and name (form virtual `+0x130`) of `form`.
fn log_missing_form(e: &mut Engine, text: u32, id: u32, form: Ptr) {
    let name = e.vcall(form.addr(), 0x130, &args![]).u32();
    let form_id = e.call(FORM_ID_GETTER, &args![form]).u32();
    e.call(MESSAGE_LOG, &args![text, id, form_id, name]);
}

// Translated from 0047dd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the base flags and marks the form changed (`0x2`). Unnamed in the
/// map.
pub fn fn_0047dd30(e: &mut Engine, this: Ptr<TESActorBaseData>, flags: u32) {
    e.set(this, TESActorBaseData::iActorBaseFlags, flags);
    fn_0047e2a0(e, this, 2);
}

// Translated from 0047dd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::SetFlagBit` (Xbox PDB): sets or clears the base flag
/// bits `mask` (`value` not zero sets). When the state changes: for mask `1`
/// (the form's own change flag `0x1000000`) the component is cast to a
/// `TESForm` and, if the save/load game says it has that change
/// (`BGSSaveLoadGame::GetChange`), `0047e2f0` is called with `0x1000000`,
/// otherwise `0047e2a0`; for any other mask `0047e2a0` is called with `2`
/// when `mark_changed` is not zero.
pub fn tes_actor_base_data_set_flag_bit(
    e: &mut Engine,
    this: Ptr<TESActorBaseData>,
    mask: u32,
    value: u8,
    mark_changed: u8,
) {
    let flags = e.get(this, TESActorBaseData::iActorBaseFlags);
    let current = (flags & mask != 0) as u8;
    if current != value {
        if mask == 1 {
            let form = e
                .call(
                    DYNAMIC_CAST,
                    &args![this, 0u32, RTTI_TES_ACTOR_BASE_DATA, RTTI_TES_FORM, 0u32],
                )
                .u32();
            if form != 0 {
                // The by-value `BGSChangeFlags` argument is built in place
                // (`008c71b0` stores the mask into a one-word object).
                let change_flags = e.with_stack(4, |e, slot| {
                    e.call(CHANGE_FLAGS_CONSTRUCT, &args![slot, 0x0100_0000u32]);
                    e.mem.u32(slot.addr())
                });
                let save_load = e.mem.u32(DATA_HANDLER_GLOBAL);
                if !e
                    .call(GET_CHANGE, &args![save_load, form, change_flags])
                    .bool()
                {
                    fn_0047e2a0(e, this, 0x0100_0000);
                } else {
                    fn_0047e2f0(e, this, 0x0100_0000);
                }
            }
        } else if mark_changed != 0 {
            fn_0047e2a0(e, this, 2);
        }
    }
    let flags = e.get(this, TESActorBaseData::iActorBaseFlags);
    let flags = if value != 0 {
        flags | mask
    } else {
        flags & !mask
    };
    e.set(this, TESActorBaseData::iActorBaseFlags, flags);
}

// Translated from 0047de40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `usCalcLevelMin` and marks the form changed (`2`). Unnamed in the map.
pub fn fn_0047de40(e: &mut Engine, this: Ptr<TESActorBaseData>, value: u16) {
    e.set(this, TESActorBaseData::usCalcLevelMin, value);
    fn_0047e2a0(e, this, 2);
}

// Translated from 0047de70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `usCalcLevelMax` and marks the form changed (`2`). Unnamed in the map.
pub fn fn_0047de70(e: &mut Engine, this: Ptr<TESActorBaseData>, value: u16) {
    e.set(this, TESActorBaseData::usCalcLevelMax, value);
    fn_0047e2a0(e, this, 2);
}

// Translated from 0047dea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `usBaseDisposition` and marks the form changed (`2`). Unnamed in the
/// map.
pub fn fn_0047dea0(e: &mut Engine, this: Ptr<TESActorBaseData>, value: u16) {
    e.set(this, TESActorBaseData::usBaseDisposition, value);
    fn_0047e2a0(e, this, 2);
}

// Translated from 0047ded0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The actor's level: `sLevel`, which, when base flag `0x80` is set
/// (`00461560`) and the object at global `011dea3c` has a form at `+0x20`
/// with a component at `+0x30`, is `sLevel / 1000.0` times that component's
/// `sLevel` (truncated). Then, if `usCalcLevelMin` is not zero and the level
/// is below it, that minimum is returned; otherwise a non-zero
/// `usCalcLevelMax` below the level caps it. Unnamed in the map.
pub fn fn_0047ded0(e: &mut Engine, this: Ptr<TESActorBaseData>) -> u16 {
    let mut level = fn_0047d370(e, this);
    if e.call(FLAG_WORD_HAS_BIT_80, &args![this]).bool() {
        let divisor: f64 = e.global(THOUSAND);
        let scale = (level as f64 / divisor) as f32;
        let source = e.mem.u32(LEVEL_SOURCE_OBJECT);
        let form = e.call(WORD_AT_20, &args![source]).u32();
        let component = if form != 0 { form + 0x30 } else { 0 };
        if component != 0 {
            let other = fn_0047d370(e, Ptr::new(component));
            level = float_to_int_truncated(other as f64 * scale as f64) as u16;
        }
        if fn_0047d390(e, this) != 0 && level < fn_0047d390(e, this) {
            return fn_0047d390(e, this);
        }
        if fn_0047d3b0(e, this) != 0 && level > fn_0047d3b0(e, this) {
            level = fn_0047d3b0(e, this);
        }
    }
    level
}

// Translated from 0047dfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `sLevel` and marks the form changed (`2`). Unnamed in the map.
pub fn fn_0047dfe0(e: &mut Engine, this: Ptr<TESActorBaseData>, value: u16) {
    e.set(this, TESActorBaseData::sLevel, value);
    fn_0047e2a0(e, this, 2);
}

// Translated from 0047e010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `usFatigue` and marks the form changed (`2`). Unnamed in the map.
pub fn fn_0047e010(e: &mut Engine, this: Ptr<TESActorBaseData>, value: u16) {
    e.set(this, TESActorBaseData::usFatigue, value);
    fn_0047e2a0(e, this, 2);
}

// Translated from 0047e040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::GetAlignmentForKarma` (Xbox PDB): the alignment index
/// of a karma value from the four threshold settings: for a karma up to the
/// `011c5144` threshold, 4 when also up to `011c4f64`, else 2; for a higher
/// (or NaN) karma, 1 when below `011c4e54` (or NaN), 0 when below `011c5090`
/// (or NaN), else 3.
pub fn tes_actor_base_data_get_alignment_for_karma(e: &mut Engine, karma: f32) -> i32 {
    let karma = karma as f64;
    let first = setting_float(e, KARMA_THRESHOLD_011C5144) as f64;
    if karma <= first {
        let second = setting_float(e, KARMA_THRESHOLD_011C4F64) as f64;
        if karma <= second {
            4
        } else {
            2
        }
    } else {
        let third = setting_float(e, KARMA_THRESHOLD_011C4E54) as f64;
        if !matches!(
            third.partial_cmp(&karma),
            Some(Ordering::Less | Ordering::Equal)
        ) {
            return 1;
        }
        let fourth = setting_float(e, KARMA_THRESHOLD_011C5090) as f64;
        if !matches!(
            fourth.partial_cmp(&karma),
            Some(Ordering::Less | Ordering::Equal)
        ) {
            0
        } else {
            3
        }
    }
}

// Translated from 0047e0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::GetKarmicTitle` (Xbox PDB map name): the title string
/// of an actor, picked by its level (`fn_0087f9f0`, clamped to 1..=50) and the
/// alignment of its karma (the value of actor value `0x17` through the
/// object at `actor + 0xa4`, virtual `+8`; alignments 3 and 4 use the rows of
/// 0 and 2). A missing title gives the empty string; a null actor too.
pub fn tes_actor_base_data_get_karmic_title(e: &mut Engine, actor: Ptr) -> u32 {
    let mut title = 0u32;
    if !actor.is_null() {
        let level = e.call(ACTOR_LEVEL, &args![actor]).u16() as i32;
        let level_index = (level - 1).clamp(0, 0x31);
        let karma = e
            .vcall(actor.addr() + 0xa4, 8, &args![KARMA_ACTOR_VALUE])
            .i32();
        let alignment = tes_actor_base_data_get_alignment_for_karma(e, karma as f32);
        let row = match alignment {
            0..=2 => alignment as u32,
            3 => 0,
            _ => 2,
        };
        let setting = e
            .mem
            .u32(KARMIC_TITLE_TABLE + row * 0xc8 + level_index as u32 * 4);
        if setting != 0 {
            title = e.call(SETTING_STRING, &args![setting]).u32();
        }
    }
    if title == 0 {
        title = EMPTY_STRING;
    }
    title
}

// Translated from 0047e1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the karma, clamps it to the upper setting (`011c5108`) or else
/// the lower one (`011c5160`), and marks the form changed (`2`). Unnamed in
/// the map.
pub fn fn_0047e1c0(e: &mut Engine, this: Ptr<TESActorBaseData>, karma: f32) {
    e.set(this, TESActorBaseData::fKarma, karma);
    let upper = setting_float(e, KARMA_MAX_SETTING);
    let current = e.get(this, TESActorBaseData::fKarma);
    if upper < current {
        e.set(this, TESActorBaseData::fKarma, upper);
    } else {
        let lower = setting_float(e, KARMA_MIN_SETTING);
        let current = e.get(this, TESActorBaseData::fKarma);
        if lower > current {
            e.set(this, TESActorBaseData::fKarma, lower);
        }
    }
    fn_0047e2a0(e, this, 2);
}

// Translated from 0047e240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `usBartergold` and marks the form changed (`2`). Unnamed in the map.
pub fn fn_0047e240(e: &mut Engine, this: Ptr<TESActorBaseData>, value: u16) {
    e.set(this, TESActorBaseData::usBartergold, value);
    fn_0047e2a0(e, this, 2);
}

// Translated from 0047e270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `usSpeedMult` and marks the form changed (`2`). Unnamed in the map.
pub fn fn_0047e270(e: &mut Engine, this: Ptr<TESActorBaseData>, value: u16) {
    e.set(this, TESActorBaseData::usSpeedMult, value);
    fn_0047e2a0(e, this, 2);
}

// Translated from 0047e2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Marks the owning form (`this - 0x30`, null for a null component) changed
/// with `flags`: its virtual `+0x48`. Unnamed in the map.
pub fn fn_0047e2a0(e: &mut Engine, this: Ptr<TESActorBaseData>, flags: u32) {
    let form = if this.is_null() {
        0
    } else {
        this.addr().wrapping_sub(0x30)
    };
    if form != 0 {
        e.vcall(form, 0x48, &args![flags]);
    }
}

// Translated from 0047e2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same on the form's virtual `+0x4c` (the counterpart that clears the
/// change flags). Unnamed in the map.
pub fn fn_0047e2f0(e: &mut Engine, this: Ptr<TESActorBaseData>, flags: u32) {
    let form = if this.is_null() {
        0
    } else {
        this.addr().wrapping_sub(0x30)
    };
    if form != 0 {
        e.vcall(form, 0x4c, &args![flags]);
    }
}

// Translated from 0047e340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets bits in `iChangeFlags`. Unnamed in the map.
pub fn fn_0047e340(e: &mut Engine, this: Ptr<TESActorBaseData>, mask: u32) {
    let flags = e.get(this, TESActorBaseData::iChangeFlags);
    e.set(this, TESActorBaseData::iChangeFlags, flags | mask);
}

// Translated from 0047e360 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears bits in `iChangeFlags`. Unnamed in the map.
pub fn fn_0047e360(e: &mut Engine, this: Ptr<TESActorBaseData>, mask: u32) {
    let flags = e.get(this, TESActorBaseData::iChangeFlags);
    e.set(this, TESActorBaseData::iChangeFlags, flags & !mask);
}

/// Whether the debug switch byte (`011de4e8 + 4`) is set.
fn save_debug_enabled(e: &mut Engine) -> bool {
    let byte = e.call(BYTE_FIELD_POINTER, &args![SAVE_DEBUG_SWITCH]).u32();
    e.mem.u8(byte) != 0
}

// Translated from 0047e380 (decompiled, FalloutNV.exe 1.4.0.525)
/// `GetSaveSize` of the component (unnamed in the map): the size `SaveGame`
/// writes: 6 for the block header when save-game blocks are in use, `0x18`
/// when `flags` has bit 1, 2 for the faction count and 5 per non-null
/// faction entry. With the debug switch on, logs the size.
pub fn fn_0047e380(e: &mut Engine, this: Ptr<TESActorBaseData>, flags: u32) -> u16 {
    let mut size: u16 = 0;
    let base = size;
    let save_load = e.mem.u32(SAVE_LOAD_GAME_GLOBAL);
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
        size = size.wrapping_add(4);
        size = size.wrapping_add(2);
    }
    if flags & 2 != 0 {
        size = size.wrapping_add(0x18);
    }
    size = size.wrapping_add(2);
    let count = e.call(LIST_COUNT_ITEMS, &args![this.addr() + 0x2c]).u32();
    size = size.wrapping_add(count.wrapping_mul(5) as u16);
    if save_debug_enabled(e) {
        log_save_function(
            e,
            save_load,
            size.wrapping_sub(base) as u32,
            0xc4e,
            TEXT_SAVE_SIZE_FORM,
            TEXT_SAVE_SIZE_PLAIN,
        );
    }
    size
}

/// The debug line of `GetSaveSize` / `SaveGame`: with a current save entry
/// the message names the entry's form ID, its form's name (virtual `+0x130`)
/// and the entry's flags (the word at `+5`).
fn log_save_function(
    e: &mut Engine,
    save_load: u32,
    amount: u32,
    line: u32,
    text_with_form: u32,
    text_plain: u32,
) {
    let entry = e.call(CURRENT_SAVE_ENTRY, &args![save_load]).u32();
    if entry != 0 {
        let form_id = e.mem.u32(entry);
        let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
        let name = e.vcall(form, 0x130, &args![]).u32();
        let entry_flags = e.mem.u32(entry + 5);
        e.call(
            ERROR_LOG,
            &args![
                text_with_form,
                amount,
                form_id,
                name,
                entry_flags,
                line,
                SOURCE_FILE_PATH
            ],
        );
    } else {
        e.call(
            ERROR_LOG,
            &args![text_plain, amount, line, SOURCE_FILE_PATH],
        );
    }
}

// Translated from 0047e4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SaveGame` of the component (unnamed in the map): writes, into the save
/// buffer (global `011de45c`), a `KOBL` block header and a 16-bit size
/// placeholder when blocks are in use, the `0x18` bytes of `ACTOR_BASE_DATA`
/// when `flags` has bit 1, then the faction count (16 bit) and for each
/// non-null `FACTION_RANK` its faction's form ID (as a numeric ID, 4 bytes)
/// and rank (1 byte); the count and the block size are patched in at the end.
/// The debug switch logs the number of bytes written; a block over 64 KiB is
/// reported.
pub fn fn_0047e4a0(e: &mut Engine, this: Ptr<TESActorBaseData>, flags: u32) {
    let save_load = e.mem.u32(SAVE_LOAD_GAME_GLOBAL);
    let mut block_size_slot = 0u32;
    let mut start = e.call(BUFFER_POSITION, &args![save_load]).u32();
    if save_debug_enabled(e) {
        start = e.call(BUFFER_POSITION, &args![save_load]).u32();
    }
    let blocks = e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool();
    if blocks {
        e.with_stack(4, |e, tag| {
            e.mem.set_u32(tag.addr(), TAG_KOBL);
            e.call(SAVE_BYTES, &args![save_load, tag, 4u32]);
        });
        block_size_slot = e.call(BUFFER_POSITION, &args![save_load]).u32();
        e.with_stack(4, |e, size| {
            e.call(SAVE_BYTES, &args![save_load, size, 2u32]);
        });
    }
    if flags & 2 != 0 {
        e.call(SAVE_BYTES, &args![save_load, this.addr() + 4, 0x18u32]);
    }
    let mut count = 0u16;
    let count_slot = e.call(BUFFER_POSITION, &args![save_load]).u32();
    e.with_stack(4, |e, zero| {
        e.call(SAVE_BYTES, &args![save_load, zero, 2u32]);
    });
    let mut node = faction_list_head(e, this);
    while node != 0 {
        let entry = node_item(e, node);
        if entry != 0 {
            let faction = e.mem.u32(entry);
            let numeric_id = e.call(FORM_ID_GETTER, &args![faction]).u32();
            let rank = e.mem.u8(entry + 4);
            e.with_stack(4, |e, id_slot| {
                e.mem.set_u32(id_slot.addr(), numeric_id);
                e.call(SAVE_NUMERIC_ID, &args![save_load, id_slot, 4u32]);
            });
            e.with_stack(4, |e, rank_slot| {
                e.mem.set_u8(rank_slot.addr(), rank);
                e.call(SAVE_BYTES, &args![save_load, rank_slot, 1u32]);
            });
            count = count.wrapping_add(1);
        }
        node = node_next(e, node);
    }
    e.mem.set_u16(count_slot, count);
    if save_debug_enabled(e) {
        let end = e.call(BUFFER_POSITION, &args![save_load]).u32();
        log_save_function(
            e,
            save_load,
            end.wrapping_sub(start),
            0xc77,
            TEXT_SAVE_FORM,
            TEXT_SAVE_PLAIN,
        );
    }
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
        let end = e.call(BUFFER_POSITION, &args![save_load]).u32();
        if end > block_size_slot.wrapping_add(0xffff) {
            e.call(
                MESSAGE_LOG,
                &args![TEXT_SAVE_BLOCK_TOO_BIG, SOURCE_FILE_PATH, 0xc77u32],
            );
        }
        e.mem
            .set_u16(block_size_slot, end.wrapping_sub(block_size_slot) as u16);
    }
}

/// One `LoadGame` buffer message (`005b5e40`). `amount` is `Some(bytes)` for
/// the overrun / underrun messages. With a current load entry the message
/// names its form (the form's virtual `+0x130`, which takes no argument but
/// leaves the entry's version byte at `+9` and flags word at `+5` on the
/// stack as the last two message arguments); without one it ends with the
/// save version byte.
fn log_load_message(
    e: &mut Engine,
    save_load: u32,
    entry: u32,
    texts: (u32, u32),
    amount: Option<u32>,
    line: u32,
) {
    let mut words = Vec::new();
    if entry != 0 {
        let form_id = e.mem.u32(entry);
        let form = e.call(LOOKUP_FORM_BY_ID, &args![form_id]).u32();
        let name = e.vcall(form, 0x130, &args![]).u32();
        words.push(texts.1);
        words.extend(amount);
        words.extend([SOURCE_FILE_PATH, line, form_id, name]);
        words.push(e.mem.u8(entry + 9) as u32);
        words.push(e.mem.u32(entry + 5));
    } else {
        let version = e.call(SAVE_VERSION, &args![save_load]).u32() & 0xff;
        words.push(texts.0);
        words.extend(amount);
        words.extend([SOURCE_FILE_PATH, line, version]);
    }
    e.call(MESSAGE_LOG, &words);
}

// Translated from 0047e6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::LoadGame` (Xbox PDB): reads, from the save buffer
/// (global `011de45c`), the `KOBL` block header (logging when it differs)
/// and the 16-bit block size when blocks are in use, the `0x18` bytes of
/// `ACTOR_BASE_DATA` when `flags` has bit 1, then replaces the faction list
/// by the stored entries (16-bit count; per entry a numeric form ID and a
/// rank byte; entries whose form does not resolve are dropped). With blocks
/// in use it finally logs a buffer overrun or underrun against the block
/// size. `RET 8`: the second stack word is never read.
pub fn tes_actor_base_data_load_game(
    e: &mut Engine,
    this: Ptr<TESActorBaseData>,
    flags: u32,
    _unused_1: u32,
) {
    let save_load = e.mem.u32(SAVE_LOAD_GAME_GLOBAL);
    let mut block_size = 0u16;
    let mut block_start = 0u32;
    let blocks = e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool();
    if blocks {
        let tag = e.with_stack(4, |e, tag| {
            e.call(LOAD_BYTES, &args![save_load, tag, 4u32]);
            e.mem.u32(tag.addr())
        });
        if tag != TAG_KOBL {
            let entry = e.call(CURRENT_LOAD_ENTRY, &args![save_load]).u32();
            log_load_message(
                e,
                save_load,
                entry,
                (TEXT_LOAD_HEADER_PLAIN, TEXT_LOAD_HEADER_FORM),
                None,
                0xc80,
            );
        }
        block_start = e.call(BUFFER_POSITION, &args![save_load]).u32();
        block_size = e.with_stack(4, |e, size| {
            e.mem.set_u16(size.addr(), 0);
            e.call(LOAD_BYTES, &args![save_load, size, 2u32]);
            e.mem.u16(size.addr())
        });
    }
    if flags & 2 != 0 {
        e.call(LOAD_BYTES, &args![save_load, this.addr() + 4, 0x18u32]);
    }
    tes_actor_base_data_clear_faction_list(e, this);
    let count = e.with_stack(4, |e, count| {
        e.call(LOAD_BYTES, &args![save_load, count, 2u32]);
        e.mem.u16(count.addr())
    });
    for _ in 0..count {
        let (id, rank) = e.with_stack(8, |e, slot| {
            e.call(LOAD_NUMERIC_ID, &args![save_load, slot, 4u32]);
            e.call(LOAD_BYTES, &args![save_load, slot.addr() + 4, 1u32]);
            (e.mem.u32(slot.addr()), e.mem.u8(slot.addr() + 4))
        });
        let faction = e.call(LOOKUP_FORM_BY_ID, &args![id]).u32();
        if faction != 0 {
            let entry = e.call(OPERATOR_NEW, &args![8u32]).u32();
            e.mem.set_u32(entry, faction);
            e.mem.set_u8(entry + 4, rank);
            e.with_stack(4, |e, slot| {
                e.mem.set_u32(slot.addr(), entry);
                let head = faction_list_head(e, this);
                e.call(LIST_TAKE_HEAD, &args![head, slot]);
            });
        }
    }
    if e.call(USE_SAVE_GAME_BLOCKS, &args![save_load]).bool() {
        let position = e.call(BUFFER_POSITION, &args![save_load]).u32();
        let entry = e.call(CURRENT_LOAD_ENTRY, &args![save_load]).u32();
        let expected = (block_size as u32).wrapping_add(block_start);
        if position > expected {
            log_load_message(
                e,
                save_load,
                entry,
                (TEXT_LOAD_OVERRUN_PLAIN, TEXT_LOAD_OVERRUN_FORM),
                Some(position - expected),
                0xca2,
            );
        } else if position < expected {
            log_load_message(
                e,
                save_load,
                entry,
                (TEXT_LOAD_UNDERRUN_PLAIN, TEXT_LOAD_UNDERRUN_FORM),
                Some(expected - position),
                0xca2,
            );
        }
    }
}

// Translated from 004a8ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESActorBaseData::GetFatigue` (Xbox PDB): the 16-bit `usFatigue`.
pub fn tes_actor_base_data_get_fatigue(e: &mut Engine, this: Ptr<TESActorBaseData>) -> u16 {
    e.get(this, TESActorBaseData::usFatigue)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0047c700,
            tes_actor_base_data_tes_actor_base_data(Ptr<TESActorBaseData>) -> Ptr<TESActorBaseData>
        ),
        entry!(
            0x0047c770,
            tes_actor_base_data_get_no_vats_melee(Ptr<TESActorBaseData>) -> bool
        ),
        entry!(
            0x0047c790,
            tes_actor_base_data_get_allow_pc_dialogue(Ptr<TESActorBaseData>) -> bool
        ),
        entry!(
            0x0047c7b0,
            tes_actor_base_data_get_allow_pick_pocket(Ptr<TESActorBaseData>) -> bool
        ),
        entry!(
            0x0047c7d0,
            tes_actor_base_data_get_is_ghost(Ptr<TESActorBaseData>) -> bool
        ),
        entry!(
            0x0047c7f0,
            tes_actor_base_data_get_invulnerable(Ptr<TESActorBaseData>) -> bool
        ),
        entry!(
            0x0047c810,
            tes_actor_base_data_get_cant_open_doors(Ptr<TESActorBaseData>) -> bool
        ),
        entry!(
            0x0047c830,
            tes_actor_base_data_get_auto_calc_service_flags(Ptr<TESActorBaseData>) -> bool
        ),
        entry!(0x0047c850, fn_0047c850(Ptr) -> bool),
        entry!(
            0x0047c860,
            tes_actor_base_data_get_karma(Ptr<TESActorBaseData>) -> f32
        ),
        entry!(0x0047c880, fn_0047c880(Ptr<TESActorBaseData>)),
        entry!(
            0x0047c8e0,
            tes_actor_base_data_initialize_data_component(Ptr<TESActorBaseData>)
        ),
        entry!(
            0x0047c9b0,
            tes_actor_base_data_clear_data_component(Ptr<TESActorBaseData>)
        ),
        entry!(
            0x0047c9d0,
            tes_actor_base_data_clear_faction_list(Ptr<TESActorBaseData>)
        ),
        entry!(
            0x0047ca20,
            tes_actor_base_data_copy_faction_list(Ptr<TESActorBaseData>, Ptr<TESActorBaseData>)
        ),
        entry!(0x0047cac0, tes_actor_base_data_save(Ptr<TESActorBaseData>)),
        entry!(0x0047cb90, fn_0047cb90(Ptr)),
        entry!(0x0047cca0, fn_0047cca0(Ptr<TESActorBaseData>) -> bool),
        entry!(0x0047ccc0, fn_0047ccc0(Ptr<TESActorBaseData>, u8)),
        entry!(0x0047cce0, fn_0047cce0(Ptr<TESActorBaseData>, i32, u8)),
        entry!(0x0047cd40, fn_0047cd40(Ptr<TESActorBaseData>, i32) -> bool),
        entry!(0x0047cd90, fn_0047cd90(Ptr<TESActorBaseData>, u16)),
        entry!(0x0047cdb0, fn_0047cdb0(Ptr<TESActorBaseData>) -> bool),
        entry!(0x0047cdd0, fn_0047cdd0(Ptr<TESActorBaseData>) -> bool),
        entry!(0x0047ce10, fn_0047ce10(Ptr<TESActorBaseData>, Ptr)),
        entry!(0x0047d0b0, path_manager_q_instance() -> u32),
        entry!(0x0047d130, create_template_actor_base(u32, u32) -> u32),
        entry!(0x0047d1a0, fn_0047d1a0(Ptr, Ptr) -> Ptr),
        entry!(
            0x0047d220,
            tes_actor_base_data_copy_component(Ptr<TESActorBaseData>, Ptr)
        ),
        entry!(0x0047d370, fn_0047d370(Ptr<TESActorBaseData>) -> u16),
        entry!(0x0047d390, fn_0047d390(Ptr<TESActorBaseData>) -> u16),
        entry!(0x0047d3b0, fn_0047d3b0(Ptr<TESActorBaseData>) -> u16),
        entry!(0x0047d3d0, fn_0047d3d0(Ptr<TESActorBaseData>) -> u16),
        entry!(0x0047d3f0, fn_0047d3f0(Ptr<TESActorBaseData>) -> u16),
        entry!(
            0x0047d410,
            tes_actor_base_data_compare_component(Ptr<TESActorBaseData>, Ptr) -> bool
        ),
        entry!(
            0x0047d680,
            tes_actor_base_data_get_faction_rank(Ptr<TESActorBaseData>, u32, u8) -> i32
        ),
        entry!(0x0047d720, fn_0047d720(Ptr) -> bool),
        entry!(
            0x0047d740,
            tes_actor_base_data_is_in_evil_factions_only(Ptr<TESActorBaseData>) -> bool
        ),
        entry!(0x0047d7c0, fn_0047d7c0(Ptr) -> bool),
        entry!(0x0047d7e0, fn_0047d7e0(Ptr) -> bool),
        entry!(
            0x0047d800,
            tes_actor_base_data_set_faction_rank(Ptr<TESActorBaseData>, u32, i8)
        ),
        entry!(
            0x0047d8b0,
            tes_actor_base_data_init_item(Ptr<TESActorBaseData>, Ptr)
        ),
        entry!(0x0047dd30, fn_0047dd30(Ptr<TESActorBaseData>, u32)),
        entry!(
            0x0047dd50,
            tes_actor_base_data_set_flag_bit(Ptr<TESActorBaseData>, u32, u8, u8)
        ),
        entry!(0x0047de40, fn_0047de40(Ptr<TESActorBaseData>, u16)),
        entry!(0x0047de70, fn_0047de70(Ptr<TESActorBaseData>, u16)),
        entry!(0x0047dea0, fn_0047dea0(Ptr<TESActorBaseData>, u16)),
        entry!(0x0047ded0, fn_0047ded0(Ptr<TESActorBaseData>) -> u16),
        entry!(0x0047dfe0, fn_0047dfe0(Ptr<TESActorBaseData>, u16)),
        entry!(0x0047e010, fn_0047e010(Ptr<TESActorBaseData>, u16)),
        entry!(0x0047e040, tes_actor_base_data_get_alignment_for_karma(f32) -> i32),
        entry!(0x0047e0e0, tes_actor_base_data_get_karmic_title(Ptr) -> u32),
        entry!(0x0047e1c0, fn_0047e1c0(Ptr<TESActorBaseData>, f32)),
        entry!(0x0047e240, fn_0047e240(Ptr<TESActorBaseData>, u16)),
        entry!(0x0047e270, fn_0047e270(Ptr<TESActorBaseData>, u16)),
        entry!(0x0047e2a0, fn_0047e2a0(Ptr<TESActorBaseData>, u32)),
        entry!(0x0047e2f0, fn_0047e2f0(Ptr<TESActorBaseData>, u32)),
        entry!(0x0047e340, fn_0047e340(Ptr<TESActorBaseData>, u32)),
        entry!(0x0047e360, fn_0047e360(Ptr<TESActorBaseData>, u32)),
        entry!(0x0047e380, fn_0047e380(Ptr<TESActorBaseData>, u32) -> u16),
        entry!(0x0047e4a0, fn_0047e4a0(Ptr<TESActorBaseData>, u32)),
        entry!(
            0x0047e6f0,
            tes_actor_base_data_load_game(Ptr<TESActorBaseData>, u32, u32)
        ),
        entry!(
            0x004a8ae0,
            tes_actor_base_data_get_fatigue(Ptr<TESActorBaseData>) -> u16
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// Every `(slot, argument words)` a recording vtable double saw, with
    /// the receiver first.
    type Seen = Rc<RefCell<Vec<Vec<u32>>>>;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn ret_float(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    /// An engine with the pages the globals live in mapped and doubles for
    /// the allocator and for the list operations whose real versions need
    /// the game's node allocator. The pure list getters (`005d8a70`,
    /// `006815c0`, `00726070`, `008256d0`) and the setting getters run for
    /// real.
    fn engine() -> Engine {
        let mut e = Engine::new();
        for page in [
            0x0101_1000u32,
            0x0101_2000,
            0x0101_5000,
            0x0101_7000,
            0x0101_b000,
        ] {
            e.map(page, 0x1000);
        }
        e.map(0x0118_3000, 0x4000);
        e.map(0x011c_4000, 0x2000);
        e.map(0x011d_d000, 0x3000);
        e.register(OPERATOR_NEW, |e, a| ret(e.mem.alloc(a[0])));
        e.register(OPERATOR_DELETE, |e, a| {
            if a[0] != 0 {
                e.mem.free(a[0]);
            }
            Ret::default()
        });
        e.register(MEMCPY, |e, a| {
            let bytes = e.mem.bytes(a[1], a[2]);
            e.mem.write(a[0], &bytes);
            ret(a[0])
        });
        // Remove the first node: the head takes the next node's item.
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            if next != 0 {
                let item = e.mem.u32(next);
                let after = e.mem.u32(next + 4);
                e.mem.set_u32(a[0], item);
                e.mem.set_u32(a[0] + 4, after);
            } else {
                e.mem.set_u32(a[0], 0);
            }
            Ret::default()
        });
        // Take over the head item of `other`, keeping the old head as the
        // second node.
        e.register(LIST_TAKE_HEAD, |e, a| {
            let item = e.mem.u32(a[1]);
            if item != 0 {
                let old = e.mem.u32(a[0]);
                if old != 0 {
                    let node = e.mem.alloc(8);
                    e.mem.set_u32(node, old);
                    let next = e.mem.u32(a[0] + 4);
                    e.mem.set_u32(node + 4, next);
                    e.mem.set_u32(a[0] + 4, node);
                }
                e.mem.set_u32(a[0], item);
            }
            Ret::default()
        });
        // Append an item at the end of the chain.
        e.register(LIST_APPEND, |e, a| {
            let item = e.mem.u32(a[1]);
            if item != 0 {
                let mut node = a[0];
                while e.mem.u32(node + 4) != 0 {
                    node = e.mem.u32(node + 4);
                }
                if e.mem.u32(node) != 0 {
                    let fresh = e.mem.alloc(8);
                    e.mem.set_u32(node + 4, fresh);
                    node = fresh;
                }
                e.mem.set_u32(node, item);
            }
            Ret::default()
        });
        e.register(LIST_COUNT_ITEMS, |e, a| {
            let mut node = a[0];
            let mut count = 0;
            while node != 0 {
                if e.mem.u32(node) != 0 {
                    count += 1;
                }
                node = e.mem.u32(node + 4);
            }
            ret(count)
        });
        e
    }

    fn log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    /// The addresses called, in order.
    fn called(e: &Engine) -> Vec<u32> {
        e.call_log
            .as_ref()
            .map(|l| l.iter().map(|c| c.0).collect())
            .unwrap_or_default()
    }

    /// Arguments of every logged call to `addr`.
    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .map(|l| {
                l.iter()
                    .filter(|c| c.0 == addr)
                    .map(|c| c.1.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    fn component(e: &mut Engine) -> Ptr<TESActorBaseData> {
        e.new_object()
    }

    /// Writes `items` as a chain of 8-byte nodes starting at `head`.
    fn make_list(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, item) in items.iter().enumerate() {
            e.mem.set_u32(node, *item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            } else {
                e.mem.set_u32(node + 4, 0);
            }
        }
    }

    fn list_items(e: &Engine, head: u32) -> Vec<u32> {
        let mut out = vec![];
        let mut node = head;
        while node != 0 {
            out.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        out
    }

    /// A `FACTION_RANK` (8 bytes) for `faction` with `rank`.
    fn faction_rank(e: &mut Engine, faction: u32, rank: u8) -> u32 {
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, faction);
        e.mem.set_u8(entry + 4, rank);
        entry
    }

    fn put_faction_list(e: &mut Engine, this: Ptr<TESActorBaseData>, entries: &[(u32, u8)]) {
        let items: Vec<u32> = entries
            .iter()
            .map(|&(faction, rank)| faction_rank(e, faction, rank))
            .collect();
        make_list(e, this.addr() + 0x2c, &items);
    }

    /// Writes slot targets of a vtable at `vtable`.
    fn vtable(e: &mut Engine, vtable: u32, slots: &[(u32, u32)]) {
        e.map(vtable & !0xfff, 0x2000);
        for &(slot, target) in slots {
            e.mem.set_u32(vtable + slot, target);
        }
    }

    /// A component inside a form whose vtable records the `+0x48` / `+0x4c`
    /// slot calls (the argument word and which slot).
    fn component_in_form(e: &mut Engine, tag: u32) -> (Ptr<TESActorBaseData>, Seen) {
        let form = e.mem.alloc(0x30 + 0x34);
        let vt = 0x0300_0000 + tag * 0x2000;
        let seen: Seen = Rc::default();
        for slot in [0x48u32, 0x4c] {
            let sink = seen.clone();
            let target = 0x0310_0000 + tag * 0x100 + slot;
            e.register_double(target, move |_, a| {
                sink.borrow_mut().push(vec![slot, a[1]]);
                Ret::default()
            });
            vtable(e, vt, &[(slot, target)]);
        }
        e.mem.set_u32(form, vt);
        (Ptr::new(form + 0x30), seen)
    }

    #[test]
    fn constructor_builds_the_base_writes_the_vtable_and_sets_the_defaults() {
        let mut e = engine();
        e.mem.set_f32(FATIGUE_DEFAULT_SETTING + 4, 75.9);
        e.mem.set_f32(KARMA_DEFAULT_SETTING + 4, 12.5);
        e.mem.set_u16(DISPOSITION_DEFAULT_SETTING + 4, 35);
        let this = component(&mut e);
        e.mem.set_u32(this.addr() + 0x2c, 0xdead);
        log(&mut e);
        let result = e.call(0x0047c700, &args![this]).u32();
        assert_eq!(result, this.addr());
        assert_eq!(
            called(&e)[..3],
            [0x0047c700, BASE_COMPONENT_CONSTRUCT, LIST_NODE_CONSTRUCT]
        );
        assert_eq!(e.mem.u32(this.addr()), TES_ACTOR_BASE_DATA_VTABLE);
        assert_eq!(e.mem.u32(this.addr() + 0x2c), 0);
        assert_eq!(e.get(this, TESActorBaseData::usFatigue), 75);
        assert_eq!(e.get(this, TESActorBaseData::usBaseDisposition), 35);
    }

    fn check_flag_getter(address: u32, mask: u32) {
        let mut e = engine();
        let this = component(&mut e);
        assert!(!e.call(address, &args![this]).bool());
        e.set(this, TESActorBaseData::iActorBaseFlags, mask);
        assert!(e.call(address, &args![this]).bool());
        e.set(this, TESActorBaseData::iActorBaseFlags, !mask);
        assert!(!e.call(address, &args![this]).bool());
    }

    #[test]
    fn get_no_vats_melee_tests_flag_100000() {
        check_flag_getter(0x0047c770, 0x0010_0000);
    }

    #[test]
    fn get_allow_pc_dialogue_tests_flag_200000() {
        check_flag_getter(0x0047c790, 0x0020_0000);
    }

    #[test]
    fn get_allow_pick_pocket_tests_flag_10000000() {
        check_flag_getter(0x0047c7b0, 0x1000_0000);
    }

    #[test]
    fn get_is_ghost_tests_flag_20000000() {
        check_flag_getter(0x0047c7d0, 0x2000_0000);
    }

    #[test]
    fn get_invulnerable_tests_flag_80000000() {
        check_flag_getter(0x0047c7f0, 0x8000_0000);
    }

    #[test]
    fn get_cant_open_doors_tests_flag_400000() {
        check_flag_getter(0x0047c810, 0x0040_0000);
    }

    #[test]
    fn get_auto_calc_service_flags_tests_flag_800000() {
        check_flag_getter(0x0047c830, 0x0080_0000);
    }

    #[test]
    fn get_karma_returns_the_stored_float() {
        let mut e = engine();
        let this = component(&mut e);
        e.set(this, TESActorBaseData::fKarma, -37.25);
        assert_eq!(e.call(0x0047c860, &args![this]).f32(), -37.25);
    }

    #[test]
    fn destructor_body_restores_the_vtable_clears_the_list_and_destroys_the_chain() {
        let mut e = engine();
        e.register(CHAINED_CONTAINER_DESTRUCT, |_, _| Ret::default());
        let this = component(&mut e);
        e.mem.set_u32(this.addr(), 0x1234);
        put_faction_list(&mut e, this, &[(0x100, 1), (0x200, 2)]);
        log(&mut e);
        e.call(0x0047c880, &args![this]);
        assert_eq!(e.mem.u32(this.addr()), TES_ACTOR_BASE_DATA_VTABLE);
        assert_eq!(list_items(&e, this.addr() + 0x2c), vec![0]);
        assert_eq!(
            calls_to(&e, CHAINED_CONTAINER_DESTRUCT),
            vec![vec![this.addr() + 0x2c]]
        );
    }

    #[test]
    fn initialize_data_component_sets_the_defaults_and_truncates_the_fatigue() {
        let mut e = engine();
        e.mem.set_f32(FATIGUE_DEFAULT_SETTING + 4, 70000.9);
        e.mem.set_f32(KARMA_DEFAULT_SETTING + 4, -3.5);
        e.mem.set_u16(DISPOSITION_DEFAULT_SETTING + 4, 50);
        let this = component(&mut e);
        for offset in (4..0x2c).step_by(4) {
            e.mem.set_u32(this.addr() + offset, 0xffff_ffff);
        }
        e.call(0x0047c8e0, &args![this]);
        assert_eq!(e.get(this, TESActorBaseData::iActorBaseFlags), 0);
        assert_eq!(e.get(this, TESActorBaseData::usFatigue), 70000u32 as u16);
        assert_eq!(e.get(this, TESActorBaseData::usBartergold), 0);
        assert_eq!(e.get(this, TESActorBaseData::sLevel), 1);
        assert_eq!(e.get(this, TESActorBaseData::usCalcLevelMin), 0);
        assert_eq!(e.get(this, TESActorBaseData::usCalcLevelMax), 0);
        assert_eq!(e.get(this, TESActorBaseData::usSpeedMult), 100);
        assert_eq!(e.get(this, TESActorBaseData::fKarma), -3.5);
        assert_eq!(e.get(this, TESActorBaseData::usBaseDisposition), 50);
        assert_eq!(e.get(this, TESActorBaseData::sTemplateUseFlags), 0);
        assert!(e.get(this, TESActorBaseData::pDeathItem).is_null());
        assert!(e.get(this, TESActorBaseData::pVoiceType).is_null());
        assert!(e.get(this, TESActorBaseData::pTemplateForm).is_null());
        assert_eq!(e.get(this, TESActorBaseData::iChangeFlags), 0);
        // NaN truncates to the integer indefinite, whose low 16 bits are 0.
        e.mem.set_f32(FATIGUE_DEFAULT_SETTING + 4, f32::NAN);
        e.call(0x0047c8e0, &args![this]);
        assert_eq!(e.get(this, TESActorBaseData::usFatigue), 0);
    }

    #[test]
    fn clear_data_component_clears_the_faction_list() {
        let mut e = engine();
        let this = component(&mut e);
        put_faction_list(&mut e, this, &[(0x100, 1)]);
        log(&mut e);
        e.call(0x0047c9b0, &args![this]);
        assert_eq!(list_items(&e, this.addr() + 0x2c), vec![0]);
        assert_eq!(calls_to(&e, OPERATOR_DELETE).len(), 1);
    }

    #[test]
    fn clear_faction_list_deletes_every_entry_and_stops_when_empty() {
        let mut e = engine();
        let this = component(&mut e);
        put_faction_list(&mut e, this, &[(0x100, 1), (0x200, 2), (0x300, 3)]);
        let first = e.mem.u32(this.addr() + 0x2c);
        log(&mut e);
        e.call(0x0047c9d0, &args![this]);
        assert_eq!(list_items(&e, this.addr() + 0x2c), vec![0]);
        let deletes = calls_to(&e, OPERATOR_DELETE);
        assert_eq!(deletes.len(), 3);
        assert_eq!(deletes[0], vec![first]);
        // An empty list does nothing.
        log(&mut e);
        e.call(0x0047c9d0, &args![this]);
        assert!(calls_to(&e, OPERATOR_DELETE).is_empty());
    }

    #[test]
    fn copy_faction_list_replaces_this_list_with_copies_of_the_sources_entries() {
        let mut e = engine();
        let this = component(&mut e);
        let source = component(&mut e);
        put_faction_list(&mut e, this, &[(0x900, 9)]);
        put_faction_list(&mut e, source, &[(0x100, 1), (0x200, 0xfe)]);
        let originals = list_items(&e, source.addr() + 0x2c);
        e.call(0x0047ca20, &args![this, source]);
        let copies = list_items(&e, this.addr() + 0x2c);
        assert_eq!(copies.len(), 2);
        for (copy, original) in copies.iter().zip(&originals) {
            assert_ne!(copy, original);
            assert_eq!(e.mem.bytes(*copy, 8), e.mem.bytes(*original, 8));
        }
        assert_eq!(list_items(&e, source.addr() + 0x2c), originals);
    }

    #[test]
    fn save_writes_the_acbs_chunk_and_the_form_id_chunks() {
        let mut e = engine();
        e.register(IS_BIG_ENDIAN, |_, _| ret(0));
        e.register(TES_FORM_ADD_CHUNK_DATA, |_, _| Ret::default());
        e.register(ADD_CHUNK_VALUE, |_, _| Ret::default());
        e.register(SWAP_U32, |_, _| Ret::default());
        e.register(SWAP_U16, |_, _| Ret::default());
        let this = component(&mut e);
        let death = e.mem.alloc(0x10);
        e.mem.set_u32(death + 0xc, 0x0100_0aaa);
        let template = e.mem.alloc(0x10);
        e.mem.set_u32(template + 0xc, 0x0100_0bbb);
        e.set(this, TESActorBaseData::pDeathItem, Ptr::new(death));
        e.set(this, TESActorBaseData::pTemplateForm, Ptr::new(template));
        log(&mut e);
        e.call(0x0047cac0, &args![this]);
        assert_eq!(
            calls_to(&e, TES_FORM_ADD_CHUNK_DATA),
            vec![vec![TAG_ACBS, this.addr() + 4, 0x18]]
        );
        assert_eq!(
            calls_to(&e, ADD_CHUNK_VALUE),
            vec![vec![TAG_INAM, 0x0100_0aaa], vec![TAG_TPLT, 0x0100_0bbb]]
        );
        assert!(calls_to(&e, SWAP_U32).is_empty());
        // Big-endian: the data is swapped before and after the write.
        e.register(IS_BIG_ENDIAN, |_, _| ret(1));
        log(&mut e);
        e.call(0x0047cac0, &args![this]);
        assert_eq!(calls_to(&e, SWAP_U32).len(), 4);
        assert_eq!(calls_to(&e, SWAP_U16).len(), 16);
        let order = called(&e);
        let chunk = order
            .iter()
            .position(|&a| a == TES_FORM_ADD_CHUNK_DATA)
            .unwrap();
        assert!(order[..chunk].contains(&SWAP_U32) && order[chunk..].contains(&SWAP_U32));
    }

    #[test]
    fn byte_swap_of_the_actor_data_uses_32_bit_swaps_for_the_flags_and_karma() {
        let mut e = engine();
        e.register(SWAP_U32, |_, _| Ret::default());
        e.register(SWAP_U16, |_, _| Ret::default());
        log(&mut e);
        e.call(0x0047cb90, &args![0x5000u32]);
        let swaps: Vec<(u32, u32)> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .skip(1)
            .map(|c| (c.0, c.1[0]))
            .collect();
        assert_eq!(
            swaps,
            vec![
                (SWAP_U32, 0x5000),
                (SWAP_U16, 0x5004),
                (SWAP_U16, 0x5006),
                (SWAP_U16, 0x5008),
                (SWAP_U16, 0x500a),
                (SWAP_U16, 0x500c),
                (SWAP_U16, 0x500e),
                (SWAP_U32, 0x5010),
                (SWAP_U16, 0x5014),
                (SWAP_U16, 0x5016),
            ]
        );
    }

    #[test]
    fn template_flag_15_getter_and_setter_work_on_bit_15() {
        let mut e = engine();
        let this = component(&mut e);
        assert!(!e.call(0x0047cca0, &args![this]).bool());
        e.call(0x0047ccc0, &args![this, 1u32]);
        assert_eq!(e.get(this, TESActorBaseData::sTemplateUseFlags), 0x8000);
        assert!(e.call(0x0047cca0, &args![this]).bool());
        e.call(0x0047ccc0, &args![this, 0u32]);
        assert_eq!(e.get(this, TESActorBaseData::sTemplateUseFlags), 0);
    }

    #[test]
    fn template_flag_setter_sets_clears_and_ignores_out_of_range_bits() {
        let mut e = engine();
        let this = component(&mut e);
        e.call(0x0047cce0, &args![this, 3i32, 1u32]);
        e.call(0x0047cce0, &args![this, 0i32, 5u32]);
        assert_eq!(e.get(this, TESActorBaseData::sTemplateUseFlags), 0x0009);
        e.call(0x0047cce0, &args![this, 3i32, 0u32]);
        assert_eq!(e.get(this, TESActorBaseData::sTemplateUseFlags), 0x0001);
        e.call(0x0047cce0, &args![this, 16i32, 1u32]);
        e.call(0x0047cce0, &args![this, -1i32, 1u32]);
        assert_eq!(e.get(this, TESActorBaseData::sTemplateUseFlags), 0x0001);
    }

    #[test]
    fn template_flag_getter_reports_bits_and_is_true_out_of_range() {
        let mut e = engine();
        let this = component(&mut e);
        e.set(this, TESActorBaseData::sTemplateUseFlags, 0x0020);
        assert!(e.call(0x0047cd40, &args![this, 5i32]).bool());
        assert!(!e.call(0x0047cd40, &args![this, 4i32]).bool());
        assert!(e.call(0x0047cd40, &args![this, 16i32]).bool());
        assert!(e.call(0x0047cd40, &args![this, -2i32]).bool());
    }

    #[test]
    fn template_flags_are_replaced_as_a_whole() {
        let mut e = engine();
        let this = component(&mut e);
        e.call(0x0047cd90, &args![this, 0xabcdu32]);
        assert_eq!(e.get(this, TESActorBaseData::sTemplateUseFlags), 0xabcd);
    }

    fn template_with_type(e: &mut Engine, this: Ptr<TESActorBaseData>, form_type: u8) -> u32 {
        let template = e.mem.alloc(0x40);
        e.mem.set_u8(template + 4, form_type);
        e.set(this, TESActorBaseData::pTemplateForm, Ptr::new(template));
        template
    }

    #[test]
    fn template_form_type_test_accepts_types_2c_and_2d() {
        let mut e = engine();
        let this = component(&mut e);
        assert!(!e.call(0x0047cdd0, &args![this]).bool());
        for (form_type, expected) in [(0x2b, false), (0x2c, true), (0x2d, true), (0x2e, false)] {
            template_with_type(&mut e, this, form_type);
            assert_eq!(e.call(0x0047cdd0, &args![this]).bool(), expected);
        }
    }

    #[test]
    fn template_form_type_test_wrapper_forwards() {
        let mut e = engine();
        let this = component(&mut e);
        template_with_type(&mut e, this, 0x2c);
        assert!(e.call(0x0047cdb0, &args![this]).bool());
        template_with_type(&mut e, this, 0x30);
        assert!(!e.call(0x0047cdb0, &args![this]).bool());
    }

    /// Doubles that do nothing and return zero.
    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    const NOT_CASTABLE: u32 = 0x0bad_0000;
    const COMPONENT_VTABLE: u32 = 0x0320_0000;

    /// `__RTDynamicCast` double: the object itself, except `NOT_CASTABLE`
    /// and null, which do not cast.
    fn cast_double(e: &mut Engine) {
        e.register(DYNAMIC_CAST, |_, a| {
            ret(if a[0] == NOT_CASTABLE { 0 } else { a[0] })
        });
    }

    /// The accessors the component's vtable has at `+0x60` (fatigue),
    /// `+0x64` (karma) and `+0x68` (voice type), reading the fields.
    fn install_component_vtable(e: &mut Engine) {
        e.register(0x0330_0060, |e, a| ret(e.mem.u16(a[0] + 8) as u32));
        e.register(0x0330_0064, |e, a| ret_float(e.mem.f32(a[0] + 0x14) as f64));
        e.register(0x0330_0068, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        vtable(
            e,
            COMPONENT_VTABLE,
            &[
                (0x60, 0x0330_0060),
                (0x64, 0x0330_0064),
                (0x68, 0x0330_0068),
            ],
        );
    }

    /// The small getters and setters of other units that the component
    /// functions call.
    fn install_field_helpers(e: &mut Engine) {
        e.register(ADDRESS_PLUS_4, |_, a| ret(a[0] + 4));
        e.register(TEMPLATE_FORM_GETTER, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        e.register(WORD_AT_1C, |e, a| ret(e.mem.u32(a[0] + 0x1c)));
        e.register(STORE_DEATH_ITEM, |e, a| {
            e.mem.set_u32(a[0] + 0x1c, a[1]);
            Ret::default()
        });
        e.register(STORE_VOICE_TYPE, |e, a| {
            e.mem.set_u32(a[0] + 0x20, a[1]);
            Ret::default()
        });
        e.register(STORE_TEMPLATE_FORM, |e, a| {
            e.mem.set_u32(a[0] + 0x24, a[1]);
            Ret::default()
        });
    }

    #[test]
    fn template_application_runs_the_whole_sequence_for_an_eligible_actor() {
        let mut e = engine();
        install_field_helpers(&mut e);
        let this = component(&mut e);
        let template = template_with_type(&mut e, this, 0x2d);
        let actor = e.mem.alloc(0x100);
        let saved_form = e.mem.alloc(0x40);
        e.mem.set_u32(actor + 0x20, saved_form);
        let new_form = e.mem.alloc(0x40);
        let handler = e.mem.alloc(0x10);
        e.mem.set_u32(DATA_HANDLER_GLOBAL, handler);
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry + 4, 0x77);
        for (slot, target, value) in [
            (0xfcu32, 0x0340_00fcu32, 1u32),
            (0x100, 0x0340_0100, 1),
            (0x244, 0x0340_0244, 0),
            (0x1cc, 0x0340_01cc, 0),
        ] {
            e.register_double(target, move |_, _| ret(value));
            vtable(&mut e, 0x0340_1000, &[(slot, target)]);
        }
        e.mem.set_u32(actor, 0x0340_1000);
        quiet(
            &mut e,
            &[
                TES_CONTAINER_CONSTRUCT,
                TES_CONTAINER_DESTRUCT,
                EXTRA_LIST_CLEAR_A,
                EXTRA_LIST_CLEAR_B,
                PATH_MANAGER_CLEAR_INVENTORY_POINTERS,
                PATH_MANAGER_CONSTRUCT,
                ATEXIT,
                CLEAR_ACTOR_PROCESS_FLAGS,
                APPLY_TEMPLATE_FORM,
                SET_OBJECT_REFERENCE,
                INIT_SCRIPT,
                INVENTORY_CHANGES_STEP_A,
                INVENTORY_CHANGES_STEP_B,
                INVENTORY_CHANGES_STEP_C,
            ],
        );
        e.register(ACTOR_TEMPLATE_BLOCKED, |_, _| ret(0));
        e.register(TLS_FLAG_FOUR, |_, _| ret(0));
        e.register(WORD_AT_68, |_, _| ret(1));
        e.register(WORD_AT_0, |_, _| ret(1));
        e.register(ADDRESS_PLUS_44, |_, a| ret(a[0] + 0x44));
        e.register(GET_CONTAINER_CHANGES, |_, _| ret(0xcc));
        e.register(GET_INVENTORY_CHANGES, |_, _| ret(0x1c1c));
        e.register(LIST_IS_EMPTY_CHECK, |_, _| ret(1));
        e.register_double(BUILD_TEMPLATE_CONTAINER, move |e, a| {
            // The container's list head (at +4) gets one entry.
            e.mem.set_u32(a[2] + 4, entry);
            Ret::default()
        });
        e.register(TLS_FLAG_ONE_SET, |_, _| ret(0));
        e.register(FORM_TYPE_GETTER, |e, a| ret(e.mem.u8(a[0] + 4) as u32));
        e.register_double(CREATE_FORM_OF_TYPE, move |_, _| ret(new_form));
        let copy_target = 0x0350_0108;
        e.register(copy_target, |_, _| Ret::default());
        vtable(&mut e, 0x0350_1000, &[(0x108, copy_target)]);
        e.mem.set_u32(new_form, 0x0350_1000);
        e.mem.set_u32(new_form + 0x30, 0x0350_2000);
        let from_template = 0x0350_0010;
        e.register(from_template, |_, _| Ret::default());
        vtable(&mut e, 0x0350_2000, &[(0x10, from_template)]);
        e.mem.set_u32(saved_form + 4, 0);
        e.register(WORD_AT_20, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        let _ = template;
        log(&mut e);
        e.call(0x0047ce10, &args![this, actor]);
        let order = called(&e);
        let after_container: Vec<u32> = order
            .iter()
            .copied()
            .skip_while(|&a| a != TES_CONTAINER_CONSTRUCT)
            .collect();
        assert_eq!(
            after_container,
            vec![
                TES_CONTAINER_CONSTRUCT,
                BUILD_TEMPLATE_CONTAINER,
                ADDRESS_PLUS_4,
                0x0068_15c0,
                TLS_FLAG_FOUR,
                0x0340_00fc,
                WORD_AT_68,
                0x0340_0244,
                0x0340_01cc,
                PATH_MANAGER_CONSTRUCT,
                ATEXIT,
                WORD_AT_0,
                ADDRESS_PLUS_44,
                GET_CONTAINER_CHANGES,
                PATH_MANAGER_CLEAR_INVENTORY_POINTERS,
                ADDRESS_PLUS_44,
                EXTRA_LIST_CLEAR_A,
                ADDRESS_PLUS_44,
                EXTRA_LIST_CLEAR_B,
                0x0340_0100,
                CLEAR_ACTOR_PROCESS_FLAGS,
                WORD_AT_20,
                TLS_FLAG_ONE_SET,
                FORM_TYPE_GETTER,
                CREATE_FORM_OF_TYPE,
                copy_target,
                from_template,
                TLS_FLAG_ONE_SET,
                APPLY_TEMPLATE_FORM,
                SET_OBJECT_REFERENCE,
                INIT_SCRIPT,
                GET_INVENTORY_CHANGES,
                INVENTORY_CHANGES_STEP_A,
                INVENTORY_CHANGES_STEP_B,
                INVENTORY_CHANGES_STEP_C,
                LIST_IS_EMPTY_CHECK,
                ADDRESS_PLUS_44,
                EXTRA_LIST_CLEAR_A,
                TES_CONTAINER_DESTRUCT,
            ]
        );
        // The first container entry's second word is the template argument.
        assert_eq!(
            calls_to(&e, APPLY_TEMPLATE_FORM),
            vec![vec![actor, saved_form, 0x77]]
        );
        assert_eq!(
            calls_to(&e, SET_OBJECT_REFERENCE),
            vec![vec![actor, new_form]]
        );
        assert_eq!(
            calls_to(&e, from_template),
            vec![vec![new_form + 0x30, 0x77]]
        );
    }

    #[test]
    fn template_application_stops_early_without_a_usable_template_or_actor() {
        let mut e = engine();
        install_field_helpers(&mut e);
        e.register(ACTOR_TEMPLATE_BLOCKED, |_, _| ret(0));
        let this = component(&mut e);
        let actor = e.mem.alloc(0x100);
        log(&mut e);
        // No actor.
        e.call(0x0047ce10, &args![this, 0u32]);
        // No template form.
        e.call(0x0047ce10, &args![this, actor]);
        assert!(!called(&e).contains(&TES_CONTAINER_CONSTRUCT));
        // A template of another type.
        template_with_type(&mut e, this, 0x2b);
        e.call(0x0047ce10, &args![this, actor]);
        assert!(!called(&e).contains(&TES_CONTAINER_CONSTRUCT));
        // A usable template, but the actor is blocked.
        template_with_type(&mut e, this, 0x2c);
        e.register(ACTOR_TEMPLATE_BLOCKED, |_, _| ret(1));
        e.call(0x0047ce10, &args![this, actor]);
        assert!(!called(&e).contains(&TES_CONTAINER_CONSTRUCT));
        // A usable template of type 0x2c whose template has no component
        // (the template pointer is null at the second read).
        e.register(ACTOR_TEMPLATE_BLOCKED, |_, _| ret(0));
        let mut calls = 0;
        e.register_double(TEMPLATE_FORM_GETTER, move |e, a| {
            calls += 1;
            // The first read finds the template, the later reads null.
            ret(if calls <= 2 {
                e.mem.u32(a[0] + 0x24)
            } else {
                0
            })
        });
        e.call(0x0047ce10, &args![this, actor]);
        assert!(!called(&e).contains(&TES_CONTAINER_CONSTRUCT));
    }

    #[test]
    fn path_manager_instance_is_constructed_once_with_its_atexit() {
        let mut e = engine();
        quiet(&mut e, &[PATH_MANAGER_CONSTRUCT, ATEXIT]);
        e.set_global(PATH_MANAGER_GUARD, 0x10u32);
        log(&mut e);
        assert_eq!(e.call(0x0047d0b0, &[]).u32(), PATH_MANAGER_OBJECT);
        assert_eq!(e.global::<u32>(PATH_MANAGER_GUARD), 0x11);
        assert_eq!(called(&e), vec![0x0047d0b0, PATH_MANAGER_CONSTRUCT, ATEXIT]);
        assert_eq!(
            calls_to(&e, PATH_MANAGER_CONSTRUCT),
            vec![vec![PATH_MANAGER_OBJECT]]
        );
        assert_eq!(calls_to(&e, ATEXIT), vec![vec![PATH_MANAGER_ATEXIT]]);
        log(&mut e);
        assert_eq!(e.call(0x0047d0b0, &[]).u32(), PATH_MANAGER_OBJECT);
        assert_eq!(called(&e), vec![0x0047d0b0]);
    }

    #[test]
    fn template_actor_base_creation_copies_the_form_and_restores_the_flag() {
        let mut e = engine();
        let handler = e.mem.alloc(0x10);
        e.mem.set_u32(DATA_HANDLER_GLOBAL, handler);
        let source = e.mem.alloc(0x40);
        e.mem.set_u8(source + 4, 0x2a);
        let template = 0x4321u32;
        let new_form = e.mem.alloc(0x80);
        e.register(TLS_FLAG_ONE_SET, |_, a| ret(if a[1] == 0 { 1 } else { 0 }));
        e.register_double(CREATE_FORM_OF_TYPE, move |_, _| ret(new_form));
        let copy = 0x0360_0108;
        let from_template = 0x0360_0010;
        e.register(copy, |_, _| Ret::default());
        e.register(from_template, |_, _| Ret::default());
        vtable(&mut e, 0x0360_1000, &[(0x108, copy)]);
        vtable(&mut e, 0x0360_2000, &[(0x10, from_template)]);
        e.mem.set_u32(new_form, 0x0360_1000);
        e.mem.set_u32(new_form + 0x30, 0x0360_2000);
        log(&mut e);
        let result = e.call(0x0047d130, &args![source, template]).u32();
        assert_eq!(result, new_form);
        assert_eq!(
            called(&e),
            vec![
                0x0047d130,
                TLS_FLAG_ONE_SET,
                FORM_TYPE_GETTER,
                CREATE_FORM_OF_TYPE,
                copy,
                from_template,
                TLS_FLAG_ONE_SET
            ]
        );
        assert_eq!(calls_to(&e, CREATE_FORM_OF_TYPE), vec![vec![0x2a]]);
        assert_eq!(calls_to(&e, copy), vec![vec![new_form, source]]);
        assert_eq!(
            calls_to(&e, from_template),
            vec![vec![new_form + 0x30, template]]
        );
        // The flag set at the start (returned 1) is passed back at the end.
        let flag_calls = calls_to(&e, TLS_FLAG_ONE_SET);
        assert_eq!(flag_calls[0], vec![handler, 0]);
        assert_eq!(flag_calls[1], vec![handler, 1]);
    }

    #[test]
    fn template_chain_walk_combines_masks_until_a_form_says_no() {
        let mut e = engine();
        e.register(TEMPLATE_FORM_GETTER, |e, a| ret(e.mem.u32(a[0] + 0x24)));
        let mask = e.mem.alloc(2);
        let forms: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x80)).collect();
        // The template of a form's component (+0x30) is at form + 0x54.
        e.mem.set_u32(forms[0] + 0x30 + 0x24, forms[1]);
        e.mem.set_u32(forms[1] + 0x30 + 0x24, forms[2]);
        e.mem.set_u16(forms[0] + 0x4a, 0xfff0);
        e.mem.set_u16(forms[1] + 0x4a, 0x0ff3);
        e.mem.set_u16(forms[2] + 0x4a, 0x0000);
        for (i, answer) in [1u32, 1, 0].into_iter().enumerate() {
            let target = 0x0370_00f8 + i as u32 * 0x1000;
            e.register_double(target, move |_, _| ret(answer));
            vtable(&mut e, 0x0370_1000 + i as u32 * 0x1000, &[(0xf8, target)]);
            e.mem.set_u32(forms[i], 0x0370_1000 + i as u32 * 0x1000);
        }
        let found = e.call(0x0047d1a0, &args![forms[0], mask]).u32();
        assert_eq!(found, forms[2]);
        assert_eq!(e.mem.u16(mask), 0xfff0 & 0x0ff3);
        // A chain that ends: null result, mask of everything seen.
        e.mem.set_u32(forms[1] + 0x30 + 0x24, 0);
        let found = e.call(0x0047d1a0, &args![forms[0], mask]).u32();
        assert_eq!(found, 0);
        assert_eq!(e.mem.u16(mask), 0xfff0 & 0x0ff3);
        // A null start gives null and the full mask.
        let found = e.call(0x0047d1a0, &args![0u32, mask]).u32();
        assert_eq!(found, 0);
        assert_eq!(e.mem.u16(mask), 0xffff);
    }

    fn fill_actor_data(e: &mut Engine, this: Ptr<TESActorBaseData>, seed: u32) {
        e.mem.set_u32(this.addr(), COMPONENT_VTABLE);
        e.set(this, TESActorBaseData::iActorBaseFlags, 0x10 + seed);
        e.set(this, TESActorBaseData::usFatigue, 100 + seed as u16);
        e.set(this, TESActorBaseData::usBartergold, 200 + seed as u16);
        e.set(this, TESActorBaseData::sLevel, 5 + seed as u16);
        e.set(this, TESActorBaseData::usCalcLevelMin, 2 + seed as u16);
        e.set(this, TESActorBaseData::usCalcLevelMax, 9 + seed as u16);
        e.set(this, TESActorBaseData::usSpeedMult, 90 + seed as u16);
        e.set(this, TESActorBaseData::fKarma, 25.0 + seed as f32);
        e.set(this, TESActorBaseData::usBaseDisposition, 35 + seed as u16);
        e.set(this, TESActorBaseData::sTemplateUseFlags, 0x0003);
        e.set(this, TESActorBaseData::pDeathItem, Ptr::new(0x1111));
        e.set(this, TESActorBaseData::pVoiceType, Ptr::new(0x2222));
        e.set(this, TESActorBaseData::pTemplateForm, Ptr::new(0x3333));
    }

    #[test]
    fn copy_component_copies_every_field_through_the_marking_setters() {
        let mut e = engine();
        install_field_helpers(&mut e);
        install_component_vtable(&mut e);
        cast_double(&mut e);
        e.mem.set_f32(KARMA_MAX_SETTING + 4, 1000.0);
        e.mem.set_f32(KARMA_MIN_SETTING + 4, -1000.0);
        let (this, seen) = component_in_form(&mut e, 1);
        let source = component(&mut e);
        fill_actor_data(&mut e, source, 7);
        put_faction_list(&mut e, source, &[(0x100, 1), (0x200, 4)]);
        put_faction_list(&mut e, this, &[(0x900, 9)]);
        e.call(0x0047d220, &args![this, source]);
        assert_eq!(e.get(this, TESActorBaseData::iActorBaseFlags), 0x17);
        assert_eq!(e.get(this, TESActorBaseData::usFatigue), 107);
        assert_eq!(e.get(this, TESActorBaseData::fKarma), 32.0);
        assert_eq!(e.get(this, TESActorBaseData::usBartergold), 207);
        assert_eq!(e.get(this, TESActorBaseData::sLevel), 12);
        assert_eq!(e.get(this, TESActorBaseData::usCalcLevelMin), 9);
        assert_eq!(e.get(this, TESActorBaseData::usCalcLevelMax), 16);
        assert_eq!(e.get(this, TESActorBaseData::usSpeedMult), 97);
        assert_eq!(e.get(this, TESActorBaseData::usBaseDisposition), 42);
        assert_eq!(e.get(this, TESActorBaseData::sTemplateUseFlags), 0x0003);
        assert_eq!(e.get(this, TESActorBaseData::pDeathItem).addr(), 0x1111);
        assert_eq!(e.get(this, TESActorBaseData::pVoiceType).addr(), 0x2222);
        assert_eq!(e.get(this, TESActorBaseData::pTemplateForm).addr(), 0x3333);
        let copied: Vec<(u32, u8)> = list_items(&e, this.addr() + 0x2c)
            .iter()
            .map(|&entry| (e.mem.u32(entry), e.mem.u8(entry + 4)))
            .collect();
        assert_eq!(copied, vec![(0x100, 1), (0x200, 4)]);
        // Nine setters marked the form changed (flag word 2, slot 0x48).
        assert_eq!(*seen.borrow(), vec![vec![0x48, 2]; 9]);
    }

    #[test]
    fn copy_component_ignores_an_object_that_does_not_cast() {
        let mut e = engine();
        install_field_helpers(&mut e);
        cast_double(&mut e);
        let (this, seen) = component_in_form(&mut e, 2);
        e.set(this, TESActorBaseData::usFatigue, 11);
        e.call(0x0047d220, &args![this, NOT_CASTABLE]);
        assert_eq!(e.get(this, TESActorBaseData::usFatigue), 11);
        assert!(seen.borrow().is_empty());
    }

    #[test]
    fn plain_field_getters_return_their_words() {
        let mut e = engine();
        let this = component(&mut e);
        e.set(this, TESActorBaseData::sLevel, 0x0102);
        e.set(this, TESActorBaseData::usCalcLevelMin, 0x0203);
        e.set(this, TESActorBaseData::usCalcLevelMax, 0x0304);
        e.set(this, TESActorBaseData::usBaseDisposition, 0x0405);
        e.set(this, TESActorBaseData::usBartergold, 0x0506);
        assert_eq!(e.call(0x0047d370, &args![this]).u16(), 0x0102);
        assert_eq!(e.call(0x0047d390, &args![this]).u16(), 0x0203);
        assert_eq!(e.call(0x0047d3b0, &args![this]).u16(), 0x0304);
        assert_eq!(e.call(0x0047d3d0, &args![this]).u16(), 0x0405);
        assert_eq!(e.call(0x0047d3f0, &args![this]).u16(), 0x0506);
    }

    fn compare_pair(e: &mut Engine) -> (Ptr<TESActorBaseData>, Ptr<TESActorBaseData>) {
        install_field_helpers(e);
        install_component_vtable(e);
        cast_double(e);
        e.register(SPEED_MULT_GETTER, |e, a| ret(e.mem.u16(a[0] + 0x12) as u32));
        let a = component(e);
        let b = component(e);
        fill_actor_data(e, a, 0);
        fill_actor_data(e, b, 0);
        put_faction_list(e, a, &[(0x100, 1), (0x200, 4)]);
        put_faction_list(e, b, &[(0x200, 4), (0x100, 1)]);
        (a, b)
    }

    #[test]
    fn compare_component_reports_equal_components_as_not_different() {
        let mut e = engine();
        let (a, b) = compare_pair(&mut e);
        assert!(!e.call(0x0047d410, &args![a, b]).bool());
    }

    #[test]
    fn compare_component_reports_each_kind_of_difference() {
        let mut e = engine();
        let (a, _) = compare_pair(&mut e);
        assert!(e.call(0x0047d410, &args![a, NOT_CASTABLE]).bool());
        type Change = fn(&mut Engine, Ptr<TESActorBaseData>);
        let changes: Vec<(&str, Change)> = vec![
            ("flags", |e, p| {
                e.set(p, TESActorBaseData::iActorBaseFlags, 0x99)
            }),
            ("fatigue", |e, p| e.set(p, TESActorBaseData::usFatigue, 1)),
            ("karma", |e, p| e.set(p, TESActorBaseData::fKarma, -1.0)),
            ("karma nan", |e, p| {
                e.set(p, TESActorBaseData::fKarma, f32::NAN)
            }),
            ("bartergold", |e, p| {
                e.set(p, TESActorBaseData::usBartergold, 1)
            }),
            ("level", |e, p| e.set(p, TESActorBaseData::sLevel, 77)),
            ("calc min", |e, p| {
                e.set(p, TESActorBaseData::usCalcLevelMin, 77)
            }),
            ("calc max", |e, p| {
                e.set(p, TESActorBaseData::usCalcLevelMax, 77)
            }),
            ("death item", |e, p| {
                e.set(p, TESActorBaseData::pDeathItem, Ptr::new(0x5555))
            }),
            ("speed", |e, p| e.set(p, TESActorBaseData::usSpeedMult, 1)),
            ("disposition", |e, p| {
                e.set(p, TESActorBaseData::usBaseDisposition, 1)
            }),
            ("voice type", |e, p| {
                e.set(p, TESActorBaseData::pVoiceType, Ptr::new(0x6666))
            }),
            ("template", |e, p| {
                e.set(p, TESActorBaseData::pTemplateForm, Ptr::new(0x7777))
            }),
            ("template flags", |e, p| {
                e.set(p, TESActorBaseData::sTemplateUseFlags, 0x0100)
            }),
            ("faction rank", |e, p| {
                let first = e.mem.u32(p.addr() + 0x2c);
                e.mem.set_u8(first + 4, 3);
            }),
            ("faction", |e, p| {
                let first = e.mem.u32(p.addr() + 0x2c);
                e.mem.set_u32(first, 0x300);
            }),
            ("faction count", |e, p| {
                let head = p.addr() + 0x2c;
                let node = e.mem.alloc(8);
                let entry = faction_rank(e, 0x400, 1);
                e.mem.set_u32(node, entry);
                let next = e.mem.u32(head + 4);
                e.mem.set_u32(node + 4, next);
                e.mem.set_u32(head + 4, node);
            }),
        ];
        for (name, change) in changes {
            let mut e = engine();
            let (a, b) = compare_pair(&mut e);
            change(&mut e, b);
            assert!(e.call(0x0047d410, &args![a, b]).bool(), "{name}");
        }
    }

    #[test]
    fn faction_rank_lookup_finds_the_rank_or_minus_one() {
        let mut e = engine();
        let this = component(&mut e);
        put_faction_list(&mut e, this, &[(0x100, 3), (0x200, 0xfe)]);
        assert_eq!(e.call(0x0047d680, &args![this, 0x100u32, 0u32]).i32(), 3);
        assert_eq!(e.call(0x0047d680, &args![this, 0x200u32, 0u32]).i32(), -2);
        assert_eq!(e.call(0x0047d680, &args![this, 0x300u32, 0u32]).i32(), -1);
        // A null item ends the search.
        let items = list_items(&e, this.addr() + 0x2c);
        let mut with_gap = items.clone();
        with_gap.insert(1, 0);
        let other = component(&mut e);
        make_list(&mut e, other.addr() + 0x2c, &with_gap);
        assert_eq!(e.call(0x0047d680, &args![other, 0x200u32, 0u32]).i32(), -1);
        assert_eq!(e.call(0x0047d680, &args![other, 0x100u32, 0u32]).i32(), 3);
    }

    #[test]
    fn faction_rank_lookup_can_skip_factions_with_flag_bit_8() {
        let mut e = engine();
        let this = component(&mut e);
        let plain = e.mem.alloc(0x40);
        let flagged = e.mem.alloc(0x40);
        e.mem.set_u32(flagged + 0x34, 0x8);
        put_faction_list(&mut e, this, &[(flagged, 5), (plain, 6)]);
        assert_eq!(e.call(0x0047d680, &args![this, flagged, 0u32]).i32(), 5);
        assert_eq!(e.call(0x0047d680, &args![this, flagged, 1u32]).i32(), -1);
        assert_eq!(e.call(0x0047d680, &args![this, plain, 1u32]).i32(), 6);
    }

    #[test]
    fn faction_flag_bit_3_test_reads_the_word_at_0x34() {
        let mut e = engine();
        let faction = e.mem.alloc(0x40);
        assert!(!e.call(0x0047d720, &args![faction]).bool());
        e.mem.set_u32(faction + 0x34, 0x8);
        assert!(e.call(0x0047d720, &args![faction]).bool());
        e.mem.set_u32(faction + 0x34, 0x6);
        assert!(!e.call(0x0047d720, &args![faction]).bool());
    }

    #[test]
    fn faction_flag_bit_1_test_reads_the_word_at_0x34() {
        let mut e = engine();
        let faction = e.mem.alloc(0x40);
        assert!(!e.call(0x0047d7c0, &args![faction]).bool());
        e.mem.set_u32(faction + 0x34, 0x2);
        assert!(e.call(0x0047d7c0, &args![faction]).bool());
        e.mem.set_u32(faction + 0x34, 0xd);
        assert!(!e.call(0x0047d7c0, &args![faction]).bool());
    }

    #[test]
    fn faction_flag_bit_2_test_reads_the_word_at_0x34() {
        let mut e = engine();
        let faction = e.mem.alloc(0x40);
        assert!(!e.call(0x0047d7e0, &args![faction]).bool());
        e.mem.set_u32(faction + 0x34, 0x4);
        assert!(e.call(0x0047d7e0, &args![faction]).bool());
        e.mem.set_u32(faction + 0x34, 0xb);
        assert!(!e.call(0x0047d7e0, &args![faction]).bool());
    }

    #[test]
    fn evil_factions_only_needs_a_non_empty_list_of_flagged_factions() {
        let mut e = engine();
        let this = component(&mut e);
        // Empty list.
        assert!(!e.call(0x0047d740, &args![this]).bool());
        let evil_a = e.mem.alloc(0x40);
        let evil_b = e.mem.alloc(0x40);
        let neutral = e.mem.alloc(0x40);
        e.mem.set_u32(evil_a + 0x34, 2);
        e.mem.set_u32(evil_b + 0x34, 3);
        put_faction_list(&mut e, this, &[(evil_a, 0), (evil_b, 1)]);
        assert!(e.call(0x0047d740, &args![this]).bool());
        let other = component(&mut e);
        put_faction_list(&mut e, other, &[(evil_a, 0), (neutral, 1)]);
        assert!(!e.call(0x0047d740, &args![other]).bool());
        // A null entry is skipped.
        let third = component(&mut e);
        let first = faction_rank(&mut e, evil_a, 0);
        make_list(&mut e, third.addr() + 0x2c, &[first, 0]);
        assert!(e.call(0x0047d740, &args![third]).bool());
    }

    #[test]
    fn set_faction_rank_updates_removes_and_adds_entries() {
        let mut e = engine();
        let this = component(&mut e);
        // Null faction: nothing.
        e.call(0x0047d800, &args![this, 0u32, 3i8]);
        assert_eq!(list_items(&e, this.addr() + 0x2c), vec![0]);
        // Adding to an empty list puts the entry at the head.
        e.call(0x0047d800, &args![this, 0x100u32, 3i8]);
        let items = list_items(&e, this.addr() + 0x2c);
        assert_eq!(items.len(), 1);
        assert_eq!((e.mem.u32(items[0]), e.mem.i8(items[0] + 4)), (0x100, 3));
        // An existing faction gets the new rank.
        e.call(0x0047d800, &args![this, 0x100u32, 7i8]);
        assert_eq!(e.mem.i8(items[0] + 4), 7);
        // Another faction is added (the list takes it at the head).
        e.call(0x0047d800, &args![this, 0x200u32, 2i8]);
        let items = list_items(&e, this.addr() + 0x2c);
        assert_eq!(items.len(), 2);
        let pairs: Vec<(u32, i8)> = items
            .iter()
            .map(|&entry| (e.mem.u32(entry), e.mem.i8(entry + 4)))
            .collect();
        assert_eq!(pairs, vec![(0x200, 2), (0x100, 7)]);
        // Rank -1 removes the faction's node.
        e.call(0x0047d800, &args![this, 0x200u32, -1i8]);
        let items = list_items(&e, this.addr() + 0x2c);
        assert_eq!(items.len(), 1);
        assert_eq!(e.mem.u32(items[0]), 0x100);
        // Rank -1 for an unknown faction adds it with rank -1.
        e.call(0x0047d800, &args![this, 0x400u32, -1i8]);
        assert_eq!(list_items(&e, this.addr() + 0x2c).len(), 2);
    }

    /// Doubles for the form lookup calls of `InitItem`: the file is `0x3000`,
    /// the compile-index remap adds `0x0100_0000` to a non-zero ID and the
    /// lookup finds the IDs in `known` (the form is the ID itself plus
    /// `0x1000_0000`). The returned log has every message logged.
    fn init_item_doubles(e: &mut Engine, known: &'static [u32]) -> Seen {
        cast_double(e);
        e.register(TES_FORM_GET_FILE, |_, _| ret(0x3000));
        e.register(TES_FORM_ADD_COMPILE_INDEX, |e, a| {
            let id = e.mem.u32(a[0]);
            e.mem.set_u32(a[0], id + 0x0100_0000);
            Ret::default()
        });
        e.register_double(LOOKUP_FORM_BY_ID, move |_, a| {
            ret(if known.contains(&a[0]) {
                a[0] + 0x1000_0000
            } else {
                0
            })
        });
        let messages: Seen = Rc::default();
        let sink = messages.clone();
        e.register_double(MESSAGE_LOG, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.register(FORM_ID_GETTER, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        let name_target = 0x0380_0130;
        e.register(name_target, |_, _| ret(0x4e41_4d45));
        vtable(e, 0x0380_1000, &[(0x130, name_target)]);
        messages
    }

    fn form_with_vtable(e: &mut Engine) -> Ptr {
        let form = e.mem.alloc(0x40);
        e.mem.set_u32(form, 0x0380_1000);
        e.mem.set_u32(form + 0xc, 0x0ace_0001);
        Ptr::new(form)
    }

    #[test]
    fn init_item_resolves_the_stored_ids_and_drops_unknown_factions() {
        let mut e = engine();
        let messages = init_item_doubles(
            &mut e,
            &[
                0x0100_0100,
                0x0100_0300,
                0x0100_0010,
                0x0100_0020,
                0x0100_0030,
            ],
        );
        e.register(FORM_TYPE_GETTER, |_, _| ret(0));
        let form = form_with_vtable(&mut e);
        let this = component(&mut e);
        // A template with flag 0 set skips the karma repair.
        e.set(this, TESActorBaseData::sTemplateUseFlags, 1);
        put_faction_list(&mut e, this, &[(0x100, 1), (0x200, 2), (0x300, 3)]);
        e.set(this, TESActorBaseData::pVoiceType, Ptr::new(0x10));
        e.set(this, TESActorBaseData::pDeathItem, Ptr::new(0x20));
        e.set(this, TESActorBaseData::pTemplateForm, Ptr::new(0x30));
        log(&mut e);
        e.call(0x0047d8b0, &args![this, form]);
        let entries: Vec<(u32, u8)> = list_items(&e, this.addr() + 0x2c)
            .iter()
            .map(|&entry| (e.mem.u32(entry), e.mem.u8(entry + 4)))
            .collect();
        assert_eq!(entries, vec![(0x1100_0100, 1), (0x1100_0300, 3)]);
        assert_eq!(
            e.get(this, TESActorBaseData::pVoiceType).addr(),
            0x1100_0010
        );
        assert_eq!(
            e.get(this, TESActorBaseData::pDeathItem).addr(),
            0x1100_0020
        );
        assert_eq!(
            e.get(this, TESActorBaseData::pTemplateForm).addr(),
            0x1100_0030
        );
        assert!(messages.borrow().is_empty());
        // The file is the last file of the form; the remap gets it.
        assert_eq!(
            calls_to(&e, TES_FORM_GET_FILE),
            vec![vec![form.addr(), 0xffff_ffff]]
        );
        assert!(calls_to(&e, TES_FORM_ADD_COMPILE_INDEX)
            .iter()
            .all(|c| c[1] == 0x3000));
        // The faction casts target the faction type, the voice type and
        // death item their own, the template none.
        let casts: Vec<u32> = calls_to(&e, DYNAMIC_CAST).iter().map(|c| c[3]).collect();
        assert_eq!(
            casts,
            vec![
                RTTI_FACTION,
                RTTI_FACTION,
                RTTI_FACTION,
                RTTI_VOICE_TYPE,
                RTTI_LEVELED_ITEM
            ]
        );
    }

    #[test]
    fn init_item_logs_a_missing_death_item_and_template_and_clears_them() {
        let mut e = engine();
        let messages = init_item_doubles(&mut e, &[]);
        let form = form_with_vtable(&mut e);
        let this = component(&mut e);
        e.set(this, TESActorBaseData::pDeathItem, Ptr::new(0x20));
        e.set(this, TESActorBaseData::pTemplateForm, Ptr::new(0x30));
        // The template-use flag 0 is set with a template: no karma repair.
        e.set(this, TESActorBaseData::sTemplateUseFlags, 1);
        // A voice type that does not resolve is simply cleared.
        e.set(this, TESActorBaseData::pVoiceType, Ptr::new(0x10));
        e.call(0x0047d8b0, &args![this, form]);
        assert!(e.get(this, TESActorBaseData::pVoiceType).is_null());
        assert!(e.get(this, TESActorBaseData::pDeathItem).is_null());
        assert!(e.get(this, TESActorBaseData::pTemplateForm).is_null());
        // The log gets the remapped ID, the form's ID and its name.
        assert_eq!(
            *messages.borrow(),
            vec![
                vec![
                    TEXT_DEATH_ITEM_MISSING,
                    0x0100_0020,
                    0x0ace_0001,
                    0x4e41_4d45
                ],
                vec![TEXT_TEMPLATE_MISSING, 0x0100_0030, 0x0ace_0001, 0x4e41_4d45],
            ]
        );
    }

    /// The karma settings of a consistent table: the five default values
    /// map back to their own alignment index.
    fn set_alignment_thresholds(e: &mut Engine) {
        e.mem.set_f32(KARMA_THRESHOLD_011C4F64 + 4, -750.0);
        e.mem.set_f32(KARMA_THRESHOLD_011C5144 + 4, -250.0);
        e.mem.set_f32(KARMA_THRESHOLD_011C4E54 + 4, 250.0);
        e.mem.set_f32(KARMA_THRESHOLD_011C5090 + 4, 750.0);
    }

    fn set_default_alignment_values(e: &mut Engine, values: [f32; 5]) {
        for (i, v) in values.iter().enumerate() {
            e.mem.set_f32(DEFAULT_ALIGNMENT_VALUES + 4 * i as u32, *v);
        }
    }

    fn karma_fixture(e: &mut Engine) -> (Ptr<TESActorBaseData>, Ptr) {
        init_item_doubles(e, &[]);
        e.register(FLOATS_NEAR, |_, a| {
            let (x, y, tolerance) = (
                f32::from_bits(a[0]),
                f32::from_bits(a[1]),
                f32::from_bits(a[2]),
            );
            ret(((x - y).abs() <= tolerance) as u32)
        });
        e.mem.set_f32(KARMA_TOLERANCE, 0.001);
        e.mem.set_f64(HALF, 0.5);
        e.mem.set_f64(HUNDRED, 100.0);
        set_alignment_thresholds(e);
        let form = form_with_vtable(e);
        let this = component(e);
        (this, form)
    }

    #[test]
    fn init_item_leaves_the_karma_when_the_alignment_table_is_consistent() {
        let mut e = engine();
        let (this, form) = karma_fixture(&mut e);
        // Defaults that fall into their own alignment: very evil (4), evil
        // (2), neutral (1), good (0), very good (3) indices.
        set_default_alignment_values(&mut e, [500.0, 0.0, -500.0, 900.0, -900.0]);
        e.set_global(ALIGNMENT_TABLE_STATE, -1i32);
        e.set(this, TESActorBaseData::fKarma, 0.0);
        e.call(0x0047d8b0, &args![this, form]);
        assert_eq!(e.global::<i32>(ALIGNMENT_TABLE_STATE), 0);
        assert_eq!(e.get(this, TESActorBaseData::fKarma), 0.0);
    }

    #[test]
    fn init_item_repairs_a_karma_equal_to_a_default_of_an_inconsistent_table() {
        // The defaults do not map to their own index: index 0 value 0.0 is
        // neutral (alignment 1), so the table is inconsistent.
        let defaults = [0.0, 1000.0, -400.0, -1000.0, 1100.0];
        // (karma, expected result)
        let thresholds = (-750.0f32, -250.0f32, 250.0f32, 750.0f32);
        let diff_a4 = thresholds.3 - thresholds.2; // 5090 - 4e54
        let diff_a0 = thresholds.2 - thresholds.1; // 4e54 - 5144
        let diff_9c = thresholds.0 - thresholds.1; // 4f64 - 5144
        let cases = [
            (defaults[1], 250.0 - diff_a0 * 0.5),
            (defaults[0], diff_a4 * 0.5 + 250.0),
            (defaults[2], diff_9c * 0.5 + -250.0),
            (defaults[3], 750.0 + 100.0),
            (defaults[4], -750.0 - 100.0),
        ];
        for (karma, expected) in cases {
            let mut e = engine();
            let (this, form) = karma_fixture(&mut e);
            set_default_alignment_values(&mut e, defaults);
            e.set_global(ALIGNMENT_TABLE_STATE, -1i32);
            e.set(this, TESActorBaseData::fKarma, karma);
            e.call(0x0047d8b0, &args![this, form]);
            assert_eq!(e.global::<i32>(ALIGNMENT_TABLE_STATE), 1);
            assert_eq!(
                e.get(this, TESActorBaseData::fKarma),
                expected,
                "karma {karma}"
            );
            assert_eq!(e.global::<f32>(ALIGN_DIFF_011C52A4), diff_a4);
            assert_eq!(e.global::<f32>(ALIGN_DIFF_011C52A0), diff_a0);
            assert_eq!(e.global::<f32>(ALIGN_DIFF_011C529C), diff_9c);
        }
    }

    #[test]
    fn init_item_logs_an_unfixable_karma_of_an_inconsistent_table() {
        let mut e = engine();
        let (this, form) = karma_fixture(&mut e);
        let messages = init_item_doubles(&mut e, &[]);
        set_default_alignment_values(&mut e, [0.0, 1000.0, -400.0, -1000.0, 1100.0]);
        e.set_global(ALIGNMENT_TABLE_STATE, -1i32);
        e.set(this, TESActorBaseData::fKarma, 33.0);
        e.call(0x0047d8b0, &args![this, form]);
        assert_eq!(e.get(this, TESActorBaseData::fKarma), 33.0);
        assert_eq!(*messages.borrow(), vec![vec![TEXT_INVALID_KARMA]]);
        // With the table already known to be fine nothing is checked; with a
        // template whose flag 0 is clear the repair still runs.
        e.set_global(ALIGNMENT_TABLE_STATE, 0i32);
        e.call(0x0047d8b0, &args![this, form]);
        assert_eq!(messages.borrow().len(), 1);
    }

    #[test]
    fn init_item_without_a_form_uses_file_zero() {
        let mut e = engine();
        let (this, _) = karma_fixture(&mut e);
        e.set_global(ALIGNMENT_TABLE_STATE, 0i32);
        e.set(this, TESActorBaseData::sTemplateUseFlags, 0);
        put_faction_list(&mut e, this, &[(0x100, 1)]);
        log(&mut e);
        e.call(0x0047d8b0, &args![this, 0u32]);
        assert!(calls_to(&e, TES_FORM_GET_FILE).is_empty());
        assert_eq!(calls_to(&e, TES_FORM_ADD_COMPILE_INDEX)[0][1], 0);
    }

    #[test]
    fn base_flags_setter_stores_and_marks_the_form_changed() {
        let mut e = engine();
        let (this, seen) = component_in_form(&mut e, 3);
        e.call(0x0047dd30, &args![this, 0xa5a5u32]);
        assert_eq!(e.get(this, TESActorBaseData::iActorBaseFlags), 0xa5a5);
        assert_eq!(*seen.borrow(), vec![vec![0x48, 2]]);
    }

    fn change_flag_doubles(e: &mut Engine, has_change: bool) -> Seen {
        cast_double(e);
        e.register(CHANGE_FLAGS_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            ret(a[0])
        });
        let seen: Seen = Rc::default();
        let sink = seen.clone();
        e.register_double(GET_CHANGE, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            ret(has_change as u32)
        });
        let handler = e.mem.alloc(0x10);
        e.mem.set_u32(DATA_HANDLER_GLOBAL, handler);
        seen
    }

    #[test]
    fn set_flag_bit_changes_the_flags_and_marks_changes_by_mask() {
        let mut e = engine();
        let (this, seen) = component_in_form(&mut e, 4);
        let changes = change_flag_doubles(&mut e, false);
        // No change in state: nothing is marked.
        e.call(0x0047dd50, &args![this, 0x20u32, 0u32, 1u32]);
        assert!(seen.borrow().is_empty());
        // Another bit set with marking asked for.
        e.call(0x0047dd50, &args![this, 0x20u32, 1u32, 1u32]);
        assert_eq!(e.get(this, TESActorBaseData::iActorBaseFlags), 0x20);
        assert_eq!(*seen.borrow(), vec![vec![0x48, 2]]);
        // Without marking asked for.
        e.call(0x0047dd50, &args![this, 0x40u32, 1u32, 0u32]);
        assert_eq!(e.get(this, TESActorBaseData::iActorBaseFlags), 0x60);
        assert_eq!(seen.borrow().len(), 1);
        // Clearing.
        e.call(0x0047dd50, &args![this, 0x20u32, 0u32, 0u32]);
        assert_eq!(e.get(this, TESActorBaseData::iActorBaseFlags), 0x40);
        assert!(changes.borrow().is_empty());
        // Mask 1 asks the save/load game whether the form already has the
        // change: no (marks with slot 0x48), then yes (slot 0x4c).
        e.call(0x0047dd50, &args![this, 1u32, 1u32, 0u32]);
        assert_eq!(e.get(this, TESActorBaseData::iActorBaseFlags), 0x41);
        assert_eq!(seen.borrow().last().unwrap(), &vec![0x48, 0x0100_0000]);
        // The cast double returns the component itself.
        assert_eq!(
            *changes.borrow(),
            vec![vec![
                e.mem.u32(DATA_HANDLER_GLOBAL),
                this.addr(),
                0x0100_0000
            ]]
        );
    }

    #[test]
    fn set_flag_bit_mask_one_uses_the_clearing_slot_when_the_change_is_known() {
        let mut e = engine();
        let (this, seen) = component_in_form(&mut e, 5);
        change_flag_doubles(&mut e, true);
        e.call(0x0047dd50, &args![this, 1u32, 1u32, 0u32]);
        assert_eq!(*seen.borrow(), vec![vec![0x4c, 0x0100_0000]]);
        assert_eq!(e.get(this, TESActorBaseData::iActorBaseFlags), 1);
        // A component that does not cast to a form marks nothing but still
        // changes the flag.
        let (other, other_seen) = component_in_form(&mut e, 6);
        e.register(DYNAMIC_CAST, |_, _| ret(0));
        e.call(0x0047dd50, &args![other, 1u32, 1u32, 0u32]);
        assert!(other_seen.borrow().is_empty());
        assert_eq!(e.get(other, TESActorBaseData::iActorBaseFlags), 1);
    }

    fn check_marking_setter(address: u32, field: Field<TESActorBaseData, u16>, tag: u32) {
        let mut e = engine();
        let (this, seen) = component_in_form(&mut e, tag);
        e.call(address, &args![this, 0x1234u32]);
        assert_eq!(e.get(this, field), 0x1234);
        assert_eq!(*seen.borrow(), vec![vec![0x48, 2]]);
    }

    #[test]
    fn calc_min_setter_stores_and_marks() {
        check_marking_setter(0x0047de40, TESActorBaseData::usCalcLevelMin, 7);
    }

    #[test]
    fn calc_max_setter_stores_and_marks() {
        check_marking_setter(0x0047de70, TESActorBaseData::usCalcLevelMax, 8);
    }

    #[test]
    fn disposition_setter_stores_and_marks() {
        check_marking_setter(0x0047dea0, TESActorBaseData::usBaseDisposition, 9);
    }

    #[test]
    fn level_setter_stores_and_marks() {
        check_marking_setter(0x0047dfe0, TESActorBaseData::sLevel, 10);
    }

    #[test]
    fn fatigue_setter_stores_and_marks() {
        check_marking_setter(0x0047e010, TESActorBaseData::usFatigue, 11);
    }

    #[test]
    fn bartergold_setter_stores_and_marks() {
        check_marking_setter(0x0047e240, TESActorBaseData::usBartergold, 12);
    }

    #[test]
    fn speed_setter_stores_and_marks() {
        check_marking_setter(0x0047e270, TESActorBaseData::usSpeedMult, 13);
    }

    fn level_fixture(e: &mut Engine) -> (Ptr<TESActorBaseData>, Ptr<TESActorBaseData>) {
        e.register(WORD_AT_20, |e, a| ret(e.mem.u32(a[0] + 0x20)));
        e.mem.set_f64(THOUSAND, 1000.0);
        let this = component(e);
        let player_form = e.mem.alloc(0x30 + 0x34);
        let player_component: Ptr<TESActorBaseData> = Ptr::new(player_form + 0x30);
        let player = e.mem.alloc(0x40);
        e.mem.set_u32(player + 0x20, player_form);
        e.mem.set_u32(LEVEL_SOURCE_OBJECT, player);
        (this, player_component)
    }

    #[test]
    fn level_is_the_stored_level_without_the_multiplier_flag() {
        let mut e = engine();
        let (this, _) = level_fixture(&mut e);
        e.set(this, TESActorBaseData::sLevel, 17);
        e.set(this, TESActorBaseData::usCalcLevelMin, 20);
        assert_eq!(e.call(0x0047ded0, &args![this]).u16(), 17);
    }

    #[test]
    fn level_with_the_multiplier_flag_scales_the_players_level_and_clamps() {
        let mut e = engine();
        let (this, player) = level_fixture(&mut e);
        e.set(this, TESActorBaseData::iActorBaseFlags, 0x80);
        e.set(player, TESActorBaseData::sLevel, 25);
        // 500 / 1000 * 25 = 12.5, truncated.
        e.set(this, TESActorBaseData::sLevel, 500);
        assert_eq!(e.call(0x0047ded0, &args![this]).u16(), 12);
        // Raised to the minimum.
        e.set(this, TESActorBaseData::usCalcLevelMin, 15);
        assert_eq!(e.call(0x0047ded0, &args![this]).u16(), 15);
        // Capped at the maximum.
        e.set(this, TESActorBaseData::usCalcLevelMin, 0);
        e.set(this, TESActorBaseData::usCalcLevelMax, 10);
        assert_eq!(e.call(0x0047ded0, &args![this]).u16(), 10);
        // A level at or below the maximum is kept.
        e.set(this, TESActorBaseData::usCalcLevelMax, 12);
        assert_eq!(e.call(0x0047ded0, &args![this]).u16(), 12);
        e.set(this, TESActorBaseData::usCalcLevelMax, 13);
        assert_eq!(e.call(0x0047ded0, &args![this]).u16(), 12);
    }

    #[test]
    fn level_with_the_multiplier_flag_but_no_player_form_keeps_the_stored_level() {
        let mut e = engine();
        let (this, _) = level_fixture(&mut e);
        e.mem.set_u32(e.mem.u32(LEVEL_SOURCE_OBJECT) + 0x20, 0);
        e.set(this, TESActorBaseData::iActorBaseFlags, 0x80);
        e.set(this, TESActorBaseData::sLevel, 40);
        e.set(this, TESActorBaseData::usCalcLevelMax, 30);
        assert_eq!(e.call(0x0047ded0, &args![this]).u16(), 30);
    }

    #[test]
    fn alignment_for_karma_follows_the_threshold_settings() {
        let mut e = engine();
        set_alignment_thresholds(&mut e);
        for (karma, expected) in [
            (-1000.0f32, 4),
            (-750.0, 4),
            (-500.0, 2),
            (-250.0, 2),
            (-249.0, 1),
            (0.0, 1),
            (249.0, 1),
            (250.0, 0),
            (500.0, 0),
            (749.0, 0),
            (750.0, 3),
            (900.0, 3),
            (f32::NAN, 1),
        ] {
            assert_eq!(
                e.call(0x0047e040, &args![karma]).i32(),
                expected,
                "karma {karma}"
            );
        }
    }

    fn title_fixture(e: &mut Engine, karma: i32, level: u32) -> Ptr {
        e.register_double(ACTOR_LEVEL, move |_, _| ret(level));
        set_alignment_thresholds(e);
        let actor = e.mem.alloc(0x100);
        let target = 0x0390_0008;
        e.register_double(target, move |_, a| {
            assert_eq!(a[1], KARMA_ACTOR_VALUE);
            ret(karma as u32)
        });
        vtable(e, 0x0390_1000, &[(8, target)]);
        e.mem.set_u32(actor + 0xa4, 0x0390_1000);
        Ptr::new(actor)
    }

    fn put_title(e: &mut Engine, row: u32, level_index: u32, text: u32) {
        let setting = e.mem.alloc(8);
        e.mem.set_u32(setting + 4, text);
        e.mem
            .set_u32(KARMIC_TITLE_TABLE + row * 0xc8 + level_index * 4, setting);
    }

    #[test]
    fn karmic_title_picks_the_row_by_alignment_and_the_column_by_level() {
        let mut e = engine();
        e.map(KARMIC_TITLE_TABLE & !0xfff, 0x2000);
        put_title(&mut e, 0, 0, 0xa000);
        put_title(&mut e, 1, 4, 0xa100);
        put_title(&mut e, 2, 49, 0xa200);
        // Neutral karma (alignment 1), level 5: row 1, column 4.
        let actor = title_fixture(&mut e, 0, 5);
        assert_eq!(e.call(0x0047e0e0, &args![actor]).u32(), 0xa100);
        // Good (alignment 0) at level 1; very good (3) shares its row.
        let actor = title_fixture(&mut e, 500, 1);
        assert_eq!(e.call(0x0047e0e0, &args![actor]).u32(), 0xa000);
        let actor = title_fixture(&mut e, 900, 1);
        assert_eq!(e.call(0x0047e0e0, &args![actor]).u32(), 0xa000);
        // Evil (2) and very evil (4) share row 2; levels above 50 and
        // level 0 clamp to the ends.
        let actor = title_fixture(&mut e, -500, 99);
        assert_eq!(e.call(0x0047e0e0, &args![actor]).u32(), 0xa200);
        let actor = title_fixture(&mut e, -900, 50);
        assert_eq!(e.call(0x0047e0e0, &args![actor]).u32(), 0xa200);
        let actor = title_fixture(&mut e, 500, 0);
        assert_eq!(e.call(0x0047e0e0, &args![actor]).u32(), 0xa000);
    }

    #[test]
    fn karmic_title_falls_back_to_the_empty_string() {
        let mut e = engine();
        e.map(KARMIC_TITLE_TABLE & !0xfff, 0x2000);
        // No setting in the slot.
        let actor = title_fixture(&mut e, 0, 5);
        assert_eq!(e.call(0x0047e0e0, &args![actor]).u32(), EMPTY_STRING);
        // A setting whose string is null.
        put_title(&mut e, 1, 4, 0);
        assert_eq!(e.call(0x0047e0e0, &args![actor]).u32(), EMPTY_STRING);
        // No actor.
        assert_eq!(e.call(0x0047e0e0, &args![0u32]).u32(), EMPTY_STRING);
    }

    #[test]
    fn set_karma_clamps_to_the_upper_then_the_lower_setting_and_marks() {
        let mut e = engine();
        e.mem.set_f32(KARMA_MAX_SETTING + 4, 1000.0);
        e.mem.set_f32(KARMA_MIN_SETTING + 4, -1000.0);
        let (this, seen) = component_in_form(&mut e, 14);
        e.call(0x0047e1c0, &args![this, 123.5f32]);
        assert_eq!(e.get(this, TESActorBaseData::fKarma), 123.5);
        e.call(0x0047e1c0, &args![this, 5000.0f32]);
        assert_eq!(e.get(this, TESActorBaseData::fKarma), 1000.0);
        e.call(0x0047e1c0, &args![this, -5000.0f32]);
        assert_eq!(e.get(this, TESActorBaseData::fKarma), -1000.0);
        e.call(0x0047e1c0, &args![this, 1000.0f32]);
        assert_eq!(e.get(this, TESActorBaseData::fKarma), 1000.0);
        e.call(0x0047e1c0, &args![this, f32::NAN]);
        assert!(e.get(this, TESActorBaseData::fKarma).is_nan());
        assert_eq!(*seen.borrow(), vec![vec![0x48, 2]; 5]);
    }

    #[test]
    fn marking_the_form_changed_goes_through_the_owning_forms_slot_48() {
        let mut e = engine();
        let (this, seen) = component_in_form(&mut e, 15);
        e.call(0x0047e2a0, &args![this, 0x40u32]);
        assert_eq!(*seen.borrow(), vec![vec![0x48, 0x40]]);
        // A null component does nothing.
        e.call(0x0047e2a0, &args![0u32, 0x40u32]);
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn clearing_the_form_changed_goes_through_the_owning_forms_slot_4c() {
        let mut e = engine();
        let (this, seen) = component_in_form(&mut e, 16);
        e.call(0x0047e2f0, &args![this, 0x41u32]);
        assert_eq!(*seen.borrow(), vec![vec![0x4c, 0x41]]);
        e.call(0x0047e2f0, &args![0u32, 0x40u32]);
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn change_flags_are_set_and_cleared_by_mask() {
        let mut e = engine();
        let this = component(&mut e);
        e.call(0x0047e340, &args![this, 0x0f0fu32]);
        e.call(0x0047e340, &args![this, 0x1000u32]);
        assert_eq!(e.get(this, TESActorBaseData::iChangeFlags), 0x1f0f);
        e.call(0x0047e360, &args![this, 0x0101u32]);
        assert_eq!(e.get(this, TESActorBaseData::iChangeFlags), 0x1e0e);
    }

    fn save_fixture(e: &mut Engine, blocks: bool, debug: bool) -> (u32, Seen) {
        let save_load = e.mem.alloc(0x100);
        e.mem.set_u32(SAVE_LOAD_GAME_GLOBAL, save_load);
        e.mem.set_u8(SAVE_DEBUG_SWITCH + 4, debug as u8);
        e.register_double(USE_SAVE_GAME_BLOCKS, move |_, _| ret(blocks as u32));
        let logged: Seen = Rc::default();
        let sink = logged.clone();
        e.register_double(ERROR_LOG, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.register(CURRENT_SAVE_ENTRY, |_, _| ret(0));
        (save_load, logged)
    }

    #[test]
    fn save_size_counts_the_block_header_the_data_and_five_bytes_per_faction() {
        let mut e = engine();
        let (_, logged) = save_fixture(&mut e, true, false);
        let this = component(&mut e);
        put_faction_list(&mut e, this, &[(0x100, 1), (0x200, 2), (0x300, 3)]);
        // Block header 6 + 2 for the count + 5 * 3.
        assert_eq!(e.call(0x0047e380, &args![this, 0u32]).u16(), 6 + 2 + 15);
        // With the data flag: + 0x18.
        assert_eq!(
            e.call(0x0047e380, &args![this, 2u32]).u16(),
            6 + 0x18 + 2 + 15
        );
        let (_, _) = save_fixture(&mut e, false, false);
        assert_eq!(e.call(0x0047e380, &args![this, 3u32]).u16(), 0x18 + 2 + 15);
        assert!(logged.borrow().is_empty());
    }

    #[test]
    fn save_size_logs_the_size_with_the_current_entry_when_the_debug_switch_is_on() {
        let mut e = engine();
        let (_, logged) = save_fixture(&mut e, false, true);
        let this = component(&mut e);
        // No current entry: the plain message.
        e.call(0x0047e380, &args![this, 0u32]);
        assert_eq!(
            *logged.borrow(),
            vec![vec![TEXT_SAVE_SIZE_PLAIN, 2, 0xc4e, SOURCE_FILE_PATH]]
        );
        // With an entry the form's ID, name and the entry's flags are named.
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry, 0x0100_0042);
        e.mem.set_u32(entry + 5, 0x9999);
        e.register_double(CURRENT_SAVE_ENTRY, move |_, _| ret(entry));
        e.register(LOOKUP_FORM_BY_ID, |e, _| {
            let form = e.mem.alloc(0x10);
            e.mem.set_u32(form, 0x0395_1000);
            ret(form)
        });
        e.register(0x0395_0130, |_, _| ret(0xabcd));
        vtable(&mut e, 0x0395_1000, &[(0x130, 0x0395_0130)]);
        e.call(0x0047e380, &args![this, 2u32]);
        assert_eq!(
            logged.borrow()[1],
            vec![
                TEXT_SAVE_SIZE_FORM,
                0x1a,
                0x0100_0042,
                0xabcd,
                0x9999,
                0xc4e,
                SOURCE_FILE_PATH
            ]
        );
    }

    /// A save buffer double: `+0x14` is the write position, the writers
    /// copy their source and advance it.
    fn buffer_doubles(e: &mut Engine, save_load: u32) -> u32 {
        let buffer = e.mem.alloc(0x400);
        e.mem.set_u32(save_load + 0x14, buffer);
        e.register(BUFFER_POSITION, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        for target in [SAVE_BYTES, SAVE_NUMERIC_ID] {
            e.register(target, |e, a| {
                let position = e.mem.u32(a[0] + 0x14);
                let bytes = e.mem.bytes(a[1], a[2]);
                e.mem.write(position, &bytes);
                e.mem.set_u32(a[0] + 0x14, position + a[2]);
                Ret::default()
            });
        }
        e.register(FORM_ID_GETTER, |e, a| ret(e.mem.u32(a[0] + 0xc)));
        buffer
    }

    #[test]
    fn save_game_writes_the_block_the_data_and_the_faction_entries() {
        let mut e = engine();
        let (save_load, logged) = save_fixture(&mut e, true, false);
        let buffer = buffer_doubles(&mut e, save_load);
        let this = component(&mut e);
        e.set(this, TESActorBaseData::iActorBaseFlags, 0x11223344);
        let faction_a = e.mem.alloc(0x20);
        e.mem.set_u32(faction_a + 0xc, 0x0100_00aa);
        let faction_b = e.mem.alloc(0x20);
        e.mem.set_u32(faction_b + 0xc, 0x0100_00bb);
        let entry_a = faction_rank(&mut e, faction_a, 2);
        let entry_b = faction_rank(&mut e, faction_b, 0xff);
        make_list(&mut e, this.addr() + 0x2c, &[entry_a, 0, entry_b]);
        e.call(0x0047e4a0, &args![this, 2u32]);
        let written = e.mem.u32(save_load + 0x14) - buffer;
        // The block tag, u16 size, 0x18 data bytes, u16 count, 2 * (id + rank).
        assert_eq!(written, 4 + 2 + 0x18 + 2 + 10);
        let bytes = e.mem.bytes(buffer, written);
        assert_eq!(&bytes[..4], &TAG_KOBL.to_le_bytes());
        assert_eq!(u16::from_le_bytes([bytes[4], bytes[5]]) as u32, written - 4);
        assert_eq!(&bytes[6..10], &0x11223344u32.to_le_bytes());
        assert_eq!(u16::from_le_bytes([bytes[6 + 0x18], bytes[7 + 0x18]]), 2);
        let entries = &bytes[8 + 0x18..];
        assert_eq!(&entries[..4], &0x0100_00aau32.to_le_bytes());
        assert_eq!(entries[4], 2);
        assert_eq!(&entries[5..9], &0x0100_00bbu32.to_le_bytes());
        assert_eq!(entries[9], 0xff);
        assert!(logged.borrow().is_empty());
    }

    #[test]
    fn save_game_without_blocks_or_data_writes_only_the_faction_list_and_logs() {
        let mut e = engine();
        let (save_load, logged) = save_fixture(&mut e, false, true);
        let buffer = buffer_doubles(&mut e, save_load);
        let this = component(&mut e);
        e.call(0x0047e4a0, &args![this, 0u32]);
        assert_eq!(e.mem.u32(save_load + 0x14) - buffer, 2);
        assert_eq!(e.mem.u16(buffer), 0);
        assert_eq!(
            *logged.borrow(),
            vec![vec![TEXT_SAVE_PLAIN, 2, 0xc77, SOURCE_FILE_PATH]]
        );
    }

    #[test]
    fn save_game_reports_a_block_over_64_kib() {
        let mut e = engine();
        let (save_load, _) = save_fixture(&mut e, true, false);
        buffer_doubles(&mut e, save_load);
        let this = component(&mut e);
        // A writer that jumps the position far ahead.
        e.register(SAVE_NUMERIC_ID, |_, _| Ret::default());
        let faction = e.mem.alloc(0x20);
        let entry = faction_rank(&mut e, faction, 1);
        make_list(&mut e, this.addr() + 0x2c, &[entry]);
        let messages: Seen = Rc::default();
        let sink = messages.clone();
        e.register_double(MESSAGE_LOG, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        e.register(SAVE_BYTES, |e, a| {
            let position = e.mem.u32(a[0] + 0x14);
            e.mem.set_u32(a[0] + 0x14, position + a[2] + 0x2_0000);
            Ret::default()
        });
        // Only the position moves; the size field is written wherever the
        // block size slot is (the double does not copy), so map room.
        e.map(0x0200_0000, 0x10_0000);
        let start = 0x0200_0000;
        e.mem.set_u32(save_load + 0x14, start);
        e.call(0x0047e4a0, &args![this, 0u32]);
        assert_eq!(
            *messages.borrow(),
            vec![vec![TEXT_SAVE_BLOCK_TOO_BIG, SOURCE_FILE_PATH, 0xc77]]
        );
    }

    /// A load fixture: the save/load object, a buffer holding `bytes` with
    /// the read position at `+0x14`, a message sink, doubles for the load
    /// methods, the form lookup (`known` resolves to a form object) and the
    /// current load entry (`entry`, may be 0). Returns the buffer address.
    fn load_fixture(
        e: &mut Engine,
        blocks: bool,
        bytes: &[u8],
        known: u32,
        entry: u32,
    ) -> (u32, u32, Seen) {
        let (save_load, _) = save_fixture(e, blocks, false);
        let buffer = e.mem.alloc(0x100);
        e.mem.write(buffer, bytes);
        e.mem.set_u32(save_load + 0x14, buffer);
        e.register(BUFFER_POSITION, |e, a| ret(e.mem.u32(a[0] + 0x14)));
        for target in [LOAD_BYTES, LOAD_NUMERIC_ID] {
            e.register(target, |e, a| {
                let position = e.mem.u32(a[0] + 0x14);
                let data = e.mem.bytes(position, a[2]);
                e.mem.write(a[1], &data);
                e.mem.set_u32(a[0] + 0x14, position + a[2]);
                Ret::default()
            });
        }
        let form = e.mem.alloc(0x20);
        e.register_double(LOOKUP_FORM_BY_ID, move |_, a| {
            ret(if a[0] == known { form } else { 0 })
        });
        e.register_double(CURRENT_LOAD_ENTRY, move |_, _| ret(entry));
        e.register(SAVE_VERSION, |_, _| ret(0x1234_5678));
        let messages: Seen = Rc::default();
        let sink = messages.clone();
        e.register_double(MESSAGE_LOG, move |_, a| {
            sink.borrow_mut().push(a.to_vec());
            Ret::default()
        });
        (save_load, form, messages)
    }

    #[test]
    fn load_game_reads_the_data_and_replaces_the_faction_list() {
        let mut e = engine();
        let mut bytes = TAG_KOBL.to_le_bytes().to_vec();
        bytes.extend(38u16.to_le_bytes());
        bytes.extend((0..0x18).map(|i| 0x40 + i as u8));
        bytes.extend(2u16.to_le_bytes());
        bytes.extend(0x100u32.to_le_bytes());
        bytes.push(7);
        bytes.extend(0x200u32.to_le_bytes());
        bytes.push(9);
        let (_, form, messages) = load_fixture(&mut e, true, &bytes, 0x100, 0);
        let this = component(&mut e);
        put_faction_list(&mut e, this, &[(0x999, 1)]);
        e.call(0x0047e6f0, &args![this, 2u32, 0u32]);
        assert_eq!(e.mem.u8(this.addr() + 4), 0x40);
        assert_eq!(e.mem.u8(this.addr() + 4 + 0x17), 0x40 + 0x17);
        // The old entry is gone; the unresolved ID is dropped.
        let items = list_items(&e, this.addr() + 0x2c);
        assert_eq!(items.len(), 1);
        assert_eq!(e.mem.u32(items[0]), form);
        assert_eq!(e.mem.u8(items[0] + 4), 7);
        assert!(messages.borrow().is_empty());
    }

    #[test]
    fn load_game_without_blocks_reads_only_the_faction_list() {
        let mut e = engine();
        let mut bytes = 1u16.to_le_bytes().to_vec();
        bytes.extend(0x100u32.to_le_bytes());
        bytes.push(3);
        let (_, form, messages) = load_fixture(&mut e, false, &bytes, 0x100, 0);
        let this = component(&mut e);
        e.set(this, TESActorBaseData::iActorBaseFlags, 0x55);
        e.call(0x0047e6f0, &args![this, 0u32, 0u32]);
        assert_eq!(e.get(this, TESActorBaseData::iActorBaseFlags), 0x55);
        let items = list_items(&e, this.addr() + 0x2c);
        assert_eq!(e.mem.u32(items[0]), form);
        assert_eq!(e.mem.u8(items[0] + 4), 3);
        assert!(messages.borrow().is_empty());
    }

    #[test]
    fn load_game_logs_a_wrong_header_and_an_overrun_without_a_load_entry() {
        let mut e = engine();
        let mut bytes = 0x1111_1111u32.to_le_bytes().to_vec();
        bytes.extend(0u16.to_le_bytes());
        bytes.extend(0u16.to_le_bytes());
        let (_, _, messages) = load_fixture(&mut e, true, &bytes, 0, 0);
        let this = component(&mut e);
        e.call(0x0047e6f0, &args![this, 0u32, 0u32]);
        assert_eq!(
            *messages.borrow(),
            vec![
                vec![TEXT_LOAD_HEADER_PLAIN, SOURCE_FILE_PATH, 0xc80, 0x78],
                // The block starts at the size field: it and the count are 4 bytes over.
                vec![TEXT_LOAD_OVERRUN_PLAIN, 4, SOURCE_FILE_PATH, 0xca2, 0x78],
            ]
        );
    }

    #[test]
    fn load_game_logs_an_underrun_naming_the_load_entry() {
        let mut e = engine();
        let mut bytes = TAG_KOBL.to_le_bytes().to_vec();
        bytes.extend(10u16.to_le_bytes());
        bytes.extend(0u16.to_le_bytes());
        let entry = e.mem.alloc(0x10);
        e.mem.set_u32(entry, 0x0100_0042);
        e.mem.set_u32(entry + 5, 0xf1a9);
        e.mem.set_u8(entry + 9, 0x33);
        let (_, form, messages) = load_fixture(&mut e, true, &bytes, 0x0100_0042, entry);
        e.mem.set_u32(form, 0x0395_1000);
        e.register(0x0395_0130, |_, _| ret(0xabcd));
        vtable(&mut e, 0x0395_1000, &[(0x130, 0x0395_0130)]);
        let this = component(&mut e);
        e.call(0x0047e6f0, &args![this, 0u32, 0u32]);
        // Block size 10 counted from the size field; 4 bytes read.
        assert_eq!(
            *messages.borrow(),
            vec![vec![
                TEXT_LOAD_UNDERRUN_FORM,
                6,
                SOURCE_FILE_PATH,
                0xca2,
                0x0100_0042,
                0xabcd,
                0x33,
                0xf1a9
            ]]
        );
    }

    #[test]
    fn get_fatigue_returns_the_stored_halfword() {
        let mut e = engine();
        let this = component(&mut e);
        e.set(this, TESActorBaseData::usFatigue, 0x1234);
        assert_eq!(e.call(0x004a8ae0, &args![this]).u16(), 0x1234);
    }
}
