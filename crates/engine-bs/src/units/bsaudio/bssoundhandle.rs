//! `BSAudio/bssoundhandle.cpp` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A `BSSoundHandle` is 12 bytes: the sound id the audio manager knows the
//! sound by (`-1` for "no sound"), a flag byte and the state the handle
//! assumes the sound is in. Nearly every method is a thin wrapper: when the
//! id is not `-1` it fetches the audio manager (`00ad9060`, the engine map's
//! `BSAudioManager::QInstance`, which takes no argument) and calls the
//! manager method with the sound id first and then the wrapper's own
//! arguments (the code pushes the arguments, then the id, then calls
//! `QInstance`, which leaves them on the stack for the thiscall method).
//! The manager methods are in `BSAudio/bsaudiomanager.cpp` and are called
//! by address; their names below come from the engine map.

#[allow(unused_imports)]
use crate::prelude::*;

/// `BSAudioManager::QInstance`: returns the manager (no arguments).
const AUDIO_MANAGER_INSTANCE: u32 = 0x00ad_9060;
/// The sound id of a handle that holds no sound.
const NO_SOUND: u32 = 0xffff_ffff;
/// The pointer to the object whose byte at +0x04 the completion-callback
/// setters test (`BSAudioManager::Update` reads it too); its class is not
/// confirmed here.
const CALLBACK_GATE_OBJECT: u32 = 0x011f_6d98;
/// `BSSoundHandle::pfnAddMovingSoundCallback` (Xbox PDB static): a cdecl
/// function taking the followed object, called when it is not null.
const ADD_MOVING_SOUND_CALLBACK: u32 = 0x011f_6e08;

/// `BSSoundHandle::ASSUMED_STATE_PLAYING` (Xbox PDB).
const STATE_PLAYING: u32 = 1;
/// `BSSoundHandle::ASSUMED_STATE_STOPPED` (Xbox PDB).
const STATE_STOPPED: u32 = 2;
/// `BSSoundHandle::ASSUMED_STATE_PAUSED` (Xbox PDB).
const STATE_PAUSED: u32 = 3;

/// `BSAudioManager::Play`.
const MANAGER_PLAY: u32 = 0x00ad_b3e0;
/// `BSAudioManager::PlayAfter`.
const MANAGER_PLAY_AFTER: u32 = 0x00ad_b4e0;
/// `BSAudioManager::Pause`.
const MANAGER_PAUSE: u32 = 0x00ad_b590;
/// `BSAudioManager::Stop`.
const MANAGER_STOP: u32 = 0x00ad_b610;
/// `BSAudioManager::IsPlaying`.
const MANAGER_IS_PLAYING: u32 = 0x00ad_c910;
/// Unnamed manager method (returns a word).
const MANAGER_00ADC9A0: u32 = 0x00ad_c9a0;
/// `BSAudioManager::SetStaticAttenuation`.
const MANAGER_SET_STATIC_ATTENUATION: u32 = 0x00ad_b720;
/// `BSAudioManager::SetVolume`.
const MANAGER_SET_VOLUME: u32 = 0x00ad_c0d0;
/// `BSAudioManager::GetVolume` (float in ST0).
const MANAGER_GET_VOLUME: u32 = 0x00ad_c8b0;
/// Unnamed manager method taking a float.
const MANAGER_00ADB7C0: u32 = 0x00ad_b7c0;
/// `BSAudioManager::SetModFrequency`.
const MANAGER_SET_MOD_FREQUENCY: u32 = 0x00ad_b850;
/// Unnamed manager method taking a byte.
const MANAGER_00ADB8E0: u32 = 0x00ad_b8e0;
/// Unnamed manager method taking a word.
const MANAGER_00ADB960: u32 = 0x00ad_b960;
/// `BSAudioManager::GetDuration`.
const MANAGER_GET_DURATION: u32 = 0x00ad_c830;
/// `BSAudioManager::SetPosition`.
const MANAGER_SET_POSITION: u32 = 0x00ad_b970;
/// Unnamed manager method taking three floats.
const MANAGER_00ADB9F0: u32 = 0x00ad_b9f0;
/// `BSAudioManager::SetMinMax`.
const MANAGER_SET_MIN_MAX: u32 = 0x00ad_ba70;
/// Unnamed manager method taking five 16-bit values.
const MANAGER_00ADBB30: u32 = 0x00ad_bb30;
/// Unnamed manager method taking a 16-bit value.
const MANAGER_00ADBEC0: u32 = 0x00ad_bec0;
/// Unnamed manager method taking three floats.
const MANAGER_00ADBF40: u32 = 0x00ad_bf40;
/// `BSAudioManager::IsValid`.
const MANAGER_IS_VALID: u32 = 0x00ad_c7d0;
/// `BSAudioManager::Release`.
const MANAGER_RELEASE: u32 = 0x00ad_bf50;
/// Unnamed manager method taking a word.
const MANAGER_00ADC670: u32 = 0x00ad_c670;
/// `BSAudioManager::FadeOut`.
const MANAGER_FADE_OUT: u32 = 0x00ad_c560;
/// Unnamed manager method taking two bytes.
const MANAGER_00ADC4E0: u32 = 0x00ad_c4e0;
/// `BSAudioManager::SetCompletionCallback`.
const MANAGER_SET_COMPLETION_CALLBACK: u32 = 0x00ad_bfd0;
/// Unnamed manager method taking a callback and its argument.
const MANAGER_00ADC050: u32 = 0x00ad_c050;
/// `BSAudioManager::SetObjectToFollow`.
const MANAGER_SET_OBJECT_TO_FOLLOW: u32 = 0x00ad_c1e0;
/// Unnamed manager method taking a word.
const MANAGER_00ADC260: u32 = 0x00ad_c260;
/// Unnamed manager method taking a byte.
const MANAGER_00ADC2E0: u32 = 0x00ad_c2e0;
/// Manager method called by `SetSynchStartTime`.
const MANAGER_SET_SYNCH_START_TIME: u32 = 0x00ad_c360;
/// Unnamed manager method taking only the sound id.
const MANAGER_00ADC3E0: u32 = 0x00ad_c3e0;
/// `BSAudioManager::SetPriority`.
const MANAGER_SET_PRIORITY: u32 = 0x00ad_c460;

layout! {
    /// `BSSoundHandle` (Xbox PDB), 0xC bytes.
    pub struct BSSoundHandle: 0xC {
        /// `iSoundID` (Xbox PDB): `-1` when the handle holds no sound.
        0x00 iSoundID: u32,
        /// `bAssumeSuccess` (Xbox PDB).
        0x04 bAssumeSuccess: bool,
        /// `eState` (Xbox PDB `ASSUMED_STATE`: initialized 0, playing 1,
        /// stopped 2, paused 3), written as a 32-bit word.
        0x08 eState: u32,
    }
}

/// Calls manager `method` with the manager as `this`, then the sound id of
/// `handle`, then `rest` (the wrapper's own arguments).
fn manager_call(e: &mut Engine, handle: Ptr<BSSoundHandle>, method: u32, rest: &[u32]) -> Ret {
    let sound_id = e.get(handle, BSSoundHandle::iSoundID);
    let manager = e.call(AUDIO_MANAGER_INSTANCE, &[]).u32();
    let mut words = vec![manager, sound_id];
    words.extend_from_slice(rest);
    e.call(method, &words)
}

/// Whether `handle` holds no sound.
fn has_no_sound(e: &Engine, handle: Ptr<BSSoundHandle>) -> bool {
    e.get(handle, BSSoundHandle::iSoundID) == NO_SOUND
}

/// The immediate part of the completion-callback setters: when the byte at
/// +4 of the object at `011f6d98` is clear and `callback` is not null, calls
/// the cdecl callback with `(callback_argument, status)`.
fn callback_gate(e: &mut Engine, callback: u32, callback_argument: u32, status: u32) {
    let gate_object = e.global::<u32>(CALLBACK_GATE_OBJECT);
    if e.mem.u8(gate_object + 4) == 0 && callback != 0 {
        e.call(callback, &args![callback_argument, status]);
    }
}

// Translated from 00ad8830 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::Play` (Xbox PDB): sets the state to playing and plays the
/// sound with the given flag byte; false when the handle holds no sound.
pub fn bs_sound_handle_play(e: &mut Engine, this: Ptr<BSSoundHandle>, flag: u8) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    e.set(this, BSSoundHandle::eState, STATE_PLAYING);
    manager_call(e, this, MANAGER_PLAY, &[flag as u32]).bool()
}

// Translated from 00ad8870 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::PlayAfter` (Xbox PDB): sets the state to playing and
/// plays the sound after another (two words, passed on unchanged).
pub fn bs_sound_handle_play_after(
    e: &mut Engine,
    this: Ptr<BSSoundHandle>,
    first: u32,
    second: u32,
) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    e.set(this, BSSoundHandle::eState, STATE_PLAYING);
    manager_call(e, this, MANAGER_PLAY_AFTER, &[first, second]).bool()
}

// Translated from 00ad88b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::Pause` (Xbox PDB): sets the state to paused and pauses
/// the sound.
pub fn bs_sound_handle_pause(e: &mut Engine, this: Ptr<BSSoundHandle>) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    e.set(this, BSSoundHandle::eState, STATE_PAUSED);
    manager_call(e, this, MANAGER_PAUSE, &[]).bool()
}

// Translated from 00ad88f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::Stop` (Xbox PDB): sets the state to stopped and stops
/// the sound.
pub fn bs_sound_handle_stop(e: &mut Engine, this: Ptr<BSSoundHandle>) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    e.set(this, BSSoundHandle::eState, STATE_STOPPED);
    manager_call(e, this, MANAGER_STOP, &[]).bool()
}

// Translated from 00ad8930 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::IsPlaying` (Xbox PDB): false without a sound; while
/// `bAssumeSuccess` is set it answers from the assumed state without asking
/// the manager, otherwise the manager decides.
pub fn bs_sound_handle_is_playing(e: &mut Engine, this: Ptr<BSSoundHandle>) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    if e.get(this, BSSoundHandle::bAssumeSuccess) {
        return e.get(this, BSSoundHandle::eState) == STATE_PLAYING;
    }
    manager_call(e, this, MANAGER_IS_PLAYING, &[]).bool()
}

// Translated from 00ad8980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Asks the manager (`00adc9a0`, unnamed in the map) about the sound and
/// returns its word; 0 without a sound.
pub fn fn_00ad8980(e: &mut Engine, this: Ptr<BSSoundHandle>) -> u32 {
    if has_no_sound(e, this) {
        return 0;
    }
    manager_call(e, this, MANAGER_00ADC9A0, &[]).u32()
}

// Translated from 00ad89b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::SetStaticAttenuation` (Xbox PDB): passes the 16-bit
/// attenuation to the manager.
pub fn bs_sound_handle_set_static_attenuation(
    e: &mut Engine,
    this: Ptr<BSSoundHandle>,
    attenuation: u16,
) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    manager_call(
        e,
        this,
        MANAGER_SET_STATIC_ATTENUATION,
        &[attenuation as u32],
    )
    .bool()
}

// Translated from 00ad89e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::SetVolume` (Xbox PDB): passes the volume to the manager.
pub fn bs_sound_handle_set_volume(e: &mut Engine, this: Ptr<BSSoundHandle>, volume: f32) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    manager_call(e, this, MANAGER_SET_VOLUME, &[volume.to_bits()]).bool()
}

// Translated from 00ad8a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::GetVolume` (Xbox PDB): the manager's volume for the
/// sound (a float in ST0); 0.0 without a sound.
pub fn bs_sound_handle_get_volume(e: &mut Engine, this: Ptr<BSSoundHandle>) -> f32 {
    if has_no_sound(e, this) {
        return 0.0;
    }
    manager_call(e, this, MANAGER_GET_VOLUME, &[]).f32()
}

// Translated from 00ad8a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes a float to the manager (`00adb7c0`, unnamed in the map).
pub fn fn_00ad8a50(e: &mut Engine, this: Ptr<BSSoundHandle>, value: f32) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    manager_call(e, this, MANAGER_00ADB7C0, &[value.to_bits()]).bool()
}

// Translated from 00ad8a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::SetModFrequency` (Xbox PDB): passes the frequency
/// modifier to the manager.
pub fn bs_sound_handle_set_mod_frequency(
    e: &mut Engine,
    this: Ptr<BSSoundHandle>,
    frequency: f32,
) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    manager_call(e, this, MANAGER_SET_MOD_FREQUENCY, &[frequency.to_bits()]).bool()
}

// Translated from 00ad8ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes a byte to the manager (`00adb8e0`, unnamed in the map).
pub fn fn_00ad8ad0(e: &mut Engine, this: Ptr<BSSoundHandle>, value: u8) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    manager_call(e, this, MANAGER_00ADB8E0, &[value as u32]).bool()
}

// Translated from 00ad8b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes a word to the manager (`00adb960`, unnamed in the map).
pub fn fn_00ad8b00(e: &mut Engine, this: Ptr<BSSoundHandle>, value: u32) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    manager_call(e, this, MANAGER_00ADB960, &[value]).bool()
}

// Translated from 00ad8b30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::GetDuration` (Xbox PDB): the manager's answer (a word in
/// EAX) for the sound; 0 without a sound.
pub fn bs_sound_handle_get_duration(e: &mut Engine, this: Ptr<BSSoundHandle>) -> u32 {
    if has_no_sound(e, this) {
        return 0;
    }
    manager_call(e, this, MANAGER_GET_DURATION, &[]).u32()
}

// Translated from 00ad8b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::SetPosition` (Xbox PDB): passes the three coordinates to
/// the manager.
pub fn bs_sound_handle_set_position(
    e: &mut Engine,
    this: Ptr<BSSoundHandle>,
    x: f32,
    y: f32,
    z: f32,
) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    manager_call(
        e,
        this,
        MANAGER_SET_POSITION,
        &[x.to_bits(), y.to_bits(), z.to_bits()],
    )
    .bool()
}

// Translated from 00ad8ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes three floats to the manager (`00adb9f0`, unnamed in the map).
pub fn fn_00ad8ba0(e: &mut Engine, this: Ptr<BSSoundHandle>, x: f32, y: f32, z: f32) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    manager_call(
        e,
        this,
        MANAGER_00ADB9F0,
        &[x.to_bits(), y.to_bits(), z.to_bits()],
    )
    .bool()
}

// Translated from 00ad8be0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::SetMinMax` (Xbox PDB): passes the two distances to the
/// manager.
pub fn bs_sound_handle_set_min_max(e: &mut Engine, this: Ptr<BSSoundHandle>, min: f32, max: f32) {
    if has_no_sound(e, this) {
        return;
    }
    manager_call(
        e,
        this,
        MANAGER_SET_MIN_MAX,
        &[min.to_bits(), max.to_bits()],
    );
}

// Translated from 00ad8c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes five 16-bit values to the manager (`00adbb30`, unnamed in the
/// map).
pub fn fn_00ad8c20(
    e: &mut Engine,
    this: Ptr<BSSoundHandle>,
    value_1: u16,
    value_2: u16,
    value_3: u16,
    value_4: u16,
    value_5: u16,
) {
    if has_no_sound(e, this) {
        return;
    }
    manager_call(
        e,
        this,
        MANAGER_00ADBB30,
        &[
            value_1 as u32,
            value_2 as u32,
            value_3 as u32,
            value_4 as u32,
            value_5 as u32,
        ],
    );
}

// Translated from 00ad8c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes a 16-bit value to the manager (`00adbec0`, unnamed in the map).
pub fn fn_00ad8c70(e: &mut Engine, this: Ptr<BSSoundHandle>, value: u16) {
    if has_no_sound(e, this) {
        return;
    }
    manager_call(e, this, MANAGER_00ADBEC0, &[value as u32]);
}

// Translated from 00ad8ca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes three floats to the manager (`00adbf40`, unnamed in the map).
pub fn fn_00ad8ca0(e: &mut Engine, this: Ptr<BSSoundHandle>, x: f32, y: f32, z: f32) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    manager_call(
        e,
        this,
        MANAGER_00ADBF40,
        &[x.to_bits(), y.to_bits(), z.to_bits()],
    )
    .bool()
}

// Translated from 00ad8ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::IsValid` (Xbox PDB): false without a sound, else the
/// manager's answer.
pub fn bs_sound_handle_is_valid(e: &mut Engine, this: Ptr<BSSoundHandle>) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    manager_call(e, this, MANAGER_IS_VALID, &[]).bool()
}

// Translated from 00ad8d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::Release` (Xbox PDB): releases the sound in the manager;
/// when that succeeds the handle forgets the sound id (`-1`). Returns the
/// manager's answer.
pub fn bs_sound_handle_release(e: &mut Engine, this: Ptr<BSSoundHandle>) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    let released = manager_call(e, this, MANAGER_RELEASE, &[]).bool();
    if released {
        e.set(this, BSSoundHandle::iSoundID, NO_SOUND);
    }
    released
}

// Translated from 00ad8d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the state to playing, passes a word to the manager (`00adc670`,
/// unnamed in the map) and answers true; false without a sound.
pub fn fn_00ad8d60(e: &mut Engine, this: Ptr<BSSoundHandle>, value: u32) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    e.set(this, BSSoundHandle::eState, STATE_PLAYING);
    manager_call(e, this, MANAGER_00ADC670, &[value]);
    true
}

// Translated from 00ad8da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::FadeOutAndRelease` (Xbox PDB): fades the sound out over
/// the given time with the manager's second argument `0x26`, forgets the
/// sound id and answers true; false without a sound.
pub fn bs_sound_handle_fade_out_and_release(
    e: &mut Engine,
    this: Ptr<BSSoundHandle>,
    fade_time: u32,
) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    manager_call(e, this, MANAGER_FADE_OUT, &[fade_time, 0x26]);
    e.set(this, BSSoundHandle::iSoundID, NO_SOUND);
    true
}

// Translated from 00ad8de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::FadeOut` (Xbox PDB): sets the state to stopped, fades the
/// sound out over the given time (second argument 0) and answers true;
/// false without a sound.
pub fn bs_sound_handle_fade_out(e: &mut Engine, this: Ptr<BSSoundHandle>, fade_time: u32) -> bool {
    if has_no_sound(e, this) {
        return false;
    }
    e.set(this, BSSoundHandle::eState, STATE_STOPPED);
    manager_call(e, this, MANAGER_FADE_OUT, &[fade_time, 0]);
    true
}

// Translated from 00ad8e20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes two bytes to the manager (`00adc4e0`, unnamed in the map).
pub fn fn_00ad8e20(e: &mut Engine, this: Ptr<BSSoundHandle>, first: u8, second: u8) {
    if has_no_sound(e, this) {
        return;
    }
    manager_call(e, this, MANAGER_00ADC4E0, &[first as u32, second as u32]);
}

// Translated from 00ad8e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::SetCompletionCallback` (Xbox PDB): when the byte at +4
/// of the object at `011f6d98` is clear and a callback is given, calls the
/// cdecl callback with the argument and 0 at once; then, if the handle holds
/// a sound, hands callback and argument to the manager.
pub fn bs_sound_handle_set_completion_callback(
    e: &mut Engine,
    this: Ptr<BSSoundHandle>,
    callback: u32,
    callback_argument: u32,
) {
    callback_gate(e, callback, callback_argument, 0);
    if has_no_sound(e, this) {
        return;
    }
    manager_call(
        e,
        this,
        MANAGER_SET_COMPLETION_CALLBACK,
        &[callback, callback_argument],
    );
}

// Translated from 00ad8ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Like `SetCompletionCallback` (`00ad8e60`) but the immediate call passes
/// `0x1388` (5000) as the second argument and the manager method is
/// `00adc050` (unnamed in the map).
pub fn fn_00ad8ec0(
    e: &mut Engine,
    this: Ptr<BSSoundHandle>,
    callback: u32,
    callback_argument: u32,
) {
    callback_gate(e, callback, callback_argument, 0x1388);
    if has_no_sound(e, this) {
        return;
    }
    manager_call(e, this, MANAGER_00ADC050, &[callback, callback_argument]);
}

// Translated from 00ad8f20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::SetObjectToFollow` (Xbox PDB): nothing without a sound or
/// without an object; otherwise calls the cdecl
/// `pfnAddMovingSoundCallback` (when set) with the object, then hands the
/// object to the manager.
pub fn bs_sound_handle_set_object_to_follow(e: &mut Engine, this: Ptr<BSSoundHandle>, object: u32) {
    if has_no_sound(e, this) || object == 0 {
        return;
    }
    let callback = e.global::<u32>(ADD_MOVING_SOUND_CALLBACK);
    if callback != 0 {
        e.call(callback, &args![object]);
    }
    manager_call(e, this, MANAGER_SET_OBJECT_TO_FOLLOW, &[object]);
}

// Translated from 00ad8f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes a word to the manager (`00adc260`, unnamed in the map).
pub fn fn_00ad8f70(e: &mut Engine, this: Ptr<BSSoundHandle>, value: u32) {
    if has_no_sound(e, this) {
        return;
    }
    manager_call(e, this, MANAGER_00ADC260, &[value]);
}

// Translated from 00ad8fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Passes a byte to the manager (`00adc2e0`, unnamed in the map).
pub fn fn_00ad8fa0(e: &mut Engine, this: Ptr<BSSoundHandle>, value: u8) {
    if has_no_sound(e, this) {
        return;
    }
    manager_call(e, this, MANAGER_00ADC2E0, &[value as u32]);
}

// Translated from 00ad8fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::SetSynchStartTime` (Xbox PDB): passes the start-time word
/// to the manager.
pub fn bs_sound_handle_set_synch_start_time(
    e: &mut Engine,
    this: Ptr<BSSoundHandle>,
    start_time: u32,
) {
    if has_no_sound(e, this) {
        return;
    }
    manager_call(e, this, MANAGER_SET_SYNCH_START_TIME, &[start_time]);
}

// Translated from 00ad9000 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the manager (`00adc3e0`, unnamed in the map) with only the sound
/// id.
pub fn fn_00ad9000(e: &mut Engine, this: Ptr<BSSoundHandle>) {
    if has_no_sound(e, this) {
        return;
    }
    manager_call(e, this, MANAGER_00ADC3E0, &[]);
}

// Translated from 00ad9030 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSSoundHandle::SetPriority` (Xbox PDB): passes the priority to the
/// manager.
pub fn bs_sound_handle_set_priority(e: &mut Engine, this: Ptr<BSSoundHandle>, priority: u32) {
    if has_no_sound(e, this) {
        return;
    }
    manager_call(e, this, MANAGER_SET_PRIORITY, &[priority]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ad8830,
            bs_sound_handle_play(Ptr<BSSoundHandle>, u8) -> bool
        ),
        entry!(
            0x00ad8870,
            bs_sound_handle_play_after(Ptr<BSSoundHandle>, u32, u32) -> bool
        ),
        entry!(
            0x00ad88b0,
            bs_sound_handle_pause(Ptr<BSSoundHandle>) -> bool
        ),
        entry!(0x00ad88f0, bs_sound_handle_stop(Ptr<BSSoundHandle>) -> bool),
        entry!(
            0x00ad8930,
            bs_sound_handle_is_playing(Ptr<BSSoundHandle>) -> bool
        ),
        entry!(0x00ad8980, fn_00ad8980(Ptr<BSSoundHandle>) -> u32),
        entry!(
            0x00ad89b0,
            bs_sound_handle_set_static_attenuation(Ptr<BSSoundHandle>, u16) -> bool
        ),
        entry!(
            0x00ad89e0,
            bs_sound_handle_set_volume(Ptr<BSSoundHandle>, f32) -> bool
        ),
        entry!(
            0x00ad8a20,
            bs_sound_handle_get_volume(Ptr<BSSoundHandle>) -> f32
        ),
        entry!(0x00ad8a50, fn_00ad8a50(Ptr<BSSoundHandle>, f32) -> bool),
        entry!(
            0x00ad8a90,
            bs_sound_handle_set_mod_frequency(Ptr<BSSoundHandle>, f32) -> bool
        ),
        entry!(0x00ad8ad0, fn_00ad8ad0(Ptr<BSSoundHandle>, u8) -> bool),
        entry!(0x00ad8b00, fn_00ad8b00(Ptr<BSSoundHandle>, u32) -> bool),
        entry!(
            0x00ad8b30,
            bs_sound_handle_get_duration(Ptr<BSSoundHandle>) -> u32
        ),
        entry!(
            0x00ad8b60,
            bs_sound_handle_set_position(Ptr<BSSoundHandle>, f32, f32, f32) -> bool
        ),
        entry!(
            0x00ad8ba0,
            fn_00ad8ba0(Ptr<BSSoundHandle>, f32, f32, f32) -> bool
        ),
        entry!(
            0x00ad8be0,
            bs_sound_handle_set_min_max(Ptr<BSSoundHandle>, f32, f32)
        ),
        entry!(
            0x00ad8c20,
            fn_00ad8c20(Ptr<BSSoundHandle>, u16, u16, u16, u16, u16)
        ),
        entry!(0x00ad8c70, fn_00ad8c70(Ptr<BSSoundHandle>, u16)),
        entry!(
            0x00ad8ca0,
            fn_00ad8ca0(Ptr<BSSoundHandle>, f32, f32, f32) -> bool
        ),
        entry!(
            0x00ad8ce0,
            bs_sound_handle_is_valid(Ptr<BSSoundHandle>) -> bool
        ),
        entry!(
            0x00ad8d10,
            bs_sound_handle_release(Ptr<BSSoundHandle>) -> bool
        ),
        entry!(0x00ad8d60, fn_00ad8d60(Ptr<BSSoundHandle>, u32) -> bool),
        entry!(
            0x00ad8da0,
            bs_sound_handle_fade_out_and_release(Ptr<BSSoundHandle>, u32) -> bool
        ),
        entry!(
            0x00ad8de0,
            bs_sound_handle_fade_out(Ptr<BSSoundHandle>, u32) -> bool
        ),
        entry!(0x00ad8e20, fn_00ad8e20(Ptr<BSSoundHandle>, u8, u8)),
        entry!(
            0x00ad8e60,
            bs_sound_handle_set_completion_callback(Ptr<BSSoundHandle>, u32, u32)
        ),
        entry!(0x00ad8ec0, fn_00ad8ec0(Ptr<BSSoundHandle>, u32, u32)),
        entry!(
            0x00ad8f20,
            bs_sound_handle_set_object_to_follow(Ptr<BSSoundHandle>, u32)
        ),
        entry!(0x00ad8f70, fn_00ad8f70(Ptr<BSSoundHandle>, u32)),
        entry!(0x00ad8fa0, fn_00ad8fa0(Ptr<BSSoundHandle>, u8)),
        entry!(
            0x00ad8fd0,
            bs_sound_handle_set_synch_start_time(Ptr<BSSoundHandle>, u32)
        ),
        entry!(0x00ad9000, fn_00ad9000(Ptr<BSSoundHandle>)),
        entry!(
            0x00ad9030,
            bs_sound_handle_set_priority(Ptr<BSSoundHandle>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The manager `QInstance` hands out in the tests.
    const MANAGER: u32 = 0x0050_0000;
    /// The sound id the test handles hold.
    const SOUND: u32 = 0x77;

    /// An engine whose `QInstance` answers [`MANAGER`], whose manager
    /// `method` answers `ret`, and which logs calls; plus a handle holding
    /// [`SOUND`] (or no sound).
    fn fixture(method: u32, ret: Ret, has_sound: bool) -> (Engine, Ptr<BSSoundHandle>) {
        let mut e = Engine::new();
        e.register(AUDIO_MANAGER_INSTANCE, |_, _| Ret {
            eax: MANAGER,
            ..Ret::default()
        });
        e.register_double(method, move |_, _| ret);
        let handle: Ptr<BSSoundHandle> = e.new_object();
        e.set(
            handle,
            BSSoundHandle::iSoundID,
            if has_sound { SOUND } else { NO_SOUND },
        );
        e.call_log = Some(vec![]);
        (e, handle)
    }

    fn yes() -> Ret {
        Ret {
            eax: 1,
            ..Ret::default()
        }
    }

    /// The arguments the manager method received, if it was called.
    fn manager_args(e: &mut Engine, method: u32) -> Option<Vec<u32>> {
        let log = e.call_log.take().unwrap();
        log.into_iter()
            .find(|(addr, _)| *addr == method)
            .map(|(_, args)| args)
    }

    /// Without a sound the wrapper never reaches the manager.
    fn assert_not_forwarded(mut e: Engine, method: u32) {
        assert_eq!(manager_args(&mut e, method), None);
    }

    #[test]
    fn play_sets_state_and_forwards_flag() {
        let (mut e, h) = fixture(MANAGER_PLAY, yes(), true);
        assert!(e.call(0x00ad8830, &args![h, 0x1u8]).bool());
        assert_eq!(e.get(h, BSSoundHandle::eState), STATE_PLAYING);
        assert_eq!(
            manager_args(&mut e, MANAGER_PLAY),
            Some(vec![MANAGER, SOUND, 1])
        );
        let (mut e, h) = fixture(MANAGER_PLAY, yes(), false);
        assert!(!e.call(0x00ad8830, &args![h, 1u8]).bool());
        assert_eq!(e.get(h, BSSoundHandle::eState), 0);
        assert_not_forwarded(e, MANAGER_PLAY);
    }

    #[test]
    fn play_after_forwards_both_words() {
        let (mut e, h) = fixture(MANAGER_PLAY_AFTER, yes(), true);
        assert!(e.call(0x00ad8870, &args![h, 5u32, 6u32]).bool());
        assert_eq!(e.get(h, BSSoundHandle::eState), STATE_PLAYING);
        assert_eq!(
            manager_args(&mut e, MANAGER_PLAY_AFTER),
            Some(vec![MANAGER, SOUND, 5, 6])
        );
        let (mut e, h) = fixture(MANAGER_PLAY_AFTER, yes(), false);
        assert!(!e.call(0x00ad8870, &args![h, 5u32, 6u32]).bool());
        assert_not_forwarded(e, MANAGER_PLAY_AFTER);
    }

    #[test]
    fn pause_sets_paused() {
        let (mut e, h) = fixture(MANAGER_PAUSE, yes(), true);
        assert!(e.call(0x00ad88b0, &args![h]).bool());
        assert_eq!(e.get(h, BSSoundHandle::eState), STATE_PAUSED);
        assert_eq!(
            manager_args(&mut e, MANAGER_PAUSE),
            Some(vec![MANAGER, SOUND])
        );
        let (mut e, h) = fixture(MANAGER_PAUSE, yes(), false);
        assert!(!e.call(0x00ad88b0, &args![h]).bool());
        assert_not_forwarded(e, MANAGER_PAUSE);
    }

    #[test]
    fn stop_sets_stopped() {
        let (mut e, h) = fixture(MANAGER_STOP, yes(), true);
        assert!(e.call(0x00ad88f0, &args![h]).bool());
        assert_eq!(e.get(h, BSSoundHandle::eState), STATE_STOPPED);
        assert_eq!(
            manager_args(&mut e, MANAGER_STOP),
            Some(vec![MANAGER, SOUND])
        );
        let (mut e, h) = fixture(MANAGER_STOP, yes(), false);
        assert!(!e.call(0x00ad88f0, &args![h]).bool());
        assert_not_forwarded(e, MANAGER_STOP);
    }

    #[test]
    fn is_playing_assumes_state_or_asks_manager() {
        // The assumed-success flag answers from the state alone.
        let (mut e, h) = fixture(MANAGER_IS_PLAYING, yes(), true);
        e.set(h, BSSoundHandle::bAssumeSuccess, true);
        e.set(h, BSSoundHandle::eState, STATE_PLAYING);
        assert!(e.call(0x00ad8930, &args![h]).bool());
        e.set(h, BSSoundHandle::eState, STATE_PAUSED);
        assert!(!e.call(0x00ad8930, &args![h]).bool());
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(addr, _)| *addr != MANAGER_IS_PLAYING));
        // Otherwise the manager decides.
        let (mut e, h) = fixture(MANAGER_IS_PLAYING, yes(), true);
        assert!(e.call(0x00ad8930, &args![h]).bool());
        assert_eq!(
            manager_args(&mut e, MANAGER_IS_PLAYING),
            Some(vec![MANAGER, SOUND])
        );
        let (mut e, h) = fixture(MANAGER_IS_PLAYING, yes(), false);
        e.set(h, BSSoundHandle::bAssumeSuccess, true);
        assert!(!e.call(0x00ad8930, &args![h]).bool());
    }

    #[test]
    fn fn_00ad8980_returns_the_managers_word() {
        let word = Ret {
            eax: 0x1234,
            ..Ret::default()
        };
        let (mut e, h) = fixture(MANAGER_00ADC9A0, word, true);
        assert_eq!(e.call(0x00ad8980, &args![h]).u32(), 0x1234);
        assert_eq!(
            manager_args(&mut e, MANAGER_00ADC9A0),
            Some(vec![MANAGER, SOUND])
        );
        let (mut e, h) = fixture(MANAGER_00ADC9A0, word, false);
        assert_eq!(e.call(0x00ad8980, &args![h]).u32(), 0);
        assert_not_forwarded(e, MANAGER_00ADC9A0);
    }

    #[test]
    fn set_static_attenuation_forwards_the_value() {
        let (mut e, h) = fixture(MANAGER_SET_STATIC_ATTENUATION, yes(), true);
        assert!(e.call(0x00ad89b0, &args![h, 0x1234u16]).bool());
        assert_eq!(
            manager_args(&mut e, MANAGER_SET_STATIC_ATTENUATION),
            Some(vec![MANAGER, SOUND, 0x1234])
        );
        let (mut e, h) = fixture(MANAGER_SET_STATIC_ATTENUATION, yes(), false);
        assert!(!e.call(0x00ad89b0, &args![h, 5u16]).bool());
        assert_not_forwarded(e, MANAGER_SET_STATIC_ATTENUATION);
    }

    #[test]
    fn set_volume_forwards_the_float() {
        let (mut e, h) = fixture(MANAGER_SET_VOLUME, yes(), true);
        assert!(e.call(0x00ad89e0, &args![h, 0.5f32]).bool());
        assert_eq!(
            manager_args(&mut e, MANAGER_SET_VOLUME),
            Some(vec![MANAGER, SOUND, 0.5f32.to_bits()])
        );
        let (mut e, h) = fixture(MANAGER_SET_VOLUME, yes(), false);
        assert!(!e.call(0x00ad89e0, &args![h, 0.5f32]).bool());
        assert_not_forwarded(e, MANAGER_SET_VOLUME);
    }

    #[test]
    fn get_volume_reads_st0_or_zero() {
        let volume = Ret {
            st0: 0.75,
            ..Ret::default()
        };
        let (mut e, h) = fixture(MANAGER_GET_VOLUME, volume, true);
        assert_eq!(e.call(0x00ad8a20, &args![h]).f32(), 0.75);
        assert_eq!(
            manager_args(&mut e, MANAGER_GET_VOLUME),
            Some(vec![MANAGER, SOUND])
        );
        let (mut e, h) = fixture(MANAGER_GET_VOLUME, volume, false);
        assert_eq!(e.call(0x00ad8a20, &args![h]).f32(), 0.0);
        assert_not_forwarded(e, MANAGER_GET_VOLUME);
    }

    #[test]
    fn fn_00ad8a50_forwards_the_float() {
        let (mut e, h) = fixture(MANAGER_00ADB7C0, yes(), true);
        assert!(e.call(0x00ad8a50, &args![h, 2.5f32]).bool());
        assert_eq!(
            manager_args(&mut e, MANAGER_00ADB7C0),
            Some(vec![MANAGER, SOUND, 2.5f32.to_bits()])
        );
        let (mut e, h) = fixture(MANAGER_00ADB7C0, yes(), false);
        assert!(!e.call(0x00ad8a50, &args![h, 2.5f32]).bool());
        assert_not_forwarded(e, MANAGER_00ADB7C0);
    }

    #[test]
    fn set_mod_frequency_forwards_the_float() {
        let (mut e, h) = fixture(MANAGER_SET_MOD_FREQUENCY, yes(), true);
        assert!(e.call(0x00ad8a90, &args![h, 1.25f32]).bool());
        assert_eq!(
            manager_args(&mut e, MANAGER_SET_MOD_FREQUENCY),
            Some(vec![MANAGER, SOUND, 1.25f32.to_bits()])
        );
        let (mut e, h) = fixture(MANAGER_SET_MOD_FREQUENCY, yes(), false);
        assert!(!e.call(0x00ad8a90, &args![h, 1.25f32]).bool());
        assert_not_forwarded(e, MANAGER_SET_MOD_FREQUENCY);
    }

    #[test]
    fn fn_00ad8ad0_forwards_the_byte() {
        let (mut e, h) = fixture(MANAGER_00ADB8E0, yes(), true);
        assert!(e.call(0x00ad8ad0, &args![h, 0xabu8]).bool());
        assert_eq!(
            manager_args(&mut e, MANAGER_00ADB8E0),
            Some(vec![MANAGER, SOUND, 0xab])
        );
        let (mut e, h) = fixture(MANAGER_00ADB8E0, yes(), false);
        assert!(!e.call(0x00ad8ad0, &args![h, 1u8]).bool());
        assert_not_forwarded(e, MANAGER_00ADB8E0);
    }

    #[test]
    fn fn_00ad8b00_forwards_the_word() {
        let (mut e, h) = fixture(MANAGER_00ADB960, yes(), true);
        assert!(e.call(0x00ad8b00, &args![h, 0xdead_beefu32]).bool());
        assert_eq!(
            manager_args(&mut e, MANAGER_00ADB960),
            Some(vec![MANAGER, SOUND, 0xdead_beef])
        );
        let (mut e, h) = fixture(MANAGER_00ADB960, yes(), false);
        assert!(!e.call(0x00ad8b00, &args![h, 1u32]).bool());
        assert_not_forwarded(e, MANAGER_00ADB960);
    }

    #[test]
    fn get_duration_returns_the_managers_word() {
        let word = Ret {
            eax: 4321,
            ..Ret::default()
        };
        let (mut e, h) = fixture(MANAGER_GET_DURATION, word, true);
        assert_eq!(e.call(0x00ad8b30, &args![h]).u32(), 4321);
        assert_eq!(
            manager_args(&mut e, MANAGER_GET_DURATION),
            Some(vec![MANAGER, SOUND])
        );
        let (mut e, h) = fixture(MANAGER_GET_DURATION, word, false);
        assert_eq!(e.call(0x00ad8b30, &args![h]).u32(), 0);
        assert_not_forwarded(e, MANAGER_GET_DURATION);
    }

    #[test]
    fn set_position_forwards_x_y_z_in_order() {
        let (mut e, h) = fixture(MANAGER_SET_POSITION, yes(), true);
        assert!(e.call(0x00ad8b60, &args![h, 1.0f32, 2.0f32, 3.0f32]).bool());
        assert_eq!(
            manager_args(&mut e, MANAGER_SET_POSITION),
            Some(vec![
                MANAGER,
                SOUND,
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ])
        );
        let (mut e, h) = fixture(MANAGER_SET_POSITION, yes(), false);
        assert!(!e.call(0x00ad8b60, &args![h, 1.0f32, 2.0f32, 3.0f32]).bool());
        assert_not_forwarded(e, MANAGER_SET_POSITION);
    }

    #[test]
    fn fn_00ad8ba0_forwards_three_floats() {
        let (mut e, h) = fixture(MANAGER_00ADB9F0, yes(), true);
        assert!(e.call(0x00ad8ba0, &args![h, 4.0f32, 5.0f32, 6.0f32]).bool());
        assert_eq!(
            manager_args(&mut e, MANAGER_00ADB9F0),
            Some(vec![
                MANAGER,
                SOUND,
                4.0f32.to_bits(),
                5.0f32.to_bits(),
                6.0f32.to_bits()
            ])
        );
        let (mut e, h) = fixture(MANAGER_00ADB9F0, yes(), false);
        assert!(!e.call(0x00ad8ba0, &args![h, 4.0f32, 5.0f32, 6.0f32]).bool());
        assert_not_forwarded(e, MANAGER_00ADB9F0);
    }

    #[test]
    fn set_min_max_forwards_both_distances() {
        let (mut e, h) = fixture(MANAGER_SET_MIN_MAX, yes(), true);
        e.call(0x00ad8be0, &args![h, 10.0f32, 20.0f32]);
        assert_eq!(
            manager_args(&mut e, MANAGER_SET_MIN_MAX),
            Some(vec![MANAGER, SOUND, 10.0f32.to_bits(), 20.0f32.to_bits()])
        );
        let (mut e, h) = fixture(MANAGER_SET_MIN_MAX, yes(), false);
        e.call(0x00ad8be0, &args![h, 10.0f32, 20.0f32]);
        assert_not_forwarded(e, MANAGER_SET_MIN_MAX);
    }

    #[test]
    fn fn_00ad8c20_forwards_five_halfwords() {
        let (mut e, h) = fixture(MANAGER_00ADBB30, yes(), true);
        e.call(0x00ad8c20, &args![h, 1u16, 2u16, 3u16, 4u16, 0xffffu16]);
        assert_eq!(
            manager_args(&mut e, MANAGER_00ADBB30),
            Some(vec![MANAGER, SOUND, 1, 2, 3, 4, 0xffff])
        );
        let (mut e, h) = fixture(MANAGER_00ADBB30, yes(), false);
        e.call(0x00ad8c20, &args![h, 1u16, 2u16, 3u16, 4u16, 5u16]);
        assert_not_forwarded(e, MANAGER_00ADBB30);
    }

    #[test]
    fn fn_00ad8c70_forwards_the_halfword() {
        let (mut e, h) = fixture(MANAGER_00ADBEC0, yes(), true);
        e.call(0x00ad8c70, &args![h, 0x8001u16]);
        assert_eq!(
            manager_args(&mut e, MANAGER_00ADBEC0),
            Some(vec![MANAGER, SOUND, 0x8001])
        );
        let (mut e, h) = fixture(MANAGER_00ADBEC0, yes(), false);
        e.call(0x00ad8c70, &args![h, 1u16]);
        assert_not_forwarded(e, MANAGER_00ADBEC0);
    }

    #[test]
    fn fn_00ad8ca0_forwards_three_floats() {
        let (mut e, h) = fixture(MANAGER_00ADBF40, yes(), true);
        assert!(e.call(0x00ad8ca0, &args![h, 7.0f32, 8.0f32, 9.0f32]).bool());
        assert_eq!(
            manager_args(&mut e, MANAGER_00ADBF40),
            Some(vec![
                MANAGER,
                SOUND,
                7.0f32.to_bits(),
                8.0f32.to_bits(),
                9.0f32.to_bits()
            ])
        );
        let (mut e, h) = fixture(MANAGER_00ADBF40, yes(), false);
        assert!(!e.call(0x00ad8ca0, &args![h, 7.0f32, 8.0f32, 9.0f32]).bool());
        assert_not_forwarded(e, MANAGER_00ADBF40);
    }

    #[test]
    fn is_valid_asks_the_manager() {
        let (mut e, h) = fixture(MANAGER_IS_VALID, yes(), true);
        assert!(e.call(0x00ad8ce0, &args![h]).bool());
        assert_eq!(
            manager_args(&mut e, MANAGER_IS_VALID),
            Some(vec![MANAGER, SOUND])
        );
        let (mut e, h) = fixture(MANAGER_IS_VALID, yes(), false);
        assert!(!e.call(0x00ad8ce0, &args![h]).bool());
        assert_not_forwarded(e, MANAGER_IS_VALID);
    }

    #[test]
    fn release_forgets_the_id_only_on_success() {
        let (mut e, h) = fixture(MANAGER_RELEASE, yes(), true);
        assert!(e.call(0x00ad8d10, &args![h]).bool());
        assert_eq!(e.get(h, BSSoundHandle::iSoundID), NO_SOUND);
        assert_eq!(
            manager_args(&mut e, MANAGER_RELEASE),
            Some(vec![MANAGER, SOUND])
        );
        // The manager refuses: the id stays.
        let (mut e, h) = fixture(MANAGER_RELEASE, Ret::default(), true);
        assert!(!e.call(0x00ad8d10, &args![h]).bool());
        assert_eq!(e.get(h, BSSoundHandle::iSoundID), SOUND);
        let (mut e, h) = fixture(MANAGER_RELEASE, yes(), false);
        assert!(!e.call(0x00ad8d10, &args![h]).bool());
        assert_not_forwarded(e, MANAGER_RELEASE);
    }

    #[test]
    fn fn_00ad8d60_sets_playing_and_answers_true() {
        let (mut e, h) = fixture(MANAGER_00ADC670, Ret::default(), true);
        assert!(e.call(0x00ad8d60, &args![h, 99u32]).bool());
        assert_eq!(e.get(h, BSSoundHandle::eState), STATE_PLAYING);
        assert_eq!(
            manager_args(&mut e, MANAGER_00ADC670),
            Some(vec![MANAGER, SOUND, 99])
        );
        let (mut e, h) = fixture(MANAGER_00ADC670, yes(), false);
        assert!(!e.call(0x00ad8d60, &args![h, 99u32]).bool());
        assert_not_forwarded(e, MANAGER_00ADC670);
    }

    #[test]
    fn fade_out_and_release_uses_0x26_and_forgets_the_id() {
        let (mut e, h) = fixture(MANAGER_FADE_OUT, Ret::default(), true);
        assert!(e.call(0x00ad8da0, &args![h, 250u32]).bool());
        assert_eq!(e.get(h, BSSoundHandle::iSoundID), NO_SOUND);
        assert_eq!(
            manager_args(&mut e, MANAGER_FADE_OUT),
            Some(vec![MANAGER, SOUND, 250, 0x26])
        );
        let (mut e, h) = fixture(MANAGER_FADE_OUT, yes(), false);
        assert!(!e.call(0x00ad8da0, &args![h, 250u32]).bool());
        assert_not_forwarded(e, MANAGER_FADE_OUT);
    }

    #[test]
    fn fade_out_sets_stopped_and_keeps_the_id() {
        let (mut e, h) = fixture(MANAGER_FADE_OUT, Ret::default(), true);
        assert!(e.call(0x00ad8de0, &args![h, 250u32]).bool());
        assert_eq!(e.get(h, BSSoundHandle::eState), STATE_STOPPED);
        assert_eq!(e.get(h, BSSoundHandle::iSoundID), SOUND);
        assert_eq!(
            manager_args(&mut e, MANAGER_FADE_OUT),
            Some(vec![MANAGER, SOUND, 250, 0])
        );
        let (mut e, h) = fixture(MANAGER_FADE_OUT, yes(), false);
        assert!(!e.call(0x00ad8de0, &args![h, 250u32]).bool());
        assert_not_forwarded(e, MANAGER_FADE_OUT);
    }

    #[test]
    fn fn_00ad8e20_forwards_two_bytes() {
        let (mut e, h) = fixture(MANAGER_00ADC4E0, yes(), true);
        e.call(0x00ad8e20, &args![h, 0x11u8, 0x22u8]);
        assert_eq!(
            manager_args(&mut e, MANAGER_00ADC4E0),
            Some(vec![MANAGER, SOUND, 0x11, 0x22])
        );
        let (mut e, h) = fixture(MANAGER_00ADC4E0, yes(), false);
        e.call(0x00ad8e20, &args![h, 0x11u8, 0x22u8]);
        assert_not_forwarded(e, MANAGER_00ADC4E0);
    }

    /// Maps the page holding the gate object and the callback pointer, with
    /// the gate byte at +4 set to `gate`; returns nothing but registers a
    /// callback double at `CALLBACK` that logs its arguments.
    const CALLBACK: u32 = 0x0060_0000;
    fn gate_setup(e: &mut Engine, gate: u8) {
        let gate_object = e.mem.alloc(0x10);
        e.mem.set_u8(gate_object + 4, gate);
        e.map(CALLBACK_GATE_OBJECT & !0xfff, 0x1000);
        e.set_global(CALLBACK_GATE_OBJECT, gate_object);
        e.register(CALLBACK, |_, _| Ret::default());
    }

    #[test]
    fn set_completion_callback_calls_back_when_the_gate_is_clear() {
        let (mut e, h) = fixture(MANAGER_SET_COMPLETION_CALLBACK, Ret::default(), true);
        gate_setup(&mut e, 0);
        e.call(0x00ad8e60, &args![h, CALLBACK, 0x1111u32]);
        let log = e.call_log.take().unwrap();
        let immediate = log.iter().find(|(addr, _)| *addr == CALLBACK).unwrap();
        assert_eq!(immediate.1, vec![0x1111, 0]);
        let forwarded = log
            .iter()
            .find(|(addr, _)| *addr == MANAGER_SET_COMPLETION_CALLBACK)
            .unwrap();
        assert_eq!(forwarded.1, vec![MANAGER, SOUND, CALLBACK, 0x1111]);
    }

    #[test]
    fn set_completion_callback_skips_the_immediate_call_when_gated_or_null() {
        // Gate byte set: no immediate call, still forwarded.
        let (mut e, h) = fixture(MANAGER_SET_COMPLETION_CALLBACK, Ret::default(), true);
        gate_setup(&mut e, 1);
        e.call(0x00ad8e60, &args![h, CALLBACK, 0x1111u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(addr, _)| *addr != CALLBACK));
        assert!(log
            .iter()
            .any(|(addr, _)| *addr == MANAGER_SET_COMPLETION_CALLBACK));
        // No sound: the immediate call happens, nothing is forwarded.
        let (mut e, h) = fixture(MANAGER_SET_COMPLETION_CALLBACK, Ret::default(), false);
        gate_setup(&mut e, 0);
        e.call(0x00ad8e60, &args![h, CALLBACK, 0x2222u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(addr, _)| *addr == CALLBACK));
        assert!(log
            .iter()
            .all(|(addr, _)| *addr != MANAGER_SET_COMPLETION_CALLBACK));
    }

    #[test]
    fn fn_00ad8ec0_calls_back_with_5000() {
        let (mut e, h) = fixture(MANAGER_00ADC050, Ret::default(), true);
        gate_setup(&mut e, 0);
        e.call(0x00ad8ec0, &args![h, CALLBACK, 0x3333u32]);
        let log = e.call_log.take().unwrap();
        let immediate = log.iter().find(|(addr, _)| *addr == CALLBACK).unwrap();
        assert_eq!(immediate.1, vec![0x3333, 0x1388]);
        let forwarded = log
            .iter()
            .find(|(addr, _)| *addr == MANAGER_00ADC050)
            .unwrap();
        assert_eq!(forwarded.1, vec![MANAGER, SOUND, CALLBACK, 0x3333]);
        // A null callback is never called.
        let (mut e, h) = fixture(MANAGER_00ADC050, Ret::default(), true);
        gate_setup(&mut e, 0);
        e.call(0x00ad8ec0, &args![h, 0u32, 0x3333u32]);
        let log = e.call_log.take().unwrap();
        assert!(log.iter().all(|(addr, _)| *addr != CALLBACK));
    }

    #[test]
    fn set_object_to_follow_notifies_then_forwards() {
        let (mut e, h) = fixture(MANAGER_SET_OBJECT_TO_FOLLOW, Ret::default(), true);
        e.map(ADD_MOVING_SOUND_CALLBACK & !0xfff, 0x1000);
        e.set_global(ADD_MOVING_SOUND_CALLBACK, CALLBACK);
        e.register(CALLBACK, |_, _| Ret::default());
        e.call(0x00ad8f20, &args![h, 0x4444u32]);
        let log = e.call_log.take().unwrap();
        let notified: Vec<_> = log.iter().filter(|(addr, _)| *addr == CALLBACK).collect();
        assert_eq!(notified.len(), 1);
        assert_eq!(notified[0].1, vec![0x4444]);
        let forwarded = log
            .iter()
            .find(|(addr, _)| *addr == MANAGER_SET_OBJECT_TO_FOLLOW)
            .unwrap();
        assert_eq!(forwarded.1, vec![MANAGER, SOUND, 0x4444]);
    }

    #[test]
    fn set_object_to_follow_without_callback_object_or_sound() {
        // No callback pointer: forwarded only.
        let (mut e, h) = fixture(MANAGER_SET_OBJECT_TO_FOLLOW, Ret::default(), true);
        e.map(ADD_MOVING_SOUND_CALLBACK & !0xfff, 0x1000);
        e.call(0x00ad8f20, &args![h, 0x4444u32]);
        assert_eq!(
            manager_args(&mut e, MANAGER_SET_OBJECT_TO_FOLLOW),
            Some(vec![MANAGER, SOUND, 0x4444])
        );
        // Null object: nothing.
        let (mut e, h) = fixture(MANAGER_SET_OBJECT_TO_FOLLOW, Ret::default(), true);
        e.call(0x00ad8f20, &args![h, 0u32]);
        assert_not_forwarded(e, MANAGER_SET_OBJECT_TO_FOLLOW);
        // No sound: nothing.
        let (mut e, h) = fixture(MANAGER_SET_OBJECT_TO_FOLLOW, Ret::default(), false);
        e.call(0x00ad8f20, &args![h, 0x4444u32]);
        assert_not_forwarded(e, MANAGER_SET_OBJECT_TO_FOLLOW);
    }

    #[test]
    fn fn_00ad8f70_forwards_the_word() {
        let (mut e, h) = fixture(MANAGER_00ADC260, Ret::default(), true);
        e.call(0x00ad8f70, &args![h, 0x55u32]);
        assert_eq!(
            manager_args(&mut e, MANAGER_00ADC260),
            Some(vec![MANAGER, SOUND, 0x55])
        );
        let (mut e, h) = fixture(MANAGER_00ADC260, Ret::default(), false);
        e.call(0x00ad8f70, &args![h, 0x55u32]);
        assert_not_forwarded(e, MANAGER_00ADC260);
    }

    #[test]
    fn fn_00ad8fa0_forwards_the_byte() {
        let (mut e, h) = fixture(MANAGER_00ADC2E0, Ret::default(), true);
        e.call(0x00ad8fa0, &args![h, 0x66u8]);
        assert_eq!(
            manager_args(&mut e, MANAGER_00ADC2E0),
            Some(vec![MANAGER, SOUND, 0x66])
        );
        let (mut e, h) = fixture(MANAGER_00ADC2E0, Ret::default(), false);
        e.call(0x00ad8fa0, &args![h, 0x66u8]);
        assert_not_forwarded(e, MANAGER_00ADC2E0);
    }

    #[test]
    fn set_synch_start_time_forwards_the_word() {
        let (mut e, h) = fixture(MANAGER_SET_SYNCH_START_TIME, Ret::default(), true);
        e.call(0x00ad8fd0, &args![h, 0x77u32]);
        assert_eq!(
            manager_args(&mut e, MANAGER_SET_SYNCH_START_TIME),
            Some(vec![MANAGER, SOUND, 0x77])
        );
        let (mut e, h) = fixture(MANAGER_SET_SYNCH_START_TIME, Ret::default(), false);
        e.call(0x00ad8fd0, &args![h, 0x77u32]);
        assert_not_forwarded(e, MANAGER_SET_SYNCH_START_TIME);
    }

    #[test]
    fn fn_00ad9000_forwards_only_the_id() {
        let (mut e, h) = fixture(MANAGER_00ADC3E0, Ret::default(), true);
        e.call(0x00ad9000, &args![h]);
        assert_eq!(
            manager_args(&mut e, MANAGER_00ADC3E0),
            Some(vec![MANAGER, SOUND])
        );
        let (mut e, h) = fixture(MANAGER_00ADC3E0, Ret::default(), false);
        e.call(0x00ad9000, &args![h]);
        assert_not_forwarded(e, MANAGER_00ADC3E0);
    }

    #[test]
    fn set_priority_forwards_the_word() {
        let (mut e, h) = fixture(MANAGER_SET_PRIORITY, Ret::default(), true);
        e.call(0x00ad9030, &args![h, 9u32]);
        assert_eq!(
            manager_args(&mut e, MANAGER_SET_PRIORITY),
            Some(vec![MANAGER, SOUND, 9])
        );
        let (mut e, h) = fixture(MANAGER_SET_PRIORITY, Ret::default(), false);
        e.call(0x00ad9030, &args![h, 9u32]);
        assert_not_forwarded(e, MANAGER_SET_PRIORITY);
    }
}
