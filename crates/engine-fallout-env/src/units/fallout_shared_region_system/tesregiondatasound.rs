//! `fallout shared/region system/tesregiondatasound.cpp` (Xbox PDB source unit), subsystem `fallout shared/region system`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `TESRegionDataSound` is the sound part of a region record: a music type,
//! a list of region sounds (sound form, flags, chance), the incidental media
//! set and a list of battle media sets (form ids). Loading reads the
//! `RDMO`, `RDSI`, `RDSB`, `RDSD` and `RDMD` chunks; saving writes the
//! first four.
//!
//! Functions with a C++ exception frame (`FS:[0]`) are translated without the
//! unwinding and marked incomplete.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::types::BSSimpleList;

use super::tesregiondatalandscape::blended_weight;

/// `TESRegionDataSound`'s vtable (`.?AVTESRegionDataSound@@`, checked
/// through its RTTI locator at `01023da0`).
const VTABLE: u32 = 0x0102_3da4;
/// `"MASTERFILE: Unable to find music type (%08X)."`.
const MISSING_MUSIC_TYPE_FORMAT: u32 = 0x0102_3de0;
/// The RTTI type descriptors `__RTDynamicCast` gets for the music type
/// lookup (source and target type; not decoded here).
const CAST_SOURCE_TYPE: u32 = 0x0118_3028;
const CAST_TARGET_TYPE: u32 = 0x0118_40fc;
/// The data handler pointer (`TESDataHandler*`) the sound lookup uses.
const DATA_HANDLER: u32 = 0x011c_3f2c;

/// Chunk ids as the words the game compares against.
const CHUNK_RDMO: u32 = 0x4f4d_4452;
const CHUNK_RDSI: u32 = 0x4953_4452;
const CHUNK_RDSB: u32 = 0x4253_4452;
const CHUNK_RDSD: u32 = 0x4453_4452;
const CHUNK_RDMD: u32 = 0x444d_4452;

/// `operator new` / `operator delete` (cdecl).
const ALLOCATE: u32 = 0x0040_1000;
const DEALLOCATE: u32 = 0x0040_1030;
/// The compiler's `vector constructor iterator` (`(array, size, count,
/// constructor)`, callee cleans the stack).
const VECTOR_CONSTRUCT: u32 = 0x0040_1050;
/// `LookupFormByID`-style lookup in `tesform.cpp` (cdecl, form id).
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// The form id of a form (thiscall; the map has no name).
const FORM_ID: u32 = 0x0084_e3a0;
/// `TESForm` chunk writers: a one-word chunk (cdecl `(id, word)`) and
/// `TESForm::__AddChunkData` (Xbox PDB, cdecl `(id, data, size)`).
const ADD_WORD_CHUNK: u32 = 0x0048_5910;
const ADD_CHUNK_DATA: u32 = 0x0048_5990;
/// `TESForm::AddCompileIndex` (Xbox PDB), cdecl `(form id pointer, file)`.
const ADD_COMPILE_INDEX: u32 = 0x0048_5d50;
/// The byte at `011c54ba` (cdecl, no argument): nonzero when saved data is
/// byte-swapped.
const SAVE_SWAPS_BYTES: u32 = 0x0040_1500;
/// Nonzero when the file's data is byte-swapped (thiscall on the file).
const FILE_SWAPS_BYTES: u32 = 0x0040_1680;
/// Byte-swaps a `TESRegionSound` (thiscall).
const SWAP_REGION_SOUND: u32 = 0x005a_6b10;
/// Byte-swap called on the music type word when loading `RDMO` (thiscall
/// on `this + 8`).
const SWAP_MUSIC_TYPE: u32 = 0x0050_3210;
/// `TESFile::GetTESChunk` (Xbox PDB).
const GET_TES_CHUNK: u32 = 0x0047_26b0;
/// `TESFile::GetChunkData` (Xbox PDB): reads one word (thiscall, out pointer).
const GET_CHUNK_WORD: u32 = 0x0047_27f0;
/// `TESFile::GetChunkData` overload (Xbox PDB `_ov3`): reads `size` bytes
/// (thiscall `(buffer, size)`).
const GET_CHUNK_DATA: u32 = 0x0047_2890;
/// The current chunk's size (thiscall on the file).
const CHUNK_SIZE: u32 = 0x0040_1660;
/// `TESDataHandler::GetSound` (Xbox PDB), thiscall with the form id.
const GET_SOUND: u32 = 0x0046_15d0;
/// `__RTDynamicCast` (cdecl, five words).
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// The masterfile error print (cdecl, format and arguments).
const PRINT_ERROR: u32 = 0x005b_5e40;
/// `BSSimpleList` helpers (thiscall on the list head): the node's item
/// pointer, the next node, the count, appending by pointer to the item,
/// removing the head, removing an item, clearing, the constructor, the
/// destructor, and the clearing helper the copy constructor uses.
const LIST_ITEM: u32 = 0x0068_15c0;
const LIST_NEXT: u32 = 0x0072_6070;
const LIST_COUNT: u32 = 0x005a_e380;
const LIST_APPEND: u32 = 0x0090_5820;
const LIST_REMOVE_HEAD: u32 = 0x0063_f7b0;
const LIST_REMOVE: u32 = 0x0090_5330;
const LIST_ADD: u32 = 0x005a_e3d0;
const LIST_CLEAR: u32 = 0x0082_56d0;
const LIST_CONSTRUCT: u32 = 0x0096_a2d0;
const LIST_DESTRUCT: u32 = 0x0046_ffb0;
const LIST_REMOVE_ALL: u32 = 0x0047_0470;
/// Random integer (cdecl `(0, 3)` here; nonzero results skip an entry).
const RANDOM_INT: u32 = 0x0094_4460;

/// `TESRegionData` copy constructor, destructor body and `Save`.
const REGION_DATA_COPY_CONSTRUCT: u32 = 0x004f_1460;
const REGION_DATA_DESTRUCT: u32 = 0x004f_14a0;
const REGION_DATA_SAVE: u32 = 0x004f_14c0;
/// The blend's region data getters and setters (see
/// `tesregiondatalandscape.rs`).
const REGION_DATA_GET_FLAG: u32 = 0x004f_1540;
const REGION_DATA_GET_WEIGHT: u32 = 0x005b_b4d0;
const REGION_DATA_SET_FLAG: u32 = 0x004f_15a0;
const REGION_DATA_SET_WEIGHT: u32 = 0x004f_15c0;

/// `GetID` (Xbox PDB slot of `TESRegionData`), `GetMusicType` and
/// `SetMusicType` (Xbox PDB slots of `TESRegionDataSound`; the PC vtable
/// holds `0044ddc0` and `004f4c40` there).
const SLOT_GET_ID: u32 = 0x10;
const SLOT_GET_MUSIC_TYPE: u32 = 0x28;
const SLOT_SET_MUSIC_TYPE: u32 = 0x2c;
/// The id `GetID` returns for sound data.
const SOUND_ID: u32 = 7;

layout! {
    /// `TESRegionSound` (Xbox PDB), 0x0C bytes.
    pub struct TESRegionSound: 0x0C {
        /// `uiSound` (Xbox PDB): the sound's form id, then its form.
        0x00 uiSound: u32,
        /// `uiFlags` (Xbox PDB).
        0x04 uiFlags: u32,
        /// `uiChance` (Xbox PDB).
        0x08 uiChance: u32,
    }

    /// `TESRegionSounds` (Xbox PDB), 0x18 bytes on both builds.
    pub struct TESRegionSounds: 0x18 {
        /// `pType` (Xbox PDB): `BGSMusicType*`.
        0x00 pType: Ptr,
        /// `soundList` (Xbox PDB): `BSSimpleList<TESRegionSound*>`.
        0x04 soundList: Inline<BSSimpleList>,
        /// `pMediaSetIncidentals` (Xbox PDB): a form id.
        0x0C pMediaSetIncidentals: u32,
        /// `MediaSetBattleList` (Xbox PDB): `BSSimpleList<unsigned int>`.
        0x10 MediaSetBattleList: Inline<BSSimpleList>,
    }

    /// `TESRegionDataSound` (Xbox PDB), 0x20 bytes on both builds.
    pub struct TESRegionDataSound: 0x20 {
        /// `TESRegionData::bOverride` (Xbox PDB, inherited).
        0x04 bOverride: u8,
        /// `TESRegionData::bIgnore` (Xbox PDB, inherited).
        0x05 bIgnore: u8,
        /// `sounds` (Xbox PDB).
        0x08 sounds: Inline<TESRegionSounds>,
        /// `sounds.pType` (Xbox PDB).
        0x08 pType: Ptr,
        /// `sounds.pMediaSetIncidentals` (Xbox PDB).
        0x14 pMediaSetIncidentals: u32,
    }
}

/// `this + 0x0c`: `sounds.soundList`.
fn sound_list(this: Ptr<TESRegionDataSound>) -> u32 {
    this.addr().wrapping_add(0x0c)
}

/// `this + 0x18`: `sounds.MediaSetBattleList`.
fn battle_list(this: Ptr<TESRegionDataSound>) -> u32 {
    this.addr().wrapping_add(0x18)
}

/// The word a list node's item pointer points at.
fn list_item(e: &mut Engine, node: u32) -> u32 {
    let item = e.call(LIST_ITEM, &args![node]).u32();
    e.mem.u32(item)
}

/// `count * 12` as the compiler's `new[]` computes it: `0xffffffff` when the
/// product overflows.
fn array_bytes(count: u32) -> u32 {
    count.saturating_mul(0x0c)
}

// Translated from 004f4330 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataSound::GetMediaSetIncidentals` (Xbox PDB): the form of the
/// incidental media set's form id.
pub fn tes_region_data_sound_get_media_set_incidentals(
    e: &mut Engine,
    this: Ptr<TESRegionDataSound>,
) -> Ptr {
    let id = e.get(this, TESRegionDataSound::pMediaSetIncidentals);
    e.call(LOOKUP_FORM, &args![id]).ptr()
}

// Translated from 004f4350 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataSound::SetMediaSetIncidentals` (Xbox PDB): stores the form
/// id of `media_set` (0 for none).
pub fn tes_region_data_sound_set_media_set_incidentals(
    e: &mut Engine,
    this: Ptr<TESRegionDataSound>,
    media_set: Ptr,
) {
    let id = if media_set.is_null() {
        0
    } else {
        e.call(FORM_ID, &args![media_set]).u32()
    };
    e.set(this, TESRegionDataSound::pMediaSetIncidentals, id);
}

// Translated from 004f4390 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataSound::`scalar deleting destructor'` (Xbox PDB): runs the
/// destructor and frees the object when bit 0 of `flags` is set.
pub fn tes_region_data_sound_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<TESRegionDataSound>,
    flags: u32,
) -> Ptr<TESRegionDataSound> {
    fn_004f4520(e, this);
    if flags & 1 != 0 {
        e.call(DEALLOCATE, &args![this]);
    }
    this
}

// Translated from 004f43c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESRegionSounds` constructor: constructs the sound list and the
/// battle media set list (the music type and incidental set are left as
/// they are); returns `this`.
// Translation incomplete: 004f43c0 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_004f43c0(e: &mut Engine, this: Ptr<TESRegionSounds>) -> Ptr<TESRegionSounds> {
    e.call(LIST_CONSTRUCT, &args![this.at(TESRegionSounds::soundList)]);
    e.call(
        LIST_CONSTRUCT,
        &args![this.at(TESRegionSounds::MediaSetBattleList)],
    );
    this
}

// Translated from 004f4420 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESRegionSounds` destructor: destroys the battle media set list,
/// then the sound list.
// Translation incomplete: 004f4420 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_004f4420(e: &mut Engine, this: Ptr<TESRegionSounds>) {
    e.call(
        LIST_DESTRUCT,
        &args![this.at(TESRegionSounds::MediaSetBattleList)],
    );
    e.call(LIST_DESTRUCT, &args![this.at(TESRegionSounds::soundList)]);
}

// Translated from 004f4480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The copy constructor: copies the base, sets the vtable, constructs the
/// sounds, takes `source`'s music type (through its `GetMusicType` slot and
/// `SetMusicType`) and incidental media set, and clears the battle media
/// set list (`00470470`). The sound list is not copied.
// Translation incomplete: 004f4480 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_004f4480(
    e: &mut Engine,
    this: Ptr<TESRegionDataSound>,
    source: Ptr<TESRegionDataSound>,
) -> Ptr<TESRegionDataSound> {
    e.call(REGION_DATA_COPY_CONSTRUCT, &args![this, source]);
    e.mem.set_u32(this.addr(), VTABLE);
    fn_004f43c0(e, this.at(TESRegionDataSound::sounds));
    let music_type = e.vcall(source.addr(), SLOT_GET_MUSIC_TYPE, &[]).ptr();
    tes_region_data_sound_set_music_type(e, this, music_type);
    let incidentals = e.get(source, TESRegionDataSound::pMediaSetIncidentals);
    e.set(this, TESRegionDataSound::pMediaSetIncidentals, incidentals);
    e.call(LIST_REMOVE_ALL, &args![battle_list(this)]);
    this
}

// Translated from 004f4520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor: resets the vtable, deletes the region sounds
/// (`004f4bd0`), destroys the sounds and the base.
// Translation incomplete: 004f4520 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn fn_004f4520(e: &mut Engine, this: Ptr<TESRegionDataSound>) {
    e.mem.set_u32(this.addr(), VTABLE);
    fn_004f4bd0(e, this);
    fn_004f4420(e, this.at(TESRegionDataSound::sounds));
    e.call(REGION_DATA_DESTRUCT, &args![this]);
}

// Translated from 004f4590 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataSound::Save` (Xbox PDB): saves the base, then `RDMO` (the
/// music type's form id, when there is a music type), `RDSI` (the
/// incidental media set, when set), one `RDSB` per battle media set (at
/// most the list's count, stopping at an empty item) and `RDSD`: the region
/// sounds copied into a temporary array of 12-byte entries (byte-swapped
/// before and after the write when saves are swapped).
// Translation incomplete: 004f4590 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn tes_region_data_sound_save(e: &mut Engine, this: Ptr<TESRegionDataSound>) {
    e.call(REGION_DATA_SAVE, &args![this]);
    if e.vcall(this.addr(), SLOT_GET_MUSIC_TYPE, &[]).u32() != 0 {
        let music_type = e.vcall(this.addr(), SLOT_GET_MUSIC_TYPE, &[]).u32();
        let id = e.call(FORM_ID, &args![music_type]).u32();
        e.call(ADD_WORD_CHUNK, &args![CHUNK_RDMO, id]);
    }
    let incidentals = e.get(this, TESRegionDataSound::pMediaSetIncidentals);
    if incidentals != 0 {
        e.call(ADD_WORD_CHUNK, &args![CHUNK_RDSI, incidentals]);
    }

    let count = e.call(LIST_COUNT, &args![battle_list(this)]).u32();
    if count != 0 {
        let mut node = battle_list(this);
        let mut written = 0u32;
        while node != 0 && list_item(e, node) != 0 && written < count {
            let id = list_item(e, node);
            e.call(ADD_WORD_CHUNK, &args![CHUNK_RDSB, id]);
            node = e.call(LIST_NEXT, &args![node]).u32();
            written += 1;
        }
    }

    let count = e.call(LIST_COUNT, &args![sound_list(this)]).u32();
    if count == 0 {
        return;
    }
    let block = e.call(ALLOCATE, &args![array_bytes(count)]).u32();
    let array = if block == 0 {
        0
    } else {
        e.call(
            VECTOR_CONSTRUCT,
            &args![block, 0x0cu32, count, 0x004f_4830u32],
        );
        block
    };
    let mut node = sound_list(this);
    let mut copied = 0u32;
    while node != 0 && list_item(e, node) != 0 && copied < count {
        let sound = list_item(e, node);
        let entry = array.wrapping_add(copied.wrapping_mul(0x0c));
        for word in 0..3 {
            let value = e.mem.u32(sound.wrapping_add(4 * word));
            e.mem.set_u32(entry.wrapping_add(4 * word), value);
        }
        node = e.call(LIST_NEXT, &args![node]).u32();
        copied += 1;
    }
    swap_entries(e, array, count);
    e.call(
        ADD_CHUNK_DATA,
        &args![CHUNK_RDSD, array, count.wrapping_mul(0x0c)],
    );
    swap_entries(e, array, count);
    e.call(DEALLOCATE, &args![array]);
}

/// Byte-swaps the `count` entries of `array` when saves are swapped.
fn swap_entries(e: &mut Engine, array: u32, count: u32) {
    if e.call(SAVE_SWAPS_BYTES, &[]).u8() != 0 {
        for i in 0..count {
            e.call(
                SWAP_REGION_SOUND,
                &args![array.wrapping_add(i.wrapping_mul(0x0c))],
            );
        }
    }
}

// Translated from 004f4830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `TESRegionSound` constructor: no sound, flags `0xf`, chance 0;
/// returns `this`.
pub fn fn_004f4830(e: &mut Engine, this: Ptr<TESRegionSound>) -> Ptr<TESRegionSound> {
    e.set(this, TESRegionSound::uiSound, 0);
    e.set(this, TESRegionSound::uiFlags, 0x0f);
    e.set(this, TESRegionSound::uiChance, 0);
    this
}

// Translated from 004f4860 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataSound::LoadRegionDataSound` (Xbox PDB): loads one chunk.
/// `RDMO`: the music type's form id; when nonzero it is fixed up for the
/// file (`AddCompileIndex`), looked up, cast with `__RTDynamicCast` (an
/// error is printed when that fails) and passed to `SetMusicType`. `RDSI`:
/// the incidental media set id, kept when nonzero. `RDSB`: one battle media
/// set id, appended. `RDSD`: an array of 12-byte region sounds (false when
/// the chunk is 4 bytes); each becomes a new `TESRegionSound` whose id is
/// fixed up and resolved through `TESDataHandler::GetSound`: appended when
/// the sound exists, deleted otherwise. `RDMD`: read with size 0. True for
/// those chunks, false for any other.
// Translation incomplete: 004f4860 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn tes_region_data_sound_load_region_data_sound(
    e: &mut Engine,
    this: Ptr<TESRegionDataSound>,
    file: Ptr,
) -> bool {
    let chunk = e.call(GET_TES_CHUNK, &args![file]).u32();
    match chunk {
        CHUNK_RDMD => {
            e.with_stack(4, |e, word| {
                e.call(GET_CHUNK_DATA, &args![file, word, 0u32]);
            });
        }
        CHUNK_RDMO => {
            e.with_stack(8, |e, id| {
                e.call(GET_CHUNK_WORD, &args![file, id]);
                let form_id = e.mem.u32(id.addr());
                if form_id == 0 {
                    return;
                }
                if e.call(FILE_SWAPS_BYTES, &args![file]).u8() != 0 {
                    e.call(SWAP_MUSIC_TYPE, &args![this.addr().wrapping_add(8)]);
                }
                e.call(ADD_COMPILE_INDEX, &args![id, file]);
                let form_id = e.mem.u32(id.addr());
                let form = e.call(LOOKUP_FORM, &args![form_id]).u32();
                let music_type = e
                    .call(
                        DYNAMIC_CAST,
                        &args![form, 0u32, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0u32],
                    )
                    .u32();
                if music_type == 0 {
                    let form_id = e.mem.u32(id.addr());
                    e.call(PRINT_ERROR, &args![MISSING_MUSIC_TYPE_FORMAT, form_id]);
                }
                e.vcall(this.addr(), SLOT_SET_MUSIC_TYPE, &args![music_type]);
            });
        }
        CHUNK_RDSI => {
            e.with_stack(4, |e, id| {
                e.call(GET_CHUNK_WORD, &args![file, id]);
                let value = e.mem.u32(id.addr());
                if value != 0 {
                    e.set(this, TESRegionDataSound::pMediaSetIncidentals, value);
                }
            });
        }
        CHUNK_RDSB => {
            e.with_stack(4, |e, id| {
                e.call(GET_CHUNK_WORD, &args![file, id]);
                e.call(LIST_APPEND, &args![battle_list(this), id]);
            });
        }
        CHUNK_RDSD => {
            let size = e.call(CHUNK_SIZE, &args![file]).u32();
            let count = size / 0x0c;
            if size == 4 {
                return false;
            }
            let block = e.call(ALLOCATE, &args![array_bytes(count)]).u32();
            let array = if block == 0 {
                0
            } else {
                e.call(
                    VECTOR_CONSTRUCT,
                    &args![block, 0x0cu32, count, 0x004f_4830u32],
                );
                block
            };
            e.call(
                GET_CHUNK_DATA,
                &args![file, array, count.wrapping_mul(0x0c)],
            );
            e.with_stack(4, |e, slot| {
                for i in 0..count {
                    let sound = load_region_sound(e, file, array.wrapping_add(i * 0x0c));
                    e.mem.set_u32(slot.addr(), sound);
                    let id = if sound == 0 { 0 } else { e.mem.u32(sound) };
                    let handler = e.global::<u32>(DATA_HANDLER);
                    if sound != 0 && id != 0 && e.call(GET_SOUND, &args![handler, id]).u32() != 0 {
                        let handler = e.global::<u32>(DATA_HANDLER);
                        let resolved = e.call(GET_SOUND, &args![handler, id]).u32();
                        e.mem.set_u32(sound, resolved);
                        e.call(LIST_APPEND, &args![sound_list(this), slot]);
                    } else {
                        e.call(DEALLOCATE, &args![sound]);
                    }
                }
            });
            e.call(DEALLOCATE, &args![array]);
        }
        _ => return false,
    }
    true
}

/// One `RDSD` entry made into a new `TESRegionSound`: allocated and
/// constructed (null when the allocation fails), the 12 bytes copied from
/// `entry`, byte-swapped for a swapped file, and its id fixed up with
/// `AddCompileIndex`.
fn load_region_sound(e: &mut Engine, file: Ptr, entry: u32) -> u32 {
    let block = e.call(ALLOCATE, &args![0x0cu32]).u32();
    let sound = if block == 0 {
        0
    } else {
        fn_004f4830(e, Ptr::new(block)).addr()
    };
    for word in 0..3 {
        let value = e.mem.u32(entry.wrapping_add(4 * word));
        e.mem.set_u32(sound.wrapping_add(4 * word), value);
    }
    if e.call(FILE_SWAPS_BYTES, &args![file]).u8() != 0 {
        e.call(SWAP_REGION_SOUND, &args![sound]);
    }
    e.call(ADD_COMPILE_INDEX, &args![sound, file]);
    sound
}

// Translated from 004f4bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Deletes the region sounds: while the sound list's head item is set, it
/// is removed from the list (`0063f7b0`) and freed; then the list is
/// cleared (`008256d0`).
pub fn fn_004f4bd0(e: &mut Engine, this: Ptr<TESRegionDataSound>) {
    loop {
        let list = sound_list(this);
        if list == 0 || list_item(e, list) == 0 {
            break;
        }
        let sound = list_item(e, list);
        e.call(LIST_REMOVE_HEAD, &args![list]);
        e.call(DEALLOCATE, &args![sound]);
    }
    e.call(LIST_CLEAR, &args![sound_list(this)]);
}

// Translated from 004f4c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataSound::SetMusicType` (Xbox PDB): stores `music_type` and
/// returns true, unless it is null or already the music type (false).
pub fn tes_region_data_sound_set_music_type(
    e: &mut Engine,
    this: Ptr<TESRegionDataSound>,
    music_type: Ptr,
) -> bool {
    if music_type.is_null() || music_type == e.get(this, TESRegionDataSound::pType) {
        return false;
    }
    e.set(this, TESRegionDataSound::pType, music_type);
    true
}

// Translated from 004f4c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataSound::Copy` (Xbox PDB): a new copy (`operator new(0x20)`
/// and the copy constructor), or null.
// Translation incomplete: 004f4c70 - C++ exception unwinding (FS:[0] frame) is not translated.
pub fn tes_region_data_sound_copy(
    e: &mut Engine,
    this: Ptr<TESRegionDataSound>,
) -> Ptr<TESRegionDataSound> {
    let block: Ptr<TESRegionDataSound> = e.call(ALLOCATE, &args![0x20u32]).ptr();
    if block.is_null() {
        Ptr::NULL
    } else {
        fn_004f4480(e, block, this)
    }
}

/// The source wins: its flag byte and weight byte are copied, then its
/// music type.
fn blend_take_source(e: &mut Engine, this: Ptr<TESRegionDataSound>, source: Ptr) {
    let flag = e.call(REGION_DATA_GET_FLAG, &args![source]).u8();
    e.call(REGION_DATA_SET_FLAG, &args![this, flag as u32]);
    let weight = e.call(REGION_DATA_GET_WEIGHT, &args![source]).u8();
    e.call(REGION_DATA_SET_WEIGHT, &args![this, weight as u32]);
    blend_take_music_type(e, this, source);
}

fn blend_take_music_type(e: &mut Engine, this: Ptr<TESRegionDataSound>, source: Ptr) {
    let music_type = e.vcall(source.addr(), SLOT_GET_MUSIC_TYPE, &[]).u32();
    e.vcall(this.addr(), SLOT_SET_MUSIC_TYPE, &args![music_type]);
}

fn weight(e: &mut Engine, data: Ptr) -> u8 {
    e.call(REGION_DATA_GET_WEIGHT, &args![data]).u8()
}

// Translated from 004f4cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataSound::BlendInto` (Xbox PDB): merges sound data `source`
/// (its `GetID` must be 7, and `enabled` nonzero) into this one, as the
/// landscape blend does with the music type in place of the texture: when
/// this data is ignored, `source`'s flag byte, weight byte and music type
/// are taken; when `source` is ignored nothing happens; with this data's
/// override set, `source` wins only with its flag byte set and a higher
/// weight; otherwise `source` wins with its flag byte set, else its music
/// type is taken when its weight is higher and the weight becomes the
/// blended weight.
pub fn tes_region_data_sound_blend_into(
    e: &mut Engine,
    this: Ptr<TESRegionDataSound>,
    source: Ptr,
    enabled: u32,
) {
    if source.is_null()
        || e.vcall(source.addr(), SLOT_GET_ID, &[]).u32() != SOUND_ID
        || enabled == 0
    {
        return;
    }
    if e.get(this, TESRegionDataSound::bIgnore) != 0 {
        blend_take_source(e, this, source);
    } else if e.mem.u8(source.addr() + 5) != 0 {
        // `TESRegionData::bIgnore` (Xbox PDB) +0x05 of the source.
    } else if e.get(this, TESRegionDataSound::bOverride) != 0 {
        if e.call(REGION_DATA_GET_FLAG, &args![source]).u8() != 0
            && weight(e, source) > weight(e, this.cast())
        {
            blend_take_source(e, this, source);
        }
    } else if e.call(REGION_DATA_GET_FLAG, &args![source]).u8() != 0 {
        blend_take_source(e, this, source);
    } else {
        if weight(e, source) > weight(e, this.cast()) {
            blend_take_music_type(e, this, source);
        }
        let blended = blended_weight(e, this.cast(), source);
        e.call(REGION_DATA_SET_WEIGHT, &args![this, blended as u32]);
    }
}

// Translated from 004f4f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataSound::Validate` (Xbox PDB): true when the music type
/// pointer is null (as the PC code compares it).
pub fn tes_region_data_sound_validate(e: &mut Engine, this: Ptr<TESRegionDataSound>) -> bool {
    e.get(this, TESRegionDataSound::pType).is_null()
}

// Translated from 004f4f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESRegionDataSound::GetMediaSetBattle` (Xbox PDB): picks a battle media
/// set: walking the list, an entry replaces the choice when nothing is
/// chosen yet or the random draw (`00944460(0, 3)`) is 0. The chosen id is
/// then passed to `00905330` and `005ae3d0` on the list (by pointer) and its
/// form returned; null for an empty list.
pub fn tes_region_data_sound_get_media_set_battle(
    e: &mut Engine,
    this: Ptr<TESRegionDataSound>,
) -> Ptr {
    let list = battle_list(this);
    if list == 0 || list_item(e, list) == 0 {
        return Ptr::NULL;
    }
    e.with_stack(4, |e, chosen| {
        let mut node = list;
        while node != 0 && list_item(e, node) != 0 {
            let draw = e.call(RANDOM_INT, &args![0u32, 3u32]).u32();
            if draw == 0 || e.mem.u32(chosen.addr()) == 0 {
                let id = list_item(e, node);
                e.mem.set_u32(chosen.addr(), id);
            }
            node = e.call(LIST_NEXT, &args![node]).u32();
        }
        if e.mem.u32(chosen.addr()) == 0 {
            return Ptr::NULL;
        }
        e.call(LIST_REMOVE, &args![battle_list(this), chosen]);
        e.call(LIST_ADD, &args![battle_list(this), chosen]);
        let id = e.mem.u32(chosen.addr());
        e.call(LOOKUP_FORM, &args![id]).ptr()
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x004f4330,
            tes_region_data_sound_get_media_set_incidentals(Ptr<TESRegionDataSound>) -> Ptr
        ),
        entry!(
            0x004f4350,
            tes_region_data_sound_set_media_set_incidentals(Ptr<TESRegionDataSound>, Ptr)
        ),
        entry!(
            0x004f4390,
            tes_region_data_sound_scalar_deleting_destructor(
                Ptr<TESRegionDataSound>,
                u32,
            )
                -> Ptr<TESRegionDataSound>
        ),
        entry!(
            0x004f43c0,
            fn_004f43c0(Ptr<TESRegionSounds>) -> Ptr<TESRegionSounds>
        ),
        entry!(0x004f4420, fn_004f4420(Ptr<TESRegionSounds>)),
        entry!(
            0x004f4480,
            fn_004f4480(
                Ptr<TESRegionDataSound>,
                Ptr<TESRegionDataSound>,
            ) -> Ptr<TESRegionDataSound>
        ),
        entry!(0x004f4520, fn_004f4520(Ptr<TESRegionDataSound>)),
        entry!(
            0x004f4590,
            tes_region_data_sound_save(Ptr<TESRegionDataSound>)
        ),
        entry!(
            0x004f4830,
            fn_004f4830(Ptr<TESRegionSound>) -> Ptr<TESRegionSound>
        ),
        entry!(
            0x004f4860,
            tes_region_data_sound_load_region_data_sound(Ptr<TESRegionDataSound>, Ptr) -> bool
        ),
        entry!(0x004f4bd0, fn_004f4bd0(Ptr<TESRegionDataSound>)),
        entry!(
            0x004f4c40,
            tes_region_data_sound_set_music_type(Ptr<TESRegionDataSound>, Ptr) -> bool
        ),
        entry!(
            0x004f4c70,
            tes_region_data_sound_copy(Ptr<TESRegionDataSound>) -> Ptr<TESRegionDataSound>
        ),
        entry!(
            0x004f4cf0,
            tes_region_data_sound_blend_into(Ptr<TESRegionDataSound>, Ptr, u32)
        ),
        entry!(
            0x004f4f70,
            tes_region_data_sound_validate(Ptr<TESRegionDataSound>) -> bool
        ),
        entry!(
            0x004f4f90,
            tes_region_data_sound_get_media_set_battle(Ptr<TESRegionDataSound>) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A double at `addr` returning `eax`.
    fn fixed(e: &mut Engine, addr: u32, eax: u32) {
        e.register_double(addr, move |_, _| Ret {
            eax,
            ..Ret::default()
        });
    }

    /// Doubles at `addrs` returning 0.
    fn stub(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            fixed(e, addr, 0);
        }
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

    /// List doubles over real `BSSimpleList` nodes (item, next): the item
    /// pointer is the node itself, the next node its second word.
    fn list_engine() -> Engine {
        let mut e = Engine::new();
        e.register(LIST_ITEM, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.register(LIST_NEXT, |e, a| Ret {
            eax: e.mem.u32(a[0] + 4),
            ..Ret::default()
        });
        e.register(LIST_COUNT, |e, a| {
            let mut node = a[0];
            let mut n = 0;
            while node != 0 && e.mem.u32(node) != 0 {
                n += 1;
                node = e.mem.u32(node + 4);
            }
            Ret {
                eax: n,
                ..Ret::default()
            }
        });
        e
    }

    /// Fills the list head at `head` with `items` (extra nodes allocated).
    fn fill_list(e: &mut Engine, head: u32, items: &[u32]) {
        let mut node = head;
        for (i, &item) in items.iter().enumerate() {
            e.mem.set_u32(node, item);
            if i + 1 < items.len() {
                let next = e.mem.alloc(8);
                e.mem.set_u32(node + 4, next);
                node = next;
            }
        }
    }

    fn sound_data(e: &mut Engine) -> Ptr<TESRegionDataSound> {
        e.new_object()
    }

    #[test]
    fn get_media_set_incidentals_looks_up_the_id() {
        let mut e = Engine::new();
        e.register(LOOKUP_FORM, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        let this = sound_data(&mut e);
        e.set(this, TESRegionDataSound::pMediaSetIncidentals, 0x2000);
        assert_eq!(e.call(0x004f_4330, &args![this]).u32(), 0x2100);
    }

    #[test]
    fn set_media_set_incidentals_stores_the_form_id() {
        let mut e = Engine::new();
        fixed(&mut e, FORM_ID, 0x1234);
        let this = sound_data(&mut e);
        e.call(0x004f_4350, &args![this, 0x9000u32]);
        assert_eq!(
            e.get(this, TESRegionDataSound::pMediaSetIncidentals),
            0x1234
        );
        e.call(0x004f_4350, &args![this, 0u32]);
        assert_eq!(e.get(this, TESRegionDataSound::pMediaSetIncidentals), 0);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_bit_0() {
        let mut e = list_engine();
        stub(
            &mut e,
            &[LIST_CLEAR, LIST_DESTRUCT, REGION_DATA_DESTRUCT, DEALLOCATE],
        );
        let this = sound_data(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_4390, &args![this, 0u32]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert!(calls_to(&e, DEALLOCATE).is_empty());
        e.call(0x004f_4390, &args![this, 1u32]);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![this.addr()]]);
    }

    #[test]
    fn sounds_constructor_and_destructor_handle_both_lists() {
        let mut e = Engine::new();
        stub(&mut e, &[LIST_CONSTRUCT, LIST_DESTRUCT]);
        let sounds: Ptr<TESRegionSounds> = e.new_object();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_43c0, &args![sounds]).u32(), sounds.addr());
        assert_eq!(
            calls_to(&e, LIST_CONSTRUCT),
            vec![vec![sounds.addr() + 4], vec![sounds.addr() + 0x10]]
        );
        e.call(0x004f_4420, &args![sounds]);
        assert_eq!(
            calls_to(&e, LIST_DESTRUCT),
            vec![vec![sounds.addr() + 0x10], vec![sounds.addr() + 4]]
        );
    }

    #[test]
    fn copy_constructor_takes_music_type_and_incidentals() {
        let mut e = Engine::new();
        stub(
            &mut e,
            &[REGION_DATA_COPY_CONSTRUCT, LIST_CONSTRUCT, LIST_REMOVE_ALL],
        );
        fixed(&mut e, 0x0000_9128, 0x7100);
        e.put_vtable(0x0000_a000, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x9128]);
        let source = sound_data(&mut e);
        e.mem.set_u32(source.addr(), 0xa000);
        e.set(source, TESRegionDataSound::pMediaSetIncidentals, 0x55);
        let this = sound_data(&mut e);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_4480, &args![this, source]).u32(), this.addr());
        assert_eq!(e.mem.u32(this.addr()), VTABLE);
        assert_eq!(e.get(this, TESRegionDataSound::pType).addr(), 0x7100);
        assert_eq!(e.get(this, TESRegionDataSound::pMediaSetIncidentals), 0x55);
        assert_eq!(
            calls_to(&e, REGION_DATA_COPY_CONSTRUCT),
            vec![vec![this.addr(), source.addr()]]
        );
        assert_eq!(
            calls_to(&e, LIST_REMOVE_ALL),
            vec![vec![this.addr() + 0x18]]
        );
    }

    #[test]
    fn destructor_deletes_sounds_then_destroys_lists_and_base() {
        let mut e = list_engine();
        stub(
            &mut e,
            &[LIST_CLEAR, LIST_DESTRUCT, REGION_DATA_DESTRUCT, DEALLOCATE],
        );
        // Removing the head: the next node's item moves into the head.
        e.register(LIST_REMOVE_HEAD, |e, a| {
            let next = e.mem.u32(a[0] + 4);
            let (item, after) = if next == 0 {
                (0, 0)
            } else {
                (e.mem.u32(next), e.mem.u32(next + 4))
            };
            e.mem.set_u32(a[0], item);
            e.mem.set_u32(a[0] + 4, after);
            Ret::default()
        });
        let this = sound_data(&mut e);
        fill_list(&mut e, this.addr() + 0x0c, &[0x3100, 0x3200]);
        e.call_log = Some(vec![]);
        e.call(0x004f_4520, &args![this]);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![0x3100], vec![0x3200]]);
        assert_eq!(calls_to(&e, LIST_CLEAR), vec![vec![this.addr() + 0x0c]]);
        assert_eq!(calls_to(&e, LIST_DESTRUCT).len(), 2);
        assert_eq!(calls_to(&e, REGION_DATA_DESTRUCT), vec![vec![this.addr()]]);
    }

    #[test]
    fn save_writes_music_incidentals_battle_sets_and_sounds() {
        let mut e = list_engine();
        stub(&mut e, &[REGION_DATA_SAVE, ADD_WORD_CHUNK, DEALLOCATE]);
        fixed(&mut e, 0x0000_9128, 0x7100);
        e.put_vtable(0x0000_a000, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x9128]);
        e.register(FORM_ID, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        let array = e.mem.alloc(0x18);
        fixed(&mut e, ALLOCATE, array);
        stub(&mut e, &[VECTOR_CONSTRUCT, SWAP_REGION_SOUND]);
        fixed(&mut e, SAVE_SWAPS_BYTES, 1);
        let written = Rc::new(RefCell::new(vec![]));
        let seen = written.clone();
        e.register_double(ADD_CHUNK_DATA, move |e, a| {
            let words: Vec<u32> = (0..a[2] / 4).map(|i| e.mem.u32(a[1] + 4 * i)).collect();
            seen.borrow_mut().push((a[0], words));
            Ret::default()
        });

        let this = sound_data(&mut e);
        e.mem.set_u32(this.addr(), 0xa000);
        e.set(this, TESRegionDataSound::pMediaSetIncidentals, 0x66);
        fill_list(&mut e, this.addr() + 0x18, &[0x41, 0x42]);
        let first = e.mem.alloc(0x0c);
        let second = e.mem.alloc(0x0c);
        for (sound, base) in [(first, 10), (second, 20)] {
            for i in 0..3 {
                e.mem.set_u32(sound + 4 * i, base + i);
            }
        }
        fill_list(&mut e, this.addr() + 0x0c, &[first, second]);

        e.call_log = Some(vec![]);
        e.call(0x004f_4590, &args![this]);
        assert_eq!(
            calls_to(&e, ADD_WORD_CHUNK),
            vec![
                vec![CHUNK_RDMO, 0x7101],
                vec![CHUNK_RDSI, 0x66],
                vec![CHUNK_RDSB, 0x41],
                vec![CHUNK_RDSB, 0x42]
            ]
        );
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x18]]);
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![array, 0x0c, 2, 0x004f_4830]]
        );
        assert_eq!(
            *written.borrow(),
            vec![(CHUNK_RDSD, vec![10, 11, 12, 20, 21, 22])]
        );
        // Swapped before and after the write.
        assert_eq!(
            calls_to(&e, SWAP_REGION_SOUND),
            vec![
                vec![array],
                vec![array + 0x0c],
                vec![array],
                vec![array + 0x0c]
            ]
        );
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![array]]);

        // Nothing optional: only the base.
        let empty = sound_data(&mut e);
        e.mem.set_u32(empty.addr(), 0xa000);
        fixed(&mut e, 0x0000_9128, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_4590, &args![empty]);
        assert!(calls_to(&e, ADD_WORD_CHUNK).is_empty());
        assert!(calls_to(&e, ALLOCATE).is_empty());
    }

    #[test]
    fn region_sound_constructor_sets_flags_0xf() {
        let mut e = Engine::new();
        let sound: Ptr<TESRegionSound> = e.new_object();
        e.set(sound, TESRegionSound::uiChance, 9);
        assert_eq!(e.call(0x004f_4830, &args![sound]).u32(), sound.addr());
        assert_eq!(e.get(sound, TESRegionSound::uiSound), 0);
        assert_eq!(e.get(sound, TESRegionSound::uiFlags), 0x0f);
        assert_eq!(e.get(sound, TESRegionSound::uiChance), 0);
    }

    /// Doubles for loading: the chunk id, a word reader returning `word`,
    /// no byte swapping.
    fn load_engine(chunk: u32, word: u32) -> Engine {
        let mut e = Engine::new();
        fixed(&mut e, GET_TES_CHUNK, chunk);
        e.register_double(GET_CHUNK_WORD, move |e, a| {
            e.mem.set_u32(a[1], word);
            Ret::default()
        });
        fixed(&mut e, FILE_SWAPS_BYTES, 0);
        stub(&mut e, &[ADD_COMPILE_INDEX, LIST_APPEND, DEALLOCATE]);
        e
    }

    #[test]
    fn load_reads_the_word_chunks() {
        // Unknown chunk.
        let mut e = load_engine(0x1111_1111, 0);
        let this = sound_data(&mut e);
        assert!(!e.call(0x004f_4860, &args![this, 0x8000u32]).bool());

        // RDSI keeps a nonzero id.
        let mut e = load_engine(CHUNK_RDSI, 0x77);
        let this = sound_data(&mut e);
        assert!(e.call(0x004f_4860, &args![this, 0x8000u32]).bool());
        assert_eq!(e.get(this, TESRegionDataSound::pMediaSetIncidentals), 0x77);

        // RDSB appends the id by pointer.
        let mut e = load_engine(CHUNK_RDSB, 0x78);
        let appended = Rc::new(RefCell::new(vec![]));
        let seen = appended.clone();
        e.register_double(LIST_APPEND, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let this = sound_data(&mut e);
        assert!(e.call(0x004f_4860, &args![this, 0x8000u32]).bool());
        assert_eq!(*appended.borrow(), vec![(this.addr() + 0x18, 0x78)]);

        // RDMD reads nothing.
        let mut e = load_engine(CHUNK_RDMD, 0);
        stub(&mut e, &[GET_CHUNK_DATA]);
        let this = sound_data(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_4860, &args![this, 0x8000u32]).bool());
        let reads = calls_to(&e, GET_CHUNK_DATA);
        assert_eq!(reads.len(), 1);
        assert_eq!((reads[0][0], reads[0][2]), (0x8000, 0));
    }

    #[test]
    fn load_music_type_casts_and_reports_a_missing_type() {
        let mut e = load_engine(CHUNK_RDMO, 0x0100_0042);
        fixed(&mut e, LOOKUP_FORM, 0x6600);
        fixed(&mut e, DYNAMIC_CAST, 0);
        stub(&mut e, &[PRINT_ERROR, 0x0000_912c]);
        e.put_vtable(0x0000_a000, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x912c]);
        let this = sound_data(&mut e);
        e.mem.set_u32(this.addr(), 0xa000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_4860, &args![this, 0x8000u32]).bool());
        assert_eq!(calls_to(&e, LOOKUP_FORM), vec![vec![0x0100_0042]]);
        assert_eq!(
            calls_to(&e, DYNAMIC_CAST),
            vec![vec![0x6600, 0, CAST_SOURCE_TYPE, CAST_TARGET_TYPE, 0]]
        );
        assert_eq!(
            calls_to(&e, PRINT_ERROR),
            vec![vec![MISSING_MUSIC_TYPE_FORMAT, 0x0100_0042]]
        );
        assert_eq!(calls_to(&e, 0x912c), vec![vec![this.addr(), 0]]);
        assert_eq!(calls_to(&e, ADD_COMPILE_INDEX)[0][1], 0x8000);

        // Found: no error; a swapped file swaps `this + 8`.
        fixed(&mut e, DYNAMIC_CAST, 0x6601);
        fixed(&mut e, FILE_SWAPS_BYTES, 1);
        stub(&mut e, &[SWAP_MUSIC_TYPE]);
        e.call_log = Some(vec![]);
        e.call(0x004f_4860, &args![this, 0x8000u32]);
        assert!(calls_to(&e, PRINT_ERROR).is_empty());
        assert_eq!(calls_to(&e, SWAP_MUSIC_TYPE), vec![vec![this.addr() + 8]]);
        assert_eq!(calls_to(&e, 0x912c), vec![vec![this.addr(), 0x6601]]);

        // A zero id does nothing more.
        let mut e = load_engine(CHUNK_RDMO, 0);
        let this = sound_data(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_4860, &args![this, 0x8000u32]).bool());
        assert!(calls_to(&e, ADD_COMPILE_INDEX).is_empty());
    }

    #[test]
    fn load_region_sounds_keeps_the_resolved_ones() {
        let mut e = load_engine(CHUNK_RDSD, 0);
        fixed(&mut e, CHUNK_SIZE, 0x18);
        let array = e.mem.alloc(0x18);
        let first = e.mem.alloc(0x0c);
        let second = e.mem.alloc(0x0c);
        let blocks = Rc::new(RefCell::new(vec![second, first, array]));
        let pending = blocks.clone();
        e.register_double(ALLOCATE, move |_, _| Ret {
            eax: pending.borrow_mut().pop().unwrap(),
            ..Ret::default()
        });
        stub(&mut e, &[VECTOR_CONSTRUCT]);
        e.register(GET_CHUNK_DATA, |e, a| {
            for (i, w) in [0x51u32, 1, 50, 0x52, 2, 60].iter().enumerate() {
                e.mem.set_u32(a[1] + 4 * i as u32, *w);
            }
            Ret::default()
        });
        e.map(0x011c_3000, 0x1000);
        e.set_global(DATA_HANDLER, 0x0dd0u32);
        // Only sound 0x51 exists.
        e.register(GET_SOUND, |_, a| Ret {
            eax: if a[1] == 0x51 { 0x5100 } else { 0 },
            ..Ret::default()
        });
        let appended = Rc::new(RefCell::new(vec![]));
        let seen = appended.clone();
        e.register_double(LIST_APPEND, move |e, a| {
            seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
            Ret::default()
        });
        let this = sound_data(&mut e);
        e.call_log = Some(vec![]);
        assert!(e.call(0x004f_4860, &args![this, 0x8000u32]).bool());
        assert_eq!(
            calls_to(&e, GET_CHUNK_DATA),
            vec![vec![0x8000, array, 0x18]]
        );
        assert_eq!(
            calls_to(&e, VECTOR_CONSTRUCT),
            vec![vec![array, 0x0c, 2, 0x004f_4830]]
        );
        assert_eq!(*appended.borrow(), vec![(this.addr() + 0x0c, first)]);
        assert_eq!(
            [e.mem.u32(first), e.mem.u32(first + 4), e.mem.u32(first + 8)],
            [0x5100, 1, 50]
        );
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![second], vec![array]]);
        assert_eq!(
            calls_to(&e, ADD_COMPILE_INDEX),
            vec![vec![first, 0x8000], vec![second, 0x8000]]
        );
        assert!(blocks.borrow().is_empty());

        // A 4-byte chunk fails.
        fixed(&mut e, CHUNK_SIZE, 4);
        assert!(!e.call(0x004f_4860, &args![this, 0x8000u32]).bool());
    }

    #[test]
    fn set_music_type_ignores_null_and_the_same_type() {
        let mut e = Engine::new();
        let this = sound_data(&mut e);
        assert!(!e.call(0x004f_4c40, &args![this, 0u32]).bool());
        assert!(e.call(0x004f_4c40, &args![this, 0x7100u32]).bool());
        assert_eq!(e.get(this, TESRegionDataSound::pType).addr(), 0x7100);
        assert!(!e.call(0x004f_4c40, &args![this, 0x7100u32]).bool());
    }

    #[test]
    fn copy_makes_a_new_object_or_null() {
        let mut e = Engine::new();
        let this = sound_data(&mut e);
        fixed(&mut e, ALLOCATE, 0);
        assert_eq!(e.call(0x004f_4c70, &args![this]).u32(), 0);

        let block = e.mem.alloc(0x20);
        fixed(&mut e, ALLOCATE, block);
        stub(
            &mut e,
            &[
                REGION_DATA_COPY_CONSTRUCT,
                LIST_CONSTRUCT,
                LIST_REMOVE_ALL,
                0x9128,
            ],
        );
        e.put_vtable(0x0000_a000, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0x9128]);
        e.mem.set_u32(this.addr(), 0xa000);
        assert_eq!(e.call(0x004f_4c70, &args![this]).u32(), block);
        assert_eq!(e.mem.u32(block), VTABLE);
    }

    /// Blend doubles as in the landscape tests: `GetID`, the getters
    /// reading +0x07 (flag) and +0x06 (weight), and music type slots.
    fn blend_engine(id: u32) -> Engine {
        let mut e = Engine::new();
        fixed(&mut e, 0x0000_9100, id);
        fixed(&mut e, 0x0000_9128, 0x7100);
        stub(&mut e, &[0x0000_912c]);
        e.put_vtable(
            0x0000_a000,
            &[0, 0, 0, 0, 0x9100, 0, 0, 0, 0, 0, 0x9128, 0x912c],
        );
        e.register(REGION_DATA_GET_FLAG, |e, a| Ret {
            eax: e.mem.u8(a[0] + 7) as u32,
            ..Ret::default()
        });
        e.register(REGION_DATA_GET_WEIGHT, |e, a| Ret {
            eax: e.mem.u8(a[0] + 6) as u32,
            ..Ret::default()
        });
        stub(&mut e, &[REGION_DATA_SET_FLAG, REGION_DATA_SET_WEIGHT]);
        e.register(0x0040_6cc0, |_, a| Ret {
            st0: f32::from_bits(a[0]) as f64,
            ..Ret::default()
        });
        e.map(0x0102_3000, 0x1000);
        e.set_global(0x0102_3b88, 0.005f64);
        e
    }

    fn region_data(e: &mut Engine, ignore: u8, overriding: u8, weight: u8, flag: u8) -> Ptr {
        let p = Ptr::new(e.mem.alloc(0x20));
        e.mem.set_u32(p.addr(), 0xa000);
        e.mem.set_u8(p.addr() + 4, overriding);
        e.mem.set_u8(p.addr() + 5, ignore);
        e.mem.set_u8(p.addr() + 6, weight);
        e.mem.set_u8(p.addr() + 7, flag);
        p
    }

    #[test]
    fn blend_into_follows_ignore_override_flag_and_weight() {
        let mut e = blend_engine(5);
        let this = region_data(&mut e, 0, 0, 40, 0);
        let source = region_data(&mut e, 0, 0, 80, 1);
        e.call_log = Some(vec![]);
        e.call(0x004f_4cf0, &args![this, source, 1u32]);
        assert!(calls_to(&e, 0x912c).is_empty());

        let mut e = blend_engine(7);
        let source = region_data(&mut e, 0, 0, 80, 1);
        e.call_log = Some(vec![]);
        e.call(0x004f_4cf0, &args![this, source, 0u32]);
        assert!(calls_to(&e, 0x912c).is_empty());

        // This data ignored.
        let this = region_data(&mut e, 1, 0, 40, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_4cf0, &args![this, source, 1u32]);
        assert_eq!(
            calls_to(&e, REGION_DATA_SET_FLAG),
            vec![vec![this.addr(), 1]]
        );
        assert_eq!(
            calls_to(&e, REGION_DATA_SET_WEIGHT),
            vec![vec![this.addr(), 80]]
        );
        assert_eq!(calls_to(&e, 0x912c), vec![vec![this.addr(), 0x7100]]);

        // The source ignored.
        let this = region_data(&mut e, 0, 0, 40, 0);
        let ignored = region_data(&mut e, 1, 0, 80, 1);
        e.call_log = Some(vec![]);
        e.call(0x004f_4cf0, &args![this, ignored, 1u32]);
        assert!(calls_to(&e, 0x912c).is_empty());

        // Override with a lower source weight: nothing; higher: taken.
        let this = region_data(&mut e, 0, 1, 90, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_4cf0, &args![this, source, 1u32]);
        assert!(calls_to(&e, 0x912c).is_empty());
        let this = region_data(&mut e, 0, 1, 40, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_4cf0, &args![this, source, 1u32]);
        assert_eq!(calls_to(&e, 0x912c), vec![vec![this.addr(), 0x7100]]);

        // Flag set: taken.
        let this = region_data(&mut e, 0, 0, 90, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_4cf0, &args![this, source, 1u32]);
        assert_eq!(
            calls_to(&e, REGION_DATA_SET_WEIGHT),
            vec![vec![this.addr(), 80]]
        );

        // No flag: music type by weight, then the blended weight.
        let plain = region_data(&mut e, 0, 0, 80, 0);
        let this = region_data(&mut e, 0, 0, 40, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_4cf0, &args![this, plain, 1u32]);
        assert_eq!(calls_to(&e, 0x912c), vec![vec![this.addr(), 0x7100]]);
        assert_eq!(
            calls_to(&e, REGION_DATA_SET_WEIGHT),
            vec![vec![this.addr(), 68]]
        );
        let this = region_data(&mut e, 0, 0, 80, 0);
        let light = region_data(&mut e, 0, 0, 40, 0);
        e.call_log = Some(vec![]);
        e.call(0x004f_4cf0, &args![this, light, 1u32]);
        assert!(calls_to(&e, 0x912c).is_empty());
        assert_eq!(
            calls_to(&e, REGION_DATA_SET_WEIGHT),
            vec![vec![this.addr(), 68]]
        );
    }

    #[test]
    fn validate_is_true_without_a_music_type() {
        let mut e = Engine::new();
        let this = sound_data(&mut e);
        assert!(e.call(0x004f_4f70, &args![this]).bool());
        e.set(this, TESRegionDataSound::pType, Ptr::new(0x7100));
        assert!(!e.call(0x004f_4f70, &args![this]).bool());
    }

    #[test]
    fn media_set_battle_picks_by_random_draw() {
        let mut e = list_engine();
        e.register(LOOKUP_FORM, |_, a| Ret {
            eax: a[0] + 0x100,
            ..Ret::default()
        });
        let moved = Rc::new(RefCell::new(vec![]));
        for addr in [LIST_REMOVE, LIST_ADD] {
            let seen = moved.clone();
            e.register_double(addr, move |e, a| {
                seen.borrow_mut().push((a[0], e.mem.u32(a[1])));
                Ret::default()
            });
        }
        let this = sound_data(&mut e);
        // Empty list: null, no draw.
        stub(&mut e, &[RANDOM_INT]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_4f90, &args![this]).u32(), 0);
        assert!(calls_to(&e, RANDOM_INT).is_empty());

        fill_list(&mut e, this.addr() + 0x18, &[0x41, 0x42, 0x43]);
        // Draws 1, 0, 1: the first entry, then the second replaces it.
        let mut draws = vec![1u32, 0, 1].into_iter();
        e.register_double(RANDOM_INT, move |_, _| Ret {
            eax: draws.next().unwrap(),
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x004f_4f90, &args![this]).u32(), 0x142);
        assert_eq!(calls_to(&e, RANDOM_INT), vec![vec![0, 3]; 3]);
        assert_eq!(
            *moved.borrow(),
            vec![(this.addr() + 0x18, 0x42), (this.addr() + 0x18, 0x42)]
        );
    }
}
