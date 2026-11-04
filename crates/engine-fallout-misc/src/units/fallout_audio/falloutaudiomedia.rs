//! `fallout/audio/falloutaudiomedia.cpp` (Xbox PDB source unit), subsystem `fallout/audio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `FalloutAudioMedia` plays streamed music on two channels ("A" and "B")
//! that cross-fade into each other. Its state is a set of statics; every
//! function here takes `FalloutAudioMedia::CriticalSection` (`00830470` /
//! `00830490`) around them. Each channel has a volume, fade start and end
//! volumes, an attenuation, a flag byte, a file name, a media type, timing
//! words and an ID; the PC addresses come from the code, the names from the
//! Xbox PDB's static members, whose per-channel order matches the PC layout
//! (`uiMediaAFadeEnd` is written as `now + uiMediaAFadeTime`, the ID is
//! compared with the playing thread's ID).
//!
//! Two `FAMThread`s (a `BSThread` each, 0x4C bytes on PC) play a file each
//! through DirectShow (`FalloutAudioMedia::ThreadProc`, `00830b30`): a
//! filter graph (`CoCreateInstance` with the CLSID at `0107cb18`,
//! `{e436ebb3-524f-11ce-9f53-0020af0ba770}`), the interfaces it queries by
//! the IIDs in `.rdata` (`{56a868a9-...}`, `{56a868b3-...}`, `{56a868b1-...}`,
//! `{56a868b2-...}`, `{56a868b6-...}`; the Windows SDK names them
//! IGraphBuilder, IBasicAudio, IMediaControl, IMediaPosition and
//! IMediaEventEx), and a `CBSAReader` source filter (RTTI at its vtables)
//! over a `CBSAStream` of the file.
//!
//! Flag bits of a channel (what the code does with them; their Xbox names
//! are not in the PDB): 0x01 stop the playing thread, 0x02 stop playback,
//! 0x04 paused, 0x08 fading out, 0x10 fade in progress, 0x20 loop.
//!
//! x87 note: the game computes in extended precision and stores `float`
//! results. The translations compute in `f64` and round to `f32` where the
//! code stores a `float`; the two can differ only in the last bit of a
//! result rounded twice.

#[allow(unused_imports)]
use crate::prelude::*;

/// `MemoryManager::Allocate` wrapper (`operator new`, cdecl, one word).
const ALLOCATE: u32 = 0x0040_1000;
/// `operator delete` (cdecl, one word).
const DEALLOCATE: u32 = 0x0040_1030;
/// `strcpy_s(dest, size, src)` (cdecl).
const STRCPY_S: u32 = 0x0040_6d30;
/// The game's printf-style log (cdecl, variadic).
const LOG: u32 = 0x005b_5e40;
/// Enters a `BSCriticalSection` (`this`, one word).
const CRITICAL_SECTION_ENTER: u32 = 0x0045_38a0;
/// Leaves a `BSCriticalSection` (`this`).
const CRITICAL_SECTION_LEAVE: u32 = 0x0045_38c0;
/// `BSAudio::QInstance` (map name): the object the audio clock is read from.
const AUDIO_INSTANCE: u32 = 0x0045_3a70;
/// Millisecond clock read from that object (`0063d040`, `thiscall`).
const AUDIO_CLOCK: u32 = 0x0063_d040;
/// `_ftol2_sse`: the value in ST0 as a leading `f64` argument.
const FTOL: u32 = 0x00ec_62c0;
/// `FAMThread` start: stores the thread info and wakes the thread
/// (`0082f760`, `thiscall`, one word).
const FAM_THREAD_START: u32 = 0x0082_f760;

/// Imported `WaitForSingleObject`.
const WAIT_FOR_SINGLE_OBJECT: u32 = 0x00fd_f1c8;
/// Imported `CloseHandle`.
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// Imported `SetEvent`.
const SET_EVENT: u32 = 0x00fd_f0d0;
/// Imported `CoInitializeEx`.
const CO_INITIALIZE_EX: u32 = 0x00fd_f458;
/// Imported `CoCreateInstance`.
const CO_CREATE_INSTANCE: u32 = 0x00fd_f46c;

/// `FalloutAudioMedia::CriticalSection` (Xbox PDB static).
const CRITICAL_SECTION: u32 = 0x011d_d3d8;
/// The two `FAMThread`s (an array allocated by `Startup`, with its count
/// in the word before it).
const THREADS: u32 = 0x011d_d1f4;
/// The channel the next opened file goes to: 0 = A, 1 = B (written 1 after
/// opening on A and 0 after opening on B).
const NEXT_CHANNEL: u32 = 0x011d_d36c;
/// Number of running `FalloutAudioMedia::ThreadProc`s.
const THREAD_COUNT: u32 = 0x011d_d370;
/// A word `0082fb70` clears (its writer is not in this unit).
const MUSIC_RESET: u32 = 0x011d_d374;
/// An object `0082fb70` keeps (from the region's slot 0x34; `00595cb0` and
/// `00597990` are called on it).
const CURRENT_MEDIA_OBJECT: u32 = 0x011d_d378;
/// An object `0082fb70` uses in place of the region list lookup when set.
const REGION_MEDIA_OVERRIDE: u32 = 0x011d_d380;
/// Event handle `0082fb70` signals and each playing thread waits on.
const UPDATE_EVENT: u32 = 0x011d_d3b0;
/// The thread info passed to a starting `FAMThread`: ID, then channel.
const THREAD_INFO: u32 = 0x011d_d3c4;
const THREAD_INFO_CHANNEL: u32 = 0x011d_d3c8;
/// Next media ID (incremented per opened file).
const NEXT_MEDIA_ID: u32 = 0x011a_163c;
/// Byte set by `00830640` and cleared by `00830660`; nonzero pauses playback.
const PAUSE_BYTE: u32 = 0x011d_d312;
/// Byte read by `00830ad0` and `0082fb70` (its writer is not in this unit).
const MEDIA_HOLD_BYTE: u32 = 0x011d_d313;
/// Byte `008300c0` reads: when set, only media type 8 opens.
const OPEN_BLOCKED_BYTE: u32 = 0x011d_cfa5;

/// `"NONE"`.
const NONE_TEXT: u32 = 0x0101_a6c0;

/// The statics of one channel.
struct Channel {
    /// `fMediaXVolume` (float).
    volume: u32,
    /// `fMediaXFadeStartVolume` (float).
    fade_start_volume: u32,
    /// `fMediaXFadeEndVolume` (float).
    fade_end_volume: u32,
    /// `fMediaXAttenuation` (float).
    attenuation: u32,
    /// `cMediaXFlags` (byte).
    flags: u32,
    /// `pMediaXFileName` (`char[260]`).
    file_name: u32,
    /// `eMediaXType`.
    kind: u32,
    /// `uiMediaXSynchStartTime`.
    synch_start: u32,
    /// `uiMediaXCurrentPosition`.
    current_position: u32,
    /// `uiMediaXDuration`.
    duration: u32,
    /// `uiMediaXSubDuration`.
    sub_duration: u32,
    /// `uiMediaXFadeTime`.
    fade_time: u32,
    /// `uiMediaXFadeEnd`.
    fade_end: u32,
    /// `uiMediaXLastUpdate`.
    last_update: u32,
    /// `uiMediaXID`.
    id: u32,
}

const CHANNEL_A: Channel = Channel {
    volume: 0x011d_d300,
    fade_start_volume: 0x011d_d304,
    fade_end_volume: 0x011d_d308,
    attenuation: 0x011d_d30c,
    flags: 0x011d_d310,
    file_name: 0x011d_d0f0,
    kind: 0x011d_d314,
    synch_start: 0x011d_d318,
    current_position: 0x011d_d31c,
    duration: 0x011d_d320,
    sub_duration: 0x011d_d324,
    fade_time: 0x011d_d328,
    fade_end: 0x011d_d32c,
    last_update: 0x011d_d330,
    id: 0x011d_d334,
};

const CHANNEL_B: Channel = Channel {
    volume: 0x011d_d338,
    fade_start_volume: 0x011d_d33c,
    fade_end_volume: 0x011d_d340,
    attenuation: 0x011d_d344,
    flags: 0x011d_d311,
    file_name: 0x011d_d1f8,
    kind: 0x011d_d348,
    synch_start: 0x011d_d34c,
    current_position: 0x011d_d350,
    duration: 0x011d_d354,
    sub_duration: 0x011d_d358,
    fade_time: 0x011d_d35c,
    fade_end: 0x011d_d360,
    last_update: 0x011d_d364,
    id: 0x011d_d368,
};

layout! {
    /// `` `anonymous-namespace'::FAMThread `` (Xbox PDB, 0xC4 bytes there):
    /// 0x4C bytes on PC (the `BSThread` base is 0x30; offsets from the PC
    /// code: `0082f610`, `0082f6c0`, `0082f790`, `0082f7b0`).
    pub struct FamThread: 0x4C {
        /// `m_event` (Xbox PDB): the event `Wait` waits on.
        0x30 m_event: u32,
        /// `m_param` (Xbox PDB): the media ID the thread plays.
        0x34 param_id: u32,
        /// `m_param` second word: the channel (0 = A, 1 = B).
        0x38 param_channel: u32,
        /// `m_state` (Xbox PDB): bit 0 `Run`, bit 1 `Running` (Xbox PDB enum
        /// `FAMThread::State`).
        0x3C m_state: u32,
        /// `m_havok_memory` (Xbox PDB): passed to `DoThreadSetup` and
        /// `DoThreadShutdown`.
        0x40 m_havok_memory: u32,
    }
}

/// A float the exe copies through `FLD` and `FSTP`: a signaling NaN comes
/// out quiet.
fn quiet(value: f32) -> f32 {
    if value.is_nan() {
        f32::from_bits(value.to_bits() | 0x0040_0000)
    } else {
        value
    }
}

fn global_float(e: &Engine, addr: u32) -> f32 {
    quiet(e.global::<f32>(addr))
}

fn global_byte(e: &Engine, addr: u32) -> u8 {
    e.global::<u8>(addr)
}

/// The audio clock: `BSAudio::QInstance()` then `0063d040` on it.
fn audio_clock(e: &mut Engine) -> u32 {
    let audio = e.call(AUDIO_INSTANCE, &[]).u32();
    e.call(AUDIO_CLOCK, &args![audio]).u32()
}

/// Copies 16 bytes (a GUID) from `.rdata` into game memory, as four words.
fn copy_guid(e: &mut Engine, to: u32, from: u32) {
    for i in 0..4 {
        let word: u32 = e.global(from + 4 * i);
        e.mem.set_u32(to + 4 * i, word);
    }
}

/// `seconds * 1000.0` converted with `FISTP qword` under truncation, low
/// word kept: the x87 "integer indefinite" (low word 0) for NaN and values
/// outside the 64-bit range.
fn milliseconds(seconds: f64, thousand: f64) -> u32 {
    let value = seconds * thousand;
    if value.is_nan()
        || value >= 9_223_372_036_854_775_808.0
        || value < -9_223_372_036_854_775_808.0
    {
        0
    } else {
        value.trunc() as i64 as u32
    }
}

/// A channel is no longer the one the thread at `channel` with `id` plays:
/// another file took its place or its stop bit (0x01) is set.
fn thread_should_stop(e: &Engine, channel: u32, id: u32) -> bool {
    (channel == 0
        && (id != e.global::<u32>(CHANNEL_A.id) || global_byte(e, CHANNEL_A.flags) & 1 != 0))
        || (channel == 1
            && (id != e.global::<u32>(CHANNEL_B.id) || global_byte(e, CHANNEL_B.flags) & 1 != 0))
}

/// Resets a channel's statics as `MediaClear` does.
fn clear_channel(e: &mut Engine, c: &Channel, words: &[u32]) {
    e.set_global(c.volume, 0.0f32);
    e.set_global(c.fade_start_volume, 0.0f32);
    e.set_global(c.fade_end_volume, 0.0f32);
    e.set_global(c.attenuation, 0.0f32);
    e.set_global(c.flags, 0u8);
    e.call(STRCPY_S, &args![c.file_name, 0x104u32, NONE_TEXT]);
    for &w in words {
        e.set_global(w, 0u32);
    }
}

fn channel_words(c: &Channel) -> [u32; 9] {
    [
        c.kind,
        c.synch_start,
        c.current_position,
        c.duration,
        c.sub_duration,
        c.fade_time,
        c.fade_end,
        c.last_update,
        c.id,
    ]
}

// Translated from 0082f010 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082f010 - C++ exception frame (FS:[0] unwind of the base and member constructors) not translated
/// `CBSAStream` constructor (class from the RTTI at its vtable `0107cb4c`):
/// base constructor `0082f240`, the vtable, the member at +0x10C
/// (`0044fae0`), +0x130 and +0x134 zeroed, the file name copied to +0x04
/// (`strcpy_s`, 0x104 bytes), then the file opened at +0x108 with
/// `FileFinder::GetFile_ov2(name, 0, 0x4000, 8)`. If that gives nothing or
/// `008905f0` on the file says no, the file is deleted through its slot 0
/// (`1`, deleting destructor) and +0x108 cleared. Returns `this`.
pub fn fn_0082f010(e: &mut Engine, this: Ptr, name: Ptr) -> Ptr {
    let this_addr = this.addr();
    e.call(0x0082_f240, &args![this]);
    e.mem.set_u32(this_addr, 0x0107_cb4c);
    e.call(0x0044_fae0, &args![this_addr + 0x10c]);
    e.mem.set_u32(this_addr + 0x130, 0);
    e.mem.set_u32(this_addr + 0x134, 0);
    e.call(STRCPY_S, &args![this_addr + 4, 0x104u32, name]);
    let file = e
        .call(0x00af_df20, &args![this_addr + 4, 0u32, 0x4000u32, 8u32])
        .u32();
    e.mem.set_u32(this_addr + 0x108, file);
    let file = e.mem.u32(this_addr + 0x108);
    let usable = file != 0 && e.call(0x0089_05f0, &args![file]).bool();
    if !usable {
        let file = e.mem.u32(this_addr + 0x108);
        if file != 0 {
            e.vcall(file, 0, &args![1u32]);
        }
        e.mem.set_u32(this_addr + 0x108, 0);
    }
    this
}

// Translated from 0082f6c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082f6c0 - C++ exception frame (FS:[0] unwind to the BSThread destructor) not translated
/// `FAMThread` destructor: clears the `Run` bit, wakes the thread with its
/// own parameter (`0082f760(this, &m_param)`), sleeps 10 ms (`0040fca0`)
/// until the `Running` bit is clear, closes the event, then runs the
/// `BSThread` destructor (`00aa63c0`).
pub fn fn_0082f6c0(e: &mut Engine, this: Ptr<FamThread>) {
    e.mem.set_u32(this.addr(), 0x0107_cbe8);
    let state = e.get(this, FamThread::m_state);
    e.set(this, FamThread::m_state, state & !1);
    e.call(FAM_THREAD_START, &args![this, this.addr() + 0x34]);
    while e.get(this, FamThread::m_state) & 2 != 0 {
        e.call(0x0040_fca0, &args![10u32]);
    }
    let event = e.get(this, FamThread::m_event);
    e.call(CLOSE_HANDLE, &args![event]);
    e.call(0x00aa_63c0, &args![this]);
}

// Translated from 0082f790 (decompiled, FalloutNV.exe 1.4.0.525)
/// `` `anonymous-namespace'::FAMThread::Wait `` (Xbox PDB):
/// `WaitForSingleObject(m_event, INFINITE)`; its result stays in EAX.
pub fn fam_thread_wait(e: &mut Engine, this: Ptr<FamThread>) -> u32 {
    let event = e.get(this, FamThread::m_event);
    e.call(WAIT_FOR_SINGLE_OBJECT, &args![event, 0xffff_ffffu32])
        .u32()
}

// Translated from 0082f7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `` `anonymous-namespace'::FAMThread::ThreadProc `` (Xbox PDB): sets the
/// `Running` bit; while the `Run` bit is set, waits for the event, runs
/// `DoThreadSetup(&m_havok_memory)` and, if `Run` is still set,
/// `FalloutAudioMedia::ThreadProc(&m_param)`. Then clears `Running`, runs
/// `DoThreadShutdown(&m_havok_memory)` and returns 0.
pub fn fam_thread_thread_proc(e: &mut Engine, this: Ptr<FamThread>) -> u32 {
    let state = e.get(this, FamThread::m_state);
    e.set(this, FamThread::m_state, state | 2);
    while e.get(this, FamThread::m_state) & 1 != 0 {
        fam_thread_wait(e, this);
        e.call(0x00c8_0130, &args![this.addr() + 0x40]);
        if e.get(this, FamThread::m_state) & 1 != 0 {
            fallout_audio_media_thread_proc(e, Ptr::new(this.addr() + 0x34));
        }
    }
    let state = e.get(this, FamThread::m_state);
    e.set(this, FamThread::m_state, state & !2);
    e.call(0x00c8_0240, &args![this.addr() + 0x40]);
    0
}

// Translated from 0082f830 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 0082f830 - C++ exception frame (FS:[0] unwind freeing the array) not translated
/// `FalloutAudioMedia::Startup` (Xbox PDB): allocates two `FAMThread`s
/// (0x9C bytes: the count 2, then the array built by
/// `` `eh vector constructor iterator' `` with `0082f610` and `0082f6c0`)
/// into the thread pointer (null if the allocation failed), then
/// initializes both threads (`BSThread::Initialize(0x4000, "FalloutAudioMedia
/// (1/2)" / "(2/2)", 0)`), gives them priority 1 and processor 0.
pub fn fallout_audio_media_startup(e: &mut Engine) {
    let block = e.call(ALLOCATE, &args![0x9cu32]).u32();
    let threads = if block != 0 {
        e.mem.set_u32(block, 2);
        e.call(
            0x00ec_782f,
            &args![block + 4, 0x4cu32, 2u32, 0x0082_f610u32, 0x0082_f6c0u32],
        );
        block + 4
    } else {
        0
    };
    e.set_global(THREADS, threads);
    let first: u32 = e.global(THREADS);
    e.call(0x00aa_6430, &args![first, 0x4000u32, 0x0107_cc08u32, 0u32]);
    let first: u32 = e.global(THREADS);
    e.call(0x00aa_6560, &args![first, 1u32]);
    let second = e.global::<u32>(THREADS).wrapping_add(0x4c);
    e.call(0x00aa_6430, &args![second, 0x4000u32, 0x0107_cbf0u32, 0u32]);
    let second = e.global::<u32>(THREADS).wrapping_add(0x4c);
    e.call(0x00aa_6560, &args![second, 1u32]);
    let first: u32 = e.global(THREADS);
    e.call(0x00aa_6550, &args![first, 0u32]);
    let second = e.global::<u32>(THREADS).wrapping_add(0x4c);
    e.call(0x00aa_6550, &args![second, 0u32]);
}

// Translated from 0082f950 (decompiled, FalloutNV.exe 1.4.0.525)
/// `` `anonymous-namespace'::FAMThread::`vector deleting destructor' ``
/// (Xbox PDB): with bit 1 of `flags`, destroys the array (count in the word
/// before `this`) with `` `eh vector destructor iterator' `` and frees the
/// block from that word (bit 0), returning the block; otherwise destroys
/// the one thread (`0082f6c0`), frees it on bit 0 and returns `this`.
pub fn fam_thread_vector_deleting_destructor(
    e: &mut Engine,
    this: Ptr<FamThread>,
    flags: u32,
) -> u32 {
    let this_addr = this.addr();
    if flags & 2 != 0 {
        let count = e.mem.u32(this_addr.wrapping_sub(4));
        e.call(0x00ec_5fce, &args![this, 0x4cu32, count, 0x0082_f6c0u32]);
        if flags & 1 != 0 {
            e.call(DEALLOCATE, &args![this_addr.wrapping_sub(4)]);
        }
        this_addr.wrapping_sub(4)
    } else {
        fn_0082f6c0(e, this);
        if flags & 1 != 0 {
            e.call(DEALLOCATE, &args![this]);
        }
        this_addr
    }
}

// Translated from 0082f9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: deletes the thread array (through the first
/// thread's slot 0 with 3, the array deleting destructor, when the count
/// word before it is nonzero; otherwise frees the block) and clears the
/// thread pointer.
pub fn fn_0082f9c0(e: &mut Engine) {
    let threads: u32 = e.global(THREADS);
    if threads != 0 {
        if e.mem.u32(threads.wrapping_sub(4)) != 0 {
            e.vcall(threads, 0, &args![3u32]);
        } else {
            e.call(DEALLOCATE, &args![threads.wrapping_sub(4)]);
        }
    }
    e.set_global(THREADS, 0u32);
}

// Translated from 0082fa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutAudioMedia::MediaClear` (Xbox PDB): resets both channels: volumes
/// and attenuations 0, flags 0, file names `"NONE"`, type, times and ID 0,
/// and the next-channel word 0.
pub fn fallout_audio_media_media_clear(e: &mut Engine) {
    clear_channel(e, &CHANNEL_A, &channel_words(&CHANNEL_A));
    clear_channel(e, &CHANNEL_B, &channel_words(&CHANNEL_B));
    e.set_global(NEXT_CHANNEL, 0u32);
}

// Translated from 0082fb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the per-frame music decision. Signals the
/// update event; stops if the hold byte is set; while media of type 6 or 8
/// plays, clears `MUSIC_RESET` and stops. Otherwise, unless `007027b0(0x3EF)`
/// says so, it keeps or drops the current media object (`00597990(now,
/// combat, 0)` on it), then: with the player (`011dea3c`) in a state that
/// `009698a0`/`00595600` reject, it fades types 2 and 3 out over a second
/// when they play, looks up the region's media object (the override at
/// `011dd380`, else `TESRegionDataList::Find(7)` on `009611e0` of
/// `LowProcess::GetSecondGenericLocation` of `011dcfb4`) and starts it
/// through `00597990`; otherwise the player's slot 0x22C fades all media out
/// over a second, or, when something plays and `00830ad0` is false, types
/// 4, 5, 2 and 3 fade out over five seconds.
pub fn fn_0082fb70(e: &mut Engine) {
    let event: u32 = e.global(UPDATE_EVENT);
    e.call(SET_EVENT, &args![event]);
    if global_byte(e, MEDIA_HOLD_BYTE) != 0 {
        return;
    }
    if fallout_audio_media_media_is_playing(e, 6, true)
        || fallout_audio_media_media_is_playing(e, 8, true)
    {
        if e.global::<u32>(MUSIC_RESET) != 0 {
            e.set_global(MUSIC_RESET, 0u32);
        }
        return;
    }
    let current_kind: u32 = if e.global::<u32>(NEXT_CHANNEL) == 0 {
        e.global(CHANNEL_B.kind)
    } else {
        e.global(CHANNEL_A.kind)
    };
    let now = audio_clock(e);
    let combat_manager: u32 = e.global(0x011f_1958);
    let in_combat = if combat_manager == 0 {
        0u8
    } else {
        e.call(0x0099_2d90, &args![combat_manager]).u8()
    };
    if e.call(0x0070_27b0, &args![0x3efu32]).bool() {
        return;
    }
    let media_object: u32 = e.global(CURRENT_MEDIA_OBJECT);
    if media_object != 0 {
        let media_object: u32 = e.global(CURRENT_MEDIA_OBJECT);
        if media_object == 0 {
            return;
        }
        if e.call(
            0x0059_7990,
            &args![media_object, now, in_combat as u32, 0u32],
        )
        .bool()
        {
            return;
        }
    }
    if e.global::<u32>(CURRENT_MEDIA_OBJECT) != 0 {
        e.set_global(CURRENT_MEDIA_OBJECT, 0u32);
    }

    let player: u32 = e.global(0x011d_ea3c);
    let mut player_branch = player == 0;
    if !player_branch {
        let first = e.call(0x0096_98a0, &args![player]).u32();
        if first != 0 {
            let player: u32 = e.global(0x011d_ea3c);
            let second = e.call(0x0096_98a0, &args![player]).u32();
            if e.call(0x0059_5600, &args![second, in_combat as u32]).bool() {
                player_branch = true;
            }
        }
    }

    if !player_branch {
        e.set_global(MUSIC_RESET, 0u32);
        let process: u32 = e.global(0x011d_cfb4);
        if process == 0 || e.call(0x0067_33e0, &args![process]).u32() == 0 {
            return;
        }
        if fallout_audio_media_media_is_playing(e, 2, true) {
            fn_00830680(e, 1000, 2);
        }
        if fallout_audio_media_media_is_playing(e, 3, true) {
            fn_00830680(e, 1000, 3);
        }
        let process: u32 = e.global(0x011d_cfb4);
        let location = e.call(0x0067_33e0, &args![process]).u32();
        let override_object: u32 = e.global(REGION_MEDIA_OVERRIDE);
        let region = if override_object != 0 {
            override_object
        } else {
            let list = e.call(0x0096_11e0, &args![location]).u32();
            e.call(0x004f_35b0, &args![list, 7u32]).u32()
        };
        if region == 0 {
            return;
        }
        if in_combat != 0 && e.global::<u32>(CURRENT_MEDIA_OBJECT) == 0 {
            let object = e.vcall(region, 0x34, &[]).u32();
            e.set_global(CURRENT_MEDIA_OBJECT, object);
            if e.global::<u32>(CURRENT_MEDIA_OBJECT) != 0 {
                let object: u32 = e.global(CURRENT_MEDIA_OBJECT);
                e.call(0x0059_5cb0, &args![object]);
            }
        }
        if e.global::<u32>(CURRENT_MEDIA_OBJECT) != 0 {
            let object: u32 = e.global(CURRENT_MEDIA_OBJECT);
            if object == 0 {
                return;
            }
            if e.call(0x0059_7990, &args![object, now, in_combat as u32, 0u32])
                .bool()
            {
                return;
            }
        }
        e.set_global(CURRENT_MEDIA_OBJECT, 0u32);
        if e.vcall(region, 0x30, &[]).u32() != 0 && current_kind == 0 {
            let object = e.vcall(region, 0x30, &[]).u32();
            e.call(0x0059_7990, &args![object, now, 0u32, 0u32]);
        }
        return;
    }

    let player: u32 = e.global(0x011d_ea3c);
    if player == 0 {
        return;
    }
    if e.vcall(player, 0x22c, &args![0u32]).bool() {
        fn_00830680(e, 1000, 0);
    } else if fallout_audio_media_media_is_playing(e, 0, true)
        && !fn_00830ad0(e)
        && e.global::<u32>(MUSIC_RESET) == 0
    {
        fn_00830680(e, 5000, 4);
        fn_00830680(e, 5000, 5);
        fn_00830680(e, 5000, 2);
        fn_00830680(e, 5000, 3);
    }
}

// Translated from 0082fec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the playback volume for media of type `kind`
/// with `attenuation`. The base volume is the float `00403e20` gives for
/// `011dd084` (type 8), `BSAudio::GetRadioVolume` (type 7) or `00ad7ab0` on
/// `BSAudio` (others); `0082ffd0(base) * 2000 + attenuation * 100` is
/// clamped to `-10000..=0`, raised as `004dd130(10.0, x / 2000)`, scaled by
/// `00ad79f0`, and clamped to `0.0001..=100`.
pub fn fn_0082fec0(e: &mut Engine, kind: i32, attenuation: f32) -> f32 {
    let base = if kind == 8 {
        let p = e.call(0x0040_3e20, &args![0x011d_d084u32]).u32();
        quiet(e.mem.f32(p))
    } else if kind == 7 {
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        e.call(0x00ad_7ae0, &args![audio]).f64() as f32
    } else {
        let audio = e.call(AUDIO_INSTANCE, &[]).u32();
        e.call(0x00ad_7ab0, &args![audio]).f64() as f32
    };
    let level = fn_0082ffd0(e, base) as f64;
    let two_thousand: f64 = e.global(0x0107_3490);
    let hundred: f64 = e.global(0x0101_7a40);
    let mut decibels = (level * two_thousand + attenuation as f64 * hundred) as f32;
    if (decibels as f64) < e.global::<f64>(0x0102_d268) {
        decibels = global_float(e, 0x0107_cc20);
    } else if decibels as f64 > e.global::<f64>(0x0101_2060) {
        decibels = 0.0;
    }
    let exponent = (decibels as f64 / two_thousand) as f32;
    let ten = global_float(e, 0x0101_7b78);
    let factor = e.call(0x004d_d130, &args![ten, quiet(exponent)]).f64() as f32;
    let audio = e.call(AUDIO_INSTANCE, &[]).u32();
    let master = e.call(0x00ad_79f0, &args![audio]).f64();
    let mut volume = (master * factor as f64) as f32;
    if (volume as f64) < e.global::<f64>(0x0103_2980) {
        volume = global_float(e, 0x0101_41cc);
    } else if volume as f64 > hundred {
        volume = global_float(e, 0x0101_6410);
    }
    volume
}

// Translated from 0082ffd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `0082fff0(value)`.
pub fn fn_0082ffd0(e: &mut Engine, value: f32) -> f32 {
    fn_0082fff0(e, quiet(value))
}

// Translated from 0082fff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the CRT `double` function `00ecbc80` (its
/// name is not confirmed; the callers use it as a decibel conversion) of
/// `value`, stored as a float.
pub fn fn_0082fff0(e: &mut Engine, value: f32) -> f32 {
    e.call(0x00ec_bc80, &args![quiet(value) as f64]).f64() as f32
}

// Translated from 00830010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: plays the media object `object` (nothing for
/// null): opens the file `00591a40` names as type 6 with a one-second fade,
/// looping when its float at +0x24 is positive (`00830050`), attenuation
/// `00830080`, no priority override and no synch time.
pub fn fn_00830010(e: &mut Engine, object: Ptr) {
    if object.is_null() {
        return;
    }
    let attenuation = fn_00830080(e, object);
    let looping = fn_00830050(e, object);
    let file_name = e.call(0x0059_1a40, &args![object]).u32();
    fn_008300c0(
        e,
        6,
        Ptr::new(file_name),
        1000,
        looping as u8,
        0,
        attenuation,
        0,
    );
}

// Translated from 00830050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether the float at +0x24 of `this` is
/// greater than 0 (false for NaN).
pub fn fn_00830050(e: &mut Engine, this: Ptr) -> bool {
    let value = e.mem.f32(this.addr() + 0x24);
    value as f64 > e.global::<f64>(0x0101_2060)
}

// Translated from 00830080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the float at +0x24 of `this`, negated when it
/// is greater than 0.
pub fn fn_00830080(e: &mut Engine, this: Ptr) -> f32 {
    let value = e.mem.f32(this.addr() + 0x24);
    if value as f64 > e.global::<f64>(0x0101_2060) {
        -value
    } else {
        quiet(value)
    }
}

// Translated from 008300c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 008300c0 - C++ exception frame (FS:[0] unwind of the local BSFile) and /GS cookie check (00ec408c) not translated
/// Unnamed in the engine map: opens a media file on the next channel.
/// Type 9 clears everything (`MediaClear`). While the open-blocked byte is
/// set only type 8 opens; an empty name does nothing. Without `force`, a
/// type lower than the type of the channel last opened is refused (logged
/// "out of priority"). A file that `BSFile::Exist` does not find is logged.
/// Otherwise the next channel gets volume 0, fade-in to `0082fec0(kind,
/// attenuation)`, flags 0x10 (| 0x20 when `looping`), the name, type,
/// synch time, fade time and fade end (now + fade time) and a new ID; its
/// thread is started with that ID and channel; the other channel starts
/// fading out (flags | 0x18) over the same time; the next channel flips.
#[allow(clippy::too_many_arguments)]
pub fn fn_008300c0(
    e: &mut Engine,
    kind: i32,
    file_name: Ptr,
    fade_time: u32,
    looping: u8,
    force: u8,
    attenuation: f32,
    synch_time: u32,
) {
    fn_00830470(e);
    if kind == 9 {
        fallout_audio_media_media_clear(e);
        fn_00830490(e);
        return;
    }
    if kind != 8 && global_byte(e, OPEN_BLOCKED_BYTE) != 0 {
        fn_00830490(e);
        return;
    }
    if file_name.is_null() || e.mem.i8(file_name.addr()) == 0 {
        fn_00830490(e);
        return;
    }
    if force == 0 {
        let next: u32 = e.global(NEXT_CHANNEL);
        let refused = (kind < e.global::<i32>(CHANNEL_A.kind) && next == 1)
            || (kind < e.global::<i32>(CHANNEL_B.kind) && next == 0);
        if refused {
            e.call(LOG, &args![0x0107_cc54u32]);
            fn_00830490(e);
            return;
        }
    }
    let exists = e.with_stack(0x15c, |e, file| {
        e.call(0x00b0_0260, &args![file, file_name, 0u32, 0x4000u32, 0u32]);
        let exists = e.call(0x00af_fc60, &args![file]).bool();
        if !exists {
            e.call(LOG, &args![0x0107_cc24u32, file_name]);
            fn_00830490(e);
        }
        e.call(0x00af_f240, &args![file]);
        exists
    });
    if !exists {
        return;
    }
    let now = audio_clock(e);
    let flags: u8 = if looping != 0 { 0x30 } else { 0x10 };
    let (opened, other, next_after) = if e.global::<u32>(NEXT_CHANNEL) == 0 {
        (&CHANNEL_A, &CHANNEL_B, 1u32)
    } else {
        (&CHANNEL_B, &CHANNEL_A, 0u32)
    };
    e.set_global(opened.volume, 0.0f32);
    e.set_global(opened.fade_start_volume, 0.0f32);
    let target = fn_0082fec0(e, kind, quiet(attenuation));
    e.set_global(opened.fade_end_volume, target);
    e.set_global(opened.attenuation, quiet(attenuation));
    e.set_global(opened.flags, flags);
    e.call(STRCPY_S, &args![opened.file_name, 0x104u32, file_name]);
    e.set_global(opened.kind, kind);
    e.set_global(opened.synch_start, synch_time);
    e.set_global(opened.fade_time, fade_time);
    let fade_end = now.wrapping_add(e.global::<u32>(opened.fade_time));
    e.set_global(opened.fade_end, fade_end);
    e.set_global(opened.last_update, 0u32);
    let id: u32 = e.global(NEXT_MEDIA_ID);
    e.set_global(opened.id, id);
    let id: u32 = e.global(NEXT_MEDIA_ID);
    e.set_global(THREAD_INFO, id);
    let id: u32 = e.global(NEXT_MEDIA_ID);
    e.set_global(NEXT_MEDIA_ID, id.wrapping_add(1));
    let next: u32 = e.global(NEXT_CHANNEL);
    e.set_global(THREAD_INFO_CHANNEL, next);
    let thread = next
        .wrapping_mul(0x4c)
        .wrapping_add(e.global::<u32>(THREADS));
    e.call(FAM_THREAD_START, &args![thread, THREAD_INFO]);
    e.set_global(NEXT_CHANNEL, next_after);
    let other_flags = global_byte(e, other.flags) | 0x18;
    e.set_global(other.flags, other_flags);
    let other_volume = global_float(e, other.volume);
    e.set_global(other.fade_start_volume, other_volume);
    e.set_global(other.fade_end_volume, 0.0f32);
    let time: u32 = e.global(opened.fade_time);
    e.set_global(other.fade_time, time);
    let end: u32 = e.global(opened.fade_end);
    e.set_global(other.fade_end, end);
    fn_00830490(e);
}

// Translated from 00830470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: enters `FalloutAudioMedia::CriticalSection`
/// (`004538a0(section, 0)`).
pub fn fn_00830470(e: &mut Engine) {
    e.call(CRITICAL_SECTION_ENTER, &args![CRITICAL_SECTION, 0u32]);
}

// Translated from 00830490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: leaves `FalloutAudioMedia::CriticalSection`.
pub fn fn_00830490(e: &mut Engine) {
    e.call(CRITICAL_SECTION_LEAVE, &args![CRITICAL_SECTION]);
}

// Translated from 008304a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutAudioMedia::MediaClose` (Xbox PDB): `MediaClear` under the lock.
pub fn fallout_audio_media_media_close(e: &mut Engine) {
    fn_00830470(e);
    fallout_audio_media_media_clear(e);
    fn_00830490(e);
}

// Translated from 008304c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: clears flag 0x02 (stop playback) on both
/// channels.
pub fn fn_008304c0(e: &mut Engine) {
    fn_00830470(e);
    let a = global_byte(e, CHANNEL_A.flags) & !2;
    e.set_global(CHANNEL_A.flags, a);
    let b = global_byte(e, CHANNEL_B.flags) & !2;
    e.set_global(CHANNEL_B.flags, b);
    fn_00830490(e);
}

/// `008304f0`'s work for the channel last opened (`stopped`) and the other
/// one (`kept`).
fn stop_channel(e: &mut Engine, stopped: &Channel, kept: &Channel) {
    let flags = global_byte(e, stopped.flags) | 2;
    e.set_global(stopped.flags, flags);
    let kind: i32 = e.global(stopped.kind);
    let attenuation = global_float(e, stopped.attenuation);
    let volume = fn_0082fec0(e, kind, attenuation);
    e.set_global(stopped.volume, volume);
    e.set_global(stopped.synch_start, 0u32);
    e.set_global(stopped.fade_time, 0u32);
    e.set_global(stopped.fade_end, 0u32);
    e.set_global(stopped.current_position, 0u32);
    let flags = global_byte(e, kept.flags) | 1;
    e.set_global(kept.flags, flags);
}

// Translated from 008304f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: stops playback (flag 0x02) on the channel
/// last opened, sets its volume to `0082fec0(type, attenuation)` and its
/// synch, fade and position words to 0, and sets the stop-thread flag 0x01
/// on the other channel.
pub fn fn_008304f0(e: &mut Engine) {
    fn_00830470(e);
    if e.global::<u32>(NEXT_CHANNEL) == 0 {
        stop_channel(e, &CHANNEL_B, &CHANNEL_A);
    } else {
        stop_channel(e, &CHANNEL_A, &CHANNEL_B);
    }
    fn_00830490(e);
}

// Translated from 008305e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutAudioMedia::MediaPause` (Xbox PDB): sets the pause flag 0x04 on
/// both channels.
pub fn fallout_audio_media_media_pause(e: &mut Engine) {
    fn_00830470(e);
    let a = global_byte(e, CHANNEL_A.flags) | 4;
    e.set_global(CHANNEL_A.flags, a);
    let b = global_byte(e, CHANNEL_B.flags) | 4;
    e.set_global(CHANNEL_B.flags, b);
    fn_00830490(e);
}

// Translated from 00830610 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutAudioMedia::MediaUnPause` (Xbox PDB): clears the pause flag 0x04
/// on both channels.
pub fn fallout_audio_media_media_un_pause(e: &mut Engine) {
    fn_00830470(e);
    let a = global_byte(e, CHANNEL_A.flags) & !4;
    e.set_global(CHANNEL_A.flags, a);
    let b = global_byte(e, CHANNEL_B.flags) & !4;
    e.set_global(CHANNEL_B.flags, b);
    fn_00830490(e);
}

// Translated from 00830640 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the pause byte (`011dd312`) to 1.
pub fn fn_00830640(e: &mut Engine) {
    fn_00830470(e);
    e.set_global(PAUSE_BYTE, 1u8);
    fn_00830490(e);
}

// Translated from 00830660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the pause byte (`011dd312`) to 0.
pub fn fn_00830660(e: &mut Engine) {
    fn_00830470(e);
    e.set_global(PAUSE_BYTE, 0u8);
    fn_00830490(e);
}

/// `00830680`'s work for one channel.
fn fade_out_channel(e: &mut Engine, c: &Channel, fade_time: u32, kind: i32, now: u32) {
    if (e.global::<i32>(c.kind) == kind || kind == 0) && global_byte(e, c.flags) & 8 == 0 {
        let flags = global_byte(e, c.flags) | 0x18;
        e.set_global(c.flags, flags);
        let volume = global_float(e, c.volume);
        e.set_global(c.fade_start_volume, volume);
        e.set_global(c.fade_end_volume, 0.0f32);
        e.set_global(c.fade_time, fade_time);
        let end = now.wrapping_add(e.global::<u32>(c.fade_time));
        e.set_global(c.fade_end, end);
    }
}

// Translated from 00830680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: fades out over `fade_time` ms every channel
/// playing media of type `kind` (any type for 0) that is not already
/// fading out: flags | 0x18, fade start = volume, fade end volume 0, fade
/// end = now + `fade_time`.
pub fn fn_00830680(e: &mut Engine, fade_time: u32, kind: i32) {
    fn_00830470(e);
    let now = audio_clock(e);
    fade_out_channel(e, &CHANNEL_A, fade_time, kind, now);
    fade_out_channel(e, &CHANNEL_B, fade_time, kind, now);
    fn_00830490(e);
}

// Translated from 00830750 (decompiled, FalloutNV.exe 1.4.0.525)
/// `FalloutAudioMedia::MediaIsPlaying` (Xbox PDB): whether a channel with an
/// ID plays media of type `kind` (any type for 0) and is not stopped
/// (`ignore_fading` set: flag 0x02) or not fading out (clear: flag 0x08).
pub fn fallout_audio_media_media_is_playing(
    e: &mut Engine,
    kind: i32,
    ignore_fading: bool,
) -> bool {
    fn_00830470(e);
    let mask = if ignore_fading { 2 } else { 8 };
    let playing = |e: &Engine, c: &Channel| {
        e.global::<u32>(c.id) != 0
            && (kind == 0 || e.global::<i32>(c.kind) == kind)
            && global_byte(e, c.flags) & mask == 0
    };
    let result = playing(e, &CHANNEL_A) || playing(e, &CHANNEL_B);
    fn_00830490(e);
    result
}

/// `008308c0`, `00830970` and `00830a20`: a word of the channel last opened
/// (A when the next channel is 1, B when it is 0), if it has an ID; with
/// `kind` nonzero, only if it plays that type.
fn last_opened_word(e: &mut Engine, kind: i32, word_a: u32, word_b: u32) -> u32 {
    fn_00830470(e);
    let mut value = 0;
    let next: u32 = e.global(NEXT_CHANNEL);
    let a_id: u32 = e.global(CHANNEL_A.id);
    let b_id: u32 = e.global(CHANNEL_B.id);
    if kind == 0 {
        if next == 1 && a_id != 0 {
            value = e.global(word_a);
        } else if next == 0 && b_id != 0 {
            value = e.global(word_b);
        }
    }
    if next == 1 && a_id != 0 && e.global::<i32>(CHANNEL_A.kind) == kind {
        value = e.global(word_a);
    } else if next == 0 && b_id != 0 && e.global::<i32>(CHANNEL_B.kind) == kind {
        value = e.global(word_b);
    }
    fn_00830490(e);
    value
}

// Translated from 008308c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the duration (ms) of the channel last opened
/// (of type `kind`, any for 0), 0 when none.
pub fn fn_008308c0(e: &mut Engine, kind: i32) -> u32 {
    last_opened_word(e, kind, CHANNEL_A.duration, CHANNEL_B.duration)
}

// Translated from 00830970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the sub-duration (ms) of the channel last
/// opened (of type `kind`, any for 0), 0 when none.
pub fn fn_00830970(e: &mut Engine, kind: i32) -> u32 {
    last_opened_word(e, kind, CHANNEL_A.sub_duration, CHANNEL_B.sub_duration)
}

// Translated from 00830a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the current position (ms) of the channel last
/// opened (of type `kind`, any for 0), 0 when none.
pub fn fn_00830a20(e: &mut Engine, kind: i32) -> u32 {
    last_opened_word(
        e,
        kind,
        CHANNEL_A.current_position,
        CHANNEL_B.current_position,
    )
}

// Translated from 00830ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: whether a channel with an ID plays media of
/// type 7, or the hold byte (`011dd313`) is set.
pub fn fn_00830ad0(e: &mut Engine) -> bool {
    fn_00830470(e);
    let result = (e.global::<u32>(CHANNEL_A.id) != 0 && e.global::<u32>(CHANNEL_A.kind) == 7)
        || (e.global::<u32>(CHANNEL_B.id) != 0 && e.global::<u32>(CHANNEL_B.kind) == 7)
        || global_byte(e, MEDIA_HOLD_BYTE) != 0;
    fn_00830490(e);
    result
}

/// `FalloutAudioMedia::ThreadProc`'s stack frame: the locals it passes by
/// address, as offsets below its frame pointer.
mod frame {
    /// Bytes reserved below the frame pointer.
    pub const SIZE: u32 = 0x2b0;
    /// The file name (`char[260]`).
    pub const FILE_NAME: u32 = 0x154;
    /// The media type the reader is built with (majortype, subtype, ...).
    pub const MEDIA_TYPE: u32 = 0x1d4;
    /// The result word the reader constructor gets.
    pub const RESULT: u32 = 0x1d8;
    /// Filter graph (IID `{56a8689f-...}`).
    pub const GRAPH: u32 = 0x188;
    /// Graph builder (IID `{56a868a9-...}`).
    pub const BUILDER: u32 = 0x164;
    /// Basic audio (IID `{56a868b3-...}`).
    pub const AUDIO: u32 = 0x3c;
    /// Media control (IID `{56a868b1-...}`).
    pub const CONTROL: u32 = 0x16c;
    /// Media position (IID `{56a868b2-...}`).
    pub const POSITION: u32 = 0x18;
    /// Media events (IID `{56a868b6-...}`).
    pub const EVENTS: u32 = 0x24;
    /// The `double` seconds the position calls write.
    pub const SECONDS: u32 = 0x34;
    /// Event code and its two parameters.
    pub const EVENT_CODE: u32 = 0x1ec;
    pub const EVENT_PARAM_1: u32 = 0x1e0;
    pub const EVENT_PARAM_2: u32 = 0x1e8;
}

/// What `FalloutAudioMedia::ThreadProc` copies from its channel under the
/// lock each round.
#[derive(Default)]
struct ChannelView {
    fade_start: f32,
    attenuation: f32,
    flags: u8,
    kind: i32,
    synch_start: u32,
    fade_time: u32,
    fade_end: u32,
}

/// Publishes the thread's volume, clock and positions to its channel and
/// reads the channel's settings back (both channels: 0 = A, else B).
fn exchange_channel(
    e: &mut Engine,
    channel: u32,
    volume: f32,
    now: u32,
    position: u32,
    duration: u32,
    sub_duration: u32,
) -> ChannelView {
    let c = if channel == 0 { &CHANNEL_A } else { &CHANNEL_B };
    e.set_global(c.volume, quiet(volume));
    e.set_global(c.last_update, now);
    e.set_global(c.current_position, position);
    e.set_global(c.duration, duration);
    e.set_global(c.sub_duration, sub_duration);
    ChannelView {
        fade_start: global_float(e, c.fade_start_volume),
        attenuation: global_float(e, c.attenuation),
        flags: global_byte(e, c.flags),
        kind: e.global(c.kind),
        synch_start: e.global(c.synch_start),
        fade_time: e.global(c.fade_time),
        fade_end: e.global(c.fade_end),
    }
}

/// Calls the media position's `get_Duration` (slot 0x1C) or
/// `get_CurrentPosition` (slot 0x24) and converts the seconds to ms; 0
/// when the graph or the position interface is missing.
fn position_ms(e: &mut Engine, ebp: u32, slot: u32) -> u32 {
    let graph = e.mem.u32(ebp - frame::GRAPH);
    let position = e.mem.u32(ebp - frame::POSITION);
    if graph == 0 || position == 0 {
        return 0;
    }
    e.vcall(position, slot, &args![ebp - frame::SECONDS]);
    let seconds = e.mem.f64(ebp - frame::SECONDS);
    let thousand: f64 = e.global(0x0101_7b70);
    milliseconds(seconds, thousand)
}

/// A COM call's `HRESULT` failed (negative).
fn failed(result: u32) -> bool {
    (result as i32) < 0
}

// Translated from 00830b30 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00830b30 - C++ exception frame (FS:[0] unwind of the media type and the reader allocation) and /GS cookie check (00ec408c) not translated
/// `FalloutAudioMedia::ThreadProc` (Xbox PDB): plays the file of the
/// channel in `info` (ID, channel) until that channel moves on.
///
/// Set-up: a sync tolerance from `00647b70(500, *0043d4d0(011dcfd4) + 500)`;
/// the thread count goes up; the channel's state is exchanged and its file
/// name copied. A `CBSAStream` (0x138 bytes) of the file is made; if it is
/// not open (`00404d30`) "file not found" is logged. Otherwise a media type
/// (`009f9c70`) gets the majortype GUID at `0107cae8` and, when
/// `004812f0(name, ".mp3")` is nonzero, the subtype at `0107caf8` (else "file format wrong"
/// and no playback); a `CBSAReader` (0x1E8 bytes) is built over the stream,
/// COM is initialized, the filter graph created, the reader added and its
/// pin (`slot 0x1C`) rendered, and the audio, control, position and event
/// interfaces queried; any failure ends playback.
///
/// Each round waits up to 100 ms for the update event, then: exchanges the
/// state (position, duration); computes the target volume (`0082fec0`,
/// 0 while fading out); while a fade runs (flag 0x10, before its end) the
/// volume moves from the fade start along the remaining fraction, else it
/// steps 0.05 toward the target; a fade-out below 0.01 ends playback; the
/// volume goes to the audio interface (`-10000`, or `0082ffd0 * 2000`);
/// with a synch time the position is corrected when it is off by more than
/// the tolerance; the graph is stopped (flag 0x02), paused (flag 0x04, the
/// pause byte, or `00831de0`) or run; events 1..3 restart a looping file
/// and end any other. When the channel's ID changes or its stop flag is
/// set, playback ends: the channel is cleared if it is still this ID's, the
/// thread count goes down and every interface, the reader and the stream
/// are released. Returns 0.
pub fn fallout_audio_media_thread_proc(e: &mut Engine, info: Ptr) -> u32 {
    e.with_stack(frame::SIZE, |e, block| {
        thread_proc_body(e, info, block.addr() + frame::SIZE)
    })
}

fn thread_proc_body(e: &mut Engine, info: Ptr, ebp: u32) -> u32 {
    let channel = e.mem.u32(info.addr() + 4);
    let id = e.mem.u32(info.addr());
    let mut playing = true;
    let mut volume = 0.0f32;
    let mut position = 0u32;
    let mut duration = 0u32;

    let setting = e.call(0x0043_d4d0, &args![0x011d_cfd4u32]).u32();
    let base = e.mem.u32(setting);
    let tolerance = e
        .call(0x0064_7b70, &args![0x1f4u32, base.wrapping_add(0x1f4)])
        .u32();

    fn_00830470(e);
    let count: u32 = e.global(THREAD_COUNT);
    e.set_global(THREAD_COUNT, count.wrapping_add(1));
    if thread_should_stop(e, channel, id) {
        playing = false;
    }
    let mut now = audio_clock(e);
    exchange_channel(e, channel, volume, now, position, duration, 0);
    let source = if channel == 0 {
        CHANNEL_A.file_name
    } else {
        CHANNEL_B.file_name
    };
    e.call(STRCPY_S, &args![ebp - frame::FILE_NAME, 0x104u32, source]);
    fn_00830490(e);

    for slot in [
        frame::GRAPH,
        frame::BUILDER,
        frame::AUDIO,
        frame::POSITION,
        frame::EVENTS,
        frame::CONTROL,
    ] {
        e.mem.set_u32(ebp - slot, 0);
    }
    let mut reader = 0u32;
    e.mem.set_f64(ebp - frame::SECONDS, 0.0);

    let block = e.call(ALLOCATE, &args![0x138u32]).u32();
    let stream = if block != 0 {
        fn_0082f010(e, Ptr::new(block), Ptr::new(ebp - frame::FILE_NAME)).addr()
    } else {
        0
    };
    if !e.call(0x0040_4d30, &args![stream]).bool() {
        e.call(LOG, &args![0x0107_cd2cu32]);
        playing = false;
    } else {
        e.mem.set_u32(ebp - frame::RESULT, 0);
        let media_type = ebp - frame::MEDIA_TYPE;
        e.call(0x009f_9c70, &args![media_type]);
        copy_guid(e, media_type, 0x0107_cae8);
        if e.call(0x0048_12f0, &args![ebp - frame::FILE_NAME, 0x0103_3ad8u32])
            .u32()
            != 0
        {
            copy_guid(e, media_type + 0x10, 0x0107_caf8);
        } else {
            playing = false;
            e.call(LOG, &args![0x0107_cd04u32, ebp - frame::FILE_NAME]);
        }
        if playing {
            let block = e.call(ALLOCATE, &args![0x1e8u32]).u32();
            reader = if block != 0 {
                fn_00831e20(
                    e,
                    Ptr::new(block),
                    Ptr::new(stream),
                    Ptr::new(media_type),
                    Ptr::new(ebp - frame::RESULT),
                )
                .addr()
            } else {
                0
            };
            e.vcall(reader.wrapping_add(0xc), 4, &[]);
            e.call(CO_INITIALIZE_EX, &args![0u32, 0u32]);
            let result = e
                .call(
                    CO_CREATE_INSTANCE,
                    &args![
                        0x0107_cb18u32,
                        0u32,
                        1u32,
                        0x0109_3c0cu32,
                        ebp - frame::GRAPH
                    ],
                )
                .u32();
            e.mem.set_u32(ebp - frame::RESULT, result);
            if failed(result) {
                playing = false;
            }
            let filter = if reader != 0 { reader + 0xc } else { 0 };
            let graph = e.mem.u32(ebp - frame::GRAPH);
            let result = e.vcall(graph, 0xc, &args![filter, 0u32]).u32();
            if failed(result) {
                playing = false;
            }
            let graph = e.mem.u32(ebp - frame::GRAPH);
            let result = e
                .vcall(graph, 0, &args![0x0109_3b5cu32, ebp - frame::BUILDER])
                .u32();
            if failed(result) {
                playing = false;
            }
            let pin = e.vcall(reader, 0x1c, &args![0u32]).u32();
            let pin = if pin != 0 { pin + 0xc } else { 0 };
            let builder = e.mem.u32(ebp - frame::BUILDER);
            if failed(e.vcall(builder, 0x30, &args![pin]).u32()) {
                playing = false;
            }
            for (object, iid, out) in [
                (frame::BUILDER, 0x0107_cad8u32, frame::AUDIO),
                (frame::GRAPH, 0x0107_caa8, frame::CONTROL),
                (frame::BUILDER, 0x0107_cac8, frame::POSITION),
                (frame::BUILDER, 0x0107_cab8, frame::EVENTS),
            ] {
                let this = e.mem.u32(ebp - object);
                let result = e.vcall(this, 0, &args![iid, ebp - out]).u32();
                e.mem.set_u32(ebp - frame::RESULT, result);
                if failed(result) {
                    playing = false;
                }
            }
        }
        e.call(0x009f_9c60, &args![media_type]);
    }

    let sub_duration = position_ms(e, ebp, 0x1c);

    let thousand: f64 = e.global(0x0101_7b70);
    while playing {
        let event: u32 = e.global(UPDATE_EVENT);
        e.call(WAIT_FOR_SINGLE_OBJECT, &args![event, 100u32]);
        fn_00830470(e);
        now = audio_clock(e);
        position = position_ms(e, ebp, 0x24);
        duration = position_ms(e, ebp, 0x1c);
        let view = exchange_channel(e, channel, volume, now, position, duration, sub_duration);
        fn_00830490(e);

        let target = if view.flags & 8 != 0 {
            0.0f32
        } else {
            fn_0082fec0(e, view.kind, view.attenuation)
        };
        if view.flags & 0x10 != 0 && now < view.fade_end {
            let remaining = if view.fade_time > 0 {
                (view.fade_end as f64 - now as f64) / view.fade_time as f64
            } else {
                0.0
            };
            let mut fraction = remaining as f32;
            fraction = if (fraction as f64) < e.global::<f64>(0x0101_2070) {
                fraction
            } else {
                1.0
            };
            fraction = if fraction as f64 > e.global::<f64>(0x0101_2060) {
                fraction
            } else {
                0.0
            };
            volume = if view.flags & 8 != 0 {
                (fraction as f64 * view.fade_start as f64) as f32
            } else {
                ((1.0 - fraction as f64) * (target as f64 - view.fade_start as f64)
                    + view.fade_start as f64) as f32
            };
        } else {
            let step: f64 = e.global(0x0103_c7c8);
            if target < volume {
                volume = (volume as f64 - step) as f32;
                volume = if target < volume { volume } else { target };
            } else if target > volume {
                volume = (volume as f64 + step) as f32;
                volume = if target > volume { volume } else { target };
            }
        }
        if view.flags & 8 != 0 && (volume as f64) < e.global::<f64>(0x0101_6408) {
            playing = false;
        }

        let audio = e.mem.u32(ebp - frame::AUDIO);
        if audio != 0 {
            let level = if (volume as f64) < e.global::<f64>(0x0103_2980) {
                0xffff_d8f0u32
            } else {
                let decibels = fn_0082ffd0(e, volume) as f64 * e.global::<f64>(0x0107_3490);
                e.call(FTOL, &args![decibels]).u32()
            };
            e.vcall(audio, 0x1c, &args![level]);
        }

        if view.synch_start != 0 {
            let current = position_ms(e, ebp, 0x24);
            let clock = audio_clock(e);
            let mut expected = clock.wrapping_sub(view.synch_start);
            while expected > duration {
                expected -= duration;
            }
            if clock < view.synch_start {
                expected = 0;
                e.call(LOG, &args![0x0107_ccc0u32, clock, view.synch_start]);
            }
            let difference = current.abs_diff(expected);
            if difference > tolerance && ((difference as i32) < 80000 || current == 0) {
                e.call(LOG, &args![0x0107_cc8cu32, difference, expected, current]);
                expected = expected.wrapping_add(50);
                let graph = e.mem.u32(ebp - frame::GRAPH);
                let position_object = e.mem.u32(ebp - frame::POSITION);
                if graph != 0 && position_object != 0 {
                    let seconds = expected as f64 / thousand;
                    e.vcall(position_object, 0x20, &args![seconds]);
                }
            }
        }

        let builder = e.mem.u32(ebp - frame::BUILDER);
        let control = e.mem.u32(ebp - frame::CONTROL);
        let position_object = e.mem.u32(ebp - frame::POSITION);
        if view.flags & 2 != 0 {
            if builder != 0 && control != 0 {
                e.vcall(control, 0x24, &[]);
            }
        } else {
            let pause = view.flags & 4 != 0 || global_byte(e, PAUSE_BYTE) != 0 || {
                let audio = e.call(AUDIO_INSTANCE, &[]).u32();
                fn_00831de0(e, Ptr::new(audio))
            };
            if pause {
                if builder != 0 && control != 0 {
                    e.vcall(control, 0x20, &[]);
                }
            } else if position_object != 0 && control != 0 {
                e.vcall(control, 0x1c, &[]);
            }
        }

        let events = e.mem.u32(ebp - frame::EVENTS);
        if events == 0 {
            playing = false;
        } else {
            loop {
                let events = e.mem.u32(ebp - frame::EVENTS);
                let result = e
                    .vcall(
                        events,
                        0x20,
                        &args![
                            ebp - frame::EVENT_CODE,
                            ebp - frame::EVENT_PARAM_1,
                            ebp - frame::EVENT_PARAM_2,
                            0u32
                        ],
                    )
                    .u32();
                if failed(result) {
                    break;
                }
                let code = e.mem.i32(ebp - frame::EVENT_CODE);
                if code > 0 && code <= 3 {
                    let first_param = e.mem.u32(ebp - frame::EVENT_PARAM_1);
                    let second_param = e.mem.u32(ebp - frame::EVENT_PARAM_2);
                    e.vcall(events, 0x30, &args![code, first_param, second_param]);
                    if view.flags & 0x20 != 0 {
                        let builder = e.mem.u32(ebp - frame::BUILDER);
                        let control = e.mem.u32(ebp - frame::CONTROL);
                        if builder != 0 && control != 0 {
                            e.vcall(control, 0x24, &[]);
                        }
                        let graph = e.mem.u32(ebp - frame::GRAPH);
                        let position_object = e.mem.u32(ebp - frame::POSITION);
                        if graph != 0 && position_object != 0 {
                            e.vcall(position_object, 0x20, &args![0.0f64]);
                        }
                        let position_object = e.mem.u32(ebp - frame::POSITION);
                        let control = e.mem.u32(ebp - frame::CONTROL);
                        if view.flags & 2 == 0 && position_object != 0 && control != 0 {
                            e.vcall(control, 0x1c, &[]);
                        }
                    } else {
                        playing = false;
                    }
                }
                let code = e.mem.u32(ebp - frame::EVENT_CODE);
                let first_param = e.mem.u32(ebp - frame::EVENT_PARAM_1);
                let second_param = e.mem.u32(ebp - frame::EVENT_PARAM_2);
                let events = e.mem.u32(ebp - frame::EVENTS);
                e.vcall(events, 0x30, &args![code, first_param, second_param]);
            }
        }

        fn_00830470(e);
        if thread_should_stop(e, channel, id) {
            playing = false;
        }
        fn_00830490(e);
    }

    fn_00830470(e);
    if channel == 0 && id == e.global::<u32>(CHANNEL_A.id) {
        clear_channel(e, &CHANNEL_A, &channel_words(&CHANNEL_A));
    } else if channel == 1 && id == e.global::<u32>(CHANNEL_B.id) {
        clear_channel(e, &CHANNEL_B, &channel_words(&CHANNEL_B));
    }
    let count: u32 = e.global(THREAD_COUNT);
    e.set_global(THREAD_COUNT, count.wrapping_sub(1));
    fn_00830490(e);

    for slot in [
        frame::EVENTS,
        frame::POSITION,
        frame::CONTROL,
        frame::AUDIO,
        frame::BUILDER,
        frame::GRAPH,
    ] {
        let object = e.mem.u32(ebp - slot);
        if object != 0 {
            e.vcall(object, 8, &[]);
            e.mem.set_u32(ebp - slot, 0);
        }
    }
    if reader != 0 {
        e.vcall(reader + 0xc, 8, &[]);
    }
    if stream != 0 {
        e.vcall(stream, 0, &args![1u32]);
    }
    0
}

// Translated from 00831de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00831e00` on the member at +0x14 of `this`
/// (`ThreadProc` calls it on `BSAudio::QInstance()`; nonzero pauses).
pub fn fn_00831de0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0083_1e00, &args![this.addr() + 0x14]).bool()
}

// Translated from 00831e20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00831e20 - C++ exception frame (FS:[0] unwind of the base constructor) not translated
/// `CBSAReader` constructor (class from the RTTI at its vtables): base
/// constructor `009fddf0(0, 0, stream, result)`, the three vtables (+0x00
/// `0107cdac`, +0x0C `0107cd6c`, +0x10 `0107cd54`), then `009f9d30` on the
/// member at +0x1A0 with `media_type`. Returns `this`.
pub fn fn_00831e20(e: &mut Engine, this: Ptr, stream: Ptr, media_type: Ptr, result: Ptr) -> Ptr {
    let this_addr = this.addr();
    e.call(0x009f_ddf0, &args![this, 0u32, 0u32, stream, result]);
    e.mem.set_u32(this_addr, 0x0107_cdac);
    e.mem.set_u32(this_addr + 0xc, 0x0107_cd6c);
    e.mem.set_u32(this_addr + 0x10, 0x0107_cd54);
    e.call(0x009f_9d30, &args![this_addr + 0x1a0, media_type]);
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0082f010, fn_0082f010(Ptr, Ptr) -> Ptr),
        entry!(0x0082f6c0, fn_0082f6c0(Ptr<FamThread>)),
        entry!(0x0082f790, fam_thread_wait(Ptr<FamThread>) -> u32),
        entry!(0x0082f7b0, fam_thread_thread_proc(Ptr<FamThread>) -> u32),
        entry!(0x0082f830, fallout_audio_media_startup()),
        entry!(
            0x0082f950,
            fam_thread_vector_deleting_destructor(Ptr<FamThread>, u32) -> u32
        ),
        entry!(0x0082f9c0, fn_0082f9c0()),
        entry!(0x0082fa30, fallout_audio_media_media_clear()),
        entry!(0x0082fb70, fn_0082fb70()),
        entry!(0x0082fec0, fn_0082fec0(i32, f32) -> f32),
        entry!(0x0082ffd0, fn_0082ffd0(f32) -> f32),
        entry!(0x0082fff0, fn_0082fff0(f32) -> f32),
        entry!(0x00830010, fn_00830010(Ptr)),
        entry!(0x00830050, fn_00830050(Ptr) -> bool),
        entry!(0x00830080, fn_00830080(Ptr) -> f32),
        entry!(0x008300c0, fn_008300c0(i32, Ptr, u32, u8, u8, f32, u32)),
        entry!(0x00830470, fn_00830470()),
        entry!(0x00830490, fn_00830490()),
        entry!(0x008304a0, fallout_audio_media_media_close()),
        entry!(0x008304c0, fn_008304c0()),
        entry!(0x008304f0, fn_008304f0()),
        entry!(0x008305e0, fallout_audio_media_media_pause()),
        entry!(0x00830610, fallout_audio_media_media_un_pause()),
        entry!(0x00830640, fn_00830640()),
        entry!(0x00830660, fn_00830660()),
        entry!(0x00830680, fn_00830680(u32, i32)),
        entry!(0x00830750, fallout_audio_media_media_is_playing(i32, bool) -> bool),
        entry!(0x008308c0, fn_008308c0(i32) -> u32),
        entry!(0x00830970, fn_00830970(i32) -> u32),
        entry!(0x00830a20, fn_00830a20(i32) -> u32),
        entry!(0x00830ad0, fn_00830ad0() -> bool),
        entry!(0x00830b30, fallout_audio_media_thread_proc(Ptr) -> u32),
        entry!(0x00831de0, fn_00831de0(Ptr) -> bool),
        entry!(0x00831e20, fn_00831e20(Ptr, Ptr, Ptr, Ptr) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Log = Vec<(u32, Vec<u32>)>;

    const AUDIO: u32 = 0x0000_5000;
    const NOW: u32 = 1500;

    fn ret(eax: u32) -> Ret {
        Ret {
            eax,
            ..Ret::default()
        }
    }

    fn st0(value: f64) -> Ret {
        Ret {
            st0: value,
            ..Ret::default()
        }
    }

    fn noop(_: &mut Engine, _: &[u32]) -> Ret {
        Ret::default()
    }

    fn word_f64(a: &[u32], i: usize) -> f64 {
        f64::from_bits(a[i] as u64 | (a[i + 1] as u64) << 32)
    }

    fn calls_to(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log: &Log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    fn put_f64(e: &mut Engine, addr: u32, v: f64) {
        e.map(addr, 8);
        e.set_global(addr, v);
    }

    fn put_f32(e: &mut Engine, addr: u32, v: f32) {
        e.map(addr, 4);
        e.set_global(addr, v);
    }

    /// The statics, the lock, `strcpy_s`, the clock and the constants.
    fn setup() -> Engine {
        let mut e = Engine::new();
        e.map(0x011d_cf00, 0x600);
        e.map(0x011d_ea3c, 4);
        e.map(0x011f_1958, 4);
        e.map(NEXT_MEDIA_ID, 4);
        e.map(NONE_TEXT, 8);
        e.mem.set_cstr(NONE_TEXT, b"NONE");
        e.register(CRITICAL_SECTION_ENTER, noop);
        e.register(CRITICAL_SECTION_LEAVE, noop);
        e.register(STRCPY_S, |e, a| {
            let s = e.mem.cstr(a[2]);
            e.mem.set_cstr(a[0], &s);
            Ret::default()
        });
        e.register(AUDIO_INSTANCE, |_, _| ret(AUDIO));
        e.register(AUDIO_CLOCK, |_, _| ret(NOW));
        e.register(LOG, noop);
        for (addr, v) in [
            (0x0101_2060, 0.0),
            (0x0107_3490, 2000.0),
            (0x0101_7a40, 100.0),
            (0x0102_d268, -10000.0),
            (0x0101_7b70, 1000.0),
            (0x0101_2070, 1.0),
            (0x0103_c7c8, 0.05f32 as f64),
            (0x0101_6408, 0.01f32 as f64),
            (0x0103_2980, 0.0001f32 as f64),
        ] {
            put_f64(&mut e, addr, v);
        }
        for (addr, v) in [
            (0x0107_cc20, -10000.0f32),
            (0x0101_7b78, 10.0),
            (0x0101_41cc, 0.0001),
            (0x0101_6410, 100.0),
        ] {
            put_f32(&mut e, addr, v);
        }
        e.call_log = Some(vec![]);
        e
    }

    /// `0082fec0`'s callees with real math: `log10`, `pow`, a base level
    /// and a master volume.
    fn volume_math(e: &mut Engine, base: f64, master: f64) {
        e.register(0x00ec_bc80, |_, a| st0(word_f64(a, 0).log10()));
        e.register(0x004d_d130, |_, a| {
            let base = f32::from_bits(a[0]) as f64;
            let exponent = f32::from_bits(a[1]) as f64;
            st0(base.powf(exponent))
        });
        e.register_double(0x00ad_7ab0, move |_, _| st0(base));
        e.register_double(0x00ad_7ae0, move |_, _| st0(base));
        e.register_double(0x00ad_79f0, move |_, _| st0(master));
    }

    #[test]
    fn stream_keeps_a_usable_file_and_deletes_a_bad_one() {
        for usable in [true, false] {
            let mut e = setup();
            for addr in [0x0082_f240, 0x0044_fae0, 0x0000_7001] {
                e.register(addr, noop);
            }
            e.put_vtable(0x6000_0000, &[0x0000_7001]);
            let file = e.mem.alloc(4);
            e.mem.set_u32(file, 0x6000_0000);
            e.register_double(0x00af_df20, move |_, _| ret(file));
            e.register_double(0x0089_05f0, move |_, _| ret(usable as u32));
            let this = e.mem.alloc(0x138);
            let name = e.mem.alloc(16);
            e.mem.set_cstr(name, b"a.mp3");
            e.call(0x0082_f010, &args![this, name]);
            assert_eq!(e.mem.u32(this), 0x0107_cb4c);
            assert_eq!(e.mem.cstr(this + 4), b"a.mp3");
            assert_eq!(
                calls_to(&e, 0x00af_df20),
                vec![vec![this + 4, 0, 0x4000, 8]]
            );
            assert_eq!(calls_to(&e, 0x0044_fae0), vec![vec![this + 0x10c]]);
            if usable {
                assert_eq!(e.mem.u32(this + 0x108), file);
                assert!(calls_to(&e, 0x0000_7001).is_empty());
            } else {
                assert_eq!(e.mem.u32(this + 0x108), 0);
                assert_eq!(calls_to(&e, 0x0000_7001), vec![vec![file, 1]]);
            }
        }
    }

    #[test]
    fn thread_destructor_waits_for_the_thread_to_finish() {
        let mut e = setup();
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this + 0x30, 0x77);
        e.mem.set_u32(this + 0x3c, 3);
        e.register(FAM_THREAD_START, noop);
        e.register(CLOSE_HANDLE, noop);
        e.register(0x00aa_63c0, noop);
        e.register_double(0x0040_fca0, move |e, _| {
            let state = e.mem.u32(this + 0x3c);
            e.mem.set_u32(this + 0x3c, state & !2);
            Ret::default()
        });
        e.call(0x0082_f6c0, &args![this]);
        assert_eq!(e.mem.u32(this), 0x0107_cbe8);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(
            calls_to(&e, FAM_THREAD_START),
            vec![vec![this, this + 0x34]]
        );
        assert_eq!(calls_to(&e, 0x0040_fca0), vec![vec![10]]);
        assert_eq!(calls_to(&e, CLOSE_HANDLE), vec![vec![0x77]]);
        assert_eq!(calls_to(&e, 0x00aa_63c0), vec![vec![this]]);
    }

    #[test]
    fn wait_waits_forever_on_the_event() {
        let mut e = setup();
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this + 0x30, 0x55);
        e.register(WAIT_FOR_SINGLE_OBJECT, |_, _| ret(0));
        e.call(0x0082_f790, &args![this]);
        assert_eq!(
            calls_to(&e, WAIT_FOR_SINGLE_OBJECT),
            vec![vec![0x55, 0xffff_ffff]]
        );
    }

    /// `FalloutAudioMedia::ThreadProc`'s callees for a stream that does not
    /// open.
    fn failing_stream(e: &mut Engine) {
        let setting = e.mem.alloc(4);
        e.register_double(0x0043_d4d0, move |_, _| ret(setting));
        e.register(0x0064_7b70, |_, _| ret(500));
        e.register(ALLOCATE, |_, _| ret(0));
        e.register(0x0040_4d30, |_, _| ret(0));
    }

    #[test]
    fn thread_loop_plays_while_run_is_set() {
        let mut e = setup();
        failing_stream(&mut e);
        let this = e.mem.alloc(0x4c);
        e.mem.set_u32(this + 0x34, 9);
        e.mem.set_u32(this + 0x3c, 1);
        e.register(WAIT_FOR_SINGLE_OBJECT, |_, _| ret(0));
        e.register(0x00c8_0240, noop);
        let mut setups = 0;
        e.register_double(0x00c8_0130, move |e, _| {
            setups += 1;
            if setups == 2 {
                let state = e.mem.u32(this + 0x3c);
                e.mem.set_u32(this + 0x3c, state & !1);
            }
            Ret::default()
        });
        assert_eq!(e.call(0x0082_f7b0, &args![this]).u32(), 0);
        assert_eq!(e.mem.u32(this + 0x3c), 0);
        assert_eq!(calls_to(&e, 0x00c8_0130).len(), 2);
        // One play: the stream did not open.
        assert_eq!(calls_to(&e, LOG), vec![vec![0x0107_cd2c]]);
        assert_eq!(calls_to(&e, 0x00c8_0240), vec![vec![this + 0x40]]);
    }

    #[test]
    fn startup_makes_and_starts_two_threads() {
        let mut e = setup();
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        for addr in [0x00ec_782f, 0x00aa_6430, 0x00aa_6560, 0x00aa_6550] {
            e.register(addr, noop);
        }
        e.call(0x0082_f830, &[]);
        let threads: u32 = e.global(THREADS);
        assert_eq!(e.mem.u32(threads - 4), 2);
        assert_eq!(
            calls_to(&e, 0x00ec_782f),
            vec![vec![threads, 0x4c, 2, 0x0082_f610, 0x0082_f6c0]]
        );
        assert_eq!(
            calls_to(&e, 0x00aa_6430),
            vec![
                vec![threads, 0x4000, 0x0107_cc08, 0],
                vec![threads + 0x4c, 0x4000, 0x0107_cbf0, 0]
            ]
        );
        assert_eq!(
            calls_to(&e, 0x00aa_6560),
            vec![vec![threads, 1], vec![threads + 0x4c, 1]]
        );
        assert_eq!(
            calls_to(&e, 0x00aa_6550),
            vec![vec![threads, 0], vec![threads + 0x4c, 0]]
        );
    }

    #[test]
    fn deleting_destructor_handles_arrays_and_single_threads() {
        let mut e = setup();
        e.register(0x00ec_5fce, noop);
        e.register(DEALLOCATE, noop);
        let block = e.mem.alloc(0x9c);
        e.mem.set_u32(block, 2);
        let array = block + 4;
        assert_eq!(e.call(0x0082_f950, &args![array, 3u32]).u32(), block);
        assert_eq!(
            calls_to(&e, 0x00ec_5fce),
            vec![vec![array, 0x4c, 2, 0x0082_f6c0]]
        );
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![block]]);

        let mut e = setup();
        e.register(DEALLOCATE, noop);
        for addr in [FAM_THREAD_START, CLOSE_HANDLE, 0x00aa_63c0] {
            e.register(addr, noop);
        }
        let one = e.mem.alloc(0x4c);
        assert_eq!(e.call(0x0082_f950, &args![one, 1u32]).u32(), one);
        assert_eq!(calls_to(&e, 0x00aa_63c0), vec![vec![one]]);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![one]]);
        assert!(calls_to(&e, 0x00ec_5fce).is_empty());
    }

    #[test]
    fn thread_array_is_deleted_through_its_destructor_or_freed() {
        let mut e = setup();
        e.register(DEALLOCATE, noop);
        e.register(0x0000_7002, noop);
        e.put_vtable(0x6000_1000, &[0x0000_7002]);
        let block = e.mem.alloc(0x9c);
        e.mem.set_u32(block, 2);
        e.mem.set_u32(block + 4, 0x6000_1000);
        e.set_global(THREADS, block + 4);
        e.call(0x0082_f9c0, &[]);
        assert_eq!(calls_to(&e, 0x0000_7002), vec![vec![block + 4, 3]]);
        assert_eq!(e.global::<u32>(THREADS), 0);

        e.mem.set_u32(block, 0);
        e.set_global(THREADS, block + 4);
        e.call(0x0082_f9c0, &[]);
        assert_eq!(calls_to(&e, DEALLOCATE), vec![vec![block]]);
        assert_eq!(e.global::<u32>(THREADS), 0);
    }

    fn dirty_statics(e: &mut Engine) {
        for addr in (0x011d_d300..0x011d_d370).step_by(4) {
            e.set_global(addr, 0x0101_0101u32);
        }
        e.mem.set_cstr(CHANNEL_A.file_name, b"x.mp3");
        e.mem.set_cstr(CHANNEL_B.file_name, b"y.mp3");
    }

    #[test]
    fn media_clear_resets_both_channels() {
        let mut e = setup();
        dirty_statics(&mut e);
        e.call(0x0082_fa30, &[]);
        for addr in (0x011d_d314..0x011d_d370).step_by(4) {
            assert_eq!(e.global::<u32>(addr), 0, "{addr:08x}");
        }
        for addr in [0x011d_d300, 0x011d_d304, 0x011d_d308, 0x011d_d30c] {
            assert_eq!(e.global::<u32>(addr), 0);
        }
        assert_eq!(e.global::<u8>(CHANNEL_A.flags), 0);
        assert_eq!(e.global::<u8>(CHANNEL_B.flags), 0);
        // The two bytes after the flags are not channel state.
        assert_eq!(e.global::<u8>(PAUSE_BYTE), 1);
        assert_eq!(e.global::<u8>(MEDIA_HOLD_BYTE), 1);
        assert_eq!(e.mem.cstr(CHANNEL_A.file_name), b"NONE");
        assert_eq!(e.mem.cstr(CHANNEL_B.file_name), b"NONE");
    }

    #[test]
    fn music_update_stops_on_hold_and_while_type_six_plays() {
        let mut e = setup();
        e.register(SET_EVENT, noop);
        e.set_global(UPDATE_EVENT, 0x99u32);
        e.set_global(MEDIA_HOLD_BYTE, 1u8);
        e.call(0x0082_fb70, &[]);
        assert_eq!(calls_to(&e, SET_EVENT), vec![vec![0x99]]);
        assert!(calls_to(&e, AUDIO_CLOCK).is_empty());

        e.set_global(MEDIA_HOLD_BYTE, 0u8);
        e.set_global(CHANNEL_A.id, 3u32);
        e.set_global(CHANNEL_A.kind, 6u32);
        e.set_global(MUSIC_RESET, 1u32);
        e.call(0x0082_fb70, &[]);
        assert_eq!(e.global::<u32>(MUSIC_RESET), 0);
        assert!(calls_to(&e, AUDIO_CLOCK).is_empty());
    }

    #[test]
    fn music_update_fades_everything_when_the_player_says_so() {
        let mut e = setup();
        e.register(SET_EVENT, noop);
        e.register(0x0070_27b0, |_, _| ret(0));
        e.register(0x0099_2d90, |_, _| ret(1));
        e.set_global(0x011f_1958, 0x4444u32);
        e.register(0x0000_7003, |_, _| ret(1));
        let mut slots = vec![0u32; 0x22c / 4 + 1];
        slots[0x22c / 4] = 0x0000_7003;
        e.put_vtable(0x6000_2000, &slots);
        let player = e.mem.alloc(4);
        e.mem.set_u32(player, 0x6000_2000);
        e.set_global(0x011d_ea3c, player);
        e.register(0x0096_98a0, |_, _| ret(0x123));
        e.register(0x0059_5600, |_, _| ret(1));
        e.set_global(CHANNEL_A.id, 3u32);
        e.set_global(CHANNEL_A.kind, 2u32);
        e.set_global(CHANNEL_A.volume, 0.75f32);
        e.call(0x0082_fb70, &[]);
        assert_eq!(calls_to(&e, 0x0099_2d90), vec![vec![0x4444]]);
        assert_eq!(calls_to(&e, 0x0070_27b0), vec![vec![0x3ef]]);
        assert_eq!(calls_to(&e, 0x0000_7003), vec![vec![player, 0]]);
        // 00830680(1000, 0): every channel fades out over a second.
        assert_eq!(e.global::<u8>(CHANNEL_A.flags), 0x18);
        assert_eq!(e.global::<f32>(CHANNEL_A.fade_start_volume), 0.75);
        assert_eq!(e.global::<u32>(CHANNEL_A.fade_time), 1000);
        assert_eq!(e.global::<u32>(CHANNEL_A.fade_end), NOW + 1000);
        assert_eq!(e.global::<u8>(CHANNEL_B.flags), 0x18);
    }

    #[test]
    fn music_update_starts_the_region_media() {
        let mut e = setup();
        e.register(SET_EVENT, noop);
        e.register(0x0070_27b0, |_, _| ret(0));
        e.set_global(0x011d_ea3c, 0x3000u32);
        e.register(0x0096_98a0, |_, _| ret(0));
        e.set_global(0x011d_cfb4, 0x3100u32);
        e.register(0x0067_33e0, |_, _| ret(0x3200));
        e.register(0x0096_11e0, |_, _| ret(0x3300));
        let region = e.mem.alloc(4);
        e.register_double(0x004f_35b0, move |_, _| ret(region));
        e.register(0x0000_7030, |_, _| ret(0x3400));
        let mut slots = [0u32; 13];
        slots[12] = 0x0000_7030;
        e.put_vtable(0x6000_3000, &slots);
        e.mem.set_u32(region, 0x6000_3000);
        e.register(0x0059_7990, |_, _| ret(0));
        e.set_global(MUSIC_RESET, 1u32);
        e.call(0x0082_fb70, &[]);
        assert_eq!(e.global::<u32>(MUSIC_RESET), 0);
        assert_eq!(calls_to(&e, 0x0096_11e0), vec![vec![0x3200]]);
        assert_eq!(calls_to(&e, 0x004f_35b0), vec![vec![0x3300, 7]]);
        assert_eq!(calls_to(&e, 0x0000_7030).len(), 2);
        assert_eq!(calls_to(&e, 0x0059_7990), vec![vec![0x3400, NOW, 0, 0]]);
    }

    #[test]
    fn media_volume_from_level_attenuation_and_master() {
        let mut e = setup();
        volume_math(&mut e, 1.0, 1.0);
        let level = e.mem.alloc(4);
        e.mem.set_f32(level, 0.1);
        e.register_double(0x0040_3e20, move |_, _| ret(level));
        let v = e.call(0x0082_fec0, &args![8i32, -5.0f32]).f32();
        let decibels = ((0.1f32 as f64).log10() as f32 as f64 * 2000.0 - 500.0) as f32;
        let exponent = (decibels as f64 / 2000.0) as f32;
        assert_eq!(v, 10f64.powf(exponent as f64) as f32);
        assert_eq!(calls_to(&e, 0x0040_3e20), vec![vec![0x011d_d084]]);

        // Radio type, a silent master: clamped to 0.0001.
        let mut e = setup();
        volume_math(&mut e, 1.0, 0.0);
        assert_eq!(e.call(0x0082_fec0, &args![7i32, 0.0f32]).f32(), 0.0001);
        assert_eq!(calls_to(&e, 0x00ad_7ae0), vec![vec![AUDIO]]);

        // Other types, positive decibels capped at 0, a loud master: 100.
        let mut e = setup();
        volume_math(&mut e, 1.0, 500.0);
        assert_eq!(e.call(0x0082_fec0, &args![1i32, 3.0f32]).f32(), 100.0);
        assert_eq!(
            calls_to(&e, 0x004d_d130),
            vec![vec![10.0f32.to_bits(), 0.0f32.to_bits()]]
        );

        // Far below -10000 dB: clamped to -10000.
        let mut e = setup();
        volume_math(&mut e, 1.0, 1.0);
        e.call(0x0082_fec0, &args![1i32, -500.0f32]);
        assert_eq!(calls_to(&e, 0x004d_d130)[0][1], (-5.0f32).to_bits());
    }

    #[test]
    fn decibel_helpers_pass_a_double_and_store_a_float() {
        let mut e = setup();
        e.register(0x00ec_bc80, |_, a| st0(word_f64(a, 0) + 0.1));
        assert_eq!(e.call(0x0082_ffd0, &args![2.0f32]).f32(), 2.1f64 as f32);
        assert_eq!(e.call(0x0082_fff0, &args![4.0f32]).f32(), 4.1f64 as f32);
        let words = calls_to(&e, 0x00ec_bc80);
        assert_eq!(word_f64(&words[0], 0), 2.0);
        assert_eq!(word_f64(&words[1], 0), 4.0);
    }

    /// Doubles for `008300c0`'s file check and the thread start.
    fn open_doubles(e: &mut Engine, exists: bool) {
        e.register(0x00b0_0260, noop);
        e.register(0x00af_f240, noop);
        e.register_double(0x00af_fc60, move |_, _| ret(exists as u32));
        e.register(FAM_THREAD_START, noop);
        volume_math(e, 1.0, 1.0);
    }

    #[test]
    fn media_object_opens_its_file_as_type_six() {
        let mut e = setup();
        e.call(0x0083_0010, &args![0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);

        let mut e = setup();
        open_doubles(&mut e, true);
        let object = e.mem.alloc(0x28);
        e.mem.set_f32(object + 0x24, 2.0);
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"m.mp3");
        e.register_double(0x0059_1a40, move |_, _| ret(name));
        e.set_global(THREADS, 0x8000u32);
        e.call(0x0083_0010, &args![object]);
        assert_eq!(calls_to(&e, 0x0059_1a40), vec![vec![object]]);
        assert_eq!(e.global::<u32>(CHANNEL_A.kind), 6);
        assert_eq!(e.global::<u8>(CHANNEL_A.flags), 0x30);
        assert_eq!(e.global::<f32>(CHANNEL_A.attenuation), -2.0);
        assert_eq!(e.global::<u32>(CHANNEL_A.fade_time), 1000);
        assert_eq!(e.global::<u32>(CHANNEL_A.synch_start), 0);
        assert_eq!(e.mem.cstr(CHANNEL_A.file_name), b"m.mp3");
    }

    #[test]
    fn float_at_0x24_sign_tests() {
        let mut e = setup();
        let p = e.mem.alloc(0x28);
        e.mem.set_f32(p + 0x24, 3.0);
        assert!(e.call(0x0083_0050, &args![p]).bool());
        assert_eq!(e.call(0x0083_0080, &args![p]).f32(), -3.0);
        e.mem.set_f32(p + 0x24, -2.0);
        assert!(!e.call(0x0083_0050, &args![p]).bool());
        assert_eq!(e.call(0x0083_0080, &args![p]).f32(), -2.0);
        e.mem.set_f32(p + 0x24, 0.0);
        assert!(!e.call(0x0083_0050, &args![p]).bool());
        e.mem.set_u32(p + 0x24, 0x7f80_0001);
        assert!(!e.call(0x0083_0050, &args![p]).bool());
        let nan = fn_00830080(&mut e, Ptr::new(p));
        assert_eq!(nan.to_bits(), 0x7fc0_0001);
    }

    fn open(e: &mut Engine, kind: i32, name: u32, looping: u8, force: u8) {
        e.call(
            0x0083_00c0,
            &args![kind, name, 2000u32, looping, force, -1.0f32, 77u32],
        );
    }

    #[test]
    fn open_refuses_blocked_empty_and_lower_priority_media() {
        let mut e = setup();
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"m.mp3");
        let empty = e.mem.alloc(4);
        e.set_global(OPEN_BLOCKED_BYTE, 1u8);
        open(&mut e, 6, name, 0, 0);
        assert!(calls_to(&e, LOG).is_empty());
        assert_eq!(calls_to(&e, CRITICAL_SECTION_LEAVE).len(), 1);

        e.set_global(OPEN_BLOCKED_BYTE, 0u8);
        open(&mut e, 6, 0, 0, 0);
        open(&mut e, 6, empty, 0, 0);
        assert!(calls_to(&e, LOG).is_empty());

        e.set_global(NEXT_CHANNEL, 1u32);
        e.set_global(CHANNEL_A.kind, 7u32);
        open(&mut e, 6, name, 0, 0);
        assert_eq!(calls_to(&e, LOG), vec![vec![0x0107_cc54]]);

        open_doubles(&mut e, false);
        open(&mut e, 6, name, 0, 1);
        assert_eq!(calls_to(&e, 0x00b0_0260)[0][1..], [name, 0, 0x4000, 0]);
        assert_eq!(calls_to(&e, LOG)[1], vec![0x0107_cc24, name]);
        assert_eq!(calls_to(&e, 0x00af_f240).len(), 1);
        assert!(calls_to(&e, FAM_THREAD_START).is_empty());

        dirty_statics(&mut e);
        open(&mut e, 9, name, 0, 0);
        assert_eq!(e.global::<u32>(CHANNEL_A.id), 0);
        assert_eq!(e.global::<u32>(NEXT_CHANNEL), 0);
    }

    #[test]
    fn open_takes_the_next_channel_and_fades_the_other_out() {
        let mut e = setup();
        open_doubles(&mut e, true);
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"m.mp3");
        e.set_global(THREADS, 0x8000u32);
        e.set_global(NEXT_MEDIA_ID, 40u32);
        e.set_global(CHANNEL_B.volume, 0.5f32);
        open(&mut e, 6, name, 0, 0);
        assert_eq!(e.global::<u8>(CHANNEL_A.flags), 0x10);
        assert_eq!(e.global::<u32>(CHANNEL_A.id), 40);
        assert_eq!(e.global::<u32>(NEXT_MEDIA_ID), 41);
        assert_eq!(e.global::<u32>(CHANNEL_A.synch_start), 77);
        assert_eq!(e.global::<u32>(CHANNEL_A.fade_end), NOW + 2000);
        // log10(1) * 2000 - 100 = -100 dB: 10^(-0.05).
        let expected = 10f64.powf((-100.0f64 / 2000.0) as f32 as f64) as f32;
        assert_eq!(e.global::<f32>(CHANNEL_A.fade_end_volume), expected);
        assert_eq!(
            calls_to(&e, FAM_THREAD_START),
            vec![vec![0x8000, THREAD_INFO]]
        );
        assert_eq!(e.global::<u32>(THREAD_INFO), 40);
        assert_eq!(e.global::<u32>(THREAD_INFO_CHANNEL), 0);
        assert_eq!(e.global::<u32>(NEXT_CHANNEL), 1);
        assert_eq!(e.global::<u8>(CHANNEL_B.flags), 0x18);
        assert_eq!(e.global::<f32>(CHANNEL_B.fade_start_volume), 0.5);
        assert_eq!(e.global::<u32>(CHANNEL_B.fade_end), NOW + 2000);

        open(&mut e, 6, name, 1, 0);
        assert_eq!(e.global::<u8>(CHANNEL_B.flags), 0x30);
        assert_eq!(e.global::<u32>(CHANNEL_B.id), 41);
        assert_eq!(
            calls_to(&e, FAM_THREAD_START)[1],
            vec![0x8000 + 0x4c, THREAD_INFO]
        );
        assert_eq!(e.global::<u32>(THREAD_INFO_CHANNEL), 1);
        assert_eq!(e.global::<u32>(NEXT_CHANNEL), 0);
        assert_eq!(e.global::<u8>(CHANNEL_A.flags), 0x18);
        assert_eq!(e.global::<u32>(CHANNEL_A.fade_time), 2000);
    }

    #[test]
    fn lock_and_unlock_use_the_critical_section() {
        let mut e = setup();
        e.call(0x0083_0470, &[]);
        e.call(0x0083_0490, &[]);
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_ENTER),
            vec![vec![CRITICAL_SECTION, 0]]
        );
        assert_eq!(
            calls_to(&e, CRITICAL_SECTION_LEAVE),
            vec![vec![CRITICAL_SECTION]]
        );
    }

    #[test]
    fn media_close_clears_under_the_lock() {
        let mut e = setup();
        dirty_statics(&mut e);
        e.call(0x0083_04a0, &[]);
        assert_eq!(e.global::<u32>(CHANNEL_B.id), 0);
        assert_eq!(calls_to(&e, CRITICAL_SECTION_ENTER).len(), 1);
        assert_eq!(calls_to(&e, CRITICAL_SECTION_LEAVE).len(), 1);
    }

    #[test]
    fn flag_setters_change_both_channels() {
        let mut e = setup();
        e.set_global(CHANNEL_A.flags, 0xffu8);
        e.set_global(CHANNEL_B.flags, 0x02u8);
        e.call(0x0083_04c0, &[]);
        assert_eq!(e.global::<u8>(CHANNEL_A.flags), 0xfd);
        assert_eq!(e.global::<u8>(CHANNEL_B.flags), 0);
        e.call(0x0083_05e0, &[]);
        assert_eq!(e.global::<u8>(CHANNEL_A.flags), 0xfd);
        assert_eq!(e.global::<u8>(CHANNEL_B.flags), 4);
        e.call(0x0083_0610, &[]);
        assert_eq!(e.global::<u8>(CHANNEL_A.flags), 0xf9);
        assert_eq!(e.global::<u8>(CHANNEL_B.flags), 0);
        e.call(0x0083_0640, &[]);
        assert_eq!(e.global::<u8>(PAUSE_BYTE), 1);
        e.call(0x0083_0660, &[]);
        assert_eq!(e.global::<u8>(PAUSE_BYTE), 0);
        assert_eq!(calls_to(&e, CRITICAL_SECTION_LEAVE).len(), 5);
    }

    #[test]
    fn stop_hits_the_channel_last_opened() {
        for next in [0u32, 1] {
            let mut e = setup();
            volume_math(&mut e, 1.0, 1.0);
            dirty_statics(&mut e);
            e.set_global(CHANNEL_A.flags, 0u8);
            e.set_global(CHANNEL_B.flags, 0u8);
            e.set_global(CHANNEL_A.kind, 1u32);
            e.set_global(CHANNEL_B.kind, 1u32);
            e.set_global(CHANNEL_A.attenuation, 0.0f32);
            e.set_global(CHANNEL_B.attenuation, 0.0f32);
            e.set_global(NEXT_CHANNEL, next);
            e.call(0x0083_04f0, &[]);
            let (stopped, kept) = if next == 0 {
                (&CHANNEL_B, &CHANNEL_A)
            } else {
                (&CHANNEL_A, &CHANNEL_B)
            };
            assert_eq!(e.global::<u8>(stopped.flags), 2);
            assert_eq!(e.global::<u8>(kept.flags), 1);
            assert_eq!(e.global::<f32>(stopped.volume), 1.0);
            for w in [
                stopped.synch_start,
                stopped.fade_time,
                stopped.fade_end,
                stopped.current_position,
            ] {
                assert_eq!(e.global::<u32>(w), 0);
            }
            assert_eq!(e.global::<u32>(kept.fade_time), 0x0101_0101);
        }
    }

    #[test]
    fn fade_out_by_type_skips_channels_already_fading() {
        let mut e = setup();
        e.set_global(CHANNEL_A.kind, 3u32);
        e.set_global(CHANNEL_B.kind, 4u32);
        e.set_global(CHANNEL_A.volume, 0.25f32);
        e.call(0x0083_0680, &args![700u32, 3i32]);
        assert_eq!(e.global::<u8>(CHANNEL_A.flags), 0x18);
        assert_eq!(e.global::<f32>(CHANNEL_A.fade_start_volume), 0.25);
        assert_eq!(e.global::<u32>(CHANNEL_A.fade_end), NOW + 700);
        assert_eq!(e.global::<u8>(CHANNEL_B.flags), 0);

        e.set_global(CHANNEL_A.fade_time, 1u32);
        e.call(0x0083_0680, &args![900u32, 0i32]);
        assert_eq!(e.global::<u32>(CHANNEL_A.fade_time), 1);
        assert_eq!(e.global::<u32>(CHANNEL_B.fade_time), 900);
        assert_eq!(e.global::<u8>(CHANNEL_B.flags), 0x18);
    }

    #[test]
    fn is_playing_checks_id_type_and_the_chosen_flag() {
        let mut e = setup();
        let playing = |e: &mut Engine, kind: i32, stop_flag: bool| {
            e.call(0x0083_0750, &args![kind, stop_flag]).bool()
        };
        assert!(!playing(&mut e, 0, true));
        e.set_global(CHANNEL_B.id, 2u32);
        e.set_global(CHANNEL_B.kind, 5u32);
        assert!(playing(&mut e, 0, true));
        assert!(playing(&mut e, 5, false));
        assert!(!playing(&mut e, 4, true));
        e.set_global(CHANNEL_B.flags, 8u8);
        assert!(playing(&mut e, 5, true));
        assert!(!playing(&mut e, 5, false));
        e.set_global(CHANNEL_B.flags, 2u8);
        assert!(!playing(&mut e, 0, true));
        assert!(playing(&mut e, 0, false));
    }

    #[test]
    fn last_opened_words_follow_the_next_channel() {
        let mut e = setup();
        e.set_global(CHANNEL_A.id, 1u32);
        e.set_global(CHANNEL_A.kind, 2u32);
        e.set_global(CHANNEL_A.duration, 10u32);
        e.set_global(CHANNEL_A.sub_duration, 11u32);
        e.set_global(CHANNEL_A.current_position, 12u32);
        e.set_global(CHANNEL_B.id, 3u32);
        e.set_global(CHANNEL_B.kind, 4u32);
        e.set_global(CHANNEL_B.duration, 20u32);
        e.set_global(CHANNEL_B.sub_duration, 21u32);
        e.set_global(CHANNEL_B.current_position, 22u32);
        e.set_global(NEXT_CHANNEL, 1u32);
        assert_eq!(e.call(0x0083_08c0, &args![0i32]).u32(), 10);
        assert_eq!(e.call(0x0083_0970, &args![2i32]).u32(), 11);
        assert_eq!(e.call(0x0083_0a20, &args![4i32]).u32(), 0);
        e.set_global(NEXT_CHANNEL, 0u32);
        assert_eq!(e.call(0x0083_08c0, &args![4i32]).u32(), 20);
        assert_eq!(e.call(0x0083_0970, &args![0i32]).u32(), 21);
        assert_eq!(e.call(0x0083_0a20, &args![0i32]).u32(), 22);
        e.set_global(CHANNEL_B.id, 0u32);
        assert_eq!(e.call(0x0083_0a20, &args![0i32]).u32(), 0);
    }

    #[test]
    fn radio_or_hold_byte_counts() {
        let mut e = setup();
        assert!(!e.call(0x0083_0ad0, &[]).bool());
        e.set_global(CHANNEL_B.kind, 7u32);
        assert!(!e.call(0x0083_0ad0, &[]).bool());
        e.set_global(CHANNEL_B.id, 1u32);
        assert!(e.call(0x0083_0ad0, &[]).bool());
        e.set_global(CHANNEL_B.id, 0u32);
        e.set_global(MEDIA_HOLD_BYTE, 1u8);
        assert!(e.call(0x0083_0ad0, &[]).bool());
    }

    #[test]
    fn thread_proc_without_a_stream_clears_its_channel() {
        let mut e = setup();
        failing_stream(&mut e);
        let info = e.mem.alloc(8);
        e.mem.set_u32(info, 4);
        e.mem.set_u32(info + 4, 1);
        dirty_statics(&mut e);
        e.set_global(CHANNEL_B.id, 4u32);
        e.set_global(CHANNEL_B.flags, 0u8);
        e.set_global(THREAD_COUNT, 2u32);
        assert_eq!(e.call(0x0083_0b30, &args![info]).u32(), 0);
        assert_eq!(calls_to(&e, LOG), vec![vec![0x0107_cd2c]]);
        assert_eq!(calls_to(&e, 0x0040_4d30), vec![vec![0]]);
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![0x138]]);
        assert_eq!(calls_to(&e, 0x0064_7b70), vec![vec![500, 500]]);
        assert_eq!(e.global::<u32>(CHANNEL_B.id), 0);
        assert_eq!(e.mem.cstr(CHANNEL_B.file_name), b"NONE");
        assert_eq!(e.global::<u32>(CHANNEL_A.id), 0x0101_0101);
        assert_eq!(e.global::<u32>(THREAD_COUNT), 2);
    }

    // COM objects of the full play test.
    const GRAPH: u32 = 0x6100_0000;
    const BUILDER: u32 = 0x6100_0100;
    const AUDIO_OBJECT: u32 = 0x6100_0200;
    const CONTROL: u32 = 0x6100_0300;
    const POSITION: u32 = 0x6100_0400;
    const EVENTS: u32 = 0x6100_0500;
    const COM_VTABLE: u32 = 0x6200_0000;
    const COM_SLOT: u32 = 0x7300_0000;

    /// Every interface the play test gets, dispatched on `this` and the IID.
    fn com_doubles(e: &mut Engine) {
        let slots: Vec<u32> = (0..13).map(|i| COM_SLOT + 4 * i).collect();
        e.put_vtable(COM_VTABLE, &slots);
        for object in [GRAPH, BUILDER, AUDIO_OBJECT, CONTROL, POSITION, EVENTS] {
            e.map(object, 4);
            e.mem.set_u32(object, COM_VTABLE);
        }
        e.register(CO_CREATE_INSTANCE, |e, a| {
            e.mem.set_u32(a[4], GRAPH);
            ret(0)
        });
        e.register(COM_SLOT, |e, a| {
            let object = match a[1] {
                0x0109_3b5c => BUILDER,
                0x0107_cad8 => AUDIO_OBJECT,
                0x0107_caa8 => CONTROL,
                0x0107_cac8 => POSITION,
                _ => EVENTS,
            };
            e.mem.set_u32(a[2], object);
            ret(0)
        });
        for slot in [8, 0xc, 0x30] {
            e.register(COM_SLOT + slot, |_, _| ret(0));
        }
        e.register(COM_SLOT + 0x1c, |e, a| {
            if a[0] == POSITION {
                e.mem.set_f64(a[1], 10.0);
            }
            ret(0)
        });
        e.register(COM_SLOT + 0x24, |e, a| {
            if a[0] == POSITION {
                e.mem.set_f64(a[1], 1.5);
            }
            ret(0)
        });
        let mut events = 0;
        e.register_double(COM_SLOT + 0x20, move |e, a| {
            if a[0] != EVENTS {
                return ret(0);
            }
            events += 1;
            if events == 1 {
                e.mem.set_u32(a[1], 1);
                e.mem.set_u32(a[2], 0x10);
                e.mem.set_u32(a[3], 0x20);
                ret(0)
            } else {
                ret(0x8000_4005)
            }
        });
    }

    #[test]
    fn thread_proc_plays_fades_synchs_and_stops_at_the_end() {
        let mut e = setup();
        volume_math(&mut e, 1.0, 1.0);
        let setting = e.mem.alloc(4);
        e.register_double(0x0043_d4d0, move |_, _| ret(setting));
        e.register(0x0064_7b70, |_, _| ret(500));
        e.register(ALLOCATE, |e, a| ret(e.mem.alloc(a[0])));
        for addr in [
            0x0082_f240,
            0x0044_fae0,
            0x009f_9c70,
            0x009f_9c60,
            0x009f_ddf0,
            0x009f_9d30,
            CO_INITIALIZE_EX,
            WAIT_FOR_SINGLE_OBJECT,
            0x0083_1e00,
        ] {
            e.register(addr, noop);
        }
        let file = e.mem.alloc(4);
        e.register_double(0x00af_df20, move |_, _| ret(file));
        e.register(0x0089_05f0, |_, _| ret(1));
        e.register(0x0040_4d30, |_, _| ret(1));
        e.register(0x0048_12f0, |_, _| ret(1));
        e.register(FTOL, |_, a| ret(word_f64(a, 0).trunc() as i32 as u32));
        e.map(0x0107_cae8, 0x20);
        e.set_global(0x0107_cae8, 0x1111_1111u32);
        e.set_global(0x0107_caf8, 0x2222_2222u32);
        e.register_double(0x009f_9d30, |e, a| {
            assert_eq!(e.mem.u32(a[1]), 0x1111_1111);
            assert_eq!(e.mem.u32(a[1] + 0x10), 0x2222_2222);
            Ret::default()
        });

        // Stream and reader vtables (the constructors install the game's).
        e.put_vtable(0x0107_cb4c, &[0x7400_0000]);
        e.register(0x7400_0000, noop);
        e.put_vtable(0x0107_cd6c, &[0, 0x7400_0004, 0x7400_0008]);
        e.register(0x7400_0004, noop);
        e.register(0x7400_0008, noop);
        let mut reader_slots = [0u32; 8];
        reader_slots[7] = 0x7400_001c;
        e.put_vtable(0x0107_cdac, &reader_slots);
        e.register(0x7400_001c, |_, _| ret(0x6300_0000));
        com_doubles(&mut e);

        dirty_statics(&mut e);
        e.set_global(CHANNEL_A.id, 5u32);
        e.set_global(CHANNEL_A.flags, 0x10u8);
        e.set_global(PAUSE_BYTE, 0u8);
        e.set_global(CHANNEL_A.kind, 1u32);
        e.set_global(CHANNEL_A.attenuation, 0.0f32);
        e.set_global(CHANNEL_A.fade_start_volume, 0.0f32);
        e.set_global(CHANNEL_A.fade_time, 1000u32);
        e.set_global(CHANNEL_A.fade_end, 2000u32);
        e.set_global(CHANNEL_A.synch_start, 1000u32);
        e.set_global(THREAD_COUNT, 0u32);
        e.set_global(UPDATE_EVENT, 0x66u32);
        let info = e.mem.alloc(8);
        e.mem.set_u32(info, 5);

        assert_eq!(e.call(0x0083_0b30, &args![info]).u32(), 0);

        let create = calls_to(&e, CO_CREATE_INSTANCE);
        assert_eq!(create[0][..4], [0x0107_cb18, 0, 1, 0x0109_3c0c]);
        assert_eq!(calls_to(&e, 0x009f_9d30).len(), 1);
        assert_eq!(
            calls_to(&e, COM_SLOT + 0x30)
                .into_iter()
                .filter(|a| a[0] == BUILDER)
                .collect::<Vec<_>>(),
            vec![vec![BUILDER, 0x6300_000c]]
        );
        assert_eq!(calls_to(&e, WAIT_FOR_SINGLE_OBJECT), vec![vec![0x66, 100]]);
        let volume = calls_to(&e, COM_SLOT + 0x1c)
            .into_iter()
            .find(|a| a[0] == AUDIO_OBJECT)
            .unwrap();
        // Half way through the fade to 1.0: 0.5, then log10(0.5) * 2000.
        let level = ((0.5f64.log10() as f32) as f64 * 2000.0).trunc() as i32;
        assert_eq!(volume, vec![AUDIO_OBJECT, level as u32]);
        // Expected 500 ms, at 1500 ms: logged and moved to 550 ms.
        assert_eq!(calls_to(&e, LOG), vec![vec![0x0107_cc8c, 1000, 500, 1500]]);
        let moved: Vec<_> = calls_to(&e, COM_SLOT + 0x20)
            .into_iter()
            .filter(|a| a[0] == POSITION)
            .collect();
        assert_eq!(moved.len(), 1);
        assert_eq!(word_f64(&moved[0], 1), 550.0 / 1000.0);
        // Running: event 1 freed twice, then the end (no loop flag).
        assert!(calls_to(&e, COM_SLOT + 0x1c).contains(&vec![CONTROL]));
        assert_eq!(calls_to(&e, 0x0083_1e00), vec![vec![AUDIO + 0x14]]);
        assert_eq!(
            calls_to(&e, COM_SLOT + 0x30)
                .into_iter()
                .filter(|a| a[0] == EVENTS)
                .collect::<Vec<_>>(),
            vec![vec![EVENTS, 1, 0x10, 0x20], vec![EVENTS, 1, 0x10, 0x20]]
        );
        let released: Vec<u32> = calls_to(&e, COM_SLOT + 8).iter().map(|a| a[0]).collect();
        assert_eq!(
            released,
            vec![EVENTS, POSITION, CONTROL, AUDIO_OBJECT, BUILDER, GRAPH]
        );
        assert_eq!(calls_to(&e, 0x7400_0008).len(), 1);
        assert_eq!(calls_to(&e, 0x7400_0000)[0][1], 1);
        assert_eq!(e.global::<u32>(CHANNEL_A.id), 0);
        assert_eq!(e.global::<u32>(THREAD_COUNT), 0);
    }

    #[test]
    fn audio_pause_query_reads_the_member_at_0x14() {
        let mut e = setup();
        e.register(0x0083_1e00, |_, _| ret(1));
        assert!(e.call(0x0083_1de0, &args![0x9000u32]).bool());
        assert_eq!(calls_to(&e, 0x0083_1e00), vec![vec![0x9014]]);
    }

    #[test]
    fn reader_constructor_installs_its_vtables() {
        let mut e = setup();
        e.register(0x009f_ddf0, noop);
        e.register(0x009f_9d30, noop);
        let this = e.mem.alloc(0x1e8);
        let made = e.call(0x0083_1e20, &args![this, 0x10u32, 0x20u32, 0x30u32]);
        assert_eq!(made.u32(), this);
        assert_eq!(
            calls_to(&e, 0x009f_ddf0),
            vec![vec![this, 0, 0, 0x10, 0x30]]
        );
        assert_eq!(calls_to(&e, 0x009f_9d30), vec![vec![this + 0x1a0, 0x20]]);
        assert_eq!(e.mem.u32(this), 0x0107_cdac);
        assert_eq!(e.mem.u32(this + 0xc), 0x0107_cd6c);
        assert_eq!(e.mem.u32(this + 0x10), 0x0107_cd54);
    }
}
