//! `fallout/audio/falloutaudio.cpp` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `FalloutAudio` has no instance (size 1 in the Xbox PDB); everything it owns
//! is a static: the region-sound map, the ambient and underwater loop
//! handles, the acoustic-space state and a few fade settings. The addresses of
//! those statics below carry the Xbox PDB names; each address was matched to
//! its name by how this unit uses it (and, for the settings, by the
//! `name:Audio` string their static initializers pass).
//!
//! Several functions here are accessors of `TESSound`'s `SOUND_DATA` block
//! (the 0x24-byte block the game copies out of the sound at `+0x44`).
//! x87 note: the game computes in extended precision and stores `float`
//! results; the translations compute in `f64` and round to `f32` where the
//! code stores a `float`. Millisecond conversions are `FISTP` with the
//! control word forced to truncation, so they truncate toward zero.

use crate::prelude::*;
use crate::types::{BSSimpleList, NiTPointerMap};
use crate::units::bsaudio::bssoundhandle::BSSoundHandle;
use crate::units::fallout_ai::playercharacter::PlayerCharacter;

// ---- Functions of other units this unit calls. -------------------------------

/// `BSAudio::QInstance` (Xbox PDB): the audio singleton.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// Returns a pointer to the setting's value (`this + 4`), or to a static zero
/// float for a null setting (folded accessor, no Xbox PDB name).
const SETTING_VALUE: u32 = 0x0040_3e20;
/// `TESDataHandler::GetSound_ov2` (Xbox PDB): the `TESSound` form of an editor id.
const GET_SOUND: u32 = 0x0046_16c0;
/// The string (`BSStringT` at `+0x30`) of a sound: its file name.
const SOUND_FILE_NAME: u32 = 0x0051_1840;
/// Copies the sound's 0x24-byte `SOUND_DATA` block (at `+0x44`) into the
/// buffer argument and returns the buffer.
const COPY_SOUND_DATA: u32 = 0x004e_75d0;
/// Maps a `SOUND_DATA.iFlags` word to the audio flag bits (0x10, 0x2000000,
/// 0x4000000 or 0).
const SOUND_FLAG_BITS: u32 = 0x005e_39b0;
/// cdecl `(destination, size, source)` string copy.
const COPY_STRING: u32 = 0x0040_6d30;
/// cdecl printf-style log line.
const LOG_MESSAGE: u32 = 0x005b_5e40;
/// Byte at `+0x61a` of the data handler.
const DATA_HANDLER_FLAG: u32 = 0x0045_16b0;
/// Word at `+0x218` of the data handler (`MiddleHighProcess::GetHeadNode` is the
/// map's folded name).
const DATA_HANDLER_HEAD_NODE: u32 = 0x0051_f550;
/// `BSSoundHandle` copy assignment `(this, source)`.
const SOUND_HANDLE_ASSIGN: u32 = 0x0041_8900;
/// `BSSoundHandle` reset: sound id -1, flag byte 0, state 0.
const SOUND_HANDLE_RESET: u32 = 0x0041_88d0;
/// `BSSoundHandle` construction from a sound id (`this`, id): flag byte and
/// state 0.
const SOUND_HANDLE_CONSTRUCT_WITH_ID: u32 = 0x0057_bd10;
/// `BSSoundHandle` default construction (id -1, flag byte 0, state 0).
const SOUND_HANDLE_CONSTRUCT: u32 = 0x0041_a250;
/// Empty destructor of `BSSoundHandle`.
const SOUND_HANDLE_DESTROY: u32 = 0x0048_3710;
/// Deleting destructor of `BSSoundHandle` `(this, flags)`: with bit 0 it frees.
const SOUND_HANDLE_DELETE: u32 = 0x007b_3fa0;
/// `BSSoundHandle::IsPlaying`.
const SOUND_HANDLE_IS_PLAYING: u32 = 0x00ad_8930;
/// `BSSoundHandle::IsValid`.
const SOUND_HANDLE_IS_VALID: u32 = 0x00ad_8ce0;
/// `BSSoundHandle::FadeOutAndRelease` `(this, milliseconds)`.
const SOUND_HANDLE_FADE_OUT_AND_RELEASE: u32 = 0x00ad_8da0;
/// `(this, milliseconds)`: sets the state to playing and passes the time to
/// the sound manager (fade-in).
const SOUND_HANDLE_START_FADE: u32 = 0x00ad_8d60;
/// `BSSoundHandle::Play` `(this, flag)`.
const SOUND_HANDLE_PLAY: u32 = 0x00ad_8830;
/// `BSSoundHandle::SetPosition` `(this, x, y, z)`.
const SOUND_HANDLE_SET_POSITION: u32 = 0x00ad_8b60;
/// `BSSoundHandle::SetPriority` `(this, priority)`.
const SOUND_HANDLE_SET_PRIORITY: u32 = 0x00ad_9030;
/// `BSSoundHandle::SetStaticAttenuation` `(this, word)`.
const SOUND_HANDLE_SET_STATIC_ATTENUATION: u32 = 0x00ad_89b0;
/// `BSSoundHandle::SetMinMax` `(this, min, max)`.
const SOUND_HANDLE_SET_MIN_MAX: u32 = 0x00ad_8be0;
/// `(this, a, b, c, d, e)`: five attenuation-curve words.
const SOUND_HANDLE_SET_CURVE: u32 = 0x00ad_8c20;
/// `(this, word)`: the reverb attenuation.
const SOUND_HANDLE_SET_REVERB: u32 = 0x00ad_8c70;
/// `(this, first byte, second byte)`: the time-constraint bytes.
const SOUND_HANDLE_SET_TIME_BYTES: u32 = 0x00ad_8e20;
/// `(this, byte)`: passes the pitch byte of the sound data.
const SOUND_HANDLE_SET_PITCH: u32 = 0x00ad_8ad0;
/// `BSAudio::GetSoundHandleByName` `(audio, result, name, flags)`.
const AUDIO_GET_SOUND_HANDLE_BY_NAME: u32 = 0x00ad_7550;
/// `BSAudio::GetSoundHandleByFilename` `(audio, result, file, flags, form)`.
const AUDIO_GET_SOUND_HANDLE_BY_FILENAME: u32 = 0x00ad_7480;
/// `BSAudio::Precache(audio, name, flags)`.
const AUDIO_PRECACHE: u32 = 0x00ad_7fa0;
/// `BSAudio::Precache_ov3` `(audio, file name, flags, form)`.
const AUDIO_PRECACHE_FORM: u32 = 0x00ad_8100;
/// `BSAudio::SetEnvironment` `(audio, environment)`.
const AUDIO_SET_ENVIRONMENT: u32 = 0x00ad_79b0;
/// `(audio, float, word)`: the game hour for the audio manager.
const AUDIO_SET_GAME_HOUR: u32 = 0x00ad_77f0;
/// Byte at the audio manager's `+0x175` (listener underwater).
const AUDIO_LISTENER_UNDERWATER: u32 = 0x00ad_7c50;
/// Elapsed audio time in milliseconds (the audio object's `SynchTimer`,
/// `this + 0x14`).
const AUDIO_ELAPSED_TIME: u32 = 0x0063_d040;
/// Absolute value of an `int`.
const ABSOLUTE_VALUE: u32 = 0x0072_fa60;
/// Getter of the word at `+0x10`: the audio object's listener (`pListener` in
/// `bsaudio.rs`) or the sky's `pCurrentWeather` (Xbox PDB). The map's
/// `GetCurrentProcedureIndex` is a folded name.
const WORD_AT_0X10: u32 = 0x0044_edb0;
/// Byte at `+0x45` of a sound: its `SOUND_DATA.iMax` (the map files this
/// getter under `tesobjectcell.cpp` because the linker folded it).
const SOUND_MAX_DISTANCE: u32 = 0x0055_3b90;
/// Absolute value of a float, returned in ST0.
const FLOAT_ABSOLUTE_VALUE: u32 = 0x0040_8840;
/// `(this, byte)`: stores the byte at `+4` of a sound handle.
const SOUND_HANDLE_SET_FLAG_BYTE: u32 = 0x004f_15a0;
/// `Interface::IsTopMenuID(id)`, cdecl.
const IS_TOP_MENU_ID: u32 = 0x0070_2450;
/// `ImpactMixer::InitFootsteps`.
const IMPACT_MIXER_INIT_FOOTSTEPS: u32 = 0x0083_9ab0;
/// The second impact-mixer helper called after `InitFootsteps`.
const IMPACT_MIXER_UPDATE: u32 = 0x0083_9d30;
/// A method of the `TES` object (the word at `011dea10`; unnamed in the map).
const TES_METHOD_00455690: u32 = 0x0045_5690;
/// `TES::pSky` getter (`this + 0x68`; the map's name is folded).
const TES_GET_SKY: u32 = 0x008d_8520;
/// `Sky::fCurrentGameHour` getter (`this + 0xec`, returned in ST0).
const SKY_GET_GAME_HOUR: u32 = 0x0096_6a20;
/// `(weather, index)`: the byte at `weather + index + 0xe0`.
const WEATHER_BYTE: u32 = 0x0063_dd50;
/// Parent cell (`this + 0x40`) of the player.
const PARENT_CELL: u32 = 0x008d_6f30;
/// `TESObjectCELL::GetAcousticSpace` (Xbox PDB name, on a folded body).
const CELL_GET_ACOUSTIC_SPACE: u32 = 0x0054_7590;
/// Byte at `+0x30` of an acoustic space (the transition flag test).
const SPACE_FLAG: u32 = 0x0058_a200;
/// Word at `+0x4c` of an acoustic space, the environment type for
/// `SetEnvironment`.
const SPACE_ENVIRONMENT: u32 = 0x0068_a810;
/// Word at `+0x48` of an acoustic space.
const SPACE_REGION_OWNER: u32 = 0x0067_33e0;
/// Word at `+0x18` of that object: its region data list.
const REGION_DATA_LIST: u32 = 0x0096_11e0;
/// `TESRegionDataList::Find(list, type)`.
const REGION_DATA_LIST_FIND: u32 = 0x004f_35b0;
/// Address `this + 0xc` of a region data entry: its list of region sounds.
const REGION_DATA_SOUND_LIST: u32 = 0x0048_d150;
/// Words at `+0x34`, `+0x38`, `+0x3c` and `+0x40` of an acoustic space.
const SPACE_SOUND_0: u32 = 0x005f_36f0;
const SPACE_SOUND_1: u32 = 0x009e_32d0;
const SPACE_SOUND_2: u32 = 0x0063_9b40;
const SPACE_SOUND_3: u32 = 0x008d_6f30;
/// Word at `+0xc` of a form (its form id).
const FORM_ID: u32 = 0x0084_e3a0;
/// `BSSimpleList` node helpers: empty test of a node, and next node.
const LIST_NODE_IS_EMPTY: u32 = 0x0082_56d0;
const LIST_NODE_NEXT: u32 = 0x0072_6070;
/// Random number from the game's generator (`0x00476c00` then `0x00476be0`).
const RANDOM_NUMBER: u32 = 0x0048_7f50;
/// `(chance)`: true when a random roll is within the float.
const RANDOM_CHANCE: u32 = 0x004d_ff00;
/// `__RTDynamicCast(pointer, vfdelta, source type, target type, is_reference)`.
const DYNAMIC_CAST: u32 = 0x00ec_43fb;
/// `BGSLoadGameBuffer::LoadFormID`.
const LOAD_FORM_ID: u32 = 0x0086_48a0;
/// Form lookup by form id through the form map (the word at `011c54c0`); 0 when absent.
const LOOKUP_FORM: u32 = 0x0048_39c0;
/// `BGSSaveGameBuffer::SaveFormID_ov2` `(buffer, form, flag)`.
const SAVE_FORM_ID: u32 = 0x0086_5df0;
/// Map operations on `CurrentRegionSounds`: first position, next `(map, &position, &key, &value)`,
/// remove `(map, key)`, set `(map, key, value)`, lookup `(map, key, &value)` and the empty test
/// (the map's names for these are folded `NiTMapBase` ones).
const MAP_FIRST_POSITION: u32 = 0x004b_9ba0;
const MAP_NEXT: u32 = 0x006b_7f20;
const MAP_REMOVE_AT: u32 = 0x0040_5430;
const MAP_SET_AT: u32 = 0x0084_4700;
const MAP_LOOKUP: u32 = 0x0085_3130;
const MAP_IS_EMPTY: u32 = 0x0069_b080;
/// Heap allocation `(size)`, and the allocator's `Allocate`/`Deallocate` (`00aa1070`, `00aa10f0`).
const ALLOCATE: u32 = 0x0040_1000;
const MEMORY_ALLOCATE: u32 = 0x00aa_1070;
const MEMORY_FREE: u32 = 0x00aa_10f0;
/// cdecl `memset(block, value, size)`.
const MEMSET: u32 = 0x0040_3d30;
/// Frees a block allocated with the game heap `(block)`.
const FREE_OBJECT: u32 = 0x0040_1030;
/// Empties the map: releases each node and zeroes the item count (`NiTMapBase` clear, folded).
const MAP_CLEAR: u32 = 0x0043_8af0;
/// Vtable slot `0xc` of the audio listener: returns a pointer to its
/// position (three floats).
const LISTENER_POSITION_SLOT: u32 = 0xc;
/// Vtable slot `0x1f4` of the player: returns a pointer to its position (three floats).
const PLAYER_POSITION_SLOT: u32 = 0x1f4;

// ---- Globals. -------------------------------------------------------------------

/// The `PlayerCharacter` pointer.
const PLAYER: u32 = 0x011d_ea3c;
/// The `TES` pointer.
const TES: u32 = 0x011d_ea10;
/// The `TESDataHandler` pointer.
const DATA_HANDLER: u32 = 0x011c_3f2c;
/// `FalloutAudio::CurrentRegionSounds` (Xbox PDB): `NiTPointerMap<u32, BSSoundHandle*>`.
const CURRENT_REGION_SOUNDS: u32 = 0x011d_d014;
/// `FalloutAudio::fASFadeStartTime`.
const FAS_FADE_START_TIME: u32 = 0x011d_d044;
/// `FalloutAudio::fASFadeOutTime`.
const FAS_FADE_OUT_TIME: u32 = 0x011d_d050;
/// `FalloutAudio::fRegionLoopFadeInTime`.
const REGION_LOOP_FADE_IN_TIME: u32 = 0x011d_d06c;
/// `FalloutAudio::fASFadeInTime`.
const FAS_FADE_IN_TIME: u32 = 0x011d_d094;
/// `FalloutAudio::fRegionLoopFadeOutTime`.
const REGION_LOOP_FADE_OUT_TIME: u32 = 0x011d_cfe4;
/// `FalloutAudio::UnderwaterLoop` (`BSSoundHandle`).
const UNDERWATER_LOOP: u32 = 0x011d_d0a0;
/// `FalloutAudio::AmbientLoop` (`BSSoundHandle`).
const AMBIENT_LOOP: u32 = 0x011d_d0d8;
/// `FalloutAudio::bRegionUpdateDisabled` (byte).
const REGION_UPDATE_DISABLED: u32 = 0x011d_cfa6;
/// `FalloutAudio::bLockAcousticSpace` (byte).
const LOCK_ACOUSTIC_SPACE: u32 = 0x011d_cfa7;
/// `FalloutAudio::pNoActivationSound`.
const NO_ACTIVATION_SOUND: u32 = 0x011d_cfa8;
/// `FalloutAudio::iCurrentTime`: which of the four ambient sounds plays.
const CURRENT_TIME_SLOT: u32 = 0x011d_cfac;
/// A byte cleared whenever the ambient loop is replaced. Its Xbox PDB name is
/// not confirmed (`bUseWalla` is the candidate); nothing in this unit reads it.
const AMBIENT_REPLACED_FLAG: u32 = 0x011d_cfb0;
/// `FalloutAudio::bUpdateAcousticSpace` (byte).
const UPDATE_ACOUSTIC_SPACE: u32 = 0x011d_cfb1;
/// `FalloutAudio::pCurrentSpace`.
const CURRENT_SPACE: u32 = 0x011d_cfb4;
/// `FalloutAudio::pNextAcousticSpace`.
const NEXT_SPACE: u32 = 0x011d_cfb8;
/// `FalloutAudio::iLastRegionUpdate`: the audio time of the last full update.
const LAST_REGION_UPDATE: u32 = 0x011d_cfc0;
/// The four time-of-day settings the ambient selection compares the game hour
/// against (labelled `fDawnTime_`, `fDayTime_`, `fDuskTime_` and `fNightTime_`
/// in the Ghidra project).
const DAWN_TIME: u32 = 0x011f_6dc0;
const DAY_TIME: u32 = 0x011f_6dcc;
const DUSK_TIME: u32 = 0x011f_6dd8;
const NIGHT_TIME: u32 = 0x011f_6de4;

// ---- Constants read from the exe's data. -------------------------------------------

/// `1000.0` (`double`).
const MILLISECONDS_PER_SECOND: u32 = 0x0101_7b70;
/// `500.0` (`double`).
const HALF_SECOND_SCALE: u32 = 0x0103_01a8;
/// `100.0` (`double`).
const ONE_HUNDRED: u32 = 0x0101_7a40;
/// `5.0` (`double`).
const FIVE: u32 = 0x0102_0998;
/// `0.09375` (`double`): 1/32 of a day, the time-of-day step of a sound.
const TIME_STEP: u32 = 0x0103_4200;
/// `20000000.0` (`double`).
const CHANCE_DIVISOR: u32 = 0x0107_c898;
/// `0.5` (`float`).
const HALF: u32 = 0x0101_6248;
/// `-1.0` (`double`).
const MINUS_ONE: u32 = 0x0101_a6b0;
/// `256.0` (`double`).
const HEIGHT_OFFSET: u32 = 0x0102_31d8;

// ---- Strings in the exe's data. ----------------------------------------------------

/// `"UIActivateNothing"`.
const UI_ACTIVATE_NOTHING: u32 = 0x0107_47e0;
/// `"AMBUnderwaterLP"`.
const UNDERWATER_LOOP_NAME: u32 = 0x0107_c8a0;
/// `"MUSMysteriousStrangerA01"`.
const MYSTERIOUS_STRANGER_A01: u32 = 0x0107_c8cc;
/// `"MUSMysteriousStrangerA02"`.
const MYSTERIOUS_STRANGER_A02: u32 = 0x0107_c8b0;
/// `"Audio: Could not find TESSound corresponding to form name \"%s\""`.
const SOUND_NOT_FOUND_MESSAGE: u32 = 0x0107_c854;

/// Vtable of `NiTPointerMap<unsigned int, BSSoundHandle*>`.
const SOUND_HANDLE_MAP_VTABLE: u32 = 0x0107_ca44;
/// Vtable of its base, `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int, BSSoundHandle*>`.
const SOUND_HANDLE_MAP_BASE_VTABLE: u32 = 0x0107_ca64;
/// RTTI type descriptors the load-game cast uses (source class, target class).
const FORM_TYPE_DESCRIPTOR: u32 = 0x0118_3028;
const SOUND_TYPE_DESCRIPTOR: u32 = 0x0118_32fc;

layout! {
    /// `TESSound` (Xbox PDB), the fields the audio code reads at their PC
    /// offsets: the `SOUND_DATA` block starts at `+0x44` (the size here is the
    /// extent of the fields used, not the class size).
    pub struct TESSound: 0x68 {
        /// `data.iMin` (Xbox PDB `SOUND_DATA`).
        0x44 iMin: u8,
        /// `data.iMax`.
        0x45 iMax: u8,
        /// `data.iAttenuation`.
        0x4C iAttenuation: u16,
        /// `data.iTimeConstraints`.
        0x4E iTimeConstraints: u16,
        /// `data.pAttenuationCurve`: ten bytes of `u16`; the code reads the
        /// first five.
        0x50 pAttenuationCurve: u16,
        /// `data.iReverbAttenuation`.
        0x5A iReverbAttenuation: u16,
        /// `data.iPriority`.
        0x5C iPriority: u32,
    }

    /// `TESSound::SOUND_DATA` (Xbox PDB), 0x24 bytes, as copied to a buffer.
    pub struct SoundData: 0x24 {
        /// `iPitch` (a `char`).
        0x02 iPitch: u8,
        /// `iFlags`.
        0x04 iFlags: u32,
        /// `iLoopBegin`.
        0x1C iLoopBegin: u32,
        /// `iLoopEnd`.
        0x20 iLoopEnd: u32,
    }

    /// `TESRegionSound` (Xbox PDB), 0xC bytes.
    pub struct TESRegionSound: 0x0C {
        /// `uiSound` (Xbox PDB): the `TESSound` form.
        0x00 uiSound: u32,
        /// `uiFlags`: the low 16 bits are a mask tested against the weather's byte.
        0x04 uiFlags: u32,
        /// `uiChance`.
        0x08 uiChance: u32,
    }
}

// ---- Helpers. -----------------------------------------------------------------------

fn audio_instance(e: &mut Engine) -> u32 {
    e.call(AUDIO_INSTANCE, &[]).u32()
}

/// The value of a float setting (`*SETTING_VALUE(setting)`).
fn setting_value(e: &mut Engine, setting: u32) -> f32 {
    let value = e.call(SETTING_VALUE, &args![setting]).u32();
    e.mem.f32(value)
}

/// A time setting times a scale constant, truncated like `FISTP` under a
/// truncating control word, then taken as the 32-bit word pushed.
fn truncated_scaled(e: &Engine, value: f32, scale_at: u32) -> u32 {
    let scale: f64 = e.global(scale_at);
    (value as f64 * scale) as i64 as u32
}

/// A word of the sound's data block (the block is copied each time, as in the game).
fn sound_data<T: Scalar>(e: &mut Engine, sound: u32, field: Field<SoundData, T>) -> T {
    e.with_stack(0x24, |e, copy| {
        let block = e
            .call(COPY_SOUND_DATA, &args![sound, copy])
            .ptr::<SoundData>();
        e.get(block, field)
    })
}

fn fade_out_and_release(e: &mut Engine, handle: u32, milliseconds: u32) {
    e.call(
        SOUND_HANDLE_FADE_OUT_AND_RELEASE,
        &args![handle, milliseconds],
    );
}

fn is_playing(e: &mut Engine, handle: u32) -> bool {
    e.call(SOUND_HANDLE_IS_PLAYING, &args![handle]).bool()
}

/// `GetSoundHandleByFilename` into a temporary that is assigned to `target`
/// and destroyed.
fn assign_file_handle(e: &mut Engine, target: u32, form: u32, flags: u32) {
    let file = e.call(SOUND_FILE_NAME, &args![form]).u32();
    e.with_stack(0xC, |e, temporary| {
        let audio = audio_instance(e);
        let handle = e
            .call(
                AUDIO_GET_SOUND_HANDLE_BY_FILENAME,
                &args![audio, temporary, file, flags, form],
            )
            .u32();
        e.call(SOUND_HANDLE_ASSIGN, &args![target, handle]);
        e.call(SOUND_HANDLE_DESTROY, &args![temporary]);
    });
}

fn parent_cell_space(e: &mut Engine, player: u32) -> u32 {
    if e.call(PARENT_CELL, &args![player]).u32() == 0 {
        0
    } else {
        let cell = e.call(PARENT_CELL, &args![player]).u32();
        e.call(CELL_GET_ACOUSTIC_SPACE, &args![cell]).u32()
    }
}

/// The float `Setting` is below the game hour (`FCOMPP` with the hour loaded
/// first: the branch is taken only on an ordered "less than").
fn setting_below(e: &mut Engine, setting: u32, game_hour: f32) -> bool {
    setting_value(e, setting) < game_hour
}

// Translated from 0082d280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutAudio::ResolveSoundName` (Xbox PDB): looks the sound form up by
/// editor id. When it exists, has a file name and the caller gave a buffer,
/// copies the file name (at most 0x104 bytes) into `buffer`, merges the
/// sound's flags into `*flags` and stores the form in `*out_sound`; answers
/// true. Otherwise (the form is missing, has no file name or `buffer` is
/// null) logs the unknown name when the data handler is loaded far enough and
/// answers false. A null `name` answers false at once.
pub fn fallout_audio_resolve_sound_name(
    e: &mut Engine,
    name: u32,
    buffer: u32,
    flags: Ptr,
    out_sound: Ptr,
) -> bool {
    if name == 0 {
        return false;
    }
    let handler: u32 = e.global(DATA_HANDLER);
    let sound = e.call(GET_SOUND, &args![handler, name]).u32();
    if sound != 0 && e.call(SOUND_FILE_NAME, &args![sound]).u32() != 0 && buffer != 0 {
        let file = e.call(SOUND_FILE_NAME, &args![sound]).u32();
        e.call(COPY_STRING, &args![buffer, 0x104u32, file]);
        let word = e.mem.u32(flags.addr());
        if sound_data(e, sound, SoundData::iFlags) & 0x40 != 0 {
            e.mem.set_u32(flags.addr(), word | 1);
            let word = e.mem.u32(flags.addr());
            e.mem.set_u32(flags.addr(), word & !2);
        } else if word & 1 == 0 && word & 2 == 0 {
            e.mem.set_u32(flags.addr(), word | 2);
            let word = e.mem.u32(flags.addr());
            e.mem.set_u32(flags.addr(), word & !1);
        }
        if sound_data(e, sound, SoundData::iFlags) & 1 != 0 {
            let word = e.mem.u32(flags.addr());
            e.mem.set_u32(flags.addr(), word | 0x40);
        }
        if sound_data(e, sound, SoundData::iFlags) & 0x800 != 0 {
            let word = e.mem.u32(flags.addr());
            e.mem.set_u32(flags.addr(), word | 0x0800_0000);
        }
        let sound_flags = sound_data(e, sound, SoundData::iFlags);
        let bits = e.call(SOUND_FLAG_BITS, &args![sound_flags]).u32();
        let word = e.mem.u32(flags.addr());
        e.mem.set_u32(flags.addr(), bits | word);
        e.mem.set_u32(out_sound.addr(), sound);
        return true;
    }
    if handler != 0
        && !e.call(DATA_HANDLER_FLAG, &args![handler]).bool()
        && e.call(DATA_HANDLER_HEAD_NODE, &args![handler]).u32() != 0
    {
        e.call(LOG_MESSAGE, &args![SOUND_NOT_FOUND_MESSAGE, name]);
    }
    false
}

// Translated from 0082d400 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutAudio::ApplySettingsFromMasterFile` (Xbox PDB): copies the sound
/// form's settings onto a sound handle and answers the audio flag bits.
///
/// In order: the static attenuation; the minimum and maximum distance bytes
/// (defaults 14 and 60 when zero) scaled by 5 and 100 into the handle's
/// min/max; the two time-constraint bytes when either is set; the five
/// attenuation-curve words; the reverb attenuation; the priority byte; then,
/// from the copied `SOUND_DATA` flags, bit 0 (adds 0x40 and passes the pitch),
/// bit 0x800 (adds 0x8000000), and, when `extra_flags` is given, bits 0x1000
/// and 0x2000 (set bits 1 and 2 there). The result also holds
/// `SOUND_FLAG_BITS` of the flags. A null sound answers 0.
pub fn fallout_audio_apply_settings_from_master_file(
    e: &mut Engine,
    handle: Ptr<BSSoundHandle>,
    sound: Ptr<TESSound>,
    extra_flags: Ptr,
) -> u32 {
    if sound.is_null() {
        return 0;
    }
    let mut result = 0u32;
    let attenuation = fn_0082d680(e, sound);
    e.call(
        SOUND_HANDLE_SET_STATIC_ATTENUATION,
        &args![handle, attenuation],
    );
    let mut minimum = fn_0082d660(e, sound) as u32;
    let mut maximum = e.call(SOUND_MAX_DISTANCE, &args![sound]).u8() as u32;
    if minimum == 0 {
        minimum = 0xe;
    }
    if maximum == 0 {
        maximum = 0x3c;
    }
    if fn_0082d6e0(e, sound) as u8 != 0 || fn_0082d700(e, sound) as u8 != 0 {
        let low = fn_0082d700(e, sound) as u8;
        let high = fn_0082d6e0(e, sound) as u8;
        e.call(SOUND_HANDLE_SET_TIME_BYTES, &args![handle, high, low]);
    }
    let five: f64 = e.global(FIVE);
    let hundred: f64 = e.global(ONE_HUNDRED);
    let near = (minimum as f64 * five) as f32;
    let far = (maximum as f64 * hundred) as f32;
    // The code builds the far value first, then the near one, and passes
    // (near, far).
    e.call(SOUND_HANDLE_SET_MIN_MAX, &args![handle, near, far]);
    let curve_4 = fn_0082d6a0(e, sound, 4);
    let curve_3 = fn_0082d6a0(e, sound, 3);
    let curve_2 = fn_0082d6a0(e, sound, 2);
    let curve_1 = fn_0082d6a0(e, sound, 1);
    let curve_0 = fn_0082d6a0(e, sound, 0);
    e.call(
        SOUND_HANDLE_SET_CURVE,
        &args![handle, curve_0, curve_1, curve_2, curve_3, curve_4],
    );
    let reverb = fn_0082d6c0(e, sound);
    e.call(SOUND_HANDLE_SET_REVERB, &args![handle, reverb]);
    let priority = fn_0082d720(e, sound) as u8;
    e.call(SOUND_HANDLE_SET_PRIORITY, &args![handle, priority]);
    if sound_data(e, sound.addr(), SoundData::iFlags) & 1 != 0 {
        result |= 0x40;
        let pitch = sound_data(e, sound.addr(), SoundData::iPitch);
        e.call(SOUND_HANDLE_SET_PITCH, &args![handle, pitch]);
    }
    if sound_data(e, sound.addr(), SoundData::iFlags) & 0x800 != 0 {
        result |= 0x0800_0000;
    }
    if !extra_flags.is_null() {
        if sound_data(e, sound.addr(), SoundData::iFlags) & 0x1000 != 0 {
            let word = e.mem.u32(extra_flags.addr());
            e.mem.set_u32(extra_flags.addr(), word | 1);
        }
        if sound_data(e, sound.addr(), SoundData::iFlags) & 0x2000 != 0 {
            let word = e.mem.u32(extra_flags.addr());
            e.mem.set_u32(extra_flags.addr(), word | 2);
        }
    }
    let sound_flags = sound_data(e, sound.addr(), SoundData::iFlags);
    result | e.call(SOUND_FLAG_BITS, &args![sound_flags]).u32()
}

// Translated from 0082d660 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESSound::SOUND_DATA.iMin` byte (`this + 0x44`); no Xbox PDB name.
pub fn fn_0082d660(e: &mut Engine, this: Ptr<TESSound>) -> u8 {
    e.get(this, TESSound::iMin)
}

// Translated from 0082d680 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SOUND_DATA.iAttenuation` word (`this + 0x4c`); no Xbox PDB name.
pub fn fn_0082d680(e: &mut Engine, this: Ptr<TESSound>) -> u16 {
    e.get(this, TESSound::iAttenuation)
}

// Translated from 0082d6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Entry `index` of `SOUND_DATA.pAttenuationCurve` (words from `this + 0x50`);
/// no Xbox PDB name.
pub fn fn_0082d6a0(e: &mut Engine, this: Ptr<TESSound>, index: u32) -> u16 {
    e.mem.u16(
        this.addr()
            .wrapping_add(0x50)
            .wrapping_add(index.wrapping_mul(2)),
    )
}

// Translated from 0082d6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `SOUND_DATA.iReverbAttenuation` word (`this + 0x5a`); no Xbox PDB name.
pub fn fn_0082d6c0(e: &mut Engine, this: Ptr<TESSound>) -> u16 {
    e.get(this, TESSound::iReverbAttenuation)
}

// Translated from 0082d6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The high byte of `SOUND_DATA.iTimeConstraints` (`this + 0x4e`), as a
/// number from 0 to 255; no Xbox PDB name.
pub fn fn_0082d6e0(e: &mut Engine, this: Ptr<TESSound>) -> u32 {
    (e.get(this, TESSound::iTimeConstraints) as u32 & 0xff00) >> 8
}

// Translated from 0082d700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The low byte of `SOUND_DATA.iTimeConstraints`; no Xbox PDB name.
pub fn fn_0082d700(e: &mut Engine, this: Ptr<TESSound>) -> u32 {
    e.get(this, TESSound::iTimeConstraints) as u32 & 0xff
}

// Translated from 0082d720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The low byte of `SOUND_DATA.iPriority` (`this + 0x5c`); no Xbox PDB name.
pub fn fn_0082d720(e: &mut Engine, this: Ptr<TESSound>) -> u32 {
    e.get(this, TESSound::iPriority) & 0xff
}

// Translated from 0082d740 (decompiled, FalloutNV.exe 1.4.0.525)
/// A cdecl callback (it is referenced from a table of function pointers at
/// `0086ce40`; no Xbox PDB name): for a non-null sound whose `SOUND_DATA`
/// has a nonzero loop begin or loop end, stores both into the two output
/// words and answers true; otherwise answers false.
pub fn fn_0082d740(e: &mut Engine, sound: u32, loop_begin: Ptr, loop_end: Ptr) -> bool {
    if sound == 0 {
        return false;
    }
    if sound_data(e, sound, SoundData::iLoopBegin) == 0
        && sound_data(e, sound, SoundData::iLoopEnd) == 0
    {
        return false;
    }
    let begin = sound_data(e, sound, SoundData::iLoopBegin);
    e.mem.set_u32(loop_begin.addr(), begin);
    let end = sound_data(e, sound, SoundData::iLoopEnd);
    e.mem.set_u32(loop_end.addr(), end);
    true
}

// Translation incomplete: 0082d7c0 - C++ exception-unwind frame (FS:[0]) omitted; the normal path is translated.
// Translated from 0082d7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutAudio::UpdateRegionSounds` (Xbox PDB): the audio's per-frame
/// update of the acoustic space, the ambient and underwater loops and the
/// region sounds of the player.
///
/// 1. Acoustic space. The space of the player's parent cell is the
///    candidate. When a space is current and has the `SPACE_FLAG` byte clear
///    and the candidate has it set, the candidate becomes current at once
///    (`fn_0082eae0`). A pending change (`bUpdateAcousticSpace`) is applied
///    or dropped. The "no activation" sound form is looked up once and
///    precached. With no current space but a candidate that differs, the
///    candidate becomes current; with no cell (and no transition flag on
///    the player) the space is cleared. The environment of the current space
///    is handed to the audio.
/// 2. Underwater loop: started when the listener is underwater and the
///    player has a parent cell, faded out when not.
/// 3. `ImpactMixer::InitFootsteps` and its companion run; the rest is
///    skipped while `bRegionUpdateDisabled` is set.
/// 4. The game hour goes to the audio manager. Less than 1000 ms after the
///    last full update only the cleanup runs: with an empty region sound
///    list the map of looping region sounds is drained (fade 300 ms,
///    handles deleted). Otherwise the ambient loop is chosen from the
///    current space (a transition space plays its `+0x34` sound; others
///    pick the sound whose time of day matches the hour, with the fade
///    settings), then every region sound of the list is started, kept or
///    faded according to its time-of-day window, weather mask and loop flag
///    (a looping sound is tracked in the map by form id; a one-shot is rolled
///    against its chance and played at a random point around the listener,
///    priority 0xff), and map entries whose form left the list are faded out
///    and deleted. The last-update time is stored.
///
/// The compiler's exception-unwinding frame (the destruction of the local
/// `BSSoundHandle` and the temporaries) is not translated; those destructors
/// are empty.
pub fn fallout_audio_update_region_sounds(e: &mut Engine) {
    let player: u32 = e.global(PLAYER);
    let region_sound_list = fn_0082eaa0(e, Ptr::new(player)).addr();
    let audio = audio_instance(e);
    let underwater = e.call(AUDIO_LISTENER_UNDERWATER, &args![audio]).u8();
    let candidate = if player == 0 {
        0
    } else {
        parent_cell_space(e, player)
    };
    let current: u32 = e.global(CURRENT_SPACE);
    if current != 0
        && !e.call(SPACE_FLAG, &args![current]).bool()
        && candidate != 0
        && e.call(SPACE_FLAG, &args![candidate]).bool()
    {
        fn_0082eae0(e, candidate);
        e.set_global(UPDATE_ACOUSTIC_SPACE, 0u8);
        e.set_global(NEXT_SPACE, 0u32);
    }
    if e.global::<u8>(UPDATE_ACOUSTIC_SPACE) != 0 {
        let next: u32 = e.global(NEXT_SPACE);
        if next == e.global::<u32>(CURRENT_SPACE) {
            e.set_global(NEXT_SPACE, 0u32);
            e.set_global(UPDATE_ACOUSTIC_SPACE, 0u8);
        } else {
            fn_0082eae0(e, next);
        }
    }
    if e.global::<u32>(NO_ACTIVATION_SOUND) == 0 {
        let handler: u32 = e.global(DATA_HANDLER);
        let sound = e
            .call(GET_SOUND, &args![handler, UI_ACTIVATE_NOTHING])
            .u32();
        e.set_global(NO_ACTIVATION_SOUND, sound);
        if sound != 0 {
            let file = e.call(SOUND_FILE_NAME, &args![sound]).u32();
            let audio = audio_instance(e);
            e.call(AUDIO_PRECACHE_FORM, &args![audio, file, 0x121u32, sound]);
        }
    }
    let mut changed_space = false;
    if e.global::<u32>(CURRENT_SPACE) == 0 && player != 0 {
        let cell = e.call(PARENT_CELL, &args![player]).u32();
        if cell != 0 {
            let cell = e.call(PARENT_CELL, &args![player]).u32();
            let space = e.call(CELL_GET_ACOUSTIC_SPACE, &args![cell]).u32();
            if space != e.global::<u32>(CURRENT_SPACE) {
                let cell = e.call(PARENT_CELL, &args![player]).u32();
                let space = e.call(CELL_GET_ACOUSTIC_SPACE, &args![cell]).u32();
                fn_0082eae0(e, space);
                changed_space = true;
            }
        }
    }
    if !changed_space
        && (player == 0
            || (e.call(PARENT_CELL, &args![player]).u32() == 0
                && fn_0082eac0(e, Ptr::new(player)) == 0))
    {
        fallout_audio_clear_acoustic_space(e, 0);
    }
    let mut region_sound_list = region_sound_list;
    let current: u32 = e.global(CURRENT_SPACE);
    if current != 0 && e.call(SPACE_REGION_OWNER, &args![current]).u32() != 0 {
        let owner = e.call(SPACE_REGION_OWNER, &args![current]).u32();
        let data_list = e.call(REGION_DATA_LIST, &args![owner]).u32();
        let entry = e.call(REGION_DATA_LIST_FIND, &args![data_list, 7u32]).u32();
        if entry != 0 {
            region_sound_list = e.call(REGION_DATA_SOUND_LIST, &args![entry]).u32();
        }
    }
    let current: u32 = e.global(CURRENT_SPACE);
    if current != 0 {
        let environment = e.call(SPACE_ENVIRONMENT, &args![current]).u32();
        let audio = audio_instance(e);
        e.call(AUDIO_SET_ENVIRONMENT, &args![audio, environment]);
    } else {
        let audio = audio_instance(e);
        e.call(AUDIO_SET_ENVIRONMENT, &args![audio, 0u32]);
    }
    // Underwater loop.
    if underwater != 0
        && e.call(PARENT_CELL, &args![player]).u32() != 0
        && !is_playing(e, UNDERWATER_LOOP)
    {
        e.with_stack(0xC, |e, temporary| {
            let audio = audio_instance(e);
            let handle = e
                .call(
                    AUDIO_GET_SOUND_HANDLE_BY_NAME,
                    &args![audio, temporary, UNDERWATER_LOOP_NAME, 0x2101u32],
                )
                .u32();
            e.call(SOUND_HANDLE_ASSIGN, &args![UNDERWATER_LOOP, handle]);
            e.call(SOUND_HANDLE_DESTROY, &args![temporary]);
        });
        e.call(SOUND_HANDLE_SET_FLAG_BYTE, &args![UNDERWATER_LOOP, 1u32]);
        e.call(SOUND_HANDLE_PLAY, &args![UNDERWATER_LOOP, 0u32]);
    } else if (underwater == 0 || e.call(PARENT_CELL, &args![player]).u32() == 0)
        && is_playing(e, UNDERWATER_LOOP)
    {
        fade_out_and_release(e, UNDERWATER_LOOP, 300);
    }
    e.call(IMPACT_MIXER_INIT_FOOTSTEPS, &[]);
    e.call(IMPACT_MIXER_UPDATE, &[]);
    if e.global::<u8>(REGION_UPDATE_DISABLED) != 0 {
        return;
    }
    let tes: u32 = e.global(TES);
    let sky = e.call(TES_GET_SKY, &args![tes]).u32();
    let game_hour = if sky != 0 {
        e.call(SKY_GET_GAME_HOUR, &args![sky]).f32()
    } else {
        0.0
    };
    let audio = audio_instance(e);
    e.call(AUDIO_SET_GAME_HOUR, &args![audio, game_hour, 0u32]);
    let audio = audio_instance(e);
    let now = e.call(AUDIO_ELAPSED_TIME, &args![audio]).u32();
    let last: u32 = e.global(LAST_REGION_UPDATE);
    let elapsed = e.call(ABSOLUTE_VALUE, &args![now.wrapping_sub(last)]).i32();
    if elapsed <= 1000 {
        if e.call(LIST_NODE_IS_EMPTY, &args![region_sound_list]).bool()
            && !e.call(MAP_IS_EMPTY, &args![CURRENT_REGION_SOUNDS]).bool()
        {
            drain_region_sound_map(e);
        }
        return;
    }
    if player != 0 {
        fn_0082ea80(e, Ptr::new(player));
    }
    if !e.call(IS_TOP_MENU_ID, &args![0x3f5u32]).bool() {
        let tes: u32 = e.global(TES);
        e.call(TES_METHOD_00455690, &args![tes]);
    }
    let mut fade_in_seconds = setting_value(e, FAS_FADE_START_TIME);
    if is_playing(e, AMBIENT_LOOP) {
        fade_in_seconds = setting_value(e, FAS_FADE_IN_TIME);
    }
    update_ambient_loop(e, player, game_hour, fade_in_seconds);
    if !e.call(LIST_NODE_IS_EMPTY, &args![region_sound_list]).bool()
        && !e.call(IS_TOP_MENU_ID, &args![0x3f5u32]).bool()
    {
        update_region_loops(e, region_sound_list, sky, game_hour);
    }
    let audio = audio_instance(e);
    let now = e.call(AUDIO_ELAPSED_TIME, &args![audio]).u32();
    e.set_global(LAST_REGION_UPDATE, now);
}

/// The cleanup of the quick path: removes every entry of the map of looping
/// region sounds, fading each handle out over 300 ms and deleting it.
fn drain_region_sound_map(e: &mut Engine) {
    e.with_stack(0xC, |e, scratch| {
        let position = scratch.addr();
        let key = scratch.addr() + 4;
        let handle = scratch.addr() + 8;
        let first = e
            .call(MAP_FIRST_POSITION, &args![CURRENT_REGION_SOUNDS])
            .u32();
        e.mem.set_u32(position, first);
        while e.mem.u32(position) != 0 {
            e.call(
                MAP_NEXT,
                &args![CURRENT_REGION_SOUNDS, position, key, handle],
            );
            let removed = e.mem.u32(key);
            e.call(MAP_REMOVE_AT, &args![CURRENT_REGION_SOUNDS, removed]);
            let first = e
                .call(MAP_FIRST_POSITION, &args![CURRENT_REGION_SOUNDS])
                .u32();
            e.mem.set_u32(position, first);
            let sound_handle = e.mem.u32(handle);
            fade_out_and_release(e, sound_handle, 300);
            let sound_handle = e.mem.u32(handle);
            if sound_handle != 0 {
                e.call(SOUND_HANDLE_DELETE, &args![sound_handle, 1u32]);
            }
            e.mem.set_u32(handle, 0);
        }
    });
}

/// Fades out the ambient loop over a time setting times `scale_at` and resets it.
fn fade_out_ambient(e: &mut Engine, scale_at: u32) {
    let seconds = setting_value(e, FAS_FADE_OUT_TIME);
    let milliseconds = truncated_scaled(e, seconds, scale_at);
    fade_out_and_release(e, AMBIENT_LOOP, milliseconds);
    e.call(SOUND_HANDLE_RESET, &args![AMBIENT_LOOP]);
}

/// Starts `form`'s file on the ambient loop and fades it in.
fn start_ambient(e: &mut Engine, form: u32, fade_in_seconds: f32) {
    assign_file_handle(e, AMBIENT_LOOP, form, 0x11011);
    let milliseconds = truncated_scaled(e, fade_in_seconds, MILLISECONDS_PER_SECOND);
    e.call(SOUND_HANDLE_START_FADE, &args![AMBIENT_LOOP, milliseconds]);
}

/// The ambient loop part of the update (phase 4 of `UpdateRegionSounds`).
fn update_ambient_loop(e: &mut Engine, player: u32, game_hour: f32, fade_in_seconds: f32) {
    let current: u32 = e.global(CURRENT_SPACE);
    if current == 0 || player == 0 {
        fade_out_and_release(e, AMBIENT_LOOP, 1000);
        e.call(SOUND_HANDLE_RESET, &args![AMBIENT_LOOP]);
        return;
    }
    if e.call(SPACE_FLAG, &args![current]).bool() {
        // A transition space: its first ambient sound is played whenever
        // the loop is invalid or a change is pending.
        let mut replace = e.global::<u8>(UPDATE_ACOUSTIC_SPACE) != 0;
        if !e.call(SOUND_HANDLE_IS_VALID, &args![AMBIENT_LOOP]).bool() {
            replace = true;
        }
        if replace {
            if e.global::<u8>(UPDATE_ACOUSTIC_SPACE) != 0 {
                e.set_global(NEXT_SPACE, 0u32);
                e.set_global(UPDATE_ACOUSTIC_SPACE, 0u8);
            }
            if is_playing(e, AMBIENT_LOOP) {
                fade_out_ambient(e, HALF_SECOND_SCALE);
            }
            e.set_global(AMBIENT_REPLACED_FLAG, 0u8);
            let sound = e.call(SPACE_SOUND_0, &args![current]).u32();
            if sound != 0 {
                start_ambient(e, sound, fade_in_seconds);
            }
        }
        return;
    }
    let mut replace = e.global::<u8>(UPDATE_ACOUSTIC_SPACE) != 0;
    if !replace {
        replace = match e.global::<u32>(CURRENT_TIME_SLOT) {
            0 => setting_below(e, DAY_TIME, game_hour),
            1 => setting_below(e, DUSK_TIME, game_hour),
            2 => setting_below(e, NIGHT_TIME, game_hour),
            3 => {
                // The hour is after dawn and before night.
                setting_below(e, DAWN_TIME, game_hour) && setting_value(e, NIGHT_TIME) > game_hour
            }
            _ => false,
        };
    }
    if !replace {
        return;
    }
    if e.global::<u8>(UPDATE_ACOUSTIC_SPACE) != 0 {
        e.set_global(NEXT_SPACE, 0u32);
        e.set_global(UPDATE_ACOUSTIC_SPACE, 0u8);
    } else {
        fade_out_ambient(e, MILLISECONDS_PER_SECOND);
    }
    e.set_global(AMBIENT_REPLACED_FLAG, 0u8);
    let (sound, slot) = if !setting_below(e, DAWN_TIME, game_hour) {
        (e.call(SPACE_SOUND_3, &args![current]).u32(), 3u32)
    } else if !setting_below(e, DAY_TIME, game_hour) {
        (e.call(SPACE_SOUND_0, &args![current]).u32(), 0)
    } else if !setting_below(e, DUSK_TIME, game_hour) {
        (e.call(SPACE_SOUND_1, &args![current]).u32(), 1)
    } else if !setting_below(e, NIGHT_TIME, game_hour) {
        (e.call(SPACE_SOUND_2, &args![current]).u32(), 2)
    } else {
        (e.call(SPACE_SOUND_3, &args![current]).u32(), 3)
    };
    e.set_global(CURRENT_TIME_SLOT, slot);
    if sound != 0 {
        start_ambient(e, sound, fade_in_seconds);
    }
}

/// Whether a region sound may play now (the original's "play" flag:
/// true unless a test rules it out). Without a sky nothing rules it out.
/// The weather mask rules it out when the entry has flag bits in its low
/// 16 bits, the sky has a current weather, the weather's byte `0xb` is
/// nonzero and shares no bit with them. The sound's time window
/// (`window_start` to `window_end`, its two time bytes times 0.09375; it may
/// wrap) rules
/// it out when it is at least 0.09375 wide and the game hour is outside it.
fn region_sound_allowed(
    e: &mut Engine,
    entry: Ptr<TESRegionSound>,
    sky: u32,
    game_hour: f32,
    (window_start, window_end): (f32, f32),
) -> bool {
    let mut allowed = true;
    if sky == 0 {
        return allowed;
    }
    let flags = e.get(entry, TESRegionSound::uiFlags);
    if flags & 0xffff != 0 && e.call(WORD_AT_0X10, &args![sky]).u32() != 0 {
        let weather = e.call(WORD_AT_0X10, &args![sky]).u32();
        let mask = e.call(WEATHER_BYTE, &args![weather, 0xbu32]).u8() as u32;
        if flags & 0xffff & mask == 0 && mask != 0 {
            allowed = false;
        }
    }
    let width = e
        .call(FLOAT_ABSOLUTE_VALUE, &args![window_end - window_start])
        .f32() as f64;
    let step: f64 = e.global(TIME_STEP);
    if width >= step
        && ((window_start < window_end && (game_hour < window_start || window_end < game_hour))
            || (window_end < window_start && game_hour < window_start && window_end < game_hour))
    {
        allowed = false;
    }
    allowed
}

/// The region sounds part of the update (phase 4 of `UpdateRegionSounds`).
fn update_region_loops(e: &mut Engine, region_sound_list: u32, sky: u32, game_hour: f32) {
    // Local `BSSoundHandle` the one-shot sounds are built in.
    let local = e.mem.alloc(0xC);
    e.call(SOUND_HANDLE_CONSTRUCT_WITH_ID, &args![local, 0u32]);
    let audio = audio_instance(e);
    let listener = e.call(WORD_AT_0X10, &args![audio]).u32();
    let mut node = region_sound_list;
    while node != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![node]).bool() {
        let entry = Ptr::<TESRegionSound>::new(e.mem.u32(node));
        let sound = Ptr::<TESSound>::new(e.mem.u32(entry.addr()));
        if entry.is_null() || sound.is_null() {
            node = e.call(LIST_NODE_NEXT, &args![node]).u32();
            continue;
        }
        let time_step: f64 = e.global(TIME_STEP);
        let window_start = (fn_0082d6e0(e, sound) as u8 as f64 * time_step) as f32;
        let window_end = (fn_0082d700(e, sound) as u8 as f64 * time_step) as f32;
        let sound_flags = sound_data(e, sound.addr(), SoundData::iFlags);
        let looping = e.call(SOUND_FLAG_BITS, &args![sound_flags]).u32() != 0;
        let allowed = region_sound_allowed(e, entry, sky, game_hour, (window_start, window_end));
        let key = e.call(FORM_ID, &args![sound]).u32();
        let found = e.with_stack(4, |e, out| {
            let found = e
                .call(MAP_LOOKUP, &args![CURRENT_REGION_SOUNDS, key, out])
                .bool();
            (found, e.mem.u32(out.addr()))
        });
        let (tracked, tracked_handle) = found;
        if looping && !tracked && allowed {
            let block = e.call(ALLOCATE, &args![0xCu32]).u32();
            let handle = if block == 0 {
                0
            } else {
                e.call(SOUND_HANDLE_CONSTRUCT, &args![block]).u32()
            };
            assign_file_handle(e, handle, sound.addr(), 0x11011);
            e.call(SOUND_HANDLE_SET_FLAG_BYTE, &args![handle, 1u32]);
            let key = e.call(FORM_ID, &args![sound]).u32();
            e.call(MAP_SET_AT, &args![CURRENT_REGION_SOUNDS, key, handle]);
            let seconds = setting_value(e, REGION_LOOP_FADE_IN_TIME);
            let milliseconds = truncated_scaled(e, seconds, MILLISECONDS_PER_SECOND);
            e.call(SOUND_HANDLE_START_FADE, &args![handle, milliseconds]);
        } else if looping && tracked {
            if is_playing(e, tracked_handle) && !allowed {
                let seconds = setting_value(e, REGION_LOOP_FADE_OUT_TIME);
                let milliseconds = truncated_scaled(e, seconds, MILLISECONDS_PER_SECOND);
                fade_out_and_release(e, tracked_handle, milliseconds);
                let key = e.call(FORM_ID, &args![sound]).u32();
                e.call(MAP_REMOVE_AT, &args![CURRENT_REGION_SOUNDS, key]);
                if tracked_handle != 0 {
                    e.call(SOUND_HANDLE_DELETE, &args![tracked_handle, 1u32]);
                }
            } else if e.call(SOUND_HANDLE_IS_VALID, &args![tracked_handle]).bool()
                && !is_playing(e, tracked_handle)
                && allowed
            {
                let seconds = setting_value(e, REGION_LOOP_FADE_IN_TIME);
                let milliseconds = truncated_scaled(e, seconds, MILLISECONDS_PER_SECOND);
                e.call(
                    SOUND_HANDLE_START_FADE,
                    &args![tracked_handle, milliseconds],
                );
            }
        } else if !looping && allowed {
            let chance_word = e.get(entry, TESRegionSound::uiChance);
            let divisor: f64 = e.global(CHANCE_DIVISOR);
            let chance = (chance_word as f64 / divisor) as f32;
            if e.call(RANDOM_CHANCE, &args![chance]).bool() {
                let flags = sound_data(e, sound.addr(), SoundData::iFlags);
                let file_flags = if flags & 0x40 != 0 {
                    0x11101u32
                } else {
                    0x11102
                };
                assign_file_handle(e, local, sound.addr(), file_flags);
                let player_now: u32 = e.global(PLAYER);
                let position = e.vcall(player_now, PLAYER_POSITION_SLOT, &[]).u32();
                let mut x = e.mem.f32(position);
                let mut y = e.mem.f32(position + 4);
                let mut z = e.mem.f32(position + 8);
                if listener != 0 {
                    let position = e.vcall(listener, LISTENER_POSITION_SLOT, &[]).u32();
                    x = e.mem.f32(position);
                    y = e.mem.f32(position + 4);
                    z = e.mem.f32(position + 8);
                }
                let mut offset_x = (e.call(RANDOM_NUMBER, &[]).u32() % 0x28a + 100) as f32;
                let mut offset_y = (e.call(RANDOM_NUMBER, &[]).u32() % 0x28a + 100) as f32;
                let half: f32 = e.global(HALF);
                let minus_one: f64 = e.global(MINUS_ONE);
                if e.call(RANDOM_CHANCE, &args![half]).bool() {
                    offset_x = (offset_x as f64 * minus_one) as f32;
                }
                if e.call(RANDOM_CHANCE, &args![half]).bool() {
                    offset_y = (offset_y as f64 * minus_one) as f32;
                }
                x = (x as f64 + offset_x as f64) as f32;
                y = (y as f64 + offset_y as f64) as f32;
                let height: f64 = e.global(HEIGHT_OFFSET);
                z = (z as f64 + height) as f32;
                e.call(SOUND_HANDLE_SET_PRIORITY, &args![local, 0xffu32]);
                e.call(SOUND_HANDLE_SET_POSITION, &args![local, x, y, z]);
                e.call(SOUND_HANDLE_PLAY, &args![local, 0u32]);
            }
        }
        node = e.call(LIST_NODE_NEXT, &args![node]).u32();
    }
    // Tracked sounds whose form is no longer in the list are faded out and
    // deleted. The "found" flag is set once and never cleared again, and the
    // list position is not reset between map entries, exactly as the code does.
    e.with_stack(0xC, |e, scratch| {
        let position = scratch.addr();
        let key = scratch.addr() + 4;
        let handle = scratch.addr() + 8;
        let mut found = false;
        let first = e
            .call(MAP_FIRST_POSITION, &args![CURRENT_REGION_SOUNDS])
            .u32();
        e.mem.set_u32(position, first);
        let mut cursor = region_sound_list;
        while e.mem.u32(position) != 0 {
            e.call(
                MAP_NEXT,
                &args![CURRENT_REGION_SOUNDS, position, key, handle],
            );
            let tracked_key = e.mem.u32(key);
            while cursor != 0 && !e.call(LIST_NODE_IS_EMPTY, &args![cursor]).bool() {
                let entry = e.mem.u32(cursor);
                let sound = e.mem.u32(entry);
                if tracked_key == e.call(FORM_ID, &args![sound]).u32() {
                    found = true;
                    break;
                }
                cursor = e.call(LIST_NODE_NEXT, &args![cursor]).u32();
            }
            if !found {
                e.call(MAP_REMOVE_AT, &args![CURRENT_REGION_SOUNDS, tracked_key]);
                let first = e
                    .call(MAP_FIRST_POSITION, &args![CURRENT_REGION_SOUNDS])
                    .u32();
                e.mem.set_u32(position, first);
                let seconds = setting_value(e, REGION_LOOP_FADE_OUT_TIME);
                let milliseconds = truncated_scaled(e, seconds, MILLISECONDS_PER_SECOND);
                let sound_handle = e.mem.u32(handle);
                fade_out_and_release(e, sound_handle, milliseconds);
                let sound_handle = e.mem.u32(handle);
                if sound_handle != 0 {
                    e.call(SOUND_HANDLE_DELETE, &args![sound_handle, 1u32]);
                }
                e.mem.set_u32(handle, 0);
            }
        }
    });
    e.call(SOUND_HANDLE_DESTROY, &args![local]);
    e.mem.free(local);
}

// Translated from 0082ea80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the player's `pClosestAudioMarkerInfo` (Xbox PDB `+0x7ec`, PC
/// `+0x7dc`); no Xbox PDB name for the function.
pub fn fn_0082ea80(e: &mut Engine, this: Ptr<PlayerCharacter>) {
    e.mem.set_u32(this.addr().wrapping_add(0x7dc), 0);
}

// Translated from 0082eaa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The address of the player's `CurrentRegionSoundList` (Xbox PDB `+0x784`,
/// PC `+0x774`); no Xbox PDB name for the function.
pub fn fn_0082eaa0(_e: &mut Engine, this: Ptr<PlayerCharacter>) -> Ptr<BSSimpleList> {
    this.byte_add(0x774).cast()
}

// Translated from 0082eac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The player's `bIsAcousticSpaceTransition` byte (Xbox PDB `+0x609`, PC
/// `+0x5f9`); no Xbox PDB name for the function.
pub fn fn_0082eac0(e: &mut Engine, this: Ptr<PlayerCharacter>) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x5f9))
}

// Translated from 0082eae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Makes `space` the current acoustic space (no Xbox PDB name; it matches a
/// setter of `pCurrentSpace`). A different space fades the ambient loop out
/// over `fASFadeOutTime`, resets the loop handle, passes the new space's
/// environment (`+0x4c` of the space) to the audio unless it is null, stores
/// it and sets `bUpdateAcousticSpace`. The same space does nothing.
pub fn fn_0082eae0(e: &mut Engine, space: u32) {
    if e.global::<u32>(CURRENT_SPACE) == space {
        return;
    }
    let seconds = setting_value(e, FAS_FADE_OUT_TIME);
    let milliseconds = truncated_scaled(e, seconds, MILLISECONDS_PER_SECOND);
    fade_out_and_release(e, AMBIENT_LOOP, milliseconds);
    e.call(SOUND_HANDLE_RESET, &args![AMBIENT_LOOP]);
    if space != 0 {
        let environment = e.call(SPACE_ENVIRONMENT, &args![space]).u32();
        let audio = audio_instance(e);
        e.call(AUDIO_SET_ENVIRONMENT, &args![audio, environment]);
    }
    e.set_global(CURRENT_SPACE, space);
    e.set_global(UPDATE_ACOUSTIC_SPACE, 1u8);
}

// Translated from 0082eb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutAudio::ClearAcousticSpace` (Xbox PDB): unless the space is locked
/// (`bLockAcousticSpace`), and when there is a current space and `space` is
/// null or equal to it, fades the ambient loop out over `fASFadeOutTime`,
/// resets the loop handle, clears the current space, sets
/// `bUpdateAcousticSpace` and tells the audio the environment is 0.
pub fn fallout_audio_clear_acoustic_space(e: &mut Engine, space: u32) {
    if e.global::<u8>(LOCK_ACOUSTIC_SPACE) != 0 {
        return;
    }
    let current: u32 = e.global(CURRENT_SPACE);
    if (space == 0 || space == current) && current != 0 {
        let seconds = setting_value(e, FAS_FADE_OUT_TIME);
        let milliseconds = truncated_scaled(e, seconds, MILLISECONDS_PER_SECOND);
        fade_out_and_release(e, AMBIENT_LOOP, milliseconds);
        e.call(SOUND_HANDLE_RESET, &args![AMBIENT_LOOP]);
        e.set_global(CURRENT_SPACE, 0u32);
        e.set_global(UPDATE_ACOUSTIC_SPACE, 1u8);
        let audio = audio_instance(e);
        e.call(AUDIO_SET_ENVIRONMENT, &args![audio, 0u32]);
    }
}

// Translated from 0082ec10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutAudio::GetNoActivationSound` (Xbox PDB): the `"UIActivateNothing"`
/// sound form, looked up once and kept in `pNoActivationSound`.
pub fn fallout_audio_get_no_activation_sound(e: &mut Engine) -> u32 {
    if e.global::<u32>(NO_ACTIVATION_SOUND) == 0 {
        let handler: u32 = e.global(DATA_HANDLER);
        let sound = e
            .call(GET_SOUND, &args![handler, UI_ACTIVATE_NOTHING])
            .u32();
        e.set_global(NO_ACTIVATION_SOUND, sound);
    }
    e.global(NO_ACTIVATION_SOUND)
}

// Translated from 0082ec40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutAudio::SaveGame` (Xbox PDB): saves the form id of
/// `pNoActivationSound` (flag 0) into the save buffer.
pub fn fallout_audio_save_game(e: &mut Engine, buffer: Ptr) {
    let sound: u32 = e.global(NO_ACTIVATION_SOUND);
    e.call(SAVE_FORM_ID, &args![buffer, sound, 0u32]);
}

// Translated from 0082ec60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutAudio::LoadGame` (Xbox PDB): reads a form id from the load buffer,
/// looks the form up and stores it as `pNoActivationSound` when the form
/// can be cast to the sound class (null otherwise).
pub fn fallout_audio_load_game(e: &mut Engine, buffer: Ptr) {
    let id = e.call(LOAD_FORM_ID, &args![buffer]).u32();
    let form = e.call(LOOKUP_FORM, &args![id]).u32();
    let sound = e
        .call(
            DYNAMIC_CAST,
            &args![
                form,
                0u32,
                FORM_TYPE_DESCRIPTOR,
                SOUND_TYPE_DESCRIPTOR,
                0u32
            ],
        )
        .u32();
    e.set_global(NO_ACTIVATION_SOUND, sound);
}

// Translated from 0082eca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether a point is within hearing distance of the listener: false for a
/// null `object` or no listener; otherwise the object's byte at `+0x45`
/// times 100 times `scale` is the reach, and the answer is true when none of
/// the three coordinate differences to the listener position is larger than it
/// (no Xbox PDB name).
pub fn fn_0082eca0(e: &mut Engine, object: u32, x: f32, y: f32, z: f32, scale: f32) -> bool {
    let audio = audio_instance(e);
    let listener = e.call(WORD_AT_0X10, &args![audio]).u32();
    if object == 0 || listener == 0 {
        return false;
    }
    let position = e.vcall(listener, LISTENER_POSITION_SLOT, &[]).u32();
    let listener_x = e.mem.f32(position);
    let listener_y = e.mem.f32(position + 4);
    let listener_z = e.mem.f32(position + 8);
    let byte = e.call(SOUND_MAX_DISTANCE, &args![object]).u8();
    let hundred: f64 = e.global(ONE_HUNDRED);
    let reach = ((byte as f64 * hundred) * scale as f64) as f32;
    let differences = [x - listener_x, y - listener_y, z - listener_z];
    for difference in differences {
        let distance = e.call(FLOAT_ABSOLUTE_VALUE, &args![difference]).f32();
        if reach < distance {
            return false;
        }
    }
    true
}

// Translation incomplete: 0082ed80 - C++ exception-unwind frame (FS:[0]) omitted; the normal path is translated.
// Translated from 0082ed80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutAudio::MysteriousStrangerAppears` (Xbox PDB): plays the
/// `"MUSMysteriousStrangerA01"` sound (flags 0x210101) and precaches
/// `"MUSMysteriousStrangerA02"` (flags 0x20200101).
pub fn fallout_audio_mysterious_stranger_appears(e: &mut Engine) {
    e.with_stack(0xC, |e, handle| {
        let audio = audio_instance(e);
        e.call(
            AUDIO_GET_SOUND_HANDLE_BY_NAME,
            &args![audio, handle, MYSTERIOUS_STRANGER_A01, 0x0021_0101u32],
        );
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        let audio = audio_instance(e);
        e.call(
            AUDIO_PRECACHE,
            &args![audio, MYSTERIOUS_STRANGER_A02, 0x2020_0101u32],
        );
        e.call(SOUND_HANDLE_DESTROY, &args![handle]);
    });
}

// Translation incomplete: 0082ee10 - C++ exception-unwind frame (FS:[0]) omitted; the normal path is translated.
// Translated from 0082ee10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Plays the `"MUSMysteriousStrangerA02"` sound (flags 0x200101); no Xbox
/// PDB name.
pub fn fn_0082ee10(e: &mut Engine) {
    e.with_stack(0xC, |e, handle| {
        let audio = audio_instance(e);
        e.call(
            AUDIO_GET_SOUND_HANDLE_BY_NAME,
            &args![audio, handle, MYSTERIOUS_STRANGER_A02, 0x0020_0101u32],
        );
        e.call(SOUND_HANDLE_PLAY, &args![handle, 0u32]);
        e.call(SOUND_HANDLE_DESTROY, &args![handle]);
    });
}

// Translated from 0082ee80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of `NiTPointerMap<unsigned int, BSSoundHandle*>`: builds the
/// base map with `hash_size` buckets, then installs the derived vtable.
pub fn fn_0082ee80(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    fn_0082eee0(e, this, hash_size);
    e.mem.set_u32(this.addr(), SOUND_HANDLE_MAP_VTABLE);
    this
}

// Translated from 0082eeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTPointerMap<unsigned int, BSSoundHandle*>::_scalar_deleting_destructor_`
/// (Xbox PDB): destroys the map, then frees it when bit 0 of `flags` is set.
pub fn fn_0082eeb0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    fn_0082ef50(e, this);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

// Translated from 0082eee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base map constructor (`NiTMapBase`): installs the base vtable, stores the
/// bucket count, clears the item count, allocates `hash_size` words for the
/// buckets and zero-fills them.
pub fn fn_0082eee0(e: &mut Engine, this: Ptr<NiTPointerMap>, hash_size: u32) -> Ptr<NiTPointerMap> {
    e.mem.set_u32(this.addr(), SOUND_HANDLE_MAP_BASE_VTABLE);
    e.set(this, NiTPointerMap::m_uiHashSize, hash_size);
    e.set(this, NiTPointerMap::m_uiCount, 0);
    let table = e
        .call(MEMORY_ALLOCATE, &args![hash_size.wrapping_shl(2)])
        .u32();
    e.set(this, NiTPointerMap::m_ppkHashTable, table);
    let table: u32 = e.get(this, NiTPointerMap::m_ppkHashTable);
    let size = e.get(this, NiTPointerMap::m_uiHashSize).wrapping_shl(2);
    e.call(MEMSET, &args![table, 0u32, size]);
    this
}

// Translation incomplete: 0082ef50 - C++ exception-unwind frame (FS:[0]) omitted; the normal path is translated.
// Translated from 0082ef50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destructor of `NiTPointerMap<unsigned int, BSSoundHandle*>` (the
/// decompiler's `ctype<char>` name is a false library match): installs the
/// derived vtable, clears the map and runs the base destructor.
pub fn fn_0082ef50(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), SOUND_HANDLE_MAP_VTABLE);
    e.call(MAP_CLEAR, &args![this]);
    fn_0082efb0(e, this);
}

// Translated from 0082efb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Base map destructor (`NiTMapBase`): installs the base vtable, clears the
/// map and frees the bucket array.
pub fn fn_0082efb0(e: &mut Engine, this: Ptr<NiTPointerMap>) {
    e.mem.set_u32(this.addr(), SOUND_HANDLE_MAP_BASE_VTABLE);
    e.call(MAP_CLEAR, &args![this]);
    let table = e.get(this, NiTPointerMap::m_ppkHashTable);
    e.call(MEMORY_FREE, &args![table]);
}

// Translated from 0082efe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `NiTMapBase<NiTPointerAllocator<unsigned int>, unsigned int,
/// BSSoundHandle*>::_scalar_deleting_destructor_` (Xbox PDB): runs the base
/// destructor, then frees the object when bit 0 of `flags` is set.
pub fn fn_0082efe0(e: &mut Engine, this: Ptr<NiTPointerMap>, flags: u32) -> Ptr<NiTPointerMap> {
    fn_0082efb0(e, this);
    if flags & 1 != 0 {
        e.call(FREE_OBJECT, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082d280, fallout_audio_resolve_sound_name(u32, u32, Ptr, Ptr) -> bool),
        entry!(
            0x0082d400,
            fallout_audio_apply_settings_from_master_file(
                Ptr<BSSoundHandle>,
                Ptr<TESSound>,
                Ptr,
            ) -> u32
        ),
        entry!(0x0082d660, fn_0082d660(Ptr<TESSound>) -> u8),
        entry!(0x0082d680, fn_0082d680(Ptr<TESSound>) -> u16),
        entry!(0x0082d6a0, fn_0082d6a0(Ptr<TESSound>, u32) -> u16),
        entry!(0x0082d6c0, fn_0082d6c0(Ptr<TESSound>) -> u16),
        entry!(0x0082d6e0, fn_0082d6e0(Ptr<TESSound>) -> u32),
        entry!(0x0082d700, fn_0082d700(Ptr<TESSound>) -> u32),
        entry!(0x0082d720, fn_0082d720(Ptr<TESSound>) -> u32),
        entry!(0x0082d740, fn_0082d740(u32, Ptr, Ptr) -> bool),
        entry!(0x0082d7c0, fallout_audio_update_region_sounds()),
        entry!(0x0082ea80, fn_0082ea80(Ptr<PlayerCharacter>)),
        entry!(
            0x0082eaa0,
            fn_0082eaa0(Ptr<PlayerCharacter>) -> Ptr<BSSimpleList>
        ),
        entry!(0x0082eac0, fn_0082eac0(Ptr<PlayerCharacter>) -> u8),
        entry!(0x0082eae0, fn_0082eae0(u32)),
        entry!(0x0082eb70, fallout_audio_clear_acoustic_space(u32)),
        entry!(0x0082ec10, fallout_audio_get_no_activation_sound() -> u32),
        entry!(0x0082ec40, fallout_audio_save_game(Ptr)),
        entry!(0x0082ec60, fallout_audio_load_game(Ptr)),
        entry!(0x0082eca0, fn_0082eca0(u32, f32, f32, f32, f32) -> bool),
        entry!(0x0082ed80, fallout_audio_mysterious_stranger_appears()),
        entry!(0x0082ee10, fn_0082ee10()),
        entry!(
            0x0082ee80,
            fn_0082ee80(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x0082eeb0,
            fn_0082eeb0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(
            0x0082eee0,
            fn_0082eee0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
        entry!(0x0082ef50, fn_0082ef50(Ptr<NiTPointerMap>)),
        entry!(0x0082efb0, fn_0082efb0(Ptr<NiTPointerMap>)),
        entry!(
            0x0082efe0,
            fn_0082efe0(Ptr<NiTPointerMap>, u32) -> Ptr<NiTPointerMap>
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::collections::VecDeque;
    use std::rc::Rc;

    /// A byte the doubles read the "listener underwater" answer from.
    const TEST_UNDERWATER: u32 = 0x011d_d300;
    /// The audio clock the elapsed-time double returns.
    const TEST_NOW: u32 = 0x011d_d304;
    /// The answer of `Interface::IsTopMenuID`.
    const TEST_TOP_MENU: u32 = 0x011d_d308;
    /// What `TESDataHandler::GetSound_ov2` answers for any name.
    const TEST_SOUND_FORM: u32 = 0x011d_d30c;
    /// The data handler double's loading byte and head-node word.
    const TEST_HANDLER_BUSY: u32 = 0x011d_d310;
    const TEST_HANDLER_HEAD: u32 = 0x011d_d314;
    /// The audio singleton the double of `QInstance` returns (mapped).
    const AUDIO: u32 = 0x5000_0000;
    /// The player's vtable position function and the listener's.
    const PLAYER_POSITION_FUNCTION: u32 = 0x00ff_0001;
    const LISTENER_POSITION_FUNCTION: u32 = 0x00ff_0002;

    struct World {
        e: Engine,
        /// The fake `NiTPointerMap<u32, BSSoundHandle*>`: (key, handle) pairs.
        map: Rc<RefCell<Vec<(u32, u32)>>>,
        /// Answers of the random-chance double, front first (false when empty).
        chances: Rc<RefCell<VecDeque<bool>>>,
        /// Answers of the random-number double, front first (0 when empty).
        randoms: Rc<RefCell<VecDeque<u32>>>,
    }

    fn word(e: &Engine, address: u32) -> u32 {
        e.mem.u32(address)
    }

    impl World {
        fn new() -> World {
            let mut e = Engine::new();
            for page in [
                0x0101_6000,
                0x0101_7000,
                0x0101_a000,
                0x0102_0000,
                0x0102_3000,
                0x0103_0000,
                0x0103_4000,
                0x0107_c000,
                0x011c_3000,
                0x011d_c000,
                0x011d_d000,
                0x011d_e000,
                0x011f_6000,
                AUDIO,
            ] {
                e.map(page, 0x1000);
            }
            e.set_global(MILLISECONDS_PER_SECOND, 1000.0f64);
            e.set_global(HALF_SECOND_SCALE, 500.0f64);
            e.set_global(ONE_HUNDRED, 100.0f64);
            e.set_global(FIVE, 5.0f64);
            e.set_global(TIME_STEP, 0.09375f64);
            e.set_global(CHANCE_DIVISOR, 20000000.0f64);
            e.set_global(HALF, 0.5f32);
            e.set_global(MINUS_ONE, -1.0f64);
            e.set_global(HEIGHT_OFFSET, 256.0f64);
            // The settings keep their value at +4.
            for (setting, value) in [
                (FAS_FADE_START_TIME, 3.0f32),
                (FAS_FADE_OUT_TIME, 5.0),
                (REGION_LOOP_FADE_IN_TIME, 0.6),
                (FAS_FADE_IN_TIME, 2.0),
                (REGION_LOOP_FADE_OUT_TIME, 1.0),
                (DAWN_TIME, 6.0),
                (DAY_TIME, 10.0),
                (DUSK_TIME, 18.0),
                (NIGHT_TIME, 22.0),
            ] {
                e.set_global(setting + 4, value);
            }
            // The ambient loop starts out holding no sound.
            e.set_global(AMBIENT_LOOP, u32::MAX);
            e.set_global(UNDERWATER_LOOP, u32::MAX);
            e.call_log = Some(vec![]);

            let map: Rc<RefCell<Vec<(u32, u32)>>> = Rc::default();
            let chances: Rc<RefCell<VecDeque<bool>>> = Rc::default();
            let randoms: Rc<RefCell<VecDeque<u32>>> = Rc::default();

            // Callees whose effect the tests do not look at (only their calls).
            for address in [
                AUDIO_SET_GAME_HOUR,
                AUDIO_SET_ENVIRONMENT,
                AUDIO_PRECACHE,
                AUDIO_PRECACHE_FORM,
                SOUND_HANDLE_PLAY,
                SOUND_HANDLE_SET_POSITION,
                SOUND_HANDLE_SET_PRIORITY,
                SOUND_HANDLE_SET_STATIC_ATTENUATION,
                SOUND_HANDLE_SET_MIN_MAX,
                SOUND_HANDLE_SET_CURVE,
                SOUND_HANDLE_SET_REVERB,
                SOUND_HANDLE_SET_TIME_BYTES,
                SOUND_HANDLE_SET_PITCH,
                SOUND_HANDLE_FADE_OUT_AND_RELEASE,
                SOUND_HANDLE_START_FADE,
                SOUND_HANDLE_DESTROY,
                SOUND_HANDLE_DELETE,
                SOUND_HANDLE_SET_FLAG_BYTE,
                SOUND_HANDLE_CONSTRUCT_WITH_ID,
                IMPACT_MIXER_INIT_FOOTSTEPS,
                IMPACT_MIXER_UPDATE,
                TES_METHOD_00455690,
                LOG_MESSAGE,
                COPY_STRING,
                MEMORY_FREE,
                FREE_OBJECT,
                MAP_CLEAR,
                SAVE_FORM_ID,
            ] {
                e.register(address, |_, _| Ret::default());
            }
            e.register(AUDIO_INSTANCE, |_, _| AUDIO.into_ret());
            e.register(SETTING_VALUE, |_, a| (a[0] + 4).into_ret());
            e.register(GET_SOUND, |e, _| word(e, TEST_SOUND_FORM).into_ret());
            e.register(SOUND_FILE_NAME, |e, a| word(e, a[0] + 0x30).into_ret());
            e.register(COPY_SOUND_DATA, |e, a| {
                for i in 0..9 {
                    let value = e.mem.u32(a[0] + 0x44 + 4 * i);
                    e.mem.set_u32(a[1] + 4 * i, value);
                }
                a[1].into_ret()
            });
            e.register(SOUND_FLAG_BITS, |_, a| {
                let bits = if a[0] & 0x10 != 0 {
                    0x10
                } else if a[0] & 0x200 != 0 {
                    0x0200_0000
                } else if a[0] & 0x400 != 0 {
                    0x0400_0000
                } else {
                    0
                };
                u32::into_ret(bits)
            });
            e.register(DATA_HANDLER_FLAG, |e, _| {
                e.mem.u8(TEST_HANDLER_BUSY).into_ret()
            });
            e.register(DATA_HANDLER_HEAD_NODE, |e, _| {
                word(e, TEST_HANDLER_HEAD).into_ret()
            });
            e.register(SOUND_MAX_DISTANCE, |e, a| e.mem.u8(a[0] + 0x45).into_ret());
            e.register(WORD_AT_0X10, |e, a| word(e, a[0] + 0x10).into_ret());
            e.register(FLOAT_ABSOLUTE_VALUE, |_, a| {
                f32::from_bits(a[0]).abs().into_ret()
            });
            e.register(PARENT_CELL, |e, a| word(e, a[0] + 0x40).into_ret());
            e.register(CELL_GET_ACOUSTIC_SPACE, |e, a| {
                word(e, a[0] + 0x100).into_ret()
            });
            e.register(SPACE_FLAG, |e, a| e.mem.u8(a[0] + 0x30).into_ret());
            e.register(SPACE_ENVIRONMENT, |e, a| word(e, a[0] + 0x4c).into_ret());
            e.register(SPACE_REGION_OWNER, |e, a| word(e, a[0] + 0x48).into_ret());
            e.register(REGION_DATA_LIST, |e, a| word(e, a[0] + 0x18).into_ret());
            e.register(REGION_DATA_LIST_FIND, |e, a| {
                assert_eq!(a[1], 7);
                word(e, a[0] + 0x20).into_ret()
            });
            e.register(REGION_DATA_SOUND_LIST, |_, a| (a[0] + 0xc).into_ret());
            e.register(SPACE_SOUND_0, |e, a| word(e, a[0] + 0x34).into_ret());
            e.register(SPACE_SOUND_1, |e, a| word(e, a[0] + 0x38).into_ret());
            e.register(SPACE_SOUND_2, |e, a| word(e, a[0] + 0x3c).into_ret());
            e.register(FORM_ID, |e, a| word(e, a[0] + 0xc).into_ret());
            e.register(LIST_NODE_IS_EMPTY, |e, a| {
                (word(e, a[0]) == 0 && word(e, a[0] + 4) == 0).into_ret()
            });
            e.register(LIST_NODE_NEXT, |e, a| word(e, a[0] + 4).into_ret());
            e.register(TES_GET_SKY, |e, a| word(e, a[0] + 0x68).into_ret());
            e.register(SKY_GET_GAME_HOUR, |e, a| e.mem.f32(a[0] + 0xec).into_ret());
            e.register(WEATHER_BYTE, |e, a| e.mem.u8(a[0] + a[1] + 0xe0).into_ret());
            e.register(AUDIO_LISTENER_UNDERWATER, |e, _| {
                e.mem.u8(TEST_UNDERWATER).into_ret()
            });
            e.register(AUDIO_ELAPSED_TIME, |e, _| word(e, TEST_NOW).into_ret());
            e.register(ABSOLUTE_VALUE, |_, a| (a[0] as i32).abs().into_ret());
            e.register(IS_TOP_MENU_ID, |e, a| {
                assert_eq!(a[0], 0x3f5);
                e.mem.u8(TEST_TOP_MENU).into_ret()
            });
            // Sound handles: "playing" is state 1 at +8, "valid" is id != -1.
            e.register(SOUND_HANDLE_IS_PLAYING, |e, a| {
                (word(e, a[0] + 8) == 1).into_ret()
            });
            e.register(SOUND_HANDLE_IS_VALID, |e, a| {
                (word(e, a[0]) != u32::MAX).into_ret()
            });
            e.register(SOUND_HANDLE_ASSIGN, |e, a| {
                for i in 0..3 {
                    let value = e.mem.u32(a[1] + 4 * i);
                    e.mem.set_u32(a[0] + 4 * i, value);
                }
                a[0].into_ret()
            });
            e.register(SOUND_HANDLE_RESET, |e, a| {
                e.mem.set_u32(a[0], u32::MAX);
                e.mem.set_u32(a[0] + 4, 0);
                e.mem.set_u32(a[0] + 8, 0);
                ().into_ret()
            });
            e.register(SOUND_HANDLE_CONSTRUCT, |e, a| {
                e.mem.set_u32(a[0], u32::MAX);
                a[0].into_ret()
            });
            e.register(ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
            e.register(AUDIO_GET_SOUND_HANDLE_BY_NAME, |e, a| {
                e.mem.set_u32(a[1], 0x4321);
                a[1].into_ret()
            });
            e.register(AUDIO_GET_SOUND_HANDLE_BY_FILENAME, |e, a| {
                e.mem.set_u32(a[1], 0x1234);
                e.mem.set_u32(a[1] + 8, 1);
                a[1].into_ret()
            });
            e.register_double(RANDOM_CHANCE, {
                let chances = chances.clone();
                move |_, _| chances.borrow_mut().pop_front().unwrap_or(false).into_ret()
            });
            e.register_double(RANDOM_NUMBER, {
                let randoms = randoms.clone();
                move |_, _| randoms.borrow_mut().pop_front().unwrap_or(0).into_ret()
            });
            // The fake map. A position is the 1-based index of an entry, 0 at the end.
            e.register_double(MAP_FIRST_POSITION, {
                let map = map.clone();
                move |_, _| u32::from(!map.borrow().is_empty()).into_ret()
            });
            e.register_double(MAP_NEXT, {
                let map = map.clone();
                move |e, a| {
                    let position = e.mem.u32(a[1]) as usize;
                    let (key, handle) = map.borrow()[position - 1];
                    e.mem.set_u32(a[2], key);
                    e.mem.set_u32(a[3], handle);
                    let next = if position < map.borrow().len() {
                        position + 1
                    } else {
                        0
                    };
                    e.mem.set_u32(a[1], next as u32);
                    ().into_ret()
                }
            });
            e.register_double(MAP_REMOVE_AT, {
                let map = map.clone();
                move |_, a| {
                    map.borrow_mut().retain(|(key, _)| *key != a[1]);
                    ().into_ret()
                }
            });
            e.register_double(MAP_SET_AT, {
                let map = map.clone();
                move |_, a| {
                    map.borrow_mut().push((a[1], a[2]));
                    ().into_ret()
                }
            });
            e.register_double(MAP_LOOKUP, {
                let map = map.clone();
                move |e, a| {
                    let found = map.borrow().iter().find(|(key, _)| *key == a[1]).copied();
                    if let Some((_, handle)) = found {
                        e.mem.set_u32(a[2], handle);
                    }
                    found.is_some().into_ret()
                }
            });
            e.register_double(MAP_IS_EMPTY, {
                let map = map.clone();
                move |_, _| map.borrow().is_empty().into_ret()
            });
            World {
                e,
                map,
                chances,
                randoms,
            }
        }

        /// The argument words of every call to `address`.
        fn calls(&self, address: u32) -> Vec<Vec<u32>> {
            self.e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == address)
                .map(|(_, args)| args.clone())
                .collect()
        }

        fn object(&mut self) -> u32 {
            self.e.mem.alloc(0x1000)
        }

        /// A sound form: form id, `SOUND_DATA` flags, time constraint bytes
        /// (high, low) and the `iMax` byte. The file name word is
        /// `0x7000_0000 + form_id`.
        fn sound(&mut self, form_id: u32, flags: u32, time: (u8, u8), max: u8) -> u32 {
            let sound = self.object();
            self.e.mem.set_u32(sound + 0xc, form_id);
            self.e.mem.set_u32(sound + 0x30, 0x7000_0000 + form_id);
            self.e.mem.set_u32(sound + 0x48, flags);
            self.e
                .mem
                .set_u16(sound + 0x4e, (time.0 as u16) << 8 | time.1 as u16);
            self.e.mem.set_u8(sound + 0x45, max);
            sound
        }

        /// A `TESRegionSound`.
        fn region_sound(&mut self, sound: u32, flags: u32, chance: u32) -> u32 {
            let entry = self.e.mem.alloc(0xc);
            self.e.mem.set_u32(entry, sound);
            self.e.mem.set_u32(entry + 4, flags);
            self.e.mem.set_u32(entry + 8, chance);
            entry
        }

        /// A `BSSimpleList` of the items; returns its head node.
        fn list(&mut self, items: &[u32]) -> u32 {
            let head = self.e.mem.alloc(8);
            let mut node = head;
            for (index, item) in items.iter().enumerate() {
                self.e.mem.set_u32(node, *item);
                if index + 1 < items.len() {
                    let next = self.e.mem.alloc(8);
                    self.e.mem.set_u32(node + 4, next);
                    node = next;
                }
            }
            head
        }

        /// Three floats in memory.
        fn point(&mut self, x: f32, y: f32, z: f32) -> u32 {
            let point = self.e.mem.alloc(0xc);
            for (i, value) in [x, y, z].into_iter().enumerate() {
                self.e.mem.set_f32(point + 4 * i as u32, value);
            }
            point
        }

        /// An object whose vtable slot `slot` is `function`.
        fn object_with_slot(&mut self, slot: u32, function: u32) -> u32 {
            let object = self.object();
            let vtable = self.e.mem.alloc(0x400);
            self.e.mem.set_u32(vtable + slot, function);
            self.e.mem.set_u32(object, vtable);
            object
        }

        /// The player, with the region sound list `items` embedded at
        /// `+0x774` and a position function (slot `0x1f4`) answering
        /// (100, 200, 300).
        fn player(&mut self, items: &[u32]) -> u32 {
            let player = self.object_with_slot(0x1f4, PLAYER_POSITION_FUNCTION);
            let list = self.list(items);
            let (item, next) = (word(&self.e, list), word(&self.e, list + 4));
            self.e.mem.set_u32(player + 0x774, item);
            self.e.mem.set_u32(player + 0x778, next);
            let position = self.point(100.0, 200.0, 300.0);
            self.e
                .register_double(PLAYER_POSITION_FUNCTION, move |_, _| position.into_ret());
            self.e.set_global(PLAYER, player);
            player
        }

        /// A sky with the given game hour whose current weather has the
        /// given byte `0xb`; installs it in a fresh `TES`.
        fn sky(&mut self, game_hour: f32, weather_mask: u8) -> u32 {
            let sky = self.object();
            let weather = self.object();
            self.e.mem.set_f32(sky + 0xec, game_hour);
            self.e.mem.set_u32(sky + 0x10, weather);
            self.e.mem.set_u8(weather + 0xe0 + 0xb, weather_mask);
            let tes = self.object();
            self.e.mem.set_u32(tes + 0x68, sky);
            self.e.set_global(TES, tes);
            sky
        }

        /// Makes the audio singleton's listener one at (10, 20, 30).
        fn listener(&mut self) {
            let listener =
                self.object_with_slot(LISTENER_POSITION_SLOT, LISTENER_POSITION_FUNCTION);
            let position = self.point(10.0, 20.0, 30.0);
            self.e
                .register_double(LISTENER_POSITION_FUNCTION, move |_, _| position.into_ret());
            self.e.mem.set_u32(AUDIO + 0x10, listener);
        }

        /// An acoustic space with the flag byte and the four ambient sounds.
        fn space(&mut self, flag: u8, sounds: [u32; 4]) -> u32 {
            let space = self.object();
            self.e.mem.set_u8(space + 0x30, flag);
            self.e.mem.set_u32(space + 0x4c, 0x77);
            for (i, sound) in sounds.into_iter().enumerate() {
                self.e.mem.set_u32(space + 0x34 + 4 * i as u32, sound);
            }
            space
        }

        fn set_playing(&mut self, handle: u32) {
            self.e.mem.set_u32(handle, 0x99);
            self.e.mem.set_u32(handle + 8, 1);
        }

        /// The state of a full region update: the player with the region
        /// sounds, a sky at the hour whose weather byte is `weather`, no
        /// current space, the no-activation sound known, and the audio clock
        /// 2000 ms past the last update.
        fn region_setup(&mut self, items: &[u32], hour: f32, weather: u8) -> u32 {
            let player = self.player(items);
            self.e.set_global(NO_ACTIVATION_SOUND, 1u32);
            self.sky(hour, weather);
            self.e.set_global(LAST_REGION_UPDATE, 1000u32);
            self.e.set_global(TEST_NOW, 3000u32);
            player
        }

        /// A looping region sound (its flags map to a loop bit) and its list entry.
        fn looping_entry(&mut self, form_id: u32, entry_flags: u32, time: (u8, u8)) -> (u32, u32) {
            let sound = self.sound(form_id, 0x10, time, 0);
            let entry = self.region_sound(sound, entry_flags, 0);
            (sound, entry)
        }
    }

    // ---- resolve_sound_name (0082d280) ----

    #[test]
    fn resolve_sound_name_null_name_is_false() {
        let mut w = World::new();
        assert!(!w.e.call(0x0082d280, &args![0u32, 0u32, 0u32, 0u32]).bool());
        assert!(w.calls(GET_SOUND).is_empty());
    }

    #[test]
    fn resolve_sound_name_merges_the_sound_flags() {
        let mut w = World::new();
        let sound = w.sound(0x44, 0x40 | 1 | 0x800 | 0x10, (0, 0), 0);
        w.e.set_global(TEST_SOUND_FORM, sound);
        let buffer = w.e.mem.alloc(0x104);
        let flags = w.e.mem.alloc(4);
        let out = w.e.mem.alloc(4);
        w.e.mem.set_u32(flags, 2);
        assert!(w
            .e
            .call(0x0082d280, &args![0x1234u32, buffer, flags, out])
            .bool());
        // 0x40 sets bit 0 and clears bit 1; bit 0 of the data adds 0x40, 0x800
        // adds 0x8000000, and the mapped 0x10 is or-ed in.
        assert_eq!(word(&w.e, flags), 1 | 0x40 | 0x0800_0000 | 0x10);
        assert_eq!(word(&w.e, out), sound);
        assert_eq!(w.calls(GET_SOUND), vec![vec![0, 0x1234]]);
        assert_eq!(w.calls(COPY_STRING), vec![vec![buffer, 0x104, 0x7000_0044]]);
        assert!(w.calls(LOG_MESSAGE).is_empty());
    }

    #[test]
    fn resolve_sound_name_defaults_to_the_second_flag_bit() {
        let mut w = World::new();
        let sound = w.sound(0x45, 0, (0, 0), 0);
        w.e.set_global(TEST_SOUND_FORM, sound);
        let buffer = w.e.mem.alloc(0x104);
        let flags = w.e.mem.alloc(4);
        let out = w.e.mem.alloc(4);
        assert!(w
            .e
            .call(0x0082d280, &args![7u32, buffer, flags, out])
            .bool());
        assert_eq!(word(&w.e, flags), 2);
        // Either bit already set is kept.
        w.e.mem.set_u32(flags, 1);
        w.e.call(0x0082d280, &args![7u32, buffer, flags, out]);
        assert_eq!(word(&w.e, flags), 1);
    }

    #[test]
    fn resolve_sound_name_logs_a_missing_sound_once_loaded() {
        for (busy, head, logged) in [(0u8, 5u32, true), (1, 5, false), (0, 0, false)] {
            let mut w = World::new();
            w.e.set_global(DATA_HANDLER, 0x1111u32);
            w.e.mem.set_u8(TEST_HANDLER_BUSY, busy);
            w.e.set_global(TEST_HANDLER_HEAD, head);
            let flags = w.e.mem.alloc(4);
            let out = w.e.mem.alloc(4);
            // No sound form: the lookup answers 0.
            assert!(!w
                .e
                .call(0x0082d280, &args![0x2222u32, 0x3333u32, flags, out])
                .bool());
            assert_eq!(
                w.calls(LOG_MESSAGE),
                if logged {
                    vec![vec![SOUND_NOT_FOUND_MESSAGE, 0x2222]]
                } else {
                    vec![]
                }
            );
            assert_eq!(word(&w.e, out), 0);
        }
    }

    #[test]
    fn resolve_sound_name_without_a_buffer_fails() {
        let mut w = World::new();
        let sound = w.sound(0x46, 0, (0, 0), 0);
        w.e.set_global(TEST_SOUND_FORM, sound);
        let flags = w.e.mem.alloc(4);
        let out = w.e.mem.alloc(4);
        assert!(!w.e.call(0x0082d280, &args![9u32, 0u32, flags, out]).bool());
        assert!(w.calls(COPY_STRING).is_empty());
    }

    // ---- apply_settings_from_master_file (0082d400) ----

    fn apply_world() -> (World, u32, u32) {
        let mut w = World::new();
        let sound = w.sound(0x50, 1 | 0x800 | 0x1000 | 0x2000 | 0x10, (3, 4), 9);
        w.e.mem.set_u8(sound + 0x44, 2);
        w.e.mem.set_u16(sound + 0x4c, 0x1234);
        for i in 0..5 {
            w.e.mem.set_u16(sound + 0x50 + 2 * i, 10 + i as u16);
        }
        w.e.mem.set_u16(sound + 0x5a, 0x55);
        w.e.mem.set_u32(sound + 0x5c, 0x1ff);
        w.e.mem.set_u8(sound + 0x46, 0x66);
        let handle = w.e.mem.alloc(0xc);
        (w, sound, handle)
    }

    #[test]
    fn apply_settings_copies_every_value_to_the_handle() {
        let (mut w, sound, handle) = apply_world();
        let extra = w.e.mem.alloc(4);
        let result = w.e.call(0x0082d400, &args![handle, sound, extra]).u32();
        assert_eq!(result, 0x40 | 0x0800_0000 | 0x10);
        assert_eq!(word(&w.e, extra), 3);
        assert_eq!(
            w.calls(SOUND_HANDLE_SET_STATIC_ATTENUATION),
            vec![vec![handle, 0x1234]]
        );
        // iMin 2 times 5.0, iMax 9 times 100.0.
        assert_eq!(
            w.calls(SOUND_HANDLE_SET_MIN_MAX),
            vec![vec![handle, 10.0f32.to_bits(), 900.0f32.to_bits()]]
        );
        assert_eq!(
            w.calls(SOUND_HANDLE_SET_TIME_BYTES),
            vec![vec![handle, 3, 4]]
        );
        assert_eq!(
            w.calls(SOUND_HANDLE_SET_CURVE),
            vec![vec![handle, 10, 11, 12, 13, 14]]
        );
        assert_eq!(w.calls(SOUND_HANDLE_SET_REVERB), vec![vec![handle, 0x55]]);
        assert_eq!(w.calls(SOUND_HANDLE_SET_PRIORITY), vec![vec![handle, 0xff]]);
        // The pitch byte is at data + 2 (sound + 0x46).
        assert_eq!(w.calls(SOUND_HANDLE_SET_PITCH), vec![vec![handle, 0x66]]);
    }

    #[test]
    fn apply_settings_defaults_the_distances_and_skips_optional_parts() {
        let (mut w, sound, handle) = apply_world();
        w.e.mem.set_u8(sound + 0x44, 0);
        w.e.mem.set_u8(sound + 0x45, 0);
        w.e.mem.set_u16(sound + 0x4e, 0);
        w.e.mem.set_u32(sound + 0x48, 0);
        let result = w.e.call(0x0082d400, &args![handle, sound, 0u32]).u32();
        assert_eq!(result, 0);
        assert_eq!(
            w.calls(SOUND_HANDLE_SET_MIN_MAX),
            vec![vec![handle, 70.0f32.to_bits(), 6000.0f32.to_bits()]]
        );
        assert!(w.calls(SOUND_HANDLE_SET_TIME_BYTES).is_empty());
        assert!(w.calls(SOUND_HANDLE_SET_PITCH).is_empty());
    }

    #[test]
    fn apply_settings_time_bytes_need_only_one_nonzero() {
        let (mut w, sound, handle) = apply_world();
        w.e.mem.set_u16(sound + 0x4e, 0x0700);
        w.e.call(0x0082d400, &args![handle, sound, 0u32]);
        assert_eq!(
            w.calls(SOUND_HANDLE_SET_TIME_BYTES),
            vec![vec![handle, 7, 0]]
        );
    }

    #[test]
    fn apply_settings_null_sound_returns_zero() {
        let (mut w, _, handle) = apply_world();
        assert_eq!(w.e.call(0x0082d400, &args![handle, 0u32, 0u32]).u32(), 0);
        assert!(w.calls(SOUND_HANDLE_SET_STATIC_ATTENUATION).is_empty());
    }

    // ---- the sound-data accessors ----

    #[test]
    fn sound_accessors_read_the_data_block() {
        let (mut w, sound, _) = apply_world();
        assert_eq!(w.e.call(0x0082d660, &args![sound]).u8(), 2);
        assert_eq!(w.e.call(0x0082d680, &args![sound]).u16(), 0x1234);
        for i in 0..5u32 {
            assert_eq!(w.e.call(0x0082d6a0, &args![sound, i]).u16(), 10 + i as u16);
        }
        assert_eq!(w.e.call(0x0082d6c0, &args![sound]).u16(), 0x55);
        assert_eq!(w.e.call(0x0082d6e0, &args![sound]).u32(), 3);
        assert_eq!(w.e.call(0x0082d700, &args![sound]).u32(), 4);
        assert_eq!(w.e.call(0x0082d720, &args![sound]).u32(), 0xff);
    }

    // ---- 0082d740 ----

    #[test]
    fn loop_points_are_reported_when_either_is_set() {
        for (begin, end, answer) in [
            (0u32, 0u32, false),
            (5, 0, true),
            (0, 9, true),
            (5, 9, true),
        ] {
            let mut w = World::new();
            let sound = w.sound(0x51, 0, (0, 0), 0);
            w.e.mem.set_u32(sound + 0x44 + 0x1c, begin);
            w.e.mem.set_u32(sound + 0x44 + 0x20, end);
            let out = w.e.mem.alloc(8);
            w.e.mem.set_u32(out, 0xdead);
            w.e.mem.set_u32(out + 4, 0xbeef);
            assert_eq!(
                w.e.call(0x0082d740, &args![sound, out, out + 4]).bool(),
                answer
            );
            let expected = if answer {
                (begin, end)
            } else {
                (0xdead, 0xbeef)
            };
            assert_eq!((word(&w.e, out), word(&w.e, out + 4)), expected);
        }
        let mut w = World::new();
        assert!(!w.e.call(0x0082d740, &args![0u32, 1u32, 2u32]).bool());
        assert!(w.calls(COPY_SOUND_DATA).is_empty());
    }

    // ---- the small player accessors ----

    #[test]
    fn player_accessors_use_their_offsets() {
        let mut w = World::new();
        let player = w.player(&[]);
        w.e.mem.set_u32(player + 0x7dc, 0xabcd);
        w.e.mem.set_u8(player + 0x5f9, 7);
        assert_eq!(w.e.call(0x0082eaa0, &args![player]).u32(), player + 0x774);
        assert_eq!(w.e.call(0x0082eac0, &args![player]).u8(), 7);
        w.e.call(0x0082ea80, &args![player]);
        assert_eq!(word(&w.e, player + 0x7dc), 0);
    }

    // ---- fn_0082eae0 and ClearAcousticSpace ----

    #[test]
    fn setting_the_current_space_fades_the_ambient_loop_out() {
        let mut w = World::new();
        w.set_playing(AMBIENT_LOOP);
        w.e.set_global(CURRENT_SPACE, 1u32);
        let space = w.space(0, [0; 4]);
        w.e.call(0x0082eae0, &args![space]);
        // fASFadeOutTime is 5.0 seconds.
        assert_eq!(
            w.calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE),
            vec![vec![AMBIENT_LOOP, 5000]]
        );
        assert_eq!(word(&w.e, AMBIENT_LOOP), u32::MAX);
        assert_eq!(w.calls(AUDIO_SET_ENVIRONMENT), vec![vec![AUDIO, 0x77]]);
        assert_eq!(word(&w.e, CURRENT_SPACE), space);
        assert_eq!(w.e.mem.u8(UPDATE_ACOUSTIC_SPACE), 1);
    }

    #[test]
    fn setting_the_same_space_does_nothing_and_null_skips_the_environment() {
        let mut w = World::new();
        w.e.set_global(CURRENT_SPACE, 0x4444u32);
        w.e.call(0x0082eae0, &args![0x4444u32]);
        assert!(w.calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE).is_empty());
        w.e.call(0x0082eae0, &args![0u32]);
        assert_eq!(w.calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE).len(), 1);
        assert!(w.calls(AUDIO_SET_ENVIRONMENT).is_empty());
        assert_eq!(word(&w.e, CURRENT_SPACE), 0);
        assert_eq!(w.e.mem.u8(UPDATE_ACOUSTIC_SPACE), 1);
    }

    #[test]
    fn clear_acoustic_space_obeys_the_lock_and_the_match() {
        // (locked, current, argument, cleared)
        for (locked, current, argument, cleared) in [
            (0u8, 0x10u32, 0u32, true),
            (0, 0x10, 0x10, true),
            (0, 0x10, 0x20, false),
            (0, 0, 0, false),
            (1, 0x10, 0, false),
        ] {
            let mut w = World::new();
            w.e.mem.set_u8(LOCK_ACOUSTIC_SPACE, locked);
            w.e.set_global(CURRENT_SPACE, current);
            w.e.call(0x0082eb70, &args![argument]);
            assert_eq!(
                w.calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE).len(),
                usize::from(cleared)
            );
            assert_eq!(word(&w.e, CURRENT_SPACE) == 0, cleared || current == 0);
            if cleared {
                assert_eq!(w.calls(AUDIO_SET_ENVIRONMENT), vec![vec![AUDIO, 0]]);
                assert_eq!(w.e.mem.u8(UPDATE_ACOUSTIC_SPACE), 1);
            }
        }
    }

    // ---- no-activation sound, save and load ----

    #[test]
    fn no_activation_sound_is_looked_up_once() {
        let mut w = World::new();
        w.e.set_global(DATA_HANDLER, 0x6666u32);
        w.e.set_global(TEST_SOUND_FORM, 0x777u32);
        assert_eq!(w.e.call(0x0082ec10, &[]).u32(), 0x777);
        w.e.set_global(TEST_SOUND_FORM, 0x888u32);
        assert_eq!(w.e.call(0x0082ec10, &[]).u32(), 0x777);
        assert_eq!(w.calls(GET_SOUND), vec![vec![0x6666, UI_ACTIVATE_NOTHING]]);
    }

    #[test]
    fn save_game_writes_the_no_activation_form() {
        let mut w = World::new();
        w.e.set_global(NO_ACTIVATION_SOUND, 0x1357u32);
        w.e.call(0x0082ec40, &args![0x9000u32]);
        assert_eq!(w.calls(SAVE_FORM_ID), vec![vec![0x9000, 0x1357, 0]]);
    }

    #[test]
    fn load_game_casts_the_loaded_form() {
        let mut w = World::new();
        w.e.register(LOAD_FORM_ID, |_, a| {
            assert_eq!(a[0], 0x9100);
            0x00ab_cdefu32.into_ret()
        });
        w.e.register(LOOKUP_FORM, |_, a| {
            assert_eq!(a[0], 0x00ab_cdef);
            0x2468u32.into_ret()
        });
        w.e.register(DYNAMIC_CAST, |_, a| {
            assert_eq!(
                a,
                &[0x2468, 0, FORM_TYPE_DESCRIPTOR, SOUND_TYPE_DESCRIPTOR, 0]
            );
            0x2469u32.into_ret()
        });
        w.e.call(0x0082ec60, &args![0x9100u32]);
        assert_eq!(word(&w.e, NO_ACTIVATION_SOUND), 0x2469);
    }

    // ---- 0082eca0 ----

    #[test]
    fn in_hearing_range_compares_each_axis_to_the_reach() {
        // The reach is the object's byte (2) times 100 times the scale (1.0):
        // 200. The listener is at (10, 20, 30).
        for (position, answer) in [
            ((10.0f32, 20.0f32, 30.0f32), true),
            ((210.0, 20.0, 30.0), true),
            ((211.0, 20.0, 30.0), false),
            ((10.0, -180.0, 30.0), true),
            ((10.0, -181.0, 30.0), false),
            ((10.0, 20.0, 231.0), false),
        ] {
            let mut w = World::new();
            w.listener();
            let object = w.object();
            w.e.mem.set_u8(object + 0x45, 2);
            let result =
                w.e.call(
                    0x0082eca0,
                    &args![object, position.0, position.1, position.2, 1.0f32],
                )
                .bool();
            assert_eq!(result, answer, "{position:?}");
        }
    }

    #[test]
    fn in_hearing_range_needs_an_object_and_a_listener() {
        let mut w = World::new();
        let object = w.object();
        // No listener.
        assert!(!w
            .e
            .call(0x0082eca0, &args![object, 0.0f32, 0.0f32, 0.0f32, 1.0f32])
            .bool());
        w.listener();
        // No object.
        assert!(!w
            .e
            .call(0x0082eca0, &args![0u32, 10.0f32, 20.0f32, 30.0f32, 1.0f32])
            .bool());
    }

    // ---- the Mysterious Stranger sounds ----

    #[test]
    fn mysterious_stranger_plays_one_and_precaches_two() {
        let mut w = World::new();
        w.e.call(0x0082ed80, &[]);
        let by_name = w.calls(AUDIO_GET_SOUND_HANDLE_BY_NAME);
        assert_eq!(by_name.len(), 1);
        assert_eq!(by_name[0][0], AUDIO);
        assert_eq!(&by_name[0][2..], &[MYSTERIOUS_STRANGER_A01, 0x0021_0101]);
        // The handle the first call fills is the one that is played.
        assert_eq!(w.calls(SOUND_HANDLE_PLAY), vec![vec![by_name[0][1], 0]]);
        assert_eq!(
            w.calls(AUDIO_PRECACHE),
            vec![vec![AUDIO, MYSTERIOUS_STRANGER_A02, 0x2020_0101]]
        );
        assert_eq!(w.calls(SOUND_HANDLE_DESTROY), vec![vec![by_name[0][1]]]);
    }

    #[test]
    fn mysterious_stranger_two_plays_the_second_sound() {
        let mut w = World::new();
        w.e.call(0x0082ee10, &[]);
        let by_name = w.calls(AUDIO_GET_SOUND_HANDLE_BY_NAME);
        assert_eq!(by_name.len(), 1);
        assert_eq!(&by_name[0][2..], &[MYSTERIOUS_STRANGER_A02, 0x0020_0101]);
        assert_eq!(w.calls(SOUND_HANDLE_PLAY), vec![vec![by_name[0][1], 0]]);
        assert!(w.calls(AUDIO_PRECACHE).is_empty());
    }

    // ---- the sound handle map's constructors and destructors ----

    #[test]
    fn map_constructor_builds_the_base_then_installs_the_vtable() {
        let mut w = World::new();
        w.e.register(MEMORY_ALLOCATE, |e, a| {
            assert_eq!(a[0], 0x40);
            e.mem.alloc(a[0]).into_ret()
        });
        w.e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        let map = w.e.mem.alloc(0x10);
        assert_eq!(w.e.call(0x0082ee80, &args![map, 16u32]).u32(), map);
        assert_eq!(word(&w.e, map), SOUND_HANDLE_MAP_VTABLE);
        assert_eq!(word(&w.e, map + 4), 16);
        assert_eq!(word(&w.e, map + 0xc), 0);
        assert_ne!(word(&w.e, map + 8), 0);
        assert_eq!(w.calls(MEMSET), vec![vec![word(&w.e, map + 8), 0, 0x40]]);
    }

    #[test]
    fn base_map_constructor_installs_the_base_vtable() {
        let mut w = World::new();
        w.e.register(MEMORY_ALLOCATE, |e, a| e.mem.alloc(a[0]).into_ret());
        w.e.register(MEMSET, |_, a| a[0].into_ret());
        let map = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(map + 0xc, 99);
        assert_eq!(w.e.call(0x0082eee0, &args![map, 8u32]).u32(), map);
        assert_eq!(word(&w.e, map), SOUND_HANDLE_MAP_BASE_VTABLE);
        assert_eq!(word(&w.e, map + 4), 8);
        assert_eq!(word(&w.e, map + 0xc), 0);
        assert_eq!(w.calls(MEMORY_ALLOCATE), vec![vec![32]]);
    }

    #[test]
    fn map_destructor_clears_then_runs_the_base_destructor() {
        let mut w = World::new();
        let map = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(map + 8, 0x1234);
        w.e.call(0x0082ef50, &args![map]);
        // The derived destructor clears first, then the base clears again and
        // frees the bucket array.
        assert_eq!(w.calls(MAP_CLEAR), vec![vec![map], vec![map]]);
        assert_eq!(w.calls(MEMORY_FREE), vec![vec![0x1234]]);
        assert_eq!(word(&w.e, map), SOUND_HANDLE_MAP_BASE_VTABLE);
    }

    #[test]
    fn base_map_destructor_clears_then_frees_the_buckets() {
        let mut w = World::new();
        let map = w.e.mem.alloc(0x10);
        w.e.mem.set_u32(map + 8, 0x4321);
        w.e.call(0x0082efb0, &args![map]);
        assert_eq!(w.calls(MAP_CLEAR), vec![vec![map]]);
        assert_eq!(w.calls(MEMORY_FREE), vec![vec![0x4321]]);
    }

    #[test]
    fn deleting_destructors_free_the_object_only_for_bit_zero() {
        for (address, clears) in [(0x0082eeb0u32, 2usize), (0x0082efe0, 1)] {
            for flags in [0u32, 1, 2, 3] {
                let mut w = World::new();
                let map = w.e.mem.alloc(0x10);
                assert_eq!(w.e.call(address, &args![map, flags]).u32(), map);
                assert_eq!(w.calls(MAP_CLEAR).len(), clears);
                assert_eq!(w.calls(FREE_OBJECT).len(), (flags & 1) as usize);
            }
        }
    }

    // ---- UpdateRegionSounds (0082d7c0): the acoustic space part ----

    /// A world that stops right after the footsteps.
    fn space_world() -> (World, u32) {
        let mut w = World::new();
        let player = w.player(&[]);
        w.e.mem.set_u8(REGION_UPDATE_DISABLED, 1);
        w.e.set_global(NO_ACTIVATION_SOUND, 1u32);
        (w, player)
    }

    #[test]
    fn update_stops_after_the_footsteps_when_region_updates_are_disabled() {
        let (mut w, _) = space_world();
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(w.calls(IMPACT_MIXER_INIT_FOOTSTEPS).len(), 1);
        assert_eq!(w.calls(IMPACT_MIXER_UPDATE).len(), 1);
        assert!(w.calls(AUDIO_SET_GAME_HOUR).is_empty());
        assert_eq!(word(&w.e, LAST_REGION_UPDATE), 0);
    }

    #[test]
    fn update_precaches_the_no_activation_sound_once() {
        let (mut w, _) = space_world();
        w.e.set_global(NO_ACTIVATION_SOUND, 0u32);
        w.e.set_global(DATA_HANDLER, 0x6666u32);
        let form = w.sound(0x88, 0, (0, 0), 0);
        w.e.set_global(TEST_SOUND_FORM, form);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(word(&w.e, NO_ACTIVATION_SOUND), form);
        assert_eq!(
            w.calls(AUDIO_PRECACHE_FORM),
            vec![vec![AUDIO, 0x7000_0088, 0x121, form]]
        );
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(w.calls(AUDIO_PRECACHE_FORM).len(), 1);
    }

    #[test]
    fn update_takes_the_acoustic_space_of_the_cell() {
        let (mut w, player) = space_world();
        let space = w.space(0, [0; 4]);
        let cell = w.object();
        w.e.mem.set_u32(cell + 0x100, space);
        w.e.mem.set_u32(player + 0x40, cell);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(word(&w.e, CURRENT_SPACE), space);
        assert_eq!(w.e.mem.u8(UPDATE_ACOUSTIC_SPACE), 1);
        // The environment of the new space goes to the audio twice: when the
        // space is set, and again by the update.
        assert_eq!(
            w.calls(AUDIO_SET_ENVIRONMENT),
            vec![vec![AUDIO, 0x77], vec![AUDIO, 0x77]]
        );
    }

    #[test]
    fn update_clears_the_space_without_a_cell() {
        let (mut w, _) = space_world();
        let space = w.space(0, [0; 4]);
        w.e.set_global(CURRENT_SPACE, space);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(word(&w.e, CURRENT_SPACE), 0);
        assert_eq!(
            w.calls(AUDIO_SET_ENVIRONMENT),
            vec![vec![AUDIO, 0], vec![AUDIO, 0]]
        );
    }

    #[test]
    fn update_keeps_the_space_while_the_player_is_in_a_transition() {
        let (mut w, player) = space_world();
        let space = w.space(0, [0; 4]);
        w.e.set_global(CURRENT_SPACE, space);
        w.e.mem.set_u8(player + 0x5f9, 1);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(word(&w.e, CURRENT_SPACE), space);
    }

    #[test]
    fn update_switches_to_the_flagged_space_of_the_cell_at_once() {
        let (mut w, player) = space_world();
        let old = w.space(0, [0; 4]);
        let new = w.space(1, [0; 4]);
        let cell = w.object();
        w.e.mem.set_u32(cell + 0x100, new);
        w.e.mem.set_u32(player + 0x40, cell);
        w.e.set_global(CURRENT_SPACE, old);
        w.e.set_global(NEXT_SPACE, new);
        w.e.mem.set_u8(UPDATE_ACOUSTIC_SPACE, 1);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(word(&w.e, CURRENT_SPACE), new);
        // The pending-change flag and the next space were cleared.
        assert_eq!(w.e.mem.u8(UPDATE_ACOUSTIC_SPACE), 0);
        assert_eq!(word(&w.e, NEXT_SPACE), 0);
    }

    #[test]
    fn update_applies_or_drops_a_pending_space_change() {
        let (mut w, player) = space_world();
        let space = w.space(0, [0; 4]);
        let other = w.space(0, [0; 4]);
        let cell = w.object();
        w.e.mem.set_u32(cell + 0x100, space);
        w.e.mem.set_u32(player + 0x40, cell);
        w.e.set_global(CURRENT_SPACE, space);
        // The pending space equals the current one: dropped.
        w.e.set_global(NEXT_SPACE, space);
        w.e.mem.set_u8(UPDATE_ACOUSTIC_SPACE, 1);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(w.e.mem.u8(UPDATE_ACOUSTIC_SPACE), 0);
        assert_eq!(word(&w.e, NEXT_SPACE), 0);
        // A different pending space is made current (the flag stays set).
        w.e.set_global(NEXT_SPACE, other);
        w.e.mem.set_u8(UPDATE_ACOUSTIC_SPACE, 1);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(word(&w.e, CURRENT_SPACE), other);
        assert_eq!(w.e.mem.u8(UPDATE_ACOUSTIC_SPACE), 1);
    }

    #[test]
    fn update_starts_and_stops_the_underwater_loop() {
        let (mut w, player) = space_world();
        let cell = w.object();
        w.e.mem.set_u32(player + 0x40, cell);
        w.e.mem.set_u8(TEST_UNDERWATER, 1);
        w.e.call(0x0082d7c0, &[]);
        let by_name = w.calls(AUDIO_GET_SOUND_HANDLE_BY_NAME);
        assert_eq!(by_name.len(), 1);
        assert_eq!(&by_name[0][2..], &[UNDERWATER_LOOP_NAME, 0x2101]);
        assert_eq!(word(&w.e, UNDERWATER_LOOP), 0x4321);
        assert_eq!(
            w.calls(SOUND_HANDLE_SET_FLAG_BYTE),
            vec![vec![UNDERWATER_LOOP, 1]]
        );
        assert_eq!(w.calls(SOUND_HANDLE_PLAY), vec![vec![UNDERWATER_LOOP, 0]]);
        // Playing already: nothing new.
        w.set_playing(UNDERWATER_LOOP);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(w.calls(SOUND_HANDLE_PLAY).len(), 1);
        // No longer underwater: faded out over 300 ms.
        w.e.mem.set_u8(TEST_UNDERWATER, 0);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(
            w.calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE),
            vec![vec![UNDERWATER_LOOP, 300]]
        );
    }

    // ---- UpdateRegionSounds: the clock and the quick path ----

    #[test]
    fn update_hands_the_game_hour_to_the_audio_and_stores_the_time() {
        let mut w = World::new();
        let player = w.region_setup(&[], 12.0, 0);
        w.e.mem.set_u32(player + 0x7dc, 0x5555);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(
            w.calls(AUDIO_SET_GAME_HOUR),
            vec![vec![AUDIO, 12.0f32.to_bits(), 0]]
        );
        assert_eq!(word(&w.e, LAST_REGION_UPDATE), 3000);
        // The player's closest audio marker was cleared.
        assert_eq!(word(&w.e, player + 0x7dc), 0);
        // Without a current space the ambient loop fades out over 1000 ms.
        assert_eq!(
            w.calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE),
            vec![vec![AMBIENT_LOOP, 1000]]
        );
        assert_eq!(w.calls(TES_METHOD_00455690).len(), 1);
    }

    #[test]
    fn update_without_a_sky_uses_hour_zero() {
        let mut w = World::new();
        w.region_setup(&[], 12.0, 0);
        let tes = w.object();
        w.e.set_global(TES, tes);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(w.calls(AUDIO_SET_GAME_HOUR), vec![vec![AUDIO, 0, 0]]);
    }

    #[test]
    fn update_skips_the_tes_hook_when_the_top_menu_matches() {
        let mut w = World::new();
        w.region_setup(&[], 12.0, 0);
        w.e.mem.set_u8(TEST_TOP_MENU, 1);
        w.e.call(0x0082d7c0, &[]);
        assert!(w.calls(TES_METHOD_00455690).is_empty());
    }

    #[test]
    fn quick_update_drains_the_map_when_the_list_is_empty() {
        let mut w = World::new();
        w.region_setup(&[], 12.0, 0);
        w.e.set_global(TEST_NOW, 1500u32);
        let first = w.e.mem.alloc(0xc);
        let second = w.e.mem.alloc(0xc);
        w.map.borrow_mut().extend([(0x11, first), (0x22, second)]);
        w.e.call(0x0082d7c0, &[]);
        assert!(w.map.borrow().is_empty());
        assert_eq!(
            w.calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE),
            vec![vec![first, 300], vec![second, 300]]
        );
        assert_eq!(
            w.calls(SOUND_HANDLE_DELETE),
            vec![vec![first, 1], vec![second, 1]]
        );
        // The time stamp is not updated on the quick path.
        assert_eq!(word(&w.e, LAST_REGION_UPDATE), 1000);
        assert!(w.calls(TES_METHOD_00455690).is_empty());
    }

    #[test]
    fn quick_update_leaves_the_map_when_the_list_has_sounds() {
        let mut w = World::new();
        let (_, entry) = w.looping_entry(0x31, 0, (0, 0));
        w.region_setup(&[entry], 12.0, 0);
        w.e.set_global(TEST_NOW, 1500u32);
        let handle = w.e.mem.alloc(0xc);
        w.map.borrow_mut().push((0x31, handle));
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(w.map.borrow().len(), 1);
        assert!(w.calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE).is_empty());
    }

    // ---- UpdateRegionSounds: the region sounds ----

    #[test]
    fn full_update_starts_a_looping_region_sound_and_tracks_it() {
        let mut w = World::new();
        let (sound, entry) = w.looping_entry(0x31, 0, (0, 0));
        w.region_setup(&[entry], 12.0, 0);
        w.e.call(0x0082d7c0, &[]);
        let by_file = w.calls(AUDIO_GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(by_file.len(), 1);
        assert_eq!(&by_file[0][2..], &[0x7000_0031, 0x11011, sound]);
        let map = w.map.borrow().clone();
        assert_eq!(map.len(), 1);
        assert_eq!(map[0].0, 0x31);
        let handle = map[0].1;
        assert_eq!(word(&w.e, handle), 0x1234);
        assert_eq!(w.calls(SOUND_HANDLE_SET_FLAG_BYTE), vec![vec![handle, 1]]);
        // fRegionLoopFadeInTime is 0.6 seconds.
        assert_eq!(w.calls(SOUND_HANDLE_START_FADE), vec![vec![handle, 600]]);
        // The cleanup pass finds it in the list and keeps it.
        assert!(w.calls(SOUND_HANDLE_DELETE).is_empty());
    }

    #[test]
    fn full_update_fades_out_a_playing_loop_that_the_weather_rules_out() {
        let mut w = World::new();
        // The entry wants weather bit 1; the weather's byte has only bit 2.
        let (_, entry) = w.looping_entry(0x31, 1, (0, 0));
        w.region_setup(&[entry], 12.0, 2);
        let handle = w.e.mem.alloc(0xc);
        w.set_playing(handle);
        w.map.borrow_mut().push((0x31, handle));
        w.e.call(0x0082d7c0, &[]);
        // fRegionLoopFadeOutTime is 1.0 second.
        assert!(w
            .calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE)
            .contains(&vec![handle, 1000]));
        assert_eq!(w.calls(SOUND_HANDLE_DELETE), vec![vec![handle, 1]]);
        assert!(w.map.borrow().is_empty());
    }

    #[test]
    fn full_update_restarts_a_stopped_loop_that_is_allowed_again() {
        let mut w = World::new();
        // Weather bit shared with the entry: allowed.
        let (_, entry) = w.looping_entry(0x31, 3, (0, 0));
        w.region_setup(&[entry], 12.0, 2);
        let handle = w.e.mem.alloc(0xc);
        // Valid but not playing.
        w.e.mem.set_u32(handle, 0x99);
        w.map.borrow_mut().push((0x31, handle));
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(w.calls(SOUND_HANDLE_START_FADE), vec![vec![handle, 600]]);
        assert!(!w
            .calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE)
            .iter()
            .any(|call| call[0] == handle));
        assert_eq!(w.map.borrow().len(), 1);
    }

    #[test]
    fn full_update_without_a_weather_mask_ignores_the_entry_flags() {
        let mut w = World::new();
        // The weather byte is 0: nothing rules the entry out.
        let (_, entry) = w.looping_entry(0x31, 1, (0, 0));
        w.region_setup(&[entry], 12.0, 0);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(w.map.borrow().len(), 1);
    }

    #[test]
    fn full_update_honors_the_time_window_of_a_sound() {
        // Window start 8 * 0.09375 = 0.75, end 16 * 0.09375 = 1.5; the game
        // hour is compared with those values directly.
        for (hour, allowed) in [(1.0f32, true), (12.0, false), (0.5, false)] {
            let mut w = World::new();
            let (_, entry) = w.looping_entry(0x31, 0, (8, 16));
            w.region_setup(&[entry], hour, 0);
            w.e.call(0x0082d7c0, &[]);
            assert_eq!(w.map.borrow().len(), usize::from(allowed), "hour {hour}");
        }
    }

    #[test]
    fn full_update_honors_a_window_that_wraps_past_midnight() {
        // Start 20 * 0.09375 = 1.875, end 4 * 0.09375 = 0.375: the sound is
        // ruled out only between the end and the start.
        for (hour, allowed) in [(2.0f32, true), (0.2, true), (1.0, false)] {
            let mut w = World::new();
            let (_, entry) = w.looping_entry(0x31, 0, (20, 4));
            w.region_setup(&[entry], hour, 0);
            w.e.call(0x0082d7c0, &[]);
            assert_eq!(w.map.borrow().len(), usize::from(allowed), "hour {hour}");
        }
    }

    #[test]
    fn full_update_ignores_a_window_narrower_than_a_step() {
        // Start 2 * 0.09375, end 2 * 0.09375: zero width, always allowed.
        let mut w = World::new();
        let (_, entry) = w.looping_entry(0x31, 0, (2, 2));
        w.region_setup(&[entry], 20.0, 0);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(w.map.borrow().len(), 1);
    }

    #[test]
    fn full_update_plays_a_one_shot_around_the_listener_on_a_good_roll() {
        let mut w = World::new();
        // A one-shot: its flags map to no loop bit; bit 0x40 picks 0x11101.
        let sound = w.sound(0x32, 0x40, (0, 0), 0);
        let entry = w.region_sound(sound, 0, 20_000_000);
        w.region_setup(&[entry], 12.0, 0);
        w.listener();
        // The chance passes, the x sign flips, the y sign does not.
        w.chances.borrow_mut().extend([true, true, false]);
        w.randoms.borrow_mut().extend([0u32, 5]);
        w.e.call(0x0082d7c0, &[]);
        let by_file = w.calls(AUDIO_GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(by_file.len(), 1);
        assert_eq!(&by_file[0][2..], &[0x7000_0032, 0x11101, sound]);
        // Offsets: x = -(0 % 650 + 100), y = 5 % 650 + 100, around the
        // listener (10, 20, 30); z lifted by 256.
        let position = w.calls(SOUND_HANDLE_SET_POSITION);
        assert_eq!(position.len(), 1);
        assert_eq!(f32::from_bits(position[0][1]), 10.0 - 100.0);
        assert_eq!(f32::from_bits(position[0][2]), 20.0 + 105.0);
        assert_eq!(f32::from_bits(position[0][3]), 30.0 + 256.0);
        let handle = position[0][0];
        assert_eq!(w.calls(SOUND_HANDLE_SET_PRIORITY), vec![vec![handle, 0xff]]);
        assert_eq!(w.calls(SOUND_HANDLE_PLAY), vec![vec![handle, 0]]);
        // The roll received the entry's chance (1.0 once divided by 20000000),
        // then 0.5 twice for the signs.
        assert_eq!(
            w.calls(RANDOM_CHANCE),
            vec![
                vec![1.0f32.to_bits()],
                vec![0.5f32.to_bits()],
                vec![0.5f32.to_bits()]
            ]
        );
        // The local handle was built from sound id 0 and destroyed again.
        assert_eq!(
            w.calls(SOUND_HANDLE_CONSTRUCT_WITH_ID),
            vec![vec![handle, 0]]
        );
        assert_eq!(w.calls(SOUND_HANDLE_DESTROY).last(), Some(&vec![handle]));
    }

    #[test]
    fn full_update_one_shot_uses_the_player_when_there_is_no_listener() {
        let mut w = World::new();
        let sound = w.sound(0x32, 0, (0, 0), 0);
        let entry = w.region_sound(sound, 0, 20_000_000);
        w.region_setup(&[entry], 12.0, 0);
        w.chances.borrow_mut().extend([true, false, false]);
        w.e.call(0x0082d7c0, &[]);
        let by_file = w.calls(AUDIO_GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(&by_file[0][2..], &[0x7000_0032, 0x11102, sound]);
        let position = w.calls(SOUND_HANDLE_SET_POSITION);
        assert_eq!(f32::from_bits(position[0][1]), 100.0 + 100.0);
        assert_eq!(f32::from_bits(position[0][2]), 200.0 + 100.0);
        assert_eq!(f32::from_bits(position[0][3]), 300.0 + 256.0);
    }

    #[test]
    fn full_update_one_shot_needs_the_roll() {
        let mut w = World::new();
        let sound = w.sound(0x32, 0, (0, 0), 0);
        let entry = w.region_sound(sound, 0, 20_000_000);
        w.region_setup(&[entry], 12.0, 0);
        w.e.call(0x0082d7c0, &[]);
        assert!(w.calls(AUDIO_GET_SOUND_HANDLE_BY_FILENAME).is_empty());
        assert!(w.calls(SOUND_HANDLE_PLAY).is_empty());
    }

    #[test]
    fn full_update_removes_a_tracked_sound_that_left_the_list() {
        let mut w = World::new();
        // The listed sound is a one-shot with no chance, so nothing is added
        // to the map.
        let sound = w.sound(0x32, 0, (0, 0), 0);
        let entry = w.region_sound(sound, 0, 0);
        w.region_setup(&[entry], 12.0, 0);
        let stale = w.e.mem.alloc(0xc);
        w.map.borrow_mut().push((0x99, stale));
        w.e.call(0x0082d7c0, &[]);
        // Faded out (fRegionLoopFadeOutTime is 1.0 second) and deleted.
        assert!(w.map.borrow().is_empty());
        assert!(w
            .calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE)
            .contains(&vec![stale, 1000]));
        assert_eq!(w.calls(SOUND_HANDLE_DELETE), vec![vec![stale, 1]]);
    }

    #[test]
    fn full_update_does_not_rewind_the_list_between_tracked_sounds() {
        // The cleanup walks the list with one cursor that is never reset.
        // The stale key exhausts it, so the second tracked key finds nothing
        // and is removed too although its sound is listed.
        let mut w = World::new();
        let (_, entry) = w.looping_entry(0x31, 0, (0, 0));
        w.region_setup(&[entry], 12.0, 0);
        let stale = w.e.mem.alloc(0xc);
        let kept = w.e.mem.alloc(0xc);
        w.set_playing(kept);
        w.map.borrow_mut().extend([(0x99, stale), (0x31, kept)]);
        w.e.call(0x0082d7c0, &[]);
        assert!(w.map.borrow().is_empty());
        assert_eq!(
            w.calls(SOUND_HANDLE_DELETE),
            vec![vec![stale, 1], vec![kept, 1]]
        );
    }

    #[test]
    fn full_update_keeps_everything_after_the_first_match() {
        // Once one tracked key was found in the list the "found" flag stays
        // set, so later unmatched keys are kept.
        let mut w = World::new();
        let (_, entry) = w.looping_entry(0x31, 0, (0, 0));
        w.region_setup(&[entry], 12.0, 0);
        let kept = w.e.mem.alloc(0xc);
        let late = w.e.mem.alloc(0xc);
        w.set_playing(kept);
        w.map.borrow_mut().extend([(0x31, kept), (0x99, late)]);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(w.map.borrow().len(), 2);
        assert!(w.calls(SOUND_HANDLE_DELETE).is_empty());
    }

    // ---- UpdateRegionSounds: the ambient loop ----

    /// A world with a current space of four ambient sounds (the first sound
    /// 0xa0), an `other` space with the same sounds, and a full update due.
    fn ambient_world(hour: f32, flag: u8) -> (World, u32, u32) {
        let mut w = World::new();
        let player = w.region_setup(&[], hour, 0);
        let sounds = [
            w.sound(0xa0, 0, (0, 0), 0),
            w.sound(0xa1, 0, (0, 0), 0),
            w.sound(0xa2, 0, (0, 0), 0),
            w.sound(0xa3, 0, (0, 0), 0),
        ];
        let space = w.space(flag, sounds);
        let other = w.space(flag, sounds);
        let cell = w.object();
        w.e.mem.set_u32(cell + 0x100, space);
        w.e.mem.set_u32(player + 0x40, cell);
        w.e.set_global(CURRENT_SPACE, space);
        (w, space, other)
    }

    #[test]
    fn ambient_loop_of_a_flagged_space_plays_the_first_sound() {
        let (mut w, _, _) = ambient_world(12.0, 1);
        w.e.call(0x0082d7c0, &[]);
        let by_file = w.calls(AUDIO_GET_SOUND_HANDLE_BY_FILENAME);
        assert_eq!(by_file.len(), 1);
        assert_eq!(by_file[0][2], 0x7000_00a0);
        assert_eq!(by_file[0][3], 0x11011);
        assert_eq!(word(&w.e, AMBIENT_LOOP), 0x1234);
        // fASFadeStartTime is 3.0 seconds.
        assert_eq!(
            w.calls(SOUND_HANDLE_START_FADE),
            vec![vec![AMBIENT_LOOP, 3000]]
        );
        // The loop is valid now, so a second update leaves it.
        w.e.set_global(TEST_NOW, 6000u32);
        w.e.call(0x0082d7c0, &[]);
        assert_eq!(w.calls(AUDIO_GET_SOUND_HANDLE_BY_FILENAME).len(), 1);
    }

    #[test]
    fn ambient_loop_of_a_flagged_space_is_replaced_when_a_change_is_pending() {
        let (mut w, _, other) = ambient_world(12.0, 1);
        w.set_playing(AMBIENT_LOOP);
        w.e.mem.set_u8(UPDATE_ACOUSTIC_SPACE, 1);
        w.e.set_global(NEXT_SPACE, other);
        w.e.mem.set_u8(AMBIENT_REPLACED_FLAG, 1);
        w.e.call(0x0082d7c0, &[]);
        // The pending space was made current (its change faded the old loop
        // out over fASFadeOutTime and reset it), the pending change cleared.
        assert_eq!(word(&w.e, CURRENT_SPACE), other);
        assert_eq!(w.e.mem.u8(UPDATE_ACOUSTIC_SPACE), 0);
        assert_eq!(word(&w.e, NEXT_SPACE), 0);
        assert_eq!(w.e.mem.u8(AMBIENT_REPLACED_FLAG), 0);
        assert!(w
            .calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE)
            .contains(&vec![AMBIENT_LOOP, 5000]));
        assert_eq!(w.calls(AUDIO_GET_SOUND_HANDLE_BY_FILENAME).len(), 1);
    }

    #[test]
    fn ambient_loop_of_a_flagged_space_fades_a_playing_invalid_loop_by_half() {
        let (mut w, _, _) = ambient_world(12.0, 1);
        // Playing (state 1) but the sound id is -1: invalid, so replaced; the
        // fade-out uses half the fade-out time in milliseconds (2500).
        w.e.mem.set_u32(AMBIENT_LOOP + 8, 1);
        w.e.call(0x0082d7c0, &[]);
        assert!(w
            .calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE)
            .contains(&vec![AMBIENT_LOOP, 2500]));
        // The loop was playing, so the fade-in uses fASFadeInTime (2.0 s).
        assert_eq!(
            w.calls(SOUND_HANDLE_START_FADE),
            vec![vec![AMBIENT_LOOP, 2000]]
        );
    }

    #[test]
    fn ambient_loop_picks_the_sound_by_the_hour() {
        // dawn 6, day 10, dusk 18, night 22: the thresholds of the settings.
        for (hour, slot, expected) in [
            (4.0f32, 3u32, 0xa3u32),
            (8.0, 0, 0xa0),
            (12.0, 1, 0xa1),
            (20.0, 2, 0xa2),
            (23.0, 3, 0xa3),
        ] {
            let (mut w, _, other) = ambient_world(hour, 0);
            // A pending change makes the update pick a sound at once.
            w.e.mem.set_u8(UPDATE_ACOUSTIC_SPACE, 1);
            w.e.set_global(NEXT_SPACE, other);
            w.e.call(0x0082d7c0, &[]);
            assert_eq!(word(&w.e, CURRENT_TIME_SLOT), slot, "hour {hour}");
            let by_file = w.calls(AUDIO_GET_SOUND_HANDLE_BY_FILENAME);
            assert_eq!(by_file.len(), 1, "hour {hour}");
            assert_eq!(by_file[0][2], 0x7000_0000 + expected, "hour {hour}");
        }
    }

    #[test]
    fn ambient_loop_changes_when_the_hour_passes_the_current_slot() {
        // (slot, hour, changes)
        for (slot, hour, changes) in [
            (0u32, 9.0f32, false),
            (0, 11.0, true),
            (1, 17.0, false),
            (1, 19.0, true),
            (2, 21.0, false),
            (2, 23.0, true),
            (3, 5.0, false),
            (3, 7.0, true),
            (3, 23.0, false),
            (9, 12.0, false),
        ] {
            let (mut w, _, _) = ambient_world(hour, 0);
            w.e.set_global(CURRENT_TIME_SLOT, slot);
            w.e.call(0x0082d7c0, &[]);
            assert_eq!(
                w.calls(AUDIO_GET_SOUND_HANDLE_BY_FILENAME).len(),
                usize::from(changes),
                "slot {slot} hour {hour}"
            );
            if changes {
                // Not pending: the old loop is faded out over fASFadeOutTime.
                assert!(w
                    .calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE)
                    .contains(&vec![AMBIENT_LOOP, 5000]));
            }
        }
    }

    #[test]
    fn ambient_loop_is_faded_out_without_a_player() {
        let (mut w, _, _) = ambient_world(12.0, 0);
        w.e.set_global(PLAYER, 0u32);
        w.e.mem.set_u8(TEST_TOP_MENU, 1);
        // With no player the update reads the player's list head at null.
        w.e.map(0, 0x1000);
        w.e.call(0x0082d7c0, &[]);
        assert!(w
            .calls(SOUND_HANDLE_FADE_OUT_AND_RELEASE)
            .contains(&vec![AMBIENT_LOOP, 1000]));
    }

    #[test]
    fn region_sounds_come_from_the_current_spaces_region_data() {
        let (mut w, space, _) = ambient_world(12.0, 0);
        let (_, entry) = w.looping_entry(0x31, 0, (0, 0));
        let list = w.list(&[entry]);
        // The space's owner (+0x48) has a data list (+0x18) whose type-7 entry
        // (+0x20 in the double) has the sound list embedded at +0xc.
        let owner = w.object();
        let data_list = w.object();
        let region_entry = w.object();
        w.e.mem.set_u32(space + 0x48, owner);
        w.e.mem.set_u32(owner + 0x18, data_list);
        w.e.mem.set_u32(data_list + 0x20, region_entry);
        let (item, next) = (word(&w.e, list), word(&w.e, list + 4));
        w.e.mem.set_u32(region_entry + 0xc, item);
        w.e.mem.set_u32(region_entry + 0x10, next);
        w.e.call(0x0082d7c0, &[]);
        // The loop sound of that list is tracked (the player's list was empty).
        assert_eq!(w.map.borrow().len(), 1);
    }
}
