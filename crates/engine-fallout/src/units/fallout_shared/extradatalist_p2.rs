//! `fallout shared/extradatalist.cpp` (Xbox PDB source unit), part 2: its functions from `0041a6a0` up to
//! (not including) `0041db00` in FalloutNV.exe 1.4.0.525, translated
//! (docs/ENGINE_CRATE.md). The unit's shared layouts and helpers are in
//! [`super::extradatalist`]; anything public there may be used here.
//!
//! Translated so far, in address order: `0041a6a0` to `0041b4b0` (the first
//! 40 functions of the range): the sound, ghost, worn, can-not-wear, seed and
//! package start location setters, the one-line removers of an extra data by
//! type, the starting position and rotation accessors, the starting world or
//! cell accessors and the `ExtraAction` flag and reference setters.
//!
//! Then `0041b520` to `0041c7f0` (the next 40): the accessors, constructors
//! and destructors of the action reference, health percentage, object
//! health, cell 3D, Havok (world and cell MOPP), region list, cell music
//! type, terminal state, acoustic space, climate, image space, impact swap,
//! canopy shadow mask, editor id and reference pointer extra data. Where the
//! engine map has no name the function is `fn_<address>` and its doc says
//! what the body is.
//!
//! Then `0041c8d0` to `0041da40` (the last 40 of the range): the reference
//! pointer remover, the package (`ExtraPackage`) setter, getters and
//! remover, the trespass package and player crime list accessors, the two
//! scans that decide whether a list holds only default extra data, and the
//! leveled item index, persistent cell, ragdoll data, run once packages,
//! distant data and enable state parent accessors.
//!
//! Every list operation of the main file is called by its exe address
//! (`GetExtraData` `00410220`, `RemoveExtra` `00410020` and `00410140`,
//! `AddExtra` `0040ff60`, `HasExtra` `0040fe80`), as the functions of another
//! file are. The extra data types are `EXTRA_DATA_TYPE` of the Xbox PDB (the
//! `cEtype` byte of the extra data).
//!
//! The compiler's exception-unwinding frames (`FS:[0]` chains) of the
//! functions that allocate are not translated.

#[allow(unused_imports)]
use super::extradatalist::*;
use super::extradataobjects::{
    ExtraLeveledItem, ExtraPackage, ExtraRagDollData, ExtraRunOncePacks, ExtraTresPassPackage,
};
#[allow(unused_imports)]
use crate::prelude::*;

// ---------------------------------------------------------------------------
// Callees and constants

/// `BaseExtraList::HasExtra(type)`.
const HAS_EXTRA: u32 = 0x0040_fe80;
/// `BaseExtraList::AddExtra(extra)`.
const ADD_EXTRA: u32 = 0x0040_ff60;
/// `BaseExtraList::RemoveExtra(extra, destroy)`.
const REMOVE_EXTRA: u32 = 0x0041_0020;
/// `BaseExtraList::RemoveExtra(type)`: deletes the first extra data of that
/// type.
const REMOVE_EXTRA_BY_TYPE: u32 = 0x0041_0140;
/// `BaseExtraList::GetExtraData(type)`.
const GET_EXTRA_DATA: u32 = 0x0041_0220;
/// `operator new(size)`.
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `operator delete(block)`.
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// `BSSoundHandle::~BSSoundHandle` (`00483710`, an empty function).
const SOUND_HANDLE_DESTRUCTOR: u32 = 0x0048_3710;
/// The destructor of `ScriptLocals` (`005a8bc0`; the engine map names it
/// `ScriptLocals::ScriptLocals`, but the body runs two member destructors and
/// frees the variable block at +0x10, so it is the destructor), which the
/// scalar deleting destructor `0041af70` runs.
const SCRIPT_LOCALS_DESTRUCTOR: u32 = 0x005a_8bc0;
/// `TESObjectCELL::GetWorldSpace` (Xbox PDB).
const CELL_GET_WORLD_SPACE: u32 = 0x0054_ddd0;
/// The form id getter (`MOV EAX,[ECX+0x0C]`, `TESForm::iFormID`).
const GET_FORM_ID: u32 = 0x0084_e3a0;
/// The diagnostic logger (`cdecl`, format first; the PC build's body returns
/// 0 and prints nothing).
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// `"FORMS: Reference %s %08X has no parent save cell"`, the format
/// `SetStartingWorldOrCellForRef` logs when the reference has no child cell.
const MESSAGE_NO_PARENT_SAVE_CELL: u32 = 0x0101_5140;
/// Slot (byte offset in the vtable) of the reference's virtual whose result
/// the message above prints for `%s`.
const REFERENCE_NAME_SLOT: u32 = 0x130;
/// The list's type `0x0E` extra data (`ExtraAction`) or a new one
/// (`004195b0`, main file).
const GET_OR_ADD_ACTION: u32 = 0x0041_95b0;
/// The list's `ExtraStartingPosition` or a new one made from a position
/// (`00418d50`, main file).
const GET_OR_ADD_STARTING_POSITION: u32 = 0x0041_8d50;

const EXTRA_SCRIPT: u8 = 0x0d;
const EXTRA_ACTION: u8 = 0x0e;
const EXTRA_STARTING_POSITION: u8 = 0x0f;
const EXTRA_ANIM: u8 = 0x10;
const EXTRA_CONTAINER_CHANGES: u8 = 0x15;
const EXTRA_WORN: u8 = 0x16;
const EXTRA_WORN_LEFT: u8 = 0x17;
const EXTRA_PACKAGE_START_LOCATION: u8 = 0x18;
const EXTRA_GHOST: u8 = 0x1f;
const EXTRA_OWNERSHIP: u8 = 0x21;
const EXTRA_COUNT: u8 = 0x24;
const EXTRA_HEALTH: u8 = 0x25;
const EXTRA_LIGHT: u8 = 0x29;
const EXTRA_LOCK: u8 = 0x2a;
const EXTRA_TELEPORT: u8 = 0x2b;
const EXTRA_ANIM_SAVE: u8 = 0x2d;
const EXTRA_SCALE: u8 = 0x30;
const EXTRA_SEED: u8 = 0x31;
const EXTRA_CAN_NOT_WEAR: u8 = 0x3e;
const EXTRA_POISON: u8 = 0x3f;
const EXTRA_MAGIC_LIGHT: u8 = 0x40;
const EXTRA_STARTING_WORLD_OR_CELL: u8 = 0x49;
const EXTRA_SOUND: u8 = 0x4f;
const EXTRA_ACTIVATE_LOOP_SOUND: u8 = 0x87;

/// Constructors of the extra data these setters build (`this` = the new
/// block; the other words are the constructor's arguments).
const EXTRA_GHOST_INIT: u32 = 0x0043_22c0;
const EXTRA_WORN_INIT: u32 = 0x0043_22f0;
const EXTRA_WORN_LEFT_INIT: u32 = 0x0043_2320;
const EXTRA_CAN_NOT_WEAR_INIT: u32 = 0x0043_2350;
const EXTRA_SEED_INIT: u32 = 0x0043_25b0;
const EXTRA_PACKAGE_START_LOCATION_INIT: u32 = 0x0043_26c0;
const EXTRA_STARTING_WORLD_OR_CELL_INIT: u32 = 0x0043_08c0;
/// Constructors of the two sound extra data; each takes a `BSSoundHandle`
/// by value (three words).
const EXTRA_SOUND_INIT: u32 = 0x0043_60c0;
const EXTRA_ACTIVATE_LOOP_SOUND_INIT: u32 = 0x0043_6660;

// Extra data types of the next functions (`EXTRA_DATA_TYPE`, Xbox PDB).
const EXTRA_HAVOK: u8 = 0x01;
const EXTRA_CELL_3D: u8 = 0x02;
const EXTRA_REGION_LIST: u8 = 0x04;
const EXTRA_EDITOR_ID: u8 = 0x06;
const EXTRA_CELL_MUSIC_TYPE: u8 = 0x07;
const EXTRA_CLIMATE: u8 = 0x08;
const EXTRA_CELL_CANOPY_SHADOW_MASK: u8 = 0x0a;
const EXTRA_REFERENCE_POINTER: u8 = 0x1c;
const EXTRA_TERMINAL_STATE: u8 = 0x50;
const EXTRA_OBJECT_HEALTH: u8 = 0x56;
const EXTRA_IMAGE_SPACE: u8 = 0x59;
const EXTRA_HEALTH_PERC: u8 = 0x7a;
const EXTRA_CELL_ACOUSTIC_SPACE: u8 = 0x81;
const EXTRA_IMPACT_SWAP: u8 = 0x8c;

/// Vtables the constructors of this file store at +0 (slot 0 the scalar
/// deleting destructor).
const VTABLE_EXTRA_HEALTH_PERC: u32 = 0x0101_5178;
const VTABLE_EXTRA_OBJECT_HEALTH: u32 = 0x0101_5184;
const VTABLE_EXTRA_TERMINAL_STATE: u32 = 0x0101_5190;
const VTABLE_EXTRA_EDITOR_ID: u32 = 0x0101_519c;

// Extra data types of the last functions (`EXTRA_DATA_TYPE`, Xbox PDB).
const EXTRA_PERSISTENT_CELL: u8 = 0x0c;
const EXTRA_DISTANT_DATA: u8 = 0x13;
const EXTRA_RAG_DOLL_DATA: u8 = 0x14;
const EXTRA_PACKAGE: u8 = 0x19;
const EXTRA_TRESPASS_PACKAGE: u8 = 0x1a;
const EXTRA_RUN_ONCE_PACKAGES: u8 = 0x1b;
const EXTRA_LEVELED_ITEM: u8 = 0x2f;
const EXTRA_PLAYER_CRIME_LIST: u8 = 0x35;
const EXTRA_ENABLE_STATE_PARENT: u8 = 0x37;

/// Constructors (in `extradataobjects.cpp`) of the extra data these setters
/// build; `this` is the new block, then the constructor's own words.
/// `ExtraPackage`: the package, the index, the target and the three flag
/// bytes (each passed as a word).
const EXTRA_PACKAGE_INIT: u32 = 0x0043_2870;
/// `ExtraTresPassPackage`: the package.
const EXTRA_TRESPASS_PACKAGE_INIT: u32 = 0x0043_28d0;
/// `ExtraPlayerCrimeList`: the first crime.
const EXTRA_PLAYER_CRIME_LIST_INIT: u32 = 0x0043_2aa0;
/// `ExtraLeveledItem`: the index (the caller clears the default flag).
const EXTRA_LEVELED_ITEM_INIT: u32 = 0x0043_2be0;
/// `ExtraPersistentCell`: the cell.
const EXTRA_PERSISTENT_CELL_INIT: u32 = 0x0043_2c20;
/// `ExtraRunOncePacks` (no arguments; it makes the empty package list).
const EXTRA_RUN_ONCE_PACKAGES_INIT: u32 = 0x0043_3010;
/// `ExtraDistantData` (no arguments).
const EXTRA_DISTANT_DATA_INIT: u32 = 0x0043_3260;
/// `ExtraEnableStateParent` (no arguments).
const EXTRA_ENABLE_STATE_PARENT_INIT: u32 = 0x0043_3300;
/// Adds a package (a word and a byte) to the list of an `ExtraRunOncePacks`
/// (`this` = the extra data; `extradataobjects.cpp`).
const RUN_ONCE_PACKAGES_ADD: u32 = 0x0043_31c0;

/// `TESPackage::GetIsCreated`, `IsNeverToRun` and `SetIsCreated` (Xbox PDB;
/// the last takes the new flag).
const PACKAGE_GET_IS_CREATED: u32 = 0x0067_4d40;
const PACKAGE_IS_NEVER_TO_RUN: u32 = 0x0067_4e40;
const PACKAGE_SET_IS_CREATED: u32 = 0x0067_4d70;
/// `TESSaveLoadGame::DeleteForm` (Xbox PDB; `this` is the singleton).
const SAVE_LOAD_GAME_DELETE_FORM: u32 = 0x0085_a2e0;
/// `RagDollData::UpdateDataFromReference` and `RagDollData::Copy` (Xbox
/// PDB): each takes the source (a reference, or another `RagDollData`).
const RAG_DOLL_UPDATE_FROM_REFERENCE: u32 = 0x004d_9800;
const RAG_DOLL_COPY: u32 = 0x004d_9670;
/// Called on each crime of a player crime list with the word the remover was
/// given (`009eba00`; the engine map puts it in `alarmpackage.cpp`): it
/// removes that word from the `BSSimpleList` at +0x1c (`00905330`) and counts
/// the first word of the object down.
const CRIME_REMOVE_ENTRY: u32 = 0x009e_ba00;
/// Frees the nodes of a `BSSimpleList` after its head (`00470470`; `this` =
/// the list) and the scalar deleting destructor of a list (`004702f0`, then
/// the flags word).
const SIMPLE_LIST_FREE_NODES: u32 = 0x0047_0470;
const SIMPLE_LIST_SCALAR_DELETING_DESTRUCTOR: u32 = 0x0047_02f0;
/// `BSSimpleList::Remove(value*)`: unlinks the first node whose item equals
/// the word at the given address (`00905330`; `this` = the list).
const SIMPLE_LIST_REMOVE_VALUE: u32 = 0x0090_5330;
/// Reads the byte at +3 of the time stamp `fn_0041d8a0` returns (`0086a460`).
const TIME_STAMP_BYTE: u32 = 0x0086_a460;

/// The `double` `1.0` that `fn_0041b580` compares the health percentage with
/// (`FCOMP`), read from the exe's constants.
const ONE_AS_DOUBLE: u32 = 0x0101_2070;
/// The `float` `-1.0` that `ExtraDataList::GetObjectHealth` answers without
/// an extra data.
const NO_OBJECT_HEALTH: u32 = 0x0101_2054;

/// Constructors (in `extradatacell.cpp` and `extradataobjects.cpp`) of the
/// extra data these setters build; `this` is the new block, then the
/// constructor's own words.
const EXTRA_HAVOK_INIT: u32 = 0x0040_eda0;
const EXTRA_CELL_3D_INIT: u32 = 0x0040_ec00;
const EXTRA_REGION_LIST_INIT: u32 = 0x0040_ef00;
const EXTRA_CELL_MUSIC_TYPE_INIT: u32 = 0x0040_ef30;
const EXTRA_CELL_ACOUSTIC_SPACE_INIT: u32 = 0x0040_efd0;
const EXTRA_CLIMATE_INIT: u32 = 0x0040_f070;
const EXTRA_IMAGE_SPACE_INIT: u32 = 0x0040_f110;
const EXTRA_IMPACT_SWAP_INIT: u32 = 0x0040_f1b0;
/// `ExtraCellCanopyShadowMask` constructor: the state (+0x0C) and the mask.
const EXTRA_CELL_CANOPY_SHADOW_MASK_INIT: u32 = 0x0040_f440;
const EXTRA_REFERENCE_POINTER_INIT: u32 = 0x0043_27e0;
/// The body that runs the destructor of the `BSExtraData` base
/// (`extradataobjects.cpp`, `fastcall`).
const EXTRA_DATA_BASE_DESTRUCTOR: u32 = 0x0043_2f30;

/// `NiPointer<T>` operations (`this` = the address of the smart pointer):
/// construct from a pointer, destroy, read the pointer, assign a pointer
/// (references the new object and releases the old one), compare with a
/// pointer.
const NI_POINTER_INIT: u32 = 0x0063_3c90;
const NI_POINTER_DESTROY: u32 = 0x0045_cec0;
const NI_POINTER_GET: u32 = 0x0055_9450;
const NI_POINTER_ASSIGN: u32 = 0x0066_b0d0;
const NI_POINTER_EQUALS: u32 = 0x0082_2510;
/// `BSStringT<char>` operations on the editor id: construct empty, destroy,
/// set from a C string (second word `0`).
const BS_STRING_INIT: u32 = 0x0040_37b0;
const BS_STRING_DESTROY: u32 = 0x0040_37d0;
const BS_STRING_SET: u32 = 0x0040_37f0;
/// The global holding the `TESSaveLoadGame` singleton pointer.
const SAVE_LOAD_GAME: u32 = 0x011d_e45c;
/// A stub that answers false (`xor al, al`), called on the save/load
/// singleton.
const SAVE_LOAD_GAME_STUB: u32 = 0x0047_c850;
/// `TESObjectREFR::GetRefPersists` (Xbox PDB).
const GET_REF_PERSISTS: u32 = 0x0056_53d0;

// ---------------------------------------------------------------------------
// Helpers

/// The first extra data of `extra_type` in the list (`GetExtraData`).
fn find_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> Ptr<BSExtraData> {
    e.call(GET_EXTRA_DATA, &args![list, extra_type]).ptr()
}

/// Whether the list has an extra data of `extra_type` (`HasExtra`).
fn has_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> bool {
    e.call(HAS_EXTRA, &args![list, extra_type]).bool()
}

/// Adds `extra` to the list (`AddExtra`).
fn add_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra: u32) {
    e.call(ADD_EXTRA, &args![list, extra]);
}

/// Unlinks `extra` and deletes it (`RemoveExtra(extra, true)`).
fn remove_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra: Ptr<BSExtraData>) {
    e.call(REMOVE_EXTRA, &args![list, extra, true]);
}

/// Deletes the first extra data of `extra_type` (`RemoveExtra(type)`).
fn remove_extra_by_type(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) {
    e.call(REMOVE_EXTRA_BY_TYPE, &args![list, extra_type]);
}

/// `new` and construct: allocates `size` bytes and runs the constructor at
/// `construct` on the block (`this` = the block, then `construct_args`); a
/// failed allocation gives a null extra data. The result is added to the
/// list either way, as the code does. Returns the extra data.
fn add_new_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    size: u32,
    construct: u32,
    construct_args: &[u32],
) -> Ptr<BSExtraData> {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    let extra = if block == 0 {
        0
    } else {
        let mut words = vec![block];
        words.extend_from_slice(construct_args);
        e.call(construct, &words).u32()
    };
    add_extra(e, list, extra);
    Ptr::new(extra)
}

/// A `float` loaded and stored through the x87 stack (`FLD`/`FSTP`): the
/// same bits, except that a signalling NaN comes out quiet.
fn x87_float_bits(value: f32) -> u32 {
    let bits = value.to_bits();
    let is_nan = bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0;
    if is_nan {
        bits | 0x0040_0000
    } else {
        bits
    }
}

/// The body of the two sound setters. With an extra data of `extra_type` in
/// the list: a sound handle with the same `iSoundID` as an empty handle
/// (`fn_0041a1f0` against a fresh `fn_0041a250` handle) deletes the extra
/// data, any other is copied into it (`fn_00418900`, the `BSSoundHandle` at
/// +0x0C). Without one, a non-empty handle (`fn_0041a220`) builds an extra
/// data of `0x18` bytes with `construct`, which takes the handle by value (a
/// copy made by `fn_00418900` on the stack, three words), and adds it; an
/// empty handle does nothing. The temporary empty handles are destroyed by
/// `00483710`.
fn set_sound_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    construct: u32,
    sound: Ptr<BSSoundHandle>,
) {
    let extra = find_extra(e, list, extra_type);
    if !extra.is_null() {
        let is_empty = e.with_stack(0x0c, |e, empty| {
            fn_0041a250(e, empty);
            let equal = fn_0041a1f0(e, sound.cast(), empty);
            e.call(SOUND_HANDLE_DESTRUCTOR, &args![empty]);
            equal
        });
        if is_empty {
            remove_extra(e, list, extra);
        } else {
            fn_00418900(e, Ptr::new(extra.addr() + 0x0c), sound.cast());
        }
        return;
    }
    let differs = e.with_stack(0x0c, |e, empty| {
        fn_0041a250(e, empty);
        let differs = fn_0041a220(e, sound.cast(), empty);
        e.call(SOUND_HANDLE_DESTRUCTOR, &args![empty]);
        differs
    });
    if !differs {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0x18u32]).u32();
    let new_extra = if block == 0 {
        0
    } else {
        // The handle is passed by value: a copy on the stack, three words.
        let words = e.with_stack(0x0c, |e, copy| {
            fn_00418900(e, copy, sound.cast());
            [
                e.mem.u32(copy.addr()),
                e.mem.u32(copy.addr() + 4),
                e.mem.u32(copy.addr() + 8),
            ]
        });
        e.call(construct, &args![block, words[0], words[1], words[2]])
            .u32()
    };
    add_extra(e, list, new_extra);
}

/// The shape of the flag setters (ghost, can not wear, worn): an extra data
/// of `extra_type` with no payload (`0x0C` bytes, built by `construct`) is
/// added when `wanted` and the list has none, and deleted when not `wanted`
/// and the list has one.
fn set_flag_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    construct: u32,
    wanted: bool,
) {
    let present = has_extra(e, list, extra_type);
    if wanted && !present {
        add_new_extra(e, list, 0x0c, construct, &[]);
    } else if !wanted && present {
        remove_extra_by_type(e, list, extra_type);
    }
}

/// The list's type `0x0F` extra data (`ExtraStartingPosition`), made by
/// `00418d50` from `position` when the list has none.
fn starting_position_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    position: Ptr,
) -> Ptr<BSExtraData> {
    let extra = find_extra(e, list, EXTRA_STARTING_POSITION);
    if !extra.is_null() {
        return extra;
    }
    e.call(GET_OR_ADD_STARTING_POSITION, &args![list, position])
        .ptr()
}

/// Copies the three words at `source` to `target` (a `NiPoint3`).
fn copy_point(e: &mut Engine, source: u32, target: u32) {
    for offset in [0u32, 4, 8] {
        let word = e.mem.u32(source + offset);
        e.mem.set_u32(target + offset, word);
    }
}

/// Stores the three words `x`, `y`, `z` at `target`.
fn store_point(e: &mut Engine, target: u32, x: u32, y: u32, z: u32) {
    e.mem.set_u32(target, x);
    e.mem.set_u32(target + 4, y);
    e.mem.set_u32(target + 8, z);
}

/// `new` and construct with a constructor of this file: allocates `size`
/// bytes, builds the extra data on the block with `construct` (a failed
/// allocation gives a null extra data) and adds the result to the list
/// either way, as `add_new_extra`. Returns the extra data.
fn add_extra_built_by(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    size: u32,
    construct: impl FnOnce(&mut Engine, Ptr) -> Ptr,
) -> Ptr<BSExtraData> {
    let block = e.call(OPERATOR_NEW, &args![size]).u32();
    let extra = if block == 0 {
        0
    } else {
        construct(e, Ptr::new(block)).addr()
    };
    add_extra(e, list, extra);
    Ptr::new(extra)
}

/// The shape of the setters of an extra data whose only payload is a word at
/// +0x0C (`0x10` bytes, built by `construct` from that word): a null `value`
/// deletes the extra data when the list has one; any other builds it when
/// the list has none and is stored in the existing one otherwise.
fn set_word_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    construct: u32,
    value: u32,
) {
    let extra = find_extra(e, list, extra_type);
    if value == 0 {
        if !extra.is_null() {
            remove_extra(e, list, extra);
        }
    } else if extra.is_null() {
        add_new_extra(e, list, 0x10, construct, &[value]);
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, value);
    }
}

/// The word at +0x0C of the list's extra data of `extra_type`, or 0.
fn get_word_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) -> u32 {
    let extra = find_extra(e, list, extra_type);
    if extra.is_null() {
        0
    } else {
        e.mem.u32(extra.addr() + 0x0c)
    }
}

/// The pointer held by the `NiPointer` at `offset` of the list's extra data
/// of `extra_type`; without the extra data, the pointer of a temporary
/// `NiPointer` made from null (so 0), built and destroyed on the stack.
fn get_ni_pointer_extra(
    e: &mut Engine,
    list: Ptr<ExtraDataList>,
    extra_type: u8,
    offset: u32,
) -> u32 {
    let extra = find_extra(e, list, extra_type);
    if !extra.is_null() {
        return e.call(NI_POINTER_GET, &args![extra.addr() + offset]).u32();
    }
    e.with_stack(4, |e, temporary| {
        e.call(NI_POINTER_INIT, &args![temporary, 0u32]);
        let value = e.call(NI_POINTER_GET, &args![temporary]).u32();
        e.call(NI_POINTER_DESTROY, &args![temporary]);
        value
    })
}

// ---------------------------------------------------------------------------
// Translations

// Translated from 0041a6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the sound of the type `0x87` extra data (`EXTRA_ACTIVATE_LOOP_SOUND`
/// in the Xbox enum, built by `00436660`, `0x18` bytes): see
/// `set_sound_extra` for the cases. `sound` is a pointer to the
/// `BSSoundHandle`. The engine map has no name for it (its getter is
/// `ExtraDataList::GetActivateLoopSound`).
pub fn fn_0041a6a0(e: &mut Engine, this: Ptr<ExtraDataList>, sound: Ptr<BSSoundHandle>) {
    set_sound_extra(
        e,
        this,
        EXTRA_ACTIVATE_LOOP_SOUND,
        EXTRA_ACTIVATE_LOOP_SOUND_INIT,
        sound,
    );
}

// Translated from 0041a800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetSound` (Xbox PDB): as `fn_0041a6a0`, for the type
/// `0x4F` extra data (`EXTRA_SOUND`, built by `004360c0`).
pub fn extra_data_list_set_sound(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    sound: Ptr<BSSoundHandle>,
) {
    set_sound_extra(e, this, EXTRA_SOUND, EXTRA_SOUND_INIT, sound);
}

// Translated from 0041a960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetGhost` (Xbox PDB): with `ghost` set, adds the type
/// `0x1F` extra data (`EXTRA_GHOST`, `0x0C` bytes, built by `004322c0`) when
/// the list has none; cleared, deletes it when the list has one.
pub fn extra_data_list_set_ghost(e: &mut Engine, this: Ptr<ExtraDataList>, ghost: bool) {
    set_flag_extra(e, this, EXTRA_GHOST, EXTRA_GHOST_INIT, ghost);
}

// Translated from 0041aa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetWorn` (Xbox PDB): the flag setter for the worn marker
/// of one hand. With `left` set it is the type `0x17` extra data
/// (`EXTRA_WORN_LEFT`, built by `00432320`), otherwise the type `0x16` one
/// (`EXTRA_WORN`, built by `004322f0`); `worn` adds it when missing and
/// cleared deletes it when present, as `ExtraDataList::SetGhost`.
pub fn extra_data_list_set_worn(e: &mut Engine, this: Ptr<ExtraDataList>, worn: bool, left: bool) {
    if left {
        set_flag_extra(e, this, EXTRA_WORN_LEFT, EXTRA_WORN_LEFT_INIT, worn);
    } else {
        set_flag_extra(e, this, EXTRA_WORN, EXTRA_WORN_INIT, worn);
    }
}

// Translated from 0041ab70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetCanNotWear` (Xbox PDB): the flag setter for the type
/// `0x3E` extra data (`EXTRA_CANNOTWEAR`, built by `00432350`), as
/// `ExtraDataList::SetGhost`.
pub fn extra_data_list_set_can_not_wear(e: &mut Engine, this: Ptr<ExtraDataList>, wanted: bool) {
    set_flag_extra(e, this, EXTRA_CAN_NOT_WEAR, EXTRA_CAN_NOT_WEAR_INIT, wanted);
}

// Translated from 0041ac30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the seed byte (`ExtraSeed`, type `0x31`, `0x10` bytes, byte at
/// +0x0C; built by `004325b0`). `0xFF` means no seed: it deletes an existing
/// extra data and does nothing without one. Any other value is stored in the
/// existing extra data or builds one. The engine map has no name for it.
pub fn fn_0041ac30(e: &mut Engine, this: Ptr<ExtraDataList>, seed: u8) {
    let extra = find_extra(e, this, EXTRA_SEED);
    if seed != 0xff {
        if extra.is_null() {
            add_new_extra(e, this, 0x10, EXTRA_SEED_INIT, &[seed as u32]);
        } else {
            e.mem.set_u8(extra.addr() + 0x0c, seed);
        }
    } else if !extra.is_null() {
        remove_extra(e, this, extra);
    }
}

// Translated from 0041ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetPackageStartLocation` (Xbox PDB): the type `0x18` extra
/// data (`ExtraPackageStartLocation`, `0x20` bytes: a `WORLD_LOCATION` at
/// +0x0C, whose `pLocationForm` is `location_form`, or `cell` when that is
/// null, then the position and `fZRot`). Without one, builds it (`004326c0`
/// with the two forms, the position pointer and `z_rot`, which the code
/// moves through the x87 stack) and adds it. With one, stores the form and
/// copies the position; `z_rot` is not stored then.
pub fn extra_data_list_set_package_start_location(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    location_form: u32,
    cell: u32,
    position: Ptr,
    z_rot: f32,
) {
    let extra = find_extra(e, this, EXTRA_PACKAGE_START_LOCATION);
    if extra.is_null() {
        add_new_extra(
            e,
            this,
            0x20,
            EXTRA_PACKAGE_START_LOCATION_INIT,
            &[location_form, cell, position.addr(), x87_float_bits(z_rot)],
        );
        return;
    }
    if location_form != 0 {
        e.mem.set_u32(extra.addr() + 0x0c, location_form);
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, cell);
    }
    copy_point(e, position.addr(), extra.addr() + 0x10);
}

// Translated from 0041adf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveAnimPtr` (Xbox PDB): deletes the type `0x10` extra
/// data (`EXTRA_ANIM`).
pub fn extra_data_list_remove_anim_ptr(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_ANIM);
}

// Translated from 0041ae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveAnimSave` (Xbox PDB): deletes the type `0x2D` extra
/// data (`EXTRA_ANIM_SAVE`).
pub fn extra_data_list_remove_anim_save(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_ANIM_SAVE);
}

// Translated from 0041ae30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x29` extra data (`EXTRA_LIGHT`). The engine map has no
/// name for it.
pub fn fn_0041ae30(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_LIGHT);
}

// Translated from 0041ae50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x40` extra data (`EXTRA_MAGIC_LIGHT`). The engine map
/// has no name for it.
pub fn fn_0041ae50(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_MAGIC_LIGHT);
}

// Translated from 0041ae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveLockPtr` (Xbox PDB): deletes the type `0x2A` extra
/// data (`EXTRA_LOCK`).
pub fn extra_data_list_remove_lock_ptr(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_LOCK);
}

// Translated from 0041ae90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveTeleportPtr` (Xbox PDB): deletes the type `0x2B`
/// extra data (`EXTRA_TELEPORT`).
pub fn extra_data_list_remove_teleport_ptr(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_TELEPORT);
}

// Translated from 0041aeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x15` extra data (`EXTRA_CONTAINER_CHANGES`). The
/// engine map has no name for it.
pub fn fn_0041aeb0(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_CONTAINER_CHANGES);
}

// Translated from 0041aed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveOwnership` (Xbox PDB): deletes the type `0x21` extra
/// data (`EXTRA_OWNERSHIP`).
pub fn extra_data_list_remove_ownership(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_OWNERSHIP);
}

// Translated from 0041aef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveHealth` (Xbox PDB): deletes the type `0x25` extra
/// data (`EXTRA_HEALTH`).
pub fn extra_data_list_remove_health(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_HEALTH);
}

// Translated from 0041af10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveCount` (Xbox PDB): deletes the type `0x24` extra
/// data (`EXTRA_COUNT`).
pub fn extra_data_list_remove_count(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_COUNT);
}

// Translated from 0041af30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x3F` extra data (`EXTRA_POISON`). The engine map has
/// no name for it.
pub fn fn_0041af30(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_POISON);
}

// Translated from 0041af50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x0D` extra data (`EXTRA_SCRIPT`). The engine map has
/// no name for it.
pub fn fn_0041af50(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_SCRIPT);
}

// Translated from 0041af70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `ScriptLocals` (the variables of a
/// script instance, the `pScriptVars` of an `ExtraScript`): runs the
/// destructor `005a8bc0`, then `operator delete` when bit 0 of `flags` is
/// set. Returns `this`. The engine map has no name for it.
pub fn script_locals_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(SCRIPT_LOCALS_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0041afa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::ClearScriptLocals` (Xbox PDB): sets `pScriptVars` (+0x10)
/// of the type `0x0D` extra data (`ExtraScript`) to null, without freeing
/// it; nothing without one.
pub fn extra_data_list_clear_script_locals(e: &mut Engine, this: Ptr<ExtraDataList>) {
    let extra = find_extra(e, this, EXTRA_SCRIPT);
    if !extra.is_null() {
        e.mem.set_u32(extra.addr() + 0x10, 0);
    }
}

// Translated from 0041afd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x30` extra data (`EXTRA_SCALE`). The engine map has no
/// name for it.
pub fn fn_0041afd0(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_SCALE);
}

// Translated from 0041aff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x1F` extra data (`EXTRA_GHOST`). The engine map has no
/// name for it.
pub fn fn_0041aff0(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_GHOST);
}

// Translated from 0041b010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the worn marker of one hand: the type `0x17` extra data
/// (`EXTRA_WORN_LEFT`) with `left` set, the type `0x16` one (`EXTRA_WORN`)
/// otherwise. The engine map has no name for it.
pub fn fn_0041b010(e: &mut Engine, this: Ptr<ExtraDataList>, left: bool) {
    if left {
        remove_extra_by_type(e, this, EXTRA_WORN_LEFT);
    } else {
        remove_extra_by_type(e, this, EXTRA_WORN);
    }
}

// Translated from 0041b040 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveCannotWearExtra` (Xbox PDB): deletes the type `0x3E`
/// extra data (`EXTRA_CANNOTWEAR`).
pub fn extra_data_list_remove_cannot_wear_extra(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_CAN_NOT_WEAR);
}

// Translated from 0041b060 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x18` extra data (`EXTRA_PACKAGESTARTLOC`). The engine
/// map has no name for it.
pub fn fn_0041b060(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_PACKAGE_START_LOCATION);
}

// Translated from 0041b080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the rotation (`rot`, a `NiPoint3` at +0x18) of the list's type
/// `0x0F` extra data (`ExtraStartingPosition`) to `out`; the extra data is
/// made from `position` (`00418d50`) when the list has none. Returns `out`.
/// The engine map has no name for it.
pub fn fn_0041b080(e: &mut Engine, this: Ptr<ExtraDataList>, out: Ptr, position: Ptr) -> Ptr {
    let extra = starting_position_extra(e, this, position);
    copy_point(e, extra.addr() + 0x18, out.addr());
    out
}

// Translated from 0041b0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `fn_0041b080`, for the position (`pos`, at +0x0C). Returns `out`. The
/// engine map has no name for it.
pub fn fn_0041b0d0(e: &mut Engine, this: Ptr<ExtraDataList>, out: Ptr, position: Ptr) -> Ptr {
    let extra = starting_position_extra(e, this, position);
    copy_point(e, extra.addr() + 0x0c, out.addr());
    out
}

// Translated from 0041b120 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the rotation (+0x18) of the list's `ExtraStartingPosition` (made from
/// `position` when missing, as `fn_0041b080`) to the three words
/// `x`, `y`, `z`, and stores the same words at `out`. Returns `out`. The
/// engine map has no name for it.
pub fn fn_0041b120(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    out: Ptr,
    position: Ptr,
    x: u32,
    y: u32,
    z: u32,
) -> Ptr {
    let extra = starting_position_extra(e, this, position);
    store_point(e, extra.addr() + 0x18, x, y, z);
    store_point(e, out.addr(), x, y, z);
    out
}

// Translated from 0041b180 (decompiled, FalloutNV.exe 1.4.0.525)
/// As `fn_0041b120`, for the position (+0x0C). Returns `out`. The engine map
/// has no name for it.
pub fn fn_0041b180(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    out: Ptr,
    position: Ptr,
    x: u32,
    y: u32,
    z: u32,
) -> Ptr {
    let extra = starting_position_extra(e, this, position);
    store_point(e, extra.addr() + 0x0c, x, y, z);
    store_point(e, out.addr(), x, y, z);
    out
}

// Translated from 0041b1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the form of the type `0x49` extra data (`ExtraStartingWorldOrCell`,
/// `0x10` bytes, `pStartingWorldOrCell` at +0x0C; built by `004308c0`). A
/// null `form` deletes it (`fn_0041b350`); otherwise the extra data is built
/// when the list has none and takes `form`. The engine map has no name for
/// it.
pub fn fn_0041b1e0(e: &mut Engine, this: Ptr<ExtraDataList>, form: u32) {
    if form == 0 {
        fn_0041b350(e, this);
        return;
    }
    let mut extra = find_extra(e, this, EXTRA_STARTING_WORLD_OR_CELL);
    if extra.is_null() {
        extra = add_new_extra(e, this, 0x10, EXTRA_STARTING_WORLD_OR_CELL_INIT, &[]);
    }
    e.mem.set_u32(extra.addr() + 0x0c, form);
}

// Translated from 0041b2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetStartingWorldOrCellForRef` (Xbox PDB): sets the starting
/// world or cell from a reference. The reference's base object at +0x18
/// (slot 0 of the vtable stored there) gives a cell; the starting world or
/// cell is that cell's world space (`TESObjectCELL::GetWorldSpace`) when it
/// has one, the cell itself otherwise (`fn_0041b1e0`). When the reference
/// gives no cell, its form id and the result of its virtual `0x130` are
/// logged with the format at `01015140`.
pub fn extra_data_list_set_starting_world_or_cell_for_ref(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    reference: Ptr,
) {
    let cell = e.vcall(reference.addr() + 0x18, 0, &args![]).u32();
    if cell != 0 {
        let world = e.call(CELL_GET_WORLD_SPACE, &args![cell]).u32();
        if world != 0 {
            fn_0041b1e0(e, this, world);
        } else {
            fn_0041b1e0(e, this, cell);
        }
    } else {
        let form_id = e.call(GET_FORM_ID, &args![reference]).u32();
        let name = e
            .vcall(reference.addr(), REFERENCE_NAME_SLOT, &args![])
            .u32();
        e.call(
            LOG_MESSAGE,
            &args![MESSAGE_NO_PARENT_SAVE_CELL, name, form_id],
        );
    }
}

// Translated from 0041b320 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetStartingWorldOrCell` (Xbox PDB): the
/// `pStartingWorldOrCell` (+0x0C) of the type `0x49` extra data, or null.
pub fn extra_data_list_get_starting_world_or_cell(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_STARTING_WORLD_OR_CELL);
    if extra.is_null() {
        0
    } else {
        e.mem.u32(extra.addr() + 0x0c)
    }
}

// Translated from 0041b350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x49` extra data (`EXTRA_STARTINGWORLDORCELL`). The
/// engine map has no name for it.
pub fn fn_0041b350(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_STARTING_WORLD_OR_CELL);
}

// Translated from 0041b370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `eAction` byte (+0x0C) of the type `0x0E` extra data (`ExtraAction`),
/// or 1 when the list has none. The engine map has no name for it.
pub fn fn_0041b370(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_ACTION);
    if extra.is_null() {
        1
    } else {
        e.mem.u8(extra.addr() + 0x0c) as u32
    }
}

// Translated from 0041b3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether `eAction` of the list (`fn_0041b370`) has any bit of `mask`.
/// The engine map has no name for it.
pub fn fn_0041b3a0(e: &mut Engine, this: Ptr<ExtraDataList>, mask: u32) -> bool {
    fn_0041b370(e, this) & mask != 0
}

// Translated from 0041b3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `eAction` (+0x0C, the low byte of `action`) of the type `0x0E` extra
/// data (`ExtraAction`). Without one, a value other than 1 (the default
/// `fn_0041b370` reports) gets one from `004195b0`, and 1 does nothing. With
/// one, the value 1 deletes it when its `pActionRef` (+0x10) is null. (The
/// engine map names this `ExtraDataList::SetGlobal`; the body is the
/// `ExtraAction` setter.)
pub fn fn_0041b3d0(e: &mut Engine, this: Ptr<ExtraDataList>, action: u32) {
    let mut extra = find_extra(e, this, EXTRA_ACTION);
    if extra.is_null() {
        if action != 1 {
            extra = e.call(GET_OR_ADD_ACTION, &args![this]).ptr();
        }
    } else if action == 1 && e.mem.u32(extra.addr() + 0x10) == 0 {
        remove_extra(e, this, extra);
        extra = Ptr::NULL;
    }
    if !extra.is_null() {
        e.mem.set_u8(extra.addr() + 0x0c, action as u8);
    }
}

// Translated from 0041b440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the bits of `mask` in `eAction`: `fn_0041b3d0(fn_0041b370 | mask)`.
/// The engine map has no name for it.
pub fn fn_0041b440(e: &mut Engine, this: Ptr<ExtraDataList>, mask: u32) {
    let action = fn_0041b370(e, this) | mask;
    fn_0041b3d0(e, this, action);
}

// Translated from 0041b470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the bits of `mask` in `eAction`: `fn_0041b3d0(fn_0041b370 & !mask)`.
/// The engine map has no name for it.
pub fn fn_0041b470(e: &mut Engine, this: Ptr<ExtraDataList>, mask: u32) {
    let action = fn_0041b370(e, this) & !mask;
    fn_0041b3d0(e, this, action);
}

// Translated from 0041b4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets `pActionRef` (+0x10) of the type `0x0E` extra data (`ExtraAction`).
/// Without one, a non-null `reference` gets one from `004195b0` and null does
/// nothing. With one, a null `reference` deletes it when its `eAction` is 1.
/// The engine map has no name for it.
pub fn fn_0041b4b0(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) {
    let mut extra = find_extra(e, this, EXTRA_ACTION);
    if extra.is_null() {
        if reference != 0 {
            extra = e.call(GET_OR_ADD_ACTION, &args![this]).ptr();
        }
    } else if e.mem.u8(extra.addr() + 0x0c) == 1 && reference == 0 {
        remove_extra(e, this, extra);
        extra = Ptr::NULL;
    }
    if !extra.is_null() {
        e.mem.set_u32(extra.addr() + 0x10, reference);
    }
}

// Translated from 0041b520 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetActionRef` (Xbox PDB): the `pActionRef` (+0x10) of the
/// type `0x0E` extra data (`ExtraAction`), or null.
pub fn extra_data_list_get_action_ref(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_ACTION);
    if extra.is_null() {
        0
    } else {
        e.mem.u32(extra.addr() + 0x10)
    }
}

// Translated from 0041b550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetHealthPerc` (Xbox PDB): the `fHealthPerc` (+0x0C) of
/// the type `0x7A` extra data (`EXTRA_HEALTH_PERC`), or `1.0` without one
/// (a `float` through the x87 stack).
pub fn extra_data_list_get_health_perc(e: &mut Engine, this: Ptr<ExtraDataList>) -> f32 {
    let extra = find_extra(e, this, EXTRA_HEALTH_PERC);
    if extra.is_null() {
        1.0
    } else {
        f32::from_bits(x87_float_bits(e.mem.f32(extra.addr() + 0x0c)))
    }
}

// Translated from 0041b580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `fHealthPerc` (+0x0C) of the type `0x7A` extra data
/// (`ExtraHealthPerc`, `0x10` bytes; the engine map has no name for it). A
/// value equal to `1.0` (compared as a `double` with the constant at
/// `01012070`) deletes the extra data (`RemoveExtra` is called even when
/// there is none); any other builds it when missing and is stored.
pub fn fn_0041b580(e: &mut Engine, this: Ptr<ExtraDataList>, health_perc: f32) {
    let mut extra = find_extra(e, this, EXTRA_HEALTH_PERC);
    if health_perc as f64 == e.global::<f64>(ONE_AS_DOUBLE) {
        remove_extra(e, this, extra);
    } else {
        if extra.is_null() {
            extra = add_extra_built_by(e, this, 0x10, fn_0041b650);
        }
        e.mem
            .set_u32(extra.addr() + 0x0c, x87_float_bits(health_perc));
    }
}

// Translated from 0041b650 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraHealthPerc` (type `0x7A`): the `BSExtraData` base
/// (`0040ec80`), the vtable at `01015178` and `fHealthPerc` (+0x0C) `0.0`.
/// Returns `this`. The engine map has no name for it.
pub fn fn_0041b650(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(BS_EXTRA_DATA_INIT, &args![this, EXTRA_HEALTH_PERC as u32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_HEALTH_PERC);
    e.mem.set_u32(this.addr() + 0x0c, 0);
    this
}

// Translated from 0041b680 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `ExtraHealthPerc` (slot 0 of the vtable
/// at `01015178`): runs the base destructor (`00432f30`), then
/// `operator delete` when bit 0 of `flags` is set. Returns `this`. The engine
/// map has no name for it.
pub fn fn_0041b680(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(EXTRA_DATA_BASE_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0041b6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetObjectHealth` (Xbox PDB): the `fHealth` (+0x0C) of the
/// type `0x56` extra data (`ExtraObjectHealth`), or the `float` at `01012054`
/// (`-1.0`) without one.
pub fn extra_data_list_get_object_health(e: &mut Engine, this: Ptr<ExtraDataList>) -> f32 {
    let extra = find_extra(e, this, EXTRA_OBJECT_HEALTH);
    let bits = if extra.is_null() {
        e.global::<u32>(NO_OBJECT_HEALTH)
    } else {
        e.mem.u32(extra.addr() + 0x0c)
    };
    f32::from_bits(x87_float_bits(f32::from_bits(bits)))
}

// Translated from 0041b6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `fHealth` (+0x0C) of the type `0x56` extra data
/// (`ExtraObjectHealth`, `0x10` bytes, built by `fn_0041b790`): stored in the
/// existing one, else a new one made from `health` is added. The engine map
/// has no name for it.
pub fn fn_0041b6e0(e: &mut Engine, this: Ptr<ExtraDataList>, health: f32) {
    let extra = find_extra(e, this, EXTRA_OBJECT_HEALTH);
    if extra.is_null() {
        add_extra_built_by(e, this, 0x10, |e, block| fn_0041b790(e, block, health));
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, x87_float_bits(health));
    }
}

// Translated from 0041b790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraObjectHealth` (type `0x56`): the `BSExtraData` base,
/// the vtable at `01015184` and `fHealth` (+0x0C) `health`. Returns `this`.
/// The engine map has no name for it.
pub fn fn_0041b790(e: &mut Engine, this: Ptr, health: f32) -> Ptr {
    e.call(BS_EXTRA_DATA_INIT, &args![this, EXTRA_OBJECT_HEALTH as u32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_OBJECT_HEALTH);
    e.mem.set_u32(this.addr() + 0x0c, x87_float_bits(health));
    this
}

// Translated from 0041b7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x56` extra data (`EXTRA_OBJECT_HEALTH`). The engine
/// map has no name for it.
pub fn fn_0041b7c0(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_OBJECT_HEALTH);
}

// Translated from 0041b7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `spCellNode` (`NiPointer`, +0x0C) of the type `0x02` extra data
/// (`ExtraCell3D`, `0x10` bytes, built by `0040ec00`). With a null `node` the
/// extra data is deleted when its pointer is already null (`00822510`), and
/// otherwise the null is assigned; without an extra data a null `node` does
/// nothing. A non-null `node` builds the extra data when missing, then is
/// assigned (`0066b0d0`, which references the node and releases the old
/// one). The engine map has no name for it.
pub fn fn_0041b7e0(e: &mut Engine, this: Ptr<ExtraDataList>, node: u32) {
    let mut extra = find_extra(e, this, EXTRA_CELL_3D);
    if node == 0 {
        if !extra.is_null()
            && e.call(NI_POINTER_EQUALS, &args![extra.addr() + 0x0c, 0u32])
                .bool()
        {
            remove_extra(e, this, extra);
            extra = Ptr::NULL;
        }
        if extra.is_null() {
            return;
        }
    }
    if extra.is_null() {
        extra = add_new_extra(e, this, 0x10, EXTRA_CELL_3D_INIT, &[]);
    }
    e.call(NI_POINTER_ASSIGN, &args![extra.addr() + 0x0c, node]);
}

// Translated from 0041b8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `spWorld` (`NiPointer`, +0x0C) of the type `0x01` extra data
/// (`ExtraHavok`, `0x14` bytes, built by `0040eda0` from the world). A null
/// `world` deletes the extra data when the list has one. Otherwise the
/// extra data is built from `world` when missing (and added, nothing more),
/// or the pointer assigned (`0066b0d0`). The engine map has no name for it.
pub fn fn_0041b8d0(e: &mut Engine, this: Ptr<ExtraDataList>, world: u32) {
    let extra = find_extra(e, this, EXTRA_HAVOK);
    if world != 0 {
        if extra.is_null() {
            add_new_extra(e, this, 0x14, EXTRA_HAVOK_INIT, &[world]);
        } else {
            e.call(NI_POINTER_ASSIGN, &args![extra.addr() + 0x0c, world]);
        }
    } else if !extra.is_null() {
        remove_extra(e, this, extra);
    }
}

// Translated from 0041b9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `spWorld` pointer (+0x0C) of the type `0x01` extra data
/// (`ExtraHavok`), or null (read through a temporary `NiPointer` made from
/// null, `get_ni_pointer_extra`). The engine map has no name for it.
pub fn fn_0041b9a0(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_ni_pointer_extra(e, this, EXTRA_HAVOK, 0x0c)
}

// Translated from 0041ba60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `spCellMopp` (`NiPointer`, +0x10) of the type `0x01` extra data
/// (`ExtraHavok`): a non-null `cell_mopp` builds the extra data from a null
/// world (`0040eda0`) when the list has none; then the pointer is assigned
/// (`0066b0d0`) in every case, a null `cell_mopp` too (without an extra data
/// that assignment is made on a null extra data plus 0x10, as the game does).
/// The engine map has no name for it.
pub fn fn_0041ba60(e: &mut Engine, this: Ptr<ExtraDataList>, cell_mopp: u32) {
    let mut extra = find_extra(e, this, EXTRA_HAVOK);
    if cell_mopp != 0 && extra.is_null() {
        extra = add_new_extra(e, this, 0x14, EXTRA_HAVOK_INIT, &[0]);
    }
    e.call(NI_POINTER_ASSIGN, &args![extra.addr() + 0x10, cell_mopp]);
}

// Translated from 0041bb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `spCellMopp` pointer (+0x10) of the type `0x01` extra data
/// (`ExtraHavok`), or null, as `fn_0041b9a0`. The engine map has no name for
/// it.
pub fn fn_0041bb10(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_ni_pointer_extra(e, this, EXTRA_HAVOK, 0x10)
}

// Translated from 0041bbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetRegionList` (Xbox PDB): sets the `pList` (+0x0C) of the
/// type `0x04` extra data (`ExtraRegionList`, `0x10` bytes, built by
/// `0040ef00`). A null `list` deletes the extra data when the list has one;
/// otherwise the extra data is built when missing. In an existing one a
/// different old list is destroyed first (slot 0 of its vtable, the scalar
/// deleting destructor, with flag 1), then `list` is stored.
pub fn extra_data_list_set_region_list(e: &mut Engine, this: Ptr<ExtraDataList>, list: u32) {
    let extra = find_extra(e, this, EXTRA_REGION_LIST);
    if list == 0 {
        if !extra.is_null() {
            remove_extra(e, this, extra);
        }
    } else if extra.is_null() {
        add_new_extra(e, this, 0x10, EXTRA_REGION_LIST_INIT, &[list]);
    } else {
        let old = e.mem.u32(extra.addr() + 0x0c);
        if old != 0 && old != list {
            e.vcall(old, 0, &args![1u32]);
        }
        e.mem.set_u32(extra.addr() + 0x0c, list);
    }
}

// Translated from 0041bce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetRegionList` (Xbox PDB): the `pList` (+0x0C) of the type
/// `0x04` extra data (`ExtraRegionList`), or null.
pub fn extra_data_list_get_region_list(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_word_extra(e, this, EXTRA_REGION_LIST)
}

// Translated from 0041bd10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `pType` (+0x0C) of the type `0x07` extra data
/// (`ExtraCellMusicType`, `0x10` bytes, built by `0040ef30`): see
/// `set_word_extra`. The engine map has no name for it.
pub fn fn_0041bd10(e: &mut Engine, this: Ptr<ExtraDataList>, music_type: u32) {
    set_word_extra(
        e,
        this,
        EXTRA_CELL_MUSIC_TYPE,
        EXTRA_CELL_MUSIC_TYPE_INIT,
        music_type,
    );
}

// Translated from 0041bde0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `pType` (+0x0C) of the type `0x07` extra data (`ExtraCellMusicType`),
/// or null. The engine map has no name for it.
pub fn fn_0041bde0(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_word_extra(e, this, EXTRA_CELL_MUSIC_TYPE)
}

// Translated from 0041be10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetTerminalState` (Xbox PDB): sets the `cState` byte
/// (+0x0C) of the type `0x50` extra data (`ExtraTerminalState`, `0x10`
/// bytes, built by `fn_0041bef0`). A non-zero `state` builds the extra data
/// when missing and is stored. Zero clears the byte of an existing extra
/// data and deletes it when it holds nothing else (`fn_0041bf20`); without
/// one it does nothing.
pub fn extra_data_list_set_terminal_state(e: &mut Engine, this: Ptr<ExtraDataList>, state: u8) {
    let mut extra = find_extra(e, this, EXTRA_TERMINAL_STATE);
    if state == 0 {
        if !extra.is_null() {
            e.mem.set_u8(extra.addr() + 0x0c, 0);
            if fn_0041bf20(e, extra.cast()) {
                remove_extra(e, this, extra);
            }
        }
    } else {
        if extra.is_null() {
            extra = add_extra_built_by(e, this, 0x10, fn_0041bef0);
        }
        e.mem.set_u8(extra.addr() + 0x0c, state);
    }
}

// Translated from 0041bef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraTerminalState` (type `0x50`): the `BSExtraData` base,
/// the vtable at `01015190`, `cState` (+0x0C) 0 and `cLockOverride` (+0x0D)
/// `0xFF`. Returns `this`. The engine map has no name for it.
pub fn fn_0041bef0(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(
        BS_EXTRA_DATA_INIT,
        &args![this, EXTRA_TERMINAL_STATE as u32],
    );
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_TERMINAL_STATE);
    e.mem.set_u8(this.addr() + 0x0c, 0);
    e.mem.set_u8(this.addr() + 0x0d, 0xff);
    this
}

// Translated from 0041bf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether an `ExtraTerminalState` holds nothing: `cState` (+0x0C) is 0 and
/// `cLockOverride` (+0x0D, a signed byte) is `-1`. The engine map has no name
/// for it.
pub fn fn_0041bf20(e: &mut Engine, this: Ptr) -> bool {
    e.mem.u8(this.addr() + 0x0c) == 0 && e.mem.u8(this.addr() + 0x0d) as i8 == -1
}

// Translated from 0041bf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetTerminalState` (Xbox PDB): the `cState` byte (+0x0C) of
/// the type `0x50` extra data (`ExtraTerminalState`), or 0.
pub fn extra_data_list_get_terminal_state(e: &mut Engine, this: Ptr<ExtraDataList>) -> u8 {
    let extra = find_extra(e, this, EXTRA_TERMINAL_STATE);
    if extra.is_null() {
        0
    } else {
        e.mem.u8(extra.addr() + 0x0c)
    }
}

// Translated from 0041bf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `cLockOverride` byte (+0x0D) of the type `0x50` extra data
/// (`ExtraTerminalState`), as `ExtraDataList::SetTerminalState` does for
/// `cState`: a non-zero `lock_override` builds the extra data when missing and
/// is stored; zero stores 0 in an existing one and deletes it when it holds
/// nothing else (`fn_0041bf20`). The engine map has no name for it.
pub fn fn_0041bf80(e: &mut Engine, this: Ptr<ExtraDataList>, lock_override: u8) {
    let mut extra = find_extra(e, this, EXTRA_TERMINAL_STATE);
    if lock_override as i8 == 0 {
        if !extra.is_null() {
            e.mem.set_u8(extra.addr() + 0x0d, 0);
            if fn_0041bf20(e, extra.cast()) {
                remove_extra(e, this, extra);
            }
        }
    } else {
        if extra.is_null() {
            extra = add_extra_built_by(e, this, 0x10, fn_0041bef0);
        }
        e.mem.set_u8(extra.addr() + 0x0d, lock_override);
    }
}

// Translated from 0041c060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `cLockOverride` byte (+0x0D) of the type `0x50` extra data
/// (`ExtraTerminalState`), or `0xFF` without one. The engine map has no name
/// for it.
pub fn fn_0041c060(e: &mut Engine, this: Ptr<ExtraDataList>) -> u8 {
    let extra = find_extra(e, this, EXTRA_TERMINAL_STATE);
    if extra.is_null() {
        0xff
    } else {
        e.mem.u8(extra.addr() + 0x0d)
    }
}

// Translated from 0041c090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `pSpace` (+0x0C) of the type `0x81` extra data
/// (`ExtraCellAcousticSpace`, `0x10` bytes, built by `0040efd0`): see
/// `set_word_extra`. The engine map names this `ExtraDataList::SetPoison`;
/// the body works on the acoustic space extra data (type `0x81`, the one
/// `ExtraDataList::GetAcousticSpace` reads), so it is `fn_0041c090` here.
pub fn fn_0041c090(e: &mut Engine, this: Ptr<ExtraDataList>, space: u32) {
    set_word_extra(
        e,
        this,
        EXTRA_CELL_ACOUSTIC_SPACE,
        EXTRA_CELL_ACOUSTIC_SPACE_INIT,
        space,
    );
}

// Translated from 0041c160 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetAcousticSpace` (Xbox PDB): the `pSpace` (+0x0C) of the
/// type `0x81` extra data (`ExtraCellAcousticSpace`), or null.
pub fn extra_data_list_get_acoustic_space(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_word_extra(e, this, EXTRA_CELL_ACOUSTIC_SPACE)
}

// Translated from 0041c190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `pClimate` (+0x0C) of the type `0x08` extra data
/// (`ExtraCellClimate`, `0x10` bytes, built by `0040f070`): see
/// `set_word_extra`. The engine map has no name for it.
pub fn fn_0041c190(e: &mut Engine, this: Ptr<ExtraDataList>, climate: u32) {
    set_word_extra(e, this, EXTRA_CLIMATE, EXTRA_CLIMATE_INIT, climate);
}

// Translated from 0041c260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `pClimate` (+0x0C) of the type `0x08` extra data (`ExtraCellClimate`),
/// or null. The engine map has no name for it.
pub fn fn_0041c260(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_word_extra(e, this, EXTRA_CLIMATE)
}

// Translated from 0041c290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `pImageSpace` (+0x0C) of the type `0x59` extra data
/// (`ExtraCellImageSpace`, `0x10` bytes, built by `0040f110`): see
/// `set_word_extra`. The engine map has no name for it.
pub fn fn_0041c290(e: &mut Engine, this: Ptr<ExtraDataList>, image_space: u32) {
    set_word_extra(
        e,
        this,
        EXTRA_IMAGE_SPACE,
        EXTRA_IMAGE_SPACE_INIT,
        image_space,
    );
}

// Translated from 0041c360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `pImageSpace` (+0x0C) of the type `0x59` extra data
/// (`ExtraCellImageSpace`), or null. The engine map has no name for it.
pub fn fn_0041c360(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_word_extra(e, this, EXTRA_IMAGE_SPACE)
}

// Translated from 0041c390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `pImpactSwap` (+0x0C) of the type `0x8C` extra data
/// (`ExtraCellImpactSwap`, `0x10` bytes, built by `0040f1b0`, which takes no
/// payload: the value is stored after the construction, before the extra
/// data is added). A null `impact_swap` deletes the extra data when the list
/// has one. The engine map has no name for it.
pub fn fn_0041c390(e: &mut Engine, this: Ptr<ExtraDataList>, impact_swap: u32) {
    let extra = find_extra(e, this, EXTRA_IMPACT_SWAP);
    if impact_swap == 0 {
        if !extra.is_null() {
            remove_extra(e, this, extra);
        }
    } else if extra.is_null() {
        let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
        let built = if block == 0 {
            0
        } else {
            e.call(EXTRA_IMPACT_SWAP_INIT, &args![block]).u32()
        };
        // A failed allocation stores through null + 0x0C, as the game does.
        e.mem.set_u32(built + 0x0c, impact_swap);
        add_extra(e, this, built);
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, impact_swap);
    }
}

// Translated from 0041c460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `pImpactSwap` (+0x0C) of the type `0x8C` extra data
/// (`ExtraCellImpactSwap`), or null. The engine map has no name for it.
pub fn fn_0041c460(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_word_extra(e, this, EXTRA_IMPACT_SWAP)
}

// Translated from 0041c490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetCanopyShadowMask` (Xbox PDB): the type `0x0A` extra data
/// (`ExtraCellCanopyShadowMask`, `0x1C` bytes: `eState` +0x0C, the
/// `spMask` `NiPointer` +0x10 and the `_D3DLOCKED_RECT` +0x14). A zero
/// `state` deletes the extra data when the list has one (and writes nothing
/// to `rect_out`). Otherwise the extra data is built from `state` and `mask`
/// (`0040f440`) when missing, or `state` is stored and `mask` assigned to
/// the `NiPointer` (`0066b0d0`); `rect_out` receives the address of the
/// extra data's rect (+0x14).
pub fn extra_data_list_set_canopy_shadow_mask(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    state: u32,
    mask: u32,
    rect_out: Ptr,
) {
    let mut extra = find_extra(e, this, EXTRA_CELL_CANOPY_SHADOW_MASK);
    if state == 0 {
        if !extra.is_null() {
            remove_extra(e, this, extra);
        }
        return;
    }
    if extra.is_null() {
        extra = add_new_extra(
            e,
            this,
            0x1c,
            EXTRA_CELL_CANOPY_SHADOW_MASK_INIT,
            &[state, mask],
        );
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, state);
        e.call(NI_POINTER_ASSIGN, &args![extra.addr() + 0x10, mask]);
    }
    e.mem.set_u32(rect_out.addr(), extra.addr() + 0x14);
}

// Translated from 0041c580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the type `0x0A` extra data (`ExtraCellCanopyShadowMask`): stores 0
/// in `mask_out` and `rect_out`, and with the extra data stores the mask
/// pointer (`spMask`, +0x10, read through `00559450`) and the address of the
/// rect (+0x14) there. Returns `eState` (+0x0C), 0 without the extra data.
/// The engine map has no name for it.
pub fn fn_0041c580(e: &mut Engine, this: Ptr<ExtraDataList>, mask_out: Ptr, rect_out: Ptr) -> u32 {
    let extra = find_extra(e, this, EXTRA_CELL_CANOPY_SHADOW_MASK);
    e.mem.set_u32(mask_out.addr(), 0);
    e.mem.set_u32(rect_out.addr(), 0);
    if extra.is_null() {
        return 0;
    }
    let mask = e.call(NI_POINTER_GET, &args![extra.addr() + 0x10]).u32();
    e.mem.set_u32(mask_out.addr(), mask);
    e.mem.set_u32(rect_out.addr(), extra.addr() + 0x14);
    e.mem.u32(extra.addr() + 0x0c)
}

// Translated from 0041c5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `ExtraEditorID` (type `0x06`, `0x14` bytes): the
/// `BSExtraData` base, the vtable at `0101519c`, then the `cEditorID`
/// `BSStringT<char>` (+0x0C) made empty (`004037b0`) and set from `editor_id`
/// (`004037f0`). Returns `this`. The engine map has no name for it.
pub fn fn_0041c5e0(e: &mut Engine, this: Ptr, editor_id: u32) -> Ptr {
    e.call(BS_EXTRA_DATA_INIT, &args![this, EXTRA_EDITOR_ID as u32]);
    e.mem.set_u32(this.addr(), VTABLE_EXTRA_EDITOR_ID);
    e.call(BS_STRING_INIT, &args![this.addr() + 0x0c]);
    e.call(BS_STRING_SET, &args![this.addr() + 0x0c, editor_id, 0u32]);
    this
}

// Translated from 0041c660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `ExtraEditorID` (slot 0 of the vtable at
/// `0101519c`): runs `fn_0041c690`, then `operator delete` when bit 0 of
/// `flags` is set. Returns `this`. The engine map has no name for it.
pub fn fn_0041c660(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0041c690(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 0041c690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `ExtraEditorID`: destroys the `cEditorID` `BSStringT`
/// (+0x0C, `004037d0`), then the `BSExtraData` base (`0040ecb0`). The engine
/// map names it as a `std::_Ref_count_del_alloc` destructor (folded code);
/// the body is the editor id's.
pub fn fn_0041c690(e: &mut Engine, this: Ptr) {
    e.call(BS_STRING_DESTROY, &args![this.addr() + 0x0c]);
    e.call(BS_EXTRA_DATA_DESTROY, &args![this]);
}

// Translated from 0041c6f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `cEditorID` string (+0x0C) of the type `0x06` extra data
/// (`ExtraEditorID`, `0x14` bytes, built by `fn_0041c5e0`). A null
/// `editor_id` deletes the extra data when the list has one; otherwise the
/// extra data is built from the string when missing, or the string is set in
/// it (`004037f0`). The engine map has no name for it.
pub fn fn_0041c6f0(e: &mut Engine, this: Ptr<ExtraDataList>, editor_id: u32) {
    let extra = find_extra(e, this, EXTRA_EDITOR_ID);
    if editor_id == 0 {
        if !extra.is_null() {
            remove_extra(e, this, extra);
        }
    } else if extra.is_null() {
        add_extra_built_by(e, this, 0x14, |e, block| fn_0041c5e0(e, block, editor_id));
    } else {
        e.call(BS_STRING_SET, &args![extra.addr() + 0x0c, editor_id, 0u32]);
    }
}

// Translated from 0041c7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The editor id string pointer of the type `0x06` extra data
/// (`ExtraEditorID`): the first word of its `BSStringT` (+0x0C, read through
/// `00559450`), or null. The engine map has no name for it.
pub fn fn_0041c7c0(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_EDITOR_ID);
    if extra.is_null() {
        0
    } else {
        e.call(NI_POINTER_GET, &args![extra.addr() + 0x0c]).u32()
    }
}

// Translated from 0041c7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `pRef` (+0x0C) of the type `0x1C` extra data
/// (`ExtraReferencePointer`, `0x10` bytes, built by `004327e0`). Acts only
/// when the save/load singleton's stub (`0047c850`, always false) says so, or
/// `reference` is null, or the reference persists
/// (`TESObjectREFR::GetRefPersists`); otherwise it does nothing. Then the
/// extra data is built from `reference` when missing, or `reference` is
/// stored. The engine map has no name for it.
pub fn fn_0041c7f0(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) {
    let save_load_game = e.global::<u32>(SAVE_LOAD_GAME);
    let stub = e.call(SAVE_LOAD_GAME_STUB, &args![save_load_game]).bool();
    if !stub && reference != 0 && !e.call(GET_REF_PERSISTS, &args![reference]).bool() {
        return;
    }
    let extra = find_extra(e, this, EXTRA_REFERENCE_POINTER);
    if extra.is_null() {
        add_new_extra(e, this, 0x10, EXTRA_REFERENCE_POINTER_INIT, &[reference]);
    } else {
        e.mem.set_u32(extra.addr() + 0x0c, reference);
    }
}

/// Deletes the list's first extra data of `extra_type` when it has one
/// (`GetExtraData`, then `RemoveExtra(extra, true)`).
fn remove_found_extra(e: &mut Engine, list: Ptr<ExtraDataList>, extra_type: u8) {
    let extra = find_extra(e, list, extra_type);
    if !extra.is_null() {
        remove_extra(e, list, extra);
    }
}

// Translated from 0041c8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetReferencePointer` (Xbox PDB): the `pRef` (+0x0C) of the
/// type `0x1C` extra data (`ExtraReferencePointer`), or null.
pub fn extra_data_list_get_reference_pointer(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_word_extra(e, this, EXTRA_REFERENCE_POINTER)
}

// Translated from 0041c900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x1C` extra data (`ExtraReferencePointer`) when the list
/// has one. The engine map has no name for it.
pub fn fn_0041c900(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_found_extra(e, this, EXTRA_REFERENCE_POINTER);
}

// Translated from 0041c930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetPackageExtra` (Xbox PDB): sets the type `0x19` extra
/// data (`ExtraPackage`, `0x1C` bytes). A package that is created
/// (`TESPackage::GetIsCreated`) of a type other than 1, that never runs
/// (`IsNeverToRun`) or whose type (`fn_0041ca90`) is `0x18` or `0x17` is
/// ignored. Otherwise a null `package` deletes the extra data when the list
/// has one; any other builds it from the six values when missing, or stores
/// them (`pPack`, `iindex`, `pTarg` and the three flag bytes) in it.
#[allow(clippy::too_many_arguments)]
pub fn extra_data_list_set_package_extra(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    package: Ptr,
    index: i32,
    target: Ptr,
    action_complete: u8,
    activated: u8,
    done_once: u8,
) {
    if !package.is_null() {
        let created = e.call(PACKAGE_GET_IS_CREATED, &args![package]).bool();
        if created && fn_0041ca90(e, package) != 1 {
            return;
        }
        if e.call(PACKAGE_IS_NEVER_TO_RUN, &args![package]).bool()
            || fn_0041ca90(e, package) == 0x18
            || fn_0041ca90(e, package) == 0x17
        {
            return;
        }
    }
    let extra = find_extra(e, this, EXTRA_PACKAGE);
    if package.is_null() {
        if !extra.is_null() {
            extra_data_list_remove_package_extra(e, this);
        }
    } else if extra.is_null() {
        add_new_extra(
            e,
            this,
            0x1c,
            EXTRA_PACKAGE_INIT,
            &[
                package.addr(),
                index as u32,
                target.addr(),
                action_complete as u32,
                activated as u32,
                done_once as u32,
            ],
        );
    } else {
        let extra: Ptr<ExtraPackage> = extra.cast();
        e.set(extra, ExtraPackage::pPack, package);
        e.set(extra, ExtraPackage::iindex, index);
        e.set(extra, ExtraPackage::pTarg, target);
        e.set(extra, ExtraPackage::bActionComplete, action_complete);
        e.set(extra, ExtraPackage::bActivated, activated);
        e.set(extra, ExtraPackage::bDoneOnce, done_once);
    }
}

// Translated from 0041ca90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The signed byte at +0x20 of a `TESPackage` (the value `SetPackageExtra`
/// compares with 1, `0x17` and `0x18`), as an `i32`. The engine map has no
/// name for it.
pub fn fn_0041ca90(e: &mut Engine, this: Ptr) -> i32 {
    e.mem.u8(this.addr() + 0x20) as i8 as i32
}

// Translated from 0041cab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetPackageExtraTarget` (Xbox PDB): stores `target` in the
/// `pTarg` (+0x14) of the type `0x19` extra data when the list has one.
pub fn extra_data_list_set_package_extra_target(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    target: Ptr,
) {
    let extra = find_extra(e, this, EXTRA_PACKAGE);
    if !extra.is_null() {
        e.set(extra.cast::<ExtraPackage>(), ExtraPackage::pTarg, target);
    }
}

// Translated from 0041cae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetPackageExtraIndex` (Xbox PDB): stores `index` in the
/// `iindex` (+0x10) of the type `0x19` extra data when the list has one.
pub fn extra_data_list_set_package_extra_index(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    index: i32,
) {
    let extra = find_extra(e, this, EXTRA_PACKAGE);
    if !extra.is_null() {
        e.set(extra.cast::<ExtraPackage>(), ExtraPackage::iindex, index);
    }
}

// Translated from 0041cb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetPackageExtra` (Xbox PDB): the `pPack` (+0x0C) of the
/// type `0x19` extra data, or null.
pub fn extra_data_list_get_package_extra(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_word_extra(e, this, EXTRA_PACKAGE)
}

// Translated from 0041cb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetPackageExtraIndex` (Xbox PDB): the `iindex` (+0x10) of
/// the type `0x19` extra data, or 0.
pub fn extra_data_list_get_package_extra_index(e: &mut Engine, this: Ptr<ExtraDataList>) -> i32 {
    let extra = find_extra(e, this, EXTRA_PACKAGE);
    if extra.is_null() {
        0
    } else {
        e.get(extra.cast::<ExtraPackage>(), ExtraPackage::iindex)
    }
}

// Translated from 0041cb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetPackageExtraTarget` (Xbox PDB): the `pTarg` (+0x14) of
/// the type `0x19` extra data, or null.
pub fn extra_data_list_get_package_extra_target(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    let extra = find_extra(e, this, EXTRA_PACKAGE);
    if extra.is_null() {
        0
    } else {
        e.get(extra.cast::<ExtraPackage>(), ExtraPackage::pTarg)
            .addr()
    }
}

// Translated from 0041cba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bActionComplete` byte (+0x18) of the type `0x19` extra data, or 0.
/// The engine map has no name for it (its setter is
/// `ExtraDataList::SetPackageExtraActionComplete`).
pub fn fn_0041cba0(e: &mut Engine, this: Ptr<ExtraDataList>) -> u8 {
    let extra = find_extra(e, this, EXTRA_PACKAGE);
    if extra.is_null() {
        0
    } else {
        e.get(extra.cast::<ExtraPackage>(), ExtraPackage::bActionComplete)
    }
}

// Translated from 0041cbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetPackageExtraActionComplete` (Xbox PDB): stores the byte
/// in the `bActionComplete` (+0x18) of the type `0x19` extra data when the
/// list has one.
pub fn extra_data_list_set_package_extra_action_complete(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    action_complete: u8,
) {
    let extra = find_extra(e, this, EXTRA_PACKAGE);
    if !extra.is_null() {
        e.set(
            extra.cast::<ExtraPackage>(),
            ExtraPackage::bActionComplete,
            action_complete,
        );
    }
}

// Translated from 0041cc00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bActivated` byte (+0x19) of the type `0x19` extra data, or 0. The
/// engine map has no name for it.
pub fn fn_0041cc00(e: &mut Engine, this: Ptr<ExtraDataList>) -> u8 {
    let extra = find_extra(e, this, EXTRA_PACKAGE);
    if extra.is_null() {
        0
    } else {
        e.get(extra.cast::<ExtraPackage>(), ExtraPackage::bActivated)
    }
}

// Translated from 0041cc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `bDoneOnce` byte (+0x1A) of the type `0x19` extra data, or 0. The
/// engine map has no name for it.
pub fn fn_0041cc30(e: &mut Engine, this: Ptr<ExtraDataList>) -> u8 {
    let extra = find_extra(e, this, EXTRA_PACKAGE);
    if extra.is_null() {
        0
    } else {
        e.get(extra.cast::<ExtraPackage>(), ExtraPackage::bDoneOnce)
    }
}

// Translated from 0041cc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemovePackageExtra` (Xbox PDB): deletes the type `0x19`
/// extra data when the list has one.
pub fn extra_data_list_remove_package_extra(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_found_extra(e, this, EXTRA_PACKAGE);
}

// Translated from 0041cc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `pPack` (+0x0C) of the type `0x1A` extra data
/// (`ExtraTresPassPackage`, `0x10` bytes, built by `004328d0`). Without the
/// extra data it is built from `package` (and added, even when the
/// allocation failed). With it, the package it holds is destroyed first
/// (virtual slot `0x10` of that package, with `1`) when not null, then
/// `package` is stored. The engine map has no name for it.
pub fn fn_0041cc90(e: &mut Engine, this: Ptr<ExtraDataList>, package: Ptr) {
    let extra = find_extra(e, this, EXTRA_TRESPASS_PACKAGE);
    if extra.is_null() {
        add_new_extra(
            e,
            this,
            0x10,
            EXTRA_TRESPASS_PACKAGE_INIT,
            &[package.addr()],
        );
        return;
    }
    let extra: Ptr<ExtraTresPassPackage> = extra.cast();
    let old = e.get(extra, ExtraTresPassPackage::pPack);
    if !old.is_null() {
        e.vcall(old.addr(), 0x10, &args![1u32]);
    }
    e.set(extra, ExtraTresPassPackage::pPack, package);
}

// Translated from 0041cd70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `pPack` (+0x0C) of the type `0x1A` extra data
/// (`ExtraTresPassPackage`), or null. The engine map has no name for it.
pub fn fn_0041cd70(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_word_extra(e, this, EXTRA_TRESPASS_PACKAGE)
}

// Translated from 0041cda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemoveTrespassPackage` (Xbox PDB): with the type `0x1A`
/// extra data in the list, marks its package as created
/// (`TESPackage::SetIsCreated(1)`), deletes the form through
/// `TESSaveLoadGame::DeleteForm` when the save/load singleton's stub
/// (`0047c850`, always false) says so, clears the package pointer and
/// deletes the extra data.
pub fn extra_data_list_remove_trespass_package(e: &mut Engine, this: Ptr<ExtraDataList>) {
    let extra = find_extra(e, this, EXTRA_TRESPASS_PACKAGE);
    if extra.is_null() {
        return;
    }
    let extra: Ptr<ExtraTresPassPackage> = extra.cast();
    let package = e.get(extra, ExtraTresPassPackage::pPack);
    e.call(PACKAGE_SET_IS_CREATED, &args![package, 1u32]);
    let save_load_game = e.global::<u32>(SAVE_LOAD_GAME);
    if e.call(SAVE_LOAD_GAME_STUB, &args![save_load_game]).bool() {
        let package = e.get(extra, ExtraTresPassPackage::pPack);
        e.call(SAVE_LOAD_GAME_DELETE_FORM, &args![save_load_game, package]);
    }
    e.set(extra, ExtraTresPassPackage::pPack, Ptr::new(0));
    remove_extra(e, this, extra.cast());
}

// Translated from 0041ce10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the `pPack` (+0x0C) of the type `0x1A` extra data when the list has
/// one and it holds `package`. The engine map has no name for it.
pub fn fn_0041ce10(e: &mut Engine, this: Ptr<ExtraDataList>, package: Ptr) {
    let extra = find_extra(e, this, EXTRA_TRESPASS_PACKAGE);
    if extra.is_null() {
        return;
    }
    let extra: Ptr<ExtraTresPassPackage> = extra.cast();
    if e.get(extra, ExtraTresPassPackage::pPack).addr() == package.addr() {
        e.set(extra, ExtraTresPassPackage::pPack, Ptr::new(0));
    }
}

// Translated from 0041ce50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::AddToPlayerCrimeList` (Xbox PDB): without the type `0x35`
/// extra data (`ExtraPlayerCrimeList`, `0x10` bytes) builds it from `crime`
/// (`00432aa0`); with it adds `crime` at the head of its `BSSimpleList`
/// (`pCrime`, +0x0C; `005ae3d0`, given the address of a stack word holding
/// `crime`).
pub fn extra_data_list_add_to_player_crime_list(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    crime: u32,
) {
    let extra = find_extra(e, this, EXTRA_PLAYER_CRIME_LIST);
    if extra.is_null() {
        add_new_extra(e, this, 0x10, EXTRA_PLAYER_CRIME_LIST_INIT, &[crime]);
        return;
    }
    let crimes = e.mem.u32(extra.addr() + 0x0c);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), crime);
        e.call(LIST_ADD_HEAD, &args![crimes, slot]);
    });
}

// Translated from 0041cf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetPlayerCrimeList` (Xbox PDB): the `pCrime` (+0x0C) of the
/// type `0x35` extra data, or null.
pub fn extra_data_list_get_player_crime_list(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_word_extra(e, this, EXTRA_PLAYER_CRIME_LIST)
}

// Translated from 0041cf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::RemovePlayerCrimeListExtra` (Xbox PDB): with the type
/// `0x35` extra data in the list, and a crime list in it, runs
/// `009eba00` with `value` on every crime of the list up to the first empty
/// item, frees the list's nodes (`00470470`), deletes the list
/// (`004702f0`, flags 1) and clears `pCrime`; then deletes the extra data.
pub fn extra_data_list_remove_player_crime_list_extra(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    value: u32,
) {
    let extra = find_extra(e, this, EXTRA_PLAYER_CRIME_LIST);
    if extra.is_null() {
        return;
    }
    if e.mem.u32(extra.addr() + 0x0c) != 0 {
        let mut node = e.mem.u32(extra.addr() + 0x0c);
        while node != 0 {
            let slot = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
            let crime = e.mem.u32(slot);
            e.call(CRIME_REMOVE_ENTRY, &args![crime, value]);
            node = e.call(SIMPLE_LIST_NEXT, &args![node]).u32();
        }
        let crimes = e.mem.u32(extra.addr() + 0x0c);
        e.call(SIMPLE_LIST_FREE_NODES, &args![crimes]);
        let crimes = e.mem.u32(extra.addr() + 0x0c);
        if crimes != 0 {
            e.call(SIMPLE_LIST_SCALAR_DELETING_DESTRUCTOR, &args![crimes, 1u32]);
        }
        e.mem.set_u32(extra.addr() + 0x0c, 0);
    }
    remove_extra(e, this, extra);
}

// Translated from 0041d000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the type `0x2F` extra data (`ExtraLeveledItem`) when the list has
/// one. The engine map has no name for it.
pub fn fn_0041d000(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_found_extra(e, this, EXTRA_LEVELED_ITEM);
}

// Translated from 0041d030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scans the list under the list lock: true when it holds an ownership extra
/// data (type `0x21`) and every other extra data is one of the types `0x0D`
/// (script), `0x1C` (reference pointer), `0x20` (original reference), `0x24`
/// (count), `0x27` (time left), `0x2F` (leveled item), `0x30` (scale) or
/// `0x4A` (hot key). Any other type ends the scan with false. The engine map
/// has no name for it.
pub fn fn_0041d030(e: &mut Engine, this: Ptr<ExtraDataList>) -> bool {
    lock(e, 0);
    let mut owned = false;
    let mut node: Ptr<BSExtraData> = e.get(this, ExtraDataList::pHead).cast();
    while !node.is_null() {
        match get_type(e, node) {
            0x0d | 0x1c | 0x20 | 0x24 | 0x27 | 0x2f | 0x30 | 0x4a => {}
            0x21 => owned = true,
            _ => {
                unlock(e);
                return false;
            }
        }
        node = get_next(e, node);
    }
    unlock(e);
    owned
}

// Translated from 0041d120 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::IsExtraDefaultforContainer` (Xbox PDB): scans the list
/// under the list lock and answers whether every extra data is one of the
/// types `0x0D`, `0x1C`, `0x20`, `0x21` (ownership), `0x24`, `0x27`, `0x2F`,
/// `0x30` and `0x4A`; a non-zero `worn_allowed` also allows `0x16` (worn).
/// Any other type ends the scan with false.
pub fn extra_data_list_is_extra_defaultfor_container(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    worn_allowed: u8,
) -> bool {
    lock(e, 0);
    let mut node: Ptr<BSExtraData> = e.get(this, ExtraDataList::pHead).cast();
    while !node.is_null() {
        let extra_type = get_type(e, node);
        let allowed = matches!(
            extra_type,
            0x0d | 0x1c | 0x20 | 0x21 | 0x24 | 0x27 | 0x2f | 0x30 | 0x4a
        ) || (worn_allowed != 0 && extra_type == 0x16);
        if !allowed {
            unlock(e);
            return false;
        }
        node = get_next(e, node);
    }
    unlock(e);
    true
}

// Translated from 0041d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `iIndex` (+0x0C) of the type `0x2F` extra data
/// (`ExtraLeveledItem`, `0x14` bytes, built by `00432be0` from the index) and
/// clears its `bdefault` byte (+0x10). The extra data is built and added
/// when the list has none. The engine map has no name for it.
pub fn fn_0041d280(e: &mut Engine, this: Ptr<ExtraDataList>, index: i32) {
    let mut extra = find_extra(e, this, EXTRA_LEVELED_ITEM);
    if extra.is_null() {
        extra = add_new_extra(e, this, 0x14, EXTRA_LEVELED_ITEM_INIT, &[index as u32]);
    } else {
        e.set(
            extra.cast::<ExtraLeveledItem>(),
            ExtraLeveledItem::iIndex,
            index,
        );
    }
    e.set(
        extra.cast::<ExtraLeveledItem>(),
        ExtraLeveledItem::bdefault,
        false,
    );
}

// Translated from 0041d330 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the byte in the `bdefault` (+0x10) of the type `0x2F` extra data
/// (`ExtraLeveledItem`) when the list has one. The engine map has no name for
/// it.
pub fn fn_0041d330(e: &mut Engine, this: Ptr<ExtraDataList>, default_flag: u8) {
    let extra = find_extra(e, this, EXTRA_LEVELED_ITEM);
    if !extra.is_null() {
        e.mem.set_u8(extra.addr() + 0x10, default_flag);
    }
}

// Translated from 0041d360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `iIndex` (+0x0C) of the type `0x2F` extra data (`ExtraLeveledItem`),
/// or -1 without it. The engine map has no name for it.
pub fn fn_0041d360(e: &mut Engine, this: Ptr<ExtraDataList>) -> i32 {
    let extra = find_extra(e, this, EXTRA_LEVELED_ITEM);
    if extra.is_null() {
        -1
    } else {
        e.get(extra.cast::<ExtraLeveledItem>(), ExtraLeveledItem::iIndex)
    }
}

// Translated from 0041d390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetPersistentCell` (Xbox PDB): sets the `pPersistentCell`
/// (+0x0C) of the type `0x0C` extra data (`ExtraPersistentCell`, `0x10`
/// bytes, built by `00432c20`): a null `cell` deletes the extra data when
/// the list has one, any other builds it when missing or is stored in it.
pub fn extra_data_list_set_persistent_cell(e: &mut Engine, this: Ptr<ExtraDataList>, cell: u32) {
    set_word_extra(
        e,
        this,
        EXTRA_PERSISTENT_CELL,
        EXTRA_PERSISTENT_CELL_INIT,
        cell,
    );
}

// Translated from 0041d460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `pPersistentCell` (+0x0C) of the type `0x0C` extra data, or null. The
/// engine map has no name for it.
pub fn fn_0041d460(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_word_extra(e, this, EXTRA_PERSISTENT_CELL)
}

/// The body of the two ragdoll setters: `fill` is the `RagDollData` method
/// that takes the source (`source`, a reference or another `RagDollData`)
/// and fills the data. Without the type `0x14` extra data a non-null `source`
/// builds an `ExtraRagDollData` (`0x10` bytes, `00432cb0`) holding a new
/// `RagDollData` (`0x14` bytes, `004d9330`), fills it and adds the extra
/// data; with it, a null `source` deletes the extra data and any other fills
/// the data it holds.
fn set_rag_doll_extra(e: &mut Engine, list: Ptr<ExtraDataList>, source: u32, fill: u32) {
    let extra = find_extra(e, list, EXTRA_RAG_DOLL_DATA);
    if !extra.is_null() {
        if source == 0 {
            remove_extra(e, list, extra);
        } else {
            let data = e.get(
                extra.cast::<ExtraRagDollData>(),
                ExtraRagDollData::pRagDollData,
            );
            e.call(fill, &args![data, source]);
        }
        return;
    }
    if source == 0 {
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0x10u32]).u32();
    let extra: Ptr<ExtraRagDollData> = Ptr::new(if block == 0 {
        0
    } else {
        e.call(EXTRA_RAGDOLL_DATA_INIT, &args![block]).u32()
    });
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    let data = if block == 0 {
        0
    } else {
        e.call(RAGDOLL_DATA_INIT, &args![block]).u32()
    };
    e.set(extra, ExtraRagDollData::pRagDollData, Ptr::new(data));
    let data = e.get(extra, ExtraRagDollData::pRagDollData);
    e.call(fill, &args![data, source]);
    add_extra(e, list, extra.addr());
}

// Translated from 0041d490 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetRagDollData` (Xbox PDB): see `set_rag_doll_extra`, with
/// `RagDollData::UpdateDataFromReference` (`004d9800`) filling the data from
/// `reference`.
pub fn extra_data_list_set_rag_doll_data(e: &mut Engine, this: Ptr<ExtraDataList>, reference: u32) {
    set_rag_doll_extra(e, this, reference, RAG_DOLL_UPDATE_FROM_REFERENCE);
}

// Translated from 0041d5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::SetRagDollDataFromCopy` (Xbox PDB): see
/// `set_rag_doll_extra`, with `RagDollData::Copy` (`004d9670`) copying
/// `source`.
pub fn extra_data_list_set_rag_doll_data_from_copy(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    source: u32,
) {
    set_rag_doll_extra(e, this, source, RAG_DOLL_COPY);
}

// Translated from 0041d6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::GetRagDollData` (Xbox PDB): the `pRagDollData` (+0x0C) of
/// the type `0x14` extra data, or null.
pub fn extra_data_list_get_rag_doll_data(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_word_extra(e, this, EXTRA_RAG_DOLL_DATA)
}

// Translated from 0041d700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds a package to the type `0x1B` extra data (`ExtraRunOncePacks`, `0x10`
/// bytes, built by `00433010`; made and added when the list has none): calls
/// `004331c0` on the extra data with `package` and `flag`. The engine map has
/// no name for it.
pub fn fn_0041d700(e: &mut Engine, this: Ptr<ExtraDataList>, package: u32, flag: u8) {
    let mut extra = find_extra(e, this, EXTRA_RUN_ONCE_PACKAGES);
    if extra.is_null() {
        extra = add_new_extra(e, this, 0x10, EXTRA_RUN_ONCE_PACKAGES_INIT, &[]);
    }
    e.call(RUN_ONCE_PACKAGES_ADD, &args![extra, package, flag as u32]);
}

// Translated from 0041d7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Walks the `pPackageList` (+0x0C) of the type `0x1B` extra data
/// (`ExtraRunOncePacks`) up to the first empty item. Each item is a run-once
/// package entry: a pointer to a package (word at +0) and a signed byte at
/// +4. An entry is expired when its package is not null and the byte at +3
/// of the time stamp at package +0x38 (`fn_0041d8a0`, read through
/// `0086a460`) plus the word at +4 of it (`00726070`) is at least `0x15`. An
/// entry whose byte equals `flag` and is not expired is kept; any other is
/// removed from the list (`00905330`) and deleted. The engine map has no
/// name for it.
pub fn fn_0041d7b0(e: &mut Engine, this: Ptr<ExtraDataList>, flag: i8) {
    let extra = find_extra(e, this, EXTRA_RUN_ONCE_PACKAGES);
    if extra.is_null() {
        return;
    }
    let extra: Ptr<ExtraRunOncePacks> = extra.cast();
    let mut node = e.get(extra, ExtraRunOncePacks::pPackageList).addr();
    while node != 0 {
        let slot = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        let mut expired = false;
        if entry != 0 && e.mem.u32(entry) != 0 {
            let package = e.mem.u32(entry);
            let time_stamp = fn_0041d8a0(e, Ptr::new(package)).addr();
            let high_byte = e.call(TIME_STAMP_BYTE, &args![time_stamp]).u8() as i8 as i32;
            let word = e.call(SIMPLE_LIST_NEXT, &args![time_stamp]).u32() as i32;
            if high_byte.wrapping_add(word) >= 0x15 {
                expired = true;
            }
        }
        if e.mem.u8(entry + 4) as i8 == flag && !expired {
            node = e.call(SIMPLE_LIST_NEXT, &args![node]).u32();
        } else {
            let packages = e.get(extra, ExtraRunOncePacks::pPackageList).addr();
            let entry = e.with_stack(4, |e, local| {
                e.mem.set_u32(local.addr(), entry);
                e.call(SIMPLE_LIST_REMOVE_VALUE, &args![packages, local]);
                e.mem.u32(local.addr())
            });
            e.call(OPERATOR_DELETE, &args![entry]);
            node = e.get(extra, ExtraRunOncePacks::pPackageList).addr();
        }
    }
}

// Translated from 0041d8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the inline member at +0x38 of a `TESPackage` (the time
/// stamp `fn_0041d7b0` reads). The engine map has no name for it.
pub fn fn_0041d8a0(_e: &mut Engine, this: Ptr) -> Ptr {
    Ptr::new(this.addr().wrapping_add(0x38))
}

// Translated from 0041d8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `ExtraDataList::IsInRunOnceDayPackageList` (Xbox PDB): whether the
/// `pPackageList` (+0x0C) of the type `0x1B` extra data holds, before its
/// first empty item, an entry whose first word (the package) is `package`.
pub fn extra_data_list_is_in_run_once_day_package_list(
    e: &mut Engine,
    this: Ptr<ExtraDataList>,
    package: u32,
) -> bool {
    let extra = find_extra(e, this, EXTRA_RUN_ONCE_PACKAGES);
    if extra.is_null() {
        return false;
    }
    let mut node = e
        .get(
            extra.cast::<ExtraRunOncePacks>(),
            ExtraRunOncePacks::pPackageList,
        )
        .addr();
    while node != 0 {
        let slot = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(SIMPLE_LIST_ITEM, &args![node]).u32();
        let entry = e.mem.u32(slot);
        if e.mem.u32(entry) == package {
            return true;
        }
        node = e.call(SIMPLE_LIST_NEXT, &args![node]).u32();
    }
    false
}

// Translated from 0041d930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the first type `0x1B` extra data (`ExtraRunOncePacks`) of the list
/// (`RemoveExtra(type)`). The engine map has no name for it.
pub fn fn_0041d930(e: &mut Engine, this: Ptr<ExtraDataList>) {
    remove_extra_by_type(e, this, EXTRA_RUN_ONCE_PACKAGES);
}

// Translated from 0041d950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `LandNormal` (`NiPoint3`, +0x0C) of the type `0x13` extra data
/// (`ExtraDistantData`, `0x18` bytes, built by `00433260`; made and added
/// when the list has none) from the three words at `normal`. The engine map
/// has no name for it.
pub fn fn_0041d950(e: &mut Engine, this: Ptr<ExtraDataList>, normal: Ptr) {
    let mut extra = find_extra(e, this, EXTRA_DISTANT_DATA);
    if extra.is_null() {
        extra = add_new_extra(e, this, 0x18, EXTRA_DISTANT_DATA_INIT, &[]);
    }
    copy_point(e, normal.addr(), extra.addr() + 0x0c);
}

// Translated from 0041da10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `pParent` (+0x0C) of the type `0x37` extra data
/// (`ExtraEnableStateParent`), or null. The engine map has no name for it.
pub fn fn_0041da10(e: &mut Engine, this: Ptr<ExtraDataList>) -> u32 {
    get_word_extra(e, this, EXTRA_ENABLE_STATE_PARENT)
}

// Translated from 0041da40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the `pParent` (+0x0C) of the type `0x37` extra data
/// (`ExtraEnableStateParent`, `0x14` bytes, built by `00433300`): a null
/// `parent` deletes the first such extra data (`RemoveExtra(type)`); any
/// other is stored in the extra data, which is built, given the parent and
/// added when the list has none. The engine map has no name for it.
pub fn fn_0041da40(e: &mut Engine, this: Ptr<ExtraDataList>, parent: u32) {
    if parent == 0 {
        remove_extra_by_type(e, this, EXTRA_ENABLE_STATE_PARENT);
        return;
    }
    let extra = find_extra(e, this, EXTRA_ENABLE_STATE_PARENT);
    if !extra.is_null() {
        e.mem.set_u32(extra.addr() + 0x0c, parent);
        return;
    }
    let block = e.call(OPERATOR_NEW, &args![0x14u32]).u32();
    let new_extra = if block == 0 {
        0
    } else {
        e.call(EXTRA_ENABLE_STATE_PARENT_INIT, &args![block]).u32()
    };
    e.mem.set_u32(new_extra + 0x0c, parent);
    add_extra(e, this, new_extra);
}

/// This part's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x0041a6a0,
            fn_0041a6a0(Ptr<ExtraDataList>, Ptr<BSSoundHandle>)
        ),
        entry!(
            0x0041a800,
            extra_data_list_set_sound(Ptr<ExtraDataList>, Ptr<BSSoundHandle>)
        ),
        entry!(
            0x0041a960,
            extra_data_list_set_ghost(Ptr<ExtraDataList>, bool)
        ),
        entry!(
            0x0041aa20,
            extra_data_list_set_worn(Ptr<ExtraDataList>, bool, bool)
        ),
        entry!(
            0x0041ab70,
            extra_data_list_set_can_not_wear(Ptr<ExtraDataList>, bool)
        ),
        entry!(0x0041ac30, fn_0041ac30(Ptr<ExtraDataList>, u8)),
        entry!(
            0x0041ad00,
            extra_data_list_set_package_start_location(Ptr<ExtraDataList>, u32, u32, Ptr, f32)
        ),
        entry!(
            0x0041adf0,
            extra_data_list_remove_anim_ptr(Ptr<ExtraDataList>)
        ),
        entry!(
            0x0041ae10,
            extra_data_list_remove_anim_save(Ptr<ExtraDataList>)
        ),
        entry!(0x0041ae30, fn_0041ae30(Ptr<ExtraDataList>)),
        entry!(0x0041ae50, fn_0041ae50(Ptr<ExtraDataList>)),
        entry!(
            0x0041ae70,
            extra_data_list_remove_lock_ptr(Ptr<ExtraDataList>)
        ),
        entry!(
            0x0041ae90,
            extra_data_list_remove_teleport_ptr(Ptr<ExtraDataList>)
        ),
        entry!(0x0041aeb0, fn_0041aeb0(Ptr<ExtraDataList>)),
        entry!(
            0x0041aed0,
            extra_data_list_remove_ownership(Ptr<ExtraDataList>)
        ),
        entry!(
            0x0041aef0,
            extra_data_list_remove_health(Ptr<ExtraDataList>)
        ),
        entry!(0x0041af10, extra_data_list_remove_count(Ptr<ExtraDataList>)),
        entry!(0x0041af30, fn_0041af30(Ptr<ExtraDataList>)),
        entry!(0x0041af50, fn_0041af50(Ptr<ExtraDataList>)),
        entry!(
            0x0041af70,
            script_locals_scalar_deleting_destructor(Ptr, u32) -> Ptr
        ),
        entry!(
            0x0041afa0,
            extra_data_list_clear_script_locals(Ptr<ExtraDataList>)
        ),
        entry!(0x0041afd0, fn_0041afd0(Ptr<ExtraDataList>)),
        entry!(0x0041aff0, fn_0041aff0(Ptr<ExtraDataList>)),
        entry!(0x0041b010, fn_0041b010(Ptr<ExtraDataList>, bool)),
        entry!(
            0x0041b040,
            extra_data_list_remove_cannot_wear_extra(Ptr<ExtraDataList>)
        ),
        entry!(0x0041b060, fn_0041b060(Ptr<ExtraDataList>)),
        entry!(0x0041b080, fn_0041b080(Ptr<ExtraDataList>, Ptr, Ptr) -> Ptr),
        entry!(0x0041b0d0, fn_0041b0d0(Ptr<ExtraDataList>, Ptr, Ptr) -> Ptr),
        entry!(
            0x0041b120,
            fn_0041b120(Ptr<ExtraDataList>, Ptr, Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x0041b180,
            fn_0041b180(Ptr<ExtraDataList>, Ptr, Ptr, u32, u32, u32) -> Ptr
        ),
        entry!(0x0041b1e0, fn_0041b1e0(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041b2a0,
            extra_data_list_set_starting_world_or_cell_for_ref(Ptr<ExtraDataList>, Ptr)
        ),
        entry!(
            0x0041b320,
            extra_data_list_get_starting_world_or_cell(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x0041b350, fn_0041b350(Ptr<ExtraDataList>)),
        entry!(0x0041b370, fn_0041b370(Ptr<ExtraDataList>) -> u32),
        entry!(0x0041b3a0, fn_0041b3a0(Ptr<ExtraDataList>, u32) -> bool),
        entry!(0x0041b3d0, fn_0041b3d0(Ptr<ExtraDataList>, u32)),
        entry!(0x0041b440, fn_0041b440(Ptr<ExtraDataList>, u32)),
        entry!(0x0041b470, fn_0041b470(Ptr<ExtraDataList>, u32)),
        entry!(0x0041b4b0, fn_0041b4b0(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041b520,
            extra_data_list_get_action_ref(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x0041b550,
            extra_data_list_get_health_perc(Ptr<ExtraDataList>) -> f32
        ),
        entry!(0x0041b580, fn_0041b580(Ptr<ExtraDataList>, f32)),
        entry!(0x0041b650, fn_0041b650(Ptr) -> Ptr),
        entry!(0x0041b680, fn_0041b680(Ptr, u32) -> Ptr),
        entry!(
            0x0041b6b0,
            extra_data_list_get_object_health(Ptr<ExtraDataList>) -> f32
        ),
        entry!(0x0041b6e0, fn_0041b6e0(Ptr<ExtraDataList>, f32)),
        entry!(0x0041b790, fn_0041b790(Ptr, f32) -> Ptr),
        entry!(0x0041b7c0, fn_0041b7c0(Ptr<ExtraDataList>)),
        entry!(0x0041b7e0, fn_0041b7e0(Ptr<ExtraDataList>, u32)),
        entry!(0x0041b8d0, fn_0041b8d0(Ptr<ExtraDataList>, u32)),
        entry!(0x0041b9a0, fn_0041b9a0(Ptr<ExtraDataList>) -> u32),
        entry!(0x0041ba60, fn_0041ba60(Ptr<ExtraDataList>, u32)),
        entry!(0x0041bb10, fn_0041bb10(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x0041bbd0,
            extra_data_list_set_region_list(Ptr<ExtraDataList>, u32)
        ),
        entry!(
            0x0041bce0,
            extra_data_list_get_region_list(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x0041bd10, fn_0041bd10(Ptr<ExtraDataList>, u32)),
        entry!(0x0041bde0, fn_0041bde0(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x0041be10,
            extra_data_list_set_terminal_state(Ptr<ExtraDataList>, u8)
        ),
        entry!(0x0041bef0, fn_0041bef0(Ptr) -> Ptr),
        entry!(0x0041bf20, fn_0041bf20(Ptr) -> bool),
        entry!(
            0x0041bf50,
            extra_data_list_get_terminal_state(Ptr<ExtraDataList>) -> u8
        ),
        entry!(0x0041bf80, fn_0041bf80(Ptr<ExtraDataList>, u8)),
        entry!(0x0041c060, fn_0041c060(Ptr<ExtraDataList>) -> u8),
        entry!(0x0041c090, fn_0041c090(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041c160,
            extra_data_list_get_acoustic_space(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x0041c190, fn_0041c190(Ptr<ExtraDataList>, u32)),
        entry!(0x0041c260, fn_0041c260(Ptr<ExtraDataList>) -> u32),
        entry!(0x0041c290, fn_0041c290(Ptr<ExtraDataList>, u32)),
        entry!(0x0041c360, fn_0041c360(Ptr<ExtraDataList>) -> u32),
        entry!(0x0041c390, fn_0041c390(Ptr<ExtraDataList>, u32)),
        entry!(0x0041c460, fn_0041c460(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x0041c490,
            extra_data_list_set_canopy_shadow_mask(Ptr<ExtraDataList>, u32, u32, Ptr)
        ),
        entry!(0x0041c580, fn_0041c580(Ptr<ExtraDataList>, Ptr, Ptr) -> u32),
        entry!(0x0041c5e0, fn_0041c5e0(Ptr, u32) -> Ptr),
        entry!(0x0041c660, fn_0041c660(Ptr, u32) -> Ptr),
        entry!(0x0041c690, fn_0041c690(Ptr)),
        entry!(0x0041c6f0, fn_0041c6f0(Ptr<ExtraDataList>, u32)),
        entry!(0x0041c7c0, fn_0041c7c0(Ptr<ExtraDataList>) -> u32),
        entry!(0x0041c7f0, fn_0041c7f0(Ptr<ExtraDataList>, u32)),
        entry!(
            0x0041c8d0,
            extra_data_list_get_reference_pointer(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x0041c900, fn_0041c900(Ptr<ExtraDataList>)),
        entry!(
            0x0041c930,
            extra_data_list_set_package_extra(Ptr<ExtraDataList>, Ptr, i32, Ptr, u8, u8, u8)
        ),
        entry!(0x0041ca90, fn_0041ca90(Ptr) -> i32),
        entry!(
            0x0041cab0,
            extra_data_list_set_package_extra_target(Ptr<ExtraDataList>, Ptr)
        ),
        entry!(
            0x0041cae0,
            extra_data_list_set_package_extra_index(Ptr<ExtraDataList>, i32)
        ),
        entry!(
            0x0041cb10,
            extra_data_list_get_package_extra(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x0041cb40,
            extra_data_list_get_package_extra_index(Ptr<ExtraDataList>) -> i32
        ),
        entry!(
            0x0041cb70,
            extra_data_list_get_package_extra_target(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x0041cba0, fn_0041cba0(Ptr<ExtraDataList>) -> u8),
        entry!(
            0x0041cbd0,
            extra_data_list_set_package_extra_action_complete(Ptr<ExtraDataList>, u8)
        ),
        entry!(0x0041cc00, fn_0041cc00(Ptr<ExtraDataList>) -> u8),
        entry!(0x0041cc30, fn_0041cc30(Ptr<ExtraDataList>) -> u8),
        entry!(
            0x0041cc60,
            extra_data_list_remove_package_extra(Ptr<ExtraDataList>)
        ),
        entry!(0x0041cc90, fn_0041cc90(Ptr<ExtraDataList>, Ptr)),
        entry!(0x0041cd70, fn_0041cd70(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x0041cda0,
            extra_data_list_remove_trespass_package(Ptr<ExtraDataList>)
        ),
        entry!(0x0041ce10, fn_0041ce10(Ptr<ExtraDataList>, Ptr)),
        entry!(
            0x0041ce50,
            extra_data_list_add_to_player_crime_list(Ptr<ExtraDataList>, u32)
        ),
        entry!(
            0x0041cf00,
            extra_data_list_get_player_crime_list(Ptr<ExtraDataList>) -> u32
        ),
        entry!(
            0x0041cf30,
            extra_data_list_remove_player_crime_list_extra(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x0041d000, fn_0041d000(Ptr<ExtraDataList>)),
        entry!(0x0041d030, fn_0041d030(Ptr<ExtraDataList>) -> bool),
        entry!(
            0x0041d120,
            extra_data_list_is_extra_defaultfor_container(Ptr<ExtraDataList>, u8) -> bool
        ),
        entry!(0x0041d280, fn_0041d280(Ptr<ExtraDataList>, i32)),
        entry!(0x0041d330, fn_0041d330(Ptr<ExtraDataList>, u8)),
        entry!(0x0041d360, fn_0041d360(Ptr<ExtraDataList>) -> i32),
        entry!(
            0x0041d390,
            extra_data_list_set_persistent_cell(Ptr<ExtraDataList>, u32)
        ),
        entry!(0x0041d460, fn_0041d460(Ptr<ExtraDataList>) -> u32),
        entry!(
            0x0041d490,
            extra_data_list_set_rag_doll_data(Ptr<ExtraDataList>, u32)
        ),
        entry!(
            0x0041d5b0,
            extra_data_list_set_rag_doll_data_from_copy(Ptr<ExtraDataList>, u32)
        ),
        entry!(
            0x0041d6d0,
            extra_data_list_get_rag_doll_data(Ptr<ExtraDataList>) -> u32
        ),
        entry!(0x0041d700, fn_0041d700(Ptr<ExtraDataList>, u32, u8)),
        entry!(0x0041d7b0, fn_0041d7b0(Ptr<ExtraDataList>, i8)),
        entry!(0x0041d8a0, fn_0041d8a0(Ptr) -> Ptr),
        entry!(
            0x0041d8c0,
            extra_data_list_is_in_run_once_day_package_list(Ptr<ExtraDataList>, u32) -> bool
        ),
        entry!(0x0041d930, fn_0041d930(Ptr<ExtraDataList>)),
        entry!(0x0041d950, fn_0041d950(Ptr<ExtraDataList>, Ptr)),
        entry!(0x0041da10, fn_0041da10(Ptr<ExtraDataList>) -> u32),
        entry!(0x0041da40, fn_0041da40(Ptr<ExtraDataList>, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    /// Test vtable of the extra data: slot 0 the scalar deleting destructor.
    const VTABLE: u32 = 0x0200_0000;
    const DESTRUCTOR: u32 = 0x0200_1000;
    /// Vtable at +0 of a test reference (slot `0x130 / 4` gives its name) and
    /// the one at +0x18 (slot 0 gives its cell).
    const REFERENCE_VTABLE: u32 = 0x0200_2000;
    const CHILD_CELL_VTABLE: u32 = 0x0200_3000;
    const REFERENCE_NAME: u32 = 0x0200_1010;
    const REFERENCE_CELL: u32 = 0x0200_1014;

    /// The callees of the list operations of the main file (lock, the type
    /// and next accessors, the cache clearing) and of the sound handles.
    const LOCK: u32 = 0x0040_fbf0;
    const UNLOCK: u32 = 0x0040_fba0;
    const GET_TYPE: u32 = 0x004f_1540;
    const GET_NEXT: u32 = 0x0044_ddc0;
    const SET_NEXT: u32 = 0x0040_3550;
    const SIMPLE_LIST_NEXT: u32 = 0x0072_6070;
    const READ_WORD: u32 = 0x0055_9450;
    const MEMSET: u32 = 0x0040_3d30;

    /// Every extra data type of this file with the constructor that makes it.
    const CONSTRUCTORS: &[(u32, u8)] = &[
        (EXTRA_GHOST_INIT, EXTRA_GHOST),
        (EXTRA_WORN_INIT, EXTRA_WORN),
        (EXTRA_WORN_LEFT_INIT, EXTRA_WORN_LEFT),
        (EXTRA_CAN_NOT_WEAR_INIT, EXTRA_CAN_NOT_WEAR),
        (EXTRA_SEED_INIT, EXTRA_SEED),
        (
            EXTRA_PACKAGE_START_LOCATION_INIT,
            EXTRA_PACKAGE_START_LOCATION,
        ),
        (
            EXTRA_STARTING_WORLD_OR_CELL_INIT,
            EXTRA_STARTING_WORLD_OR_CELL,
        ),
        (EXTRA_SOUND_INIT, EXTRA_SOUND),
        (EXTRA_ACTIVATE_LOOP_SOUND_INIT, EXTRA_ACTIVATE_LOOP_SOUND),
        // ExtraAction's constructor, which `004195b0` (main file) calls.
        (0x0043_1780, EXTRA_ACTION),
        (EXTRA_HAVOK_INIT, EXTRA_HAVOK),
        (EXTRA_CELL_3D_INIT, EXTRA_CELL_3D),
        (EXTRA_REGION_LIST_INIT, EXTRA_REGION_LIST),
        (EXTRA_CELL_MUSIC_TYPE_INIT, EXTRA_CELL_MUSIC_TYPE),
        (EXTRA_CELL_ACOUSTIC_SPACE_INIT, EXTRA_CELL_ACOUSTIC_SPACE),
        (EXTRA_CLIMATE_INIT, EXTRA_CLIMATE),
        (EXTRA_IMAGE_SPACE_INIT, EXTRA_IMAGE_SPACE),
        (EXTRA_IMPACT_SWAP_INIT, EXTRA_IMPACT_SWAP),
        (
            EXTRA_CELL_CANOPY_SHADOW_MASK_INIT,
            EXTRA_CELL_CANOPY_SHADOW_MASK,
        ),
        (EXTRA_REFERENCE_POINTER_INIT, EXTRA_REFERENCE_POINTER),
        (EXTRA_TRESPASS_PACKAGE_INIT, EXTRA_TRESPASS_PACKAGE),
        (EXTRA_PLAYER_CRIME_LIST_INIT, EXTRA_PLAYER_CRIME_LIST),
        (EXTRA_LEVELED_ITEM_INIT, EXTRA_LEVELED_ITEM),
        (EXTRA_PERSISTENT_CELL_INIT, EXTRA_PERSISTENT_CELL),
        (EXTRA_RUN_ONCE_PACKAGES_INIT, EXTRA_RUN_ONCE_PACKAGES),
        (EXTRA_DISTANT_DATA_INIT, EXTRA_DISTANT_DATA),
        (EXTRA_ENABLE_STATE_PARENT_INIT, EXTRA_ENABLE_STATE_PARENT),
        (EXTRA_RAGDOLL_DATA_INIT, EXTRA_RAG_DOLL_DATA),
    ];

    fn returns(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    fn stub(e: &mut Engine, address: u32) {
        e.register(address, |_, _| Ret::default());
    }

    /// An engine with working doubles for the callees of the list
    /// operations, `operator new`, and the constructors of this file's
    /// extra data (they set the vtable and the type, and keep their
    /// argument words from +0x0C).
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        // The constants (`double` 1.0, `float` -1.0) and the save/load global.
        e.map(0x0101_2000, 0x1000);
        e.set_global::<f64>(ONE_AS_DOUBLE, 1.0);
        e.set_global::<f32>(NO_OBJECT_HEALTH, -1.0);
        e.map(0x011d_e000, 0x1000);
        e.register(BS_EXTRA_DATA_INIT, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            e.mem.set_u8(a[0] + 4, a[1] as u8);
            e.mem.set_u32(a[0] + 8, 0);
            returns(a[0])
        });
        e.register(NI_POINTER_INIT, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            returns(a[0])
        });
        e.register(NI_POINTER_ASSIGN, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            returns(a[0])
        });
        e.register(NI_POINTER_EQUALS, |e, a| {
            returns((e.mem.u32(a[0]) == a[1]) as u32)
        });
        e.register(BS_STRING_SET, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            returns(a[0])
        });
        e.put_vtable(VTABLE, &[DESTRUCTOR]);
        e.register(DESTRUCTOR, |_, _| Ret::default());
        e.register(GET_TYPE, |e, a| returns(e.mem.u8(a[0] + 4) as u32));
        e.register(GET_NEXT, |e, a| returns(e.mem.u32(a[0] + 8)));
        e.register(SET_NEXT, |e, a| {
            e.mem.set_u32(a[0] + 8, a[1]);
            Ret::default()
        });
        e.register(SIMPLE_LIST_NEXT, |e, a| returns(e.mem.u32(a[0] + 4)));
        e.register(READ_WORD, |e, a| returns(e.mem.u32(a[0])));
        e.register(MEMSET, |e, a| {
            for offset in 0..a[2] {
                e.mem.set_u8(a[0] + offset, a[1] as u8);
            }
            Ret::default()
        });
        e.register(OPERATOR_NEW, |e, a| returns(e.mem.alloc(a[0])));
        // A list node is {item, next}: the item slot is the node itself.
        e.register(SIMPLE_LIST_ITEM, |_, a| returns(a[0]));
        // The package flags at +0x1C (`GetIsCreated` bit 0x800, `IsNeverToRun`
        // bit 0x8000).
        e.register(PACKAGE_GET_IS_CREATED, |e, a| {
            returns((e.mem.u32(a[0] + 0x1c) & 0x800 != 0) as u32)
        });
        e.register(PACKAGE_IS_NEVER_TO_RUN, |e, a| {
            returns((e.mem.u32(a[0] + 0x1c) & 0x8000 != 0) as u32)
        });
        // The `RagDollData` constructor only returns its block.
        e.register(RAGDOLL_DATA_INIT, |_, a| returns(a[0]));
        // `ExtraPackage`'s constructor: the package, index and target words,
        // then the three flag bytes.
        e.register_double(EXTRA_PACKAGE_INIT, |e, a| {
            e.mem.set_u32(a[0], VTABLE);
            e.mem.set_u8(a[0] + 4, EXTRA_PACKAGE);
            e.mem.set_u32(a[0] + 8, 0);
            for (index, word) in a[1..4].iter().enumerate() {
                e.mem.set_u32(a[0] + 0x0c + 4 * index as u32, *word);
            }
            for (index, flag) in a[4..7].iter().enumerate() {
                e.mem.set_u8(a[0] + 0x18 + index as u32, *flag as u8);
            }
            returns(a[0])
        });
        for address in [
            LOCK,
            UNLOCK,
            OPERATOR_DELETE,
            SOUND_HANDLE_DESTRUCTOR,
            SCRIPT_LOCALS_DESTRUCTOR,
            PACKAGE_SET_IS_CREATED,
            SAVE_LOAD_GAME_DELETE_FORM,
            CRIME_REMOVE_ENTRY,
            SIMPLE_LIST_FREE_NODES,
            SIMPLE_LIST_SCALAR_DELETING_DESTRUCTOR,
            LIST_ADD_HEAD,
            RUN_ONCE_PACKAGES_ADD,
            RAG_DOLL_UPDATE_FROM_REFERENCE,
            RAG_DOLL_COPY,
            NI_POINTER_DESTROY,
            BS_STRING_INIT,
            BS_STRING_DESTROY,
            BS_EXTRA_DATA_DESTROY,
            EXTRA_DATA_BASE_DESTRUCTOR,
            SAVE_LOAD_GAME_STUB,
            GET_REF_PERSISTS,
            LOG_MESSAGE,
        ] {
            stub(&mut e, address);
        }
        for &(address, extra_type) in CONSTRUCTORS {
            e.register_double(address, move |e, a| {
                e.mem.set_u32(a[0], VTABLE);
                e.mem.set_u8(a[0] + 4, extra_type);
                e.mem.set_u32(a[0] + 8, 0);
                for (index, word) in a[1..].iter().enumerate() {
                    e.mem.set_u32(a[0] + 0x0c + 4 * index as u32, *word);
                }
                returns(a[0])
            });
        }
        e
    }

    /// A list holding extra data of the given `(type, word at +0x0C)`, in
    /// order, linked through `AddExtra` (so the type bitmap is set).
    fn list_with(
        e: &mut Engine,
        entries: &[(u8, u32)],
    ) -> (Ptr<ExtraDataList>, Vec<Ptr<BSExtraData>>) {
        let list: Ptr<ExtraDataList> = e.new_object();
        let mut extras = vec![];
        for &(extra_type, word) in entries {
            let extra: Ptr<BSExtraData> = Ptr::new(e.mem.alloc(0x40));
            e.mem.set_u32(extra.addr(), VTABLE);
            e.set(extra, BSExtraData::cEtype, extra_type);
            e.mem.set_u32(extra.addr() + 0x0c, word);
            e.call(ADD_EXTRA, &args![list, extra]);
            extras.push(extra);
        }
        (list, extras)
    }

    /// The types in the list's chain, sorted (some types are added at the
    /// head of the chain, the others at the end).
    fn sorted_types(e: &Engine, list: Ptr<ExtraDataList>) -> Vec<u8> {
        let mut types = vec![];
        let mut current: Ptr<BSExtraData> = e.get(list, ExtraDataList::pHead).cast();
        while !current.is_null() {
            types.push(e.get(current, BSExtraData::cEtype));
            current = e.get(current, BSExtraData::pNext).cast();
        }
        types.sort();
        types
    }

    /// Runs `call` on the engine with the call log on and returns the log.
    fn logged(e: &mut Engine, call: impl FnOnce(&mut Engine)) -> Log {
        e.call_log = Some(vec![]);
        call(e);
        e.call_log.take().unwrap()
    }

    fn calls_to(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(callee, _)| *callee == address)
            .map(|(_, args)| args.clone())
            .collect()
    }

    /// The objects the log shows deleted (the destructor called).
    fn deleted(log: &Log) -> Vec<u32> {
        calls_to(log, DESTRUCTOR).iter().map(|a| a[0]).collect()
    }

    /// A sound handle in memory.
    fn sound_handle(e: &mut Engine, id: u32, assume_success: u8, state: u32) -> Ptr<BSSoundHandle> {
        let handle: Ptr<BSSoundHandle> = e.new_object();
        e.set(handle, BSSoundHandle::iSoundID, id);
        e.set(handle, BSSoundHandle::bAssumeSuccess, assume_success);
        e.set(handle, BSSoundHandle::eState, state);
        handle
    }

    /// A type that is not `extra_type`, for the extra data a function must
    /// leave alone.
    fn other_type(extra_type: u8) -> u8 {
        if extra_type == 0x01 {
            0x02
        } else {
            0x01
        }
    }

    /// Checks a sound setter of `address` for `extra_type` built by
    /// `construct`.
    fn check_sound_setter(address: u32, extra_type: u8, construct: u32) {
        // No extra data, a sound: one is built from a copy of the handle.
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let sound = sound_handle(&mut e, 5, 1, 2);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, sound]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x18]]);
        let built = calls_to(&log, construct);
        assert_eq!(built.len(), 1);
        assert_eq!((built[0][1], built[0][2] & 0xff), (5, 1));
        assert_eq!(built[0][3], 2);
        assert_eq!(sorted_types(&e, list), vec![extra_type]);
        // No extra data, an empty sound: nothing.
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let empty = sound_handle(&mut e, 0xffff_ffff, 0, 0);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, empty]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(sorted_types(&e, list).is_empty());
        // An extra data and an empty sound: it is deleted.
        let (list, extras) = list_with(&mut e, &[(other_type(extra_type), 0), (extra_type, 0)]);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, empty]);
        });
        assert_eq!(deleted(&log), vec![extras[1].addr()]);
        assert_eq!(sorted_types(&e, list), vec![other_type(extra_type)]);
        // An extra data and a sound: the handle is copied into it.
        let (list, extras) = list_with(&mut e, &[(extra_type, 0)]);
        let sound = sound_handle(&mut e, 9, 1, 4);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, sound]);
        });
        assert!(deleted(&log).is_empty());
        let handle: Ptr<BSSoundHandle> = Ptr::new(extras[0].addr() + 0x0c);
        assert_eq!(e.get(handle, BSSoundHandle::iSoundID), 9);
        assert_eq!(e.get(handle, BSSoundHandle::bAssumeSuccess), 1);
        assert_eq!(e.get(handle, BSSoundHandle::eState), 4);
    }

    /// Checks a flag setter of `address` (`flag_args` are the words after
    /// `this` for "wanted" and not "wanted") for `extra_type` built by
    /// `construct`.
    fn check_flag_setter(address: u32, extra_type: u8, construct: u32, other_args: &[u32]) {
        let call = |e: &mut Engine, list: Ptr<ExtraDataList>, wanted: bool| {
            let mut words = vec![list.addr(), wanted as u32];
            words.extend_from_slice(other_args);
            e.call(address, &words);
        };
        // Wanted and missing: built (0x0C bytes) and added.
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[(other_type(extra_type), 0)]);
        let log = logged(&mut e, |e| call(e, list, true));
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x0c]]);
        assert_eq!(calls_to(&log, construct).len(), 1);
        let mut expected = vec![other_type(extra_type), extra_type];
        expected.sort();
        assert_eq!(sorted_types(&e, list), expected);
        // Wanted and present: nothing.
        let log = logged(&mut e, |e| call(e, list, true));
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, REMOVE_EXTRA_BY_TYPE).is_empty());
        // Not wanted and present: deleted by type.
        let log = logged(&mut e, |e| call(e, list, false));
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA_BY_TYPE),
            vec![vec![list.addr(), extra_type as u32]]
        );
        assert_eq!(sorted_types(&e, list), vec![other_type(extra_type)]);
        // Not wanted and missing: nothing.
        let log = logged(&mut e, |e| call(e, list, false));
        assert!(calls_to(&log, REMOVE_EXTRA_BY_TYPE).is_empty());
    }

    /// Checks a one-line remover of `address` for `extra_type`: the first
    /// extra data of that type goes, the others stay, and an empty list is
    /// fine.
    fn check_remover(address: u32, extra_type: u8) {
        let mut e = engine();
        let decoy = other_type(extra_type);
        let (list, extras) = list_with(&mut e, &[(decoy, 0), (extra_type, 0)]);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list]);
        });
        assert_eq!(deleted(&log), vec![extras[1].addr()]);
        assert_eq!(sorted_types(&e, list), vec![decoy]);
        let (empty, _) = list_with(&mut e, &[]);
        e.call(address, &args![empty]);
        assert!(sorted_types(&e, empty).is_empty());
    }

    #[test]
    fn activate_loop_sound_setter_covers_the_four_cases() {
        check_sound_setter(
            0x0041_a6a0,
            EXTRA_ACTIVATE_LOOP_SOUND,
            EXTRA_ACTIVATE_LOOP_SOUND_INIT,
        );
    }

    #[test]
    fn sound_setter_covers_the_four_cases() {
        check_sound_setter(0x0041_a800, EXTRA_SOUND, EXTRA_SOUND_INIT);
    }

    #[test]
    fn ghost_setter_adds_and_removes_the_flag() {
        check_flag_setter(0x0041_a960, EXTRA_GHOST, EXTRA_GHOST_INIT, &[]);
    }

    #[test]
    fn worn_setter_picks_the_hand() {
        // The right hand (`left` false) is the type 0x16, the left one 0x17.
        check_flag_setter(0x0041_aa20, EXTRA_WORN, EXTRA_WORN_INIT, &[0]);
        check_flag_setter(0x0041_aa20, EXTRA_WORN_LEFT, EXTRA_WORN_LEFT_INIT, &[1]);
    }

    #[test]
    fn can_not_wear_setter_adds_and_removes_the_flag() {
        check_flag_setter(
            0x0041_ab70,
            EXTRA_CAN_NOT_WEAR,
            EXTRA_CAN_NOT_WEAR_INIT,
            &[],
        );
    }

    #[test]
    fn seed_setter_stores_builds_or_removes() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        // 0xFF without an extra data: nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0041_ac30, &args![list, 0xffu8]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // Another value without one: built (0x10 bytes) with the seed.
        let log = logged(&mut e, |e| {
            e.call(0x0041_ac30, &args![list, 0x42u8]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, EXTRA_SEED_INIT)[0][1], 0x42);
        assert_eq!(sorted_types(&e, list), vec![EXTRA_SEED]);
        // With one: the byte is stored, nothing built.
        let log = logged(&mut e, |e| {
            e.call(0x0041_ac30, &args![list, 0x07u8]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        let extra = find_extra(&mut e, list, EXTRA_SEED);
        assert_eq!(e.mem.u8(extra.addr() + 0x0c), 0x07);
        // 0xFF with one: deleted.
        let log = logged(&mut e, |e| {
            e.call(0x0041_ac30, &args![list, 0xffu8]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn package_start_location_builds_then_updates() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let position: Ptr = Ptr::new(e.mem.alloc(0x0c));
        for (index, word) in [0x1111u32, 0x2222, 0x3333].iter().enumerate() {
            e.mem.set_u32(position.addr() + 4 * index as u32, *word);
        }
        // Built from the forms, the position pointer and the rotation (a
        // signalling NaN comes out quiet from the x87 stack).
        let log = logged(&mut e, |e| {
            e.call(
                0x0041_ad00,
                &args![
                    list,
                    0xaau32,
                    0xbbu32,
                    position,
                    f32::from_bits(0x7f80_0001)
                ],
            );
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x20]]);
        let built = calls_to(&log, EXTRA_PACKAGE_START_LOCATION_INIT);
        assert_eq!(built[0][1..], [0xaa, 0xbb, position.addr(), 0x7fc0_0001]);
        // Updated: the first form wins, the position is copied.
        let extra = find_extra(&mut e, list, EXTRA_PACKAGE_START_LOCATION);
        e.mem.set_u32(position.addr() + 4, 0x9999);
        let log = logged(&mut e, |e| {
            e.call(
                0x0041_ad00,
                &args![list, 0xccu32, 0xddu32, position, 1.0f32],
            );
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0xcc);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 0x1111);
        assert_eq!(e.mem.u32(extra.addr() + 0x14), 0x9999);
        assert_eq!(e.mem.u32(extra.addr() + 0x18), 0x3333);
        // The second form stands in for a null first one.
        e.call(0x0041_ad00, &args![list, 0u32, 0xddu32, position, 1.0f32]);
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0xdd);
    }

    #[test]
    fn anim_ptr_remover() {
        check_remover(0x0041_adf0, EXTRA_ANIM);
    }

    #[test]
    fn anim_save_remover() {
        check_remover(0x0041_ae10, EXTRA_ANIM_SAVE);
    }

    #[test]
    fn light_remover() {
        check_remover(0x0041_ae30, EXTRA_LIGHT);
    }

    #[test]
    fn magic_light_remover() {
        check_remover(0x0041_ae50, EXTRA_MAGIC_LIGHT);
    }

    #[test]
    fn lock_ptr_remover() {
        check_remover(0x0041_ae70, EXTRA_LOCK);
    }

    #[test]
    fn teleport_ptr_remover() {
        check_remover(0x0041_ae90, EXTRA_TELEPORT);
    }

    #[test]
    fn container_changes_remover() {
        check_remover(0x0041_aeb0, EXTRA_CONTAINER_CHANGES);
    }

    #[test]
    fn ownership_remover() {
        check_remover(0x0041_aed0, EXTRA_OWNERSHIP);
    }

    #[test]
    fn health_remover() {
        check_remover(0x0041_aef0, EXTRA_HEALTH);
    }

    #[test]
    fn count_remover() {
        check_remover(0x0041_af10, EXTRA_COUNT);
    }

    #[test]
    fn poison_remover() {
        check_remover(0x0041_af30, EXTRA_POISON);
    }

    #[test]
    fn script_remover() {
        check_remover(0x0041_af50, EXTRA_SCRIPT);
    }

    #[test]
    fn script_locals_destructor_deletes_only_with_the_flag() {
        let mut e = engine();
        let locals: Ptr = Ptr::new(e.mem.alloc(0x20));
        let log = logged(&mut e, |e| {
            let result = e.call(0x0041_af70, &args![locals, 0u32]).ptr::<()>();
            assert_eq!(result, locals);
        });
        assert_eq!(
            calls_to(&log, SCRIPT_LOCALS_DESTRUCTOR),
            vec![vec![locals.addr()]]
        );
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            let result = e.call(0x0041_af70, &args![locals, 3u32]).ptr::<()>();
            assert_eq!(result, locals);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![locals.addr()]]);
    }

    #[test]
    fn clear_script_locals_nulls_the_variables_pointer() {
        let mut e = engine();
        let (list, extras) = list_with(&mut e, &[(other_type(EXTRA_SCRIPT), 0), (EXTRA_SCRIPT, 0)]);
        e.mem.set_u32(extras[1].addr() + 0x10, 0x1234);
        e.mem.set_u32(extras[0].addr() + 0x10, 0x5678);
        e.call(0x0041_afa0, &args![list]);
        assert_eq!(e.mem.u32(extras[1].addr() + 0x10), 0);
        assert_eq!(e.mem.u32(extras[0].addr() + 0x10), 0x5678);
        // Without the extra data: nothing happens.
        let (empty, _) = list_with(&mut e, &[]);
        e.call(0x0041_afa0, &args![empty]);
    }

    #[test]
    fn scale_remover() {
        check_remover(0x0041_afd0, EXTRA_SCALE);
    }

    #[test]
    fn ghost_remover() {
        check_remover(0x0041_aff0, EXTRA_GHOST);
    }

    #[test]
    fn worn_remover_picks_the_hand() {
        let mut e = engine();
        let (list, extras) = list_with(&mut e, &[(EXTRA_WORN, 0), (EXTRA_WORN_LEFT, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_b010, &args![list, true]);
        });
        assert_eq!(deleted(&log), vec![extras[1].addr()]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_b010, &args![list, false]);
        });
        assert_eq!(deleted(&log), vec![extras[0].addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn cannot_wear_remover() {
        check_remover(0x0041_b040, EXTRA_CAN_NOT_WEAR);
    }

    #[test]
    fn package_start_location_remover() {
        check_remover(0x0041_b060, EXTRA_PACKAGE_START_LOCATION);
    }

    /// A list with an `ExtraStartingPosition` whose position is (1, 2, 3)
    /// and rotation (4, 5, 6).
    fn list_with_starting_position(e: &mut Engine) -> (Ptr<ExtraDataList>, Ptr<BSExtraData>) {
        let (list, extras) = list_with(
            e,
            &[
                (other_type(EXTRA_STARTING_POSITION), 0),
                (EXTRA_STARTING_POSITION, 1),
            ],
        );
        for (index, word) in [1u32, 2, 3, 4, 5, 6].iter().enumerate() {
            e.mem
                .set_u32(extras[1].addr() + 0x0c + 4 * index as u32, *word);
        }
        (list, extras[1])
    }

    /// The case where the list has no starting position: `00418d50` (main
    /// file) is a double that hands out `made`.
    fn make_starting_position_double(e: &mut Engine, made: Ptr<BSExtraData>) {
        e.register_double(GET_OR_ADD_STARTING_POSITION, move |_, _| {
            returns(made.addr())
        });
    }

    #[test]
    fn starting_rotation_getter_copies_plus_0x18() {
        let mut e = engine();
        let (list, _) = list_with_starting_position(&mut e);
        let out: Ptr = Ptr::new(e.mem.alloc(0x0c));
        let log = logged(&mut e, |e| {
            let result = e.call(0x0041_b080, &args![list, out, 0u32]).ptr::<()>();
            assert_eq!(result, out);
        });
        assert!(calls_to(&log, GET_OR_ADD_STARTING_POSITION).is_empty());
        assert_eq!(
            e.mem.bytes(out.addr(), 12),
            [4, 0, 0, 0, 5, 0, 0, 0, 6, 0, 0, 0]
        );
        // Missing: the extra data `00418d50` makes from the position is read.
        let (empty, _) = list_with(&mut e, &[]);
        let made: Ptr<BSExtraData> = Ptr::new(e.mem.alloc(0x24));
        e.mem.set_u32(made.addr() + 0x18, 0x77);
        make_starting_position_double(&mut e, made);
        let position: Ptr = Ptr::new(e.mem.alloc(0x0c));
        let log = logged(&mut e, |e| {
            e.call(0x0041_b080, &args![empty, out, position]);
        });
        assert_eq!(
            calls_to(&log, GET_OR_ADD_STARTING_POSITION),
            vec![vec![empty.addr(), position.addr()]]
        );
        assert_eq!(e.mem.u32(out.addr()), 0x77);
    }

    #[test]
    fn starting_position_getter_copies_plus_0x0c() {
        let mut e = engine();
        let (list, _) = list_with_starting_position(&mut e);
        let out: Ptr = Ptr::new(e.mem.alloc(0x0c));
        let result = e.call(0x0041_b0d0, &args![list, out, 0u32]).ptr::<()>();
        assert_eq!(result, out);
        assert_eq!(
            e.mem.bytes(out.addr(), 12),
            [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0]
        );
        let (empty, _) = list_with(&mut e, &[]);
        let made: Ptr<BSExtraData> = Ptr::new(e.mem.alloc(0x24));
        e.mem.set_u32(made.addr() + 0x0c, 0x88);
        make_starting_position_double(&mut e, made);
        e.call(0x0041_b0d0, &args![empty, out, 0u32]);
        assert_eq!(e.mem.u32(out.addr()), 0x88);
    }

    #[test]
    fn starting_rotation_setter_stores_plus_0x18_and_the_out_copy() {
        let mut e = engine();
        let (list, extra) = list_with_starting_position(&mut e);
        let out: Ptr = Ptr::new(e.mem.alloc(0x0c));
        let result = e
            .call(0x0041_b120, &args![list, out, 0u32, 7u32, 8u32, 9u32])
            .ptr::<()>();
        assert_eq!(result, out);
        assert_eq!(e.mem.u32(extra.addr() + 0x18), 7);
        assert_eq!(e.mem.u32(extra.addr() + 0x20), 9);
        // The position stays.
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 1);
        assert_eq!(e.mem.u32(out.addr() + 4), 8);
        // Missing: written into the extra data `00418d50` returns.
        let (empty, _) = list_with(&mut e, &[]);
        let made: Ptr<BSExtraData> = Ptr::new(e.mem.alloc(0x24));
        make_starting_position_double(&mut e, made);
        e.call(0x0041_b120, &args![empty, out, 0u32, 1u32, 2u32, 3u32]);
        assert_eq!(e.mem.u32(made.addr() + 0x1c), 2);
    }

    #[test]
    fn starting_position_setter_stores_plus_0x0c_and_the_out_copy() {
        let mut e = engine();
        let (list, extra) = list_with_starting_position(&mut e);
        let out: Ptr = Ptr::new(e.mem.alloc(0x0c));
        let result = e
            .call(0x0041_b180, &args![list, out, 0u32, 7u32, 8u32, 9u32])
            .ptr::<()>();
        assert_eq!(result, out);
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 7);
        assert_eq!(e.mem.u32(extra.addr() + 0x14), 9);
        // The rotation stays.
        assert_eq!(e.mem.u32(extra.addr() + 0x18), 4);
        assert_eq!(e.mem.u32(out.addr() + 8), 9);
        let (empty, _) = list_with(&mut e, &[]);
        let made: Ptr<BSExtraData> = Ptr::new(e.mem.alloc(0x24));
        make_starting_position_double(&mut e, made);
        e.call(0x0041_b180, &args![empty, out, 0u32, 1u32, 2u32, 3u32]);
        assert_eq!(e.mem.u32(made.addr() + 0x10), 2);
    }

    #[test]
    fn starting_world_or_cell_setter_builds_updates_or_removes() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        // A null form without an extra data: nothing to remove.
        e.call(0x0041_b1e0, &args![list, 0u32]);
        assert!(sorted_types(&e, list).is_empty());
        // A form: built (0x10 bytes, no arguments) and stored.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b1e0, &args![list, 0x1234u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, EXTRA_STARTING_WORLD_OR_CELL_INIT).len(), 1);
        let extra = find_extra(&mut e, list, EXTRA_STARTING_WORLD_OR_CELL);
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0x1234);
        // Another form: stored in the same extra data.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b1e0, &args![list, 0x5678u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.u32(extra.addr() + 0x0c), 0x5678);
        // Null: removes it.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b1e0, &args![list, 0u32]);
        });
        assert_eq!(
            calls_to(&log, REMOVE_EXTRA_BY_TYPE),
            vec![vec![list.addr(), 0x49]]
        );
        assert!(sorted_types(&e, list).is_empty());
    }

    /// A reference with a child cell base at +0x18 whose slot 0 gives
    /// `cell`, a form id of `form_id` and a name from slot `0x130`.
    fn reference(e: &mut Engine, cell: u32, form_id: u32) -> Ptr {
        let mut slots = vec![0u32; 0x130 / 4 + 1];
        slots[0x130 / 4] = REFERENCE_NAME;
        e.put_vtable(REFERENCE_VTABLE, &slots);
        e.put_vtable(CHILD_CELL_VTABLE, &[REFERENCE_CELL]);
        e.register(REFERENCE_NAME, |_, _| returns(0x00c0_ffee));
        e.register_double(REFERENCE_CELL, move |_, _| returns(cell));
        e.register(GET_FORM_ID, |e, a| returns(e.mem.u32(a[0] + 0x0c)));
        let reference: Ptr = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(reference.addr(), REFERENCE_VTABLE);
        e.mem.set_u32(reference.addr() + 0x0c, form_id);
        e.mem.set_u32(reference.addr() + 0x18, CHILD_CELL_VTABLE);
        reference
    }

    #[test]
    fn starting_world_or_cell_for_ref_prefers_the_world_space() {
        let mut e = engine();
        e.register(CELL_GET_WORLD_SPACE, |_, a| {
            returns(if a[0] == 0xce11 { 0xf00d } else { 0 })
        });
        // A cell with a world space: the world space is stored.
        let (list, _) = list_with(&mut e, &[]);
        let with_world = reference(&mut e, 0xce11, 0x1);
        e.call(0x0041_b2a0, &args![list, with_world]);
        assert_eq!(
            extra_data_list_get_starting_world_or_cell(&mut e, list),
            0xf00d
        );
        // A cell without one: the cell is stored.
        let (list, _) = list_with(&mut e, &[]);
        let without_world = reference(&mut e, 0xce22, 0x2);
        e.call(0x0041_b2a0, &args![list, without_world]);
        assert_eq!(
            extra_data_list_get_starting_world_or_cell(&mut e, list),
            0xce22
        );
    }

    #[test]
    fn starting_world_or_cell_for_ref_logs_when_there_is_no_cell() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let orphan = reference(&mut e, 0, 0x0001_4abc);
        let log = logged(&mut e, |e| {
            e.call(0x0041_b2a0, &args![list, orphan]);
        });
        assert_eq!(
            calls_to(&log, LOG_MESSAGE),
            vec![vec![MESSAGE_NO_PARENT_SAVE_CELL, 0x00c0_ffee, 0x0001_4abc]]
        );
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn starting_world_or_cell_getter() {
        let mut e = engine();
        let (list, _) = list_with(
            &mut e,
            &[
                (other_type(EXTRA_STARTING_WORLD_OR_CELL), 1),
                (EXTRA_STARTING_WORLD_OR_CELL, 0x4242),
            ],
        );
        assert_eq!(e.call(0x0041_b320, &args![list]).u32(), 0x4242);
        let (empty, _) = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0041_b320, &args![empty]).u32(), 0);
    }

    #[test]
    fn starting_world_or_cell_remover() {
        check_remover(0x0041_b350, EXTRA_STARTING_WORLD_OR_CELL);
    }

    #[test]
    fn action_getter_is_a_byte_and_defaults_to_one() {
        let mut e = engine();
        let (list, _) = list_with(
            &mut e,
            &[(other_type(EXTRA_ACTION), 0), (EXTRA_ACTION, 0x1234_5603)],
        );
        assert_eq!(e.call(0x0041_b370, &args![list]).u32(), 3);
        let (empty, _) = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0041_b370, &args![empty]).u32(), 1);
    }

    #[test]
    fn action_mask_test() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[(EXTRA_ACTION, 0x06)]);
        assert!(e.call(0x0041_b3a0, &args![list, 0x04u32]).bool());
        assert!(!e.call(0x0041_b3a0, &args![list, 0x09u32]).bool());
        // The default 1 when there is no extra data.
        let (empty, _) = list_with(&mut e, &[]);
        assert!(e.call(0x0041_b3a0, &args![empty, 1u32]).bool());
        assert!(!e.call(0x0041_b3a0, &args![empty, 2u32]).bool());
    }

    #[test]
    fn action_setter_builds_stores_or_removes() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        // 1 is the default: nothing is built.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b3d0, &args![list, 1u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // Another value: `004195b0` builds the extra data (0x14 bytes).
        let log = logged(&mut e, |e| {
            e.call(0x0041_b3d0, &args![list, 5u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        let extra = find_extra(&mut e, list, EXTRA_ACTION);
        assert_eq!(e.mem.u8(extra.addr() + 0x0c), 5);
        // A reference keeps the extra data when the value goes back to 1.
        e.mem.set_u32(extra.addr() + 0x10, 0xabc);
        e.call(0x0041_b3d0, &args![list, 1u32]);
        assert_eq!(e.mem.u8(extra.addr() + 0x0c), 1);
        assert_eq!(sorted_types(&e, list), vec![EXTRA_ACTION]);
        // Without a reference the value 1 deletes it.
        e.mem.set_u32(extra.addr() + 0x10, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0041_b3d0, &args![list, 1u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn action_bits_are_set_with_an_or() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[(EXTRA_ACTION, 0x04)]);
        e.call(0x0041_b440, &args![list, 0x02u32]);
        assert_eq!(fn_0041b370(&mut e, list), 0x06);
        // Without the extra data the default 1 is the base.
        let (empty, _) = list_with(&mut e, &[]);
        e.call(0x0041_b440, &args![empty, 0x02u32]);
        assert_eq!(fn_0041b370(&mut e, empty), 0x03);
    }

    #[test]
    fn action_bits_are_cleared_with_an_and_not() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[(EXTRA_ACTION, 0x07)]);
        e.call(0x0041_b470, &args![list, 0x02u32]);
        assert_eq!(fn_0041b370(&mut e, list), 0x05);
        // Clearing bit 0 of the default 1 builds the extra data with 0.
        let (empty, _) = list_with(&mut e, &[]);
        e.call(0x0041_b470, &args![empty, 0x01u32]);
        assert_eq!(sorted_types(&e, empty), vec![EXTRA_ACTION]);
        assert_eq!(fn_0041b370(&mut e, empty), 0);
    }

    #[test]
    fn action_reference_setter_builds_stores_or_removes() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        // Null without an extra data: nothing.
        e.call(0x0041_b4b0, &args![list, 0u32]);
        assert!(sorted_types(&e, list).is_empty());
        // A reference: `004195b0` builds the extra data, the reference is stored.
        e.call(0x0041_b4b0, &args![list, 0xbeefu32]);
        let extra = find_extra(&mut e, list, EXTRA_ACTION);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 0xbeef);
        // Null with eAction not 1: stored, the extra data stays.
        e.mem.set_u8(extra.addr() + 0x0c, 4);
        e.call(0x0041_b4b0, &args![list, 0u32]);
        assert_eq!(e.mem.u32(extra.addr() + 0x10), 0);
        assert_eq!(sorted_types(&e, list), vec![EXTRA_ACTION]);
        // Null with eAction 1: deleted.
        e.mem.set_u8(extra.addr() + 0x0c, 1);
        let log = logged(&mut e, |e| {
            e.call(0x0041_b4b0, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }

    // -----------------------------------------------------------------------
    // Tests of the functions `0041b520` to `0041c7f0`

    /// The word at `offset` of an extra data.
    fn word_at(e: &Engine, extra: Ptr<BSExtraData>, offset: u32) -> u32 {
        e.mem.u32(extra.addr() + offset)
    }

    /// Checks a setter of `address` for the extra data of `extra_type` whose
    /// only payload is a word at +0x0C (`0x10` bytes, built by `construct`).
    fn check_word_setter(address: u32, extra_type: u8, construct: u32) {
        let mut e = engine();
        let decoy = other_type(extra_type);
        let (list, _) = list_with(&mut e, &[(decoy, 0x77)]);
        // Null without an extra data: nothing.
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(sorted_types(&e, list), vec![decoy]);
        // A value: the extra data is built from it and added.
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0x1234u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        let built = calls_to(&log, construct);
        assert_eq!(built.len(), 1);
        assert_eq!(built[0][1], 0x1234);
        let extra = find_extra(&mut e, list, extra_type);
        assert_eq!(word_at(&e, extra, 0x0c), 0x1234);
        // Another value: stored in it.
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0x5678u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(word_at(&e, extra, 0x0c), 0x5678);
        // Null: the extra data goes.
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert_eq!(sorted_types(&e, list), vec![decoy]);
    }

    /// Checks a getter of `address` for the word at +0x0C of the extra data
    /// of `extra_type`.
    fn check_word_getter(address: u32, extra_type: u8) {
        let mut e = engine();
        let decoy = other_type(extra_type);
        let (list, _) = list_with(&mut e, &[(decoy, 0x55)]);
        assert_eq!(e.call(address, &args![list]).u32(), 0);
        let (list, _) = list_with(&mut e, &[(decoy, 0x55), (extra_type, 0xabcd)]);
        assert_eq!(e.call(address, &args![list]).u32(), 0xabcd);
    }

    /// Lets the extra data a constructor of this file builds be deleted: its
    /// vtable's slot 0 is the test destructor.
    fn make_deletable(e: &mut Engine, vtable: u32) {
        e.put_vtable(vtable, &[DESTRUCTOR]);
    }

    #[test]
    fn action_ref_getter_reads_plus_0x10() {
        let mut e = engine();
        let (empty, _) = list_with(&mut e, &[(other_type(EXTRA_ACTION), 0)]);
        assert_eq!(e.call(0x0041_b520, &args![empty]).u32(), 0);
        let (list, extras) = list_with(&mut e, &[(EXTRA_ACTION, 1)]);
        e.mem.set_u32(extras[0].addr() + 0x10, 0xcafe);
        assert_eq!(e.call(0x0041_b520, &args![list]).u32(), 0xcafe);
    }

    #[test]
    fn health_perc_getter_defaults_to_one() {
        let mut e = engine();
        let (empty, _) = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0041_b550, &args![empty]).f32(), 1.0);
        let (list, _) = list_with(&mut e, &[(EXTRA_HEALTH_PERC, 0.25f32.to_bits())]);
        assert_eq!(e.call(0x0041_b550, &args![list]).f32(), 0.25);
    }

    #[test]
    fn health_perc_setter_builds_stores_or_removes_at_one() {
        let mut e = engine();
        make_deletable(&mut e, VTABLE_EXTRA_HEALTH_PERC);
        let (list, _) = list_with(&mut e, &[]);
        // 1.0 without an extra data: RemoveExtra still runs, on null.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b580, &args![list, 1.0f32]);
        });
        assert_eq!(calls_to(&log, REMOVE_EXTRA), vec![vec![list.addr(), 0, 1]]);
        assert!(sorted_types(&e, list).is_empty());
        // Another value builds it (0x10 bytes) and stores the value.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b580, &args![list, 0.5f32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        let extra = find_extra(&mut e, list, EXTRA_HEALTH_PERC);
        assert_eq!(e.mem.u32(extra.addr()), VTABLE_EXTRA_HEALTH_PERC);
        assert_eq!(e.mem.f32(extra.addr() + 0x0c), 0.5);
        // Again: stored in the existing one.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b580, &args![list, 0.75f32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.f32(extra.addr() + 0x0c), 0.75);
        // 1.0 deletes it.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b580, &args![list, 1.0f32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn health_perc_constructor_sets_the_vtable_and_zero() {
        let mut e = engine();
        let block = Ptr::<()>::new(e.mem.alloc(0x10));
        e.mem.set_u32(block.addr() + 0x0c, 0xffff_ffff);
        let log = logged(&mut e, |e| {
            let result = e.call(0x0041_b650, &args![block]).u32();
            assert_eq!(result, block.addr());
        });
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_INIT),
            vec![vec![block.addr(), 0x7a]]
        );
        assert_eq!(e.mem.u32(block.addr()), VTABLE_EXTRA_HEALTH_PERC);
        assert_eq!(e.mem.u32(block.addr() + 0x0c), 0);
    }

    #[test]
    fn health_perc_scalar_deleting_destructor_deletes_only_with_the_flag() {
        let mut e = engine();
        let block = Ptr::<()>::new(e.mem.alloc(0x10));
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041_b680, &args![block, 0u32]).u32(), block.addr());
        });
        assert_eq!(
            calls_to(&log, EXTRA_DATA_BASE_DESTRUCTOR),
            vec![vec![block.addr()]]
        );
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            e.call(0x0041_b680, &args![block, 1u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![block.addr()]]);
    }

    #[test]
    fn object_health_getter_defaults_to_minus_one() {
        let mut e = engine();
        let (empty, _) = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0041_b6b0, &args![empty]).f32(), -1.0);
        let (list, _) = list_with(&mut e, &[(EXTRA_OBJECT_HEALTH, 12.5f32.to_bits())]);
        assert_eq!(e.call(0x0041_b6b0, &args![list]).f32(), 12.5);
    }

    #[test]
    fn object_health_setter_builds_or_stores() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_b6e0, &args![list, 2.5f32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        let extra = find_extra(&mut e, list, EXTRA_OBJECT_HEALTH);
        assert_eq!(e.mem.u32(extra.addr()), VTABLE_EXTRA_OBJECT_HEALTH);
        assert_eq!(e.mem.f32(extra.addr() + 0x0c), 2.5);
        let log = logged(&mut e, |e| {
            e.call(0x0041_b6e0, &args![list, 7.0f32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.mem.f32(extra.addr() + 0x0c), 7.0);
    }

    #[test]
    fn object_health_constructor_stores_the_health() {
        let mut e = engine();
        let block = Ptr::<()>::new(e.mem.alloc(0x10));
        let log = logged(&mut e, |e| {
            let result = e.call(0x0041_b790, &args![block, 3.5f32]).u32();
            assert_eq!(result, block.addr());
        });
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_INIT),
            vec![vec![block.addr(), 0x56]]
        );
        assert_eq!(e.mem.u32(block.addr()), VTABLE_EXTRA_OBJECT_HEALTH);
        assert_eq!(e.mem.f32(block.addr() + 0x0c), 3.5);
    }

    #[test]
    fn object_health_remover() {
        check_remover(0x0041_b7c0, EXTRA_OBJECT_HEALTH);
    }

    #[test]
    fn cell_3d_setter_covers_node_cases() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        // Null without an extra data: nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b7e0, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert!(calls_to(&log, NI_POINTER_ASSIGN).is_empty());
        // A node: the extra data is built (0x10 bytes, no payload words) and
        // the pointer assigned.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b7e0, &args![list, 0x1000u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        let extra = find_extra(&mut e, list, EXTRA_CELL_3D);
        assert_eq!(calls_to(&log, EXTRA_CELL_3D_INIT).len(), 1);
        assert_eq!(
            calls_to(&log, NI_POINTER_ASSIGN),
            vec![vec![extra.addr() + 0x0c, 0x1000]]
        );
        // Another node: assigned.
        e.call(0x0041_b7e0, &args![list, 0x2000u32]);
        assert_eq!(word_at(&e, extra, 0x0c), 0x2000);
        // Null while the pointer is set: assigned, the extra data stays.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b7e0, &args![list, 0u32]);
        });
        assert!(deleted(&log).is_empty());
        assert_eq!(word_at(&e, extra, 0x0c), 0);
        assert_eq!(sorted_types(&e, list), vec![EXTRA_CELL_3D]);
        // Null while the pointer is already null: deleted.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b7e0, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn havok_world_setter_builds_assigns_or_removes() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_b8d0, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // A world builds it (0x14 bytes, from the world) and assigns nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b8d0, &args![list, 0x1111u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(calls_to(&log, EXTRA_HAVOK_INIT).len(), 1);
        assert_eq!(calls_to(&log, EXTRA_HAVOK_INIT)[0][1], 0x1111);
        assert!(calls_to(&log, NI_POINTER_ASSIGN).is_empty());
        let extra = find_extra(&mut e, list, EXTRA_HAVOK);
        // Another world: assigned at +0x0C.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b8d0, &args![list, 0x2222u32]);
        });
        assert_eq!(
            calls_to(&log, NI_POINTER_ASSIGN),
            vec![vec![extra.addr() + 0x0c, 0x2222]]
        );
        // Null: deleted.
        let log = logged(&mut e, |e| {
            e.call(0x0041_b8d0, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
    }

    /// Checks a getter of the `NiPointer` at `offset` of the Havok extra data.
    fn check_havok_getter(address: u32, offset: u32) {
        let mut e = engine();
        let (list, extras) = list_with(&mut e, &[(EXTRA_HAVOK, 0)]);
        e.mem.set_u32(extras[0].addr() + offset, 0x3333);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(address, &args![list]).u32(), 0x3333);
        });
        assert!(calls_to(&log, NI_POINTER_INIT).is_empty());
        // Without the extra data: a temporary NiPointer made from null, read
        // and destroyed.
        let (empty, _) = list_with(&mut e, &[(other_type(EXTRA_HAVOK), 0)]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(address, &args![empty]).u32(), 0);
        });
        let init = calls_to(&log, NI_POINTER_INIT);
        let destroy = calls_to(&log, NI_POINTER_DESTROY);
        assert_eq!(init.len(), 1);
        assert_eq!(init[0][1], 0);
        assert_eq!(destroy, vec![vec![init[0][0]]]);
    }

    #[test]
    fn havok_world_getter() {
        check_havok_getter(0x0041_b9a0, 0x0c);
    }

    #[test]
    fn havok_cell_mopp_getter() {
        check_havok_getter(0x0041_bb10, 0x10);
    }

    #[test]
    fn havok_cell_mopp_setter_builds_from_a_null_world_then_assigns() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_ba60, &args![list, 0x4444u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(calls_to(&log, EXTRA_HAVOK_INIT)[0][1], 0);
        let extra = find_extra(&mut e, list, EXTRA_HAVOK);
        assert_eq!(
            calls_to(&log, NI_POINTER_ASSIGN),
            vec![vec![extra.addr() + 0x10, 0x4444]]
        );
        // Existing: only assigned, null too (the extra data stays).
        let log = logged(&mut e, |e| {
            e.call(0x0041_ba60, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(
            calls_to(&log, NI_POINTER_ASSIGN),
            vec![vec![extra.addr() + 0x10, 0]]
        );
        assert_eq!(word_at(&e, extra, 0x10), 0);
        assert_eq!(sorted_types(&e, list), vec![EXTRA_HAVOK]);
    }

    #[test]
    fn region_list_setter_destroys_the_replaced_list() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        // Null without an extra data: nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0041_bbd0, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // A list builds it.
        let log = logged(&mut e, |e| {
            e.call(0x0041_bbd0, &args![list, 0x4000u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, EXTRA_REGION_LIST_INIT)[0][1], 0x4000);
        let extra = find_extra(&mut e, list, EXTRA_REGION_LIST);
        // A different old list (an object with a vtable) is destroyed with
        // flag 1 before the new one is stored.
        let old: Ptr = Ptr::new(e.mem.alloc(0x10));
        e.mem.set_u32(old.addr(), VTABLE);
        e.mem.set_u32(extra.addr() + 0x0c, old.addr());
        let log = logged(&mut e, |e| {
            e.call(0x0041_bbd0, &args![list, 0x5000u32]);
        });
        assert_eq!(calls_to(&log, DESTRUCTOR), vec![vec![old.addr(), 1]]);
        assert_eq!(word_at(&e, extra, 0x0c), 0x5000);
        // The same list again: no destruction. A null old list neither.
        let log = logged(&mut e, |e| {
            e.call(0x0041_bbd0, &args![list, 0x5000u32]);
        });
        assert!(deleted(&log).is_empty());
        e.mem.set_u32(extra.addr() + 0x0c, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0041_bbd0, &args![list, 0x6000u32]);
        });
        assert!(deleted(&log).is_empty());
        assert_eq!(word_at(&e, extra, 0x0c), 0x6000);
        // Null deletes the extra data.
        let log = logged(&mut e, |e| {
            e.call(0x0041_bbd0, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn region_list_getter() {
        check_word_getter(0x0041_bce0, EXTRA_REGION_LIST);
    }

    #[test]
    fn cell_music_type_setter() {
        check_word_setter(
            0x0041_bd10,
            EXTRA_CELL_MUSIC_TYPE,
            EXTRA_CELL_MUSIC_TYPE_INIT,
        );
    }

    #[test]
    fn cell_music_type_getter() {
        check_word_getter(0x0041_bde0, EXTRA_CELL_MUSIC_TYPE);
    }

    #[test]
    fn terminal_state_setter_builds_stores_clears_and_removes() {
        let mut e = engine();
        make_deletable(&mut e, VTABLE_EXTRA_TERMINAL_STATE);
        let (list, _) = list_with(&mut e, &[]);
        // Zero without an extra data: nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0041_be10, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // Non-zero builds it with the lock override at 0xFF.
        let log = logged(&mut e, |e| {
            e.call(0x0041_be10, &args![list, 2u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        let extra = find_extra(&mut e, list, EXTRA_TERMINAL_STATE);
        assert_eq!(e.mem.u8(extra.addr() + 0x0c), 2);
        assert_eq!(e.mem.u8(extra.addr() + 0x0d), 0xff);
        // Non-zero again: stored.
        e.call(0x0041_be10, &args![list, 3u32]);
        assert_eq!(e.mem.u8(extra.addr() + 0x0c), 3);
        // Zero with a lock override set: cleared, kept.
        e.mem.set_u8(extra.addr() + 0x0d, 5);
        let log = logged(&mut e, |e| {
            e.call(0x0041_be10, &args![list, 0u32]);
        });
        assert!(deleted(&log).is_empty());
        assert_eq!(e.mem.u8(extra.addr() + 0x0c), 0);
        // Zero with nothing else in it: deleted.
        e.mem.set_u8(extra.addr() + 0x0c, 4);
        e.mem.set_u8(extra.addr() + 0x0d, 0xff);
        let log = logged(&mut e, |e| {
            e.call(0x0041_be10, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn terminal_state_constructor() {
        let mut e = engine();
        let block = Ptr::<()>::new(e.mem.alloc(0x10));
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041_bef0, &args![block]).u32(), block.addr());
        });
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_INIT),
            vec![vec![block.addr(), 0x50]]
        );
        assert_eq!(e.mem.u32(block.addr()), VTABLE_EXTRA_TERMINAL_STATE);
        assert_eq!(e.mem.u8(block.addr() + 0x0c), 0);
        assert_eq!(e.mem.u8(block.addr() + 0x0d), 0xff);
    }

    #[test]
    fn terminal_state_emptiness_test() {
        let mut e = engine();
        let block = Ptr::<()>::new(e.mem.alloc(0x10));
        for (state, lock_override, empty) in [
            (0u8, 0xffu8, true),
            (1, 0xff, false),
            (0, 0, false),
            (0, 0x7f, false),
        ] {
            e.mem.set_u8(block.addr() + 0x0c, state);
            e.mem.set_u8(block.addr() + 0x0d, lock_override);
            assert_eq!(e.call(0x0041_bf20, &args![block]).bool(), empty);
        }
    }

    #[test]
    fn terminal_state_getter() {
        let mut e = engine();
        let (empty, _) = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0041_bf50, &args![empty]).u32() & 0xff, 0);
        let (list, _) = list_with(&mut e, &[(EXTRA_TERMINAL_STATE, 3)]);
        assert_eq!(e.call(0x0041_bf50, &args![list]).u32() & 0xff, 3);
    }

    #[test]
    fn terminal_lock_override_setter() {
        let mut e = engine();
        make_deletable(&mut e, VTABLE_EXTRA_TERMINAL_STATE);
        let (list, _) = list_with(&mut e, &[]);
        // Zero without an extra data: nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0041_bf80, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // A value builds it and is stored at +0x0D.
        e.call(0x0041_bf80, &args![list, 7u32]);
        let extra = find_extra(&mut e, list, EXTRA_TERMINAL_STATE);
        assert_eq!(e.mem.u8(extra.addr() + 0x0c), 0);
        assert_eq!(e.mem.u8(extra.addr() + 0x0d), 7);
        // 0xFF (a non-zero byte) is stored too.
        e.call(0x0041_bf80, &args![list, 0xffu32]);
        assert_eq!(e.mem.u8(extra.addr() + 0x0d), 0xff);
        // Zero stores 0; the extra data is deleted only when the byte is 0xFF
        // and the state 0, which a stored 0 never is.
        let log = logged(&mut e, |e| {
            e.call(0x0041_bf80, &args![list, 0u32]);
        });
        assert!(deleted(&log).is_empty());
        assert_eq!(e.mem.u8(extra.addr() + 0x0d), 0);
        assert_eq!(sorted_types(&e, list), vec![EXTRA_TERMINAL_STATE]);
    }

    #[test]
    fn terminal_lock_override_getter() {
        let mut e = engine();
        let (empty, _) = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0041_c060, &args![empty]).u32() & 0xff, 0xff);
        let (list, extras) = list_with(&mut e, &[(EXTRA_TERMINAL_STATE, 0)]);
        e.mem.set_u8(extras[0].addr() + 0x0d, 9);
        assert_eq!(e.call(0x0041_c060, &args![list]).u32() & 0xff, 9);
    }

    #[test]
    fn acoustic_space_setter() {
        check_word_setter(
            0x0041_c090,
            EXTRA_CELL_ACOUSTIC_SPACE,
            EXTRA_CELL_ACOUSTIC_SPACE_INIT,
        );
    }

    #[test]
    fn acoustic_space_getter() {
        check_word_getter(0x0041_c160, EXTRA_CELL_ACOUSTIC_SPACE);
    }

    #[test]
    fn climate_setter() {
        check_word_setter(0x0041_c190, EXTRA_CLIMATE, EXTRA_CLIMATE_INIT);
    }

    #[test]
    fn climate_getter() {
        check_word_getter(0x0041_c260, EXTRA_CLIMATE);
    }

    #[test]
    fn image_space_setter() {
        check_word_setter(0x0041_c290, EXTRA_IMAGE_SPACE, EXTRA_IMAGE_SPACE_INIT);
    }

    #[test]
    fn image_space_getter() {
        check_word_getter(0x0041_c360, EXTRA_IMAGE_SPACE);
    }

    #[test]
    fn impact_swap_setter_stores_after_construction() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_c390, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // The constructor takes no payload; the value is stored afterwards.
        let log = logged(&mut e, |e| {
            e.call(0x0041_c390, &args![list, 0x1234u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, EXTRA_IMPACT_SWAP_INIT).len(), 1);
        assert_eq!(calls_to(&log, EXTRA_IMPACT_SWAP_INIT)[0].len(), 1);
        let extra = find_extra(&mut e, list, EXTRA_IMPACT_SWAP);
        assert_eq!(word_at(&e, extra, 0x0c), 0x1234);
        // Existing: stored.
        e.call(0x0041_c390, &args![list, 0x5678u32]);
        assert_eq!(word_at(&e, extra, 0x0c), 0x5678);
        // Null: deleted.
        let log = logged(&mut e, |e| {
            e.call(0x0041_c390, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn impact_swap_getter() {
        check_word_getter(0x0041_c460, EXTRA_IMPACT_SWAP);
    }

    #[test]
    fn canopy_shadow_mask_setter() {
        let mut e = engine();
        let out = Ptr::<()>::new(e.mem.alloc(4));
        e.mem.set_u32(out.addr(), 0xdead);
        let (list, _) = list_with(&mut e, &[]);
        // State 0 without an extra data: nothing, `out` untouched.
        e.call(0x0041_c490, &args![list, 0u32, 0u32, out]);
        assert!(sorted_types(&e, list).is_empty());
        assert_eq!(e.mem.u32(out.addr()), 0xdead);
        // A state and a mask: built (0x1C bytes) from both; `out` receives
        // the address of the rect at +0x14.
        let log = logged(&mut e, |e| {
            e.call(0x0041_c490, &args![list, 2u32, 0x7000u32, out]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x1c]]);
        let built = calls_to(&log, EXTRA_CELL_CANOPY_SHADOW_MASK_INIT);
        assert_eq!((built[0][1], built[0][2]), (2, 0x7000));
        let extra = find_extra(&mut e, list, EXTRA_CELL_CANOPY_SHADOW_MASK);
        assert_eq!(e.mem.u32(out.addr()), extra.addr() + 0x14);
        // Existing: the state is stored and the mask assigned.
        let log = logged(&mut e, |e| {
            e.call(0x0041_c490, &args![list, 3u32, 0x8000u32, out]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(word_at(&e, extra, 0x0c), 3);
        assert_eq!(
            calls_to(&log, NI_POINTER_ASSIGN),
            vec![vec![extra.addr() + 0x10, 0x8000]]
        );
        // State 0 deletes it and leaves `out` alone.
        e.mem.set_u32(out.addr(), 0xbeef);
        let log = logged(&mut e, |e| {
            e.call(0x0041_c490, &args![list, 0u32, 0x9000u32, out]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert_eq!(e.mem.u32(out.addr()), 0xbeef);
    }

    #[test]
    fn canopy_shadow_mask_getter() {
        let mut e = engine();
        let mask_out = Ptr::<()>::new(e.mem.alloc(4));
        let rect_out = Ptr::<()>::new(e.mem.alloc(4));
        e.mem.set_u32(mask_out.addr(), 0xdead);
        e.mem.set_u32(rect_out.addr(), 0xdead);
        let (empty, _) = list_with(&mut e, &[]);
        let state = e.call(0x0041_c580, &args![empty, mask_out, rect_out]).u32();
        assert_eq!(state, 0);
        assert_eq!(e.mem.u32(mask_out.addr()), 0);
        assert_eq!(e.mem.u32(rect_out.addr()), 0);
        let (list, extras) = list_with(&mut e, &[(EXTRA_CELL_CANOPY_SHADOW_MASK, 5)]);
        e.mem.set_u32(extras[0].addr() + 0x10, 0x7777);
        let state = e.call(0x0041_c580, &args![list, mask_out, rect_out]).u32();
        assert_eq!(state, 5);
        assert_eq!(e.mem.u32(mask_out.addr()), 0x7777);
        assert_eq!(e.mem.u32(rect_out.addr()), extras[0].addr() + 0x14);
    }

    #[test]
    fn editor_id_constructor_builds_the_string() {
        let mut e = engine();
        let block = Ptr::<()>::new(e.mem.alloc(0x14));
        let log = logged(&mut e, |e| {
            assert_eq!(
                e.call(0x0041_c5e0, &args![block, 0x9000u32]).u32(),
                block.addr()
            );
        });
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_INIT),
            vec![vec![block.addr(), 0x06]]
        );
        assert_eq!(e.mem.u32(block.addr()), VTABLE_EXTRA_EDITOR_ID);
        assert_eq!(
            calls_to(&log, BS_STRING_INIT),
            vec![vec![block.addr() + 0x0c]]
        );
        assert_eq!(
            calls_to(&log, BS_STRING_SET),
            vec![vec![block.addr() + 0x0c, 0x9000, 0]]
        );
    }

    #[test]
    fn editor_id_destructor_runs_the_member_then_the_base() {
        let mut e = engine();
        let block = Ptr::<()>::new(e.mem.alloc(0x14));
        let log = logged(&mut e, |e| {
            e.call(0x0041_c690, &args![block]);
        });
        let order: Vec<u32> = log.iter().map(|(callee, _)| *callee).collect();
        assert_eq!(
            order,
            vec![0x0041_c690, BS_STRING_DESTROY, BS_EXTRA_DATA_DESTROY]
        );
        assert_eq!(
            calls_to(&log, BS_STRING_DESTROY),
            vec![vec![block.addr() + 0x0c]]
        );
        assert_eq!(
            calls_to(&log, BS_EXTRA_DATA_DESTROY),
            vec![vec![block.addr()]]
        );
    }

    #[test]
    fn editor_id_scalar_deleting_destructor_deletes_only_with_the_flag() {
        let mut e = engine();
        let block = Ptr::<()>::new(e.mem.alloc(0x14));
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041_c660, &args![block, 0u32]).u32(), block.addr());
        });
        assert_eq!(calls_to(&log, BS_STRING_DESTROY).len(), 1);
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        let log = logged(&mut e, |e| {
            e.call(0x0041_c660, &args![block, 1u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_DELETE), vec![vec![block.addr()]]);
    }

    #[test]
    fn editor_id_setter_builds_sets_or_removes() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_c6f0, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // A string builds it (0x14 bytes) through the constructor.
        let log = logged(&mut e, |e| {
            e.call(0x0041_c6f0, &args![list, 0xa000u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        let extra = find_extra(&mut e, list, EXTRA_EDITOR_ID);
        assert_eq!(
            calls_to(&log, BS_STRING_SET),
            vec![vec![extra.addr() + 0x0c, 0xa000, 0]]
        );
        // Existing: the string is set in it.
        let log = logged(&mut e, |e| {
            e.call(0x0041_c6f0, &args![list, 0xb000u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(
            calls_to(&log, BS_STRING_SET),
            vec![vec![extra.addr() + 0x0c, 0xb000, 0]]
        );
        // Null: deleted (the destructor in the test vtable).
        make_deletable(&mut e, VTABLE_EXTRA_EDITOR_ID);
        let log = logged(&mut e, |e| {
            e.call(0x0041_c6f0, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn editor_id_getter_reads_the_string_word() {
        let mut e = engine();
        let (empty, _) = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0041_c7c0, &args![empty]).u32(), 0);
        let (list, _) = list_with(&mut e, &[(EXTRA_EDITOR_ID, 0xc000)]);
        let log = logged(&mut e, |e| {
            assert_eq!(e.call(0x0041_c7c0, &args![list]).u32(), 0xc000);
        });
        assert_eq!(calls_to(&log, NI_POINTER_GET).len(), 1);
    }

    #[test]
    fn reference_pointer_setter_acts_only_for_persisting_references() {
        let mut e = engine();
        e.set_global::<u32>(SAVE_LOAD_GAME, 0x5000);
        e.register(GET_REF_PERSISTS, |_, a| returns((a[0] == 0x7000) as u32));
        let (list, _) = list_with(&mut e, &[]);
        // A reference that does not persist, the stub false: nothing.
        let log = logged(&mut e, |e| {
            e.call(0x0041_c7f0, &args![list, 0x6000u32]);
        });
        assert_eq!(calls_to(&log, SAVE_LOAD_GAME_STUB), vec![vec![0x5000]]);
        assert_eq!(calls_to(&log, GET_REF_PERSISTS), vec![vec![0x6000]]);
        assert!(sorted_types(&e, list).is_empty());
        // A persisting reference: built (0x10 bytes) from it.
        let log = logged(&mut e, |e| {
            e.call(0x0041_c7f0, &args![list, 0x7000u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, EXTRA_REFERENCE_POINTER_INIT)[0][1], 0x7000);
        let extra = find_extra(&mut e, list, EXTRA_REFERENCE_POINTER);
        // Existing: stored.
        e.call(0x0041_c7f0, &args![list, 0x7000u32]);
        assert_eq!(word_at(&e, extra, 0x0c), 0x7000);
        // Null: acts without asking whether it persists.
        let log = logged(&mut e, |e| {
            e.call(0x0041_c7f0, &args![list, 0u32]);
        });
        assert!(calls_to(&log, GET_REF_PERSISTS).is_empty());
        assert_eq!(word_at(&e, extra, 0x0c), 0);
        // The stub true: acts for a reference that does not persist.
        e.register(SAVE_LOAD_GAME_STUB, |_, _| returns(1));
        let log = logged(&mut e, |e| {
            e.call(0x0041_c7f0, &args![list, 0x6000u32]);
        });
        assert!(calls_to(&log, GET_REF_PERSISTS).is_empty());
        assert_eq!(word_at(&e, extra, 0x0c), 0x6000);
    }

    // -----------------------------------------------------------------------
    // Tests of the functions `0041c8d0` to `0041da40`

    /// A `TESPackage` stand-in: `flags` at +0x1C, the type byte at +0x20.
    fn package_object(e: &mut Engine, flags: u32, package_type: i8) -> Ptr {
        let package = Ptr::new(e.mem.alloc(0x40));
        e.mem.set_u32(package.addr() + 0x1c, flags);
        e.mem.set_u8(package.addr() + 0x20, package_type as u8);
        package
    }

    /// A `BSSimpleList` node `{item, next}`.
    fn list_node(e: &mut Engine, item: u32, next: u32) -> u32 {
        let node = e.mem.alloc(8);
        e.mem.set_u32(node, item);
        e.mem.set_u32(node + 4, next);
        node
    }

    /// The items of the node chain starting at `node`, up to a null next.
    fn node_items(e: &Engine, mut node: u32) -> Vec<u32> {
        let mut items = vec![];
        while node != 0 {
            items.push(e.mem.u32(node));
            node = e.mem.u32(node + 4);
        }
        items
    }

    #[test]
    fn reference_pointer_getter_and_remover() {
        check_word_getter(0x0041_c8d0, EXTRA_REFERENCE_POINTER);
        check_remover(0x0041_c900, EXTRA_REFERENCE_POINTER);
    }

    #[test]
    fn package_type_is_a_signed_byte() {
        let mut e = engine();
        let package = package_object(&mut e, 0, 0x18);
        assert_eq!(e.call(0x0041_ca90, &args![package]).u32(), 0x18);
        let package = package_object(&mut e, 0, -3);
        assert_eq!(e.call(0x0041_ca90, &args![package]).u32() as i32, -3);
    }

    #[test]
    fn package_extra_setter_builds_stores_and_removes() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let package = package_object(&mut e, 0, 5);
        let target = e.mem.alloc(0x10);
        // Missing: built (0x1C bytes) from the six values.
        let log = logged(&mut e, |e| {
            e.call(
                0x0041_c930,
                &args![list, package, 7u32, target, 1u32, 0u32, 1u32],
            );
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x1c]]);
        assert_eq!(
            calls_to(&log, EXTRA_PACKAGE_INIT)[0][1..],
            [package.addr(), 7, target, 1, 0, 1]
        );
        let extra = find_extra(&mut e, list, EXTRA_PACKAGE);
        assert_eq!(word_at(&e, extra, 0x0c), package.addr());
        assert_eq!(word_at(&e, extra, 0x10), 7);
        assert_eq!(word_at(&e, extra, 0x14), target);
        assert_eq!(e.mem.u8(extra.addr() + 0x18), 1);
        assert_eq!(e.mem.u8(extra.addr() + 0x19), 0);
        assert_eq!(e.mem.u8(extra.addr() + 0x1a), 1);
        // Present: the values are stored.
        let other = package_object(&mut e, 0, 6);
        let log = logged(&mut e, |e| {
            e.call(
                0x0041_c930,
                &args![list, other, 0xffff_fffeu32, 0x44u32, 0u32, 1u32, 0u32],
            );
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(word_at(&e, extra, 0x0c), other.addr());
        assert_eq!(word_at(&e, extra, 0x10) as i32, -2);
        assert_eq!(word_at(&e, extra, 0x14), 0x44);
        assert_eq!(e.mem.u8(extra.addr() + 0x18), 0);
        assert_eq!(e.mem.u8(extra.addr() + 0x19), 1);
        assert_eq!(e.mem.u8(extra.addr() + 0x1a), 0);
        // Null: the extra data goes.
        make_deletable(&mut e, VTABLE);
        let log = logged(&mut e, |e| {
            e.call(
                0x0041_c930,
                &args![list, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            );
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
        // Null with nothing to remove: nothing happens.
        let log = logged(&mut e, |e| {
            e.call(
                0x0041_c930,
                &args![list, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32],
            );
        });
        assert!(deleted(&log).is_empty());
    }

    #[test]
    fn package_extra_setter_ignores_unusable_packages() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        // Created (flag 0x800) and not of type 1; never to run (0x8000); of
        // type 0x18 or 0x17: all ignored.
        let created = package_object(&mut e, 0x800, 5);
        let never = package_object(&mut e, 0x8000, 5);
        let type_18 = package_object(&mut e, 0, 0x18);
        let type_17 = package_object(&mut e, 0, 0x17);
        for package in [created, never, type_18, type_17] {
            let log = logged(&mut e, |e| {
                e.call(
                    0x0041_c930,
                    &args![list, package, 1u32, 2u32, 0u32, 0u32, 0u32],
                );
            });
            assert!(calls_to(&log, OPERATOR_NEW).is_empty());
            assert!(sorted_types(&e, list).is_empty());
        }
        // Created but of type 1 is accepted.
        let created_one = package_object(&mut e, 0x800, 1);
        e.call(
            0x0041_c930,
            &args![list, created_one, 1u32, 2u32, 0u32, 0u32, 0u32],
        );
        assert_eq!(sorted_types(&e, list), vec![EXTRA_PACKAGE]);
        // A created type 1 package that never runs is still ignored.
        let (list, _) = list_with(&mut e, &[]);
        let created_never = package_object(&mut e, 0x8800, 1);
        e.call(
            0x0041_c930,
            &args![list, created_never, 1u32, 2u32, 0u32, 0u32, 0u32],
        );
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn package_extra_accessors() {
        let mut e = engine();
        let (list, extras) = list_with(&mut e, &[(other_type(EXTRA_PACKAGE), 0)]);
        // Without the extra data: zeros, and the setters do nothing.
        for getter in [
            0x0041_cb10u32,
            0x0041_cb40,
            0x0041_cb70,
            0x0041_cba0,
            0x0041_cc00,
            0x0041_cc30,
        ] {
            assert_eq!(e.call(getter, &args![list]).u32() & 0xff, 0);
        }
        for setter in [0x0041_cab0u32, 0x0041_cae0, 0x0041_cbd0] {
            e.call(setter, &args![list, 5u32]);
        }
        assert_eq!(word_at(&e, extras[0], 0x14), 0);
        // With it.
        let (list, extras) = list_with(&mut e, &[(EXTRA_PACKAGE, 0xaaa0)]);
        let extra = extras[0];
        e.call(0x0041_cab0, &args![list, 0xbbb0u32]);
        e.call(0x0041_cae0, &args![list, 0xffff_fff9u32]);
        e.call(0x0041_cbd0, &args![list, 0x7fu32]);
        assert_eq!(word_at(&e, extra, 0x14), 0xbbb0);
        assert_eq!(word_at(&e, extra, 0x10) as i32, -7);
        assert_eq!(e.mem.u8(extra.addr() + 0x18), 0x7f);
        assert_eq!(e.call(0x0041_cb10, &args![list]).u32(), 0xaaa0);
        assert_eq!(e.call(0x0041_cb40, &args![list]).u32() as i32, -7);
        assert_eq!(e.call(0x0041_cb70, &args![list]).u32(), 0xbbb0);
        assert_eq!(e.call(0x0041_cba0, &args![list]).u32() & 0xff, 0x7f);
        e.mem.set_u8(extra.addr() + 0x19, 1);
        e.mem.set_u8(extra.addr() + 0x1a, 2);
        assert_eq!(e.call(0x0041_cc00, &args![list]).u32() & 0xff, 1);
        assert_eq!(e.call(0x0041_cc30, &args![list]).u32() & 0xff, 2);
        // The remover.
        check_remover(0x0041_cc60, EXTRA_PACKAGE);
    }

    #[test]
    fn trespass_package_setter_builds_or_destroys_the_old_package() {
        const OLD_VTABLE: u32 = 0x0200_6000;
        const OLD_DESTRUCTOR: u32 = 0x0200_6100;
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_cc90, &args![list, 0x5550u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, EXTRA_TRESPASS_PACKAGE_INIT)[0][1], 0x5550);
        let extra = find_extra(&mut e, list, EXTRA_TRESPASS_PACKAGE);
        assert_eq!(word_at(&e, extra, 0x0c), 0x5550);
        // Present, holding a package: that one is destroyed (slot 0x10, 1).
        e.put_vtable(OLD_VTABLE, &[0, 0, 0, 0, OLD_DESTRUCTOR]);
        stub(&mut e, OLD_DESTRUCTOR);
        let old = e.mem.alloc(0x10);
        e.mem.set_u32(old, OLD_VTABLE);
        e.mem.set_u32(extra.addr() + 0x0c, old);
        let log = logged(&mut e, |e| {
            e.call(0x0041_cc90, &args![list, 0x6660u32]);
        });
        assert_eq!(calls_to(&log, OLD_DESTRUCTOR), vec![vec![old, 1]]);
        assert_eq!(word_at(&e, extra, 0x0c), 0x6660);
        // Present, holding null: just stored.
        e.mem.set_u32(extra.addr() + 0x0c, 0);
        let log = logged(&mut e, |e| {
            e.call(0x0041_cc90, &args![list, 0x7770u32]);
        });
        // Only the call itself and the extra data lookup.
        assert_eq!(log.len(), 2);
        assert_eq!(word_at(&e, extra, 0x0c), 0x7770);
        check_word_getter(0x0041_cd70, EXTRA_TRESPASS_PACKAGE);
    }

    #[test]
    fn trespass_package_remover_marks_the_package_created() {
        let mut e = engine();
        e.set_global::<u32>(SAVE_LOAD_GAME, 0x5000);
        // Nothing to remove: no calls.
        let (empty, _) = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_cda0, &args![empty]);
        });
        // Only the call itself and the extra data lookup.
        assert_eq!(log.len(), 2);
        // The stub false: the package is marked, the form is not deleted.
        make_deletable(&mut e, VTABLE);
        let (list, extras) = list_with(&mut e, &[(EXTRA_TRESPASS_PACKAGE, 0x8880)]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_cda0, &args![list]);
        });
        assert_eq!(
            calls_to(&log, PACKAGE_SET_IS_CREATED),
            vec![vec![0x8880, 1]]
        );
        assert!(calls_to(&log, SAVE_LOAD_GAME_DELETE_FORM).is_empty());
        assert_eq!(word_at(&e, extras[0], 0x0c), 0);
        assert_eq!(deleted(&log), vec![extras[0].addr()]);
        assert!(sorted_types(&e, list).is_empty());
        // The stub true: the form goes through the save/load singleton.
        e.register(SAVE_LOAD_GAME_STUB, |_, _| returns(1));
        let (list, _) = list_with(&mut e, &[(EXTRA_TRESPASS_PACKAGE, 0x9990)]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_cda0, &args![list]);
        });
        assert_eq!(
            calls_to(&log, SAVE_LOAD_GAME_DELETE_FORM),
            vec![vec![0x5000, 0x9990]]
        );
    }

    #[test]
    fn trespass_package_clearing_needs_the_same_package() {
        let mut e = engine();
        let (empty, _) = list_with(&mut e, &[]);
        e.call(0x0041_ce10, &args![empty, 0x1110u32]);
        let (list, extras) = list_with(&mut e, &[(EXTRA_TRESPASS_PACKAGE, 0x1110)]);
        e.call(0x0041_ce10, &args![list, 0x2220u32]);
        assert_eq!(word_at(&e, extras[0], 0x0c), 0x1110);
        e.call(0x0041_ce10, &args![list, 0x1110u32]);
        assert_eq!(word_at(&e, extras[0], 0x0c), 0);
        assert_eq!(sorted_types(&e, list), vec![EXTRA_TRESPASS_PACKAGE]);
    }

    #[test]
    fn crime_list_adder_builds_or_adds_at_the_head() {
        const SEEN: u32 = 0x0200_5000;
        let mut e = engine();
        e.map(SEEN, 0x10);
        e.register(LIST_ADD_HEAD, |e, a| {
            let crime = e.mem.u32(a[1]);
            e.mem.set_u32(SEEN, crime);
            e.mem.set_u32(SEEN + 4, a[0]);
            Ret::default()
        });
        let (list, _) = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_ce50, &args![list, 0xc0deu32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        assert_eq!(calls_to(&log, EXTRA_PLAYER_CRIME_LIST_INIT)[0][1], 0xc0de);
        let extra = find_extra(&mut e, list, EXTRA_PLAYER_CRIME_LIST);
        assert_eq!(word_at(&e, extra, 0x0c), 0xc0de);
        // Present: the head of its list gets the crime through a stack word.
        e.mem.set_u32(extra.addr() + 0x0c, 0x7770);
        let log = logged(&mut e, |e| {
            e.call(0x0041_ce50, &args![list, 0xbeefu32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&log, LIST_ADD_HEAD).len(), 1);
        assert_eq!(e.mem.u32(SEEN), 0xbeef);
        assert_eq!(e.mem.u32(SEEN + 4), 0x7770);
        check_word_getter(0x0041_cf00, EXTRA_PLAYER_CRIME_LIST);
    }

    #[test]
    fn crime_list_remover_walks_frees_and_deletes() {
        let mut e = engine();
        make_deletable(&mut e, VTABLE);
        // Three nodes; the third has an empty item and ends the walk.
        let third = list_node(&mut e, 0, 0);
        let second = list_node(&mut e, 0xc2, third);
        let first = list_node(&mut e, 0xc1, second);
        let (list, extras) = list_with(&mut e, &[(EXTRA_PLAYER_CRIME_LIST, first)]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_cf30, &args![list, 0x99u32]);
        });
        assert_eq!(
            calls_to(&log, CRIME_REMOVE_ENTRY),
            vec![vec![0xc1, 0x99], vec![0xc2, 0x99]]
        );
        assert_eq!(calls_to(&log, SIMPLE_LIST_FREE_NODES), vec![vec![first]]);
        assert_eq!(
            calls_to(&log, SIMPLE_LIST_SCALAR_DELETING_DESTRUCTOR),
            vec![vec![first, 1]]
        );
        assert_eq!(word_at(&e, extras[0], 0x0c), 0);
        assert_eq!(deleted(&log), vec![extras[0].addr()]);
        assert!(sorted_types(&e, list).is_empty());
        // No crime list in it: the extra data is just deleted.
        let (list, extras) = list_with(&mut e, &[(EXTRA_PLAYER_CRIME_LIST, 0)]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_cf30, &args![list, 0x99u32]);
        });
        assert!(calls_to(&log, SIMPLE_LIST_FREE_NODES).is_empty());
        assert_eq!(deleted(&log), vec![extras[0].addr()]);
        // No extra data: nothing.
        let (empty, _) = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_cf30, &args![empty, 0x99u32]);
        });
        // Only the call itself and the extra data lookup.
        assert_eq!(log.len(), 2);
    }

    #[test]
    fn leveled_item_remover() {
        check_remover(0x0041_d000, EXTRA_LEVELED_ITEM);
    }

    #[test]
    fn ownership_only_scan_under_the_lock() {
        let mut e = engine();
        let cases: [(&[u8], bool); 7] = [
            (&[0x21], true),
            (
                &[0x21, 0x1c, 0x24, 0x0d, 0x20, 0x2f, 0x30, 0x4a, 0x27],
                true,
            ),
            (&[0x1c, 0x24], false),
            (&[], false),
            (&[0x21, 0x05], false),
            (&[0x05, 0x21], false),
            (&[0x21, 0x16], false),
        ];
        for (types, expected) in cases {
            let entries: Vec<(u8, u32)> = types.iter().map(|&t| (t, 0)).collect();
            let (list, _) = list_with(&mut e, &entries);
            let log = logged(&mut e, |e| {
                assert_eq!(e.call(0x0041_d030, &args![list]).bool(), expected);
            });
            assert_eq!(calls_to(&log, LOCK), vec![vec![EXTRA_CRIT_SECTION, 0]]);
            assert_eq!(calls_to(&log, UNLOCK), vec![vec![EXTRA_CRIT_SECTION]]);
        }
    }

    #[test]
    fn default_for_container_scan_depends_on_the_flag() {
        let mut e = engine();
        let cases: [(&[u8], u32, bool); 8] = [
            (&[], 0, true),
            (
                &[0x21, 0x1c, 0x24, 0x0d, 0x20, 0x2f, 0x30, 0x4a, 0x27],
                0,
                true,
            ),
            (&[0x16], 0, false),
            (&[0x16], 1, true),
            (&[0x21, 0x16, 0x24], 1, true),
            (&[0x05], 0, false),
            (&[0x05], 1, false),
            (&[0x21, 0x05], 1, false),
        ];
        for (types, flag, expected) in cases {
            let entries: Vec<(u8, u32)> = types.iter().map(|&t| (t, 0)).collect();
            let (list, _) = list_with(&mut e, &entries);
            let log = logged(&mut e, |e| {
                assert_eq!(e.call(0x0041_d120, &args![list, flag]).bool(), expected);
            });
            assert_eq!(calls_to(&log, LOCK).len(), 1);
            assert_eq!(calls_to(&log, UNLOCK).len(), 1);
        }
    }

    #[test]
    fn leveled_item_index_and_default_flag() {
        let mut e = engine();
        let (empty, _) = list_with(&mut e, &[]);
        assert_eq!(e.call(0x0041_d360, &args![empty]).u32(), 0xffff_ffff);
        // Setting the flag without the extra data does nothing.
        e.call(0x0041_d330, &args![empty, 1u32]);
        assert!(sorted_types(&e, empty).is_empty());
        // Missing: built (0x14 bytes) from the index, flag cleared.
        let log = logged(&mut e, |e| {
            e.call(0x0041_d280, &args![empty, 4u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        assert_eq!(calls_to(&log, EXTRA_LEVELED_ITEM_INIT)[0][1], 4);
        let extra = find_extra(&mut e, empty, EXTRA_LEVELED_ITEM);
        assert_eq!(e.call(0x0041_d360, &args![empty]).u32(), 4);
        assert_eq!(e.mem.u8(extra.addr() + 0x10), 0);
        // The flag setter, then the index setter clears it again.
        e.call(0x0041_d330, &args![empty, 1u32]);
        assert_eq!(e.mem.u8(extra.addr() + 0x10), 1);
        let log = logged(&mut e, |e| {
            e.call(0x0041_d280, &args![empty, 9u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(e.call(0x0041_d360, &args![empty]).u32(), 9);
        assert_eq!(e.mem.u8(extra.addr() + 0x10), 0);
    }

    #[test]
    fn persistent_cell_setter_and_getter() {
        check_word_setter(
            0x0041_d390,
            EXTRA_PERSISTENT_CELL,
            EXTRA_PERSISTENT_CELL_INIT,
        );
        check_word_getter(0x0041_d460, EXTRA_PERSISTENT_CELL);
    }

    /// Checks a ragdoll setter of `address` whose `fill` callee takes
    /// `(data, source)`.
    fn check_rag_doll_setter(address: u32, fill: u32) {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        // Nothing, null: nothing.
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // Missing, a source: both objects are made, the data is filled
        // after it is stored in the extra data, and the extra data is added.
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0x5150u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10], vec![0x14]]);
        let extra = find_extra(&mut e, list, EXTRA_RAG_DOLL_DATA);
        let data = word_at(&e, extra, 0x0c);
        assert_ne!(data, 0);
        assert_eq!(calls_to(&log, RAGDOLL_DATA_INIT), vec![vec![data]]);
        assert_eq!(calls_to(&log, fill), vec![vec![data, 0x5150]]);
        // Present, a source: the held data is filled again.
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0x6160u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(calls_to(&log, fill), vec![vec![data, 0x6160]]);
        // Present, null: the extra data is deleted.
        make_deletable(&mut e, VTABLE);
        let log = logged(&mut e, |e| {
            e.call(address, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(calls_to(&log, fill).is_empty());
        assert!(sorted_types(&e, list).is_empty());
    }

    #[test]
    fn rag_doll_setters_and_getter() {
        check_rag_doll_setter(0x0041_d490, RAG_DOLL_UPDATE_FROM_REFERENCE);
        check_rag_doll_setter(0x0041_d5b0, RAG_DOLL_COPY);
        check_word_getter(0x0041_d6d0, EXTRA_RAG_DOLL_DATA);
    }

    #[test]
    fn run_once_package_adder_makes_the_extra_data_once() {
        let mut e = engine();
        let (list, _) = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_d700, &args![list, 0x4440u32, 1u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x10]]);
        let extra = find_extra(&mut e, list, EXTRA_RUN_ONCE_PACKAGES);
        assert_eq!(
            calls_to(&log, RUN_ONCE_PACKAGES_ADD),
            vec![vec![extra.addr(), 0x4440, 1]]
        );
        let log = logged(&mut e, |e| {
            e.call(0x0041_d700, &args![list, 0x5550u32, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(
            calls_to(&log, RUN_ONCE_PACKAGES_ADD),
            vec![vec![extra.addr(), 0x5550, 0]]
        );
    }

    /// A run-once entry `{package, flag byte}`.
    fn run_once_entry(e: &mut Engine, package: u32, flag: u8) -> u32 {
        let entry = e.mem.alloc(8);
        e.mem.set_u32(entry, package);
        e.mem.set_u8(entry + 4, flag);
        entry
    }

    /// A package whose time stamp (+0x38) has the byte at +3 and the word at
    /// +4 given.
    fn timed_package(e: &mut Engine, byte: u8, word: u32) -> u32 {
        let package = e.mem.alloc(0x40);
        e.mem.set_u8(package + 0x38 + 3, byte);
        e.mem.set_u32(package + 0x38 + 4, word);
        package
    }

    #[test]
    fn package_time_stamp_is_at_plus_0x38() {
        let mut e = engine();
        assert_eq!(e.call(0x0041_d8a0, &args![0x1000u32]).u32(), 0x1038);
    }

    #[test]
    fn run_once_cleanup_keeps_fresh_matching_entries() {
        let mut e = engine();
        e.register(TIME_STAMP_BYTE, |e, a| returns(e.mem.u8(a[0] + 3) as u32));
        // Unlinks the node whose item equals the word at `a[1]`; the head
        // node is replaced by the contents of the next one.
        e.register(SIMPLE_LIST_REMOVE_VALUE, |e, a| {
            let (head, target) = (a[0], e.mem.u32(a[1]));
            if e.mem.u32(head) == target {
                let next = e.mem.u32(head + 4);
                let (item, after) = if next == 0 {
                    (0, 0)
                } else {
                    (e.mem.u32(next), e.mem.u32(next + 4))
                };
                e.mem.set_u32(head, item);
                e.mem.set_u32(head + 4, after);
            } else {
                let mut previous = head;
                loop {
                    let node = e.mem.u32(previous + 4);
                    if node == 0 {
                        break;
                    }
                    if e.mem.u32(node) == target {
                        let after = e.mem.u32(node + 4);
                        e.mem.set_u32(previous + 4, after);
                        break;
                    }
                    previous = node;
                }
            }
            Ret::default()
        });
        // The byte at +3 is signed and added to the word at +4: 0x15 or more
        // expires the entry.
        let fresh = timed_package(&mut e, 0x0a, 0x0a);
        let expired = timed_package(&mut e, 0x0a, 0x0b);
        let negative_fresh = timed_package(&mut e, 0xff, 0x15);
        let negative_expired = timed_package(&mut e, 0xff, 0x16);
        let mismatch = run_once_entry(&mut e, fresh, 3);
        let keep_fresh = run_once_entry(&mut e, fresh, 1);
        let drop_expired = run_once_entry(&mut e, expired, 1);
        let keep_without_package = run_once_entry(&mut e, 0, 1);
        let keep_negative = run_once_entry(&mut e, negative_fresh, 1);
        let drop_negative = run_once_entry(&mut e, negative_expired, 1);
        let mut next = 0;
        let mut head = 0;
        for entry in [
            drop_negative,
            keep_negative,
            keep_without_package,
            drop_expired,
            keep_fresh,
            mismatch,
        ] {
            head = list_node(&mut e, entry, next);
            next = head;
        }
        let (list, _) = list_with(&mut e, &[(EXTRA_RUN_ONCE_PACKAGES, head)]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_d7b0, &args![list, 1u32]);
        });
        assert_eq!(
            calls_to(&log, OPERATOR_DELETE),
            vec![vec![mismatch], vec![drop_expired], vec![drop_negative]]
        );
        assert!(calls_to(&log, SIMPLE_LIST_REMOVE_VALUE)
            .iter()
            .all(|call| call[0] == head));
        assert_eq!(
            node_items(&e, head),
            vec![keep_fresh, keep_without_package, keep_negative]
        );
        // The flag is a signed byte: -1 matches the byte 0xff.
        let signed = run_once_entry(&mut e, fresh, 0xff);
        let node = list_node(&mut e, signed, 0);
        let (list, _) = list_with(&mut e, &[(EXTRA_RUN_ONCE_PACKAGES, node)]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_d7b0, &args![list, 0xffu32]);
        });
        assert!(calls_to(&log, OPERATOR_DELETE).is_empty());
        // No extra data, or an empty list: nothing.
        let (empty, _) = list_with(&mut e, &[]);
        e.call(0x0041_d7b0, &args![empty, 1u32]);
        let (zero, _) = list_with(&mut e, &[(EXTRA_RUN_ONCE_PACKAGES, 0)]);
        e.call(0x0041_d7b0, &args![zero, 1u32]);
    }

    #[test]
    fn run_once_membership_stops_at_an_empty_item() {
        let mut e = engine();
        let first = run_once_entry(&mut e, 0x111, 0);
        let second = run_once_entry(&mut e, 0x222, 0);
        let hidden = run_once_entry(&mut e, 0x333, 0);
        let hidden_node = list_node(&mut e, hidden, 0);
        let terminator = list_node(&mut e, 0, hidden_node);
        let second_node = list_node(&mut e, second, terminator);
        let head = list_node(&mut e, first, second_node);
        let (list, _) = list_with(&mut e, &[(EXTRA_RUN_ONCE_PACKAGES, head)]);
        assert!(e.call(0x0041_d8c0, &args![list, 0x111u32]).bool());
        assert!(e.call(0x0041_d8c0, &args![list, 0x222u32]).bool());
        assert!(!e.call(0x0041_d8c0, &args![list, 0x333u32]).bool());
        assert!(!e.call(0x0041_d8c0, &args![list, 0x444u32]).bool());
        let (empty, _) = list_with(&mut e, &[]);
        assert!(!e.call(0x0041_d8c0, &args![empty, 0x111u32]).bool());
        let (zero, _) = list_with(&mut e, &[(EXTRA_RUN_ONCE_PACKAGES, 0)]);
        assert!(!e.call(0x0041_d8c0, &args![zero, 0x111u32]).bool());
        check_remover(0x0041_d930, EXTRA_RUN_ONCE_PACKAGES);
    }

    #[test]
    fn distant_data_copies_the_normal() {
        let mut e = engine();
        let normal = e.mem.alloc(0x0c);
        for (index, value) in [1.0f32, -2.0, 0.5].iter().enumerate() {
            e.mem.set_u32(normal + 4 * index as u32, value.to_bits());
        }
        let (list, _) = list_with(&mut e, &[]);
        let log = logged(&mut e, |e| {
            e.call(0x0041_d950, &args![list, normal]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x18]]);
        assert_eq!(calls_to(&log, EXTRA_DISTANT_DATA_INIT).len(), 1);
        let extra = find_extra(&mut e, list, EXTRA_DISTANT_DATA);
        assert_eq!(f32::from_bits(word_at(&e, extra, 0x0c)), 1.0);
        assert_eq!(f32::from_bits(word_at(&e, extra, 0x10)), -2.0);
        assert_eq!(f32::from_bits(word_at(&e, extra, 0x14)), 0.5);
        // Present: overwritten without a new extra data.
        e.mem.set_u32(normal, 3.0f32.to_bits());
        let log = logged(&mut e, |e| {
            e.call(0x0041_d950, &args![list, normal]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(f32::from_bits(word_at(&e, extra, 0x0c)), 3.0);
    }

    #[test]
    fn enable_state_parent_setter_and_getter() {
        let mut e = engine();
        check_word_getter(0x0041_da10, EXTRA_ENABLE_STATE_PARENT);
        let (list, _) = list_with(&mut e, &[]);
        // Null with nothing: no new extra data.
        let log = logged(&mut e, |e| {
            e.call(0x0041_da40, &args![list, 0u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        // A parent: built (0x14 bytes), the parent stored, added.
        let log = logged(&mut e, |e| {
            e.call(0x0041_da40, &args![list, 0x3130u32]);
        });
        assert_eq!(calls_to(&log, OPERATOR_NEW), vec![vec![0x14]]);
        let extra = find_extra(&mut e, list, EXTRA_ENABLE_STATE_PARENT);
        assert_eq!(word_at(&e, extra, 0x0c), 0x3130);
        // Present: stored.
        let log = logged(&mut e, |e| {
            e.call(0x0041_da40, &args![list, 0x4140u32]);
        });
        assert!(calls_to(&log, OPERATOR_NEW).is_empty());
        assert_eq!(word_at(&e, extra, 0x0c), 0x4140);
        // Null: deleted.
        make_deletable(&mut e, VTABLE);
        let log = logged(&mut e, |e| {
            e.call(0x0041_da40, &args![list, 0u32]);
        });
        assert_eq!(deleted(&log), vec![extra.addr()]);
        assert!(sorted_types(&e, list).is_empty());
    }
}
