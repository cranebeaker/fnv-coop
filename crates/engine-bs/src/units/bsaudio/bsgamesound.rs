//! `BSAudio/bsgamesound.cpp` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! `BSGameSound` is the platform-independent part of a playing sound: its
//! id, flags, attenuations, volume, time window, file name and the hashes
//! the audio manager finds it by. The PC build is compiled without
//! optimisation here (every local goes through the stack), so a few
//! compiler-folded constant branches are left in the code; they are
//! translated as the exe has them.

#[allow(unused_imports)]
use crate::prelude::*;

/// The global `BSAudio*` (`BSAudio::pAudioInstance`, Xbox PDB static).
const AUDIO_INSTANCE: u32 = 0x011f_6d98;
/// `BSAudio::pfnRandomFilenameCallback` (Xbox PDB, +0x28 of `BSAudio`): a
/// cdecl function taking the file name, or null.
const AUDIO_RANDOM_FILENAME_CALLBACK: u32 = 0x28;
/// The `BSAudio` virtual slot `SetFilename` calls with the file name (one
/// argument). In `BSWin32Audio`'s PC vtable (`010a39d8`) it is `00aea640`,
/// which puts `data\sound\` before names starting with `fx` or `song`
/// (`units/bsaudio/run_00aea5c0.rs`); the Xbox PDB's `FixSoundPath` sits
/// one slot later there, so the name is not confirmed.
const AUDIO_PATH_SLOT: u32 = 0x18;
/// `_strcpy_s` (CRT, cdecl: destination, size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `_strlen` (CRT, cdecl).
const STRLEN: u32 = 0x00ec_6130;
/// `BSAudioManager::QInstance` (no arguments): the audio manager.
const AUDIO_MANAGER_INSTANCE: u32 = 0x00ad_9060;
/// The audio manager's next sound id (`BSAudioManager` +0x180 on PC).
const MANAGER_NEXT_SOUND_ID: u32 = 0x180;
/// The `float` 1500.0 (`fMaxDist`'s default).
const DEFAULT_MAX_DISTANCE: u32 = 0x0102_23ec;
/// The `float` 50.0 (`fMinDist`'s default).
const DEFAULT_MIN_DISTANCE: u32 = 0x0101_b268;
/// The `float` `FLT_MAX` (`fDistanceToListener`'s default).
const DEFAULT_DISTANCE_TO_LISTENER: u32 = 0x0101_6970;
/// The `double` 0.0.
const ZERO: u32 = 0x0101_2060;
/// The `double` -1.0.
const MINUS_ONE: u32 = 0x0101_a6b0;
/// The `double` 0.09375: the shortest time window `fn_00ae9b50` tests.
const SHORTEST_WINDOW: u32 = 0x0103_4200;
/// `Prepare`'s function-local static `BSRandom` and the guard word whose
/// bit 0 says it has been constructed.
const PREPARE_RANDOM: u32 = 0x011c_4180;
const PREPARE_RANDOM_GUARD: u32 = 0x011c_4b4c;
/// `BSRandom::BSRandom` (Xbox PDB name of `00aa5140`, `this` only).
const BS_RANDOM_CONSTRUCT: u32 = 0x00aa_5140;
/// `BSRandom::UnsignedInt` (Xbox PDB name of `00aa5230`: `this`, bound).
const BS_RANDOM_UNSIGNED_INT: u32 = 0x00aa_5230;
/// `BSGameSound::Seek` (Xbox PDB virtual slot +0x6C, one argument).
const SEEK_SLOT: u32 = 0x6c;

layout! {
    /// `BSGameSound` (Xbox PDB), 0x198 bytes; the offsets used here are the
    /// ones the PC code uses.
    pub struct BSGameSound: 0x198 {
        /// `iSoundID` (Xbox PDB).
        0x004 iSoundID: u32,
        /// `iSoundTypeFlags` (Xbox PDB).
        0x008 iSoundTypeFlags: u32,
        /// `iModifierFlags` (Xbox PDB).
        0x00c iModifierFlags: u32,
        /// `iStateFlags` (Xbox PDB).
        0x010 iStateFlags: u32,
        /// `iDurationInMS` (Xbox PDB).
        0x014 iDurationInMS: u32,
        /// `iStaticAttenuation` (Xbox PDB).
        0x018 iStaticAttenuation: u16,
        /// `iReverbAttenuation` (Xbox PDB).
        0x01a iReverbAttenuation: u16,
        /// `iSystemAttenuation` (Xbox PDB).
        0x01c iSystemAttenuation: u16,
        /// `iDistanceAttenuation` (Xbox PDB).
        0x01e iDistanceAttenuation: u16,
        /// `iFaderAttenuation` (Xbox PDB).
        0x020 iFaderAttenuation: u16,
        /// `fCurrentVolume` (Xbox PDB), as its bit pattern.
        0x024 fCurrentVolume: u32,
        /// `fBeginTime` (Xbox PDB), as its bit pattern.
        0x028 fBeginTime: u32,
        /// `fEndTime` (Xbox PDB), as its bit pattern.
        0x02c fEndTime: u32,
        /// `iWeatherFlags` (Xbox PDB).
        0x030 iWeatherFlags: u32,
        /// `iSamplesPerSecond` (Xbox PDB).
        0x034 iSamplesPerSecond: u16,
        /// `pFilename` (Xbox PDB): `char[260]`, inline.
        0x036 pFilename: u8,
        /// `fMaxDist` (Xbox PDB), as its bit pattern.
        0x13c fMaxDist: u32,
        /// `fMinDist` (Xbox PDB), as its bit pattern.
        0x140 fMinDist: u32,
        /// `iSoundHash` (Xbox PDB).
        0x144 iSoundHash: u32,
        /// `iDirectoryHash` (Xbox PDB).
        0x148 iDirectoryHash: u32,
        /// `iSoundSize` (Xbox PDB).
        0x14c iSoundSize: u32,
        /// `fDistanceToListener` (Xbox PDB), as its bit pattern.
        0x150 fDistanceToListener: u32,
        /// `iEnvironmentType` (Xbox PDB).
        0x154 iEnvironmentType: u32,
        /// `iFreqVariance` (Xbox PDB).
        0x158 iFreqVariance: u8,
        /// `iModSamplesPerSecond` (Xbox PDB).
        0x15a iModSamplesPerSecond: u16,
        /// `iSynchStartTime` (Xbox PDB).
        0x15c iSynchStartTime: u32,
        /// `iCleanupDelayInMS` (Xbox PDB).
        0x160 iCleanupDelayInMS: u32,
        /// `fnCompletionCallback` (Xbox PDB).
        0x164 fnCompletionCallback: u32,
        /// `fnPlayedCallback` (Xbox PDB): cdecl, (context, duration).
        0x168 fnPlayedCallback: u32,
        /// `pSoundCompletionContext` (Xbox PDB).
        0x16c pSoundCompletionContext: u32,
        /// `pSoundPlayContext` (Xbox PDB).
        0x170 pSoundPlayContext: u32,
        /// `iLoopStart` (Xbox PDB).
        0x174 iLoopStart: u32,
        /// `iLoopEnd` (Xbox PDB).
        0x178 iLoopEnd: u32,
    }
}

/// Offset of the inline file name (`pFilename`, `char[260]`).
const FILENAME: u32 = 0x36;

// Translated from 00ae9460 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::SetFilename` (Xbox PDB): copies `filename` into the sound
/// (unless it already is the sound's own buffer), lets the `BSAudio`
/// instance's slot +0x18 process it, and sets the hashes. A name ending in
/// `\` or `/` is a directory: its hash goes to `iDirectoryHash` (once), and
/// with `pick_random` set and a random-file callback present the callback
/// picks a file in it (the slot runs again on the result). The sound hash
/// is the name's hash, or 0 for a directory without `pick_random`, which
/// also makes the result false. Null `filename`: false, nothing changes.
pub fn bs_game_sound_set_filename(
    e: &mut Engine,
    this: Ptr<BSGameSound>,
    filename: Ptr,
    pick_random: u8,
) -> bool {
    let mut result = true;
    if filename.is_null() {
        return false;
    }
    let name = this.addr().wrapping_add(FILENAME);
    if name != filename.addr() {
        e.call(STRCPY_S, &args![name, 0x104u32, filename]);
    }
    let audio = e.global::<u32>(AUDIO_INSTANCE);
    e.vcall(audio, AUDIO_PATH_SLOT, &args![name]);
    let audio = e.global::<u32>(AUDIO_INSTANCE);
    let callback = e
        .mem
        .u32(audio.wrapping_add(AUDIO_RANDOM_FILENAME_CALLBACK));
    let length = e.call(STRLEN, &args![name]).u32();
    // The last character: `this + length + 0x35` (for an empty name the
    // byte before the buffer, as the exe reads it).
    let last = this.addr().wrapping_add(length).wrapping_add(0x35);
    let is_directory = e.mem.u8(last) == b'\\' || e.mem.u8(last) == b'/';
    if is_directory && e.get(this, BSGameSound::iDirectoryHash) == 0 {
        let hash = bs_game_sound_get_name_hash(e, Ptr::new(name));
        e.set(this, BSGameSound::iDirectoryHash, hash);
    }
    if pick_random != 0 && is_directory && callback != 0 {
        e.call(callback, &args![name]);
        let audio = e.global::<u32>(AUDIO_INSTANCE);
        e.vcall(audio, AUDIO_PATH_SLOT, &args![name]);
    } else if is_directory && pick_random == 0 {
        result = false;
    }
    if is_directory && pick_random == 0 {
        e.set(this, BSGameSound::iSoundHash, 0);
    } else {
        let hash = bs_game_sound_get_name_hash(e, Ptr::new(name));
        e.set(this, BSGameSound::iSoundHash, hash);
    }
    result
}

// Translated from 00ae95e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Resets a `BSGameSound`'s fields to their defaults (called by the
/// constructor `00ae9360`): ids, flags (state flag 0x100 set), attenuations,
/// time window, hashes and callbacks zero, volume 1.0, distances from the
/// exe's constants (1500.0, 50.0, `FLT_MAX` to the listener). The last step
/// is a compiler-folded `flag ? (state | 0x8000) : (state & !0x8000)` whose
/// flag is the constant 0, so state flag 0x8000 ends clear.
pub fn fn_00ae95e0(e: &mut Engine, this: Ptr<BSGameSound>) {
    e.set(this, BSGameSound::iSoundID, 0);
    e.set(this, BSGameSound::iSoundTypeFlags, 0);
    e.set(this, BSGameSound::iModifierFlags, 0);
    e.set(this, BSGameSound::iStateFlags, 0);
    let state = e.get(this, BSGameSound::iStateFlags) | 0x100;
    e.set(this, BSGameSound::iStateFlags, state);
    e.set(this, BSGameSound::iDurationInMS, 0);
    e.set(this, BSGameSound::iStaticAttenuation, 0);
    e.set(this, BSGameSound::iReverbAttenuation, 0);
    e.set(this, BSGameSound::iSystemAttenuation, 0);
    e.set(this, BSGameSound::iDistanceAttenuation, 0);
    e.set(this, BSGameSound::iFaderAttenuation, 0);
    e.set(this, BSGameSound::fCurrentVolume, 1.0f32.to_bits());
    e.set(this, BSGameSound::fBeginTime, 0.0f32.to_bits());
    e.set(this, BSGameSound::fEndTime, 0.0f32.to_bits());
    e.set(this, BSGameSound::iWeatherFlags, 0);
    e.set(this, BSGameSound::iSamplesPerSecond, 0);
    let max = x87_quiet_bits(e.global::<u32>(DEFAULT_MAX_DISTANCE));
    e.set(this, BSGameSound::fMaxDist, max);
    let min = x87_quiet_bits(e.global::<u32>(DEFAULT_MIN_DISTANCE));
    e.set(this, BSGameSound::fMinDist, min);
    e.set(this, BSGameSound::iSoundSize, 0);
    e.set(this, BSGameSound::iEnvironmentType, 0);
    e.set(this, BSGameSound::iCleanupDelayInMS, 0);
    e.set(this, BSGameSound::iSynchStartTime, 0);
    e.set(this, BSGameSound::iSoundHash, 0);
    e.set(this, BSGameSound::iDirectoryHash, 0);
    let distance = x87_quiet_bits(e.global::<u32>(DEFAULT_DISTANCE_TO_LISTENER));
    e.set(this, BSGameSound::fDistanceToListener, distance);
    e.set(this, BSGameSound::iFreqVariance, 0);
    e.set(this, BSGameSound::iModSamplesPerSecond, 0);
    e.set(this, BSGameSound::iLoopStart, 0);
    e.set(this, BSGameSound::iLoopEnd, 0);
    e.set(this, BSGameSound::fnCompletionCallback, 0);
    e.set(this, BSGameSound::fnPlayedCallback, 0);
    e.set(this, BSGameSound::pSoundCompletionContext, 0);
    e.set(this, BSGameSound::pSoundPlayContext, 0);
    // `XOR EAX,EAX; JZ`: the folded flag is always 0.
    let folded_flag = 0u32;
    let state = if folded_flag != 0 {
        e.get(this, BSGameSound::iStateFlags) | 0x8000
    } else {
        e.get(this, BSGameSound::iStateFlags) & 0xffff_7fff
    };
    e.set(this, BSGameSound::iStateFlags, state);
}

// Translated from 00ae97a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::Copy` (Xbox PDB): copies `source`'s hashes, file name
/// (through `SetFilename` without picking a random file), flags, duration,
/// attenuations (not the fader's), volume, time window, weather flags,
/// sample rates, maximum distance, size, loop points and callbacks, and
/// gives the copy a new id from the audio manager's counter. The second
/// stack word (`RET 8`) is not read.
pub fn bs_game_sound_copy(
    e: &mut Engine,
    this: Ptr<BSGameSound>,
    source: Ptr<BSGameSound>,
    _unused_2: u32,
) {
    let v = e.get(source, BSGameSound::iSoundHash);
    e.set(this, BSGameSound::iSoundHash, v);
    let v = e.get(source, BSGameSound::iDirectoryHash);
    e.set(this, BSGameSound::iDirectoryHash, v);
    bs_game_sound_set_filename(e, this, source.byte_add(FILENAME), 0);
    let manager = e.call(AUDIO_MANAGER_INSTANCE, &[]).u32();
    let id = e.mem.u32(manager.wrapping_add(MANAGER_NEXT_SOUND_ID));
    let next = e
        .mem
        .u32(manager.wrapping_add(MANAGER_NEXT_SOUND_ID))
        .wrapping_add(1);
    e.mem
        .set_u32(manager.wrapping_add(MANAGER_NEXT_SOUND_ID), next);
    e.set(this, BSGameSound::iSoundID, id);
    let v = e.get(source, BSGameSound::iStateFlags);
    e.set(this, BSGameSound::iStateFlags, v);
    let v = e.get(source, BSGameSound::iModifierFlags);
    e.set(this, BSGameSound::iModifierFlags, v);
    let v = e.get(source, BSGameSound::iDurationInMS);
    e.set(this, BSGameSound::iDurationInMS, v);
    let v = e.get(source, BSGameSound::iStaticAttenuation);
    e.set(this, BSGameSound::iStaticAttenuation, v);
    let v = e.get(source, BSGameSound::iReverbAttenuation);
    e.set(this, BSGameSound::iReverbAttenuation, v);
    let v = e.get(source, BSGameSound::iSystemAttenuation);
    e.set(this, BSGameSound::iSystemAttenuation, v);
    let v = e.get(source, BSGameSound::iDistanceAttenuation);
    e.set(this, BSGameSound::iDistanceAttenuation, v);
    let v = x87_quiet_bits(e.get(source, BSGameSound::fCurrentVolume));
    e.set(this, BSGameSound::fCurrentVolume, v);
    let v = x87_quiet_bits(e.get(source, BSGameSound::fBeginTime));
    e.set(this, BSGameSound::fBeginTime, v);
    let v = x87_quiet_bits(e.get(source, BSGameSound::fEndTime));
    e.set(this, BSGameSound::fEndTime, v);
    let v = e.get(source, BSGameSound::iWeatherFlags);
    e.set(this, BSGameSound::iWeatherFlags, v);
    let v = e.get(source, BSGameSound::iSamplesPerSecond);
    e.set(this, BSGameSound::iSamplesPerSecond, v);
    let v = e.get(source, BSGameSound::iModSamplesPerSecond);
    e.set(this, BSGameSound::iModSamplesPerSecond, v);
    let v = x87_quiet_bits(e.get(source, BSGameSound::fMaxDist));
    e.set(this, BSGameSound::fMaxDist, v);
    let v = e.get(source, BSGameSound::iSoundSize);
    e.set(this, BSGameSound::iSoundSize, v);
    let v = e.get(source, BSGameSound::iLoopStart);
    e.set(this, BSGameSound::iLoopStart, v);
    let v = e.get(source, BSGameSound::iLoopEnd);
    e.set(this, BSGameSound::iLoopEnd, v);
    let v = e.get(source, BSGameSound::fnCompletionCallback);
    e.set(this, BSGameSound::fnCompletionCallback, v);
    let v = e.get(source, BSGameSound::fnPlayedCallback);
    e.set(this, BSGameSound::fnPlayedCallback, v);
    let v = e.get(source, BSGameSound::pSoundCompletionContext);
    e.set(this, BSGameSound::pSoundCompletionContext, v);
    let v = e.get(source, BSGameSound::pSoundPlayContext);
    e.set(this, BSGameSound::pSoundPlayContext, v);
}

// Translated from 00ae9af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the played callback once: when `fnPlayedCallback` is set it is
/// called (cdecl) with `pSoundPlayContext` and `iDurationInMS`, then both
/// the callback and its context are cleared.
pub fn fn_00ae9af0(e: &mut Engine, this: Ptr<BSGameSound>) {
    if e.get(this, BSGameSound::fnPlayedCallback) != 0 {
        let duration = e.get(this, BSGameSound::iDurationInMS);
        let context = e.get(this, BSGameSound::pSoundPlayContext);
        let callback = e.get(this, BSGameSound::fnPlayedCallback);
        e.call(callback, &args![context, duration]);
        e.set(this, BSGameSound::pSoundPlayContext, 0);
        e.set(this, BSGameSound::fnPlayedCallback, 0);
    }
}

// Translated from 00ae9b50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether the sound may play for weather `mask` at time `time`: false when
/// the sound has weather flags, `mask` is not zero and they share no bit.
/// Otherwise true when the window `|fEndTime - fBeginTime|` is shorter than
/// 0.09375 (or NaN); else, for a window with end after begin, true when
/// `time` is inside it, and for a window wrapping past midnight (end before
/// begin), true when `time` is not between end and begin. Each test is the
/// exe's x87 flag test, NaN included.
pub fn fn_00ae9b50(e: &mut Engine, this: Ptr<BSGameSound>, mask: u32, time: f32) -> bool {
    let weather = e.get(this, BSGameSound::iWeatherFlags);
    if weather & mask == 0 && weather != 0 && mask != 0 {
        return false;
    }
    let begin = x87_m32(e.get(this, BSGameSound::fBeginTime));
    let end = x87_m32(e.get(this, BSGameSound::fEndTime));
    let window = x87_store32(x87_sub(end, begin));
    let window_value = x87_load32(window);
    // `TEST AH,5; JP`: the multiplication by -1 runs only when below 0.
    let absolute = if window_value < e.global::<f64>(ZERO) {
        x87_mul(window_value, e.global::<f64>(MINUS_ONE))
    } else {
        window_value
    };
    let window = x87_load32(x87_store32(absolute));
    // `TEST AH,1; JNZ`: below, or unordered.
    if !(window >= e.global::<f64>(SHORTEST_WINDOW)) {
        return true;
    }
    let time = x87_load32(time.to_bits());
    let begin = x87_m32(e.get(this, BSGameSound::fBeginTime));
    let end = x87_m32(e.get(this, BSGameSound::fEndTime));
    if end > begin {
        if time < begin {
            return false;
        }
        let end = x87_m32(e.get(this, BSGameSound::fEndTime));
        if time > end {
            return false;
        }
    }
    let begin = x87_m32(e.get(this, BSGameSound::fBeginTime));
    let end = x87_m32(e.get(this, BSGameSound::fEndTime));
    if !(end < begin) {
        return true;
    }
    let begin = x87_m32(e.get(this, BSGameSound::fBeginTime));
    if !(time < begin) {
        return true;
    }
    let end = x87_m32(e.get(this, BSGameSound::fEndTime));
    !(time > end)
}

// Translated from 00ae9c50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the state flags for a start: with `flag` set, state flags 0x1 and
/// 0x100 are set and 0x1000 cleared; without, 0x100 and 0x1000 are cleared,
/// 0x1 and 0x10 set and `iDurationInMS` becomes 1. Returns `flag`.
pub fn fn_00ae9c50(e: &mut Engine, this: Ptr<BSGameSound>, flag: u8) -> u8 {
    if flag != 0 {
        let s = e.get(this, BSGameSound::iStateFlags) | 0x1;
        e.set(this, BSGameSound::iStateFlags, s);
        let s = e.get(this, BSGameSound::iStateFlags) | 0x100;
        e.set(this, BSGameSound::iStateFlags, s);
        let s = e.get(this, BSGameSound::iStateFlags) & 0xffff_efff;
        e.set(this, BSGameSound::iStateFlags, s);
    } else {
        let s = e.get(this, BSGameSound::iStateFlags) & 0xffff_feff;
        e.set(this, BSGameSound::iStateFlags, s);
        let s = e.get(this, BSGameSound::iStateFlags) | 0x1;
        e.set(this, BSGameSound::iStateFlags, s);
        let s = e.get(this, BSGameSound::iStateFlags) & 0xffff_efff;
        e.set(this, BSGameSound::iStateFlags, s);
        let s = e.get(this, BSGameSound::iStateFlags) | 0x10;
        e.set(this, BSGameSound::iStateFlags, s);
        e.set(this, BSGameSound::iDurationInMS, 1);
    }
    flag
}

// Translated from 00ae9cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether bit 0x2 of the word at +0x08 (`iSoundTypeFlags` for a
/// `BSGameSound`) is set: 1 or 0 in the whole of `EAX`. (Ghidra matches the
/// body to a library function of the same code.)
pub fn fn_00ae9cf0(e: &mut Engine, this: Ptr<BSGameSound>) -> u32 {
    u32::from(e.get(this, BSGameSound::iSoundTypeFlags) & 0x2 != 0)
}

// Translated from 00ae9d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Whether both bit 0x1 and bit 0x0800_0000 of `iSoundTypeFlags` are set.
pub fn fn_00ae9d10(e: &mut Engine, this: Ptr<BSGameSound>) -> bool {
    let flags = e.get(this, BSGameSound::iSoundTypeFlags);
    flags & 0x1 != 0 && flags & 0x0800_0000 != 0
}

// Translated from 00ae9d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::CalculateHashes` (Xbox PDB): `SetFilename` on the sound's
/// own name buffer with `pick_random`; false when that buffer's address is
/// null (`this` is `-0x36`).
pub fn bs_game_sound_calculate_hashes(
    e: &mut Engine,
    this: Ptr<BSGameSound>,
    pick_random: u8,
) -> bool {
    if this.addr().wrapping_add(FILENAME) == 0 {
        return false;
    }
    bs_game_sound_set_filename(e, this, this.byte_add(FILENAME), pick_random)
}

// Translated from 00ae9d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::GetNameHash` (Xbox PDB, cdecl): `hash = hash * 0x1003F +
/// c` over the signed characters of `name` after the first one; 0 for a
/// null `name`.
pub fn bs_game_sound_get_name_hash(e: &mut Engine, name: Ptr) -> u32 {
    let mut hash = 0u32;
    if name.is_null() {
        return hash;
    }
    let mut p = name.addr();
    loop {
        p = p.wrapping_add(1);
        let c = e.mem.i8(p) as i32 as u32;
        if c == 0 {
            return hash;
        }
        hash = c
            .wrapping_add(hash << 6)
            .wrapping_add(hash << 16)
            .wrapping_sub(hash);
    }
}

// Translated from 00ae9de0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::Prepare` (Xbox PDB): for a sound with modifier flag 0x2
/// seeks (virtual `Seek`, +0x6C) to a random point below `iDurationInMS`,
/// drawn from the function's static `BSRandom` (constructed on first use),
/// then sets state flag 0x800 (a compiler-folded constant-true branch; the
/// clearing arm never runs) and returns true.
// Translation incomplete: 00ae9de0 - SEH exception frame (FS:[0]) around the static BSRandom's construction is not modelled.
pub fn bs_game_sound_prepare(e: &mut Engine, this: Ptr<BSGameSound>) -> bool {
    if e.get(this, BSGameSound::iModifierFlags) & 0x2 != 0 {
        let duration = e.get(this, BSGameSound::iDurationInMS);
        if e.global::<u32>(PREPARE_RANDOM_GUARD) & 1 == 0 {
            let guard = e.global::<u32>(PREPARE_RANDOM_GUARD) | 1;
            e.set_global(PREPARE_RANDOM_GUARD, guard);
            e.call(BS_RANDOM_CONSTRUCT, &args![PREPARE_RANDOM]);
        }
        let offset = e
            .call(BS_RANDOM_UNSIGNED_INT, &args![PREPARE_RANDOM, duration])
            .u32();
        e.vcall(this.addr(), SEEK_SLOT, &args![offset]);
    }
    // `MOV ECX,1; TEST ECX,ECX; JZ`: the folded flag is always 1.
    let folded_flag = 1u32;
    let state = if folded_flag != 0 {
        e.get(this, BSGameSound::iStateFlags) | 0x800
    } else {
        e.get(this, BSGameSound::iStateFlags) & 0xffff_f7ff
    };
    e.set(this, BSGameSound::iStateFlags, state);
    true
}

// Translated from 00ae9ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::IsPlaying` (Xbox PDB): state flag 0x20, as 1 or 0 in `EAX`.
pub fn bs_game_sound_is_playing(e: &mut Engine, this: Ptr<BSGameSound>) -> u32 {
    u32::from(e.get(this, BSGameSound::iStateFlags) & 0x20 != 0)
}

// Translated from 00ae9f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::IsInDecay` (Xbox PDB): state flag 0x80000, as 1 or 0.
pub fn bs_game_sound_is_in_decay(e: &mut Engine, this: Ptr<BSGameSound>) -> u32 {
    u32::from(e.get(this, BSGameSound::iStateFlags) & 0x8_0000 != 0)
}

// Translated from 00ae9f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::IsPaused` (Xbox PDB): state flag 0x40, as 1 or 0.
pub fn bs_game_sound_is_paused(e: &mut Engine, this: Ptr<BSGameSound>) -> u32 {
    u32::from(e.get(this, BSGameSound::iStateFlags) & 0x40 != 0)
}

// Translated from 00ae9f50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::GetVolume` (Xbox PDB): `fCurrentVolume` in `ST0`.
pub fn bs_game_sound_get_volume(e: &mut Engine, this: Ptr<BSGameSound>) -> f64 {
    x87_load32(e.get(this, BSGameSound::fCurrentVolume))
}

// Translated from 00ae9fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::SetEnvironmentType` (Xbox PDB): stores `kind`.
pub fn bs_game_sound_set_environment_type(e: &mut Engine, this: Ptr<BSGameSound>, kind: u32) {
    e.set(this, BSGameSound::iEnvironmentType, kind);
}

/// `BSGameSound`'s destructor body (`00ae93f0`, `thiscall`, no stack words), the heap the
/// deleting destructor frees into (`011f6238`) and its free (`00aa4060`, `thiscall`, one
/// word).
const BS_GAME_SOUND_DESTRUCTOR: u32 = 0x00ae_93f0;
const HEAP: u32 = 0x011f_6238;
const HEAP_FREE: u32 = 0x00aa_4060;
/// `iPriority` (Xbox PDB, +0x188 of `BSGameSound`; beyond the fields the layout above
/// declares).
const PRIORITY: u32 = 0x188;
/// `iStateFlags` bit 0x20000: the sound is inaudible (`SetInaudible`).
const STATE_INAUDIBLE: u32 = 0x2_0000;

// Translated from 00ae9960 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::Update` (Xbox PDB; `thiscall`, one stack word it does not read). When the
/// sound has bit 0x2 of `iSoundTypeFlags` (`00ae9cf0`) or both 0x1 and 0x0800_0000
/// (`00ae9d10`), lacks bit 0x100 there and is audible (`iStateFlags` bit 0x20000 clear), its
/// virtual slot +4 answers true (`AL`), it has one of 0x10, 0x0400_0000, 0x0200_0000 but not
/// 0x0200_0000, and `fDistanceToListener` is above `fMaxDist` (x87 `FCOMPP`, false for
/// NaN): slot +0x38, then slot +0x14 with 1, and it answers. Otherwise, when it is inaudible,
/// slot +0xc answers false (`AL`) and `fMaxDist` is above `fDistanceToListener`: slot +0x14
/// with 0, then slot +0x30 with 1. Always answers true.
pub fn bs_game_sound_update(e: &mut Engine, this: Ptr<BSGameSound>, _unused_1: u32) -> bool {
    let object = this.addr();
    let applies = fn_00ae9cf0(e, this) as u8 != 0 || fn_00ae9d10(e, this);
    if applies
        && e.get(this, BSGameSound::iSoundTypeFlags) & 0x100 == 0
        && e.get(this, BSGameSound::iStateFlags) & STATE_INAUDIBLE == 0
        && e.vcall(object, 0x4, &args![]).bool()
    {
        let kind = e.get(this, BSGameSound::iSoundTypeFlags);
        let streamed = kind & 0x10 != 0 || kind & 0x0400_0000 != 0 || kind & 0x0200_0000 != 0;
        if streamed && e.get(this, BSGameSound::iSoundTypeFlags) & 0x0200_0000 == 0 {
            let distance = f32::from_bits(e.get(this, BSGameSound::fDistanceToListener));
            let maximum = f32::from_bits(e.get(this, BSGameSound::fMaxDist));
            if distance > maximum {
                e.vcall(object, 0x38, &args![]);
                e.vcall(object, 0x14, &args![1u32]);
                return true;
            }
        }
    }
    if e.get(this, BSGameSound::iStateFlags) & STATE_INAUDIBLE != 0
        && !e.vcall(object, 0xc, &args![]).bool()
    {
        let distance = f32::from_bits(e.get(this, BSGameSound::fDistanceToListener));
        let maximum = f32::from_bits(e.get(this, BSGameSound::fMaxDist));
        if maximum > distance {
            e.vcall(object, 0x14, &args![0u32]);
            e.vcall(object, 0x30, &args![1u32]);
        }
    }
    true
}

// Translated from 00ae9ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::_scalar_deleting_destructor_` (Xbox PDB; `thiscall`, one word of flags):
/// the destructor body `00ae93f0`, then with flag bit 0 the block freed into the heap at
/// `011f6238` (`00aa4060`). Answers `this`.
pub fn bs_game_sound_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<BSGameSound>,
    flags: u32,
) -> Ptr<BSGameSound> {
    e.call(BS_GAME_SOUND_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![HEAP, this]);
    }
    this
}

// Translated from 00ae9f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::SetInaudible` (Xbox PDB; `thiscall`, one word, its low byte read): sets bit
/// 0x20000 of `iStateFlags` for a nonzero `inaudible`, clears it otherwise.
pub fn bs_game_sound_set_inaudible(e: &mut Engine, this: Ptr<BSGameSound>, inaudible: u8) {
    let flags = e.get(this, BSGameSound::iStateFlags);
    let flags = if inaudible != 0 {
        flags | STATE_INAUDIBLE
    } else {
        flags & !STATE_INAUDIBLE
    };
    e.set(this, BSGameSound::iStateFlags, flags);
}

// Translated from 00ae9fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::PriorityTest` (Xbox PDB; `thiscall`, one word: another sound): 1 when this
/// sound's `iPriority` (+0x188) is below the other's (unsigned), else 0, in the whole of
/// `EAX`.
pub fn bs_game_sound_priority_test(
    e: &mut Engine,
    this: Ptr<BSGameSound>,
    other: Ptr<BSGameSound>,
) -> u32 {
    let mine = e.mem.u32(this.addr().wrapping_add(PRIORITY));
    let theirs = e.mem.u32(other.addr().wrapping_add(PRIORITY));
    u32::from(mine < theirs)
}

// Translated from 00aea000 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSGameSound::PriorityTest` (Xbox PDB, the overload taking a priority; `thiscall`, one
/// word): 1 when `iPriority` (+0x188) is below `priority` (unsigned), else 0, in the whole of
/// `EAX`.
pub fn bs_game_sound_priority_test_ov2(
    e: &mut Engine,
    this: Ptr<BSGameSound>,
    priority: u32,
) -> u32 {
    u32::from(e.mem.u32(this.addr().wrapping_add(PRIORITY)) < priority)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(
            0x00ae_9960,
            bs_game_sound_update(Ptr<BSGameSound>, u32) -> bool
        ),
        entry!(
            0x00ae_9ec0,
            bs_game_sound_scalar_deleting_destructor(Ptr<BSGameSound>, u32) -> Ptr<BSGameSound>
        ),
        entry!(
            0x00ae_9f70,
            bs_game_sound_set_inaudible(Ptr<BSGameSound>, u8)
        ),
        entry!(
            0x00ae_9fd0,
            bs_game_sound_priority_test(Ptr<BSGameSound>, Ptr<BSGameSound>) -> u32
        ),
        entry!(
            0x00ae_a000,
            bs_game_sound_priority_test_ov2(Ptr<BSGameSound>, u32) -> u32
        ),
        entry!(
            0x00ae_9460,
            bs_game_sound_set_filename(Ptr<BSGameSound>, Ptr, u8) -> bool
        ),
        entry!(0x00ae_95e0, fn_00ae95e0(Ptr<BSGameSound>)),
        entry!(
            0x00ae_97a0,
            bs_game_sound_copy(Ptr<BSGameSound>, Ptr<BSGameSound>, u32)
        ),
        entry!(0x00ae_9af0, fn_00ae9af0(Ptr<BSGameSound>)),
        entry!(0x00ae_9b50, fn_00ae9b50(Ptr<BSGameSound>, u32, f32) -> bool),
        entry!(0x00ae_9c50, fn_00ae9c50(Ptr<BSGameSound>, u8) -> u8),
        entry!(0x00ae_9cf0, fn_00ae9cf0(Ptr<BSGameSound>) -> u32),
        entry!(0x00ae_9d10, fn_00ae9d10(Ptr<BSGameSound>) -> bool),
        entry!(
            0x00ae_9d50,
            bs_game_sound_calculate_hashes(Ptr<BSGameSound>, u8) -> bool
        ),
        entry!(0x00ae_9d80, bs_game_sound_get_name_hash(Ptr) -> u32),
        entry!(0x00ae_9de0, bs_game_sound_prepare(Ptr<BSGameSound>) -> bool),
        entry!(
            0x00ae_9ef0,
            bs_game_sound_is_playing(Ptr<BSGameSound>) -> u32
        ),
        entry!(
            0x00ae_9f10,
            bs_game_sound_is_in_decay(Ptr<BSGameSound>) -> u32
        ),
        entry!(
            0x00ae_9f30,
            bs_game_sound_is_paused(Ptr<BSGameSound>) -> u32
        ),
        entry!(
            0x00ae_9f50,
            bs_game_sound_get_volume(Ptr<BSGameSound>) -> f64
        ),
        entry!(
            0x00ae_9fb0,
            bs_game_sound_set_environment_type(Ptr<BSGameSound>, u32)
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Rc<RefCell<Vec<(u32, Vec<u32>)>>>;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// An engine with a `BSAudio` instance whose slot +0x18 is a recording
    /// double and whose random-file callback (if any) appends `x.wav`.
    fn audio_engine(with_callback: bool) -> Engine {
        let mut e = Engine::new();
        e.map(0x011f_6000, 0x1000);
        let audio = e.mem.alloc(0x38);
        e.put_vtable(0x0090_0000, &[0, 0, 0, 0, 0, 0, 0x0090_1000, 0]);
        e.mem.set_u32(audio, 0x0090_0000);
        e.register(0x0090_1000, |_, _| Ret::default());
        if with_callback {
            e.mem.set_u32(audio + 0x28, 0x0090_2000);
            e.register(0x0090_2000, |e, a| {
                let end = a[0] + e.mem.cstr(a[0]).len() as u32;
                e.mem.set_cstr(end, b"x.wav");
                Ret::default()
            });
        }
        e.set_global(AUDIO_INSTANCE, audio);
        e
    }

    fn hash_of(e: &mut Engine, s: &[u8]) -> u32 {
        let p = e.mem.alloc(s.len() as u32 + 1);
        e.mem.set_cstr(p, s);
        bs_game_sound_get_name_hash(e, Ptr::new(p))
    }

    #[test]
    fn set_filename_hashes_a_file_and_runs_the_path_slot() {
        let mut e = audio_engine(false);
        let sound = e.new_object::<BSGameSound>();
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"a\\b.wav");
        e.call_log = Some(vec![]);
        let ok = e.call(0x00ae_9460, &args![sound, name, 0u32]).bool();
        assert!(ok);
        assert_eq!(e.mem.cstr(sound.addr() + 0x36), b"a\\b.wav");
        let audio = e.global::<u32>(AUDIO_INSTANCE);
        assert_eq!(
            calls_to(&e, 0x0090_1000),
            vec![vec![audio, sound.addr() + 0x36]]
        );
        let expected = hash_of(&mut e, b"a\\b.wav");
        assert_eq!(e.get(sound, BSGameSound::iSoundHash), expected);
        assert_eq!(e.get(sound, BSGameSound::iDirectoryHash), 0);
    }

    #[test]
    fn set_filename_on_a_directory() {
        // Without pick_random: false, sound hash 0, directory hash set.
        let mut e = audio_engine(true);
        let sound = e.new_object::<BSGameSound>();
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"dir/");
        assert!(!bs_game_sound_set_filename(
            &mut e,
            sound,
            Ptr::new(name),
            0
        ));
        assert_eq!(e.get(sound, BSGameSound::iSoundHash), 0);
        let dir = hash_of(&mut e, b"dir/");
        assert_eq!(e.get(sound, BSGameSound::iDirectoryHash), dir);
        // With pick_random and a callback: the callback picks a file.
        let sound = e.new_object::<BSGameSound>();
        e.set(sound, BSGameSound::iDirectoryHash, 7);
        e.call_log = Some(vec![]);
        assert!(bs_game_sound_set_filename(&mut e, sound, Ptr::new(name), 1));
        assert_eq!(e.mem.cstr(sound.addr() + 0x36), b"dir/x.wav");
        assert_eq!(calls_to(&e, 0x0090_1000).len(), 2);
        assert_eq!(e.get(sound, BSGameSound::iDirectoryHash), 7);
        let file = hash_of(&mut e, b"dir/x.wav");
        assert_eq!(e.get(sound, BSGameSound::iSoundHash), file);
        // Null name: false, nothing called.
        e.call_log = Some(vec![]);
        assert!(!bs_game_sound_set_filename(&mut e, sound, Ptr::NULL, 1));
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn reset_sets_the_defaults() {
        let mut e = Engine::new();
        for page in [0x0101_6000, 0x0101_b000, 0x0102_2000] {
            e.map(page, 0x1000);
        }
        e.set_global(DEFAULT_MAX_DISTANCE, 1500.0f32);
        e.set_global(DEFAULT_MIN_DISTANCE, 50.0f32);
        e.set_global(DEFAULT_DISTANCE_TO_LISTENER, f32::MAX);
        let sound = e.new_object::<BSGameSound>();
        e.mem.write(sound.addr(), &[0xffu8; 0x198]);
        e.call(0x00ae_95e0, &args![sound]);
        assert_eq!(e.get(sound, BSGameSound::iStateFlags), 0x100);
        assert_eq!(e.get(sound, BSGameSound::fCurrentVolume), 1.0f32.to_bits());
        assert_eq!(e.get(sound, BSGameSound::fMaxDist), 1500.0f32.to_bits());
        assert_eq!(e.get(sound, BSGameSound::fMinDist), 50.0f32.to_bits());
        assert_eq!(
            e.get(sound, BSGameSound::fDistanceToListener),
            f32::MAX.to_bits()
        );
        assert_eq!(e.get(sound, BSGameSound::iFaderAttenuation), 0);
        assert_eq!(e.get(sound, BSGameSound::pSoundPlayContext), 0);
        // Untouched: the vtable word and the name.
        assert_eq!(e.mem.u32(sound.addr()), 0xffff_ffff);
        assert_eq!(e.mem.u8(sound.addr() + 0x36), 0xff);
    }

    #[test]
    fn copy_takes_the_fields_and_a_new_id() {
        let mut e = audio_engine(false);
        let manager = e.mem.alloc(0x200);
        e.mem.set_u32(manager + 0x180, 41);
        e.register_double(AUDIO_MANAGER_INSTANCE, move |_, _| Ret {
            eax: manager,
            ..Ret::default()
        });
        let source = e.new_object::<BSGameSound>();
        e.mem.set_cstr(source.addr() + 0x36, b"s.wav");
        e.set(source, BSGameSound::iSoundHash, 5);
        e.set(source, BSGameSound::iDirectoryHash, 6);
        e.set(source, BSGameSound::iStateFlags, 0x20);
        e.set(source, BSGameSound::iFaderAttenuation, 9);
        e.set(source, BSGameSound::fEndTime, 0x7f80_0001);
        e.set(source, BSGameSound::pSoundPlayContext, 0x1234);
        let sound = e.new_object::<BSGameSound>();
        e.call(0x00ae_97a0, &args![sound, source, 0u32]);
        assert_eq!(e.get(sound, BSGameSound::iSoundID), 41);
        assert_eq!(e.mem.u32(manager + 0x180), 42);
        assert_eq!(e.mem.cstr(sound.addr() + 0x36), b"s.wav");
        // SetFilename recomputes the sound hash; the directory hash stays.
        let expected = hash_of(&mut e, b"s.wav");
        assert_eq!(e.get(sound, BSGameSound::iSoundHash), expected);
        assert_eq!(e.get(sound, BSGameSound::iDirectoryHash), 6);
        assert_eq!(e.get(sound, BSGameSound::iStateFlags), 0x20);
        assert_eq!(e.get(sound, BSGameSound::iFaderAttenuation), 0);
        assert_eq!(e.get(sound, BSGameSound::fEndTime), 0x7fc0_0001);
        assert_eq!(e.get(sound, BSGameSound::pSoundPlayContext), 0x1234);
    }

    #[test]
    fn played_callback_runs_once() {
        let mut e = Engine::new();
        let log: Log = Rc::new(RefCell::new(vec![]));
        let l = log.clone();
        e.register_double(0x0090_3000, move |_, a| {
            l.borrow_mut().push((0x0090_3000, a.to_vec()));
            Ret::default()
        });
        let sound = e.new_object::<BSGameSound>();
        e.set(sound, BSGameSound::fnPlayedCallback, 0x0090_3000);
        e.set(sound, BSGameSound::pSoundPlayContext, 0x77);
        e.set(sound, BSGameSound::iDurationInMS, 500);
        e.call(0x00ae_9af0, &args![sound]);
        e.call(0x00ae_9af0, &args![sound]);
        assert_eq!(*log.borrow(), vec![(0x0090_3000, vec![0x77, 500])]);
        assert_eq!(e.get(sound, BSGameSound::fnPlayedCallback), 0);
        assert_eq!(e.get(sound, BSGameSound::pSoundPlayContext), 0);
    }

    fn window_engine() -> Engine {
        let mut e = Engine::new();
        for page in [0x0101_2000, 0x0101_a000, 0x0103_4000] {
            e.map(page, 0x1000);
        }
        e.set_global(ZERO, 0.0f64);
        e.set_global(MINUS_ONE, -1.0f64);
        e.set_global(SHORTEST_WINDOW, 0.09375f64);
        e
    }

    #[test]
    fn time_window_test() {
        let mut e = window_engine();
        let sound = e.new_object::<BSGameSound>();
        let check =
            |e: &mut Engine, mask: u32, t: f32| e.call(0x00ae_9b50, &args![sound, mask, t]).bool();
        // Weather flags that share no bit with the mask: false.
        e.set(sound, BSGameSound::iWeatherFlags, 0x2);
        assert!(!check(&mut e, 0x1, 5.0));
        // Zero window: always true.
        assert!(check(&mut e, 0x2, 5.0));
        // A day window 6..18.
        e.set(sound, BSGameSound::fBeginTime, 6.0f32.to_bits());
        e.set(sound, BSGameSound::fEndTime, 18.0f32.to_bits());
        assert!(check(&mut e, 0, 12.0));
        assert!(!check(&mut e, 0, 5.0));
        assert!(!check(&mut e, 0, 19.0));
        assert!(check(&mut e, 0, 18.0));
        // A night window 20..4 (end before begin; negative difference).
        e.set(sound, BSGameSound::fBeginTime, 20.0f32.to_bits());
        e.set(sound, BSGameSound::fEndTime, 4.0f32.to_bits());
        assert!(check(&mut e, 0, 22.0));
        assert!(check(&mut e, 0, 2.0));
        assert!(!check(&mut e, 0, 12.0));
        // NaN time: inside by the flag tests.
        assert!(check(&mut e, 0, f32::NAN));
    }

    #[test]
    fn start_flags() {
        let mut e = Engine::new();
        let sound = e.new_object::<BSGameSound>();
        e.set(sound, BSGameSound::iStateFlags, 0x1000);
        assert_eq!(e.call(0x00ae_9c50, &args![sound, 1u32]).u8(), 1);
        assert_eq!(e.get(sound, BSGameSound::iStateFlags), 0x101);
        e.set(sound, BSGameSound::iStateFlags, 0x1100);
        assert_eq!(e.call(0x00ae_9c50, &args![sound, 0u32]).u8(), 0);
        assert_eq!(e.get(sound, BSGameSound::iStateFlags), 0x11);
        assert_eq!(e.get(sound, BSGameSound::iDurationInMS), 1);
    }

    #[test]
    fn type_flag_tests() {
        let mut e = Engine::new();
        let sound = e.new_object::<BSGameSound>();
        e.set(sound, BSGameSound::iSoundTypeFlags, 0x2);
        assert_eq!(e.call(0x00ae_9cf0, &args![sound]).u32(), 1);
        assert!(!e.call(0x00ae_9d10, &args![sound]).bool());
        e.set(sound, BSGameSound::iSoundTypeFlags, 0x0800_0001);
        assert_eq!(e.call(0x00ae_9cf0, &args![sound]).u32(), 0);
        assert!(e.call(0x00ae_9d10, &args![sound]).bool());
        e.set(sound, BSGameSound::iSoundTypeFlags, 0x0800_0000);
        assert!(!e.call(0x00ae_9d10, &args![sound]).bool());
    }

    #[test]
    fn calculate_hashes_uses_the_own_name() {
        let mut e = audio_engine(false);
        let sound = e.new_object::<BSGameSound>();
        e.mem.set_cstr(sound.addr() + 0x36, b"ab");
        assert!(e.call(0x00ae_9d50, &args![sound, 0u32]).bool());
        let expected = hash_of(&mut e, b"ab");
        assert_eq!(e.get(sound, BSGameSound::iSoundHash), expected);
        // `this + 0x36 == 0`: false without a call.
        e.call_log = Some(vec![]);
        let wrapped = Ptr::new(0u32.wrapping_sub(0x36));
        assert!(!bs_game_sound_calculate_hashes(&mut e, wrapped, 0));
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn name_hash_skips_the_first_character() {
        let mut e = Engine::new();
        assert_eq!(e.call(0x00ae_9d80, &args![0u32]).u32(), 0);
        let p = e.mem.alloc(8);
        e.mem.set_cstr(p, b"xab");
        let expected = (b'a' as u32)
            .wrapping_mul(0x1003f)
            .wrapping_add(b'b' as u32);
        assert_eq!(e.call(0x00ae_9d80, &args![p]).u32(), expected);
        e.mem.set_cstr(p, b"q\xff");
        assert_eq!(e.call(0x00ae_9d80, &args![p]).u32(), 0xffff_ffff);
        e.mem.set_cstr(p, b"q");
        assert_eq!(e.call(0x00ae_9d80, &args![p]).u32(), 0);
    }

    #[test]
    fn prepare_seeks_randomly_once_constructed() {
        let mut e = Engine::new();
        e.map(0x011c_4000, 0x1000);
        e.register(BS_RANDOM_CONSTRUCT, |_, _| Ret::default());
        e.register(BS_RANDOM_UNSIGNED_INT, |_, a| Ret {
            eax: a[1] / 2,
            ..Ret::default()
        });
        e.register(0x0090_4000, |_, _| Ret::default());
        let mut slots = vec![0u32; 28];
        slots[0x6c / 4] = 0x0090_4000;
        e.put_vtable(0x0090_5000, &slots);
        let sound = e.new_object::<BSGameSound>();
        e.mem.set_u32(sound.addr(), 0x0090_5000);
        e.set(sound, BSGameSound::iModifierFlags, 0x2);
        e.set(sound, BSGameSound::iDurationInMS, 1000);
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ae_9de0, &args![sound]).bool());
        assert!(bs_game_sound_prepare(&mut e, sound));
        assert_eq!(
            calls_to(&e, BS_RANDOM_CONSTRUCT),
            vec![vec![PREPARE_RANDOM]]
        );
        assert_eq!(
            calls_to(&e, 0x0090_4000),
            vec![vec![sound.addr(), 500], vec![sound.addr(), 500]]
        );
        assert_eq!(e.global::<u32>(PREPARE_RANDOM_GUARD), 1);
        assert_eq!(e.get(sound, BSGameSound::iStateFlags), 0x800);
        // Without modifier flag 0x2: only the state flag.
        let other = e.new_object::<BSGameSound>();
        e.call_log = Some(vec![]);
        assert!(bs_game_sound_prepare(&mut e, other));
        assert!(calls_to(&e, BS_RANDOM_UNSIGNED_INT).is_empty());
        assert_eq!(e.get(other, BSGameSound::iStateFlags), 0x800);
    }

    #[test]
    fn state_getters_and_setters() {
        let mut e = Engine::new();
        let sound = e.new_object::<BSGameSound>();
        e.set(sound, BSGameSound::iStateFlags, 0x20);
        assert_eq!(e.call(0x00ae_9ef0, &args![sound]).u32(), 1);
        assert_eq!(e.call(0x00ae_9f10, &args![sound]).u32(), 0);
        assert_eq!(e.call(0x00ae_9f30, &args![sound]).u32(), 0);
        e.set(sound, BSGameSound::iStateFlags, 0x8_0040);
        assert_eq!(e.call(0x00ae_9ef0, &args![sound]).u32(), 0);
        assert_eq!(e.call(0x00ae_9f10, &args![sound]).u32(), 1);
        assert_eq!(e.call(0x00ae_9f30, &args![sound]).u32(), 1);
        e.set(sound, BSGameSound::fCurrentVolume, 0.25f32.to_bits());
        assert_eq!(e.call(0x00ae_9f50, &args![sound]).f64(), 0.25);
        e.call(0x00ae_9fb0, &args![sound, 3u32]);
        assert_eq!(e.get(sound, BSGameSound::iEnvironmentType), 3);
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;

    fn calls_to(e: &Engine, callee: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == callee)
            .map(|(_, words)| words.clone())
            .collect()
    }

    /// A sound whose virtual table at 0x7000 has slot `i` at `0x8000 + 4 * i`; slots +4 and
    /// +0xc answer `alive` and `paused` (false as 0x100, so only `AL` counts), the others
    /// nothing.
    fn sound_engine(alive: bool, paused: bool) -> (Engine, Ptr<BSGameSound>) {
        let mut e = Engine::new();
        let slots: Vec<u32> = (0..0x20).map(|i| 0x8000 + 4 * i).collect();
        e.put_vtable(0x7000, &slots);
        for slot in [0x14u32, 0x30, 0x38] {
            e.register(0x8000 + slot, |_, _| Ret::default());
        }
        let yes: AbiFn = |_, _| 1u32.into_ret();
        let no: AbiFn = |_, _| 0x100u32.into_ret();
        e.register(0x8004, if alive { yes } else { no });
        e.register(0x800c, if paused { yes } else { no });
        let sound = e.new_object::<BSGameSound>();
        e.mem.set_u32(sound.addr(), 0x7000);
        (e, sound)
    }

    fn slot_calls(e: &Engine) -> Vec<(u32, Vec<u32>)> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| (0x8000..0x8080).contains(address))
            .map(|(address, words)| (address - 0x8000, words[1..].to_vec()))
            .collect()
    }

    #[test]
    fn update_stops_a_streamed_sound_beyond_its_range_and_wakes_an_inaudible_one() {
        let (mut e, sound) = sound_engine(true, false);
        e.set(sound, BSGameSound::iSoundTypeFlags, 0x2 | 0x10);
        e.set(sound, BSGameSound::fDistanceToListener, 2000.0f32.to_bits());
        e.set(sound, BSGameSound::fMaxDist, 1500.0f32.to_bits());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ae_9960, &args![sound, 0u32]).bool());
        assert_eq!(
            slot_calls(&e),
            vec![(0x4, vec![]), (0x38, vec![]), (0x14, vec![1])]
        );
        // Within range (or equal, or NaN): nothing more.
        for distance in [1500.0f32, 100.0, f32::NAN] {
            e.set(sound, BSGameSound::fDistanceToListener, distance.to_bits());
            e.call_log = Some(vec![]);
            e.call(0x00ae_9960, &args![sound, 0u32]);
            assert_eq!(slot_calls(&e), vec![(0x4, vec![])]);
        }
        // Bit 0x0200_0000 (or no streaming bit) keeps it playing.
        e.set(sound, BSGameSound::fDistanceToListener, 2000.0f32.to_bits());
        for kind in [0x2 | 0x0200_0000, 0x2] {
            e.set(sound, BSGameSound::iSoundTypeFlags, kind);
            e.call_log = Some(vec![]);
            e.call(0x00ae_9960, &args![sound, 0u32]);
            assert_eq!(slot_calls(&e), vec![(0x4, vec![])]);
        }
        // 0x1 with 0x0800_0000 qualifies too; 0x100 does not.
        e.set(
            sound,
            BSGameSound::iSoundTypeFlags,
            0x1 | 0x0800_0000 | 0x0400_0000,
        );
        e.call_log = Some(vec![]);
        e.call(0x00ae_9960, &args![sound, 0u32]);
        assert_eq!(slot_calls(&e).len(), 3);
        e.set(sound, BSGameSound::iSoundTypeFlags, 0x2 | 0x10 | 0x100);
        e.call_log = Some(vec![]);
        e.call(0x00ae_9960, &args![sound, 0u32]);
        assert!(slot_calls(&e).is_empty());
        // Inaudible and back in range: slot +0x14 with 0, slot +0x30 with 1.
        e.set(sound, BSGameSound::iSoundTypeFlags, 0x2 | 0x10);
        e.set(sound, BSGameSound::iStateFlags, STATE_INAUDIBLE);
        e.set(sound, BSGameSound::fDistanceToListener, 100.0f32.to_bits());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ae_9960, &args![sound, 0u32]).bool());
        assert_eq!(
            slot_calls(&e),
            vec![(0xc, vec![]), (0x14, vec![0]), (0x30, vec![1])]
        );
        // Still out of range: only the test.
        e.set(sound, BSGameSound::fDistanceToListener, 1500.0f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x00ae_9960, &args![sound, 0u32]);
        assert_eq!(slot_calls(&e), vec![(0xc, vec![])]);
    }

    #[test]
    fn update_leaves_a_sound_its_slots_refuse() {
        // Slot +4 answers false (`AL` 0): no stop.
        let (mut e, sound) = sound_engine(false, true);
        e.set(sound, BSGameSound::iSoundTypeFlags, 0x2 | 0x10);
        e.set(sound, BSGameSound::fDistanceToListener, 2000.0f32.to_bits());
        e.set(sound, BSGameSound::fMaxDist, 1500.0f32.to_bits());
        e.call_log = Some(vec![]);
        assert!(e.call(0x00ae_9960, &args![sound, 0u32]).bool());
        assert_eq!(slot_calls(&e), vec![(0x4, vec![])]);
        // Inaudible and slot +0xc answering true: nothing.
        e.set(sound, BSGameSound::iStateFlags, STATE_INAUDIBLE);
        e.set(sound, BSGameSound::fDistanceToListener, 100.0f32.to_bits());
        e.call_log = Some(vec![]);
        e.call(0x00ae_9960, &args![sound, 0u32]);
        assert_eq!(slot_calls(&e), vec![(0xc, vec![])]);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_with_flag_bit_0() {
        let mut e = Engine::new();
        e.register(BS_GAME_SOUND_DESTRUCTOR, |_, _| Ret::default());
        e.register(HEAP_FREE, |_, _| Ret::default());
        let sound = 0x5000u32;
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_9ec0, &args![sound, 0xfeu32]).u32(), sound);
        assert!(calls_to(&e, HEAP_FREE).is_empty());
        assert_eq!(e.call(0x00ae_9ec0, &args![sound, 1u32]).u32(), sound);
        assert_eq!(calls_to(&e, HEAP_FREE), vec![vec![HEAP, sound]]);
        assert_eq!(calls_to(&e, BS_GAME_SOUND_DESTRUCTOR), vec![vec![sound]; 2]);
    }

    #[test]
    fn set_inaudible_sets_or_clears_bit_0x20000() {
        let mut e = Engine::new();
        let sound = e.new_object::<BSGameSound>();
        e.set(sound, BSGameSound::iStateFlags, 0x101);
        e.call(0x00ae_9f70, &args![sound, 0x101u32]);
        assert_eq!(e.get(sound, BSGameSound::iStateFlags), 0x2_0101);
        // Only the low byte counts.
        e.call(0x00ae_9f70, &args![sound, 0x100u32]);
        assert_eq!(e.get(sound, BSGameSound::iStateFlags), 0x101);
    }

    #[test]
    fn priority_tests_compare_unsigned() {
        let mut e = Engine::new();
        let a = e.new_object::<BSGameSound>();
        let b = e.new_object::<BSGameSound>();
        e.mem.set_u32(a.addr() + PRIORITY, 3);
        e.mem.set_u32(b.addr() + PRIORITY, 0xffff_fff0);
        assert_eq!(e.call(0x00ae_9fd0, &args![a, b]).u32(), 1);
        assert_eq!(e.call(0x00ae_9fd0, &args![b, a]).u32(), 0);
        assert_eq!(e.call(0x00ae_9fd0, &args![a, a]).u32(), 0);
        assert_eq!(e.call(0x00ae_a000, &args![a, 4u32]).u32(), 1);
        assert_eq!(e.call(0x00ae_a000, &args![a, 3u32]).u32(), 0);
        assert_eq!(e.call(0x00ae_a000, &args![b, 2u32]).u32(), 0);
    }
}
