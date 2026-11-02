//! `BSAudio/bsaudio.cpp` (Xbox PDB source unit), subsystem `BSAudio`.
//! FalloutNV.exe 1.4.0.525 translations. The manager's PC volume fields
//! begin at +0x140; the Xbox PDB's corresponding fields begin at +0x190.

use crate::prelude::*;

const MANAGER_INSTANCE: u32 = 0x00ad_9060;
const MANAGER_VOLUME_CHANGED: u32 = 0x00ad_c160;
const AUDIO_TLS_CONTEXT: u32 = 0x2b4;
const AUDIO_CONTEXT_VALUE: u32 = 0xb;
const AUDIO_ENABLED_SETTING: u32 = 0x011f_6da0;
const MULTITHREAD_SETTING: u32 = 0x011f_6e00;
const DEFAULT_LISTENER_POSITION: u32 = 0x011f_426c;

layout! {
    /// `BSAudio` (Xbox PDB), with these offsets verified in the PC body.
    pub struct BSAudio: 0x38 {
        0x04 bAudioEnabled: u8,
        0x05 bInitialized: u8,
        0x06 bMultiThread: u8,
        0x10 pListener: Ptr,
        /// `SynchTimer.iFlags` (Xbox PDB), timer at +0x14.
        0x1c synchTimerFlags: u32,
        0x30 pfnSynchPausedCallback: u32,
        0x34 pfnSynchUnPausedCallback: u32,
    }
}

/// The normal-entry TLS context installed by the SEH-framed audio methods.
/// Exception cleanup of the frame is outside the engine memory model.
fn in_audio_context<R>(e: &mut Engine, f: impl FnOnce(&mut Engine) -> R) -> R {
    let at = e.tls() + AUDIO_TLS_CONTEXT;
    let prior = e.mem.u32(at);
    e.mem.set_u32(at, AUDIO_CONTEXT_VALUE);
    let result = f(e);
    e.mem.set_u32(at, prior);
    result
}

fn manager(e: &mut Engine) -> u32 {
    e.call(MANAGER_INSTANCE, &[]).u32()
}

/// PC offsets verified by `00ad79f0`–`00ad7e60`; Xbox PDB names at the
/// corresponding offsets are Master, Foot, Voice, Effects, Music, Radio and
/// their secondary counterparts. The `BSAudioManager` owner declares layout.
fn volume(e: &mut Engine, offset: u32) -> f32 {
    let p = manager(e);
    e.mem.f32(p + offset)
}

fn set_volume(e: &mut Engine, offset: u32, value: f32) {
    let p = manager(e);
    e.mem.set_f32(p + offset, value);
    e.call(MANAGER_VOLUME_CHANGED, &args![p]);
}

// Both dialogue wrappers pass the null smart pointer and point by value.
// The point is captured before the singleton getter, as the PC pushes show.
fn dialogue_command(e: &mut Engine, enabled: u32) {
    let x = e.global::<u32>(DEFAULT_LISTENER_POSITION);
    let y = e.global::<u32>(DEFAULT_LISTENER_POSITION + 4);
    let z = e.global::<u32>(DEFAULT_LISTENER_POSITION + 8);
    let p = manager(e);
    e.call(
        0x00ad_b1a0,
        &args![p, 13u32, enabled, 0u32, 0u32, 0u32, x, y, z],
    );
}

// Translated from 0070bbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::UnPauseAllOfType` (Xbox PDB): unpauses mask 0x40000000.
pub fn bs_audio_un_pause_all_of_type(e: &mut Engine, this: Ptr<BSAudio>) {
    fn_00ad8540(e, this, 0x4000_0000, 1);
}

// Translated from 00ad7020 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::Init` (Xbox PDB): applies the enable setting, initializes the
/// synchronization timer and starts the audio thread when selected.
// Translation incomplete: 00ad7020 - SEH exception unwinding is unsupported.
pub fn bs_audio_init(e: &mut Engine, this: Ptr<BSAudio>, _unused_1: u32) {
    in_audio_context(e, |e| {
        if e.global::<u8>(AUDIO_ENABLED_SETTING) == 0 {
            e.set(this, BSAudio::bAudioEnabled, 0);
            e.set(this, BSAudio::bInitialized, 0);
        } else {
            e.call(0x00ae_a150, &args![this.addr() + 0x14]);
            if e.get(this, BSAudio::bMultiThread) != 0 {
                let p = manager(e);
                e.call(0x00ad_9f30, &args![p]);
            }
            e.set(this, BSAudio::bInitialized, 1);
        }
    });
}

// Translated from 00ad7150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::Shutdown` (Xbox PDB): stops the audio thread and manager when
/// the audio instance's enabled byte is set.
// Translation incomplete: 00ad7150 - SEH exception unwinding is unsupported.
pub fn bs_audio_shutdown(e: &mut Engine, this: Ptr<BSAudio>) {
    in_audio_context(e, |e| {
        if e.get(this, BSAudio::bAudioEnabled) != 0 {
            if e.get(this, BSAudio::bMultiThread) != 0 {
                let p = manager(e);
                e.call(0x00ad_a0f0, &args![p]);
            }
            e.set(this, BSAudio::bInitialized, 0);
            let p = manager(e);
            e.call(0x00ad_9870, &args![p]);
        }
    });
}

// Translated from 00ad7230 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::SetMultiThreaded` (Xbox PDB): changes the audio thread state
/// only when audio is enabled and the setting permits enabling threads.
// Translation incomplete: 00ad7230 - SEH exception unwinding is unsupported.
pub fn bs_audio_set_multi_threaded(e: &mut Engine, this: Ptr<BSAudio>, enabled: u8) {
    in_audio_context(e, |e| {
        if e.get(this, BSAudio::bAudioEnabled) == 0
            || (e.global::<u8>(MULTITHREAD_SETTING) == 0 && enabled != 0)
        {
            return;
        }
        let current = e.get(this, BSAudio::bMultiThread);
        if enabled != 0 && current == 0 {
            let p = manager(e);
            e.call(0x00ad_9f30, &args![p]);
        } else if enabled == 0 && current != 0 {
            let p = manager(e);
            e.call(0x00ad_a020, &args![p]);
        }
        e.set(this, BSAudio::bMultiThread, enabled);
    });
}

// Translated from 00ad73b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::GetSoundHandleByNumericID` (Xbox PDB): fills and returns the
/// caller's 12-byte handle through the audio manager.
// Translation incomplete: 00ad73b0 - SEH exception unwinding is unsupported.
pub fn bs_audio_get_sound_handle_by_numeric_id(
    e: &mut Engine,
    _this: Ptr<BSAudio>,
    result: Ptr,
    numeric_id: u32,
    flags: u32,
) -> Ptr {
    in_audio_context(e, |e| {
        let p = manager(e);
        e.call(0x00ae_5870, &args![p, result, numeric_id, flags]);
    });
    result
}

// Translated from 00ad7480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::GetSoundHandleByFilename` (Xbox PDB): manager forwarding.
// Translation incomplete: 00ad7480 - SEH exception unwinding is unsupported.
pub fn bs_audio_get_sound_handle_by_filename(
    e: &mut Engine,
    _this: Ptr<BSAudio>,
    result: Ptr,
    filename: Ptr,
    flags: u32,
    extra: u32,
) -> Ptr {
    in_audio_context(e, |e| {
        let p = manager(e);
        e.call(0x00ae_5a50, &args![p, result, filename, flags, extra]);
    });
    result
}

// Translated from 00ad7550 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::GetSoundHandleByName` (Xbox PDB): manager forwarding.
// Translation incomplete: 00ad7550 - SEH exception unwinding is unsupported.
pub fn bs_audio_get_sound_handle_by_name(
    e: &mut Engine,
    _this: Ptr<BSAudio>,
    result: Ptr,
    name: Ptr,
    flags: u32,
) -> Ptr {
    in_audio_context(e, |e| {
        let p = manager(e);
        e.call(0x00ae_5680, &args![p, result, name, flags]);
    });
    result
}

// Translated from 00ad7620 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::SpawnSoundReference` (Xbox PDB): gets a numeric handle,
/// positions and plays it, then copies its three populated fields out.
// Translation incomplete: 00ad7620 - SEH exception unwinding is unsupported.
#[allow(clippy::too_many_arguments)] // The exe ABI has seven stack arguments.
pub fn bs_audio_spawn_sound_reference(
    e: &mut Engine,
    _this: Ptr<BSAudio>,
    result: Ptr,
    numeric_id: u32,
    flags: u32,
    x: f32,
    y: f32,
    z: f32,
    _unused_7: u32,
) -> Ptr {
    in_audio_context(e, |e| {
        e.with_stack(12, |e, handle| {
            let p = manager(e);
            e.call(0x00ae_5870, &args![p, handle, numeric_id, flags]);
            e.call(0x00ad_8b60, &args![handle, x, y, z]);
            e.call(0x00ad_8830, &args![handle, 1u32]);
            e.mem.set_u32(result.addr(), e.mem.u32(handle.addr()));
            e.mem.set_u8(result.addr() + 4, e.mem.u8(handle.addr() + 4));
            e.mem
                .set_u32(result.addr() + 8, e.mem.u32(handle.addr() + 8));
        });
    });
    result
}

// Translated from 00ad7740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the low byte to `BSAudioManager::Update` (Xbox PDB).
// Translation incomplete: 00ad7740 - SEH exception unwinding is unsupported.
pub fn fn_00ad7740(e: &mut Engine, _this: Ptr<BSAudio>, update_flag: u8) {
    in_audio_context(e, |e| {
        let p = manager(e);
        e.call(0x00ad_a1d0, &args![p, update_flag as u32]);
    });
}

// Translated from 00ad77f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards a float and word to the manager routine at `00ada300`.
// Translation incomplete: 00ad77f0 - SEH exception unwinding is unsupported.
pub fn fn_00ad77f0(e: &mut Engine, _this: Ptr<BSAudio>, value: f32, flag: u32) {
    in_audio_context(e, |e| {
        let p = manager(e);
        e.call(0x00ad_a300, &args![p, value, flag]);
    });
}

// Translated from 00ad78b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::PositionListener` (Xbox PDB): passes a point by value.
pub fn bs_audio_position_listener(e: &mut Engine, _this: Ptr<BSAudio>, x: f32, y: f32, z: f32) {
    let p = manager(e);
    e.call(0x00ad_ef80, &args![p, x, y, z]);
}

// Translated from 00ad78e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Copies the listener's virtual-position result, or the PC default point.
pub fn fn_00ad78e0(e: &mut Engine, this: Ptr<BSAudio>, result: Ptr) -> Ptr {
    let listener = e.get(this, BSAudio::pListener);
    let source = if listener.is_null() {
        DEFAULT_LISTENER_POSITION
    } else {
        e.vcall(listener.addr(), 0x0c, &[]).u32()
    };
    for offset in [0, 4, 8] {
        e.mem
            .set_u32(result.addr() + offset, e.mem.u32(source + offset));
    }
    result
}

// Translated from 00ad7940 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::OrientListener` (Xbox PDB): passes two by-value points as
/// addresses to the listener's `SetOrientation` virtual slot.
#[allow(clippy::too_many_arguments)] // The exe ABI passes six coordinate words.
pub fn bs_audio_orient_listener(
    e: &mut Engine,
    this: Ptr<BSAudio>,
    front_x: f32,
    front_y: f32,
    front_z: f32,
    top_x: f32,
    top_y: f32,
    top_z: f32,
) {
    let listener = e.get(this, BSAudio::pListener);
    if listener.is_null() {
        return;
    }
    e.with_stack(24, |e, points| {
        for (index, coordinate) in [front_x, front_y, front_z, top_x, top_y, top_z]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(points.addr() + index as u32 * 4, coordinate);
        }
        e.vcall(listener.addr(), 0x14, &args![points, points.addr() + 12]);
    });
}

// Translated from 00ad7990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::SetListenerUnderwater` (Xbox PDB): forwards the low byte.
pub fn bs_audio_set_listener_underwater(e: &mut Engine, _this: Ptr<BSAudio>, underwater: u8) {
    let p = manager(e);
    e.call(0x00ad_eff0, &args![p, underwater as u32]);
}

// Translated from 00ad79b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::SetEnvironment` (Xbox PDB): listener slot +0x28, if present.
pub fn bs_audio_set_environment(e: &mut Engine, this: Ptr<BSAudio>, environment: u32) {
    let listener = e.get(this, BSAudio::pListener);
    if !listener.is_null() {
        e.vcall(listener.addr(), 0x28, &args![environment]);
    }
}

// Translated from 00ad79f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the PC manager's master volume at +0x140 (Xbox PDB field).
pub fn fn_00ad79f0(e: &mut Engine, _this: Ptr<BSAudio>) -> f32 {
    volume(e, 0x140)
}

// Translated from 00ad7a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the PC manager's foot volume at +0x144 (Xbox PDB field).
pub fn fn_00ad7a20(e: &mut Engine, _this: Ptr<BSAudio>) -> f32 {
    volume(e, 0x144)
}

// Translated from 00ad7a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the PC manager's voice volume at +0x148 (Xbox PDB field).
pub fn fn_00ad7a50(e: &mut Engine, _this: Ptr<BSAudio>) -> f32 {
    volume(e, 0x148)
}

// Translated from 00ad7a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the PC manager's effects volume at +0x14c (Xbox PDB field).
pub fn fn_00ad7a80(e: &mut Engine, _this: Ptr<BSAudio>) -> f32 {
    volume(e, 0x14c)
}

// Translated from 00ad7ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the PC manager's music volume at +0x150 (Xbox PDB field).
pub fn fn_00ad7ab0(e: &mut Engine, _this: Ptr<BSAudio>) -> f32 {
    volume(e, 0x150)
}

// Translated from 00ad7ae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the PC manager's radio volume at +0x154 (Xbox PDB field).
pub fn bs_audio_get_radio_volume(e: &mut Engine, _this: Ptr<BSAudio>) -> f32 {
    volume(e, 0x154)
}

// Translated from 00ad7b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores master volume at PC +0x140 and notifies the manager.
pub fn fn_00ad7b10(e: &mut Engine, _this: Ptr<BSAudio>, value: f32) {
    set_volume(e, 0x140, value);
}

// Translated from 00ad7b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores foot volume at PC +0x144 and notifies the manager.
pub fn fn_00ad7b40(e: &mut Engine, _this: Ptr<BSAudio>, value: f32) {
    set_volume(e, 0x144, value);
}

// Translated from 00ad7b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores voice volume at PC +0x148 and notifies the manager.
pub fn fn_00ad7b70(e: &mut Engine, _this: Ptr<BSAudio>, value: f32) {
    set_volume(e, 0x148, value);
}

// Translated from 00ad7ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores effects volume at PC +0x14c and notifies the manager.
pub fn fn_00ad7ba0(e: &mut Engine, _this: Ptr<BSAudio>, value: f32) {
    set_volume(e, 0x14c, value);
}

// Translated from 00ad7bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores music volume at PC +0x150 and notifies the manager.
pub fn fn_00ad7bd0(e: &mut Engine, _this: Ptr<BSAudio>, value: f32) {
    set_volume(e, 0x150, value);
}

// Translated from 00ad7c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores radio volume at PC +0x154 and notifies the manager.
pub fn bs_audio_set_radio_volume(e: &mut Engine, _this: Ptr<BSAudio>, value: f32) {
    set_volume(e, 0x154, value);
}

// Translated from 00ad7c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `BSAudioManager::VolumeChanged` without changing a field.
pub fn fn_00ad7c30(e: &mut Engine, _this: Ptr<BSAudio>) {
    let p = manager(e);
    e.call(MANAGER_VOLUME_CHANGED, &args![p]);
}

// Translated from 00ad7c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the PC manager's listener-underwater byte at +0x175 (Xbox PDB;
/// the setter at `00adeff0` confirms the PC offset).
pub fn fn_00ad7c50(e: &mut Engine, _this: Ptr<BSAudio>) -> u8 {
    let p = manager(e);
    e.mem.u8(p + 0x175)
}

// Translated from 00ad7c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the PC manager's master secondary volume at +0x158 (Xbox PDB field).
pub fn fn_00ad7c80(e: &mut Engine, _this: Ptr<BSAudio>) -> f32 {
    volume(e, 0x158)
}

// Translated from 00ad7cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the PC manager's foot secondary volume at +0x15c (Xbox PDB field).
pub fn fn_00ad7cb0(e: &mut Engine, _this: Ptr<BSAudio>) -> f32 {
    volume(e, 0x15c)
}

// Translated from 00ad7ce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the PC manager's voice secondary volume at +0x160 (Xbox PDB field).
pub fn fn_00ad7ce0(e: &mut Engine, _this: Ptr<BSAudio>) -> f32 {
    volume(e, 0x160)
}

// Translated from 00ad7d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the PC manager's effects secondary volume at +0x164 (Xbox PDB field).
pub fn fn_00ad7d10(e: &mut Engine, _this: Ptr<BSAudio>) -> f32 {
    volume(e, 0x164)
}

// Translated from 00ad7d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the PC manager's music secondary volume at +0x168 (Xbox PDB field).
pub fn fn_00ad7d40(e: &mut Engine, _this: Ptr<BSAudio>) -> f32 {
    volume(e, 0x168)
}

// Translated from 00ad7d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Reads the PC manager's radio secondary volume at +0x16c (Xbox PDB field).
pub fn fn_00ad7d70(e: &mut Engine, _this: Ptr<BSAudio>) -> f32 {
    volume(e, 0x16c)
}

// Translated from 00ad7da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores master secondary volume at PC +0x158 and notifies the manager.
pub fn bs_audio_set_master_secondary_volume(e: &mut Engine, _this: Ptr<BSAudio>, value: f32) {
    set_volume(e, 0x158, value);
}

// Translated from 00ad7dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores foot secondary volume at PC +0x15c and notifies the manager.
pub fn fn_00ad7dd0(e: &mut Engine, _this: Ptr<BSAudio>, value: f32) {
    set_volume(e, 0x15c, value);
}

// Translated from 00ad7e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores voice secondary volume at PC +0x160 and notifies the manager.
pub fn fn_00ad7e00(e: &mut Engine, _this: Ptr<BSAudio>, value: f32) {
    set_volume(e, 0x160, value);
}

// Translated from 00ad7e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores effects secondary volume at PC +0x164 and notifies the manager.
pub fn fn_00ad7e30(e: &mut Engine, _this: Ptr<BSAudio>, value: f32) {
    set_volume(e, 0x164, value);
}

// Translated from 00ad7e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores music secondary volume at PC +0x168 and notifies the manager.
pub fn fn_00ad7e60(e: &mut Engine, _this: Ptr<BSAudio>, value: f32) {
    set_volume(e, 0x168, value);
}

// Translated from 00ad7e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the PC manager secondary radio volume and signals the change.
pub fn fn_00ad7e90(e: &mut Engine, _this: Ptr<BSAudio>, value: f32) {
    set_volume(e, 0x16c, value);
}

// Translated from 00ad7ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets all six secondary manager volumes.
pub fn fn_00ad7ec0(e: &mut Engine, _this: Ptr<BSAudio>) {
    let p = manager(e);
    fn_00ad87d0(e, Ptr::new(p));
}

// Translated from 00ad7ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::GetDebugString` (Xbox PDB): returns the manager list for a page.
// Translation incomplete: 00ad7ee0 - SEH exception unwinding is unsupported.
pub fn bs_audio_get_debug_string(e: &mut Engine, _this: Ptr<BSAudio>, page: u32) -> Ptr {
    in_audio_context(e, |e| {
        let p = manager(e);
        e.call(0x00ae5ff0, &args![p, page]).ptr()
    })
}

// Translated from 00ad7fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::Precache` (Xbox PDB): named precache request.
// Translation incomplete: 00ad7fa0 - SEH exception unwinding is unsupported.
pub fn bs_audio_precache(e: &mut Engine, _this: Ptr<BSAudio>, name: Ptr, flags: u32) {
    in_audio_context(e, |e| {
        let p = manager(e);
        e.call(0x00add570, &args![p, name, flags]);
    });
}

// Translated from 00ad8050 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::Precache_ov2` (Xbox PDB): numeric precache request.
// Translation incomplete: 00ad8050 - SEH exception unwinding is unsupported.
pub fn bs_audio_precache_ov2(e: &mut Engine, _this: Ptr<BSAudio>, numeric_id: u32, flags: u32) {
    in_audio_context(e, |e| {
        let p = manager(e);
        e.call(0x00add630, &args![p, numeric_id, flags]);
    });
}

// Translated from 00ad8100 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::Precache_ov3` (Xbox PDB): filename precache request.
// Translation incomplete: 00ad8100 - SEH exception unwinding is unsupported.
pub fn bs_audio_precache_ov3(
    e: &mut Engine,
    _this: Ptr<BSAudio>,
    filename: Ptr,
    flags: u32,
    extra: u32,
) {
    in_audio_context(e, |e| {
        let p = manager(e);
        e.call(0x00add6f0, &args![p, filename, flags, extra]);
    });
}

// Translated from 00ad81c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::SetAudioCacheSizeInK` (Xbox PDB): manager forwarding.
// Translation incomplete: 00ad81c0 - SEH exception unwinding is unsupported.
pub fn bs_audio_set_audio_cache_size_in_k(e: &mut Engine, _this: Ptr<BSAudio>, size: u32) {
    in_audio_context(e, |e| {
        let p = manager(e);
        e.call(0x00ae6bf0, &args![p, size]);
    });
}

// Translated from 00ad8270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the manager cache-size word.
// Translation incomplete: 00ad8270 - SEH exception unwinding is unsupported.
pub fn fn_00ad8270(e: &mut Engine, _this: Ptr<BSAudio>) -> u32 {
    in_audio_context(e, |e| {
        let p = manager(e);
        e.call(0x00ae6c60, &args![p]).u32()
    })
}

// Translated from 00ad8320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the low argument byte to the manager.
// Translation incomplete: 00ad8320 - SEH exception unwinding is unsupported.
pub fn fn_00ad8320(e: &mut Engine, _this: Ptr<BSAudio>, enabled: u8) {
    in_audio_context(e, |e| {
        let p = manager(e);
        e.call(0x00ae6ca0, &args![p, enabled]);
    });
}

// Translated from 00ad83d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the manager byte without boolean normalization.
// Translation incomplete: 00ad83d0 - SEH exception unwinding is unsupported.
pub fn fn_00ad83d0(e: &mut Engine, _this: Ptr<BSAudio>) -> u8 {
    in_audio_context(e, |e| {
        let p = manager(e);
        e.call(0x00ae6cf0, &args![p]).u32() as u8
    })
}

// Translated from 00ad8480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::MuteType` (Xbox PDB): forwards all three words.
pub fn bs_audio_mute_type(e: &mut Engine, _this: Ptr<BSAudio>, mask: u32, value: u32, extra: u32) {
    let p = manager(e);
    e.call(0x00adca20, &args![p, mask, value, extra]);
}

// Translated from 00ad84b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards both words to the manager.
pub fn fn_00ad84b0(e: &mut Engine, _this: Ptr<BSAudio>, mask: u32, value: u32) {
    let p = manager(e);
    e.call(0x00adcaa0, &args![p, mask, value]);
}

// Translated from 00ad84e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::KillAllOfType` (Xbox PDB): manager forwarding.
pub fn bs_audio_kill_all_of_type(e: &mut Engine, _this: Ptr<BSAudio>, mask: u32, value: u32) {
    let p = manager(e);
    e.call(0x00adcd00, &args![p, mask, value]);
}

// Translated from 00ad8510 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the mask and zero-extended low byte.
pub fn fn_00ad8510(e: &mut Engine, _this: Ptr<BSAudio>, mask: u32, enabled: u8) {
    let p = manager(e);
    e.call(0x00adcb20, &args![p, mask, enabled]);
}

// Translated from 00ad8540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Forwards the mask and zero-extended low byte.
pub fn fn_00ad8540(e: &mut Engine, _this: Ptr<BSAudio>, mask: u32, enabled: u8) {
    let p = manager(e);
    e.call(0x00adcc10, &args![p, mask, enabled]);
}

// Translated from 00ad8570 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::StopMovingSounds` (Xbox PDB): manager forwarding.
pub fn bs_audio_stop_moving_sounds(
    e: &mut Engine,
    _this: Ptr<BSAudio>,
    source: u32,
    value: f32,
    flags: u32,
) {
    let p = manager(e);
    e.call(0x00adb690, &args![p, source, value, flags]);
}

// Translated from 00ad85a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::EnterDialogue` (Xbox PDB): queues command 13.
// Translation incomplete: 00ad85a0 - SEH exception unwinding is unsupported.
pub fn bs_audio_enter_dialogue(e: &mut Engine, _this: Ptr<BSAudio>) {
    dialogue_command(e, 1);
}

// Translated from 00ad8650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::ExitDialogue` (Xbox PDB): queues command 13.
// Translation incomplete: 00ad8650 - SEH exception unwinding is unsupported.
pub fn bs_audio_exit_dialogue(e: &mut Engine, _this: Ptr<BSAudio>) {
    dialogue_command(e, 0);
}

// Translated from 00ad8700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the optional synchronization callback, then updates timer flag bit 0.
pub fn fn_00ad8700(e: &mut Engine, this: Ptr<BSAudio>) {
    let callback = e.get(this, BSAudio::pfnSynchPausedCallback);
    if callback != 0 {
        e.call(callback, &[]);
    }
    // The callback can change the flags; the PC reloads them afterwards.
    let flags = e.get(this, BSAudio::synchTimerFlags);
    e.set(this, BSAudio::synchTimerFlags, flags | 1);
}

// Translated from 00ad8740 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls the optional synchronization callback, then updates timer flag bit 0.
pub fn fn_00ad8740(e: &mut Engine, this: Ptr<BSAudio>) {
    let callback = e.get(this, BSAudio::pfnSynchUnPausedCallback);
    if callback != 0 {
        e.call(callback, &[]);
    }
    // The callback can change the flags; the PC reloads them afterwards.
    let flags = e.get(this, BSAudio::synchTimerFlags);
    e.set(this, BSAudio::synchTimerFlags, flags & !1);
}

// Translated from 00ad8780 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSAudio::EndAllSoundsOfType` (Xbox PDB): empty PC body, RET 4.
pub fn bs_audio_end_all_sounds_of_type(e: &mut Engine, _this: Ptr<BSAudio>, _unused_1: u32) {
    let _ = e;
}

// Translated from 00ad87d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets the six secondary volumes on a manager receiver.
pub fn fn_00ad87d0(e: &mut Engine, this: Ptr) {
    for offset in [0x158, 0x15c, 0x160, 0x164, 0x168, 0x16c] {
        // FLD1 is an instruction constant, not an executable data read.
        e.mem.set_f32(this.addr() + offset, 1.0);
    }
    e.call(MANAGER_VOLUME_CHANGED, &args![this]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0070bbc0, bs_audio_un_pause_all_of_type(Ptr<BSAudio>)),
        entry!(0x00ad7020, bs_audio_init(Ptr<BSAudio>, u32)),
        entry!(0x00ad7150, bs_audio_shutdown(Ptr<BSAudio>)),
        entry!(0x00ad7230, bs_audio_set_multi_threaded(Ptr<BSAudio>, u8)),
        entry!(
            0x00ad73b0,
            bs_audio_get_sound_handle_by_numeric_id(Ptr<BSAudio>, Ptr, u32, u32) -> Ptr
        ),
        entry!(
            0x00ad7480,
            bs_audio_get_sound_handle_by_filename(Ptr<BSAudio>, Ptr, Ptr, u32, u32) -> Ptr
        ),
        entry!(
            0x00ad7550,
            bs_audio_get_sound_handle_by_name(Ptr<BSAudio>, Ptr, Ptr, u32) -> Ptr
        ),
        entry!(
            0x00ad7620,
            bs_audio_spawn_sound_reference(Ptr<BSAudio>, Ptr, u32, u32, f32, f32, f32, u32) -> Ptr
        ),
        entry!(0x00ad7740, fn_00ad7740(Ptr<BSAudio>, u8)),
        entry!(0x00ad77f0, fn_00ad77f0(Ptr<BSAudio>, f32, u32)),
        entry!(
            0x00ad78b0,
            bs_audio_position_listener(Ptr<BSAudio>, f32, f32, f32)
        ),
        entry!(0x00ad78e0, fn_00ad78e0(Ptr<BSAudio>, Ptr) -> Ptr),
        entry!(
            0x00ad7940,
            bs_audio_orient_listener(Ptr<BSAudio>, f32, f32, f32, f32, f32, f32)
        ),
        entry!(
            0x00ad7990,
            bs_audio_set_listener_underwater(Ptr<BSAudio>, u8)
        ),
        entry!(0x00ad79b0, bs_audio_set_environment(Ptr<BSAudio>, u32)),
        entry!(0x00ad79f0, fn_00ad79f0(Ptr<BSAudio>) -> f32),
        entry!(0x00ad7a20, fn_00ad7a20(Ptr<BSAudio>) -> f32),
        entry!(0x00ad7a50, fn_00ad7a50(Ptr<BSAudio>) -> f32),
        entry!(0x00ad7a80, fn_00ad7a80(Ptr<BSAudio>) -> f32),
        entry!(0x00ad7ab0, fn_00ad7ab0(Ptr<BSAudio>) -> f32),
        entry!(0x00ad7ae0, bs_audio_get_radio_volume(Ptr<BSAudio>) -> f32),
        entry!(0x00ad7b10, fn_00ad7b10(Ptr<BSAudio>, f32)),
        entry!(0x00ad7b40, fn_00ad7b40(Ptr<BSAudio>, f32)),
        entry!(0x00ad7b70, fn_00ad7b70(Ptr<BSAudio>, f32)),
        entry!(0x00ad7ba0, fn_00ad7ba0(Ptr<BSAudio>, f32)),
        entry!(0x00ad7bd0, fn_00ad7bd0(Ptr<BSAudio>, f32)),
        entry!(0x00ad7c00, bs_audio_set_radio_volume(Ptr<BSAudio>, f32)),
        entry!(0x00ad7c30, fn_00ad7c30(Ptr<BSAudio>)),
        entry!(0x00ad7c50, fn_00ad7c50(Ptr<BSAudio>) -> u8),
        entry!(0x00ad7c80, fn_00ad7c80(Ptr<BSAudio>) -> f32),
        entry!(0x00ad7cb0, fn_00ad7cb0(Ptr<BSAudio>) -> f32),
        entry!(0x00ad7ce0, fn_00ad7ce0(Ptr<BSAudio>) -> f32),
        entry!(0x00ad7d10, fn_00ad7d10(Ptr<BSAudio>) -> f32),
        entry!(0x00ad7d40, fn_00ad7d40(Ptr<BSAudio>) -> f32),
        entry!(0x00ad7d70, fn_00ad7d70(Ptr<BSAudio>) -> f32),
        entry!(
            0x00ad7da0,
            bs_audio_set_master_secondary_volume(Ptr<BSAudio>, f32)
        ),
        entry!(0x00ad7dd0, fn_00ad7dd0(Ptr<BSAudio>, f32)),
        entry!(0x00ad7e00, fn_00ad7e00(Ptr<BSAudio>, f32)),
        entry!(0x00ad7e30, fn_00ad7e30(Ptr<BSAudio>, f32)),
        entry!(0x00ad7e60, fn_00ad7e60(Ptr<BSAudio>, f32)),
        entry!(0x00ad7e90, fn_00ad7e90(Ptr<BSAudio>, f32)),
        entry!(0x00ad7ec0, fn_00ad7ec0(Ptr<BSAudio>)),
        entry!(
            0x00ad7ee0,
            bs_audio_get_debug_string(Ptr<BSAudio>, u32) -> Ptr
        ),
        entry!(0x00ad7fa0, bs_audio_precache(Ptr<BSAudio>, Ptr, u32)),
        entry!(0x00ad8050, bs_audio_precache_ov2(Ptr<BSAudio>, u32, u32)),
        entry!(
            0x00ad8100,
            bs_audio_precache_ov3(Ptr<BSAudio>, Ptr, u32, u32)
        ),
        entry!(
            0x00ad81c0,
            bs_audio_set_audio_cache_size_in_k(Ptr<BSAudio>, u32)
        ),
        entry!(0x00ad8270, fn_00ad8270(Ptr<BSAudio>) -> u32),
        entry!(0x00ad8320, fn_00ad8320(Ptr<BSAudio>, u8)),
        entry!(0x00ad83d0, fn_00ad83d0(Ptr<BSAudio>) -> u8),
        entry!(0x00ad8480, bs_audio_mute_type(Ptr<BSAudio>, u32, u32, u32)),
        entry!(0x00ad84b0, fn_00ad84b0(Ptr<BSAudio>, u32, u32)),
        entry!(
            0x00ad84e0,
            bs_audio_kill_all_of_type(Ptr<BSAudio>, u32, u32)
        ),
        entry!(0x00ad8510, fn_00ad8510(Ptr<BSAudio>, u32, u8)),
        entry!(0x00ad8540, fn_00ad8540(Ptr<BSAudio>, u32, u8)),
        entry!(
            0x00ad8570,
            bs_audio_stop_moving_sounds(Ptr<BSAudio>, u32, f32, u32)
        ),
        entry!(0x00ad85a0, bs_audio_enter_dialogue(Ptr<BSAudio>)),
        entry!(0x00ad8650, bs_audio_exit_dialogue(Ptr<BSAudio>)),
        entry!(0x00ad8700, fn_00ad8700(Ptr<BSAudio>)),
        entry!(0x00ad8740, fn_00ad8740(Ptr<BSAudio>)),
        entry!(
            0x00ad8780,
            bs_audio_end_all_sounds_of_type(Ptr<BSAudio>, u32)
        ),
        entry!(0x00ad87d0, fn_00ad87d0(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANAGER: u32 = 0x0020_0000;
    const VTABLE: u32 = 0x0021_0000;
    const LISTENER: u32 = 0x0022_0000;
    const VIRTUAL_RETURN: u32 = 0x0023_0000;

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn setup() -> (Engine, Ptr<BSAudio>) {
        let mut e = Engine::new();
        let audio = e.new_object::<BSAudio>();
        e.map(MANAGER, 0x200);
        e.register(MANAGER_INSTANCE, |_, _| MANAGER.into_ret());
        (e, audio)
    }

    #[test]
    fn unpause_all_of_type_passes_mask_and_enable_word() {
        let (mut e, audio) = setup();
        e.register(0x00ad_cc10, noop);
        e.call_log = Some(vec![]);
        bs_audio_un_pause_all_of_type(&mut e, audio);
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (MANAGER_INSTANCE, vec![]),
                (0x00ad_cc10, args![MANAGER, 0x4000_0000u32, 1u32].to_vec())
            ]
        );
    }

    #[test]
    fn init_disabled_and_enabled_thread_paths_restore_tls() {
        let (mut e, audio) = setup();
        e.map(AUDIO_ENABLED_SETTING, 1);
        e.set(audio, BSAudio::bAudioEnabled, 1);
        e.set(audio, BSAudio::bInitialized, 1);
        let context = e.tls() + AUDIO_TLS_CONTEXT;
        e.mem.set_u32(context, 7);
        bs_audio_init(&mut e, audio, 0xdead_beef);
        assert_eq!(e.get(audio, BSAudio::bAudioEnabled), 0);
        assert_eq!(e.get(audio, BSAudio::bInitialized), 0);
        assert_eq!(e.mem.u32(context), 7);

        e.set_global(AUDIO_ENABLED_SETTING, 1u8);
        e.set(audio, BSAudio::bMultiThread, 0);
        e.register_double(0x00ae_a150, move |e, a| {
            let context = e.tls() + AUDIO_TLS_CONTEXT;
            assert_eq!(e.mem.u32(context), AUDIO_CONTEXT_VALUE);
            assert_eq!(a.len(), 1);
            // Init reads this byte after the timer call, not before it.
            e.set(audio, BSAudio::bMultiThread, 1);
            Ret::default()
        });
        e.register(0x00ad_9f30, noop);
        e.call_log = Some(vec![]);
        bs_audio_init(&mut e, audio, 9);
        assert_eq!(e.get(audio, BSAudio::bInitialized), 1);
        assert_eq!(e.mem.u32(context), 7);
        let log = e.call_log.unwrap();
        assert_eq!(log[0], (0x00ae_a150, args![audio.addr() + 0x14].to_vec()));
        assert_eq!(log[1], (MANAGER_INSTANCE, vec![]));
        assert_eq!(log[2], (0x00ad_9f30, args![MANAGER].to_vec()));
    }

    #[test]
    fn shutdown_stops_thread_before_manager_and_clears_initialized() {
        let (mut e, audio) = setup();
        e.set(audio, BSAudio::bAudioEnabled, 1);
        e.set(audio, BSAudio::bInitialized, 1);
        e.set(audio, BSAudio::bMultiThread, 1);
        e.register_double(0x00ad_a0f0, move |e, _| {
            e.set(audio, BSAudio::bInitialized, 2);
            Ret::default()
        });
        e.register_double(0x00ad_9870, move |e, _| {
            assert_eq!(e.get(audio, BSAudio::bInitialized), 0);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        bs_audio_shutdown(&mut e, audio);
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (MANAGER_INSTANCE, vec![]),
                (0x00ad_a0f0, args![MANAGER].to_vec()),
                (MANAGER_INSTANCE, vec![]),
                (0x00ad_9870, args![MANAGER].to_vec()),
            ]
        );
    }

    #[test]
    fn set_multi_threaded_honors_setting_and_transitions() {
        let (mut e, audio) = setup();
        e.map(MULTITHREAD_SETTING, 1);
        e.register_double(0x00ad_9f30, move |e, _| {
            e.set(audio, BSAudio::bMultiThread, 0x7f);
            Ret::default()
        });
        e.register_double(0x00ad_a020, move |e, _| {
            e.set(audio, BSAudio::bMultiThread, 0x7f);
            Ret::default()
        });
        bs_audio_set_multi_threaded(&mut e, audio, 1);
        assert_eq!(e.get(audio, BSAudio::bMultiThread), 0);
        e.set(audio, BSAudio::bAudioEnabled, 1);
        bs_audio_set_multi_threaded(&mut e, audio, 1);
        assert_eq!(e.get(audio, BSAudio::bMultiThread), 0);
        e.set_global(MULTITHREAD_SETTING, 1u8);
        e.call_log = Some(vec![]);
        bs_audio_set_multi_threaded(&mut e, audio, 1);
        assert_eq!(e.get(audio, BSAudio::bMultiThread), 1);
        bs_audio_set_multi_threaded(&mut e, audio, 0);
        assert_eq!(e.get(audio, BSAudio::bMultiThread), 0);
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (MANAGER_INSTANCE, vec![]),
                (0x00ad_9f30, args![MANAGER].to_vec()),
                (MANAGER_INSTANCE, vec![]),
                (0x00ad_a020, args![MANAGER].to_vec()),
            ]
        );
    }

    #[test]
    fn numeric_id_handle_forwards_three_words_and_returns_result() {
        let (mut e, audio) = setup();
        let result = e.mem.alloc(12);
        e.register(0x00ae_5870, noop);
        e.call_log = Some(vec![]);
        let got = bs_audio_get_sound_handle_by_numeric_id(&mut e, audio, Ptr::new(result), 17, 23);
        assert_eq!(got.addr(), result);
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (MANAGER_INSTANCE, vec![]),
                (0x00ae_5870, args![MANAGER, result, 17u32, 23u32].to_vec()),
            ]
        );
    }

    #[test]
    fn filename_handle_forwards_all_four_words() {
        let (mut e, audio) = setup();
        let result = Ptr::new(e.mem.alloc(12));
        e.register(0x00ae_5a50, noop);
        e.call_log = Some(vec![]);
        assert_eq!(
            bs_audio_get_sound_handle_by_filename(&mut e, audio, result, Ptr::new(0x1234), 5, 6),
            result
        );
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (MANAGER_INSTANCE, vec![]),
                (
                    0x00ae_5a50,
                    args![MANAGER, result, 0x1234u32, 5u32, 6u32].to_vec()
                ),
            ]
        );
    }

    #[test]
    fn named_handle_forwards_all_three_words() {
        let (mut e, audio) = setup();
        let result = Ptr::new(e.mem.alloc(12));
        e.register(0x00ae_5680, noop);
        e.call_log = Some(vec![]);
        assert_eq!(
            bs_audio_get_sound_handle_by_name(&mut e, audio, result, Ptr::new(0x2468), 7),
            result
        );
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (MANAGER_INSTANCE, vec![]),
                (
                    0x00ae_5680,
                    args![MANAGER, result, 0x2468u32, 7u32].to_vec()
                ),
            ]
        );
    }

    #[test]
    fn spawn_sound_copies_handle_after_position_and_play() {
        let (mut e, audio) = setup();
        let output = Ptr::new(e.mem.alloc(12));
        for offset in 5..8 {
            e.mem.set_u8(output.addr() + offset, 0xa0 + offset as u8);
        }
        e.register_double(0x00ae_5870, |e, a| {
            e.mem.set_u32(a[1], 0x11);
            e.mem.set_u8(a[1] + 4, 0x22);
            e.mem.set_u8(a[1] + 5, 0x99);
            e.mem.set_u32(a[1] + 8, 0x33);
            Ret::default()
        });
        e.register_double(0x00ad_8b60, |e, a| {
            assert_eq!(
                &a[1..],
                &[1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits()]
            );
            e.mem.set_u8(a[0] + 4, 0x44);
            Ret::default()
        });
        e.register_double(0x00ad_8830, |e, a| {
            assert_eq!(a[1], 1);
            e.mem.set_u32(a[0] + 8, 0x55);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            bs_audio_spawn_sound_reference(&mut e, audio, output, 9, 10, 1.0, 2.0, 3.0, 99),
            output
        );
        assert_eq!(e.mem.u32(output.addr()), 0x11);
        assert_eq!(e.mem.u8(output.addr() + 4), 0x44);
        for offset in 5..8 {
            assert_eq!(e.mem.u8(output.addr() + offset), 0xa0 + offset as u8);
        }
        assert_eq!(e.mem.u32(output.addr() + 8), 0x55);
        let log = e.call_log.unwrap();
        assert_eq!(
            log.iter().map(|x| x.0).collect::<Vec<_>>(),
            vec![MANAGER_INSTANCE, 0x00ae_5870, 0x00ad_8b60, 0x00ad_8830]
        );
        assert_eq!(&log[1].1[2..], &[9, 10]);
    }

    #[test]
    fn manager_update_forwards_low_byte() {
        let (mut e, audio) = setup();
        e.register(0x00ad_a1d0, noop);
        e.call_log = Some(vec![]);
        fn_00ad7740(&mut e, audio, 0x92);
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (MANAGER_INSTANCE, vec![]),
                (0x00ad_a1d0, args![MANAGER, 0x92u32].to_vec()),
            ]
        );
    }

    #[test]
    fn manager_float_word_forwarding_preserves_bits_and_order() {
        let (mut e, audio) = setup();
        e.register(0x00ad_a300, noop);
        e.call_log = Some(vec![]);
        fn_00ad77f0(&mut e, audio, -0.25, 0x1234);
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (MANAGER_INSTANCE, vec![]),
                (0x00ad_a300, args![MANAGER, -0.25f32, 0x1234u32].to_vec()),
            ]
        );
    }

    #[test]
    fn position_listener_passes_point_by_value() {
        let (mut e, audio) = setup();
        e.register(0x00ad_ef80, noop);
        e.call_log = Some(vec![]);
        bs_audio_position_listener(&mut e, audio, -1.0, 2.5, 7.0);
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (MANAGER_INSTANCE, vec![]),
                (
                    0x00ad_ef80,
                    args![MANAGER, -1.0f32, 2.5f32, 7.0f32].to_vec()
                ),
            ]
        );
    }

    #[test]
    fn listener_position_uses_default_when_absent_and_virtual_when_present() {
        let (mut e, audio) = setup();
        e.map(DEFAULT_LISTENER_POSITION, 12);
        let result = Ptr::new(e.mem.alloc(12));
        for i in 0..3 {
            e.mem.set_u32(DEFAULT_LISTENER_POSITION + i * 4, i + 10);
        }
        assert_eq!(fn_00ad78e0(&mut e, audio, result), result);
        assert_eq!(
            (0..3)
                .map(|i| e.mem.u32(result.addr() + i * 4))
                .collect::<Vec<_>>(),
            vec![10, 11, 12]
        );

        e.map(LISTENER, 4);
        e.put_vtable(VTABLE, &[0, 0, 0, VIRTUAL_RETURN]);
        e.mem.set_u32(LISTENER, VTABLE);
        e.set(audio, BSAudio::pListener, Ptr::new(LISTENER));
        e.map(VIRTUAL_RETURN, 12);
        for i in 0..3 {
            e.mem.set_u32(VIRTUAL_RETURN + i * 4, i + 20);
        }
        e.register(VIRTUAL_RETURN, |_, _| VIRTUAL_RETURN.into_ret());
        // Repoint the virtual slot at a test double and retain a separate point.
        e.mem.set_u32(VTABLE + 0x0c, VIRTUAL_RETURN);
        assert_eq!(fn_00ad78e0(&mut e, audio, result), result);
        assert_eq!(
            (0..3)
                .map(|i| e.mem.u32(result.addr() + i * 4))
                .collect::<Vec<_>>(),
            vec![20, 21, 22]
        );
    }

    #[test]
    fn orient_listener_passes_two_points_to_virtual_slot() {
        let (mut e, audio) = setup();
        e.map(LISTENER, 4);
        e.put_vtable(VTABLE, &[0, 0, 0, 0, 0, VIRTUAL_RETURN]);
        e.mem.set_u32(LISTENER, VTABLE);
        e.set(audio, BSAudio::pListener, Ptr::new(LISTENER));
        e.register_double(VIRTUAL_RETURN, |e, a| {
            assert_eq!(a[0], LISTENER);
            assert_eq!(
                (0..3).map(|i| e.mem.f32(a[1] + i * 4)).collect::<Vec<_>>(),
                vec![1.0, 2.0, 3.0]
            );
            assert_eq!(
                (0..3).map(|i| e.mem.f32(a[2] + i * 4)).collect::<Vec<_>>(),
                vec![4.0, 5.0, 6.0]
            );
            Ret::default()
        });
        e.call_log = Some(vec![]);
        bs_audio_orient_listener(&mut e, audio, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
        assert_eq!(e.call_log.unwrap().len(), 1);
    }

    #[test]
    fn underwater_setting_forwards_low_byte() {
        let (mut e, audio) = setup();
        e.register(0x00ad_eff0, noop);
        e.call_log = Some(vec![]);
        bs_audio_set_listener_underwater(&mut e, audio, 0x80);
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (MANAGER_INSTANCE, vec![]),
                (0x00ad_eff0, args![MANAGER, 0x80u32].to_vec()),
            ]
        );
    }

    #[test]
    fn environment_uses_listener_virtual_only_when_present() {
        let (mut e, audio) = setup();
        e.call_log = Some(vec![]);
        bs_audio_set_environment(&mut e, audio, 13);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        e.map(LISTENER, 4);
        e.put_vtable(VTABLE, &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, VIRTUAL_RETURN]);
        e.mem.set_u32(LISTENER, VTABLE);
        e.set(audio, BSAudio::pListener, Ptr::new(LISTENER));
        e.register(VIRTUAL_RETURN, noop);
        bs_audio_set_environment(&mut e, audio, 13);
        assert_eq!(
            e.call_log.unwrap(),
            vec![(VIRTUAL_RETURN, args![LISTENER, 13u32].to_vec())]
        );
    }

    macro_rules! getter_case {
        ($name:ident, $address:expr, $offset:expr) => {
            #[test]
            fn $name() {
                let (mut e, audio) = setup();
                e.mem.set_f32(MANAGER + $offset, -1.25);
                e.call_log = Some(vec![]);
                assert_eq!(e.call($address, &args![audio]).f32(), -1.25);
                assert_eq!(
                    e.call_log.unwrap(),
                    vec![
                        ($address, args![audio].to_vec()),
                        (MANAGER_INSTANCE, vec![]),
                    ]
                );
            }
        };
    }

    macro_rules! setter_case {
        ($name:ident, $address:expr, $offset:expr) => {
            #[test]
            fn $name() {
                let (mut e, audio) = setup();
                e.register_double(MANAGER_VOLUME_CHANGED, |e, a| {
                    assert_eq!(a, &[MANAGER]);
                    assert_eq!(e.mem.f32(MANAGER + $offset).to_bits(), (-1.25f32).to_bits());
                    Ret::default()
                });
                e.call_log = Some(vec![]);
                e.call($address, &args![audio, -1.25f32]);
                assert_eq!(
                    e.call_log.unwrap(),
                    vec![
                        ($address, args![audio, -1.25f32].to_vec()),
                        (MANAGER_INSTANCE, vec![]),
                        (MANAGER_VOLUME_CHANGED, args![MANAGER].to_vec()),
                    ]
                );
            }
        };
    }

    getter_case!(get_master_volume, 0x00ad_79f0, 0x140);
    getter_case!(get_foot_volume, 0x00ad_7a20, 0x144);
    getter_case!(get_voice_volume, 0x00ad_7a50, 0x148);
    getter_case!(get_effects_volume, 0x00ad_7a80, 0x14c);
    getter_case!(get_music_volume, 0x00ad_7ab0, 0x150);
    getter_case!(get_radio_volume, 0x00ad_7ae0, 0x154);
    setter_case!(set_master_volume, 0x00ad_7b10, 0x140);
    setter_case!(set_foot_volume, 0x00ad_7b40, 0x144);
    setter_case!(set_voice_volume, 0x00ad_7b70, 0x148);
    setter_case!(set_effects_volume, 0x00ad_7ba0, 0x14c);
    setter_case!(set_music_volume, 0x00ad_7bd0, 0x150);
    setter_case!(set_radio_volume, 0x00ad_7c00, 0x154);

    #[test]
    fn volume_changed_without_new_value() {
        let (mut e, audio) = setup();
        e.register(MANAGER_VOLUME_CHANGED, noop);
        e.call_log = Some(vec![]);
        fn_00ad7c30(&mut e, audio);
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (MANAGER_INSTANCE, vec![]),
                (MANAGER_VOLUME_CHANGED, args![MANAGER].to_vec()),
            ]
        );
    }

    #[test]
    fn manager_byte_at_175_is_returned_without_normalization() {
        let (mut e, audio) = setup();
        e.mem.set_u8(MANAGER + 0x175, 0x80);
        assert_eq!(fn_00ad7c50(&mut e, audio), 0x80);
    }

    getter_case!(get_master_secondary_volume, 0x00ad_7c80, 0x158);
    getter_case!(get_foot_secondary_volume, 0x00ad_7cb0, 0x15c);
    getter_case!(get_voice_secondary_volume, 0x00ad_7ce0, 0x160);
    getter_case!(get_effects_secondary_volume, 0x00ad_7d10, 0x164);
    getter_case!(get_music_secondary_volume, 0x00ad_7d40, 0x168);
    getter_case!(get_radio_secondary_volume, 0x00ad_7d70, 0x16c);
    setter_case!(set_master_secondary_volume, 0x00ad_7da0, 0x158);
    setter_case!(set_foot_secondary_volume, 0x00ad_7dd0, 0x15c);
    setter_case!(set_voice_secondary_volume, 0x00ad_7e00, 0x160);
    setter_case!(set_effects_secondary_volume, 0x00ad_7e30, 0x164);
    setter_case!(set_music_secondary_volume, 0x00ad_7e60, 0x168);

    setter_case!(set_radio_secondary_volume, 0x00ad_7e90, 0x16c);

    fn check_secondary_reset(entry: u32, receiver: u32) {
        let (mut e, _) = setup();
        for offset in (0x140..=0x174).step_by(4) {
            e.mem.set_u32(MANAGER + offset, 0xaabb_ccdd);
        }
        e.register_double(MANAGER_VOLUME_CHANGED, |e, a| {
            assert_eq!(a, &[MANAGER]);
            for offset in (0x158..=0x16c).step_by(4) {
                assert_eq!(e.mem.f32(MANAGER + offset), 1.0);
            }
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(entry, &[receiver]);
        for offset in [0x140, 0x144, 0x148, 0x14c, 0x150, 0x154, 0x170, 0x174] {
            assert_eq!(e.mem.u32(MANAGER + offset), 0xaabb_ccdd);
        }
        let mut expected = vec![(entry, vec![receiver])];
        if entry == 0x00ad_7ec0 {
            expected.push((MANAGER_INSTANCE, vec![]));
        }
        expected.push((MANAGER_VOLUME_CHANGED, vec![MANAGER]));
        assert_eq!(e.call_log.unwrap(), expected);
    }

    #[test]
    fn reset_secondary_volumes_obtains_manager() {
        check_secondary_reset(0x00ad_7ec0, 0);
    }

    #[test]
    fn reset_secondary_volumes_uses_manager_receiver_and_preserves_neighbors() {
        check_secondary_reset(0x00ad_87d0, MANAGER);
    }

    fn check_forward(
        entry: u32,
        callee: u32,
        input: &[u32],
        output: &[u32],
        tls: bool,
        result: u32,
    ) {
        let (mut e, audio) = setup();
        let context = e.tls() + AUDIO_TLS_CONTEXT;
        e.mem.set_u32(context, 0x1234_5678);
        e.register_double(MANAGER_INSTANCE, move |e, a| {
            assert!(a.is_empty()); // Pending stack arguments belong to the manager method.
            assert_eq!(
                e.mem.u32(context),
                if tls {
                    AUDIO_CONTEXT_VALUE
                } else {
                    0x1234_5678
                }
            );
            MANAGER.into_ret()
        });
        let expected = [&[MANAGER], output].concat();
        e.register_double(callee, move |e, a| {
            assert_eq!(a, expected);
            assert_eq!(
                e.mem.u32(context),
                if tls {
                    AUDIO_CONTEXT_VALUE
                } else {
                    0x1234_5678
                }
            );
            if tls {
                e.mem.set_u32(context, 0x8765_4321);
            }
            0xfeed_be80u32.into_ret()
        });
        let arguments = [&[audio.addr()], input].concat();
        e.call_log = Some(vec![]);
        assert_eq!(e.call(entry, &arguments).u32(), result);
        assert_eq!(e.mem.u32(context), 0x1234_5678);
        assert_eq!(
            e.call_log.unwrap(),
            vec![
                (entry, arguments),
                (MANAGER_INSTANCE, vec![]),
                (callee, [&[MANAGER], output].concat()),
            ]
        );
    }

    macro_rules! forward_case {
        ($name:ident, $entry:expr, $callee:expr, $input:expr, $output:expr, $tls:expr, $result:expr) => {
            #[test]
            fn $name() {
                check_forward($entry, $callee, $input, $output, $tls, $result);
            }
        };
    }

    forward_case!(
        debug_string_returns_manager_pointer_and_restores_tls,
        0x00ad7ee0,
        0x00ae5ff0,
        &[0x1234],
        &[0x1234],
        true,
        0xfeed_be80
    );
    forward_case!(
        precache_name_forwards_two_words_and_restores_tls,
        0x00ad7fa0,
        0x00add570,
        &[0x1234, 0x89ab_cdef],
        &[0x1234, 0x89ab_cdef],
        true,
        0
    );
    forward_case!(
        precache_numeric_forwards_two_words_and_restores_tls,
        0x00ad8050,
        0x00add630,
        &[0x1234, 0x89ab_cdef],
        &[0x1234, 0x89ab_cdef],
        true,
        0
    );
    forward_case!(
        precache_filename_forwards_three_words_and_restores_tls,
        0x00ad8100,
        0x00add6f0,
        &[0x1234, 0x89ab_cdef, 0x4567],
        &[0x1234, 0x89ab_cdef, 0x4567],
        true,
        0
    );
    forward_case!(
        cache_size_setter_forwards_word_and_restores_tls,
        0x00ad81c0,
        0x00ae6bf0,
        &[0xffff_ffff],
        &[0xffff_ffff],
        true,
        0
    );
    forward_case!(
        cache_size_getter_returns_word_and_restores_tls,
        0x00ad8270,
        0x00ae6c60,
        &[],
        &[],
        true,
        0xfeed_be80
    );
    forward_case!(
        manager_byte_setter_narrows_before_forwarding_and_restores_tls,
        0x00ad8320,
        0x00ae6ca0,
        &[0xaabb_cc80],
        &[0x80],
        true,
        0
    );
    forward_case!(
        manager_byte_getter_preserves_non_boolean_low_byte,
        0x00ad83d0,
        0x00ae6cf0,
        &[],
        &[],
        true,
        0x80
    );
    forward_case!(
        mute_type_forwards_three_words,
        0x00ad8480,
        0x00adca20,
        &[0x1234, 0x89ab_cdef, 0x4567],
        &[0x1234, 0x89ab_cdef, 0x4567],
        false,
        0
    );
    forward_case!(
        manager_type_action_forwards_two_words,
        0x00ad84b0,
        0x00adcaa0,
        &[0x1234, 0x89ab_cdef],
        &[0x1234, 0x89ab_cdef],
        false,
        0
    );
    forward_case!(
        kill_type_forwards_two_words,
        0x00ad84e0,
        0x00adcd00,
        &[0x1234, 0x89ab_cdef],
        &[0x1234, 0x89ab_cdef],
        false,
        0
    );
    forward_case!(
        pause_type_narrows_second_word,
        0x00ad8510,
        0x00adcb20,
        &[0x1234, 0x89ab_cd80],
        &[0x1234, 0x80],
        false,
        0
    );
    forward_case!(
        unpause_type_narrows_second_word,
        0x00ad8540,
        0x00adcc10,
        &[0x1234, 0x89ab_cd80],
        &[0x1234, 0x80],
        false,
        0
    );
    forward_case!(
        stop_moving_sounds_passes_float_bits_and_other_words,
        0x00ad8570,
        0x00adb690,
        &[0x1234, (-1.25f32).to_bits(), 0x4567],
        &[0x1234, (-1.25f32).to_bits(), 0x4567],
        false,
        0
    );

    fn check_dialogue(entry: u32, enabled: u32) {
        let (mut e, audio) = setup();
        let context = e.tls() + AUDIO_TLS_CONTEXT;
        e.mem.set_u32(context, 0x37);
        e.map(DEFAULT_LISTENER_POSITION, 12);
        for (offset, bits) in [(0, 0x8000_0000u32), (4, 0x7fc1_2345), (8, 0x4321_1234)] {
            e.set_global(DEFAULT_LISTENER_POSITION + offset, bits);
        }
        e.register_double(MANAGER_INSTANCE, move |e, a| {
            assert!(a.is_empty());
            assert_eq!(e.mem.u32(context), 0x37); // No TLS guard in these two wrappers.
            e.set_global(DEFAULT_LISTENER_POSITION, 0x1111_2222u32);
            e.set_global(DEFAULT_LISTENER_POSITION + 4, 0x3333_4444u32);
            e.set_global(DEFAULT_LISTENER_POSITION + 8, 0x5555_6666u32);
            MANAGER.into_ret()
        });
        e.register_double(0x00ad_b1a0, move |e, a| {
            assert_eq!(
                a,
                &[
                    MANAGER,
                    13,
                    enabled,
                    0,
                    0,
                    0,
                    0x8000_0000,
                    0x7fc1_2345,
                    0x4321_1234
                ]
            );
            assert_eq!(e.mem.u32(context), 0x37);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(entry, &args![audio]);
        assert_eq!(e.call_log.unwrap().len(), 3);
    }

    #[test]
    fn enter_dialogue_captures_point_before_manager_and_passes_all_eight_stack_words() {
        check_dialogue(0x00ad_85a0, 1);
    }

    #[test]
    fn exit_dialogue_captures_point_before_manager_and_passes_all_eight_stack_words() {
        check_dialogue(0x00ad_8650, 0);
    }

    fn check_timer_callback(entry: u32, callback_offset: u32, mask: u32, bit: u32) {
        let (mut e, audio) = setup();
        // These null-callback cases also match the native PC body, including
        // guarded object bytes and RET cleanup (2026-10-11, retail 1.4.0.525).
        for flags in [0, 1, 2, 3, 0x8000_0000, u32::MAX, 0x1234_5678, 0x8765_4321] {
            e.set(audio, BSAudio::synchTimerFlags, flags);
            e.call_log = Some(vec![]);
            e.call(entry, &args![audio]);
            assert_eq!(e.get(audio, BSAudio::synchTimerFlags), (flags & mask) | bit);
            assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
        }
        e.mem
            .set_u32(audio.addr() + callback_offset, VIRTUAL_RETURN);
        e.register_double(VIRTUAL_RETURN, move |e, a| {
            assert!(a.is_empty());
            e.set(audio, BSAudio::synchTimerFlags, 0x8765_4321);
            Ret::default()
        });
        e.call(entry, &args![audio]);
        assert_eq!(
            e.get(audio, BSAudio::synchTimerFlags),
            (0x8765_4321 & mask) | bit
        );
        assert_eq!(e.call_log.unwrap().last(), Some(&(VIRTUAL_RETURN, vec![])));
    }

    #[test]
    fn pause_timer_calls_optional_callback_before_reading_flags() {
        check_timer_callback(0x00ad_8700, 0x30, u32::MAX, 1);
    }

    #[test]
    fn unpause_timer_calls_optional_callback_before_reading_flags() {
        check_timer_callback(0x00ad_8740, 0x34, !1, 0);
    }

    #[test]
    fn end_all_sounds_is_noop_with_unmapped_receiver_and_unused_word() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00ad_8780, &[0, 0xdead_beef]);
        assert_eq!(
            e.call_log.unwrap(),
            vec![(0x00ad_8780, vec![0, 0xdead_beef])]
        );
    }
}
