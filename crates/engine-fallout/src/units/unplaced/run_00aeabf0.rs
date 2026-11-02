//! `(unplaced)/run_00aeabf0` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The vtable `010a3bf4` has the RTTI name `.?AVBSWin32GameSound@@` (read through the word before
//! it); the PC class has no Xbox PDB entry. Its first 0x198 bytes are `BSGameSound` (Xbox PDB,
//! the same offsets on PC); the PC-only fields used here are described where they are read.
//! The slot names of the virtual functions are `BSGameSound`'s (Xbox PDB vtable offsets, which
//! match the PC): the functions are named by address and their slot is given.

#[allow(unused_imports)]
use crate::prelude::*;
use crate::x87::fistp_chop;

/// `BSWin32GameSound`'s vtable (RTTI `.?AVBSWin32GameSound@@`).
const WIN32_GAME_SOUND_VTABLE: u32 = 0x010a_3bf4;
/// The memory manager (`MemoryManager`) the sound allocates from (`00aa3e40`) and frees to
/// (`00aa4060`).
const MEMORY_MANAGER: u32 = 0x011f_6238;
const ALLOCATE: u32 = 0x00aa_3e40;
const FREE: u32 = 0x00aa_4060;
/// `memset` / `memcpy` / `strstr` / `strcpy_s` / `strlen` (CRT, `cdecl`).
const MEMSET: u32 = 0x00ec_61c0;
const MEMCPY: u32 = 0x00ec_44d0;
const STRSTR: u32 = 0x00ec_7750;
const STRCPY_S: u32 = 0x00ec_65a6;
const STRLEN: u32 = 0x00ec_6130;
/// The game's message printer (`005b5e40`, `cdecl`, variable arguments).
const PRINT: u32 = 0x005b_5e40;
/// The audio manager (`BSAudioManager *`) whose `+0x38` is the `IDirectSound8`.
const AUDIO_MANAGER_011F6D98: u32 = 0x011f_6d98;
/// `BSGameSound` helpers (`bsgamesound.cpp`): `00ae9c50` stores the open result (`RET 4`,
/// answers its byte), `00ae9cf0`/`00ae9d10` test the 3D flags, `00ae9d50` is
/// `BSGameSound::CalculateHashes` (`RET 4`), `00ae9af0` runs after a play.
const SET_OPENED: u32 = 0x00ae_9c50;
const IS_3D: u32 = 0x00ae_9cf0;
const IS_3D_OTHER: u32 = 0x00ae_9d10;
const CALCULATE_HASHES: u32 = 0x00ae_9d50;
const AFTER_PLAY: u32 = 0x00ae_9af0;
/// `E_FAIL`.
const E_FAIL: u32 = 0x8000_4005;
/// `0.0`, `1.0` and `100.0` (`double`).
const ZERO_01012060: u32 = 0x0101_2060;
const ONE_01012070: u32 = 0x0101_2070;
const HUNDRED_01017A40: u32 = 0x0101_7a40;

// `BSGameSound` fields (Xbox PDB names, PC offsets).
const SOUND_TYPE_FLAGS: u32 = 0x08;
const STATE_FLAGS: u32 = 0x10;
const DURATION: u32 = 0x14;
const STATIC_ATTENUATION: u32 = 0x18;
const REVERB_ATTENUATION: u32 = 0x1a;
const SYSTEM_ATTENUATION: u32 = 0x1c;
const DISTANCE_ATTENUATION: u32 = 0x1e;
const FADER_ATTENUATION: u32 = 0x20;
const CURRENT_VOLUME: u32 = 0x24;
const SAMPLES_PER_SECOND: u32 = 0x34;
const FILENAME: u32 = 0x36;
const MAX_DISTANCE: u32 = 0x13c;
const MIN_DISTANCE: u32 = 0x140;
const SOUND_SIZE: u32 = 0x14c;
const MOD_SAMPLES_PER_SECOND: u32 = 0x15a;
const LOOP_START: u32 = 0x174;
const LOOP_END: u32 = 0x178;

// PC-only fields of `BSWin32GameSound` (offsets from the PC code; meanings from the DirectSound
// calls made on them).
/// The `IDirectSound8` the sound was made with.
const DIRECT_SOUND: u32 = 0x198;
/// The sound's `IDirectSoundBuffer8`.
const BUFFER: u32 = 0x19c;
/// A second `IDirectSoundBuffer` played and stopped along with the first.
const SECOND_BUFFER: u32 = 0x1a0;
/// The `DSBUFFERDESC` (0x24 bytes): `dwSize` `+0x1a4`, `dwFlags` `+0x1a8`, `dwBufferBytes`
/// `+0x1ac`, `lpwfxFormat` `+0x1b4` (an 0x12-byte `WAVEFORMATEX`), `guid3DAlgorithm` `+0x1b8`.
const BUFFER_DESC: u32 = 0x1a4;
const BUFFER_FLAGS: u32 = 0x1a8;
const BUFFER_BYTES: u32 = 0x1ac;
const WAVE_FORMAT: u32 = 0x1b4;
const ALGORITHM_GUID: u32 = 0x1b8;
/// The sound's sample data (from the memory manager).
const SAMPLE_DATA: u32 = 0x1c8;
/// The `IDirectSoundBuffer` `CreateSoundBuffer` makes (`+0x1cc`); the `IDirectSoundBuffer8`
/// (`+0x19c`) and `IDirectSound3DBuffer` (`+0x1d4`) are queried from it.
const CREATED_BUFFER: u32 = 0x1cc;
const BUFFER_1D0: u32 = 0x1d0;
/// The `IDirectSound3DBuffer`.
const BUFFER_3D: u32 = 0x1d4;
const BUFFER_1D8: u32 = 0x1d8;
/// The sample data's size in bytes.
const DATA_BYTES: u32 = 0x1dc;
const WORD_204: u32 = 0x204;
const WORD_208: u32 = 0x208;
/// Five attenuation-curve words (`u16`, `+0x20c` to `+0x214`).
const ATTENUATION_CURVE: u32 = 0x20c;
/// The emitter position (`NiPoint3`).
const POSITION: u32 = 0x218;
const POSITION_SET: u32 = 0x224;
/// The streaming state (`+0x228`, 0x24 bytes and three blocks) and the Ogg Vorbis reader
/// (`+0x22c`, 0x2e0 bytes, `00af1d20`).
const STREAM: u32 = 0x228;
const OGG_READER: u32 = 0x22c;

fn field(e: &Engine, this: Ptr, offset: u32) -> u32 {
    e.mem.u32(this.addr().wrapping_add(offset))
}

fn set_field(e: &mut Engine, this: Ptr, offset: u32, value: u32) {
    e.mem.set_u32(this.addr().wrapping_add(offset), value);
}

fn flags8(e: &Engine, this: Ptr) -> u32 {
    field(e, this, SOUND_TYPE_FLAGS)
}

fn update_flags8(e: &mut Engine, this: Ptr, f: impl Fn(u32) -> u32) {
    let value = f(flags8(e, this));
    set_field(e, this, SOUND_TYPE_FLAGS, value);
}

fn update_state(e: &mut Engine, this: Ptr, f: impl Fn(u32) -> u32) {
    let value = f(field(e, this, STATE_FLAGS));
    set_field(e, this, STATE_FLAGS, value);
}

/// The streaming test the code repeats: `iSoundTypeFlags` bit 0x4000000 or 0x2000000.
fn is_streaming(e: &Engine, this: Ptr) -> bool {
    flags8(e, this) & 0x0400_0000 != 0 || flags8(e, this) & 0x0200_0000 != 0
}

/// The x87 condition bits `FCOM` leaves in `AH` (after `FNSTSW AX`) for `st0` compared with
/// `st1`: C0 (0x01) less, C3 (0x40) equal, C3|C2|C0 (0x45) unordered, none greater.
fn fpu_compare(st0: f64, st1: f64) -> u8 {
    if st0.is_nan() || st1.is_nan() {
        0x45
    } else if st0 < st1 {
        0x01
    } else if st0 == st1 {
        0x40
    } else {
        0
    }
}

/// The parity flag of `TEST AH,mask`.
fn parity_even(value: u8) -> bool {
    value.count_ones() % 2 == 0
}

fn double_at(e: &Engine, address: u32) -> f64 {
    f64::from_bits(e.mem.u64(address))
}

/// A COM call through the `IDirectSoundBuffer`-style object at `this + offset`.
fn com(e: &mut Engine, this: Ptr, offset: u32, slot: u32, words: &[u32]) -> u32 {
    let object = field(e, this, offset);
    e.vcall(object, slot, words).u32()
}

fn set_opened(e: &mut Engine, this: Ptr, opened: u32) -> u32 {
    e.call(SET_OPENED, &args![this, opened]).u32()
}

// Translated from 00aeac20 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aeac20 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Constructor of `BSWin32GameSound` (`RET 4`, `name`): runs the `BSGameSound` constructor
/// (`00ae9360`), stores the vtable `010a3bf4`, copies `name` into `pFilename` (`+0x36`,
/// `strcpy_s` with its length + 1; 0x104 zero bytes without a name) and clears the PC fields:
/// the buffers, the `DSBUFFERDESC` (its format block taken from the memory manager, 0x12
/// bytes), the position (the point at `011f426c`), and the attenuation curve 0, 600, 1400, 2600,
/// 5000. Returns `this`.
pub fn fn_00aeac20(e: &mut Engine, this: Ptr, name: u32) -> Ptr {
    e.call(0x00ae_9360, &args![this]);
    set_field(e, this, 0, WIN32_GAME_SOUND_VTABLE);
    let filename = this.addr().wrapping_add(FILENAME);
    if name == 0 {
        e.call(MEMSET, &args![filename, 0u32, 0x104u32]);
    } else {
        let length = e.call(STRLEN, &args![name]).u32().wrapping_add(1);
        e.call(STRCPY_S, &args![filename, length, name]);
    }
    for offset in [
        DATA_BYTES,
        BUFFER,
        SECOND_BUFFER,
        BUFFER_3D,
        BUFFER_1D8,
        BUFFER_1D0,
        DIRECT_SOUND,
        WORD_204,
        WORD_208,
    ] {
        set_field(e, this, offset, 0);
    }
    e.mem.set_u8(this.addr().wrapping_add(POSITION_SET), 0);
    for word in 0..3 {
        let value = e.mem.u32(0x011f_426c + 4 * word);
        set_field(e, this, POSITION + 4 * word, value);
    }
    e.call(
        MEMSET,
        &args![this.addr().wrapping_add(BUFFER_DESC), 0u32, 0x24u32],
    );
    let format = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x12u32]).u32();
    set_field(e, this, WAVE_FORMAT, format);
    set_field(e, this, SAMPLE_DATA, 0);
    for (index, value) in [0u16, 600, 0x578, 0xa28, 5000].iter().enumerate() {
        e.mem.set_u16(
            this.addr()
                .wrapping_add(ATTENUATION_CURVE + 2 * index as u32),
            *value,
        );
    }
    set_field(e, this, STREAM, 0);
    set_field(e, this, OGG_READER, 0);
    audio_check::constructed(e, this.addr());
    this
}

/// Releases the COM object at `this + offset` until its count reaches 0 (vtable slot 8,
/// `Release`), then clears the field.
fn release_all(e: &mut Engine, this: Ptr, offset: u32) {
    while com(e, this, offset, 8, &[]) != 0 {}
    set_field(e, this, offset, 0);
}

// Translated from 00aeafb0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aeafb0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Destructor body of `BSWin32GameSound`: stores the vtable, releases the buffer (`+0x19c`), the
/// 3D buffer (`+0x1d4`), the second buffer (`+0x1a0`, counting the global at `011f7128` down
/// once) and `+0x1d8` until their counts reach 0, frees the format block, the sample data and
/// the streaming state (its three blocks at `+0x18`, `+0x20`, `+0x1c` first), destroys and frees
/// the Ogg reader (`00af1d80`), then runs the `BSGameSound` destructor (`00ae93f0`).
pub fn fn_00aeafb0(e: &mut Engine, this: Ptr) {
    audio_check::destroying(e, this.addr());
    set_field(e, this, 0, WIN32_GAME_SOUND_VTABLE);
    if field(e, this, BUFFER) != 0 {
        release_all(e, this, BUFFER);
    }
    if field(e, this, BUFFER_3D) != 0 {
        release_all(e, this, BUFFER_3D);
    }
    if field(e, this, SECOND_BUFFER) != 0 {
        let count = e.mem.u32(0x011f_7128);
        e.mem.set_u32(0x011f_7128, count.wrapping_sub(1));
        release_all(e, this, SECOND_BUFFER);
    }
    if field(e, this, BUFFER_1D8) != 0 {
        release_all(e, this, BUFFER_1D8);
    }
    for offset in [WAVE_FORMAT, SAMPLE_DATA] {
        if field(e, this, offset) != 0 {
            let block = field(e, this, offset);
            e.call(FREE, &args![MEMORY_MANAGER, block]);
            set_field(e, this, offset, 0);
        }
    }
    if field(e, this, STREAM) != 0 {
        for inner in [0x18u32, 0x20, 0x1c] {
            let stream = field(e, this, STREAM);
            let block = e.mem.u32(stream.wrapping_add(inner));
            if block != 0 {
                e.call(FREE, &args![MEMORY_MANAGER, block]);
            }
        }
        let stream = field(e, this, STREAM);
        e.call(FREE, &args![MEMORY_MANAGER, stream]);
    }
    if field(e, this, OGG_READER) != 0 {
        let reader = field(e, this, OGG_READER);
        if reader != 0 {
            fn_00af1d80(e, Ptr::new(reader));
            e.call(FREE, &args![MEMORY_MANAGER, reader]);
        }
        set_field(e, this, OGG_READER, 0);
    }
    e.call(0x00ae_93f0, &args![this]);
}

// Translated from 00af1cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The scalar deleting destructor of `BSWin32GameSound` (vtable slot 0, `RET 4`): the destructor
/// body (`00aeafb0`) and, for bit 0 of `flags`, the memory manager's free. Returns `this`.
pub fn fn_00af1cc0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00aeafb0(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

// Translated from 00af1ba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: makes sure the sound has its `IDirectSound8` (`+0x198`), taking
/// it from the audio manager (`011f6d98` `+0x38`) when missing. Answers whether it has one.
pub fn fn_00af1ba0(e: &mut Engine, this: Ptr) -> u8 {
    if field(e, this, DIRECT_SOUND) != 0 {
        return 1;
    }
    let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
    if e.mem.u32(manager.wrapping_add(0x38)) == 0 {
        return 0;
    }
    let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
    let device = e.mem.u32(manager.wrapping_add(0x38));
    set_field(e, this, DIRECT_SOUND, device);
    1
}

// Translated from 00af13f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x14` (`SetInaudible`, `RET 4`): sets or clears bit 0x20000 of `iStateFlags`
/// from the low byte of `inaudible`.
pub fn fn_00af13f0(e: &mut Engine, this: Ptr, inaudible: u32) {
    if inaudible as u8 != 0 {
        update_state(e, this, |flags| flags | 0x20000);
    } else {
        update_state(e, this, |flags| flags & 0xfffd_ffff);
    }
}

// Translated from 00aefcf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x4c` (`SetEmitterPosition`, `RET 0xc`): passes the three `float`s (copied as
/// `float`s) to the slot `0x50` overload.
pub fn fn_00aefcf0(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32) {
    e.vcall(
        this.addr(),
        0x50,
        &args![
            x87_quiet_bits(x.to_bits()),
            x87_quiet_bits(y.to_bits()),
            x87_quiet_bits(z.to_bits())
        ],
    );
}

// Translated from 00aefd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x54` (`GetEmitterPosition`, `RET 4`): a 3D sound (`00ae9cf0` or `00ae9d10`)
/// copies its position (`+0x218`) to `result`; another one writes 0, 0, 0. Returns `result`.
pub fn fn_00aefd20(e: &mut Engine, this: Ptr, result: Ptr) -> Ptr {
    if e.call(IS_3D, &args![this]).bool() || e.call(IS_3D_OTHER, &args![this]).bool() {
        for word in 0..3 {
            let value = field(e, this, POSITION + 4 * word);
            e.mem.set_u32(result.addr().wrapping_add(4 * word), value);
        }
    } else {
        for word in 0..3 {
            e.mem.set_u32(result.addr().wrapping_add(4 * word), 0);
        }
    }
    result
}

// Translated from 00af1c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 0x10`: `data`, `offset`, `length`, `source`): locks `length`
/// bytes of the buffer at `offset` (`IDirectSoundBuffer::Lock`, slot `0x2c`, one region); a
/// failure answers 0. The region gets `length` bytes from `data + source` (`memcpy`), or zeros
/// when `source` is 0, and is unlocked (slot `0x4c`). Answers 1.
pub fn fn_00af1c00(
    e: &mut Engine,
    this: Ptr,
    data: u32,
    offset: u32,
    length: u32,
    source: u32,
) -> u8 {
    e.with_stack(8, |e, locals| {
        let region = locals.addr();
        let region_bytes = region + 4;
        let locked = com(
            e,
            this,
            BUFFER,
            0x2c,
            &[offset, length, region, region_bytes, 0, 0, 0],
        ) as i32;
        if locked < 0 {
            return 0;
        }
        let target = e.mem.u32(region);
        if source != 0 {
            e.call(MEMCPY, &args![target, source.wrapping_add(data), length]);
        } else {
            e.call(MEMSET, &args![target, 0u32, length]);
        }
        let pointer = e.mem.u32(region);
        let bytes = e.mem.u32(region_bytes);
        com(e, this, BUFFER, 0x4c, &[pointer, bytes, 0, 0]);
        1
    })
}

// Translated from 00aed420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x34` (`Pause`): with a buffer and a device, stops the buffer
/// (`IDirectSoundBuffer::Stop`, slot `0x48`) and the second buffer; on success sets bit 0x40 of
/// `iStateFlags` and clears 0x20. Answers whether the stop succeeded (`E_FAIL` otherwise).
pub fn fn_00aed420(e: &mut Engine, this: Ptr) -> u32 {
    let mut result = E_FAIL;
    if field(e, this, BUFFER) != 0 && field(e, this, DIRECT_SOUND) != 0 {
        result = com(e, this, BUFFER, 0x48, &[]);
        if field(e, this, SECOND_BUFFER) != 0 {
            com(e, this, SECOND_BUFFER, 0x48, &[]);
        }
    }
    if result as i32 >= 0 {
        update_state(e, this, |flags| flags | 0x40);
        update_state(e, this, |flags| flags & 0xffff_ffdf);
    }
    u32::from(result as i32 >= 0)
}

// Translated from 00aed4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x38` (`Stop`). With a buffer and a device: a streaming sound with streaming
/// state not in state 5 or 6 is put in state 4; one in state 5 or 6 is left alone; anything else
/// stops both buffers (slot `0x48`) and rewinds them (`SetCurrentPosition(0)`, slot `0x34`),
/// clearing bit 0x20 of `iStateFlags` on success. Bit 0x40 is then cleared when set. Answers
/// whether the stop succeeded.
pub fn fn_00aed4c0(e: &mut Engine, this: Ptr) -> u32 {
    let mut result = E_FAIL;
    if field(e, this, BUFFER) != 0 && field(e, this, DIRECT_SOUND) != 0 {
        let stream = field(e, this, STREAM);
        let parked = is_streaming(e, this)
            && stream != 0
            && e.mem.u32(stream) != 5
            && e.mem.u32(stream) != 6;
        if parked {
            let stream = field(e, this, STREAM);
            e.mem.set_u32(stream, 4);
        } else if !(is_streaming(e, this) && field(e, this, STREAM) != 0) {
            result = com(e, this, BUFFER, 0x48, &[]);
            com(e, this, BUFFER, 0x34, &[0]);
            if field(e, this, SECOND_BUFFER) != 0 {
                com(e, this, SECOND_BUFFER, 0x48, &[]);
                com(e, this, SECOND_BUFFER, 0x34, &[0]);
            }
            if result as i32 >= 0 {
                update_state(e, this, |flags| flags & 0xffff_ffdf);
            }
        }
        if field(e, this, STATE_FLAGS) & 0x40 != 0 {
            update_state(e, this, |flags| flags & 0xffff_ffbf);
        }
    }
    u32::from(result as i32 >= 0)
}

// Translated from 00aed080 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x30` (`Play`, `RET 4`: `looping`, its low byte; the exe reuses it as a local).
/// With a buffer and a device: a sound with bit 0x10 starts at a random position (`rand() %
/// (size - 1)`, `SetCurrentPosition`) and loops; a streaming one (or bit 0x10) that is not paused
/// in stream state 5 loops and, with stream data (`+0x18`) and a state other than 1 and 2,
/// rewinds and fills the buffer from the stream: all of it (state 1) when the stream's length
/// (`+8`) is above the buffer, else its tail then its head (`+0x18`, `+0x1c`, state 2) — through
/// `00af1c00`. Both buffers then play (`IDirectSoundBuffer::Play(0, 0, looping)`, slot `0x30`)
/// and `00ae9af0` runs. Success sets bit 0x20 of `iStateFlags` and clears 0x40; the volume is
/// applied again (vtable slot `0x3c` with `fCurrentVolume`). A local the exe never sets would
/// also run vtable slot `0x44` with 0. Answers whether the play succeeded.
pub fn fn_00aed080(e: &mut Engine, this: Ptr, looping: u32) -> u32 {
    let mut looping = looping as u8;
    let mut result = E_FAIL;
    let update_after = false;
    if field(e, this, BUFFER) != 0 && field(e, this, DIRECT_SOUND) != 0 {
        if flags8(e, this) & 0x10 != 0 {
            looping = 1;
            let size = field(e, this, SOUND_SIZE);
            let random = e.call(0x00ec_adb8, &args![]).u32();
            com(e, this, BUFFER, 0x34, &[random % size.wrapping_sub(1)]);
        } else if flags8(e, this) & 0x10 != 0 || is_streaming(e, this) {
            let paused_in_5 = e.vcall(this.addr(), 0xc, &args![]).bool()
                && field(e, this, STREAM) != 0
                && e.mem.u32(field(e, this, STREAM)) == 5;
            if !paused_in_5 {
                looping = 1;
                play_fill_stream(e, this);
            }
        }
        result = if looping != 0 {
            let result = com(e, this, BUFFER, 0x30, &[0, 0, 1]);
            if field(e, this, SECOND_BUFFER) != 0 {
                com(e, this, SECOND_BUFFER, 0x30, &[0, 0, 1]);
            }
            result
        } else {
            let result = com(e, this, BUFFER, 0x30, &[0, 0, 0]);
            if field(e, this, SECOND_BUFFER) != 0 {
                com(e, this, SECOND_BUFFER, 0x30, &[0, 0, 0]);
            }
            result
        };
        e.call(AFTER_PLAY, &args![this]);
    }
    if result as i32 >= 0 {
        update_state(e, this, |flags| flags | 0x20);
        update_state(e, this, |flags| flags & 0xffff_ffbf);
    }
    let volume = x87_quiet_bits(field(e, this, CURRENT_VOLUME));
    e.vcall(this.addr(), 0x3c, &args![volume]);
    if update_after {
        e.vcall(this.addr(), 0x44, &args![0u32]);
    }
    u32::from(result as i32 >= 0)
}

/// The stream fill of `Play` (`00aed080`).
fn play_fill_stream(e: &mut Engine, this: Ptr) {
    if !is_streaming(e, this) || field(e, this, STREAM) == 0 {
        return;
    }
    let stream = field(e, this, STREAM);
    if e.mem.u32(stream.wrapping_add(0x18)) == 0 || e.mem.u32(stream) == 1 || e.mem.u32(stream) == 2
    {
        return;
    }
    com(e, this, BUFFER, 0x34, &[0]);
    let stream = field(e, this, STREAM);
    let length = e.mem.u32(stream.wrapping_add(8));
    if length > field(e, this, DATA_BYTES) {
        let data = e.mem.u32(stream.wrapping_add(0x18));
        let bytes = field(e, this, DATA_BYTES);
        fn_00af1c00(e, this, 0, 0, bytes, data);
        let bytes = field(e, this, DATA_BYTES);
        let stream = field(e, this, STREAM);
        e.mem.set_u32(stream.wrapping_add(4), bytes);
        e.mem.set_u32(stream, 1);
    } else {
        let rest = field(e, this, DATA_BYTES).wrapping_sub(length);
        let data = e.mem.u32(stream.wrapping_add(0x18));
        fn_00af1c00(e, this, 0, 0, length, data);
        let stream = field(e, this, STREAM);
        let head = e.mem.u32(stream.wrapping_add(0x1c));
        let length = e.mem.u32(stream.wrapping_add(8));
        fn_00af1c00(e, this, 0, length, rest, head);
        let stream = field(e, this, STREAM);
        let length = e.mem.u32(stream.wrapping_add(8));
        e.mem
            .set_u32(stream.wrapping_add(4), length.wrapping_add(rest));
        e.mem.set_u32(stream, 2);
    }
}

/// A `float`'s value clamped by `FCOMP`: `TEST AH,mask` / `JP` keeps `value` (the parity even
/// case), otherwise `replacement`.
fn keep_or(value: u32, replacement: u32, compare: u8, mask: u8) -> u32 {
    if parity_even(compare & mask) {
        x87_quiet_bits(value)
    } else {
        replacement
    }
}

// Translated from 00aed660 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x3c` (`SetVolume`, `RET 4`; answer in `AL`). Stores `fCurrentVolume`; with a
/// buffer, the volume is clamped to `[0, 1]`, scaled by the audio manager (`00adac30` on
/// `BSAudioManager::QInstance` with `iSoundTypeFlags`), forced to the floor (`010718c0`) for a
/// non-`0x200` state with bit 0x10000 and no type bit 0x1000 or below the minimum (`0101dae8`),
/// and turned into hundredths of a decibel (`_ftol2(log10(v) * 2000)` minus the static, system
/// and fader attenuations, and the distance attenuation for a 3D sound), clamped to `[-10000,
/// -1]`; the second buffer adds the reverb attenuation (`iReverbAttenuation / 100 * -10000`,
/// truncated) and is clamped the same way. A muted sound (state bit 0x400) sets both to -10000.
/// The answer is whether the main buffer's `SetVolume` (slot `0x3c`) succeeded.
pub fn fn_00aed660(e: &mut Engine, this: Ptr, volume: f32) -> u8 {
    set_field(e, this, CURRENT_VOLUME, x87_quiet_bits(volume.to_bits()));
    let reverb = x87_store32(f64::from(e.mem.u16(this.addr() + REVERB_ATTENUATION)));
    let reverb = x87_store32(x87_div(x87_load32(reverb), double_at(e, HUNDRED_01017A40)));
    if field(e, this, BUFFER) == 0 {
        return 0;
    }
    let bits = volume.to_bits();
    let bits = if fpu_compare(x87_load32(bits), double_at(e, ONE_01012070)) & 0x41 == 0 {
        1.0f32.to_bits()
    } else {
        x87_quiet_bits(bits)
    };
    let bits = keep_or(
        bits,
        0,
        fpu_compare(x87_load32(bits), double_at(e, ZERO_01012060)),
        0x05,
    );
    let manager = e.call(0x00ad_9060, &args![]).u32();
    let flags = flags8(e, this);
    let mut bits = x87_store32(
        e.call(0x00ad_ac30, &args![manager, x87_quiet_bits(bits), flags])
            .f64(),
    );
    let state = field(e, this, STATE_FLAGS);
    if state & 0x200 == 0 && state & 0x10000 != 0 && flags8(e, this) & 0x1000 == 0 {
        bits = x87_quiet_bits(e.mem.u32(0x0107_18c0));
    }
    let floor = x87_quiet_bits(e.mem.u32(0x0107_18c0));
    bits = keep_or(
        bits,
        floor,
        fpu_compare(x87_load32(bits), double_at(e, 0x0101_dae8)),
        0x05,
    );
    let log = x87_store32(e.call(0x00ec_bc80, &args![x87_load32(bits)]).f64());
    let scaled = x87_mul(x87_load32(log), double_at(e, 0x0107_3490));
    let mut main = e.call(0x00ec_62c0, &args![scaled]).u32() as i32;
    let base = this.addr();
    let attenuation = i32::from(e.mem.u16(base + STATIC_ATTENUATION))
        + i32::from(e.mem.u16(base + SYSTEM_ATTENUATION))
        + i32::from(e.mem.u16(base + FADER_ATTENUATION));
    main = main.wrapping_sub(attenuation);
    if e.call(IS_3D, &args![this]).bool() || e.call(IS_3D_OTHER, &args![this]).bool() {
        main = main.wrapping_sub(i32::from(e.mem.u16(base + DISTANCE_ATTENUATION)));
    }
    let clamp = |value: i32| {
        let value = if value <= -10000 { -10000 } else { value };
        if value >= -1 {
            -1
        } else {
            value
        }
    };
    main = clamp(main);
    let reverb_scaled = x87_mul(x87_load32(reverb), double_at(e, 0x0102_d268));
    let reverb_level = x87_fistp64_chop(X87Ext::load64(reverb_scaled)) as i32;
    let second = clamp(main.wrapping_add(reverb_level));
    if field(e, this, STATE_FLAGS) & 0x400 != 0 {
        if field(e, this, SECOND_BUFFER) != 0 {
            com(e, this, SECOND_BUFFER, 0x3c, &[(-10000i32) as u32]);
        }
        let result = com(e, this, BUFFER, 0x3c, &[(-10000i32) as u32]);
        return u8::from(result as i32 >= 0);
    }
    let mut result = E_FAIL;
    if field(e, this, SECOND_BUFFER) != 0 {
        com(e, this, SECOND_BUFFER, 0x3c, &[second as u32]);
    }
    if field(e, this, BUFFER) != 0 {
        result = com(e, this, BUFFER, 0x3c, &[main as u32]);
    }
    u8::from(result as i32 >= 0)
}

// Translated from 00aefda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x5c` (`SetMinMax`, `RET 8`: `min`, `max`). When either differs from
/// `fMinDist`/`fMaxDist` (`FUCOM`; a NaN counts as different), both are stored and a placed
/// sound (`+0x224`) is moved to its position again (vtable slot `0x4c` with the point). A 3D
/// sound with a 3D buffer and a device then sets the buffer's minimum and maximum distance
/// (`IDirectSound3DBuffer` slots `0x44` and `0x40`, both with the `float` at `010a3dcc`,
/// immediately); `fMinDist` becomes `min` when it is above 0, else the `float` at `010a3e58`, and
/// `fMaxDist` becomes `max` when above 0, else the `float` at `010a3dcc`.
pub fn fn_00aefda0(e: &mut Engine, this: Ptr, min: f32, max: f32) {
    let base = this.addr();
    let max_changed = parity_even(
        fpu_compare(
            x87_load32(field(e, this, MAX_DISTANCE)),
            x87_load32(max.to_bits()),
        ) & 0x44,
    );
    let changed = max_changed
        || parity_even(
            fpu_compare(
                x87_load32(field(e, this, MIN_DISTANCE)),
                x87_load32(min.to_bits()),
            ) & 0x44,
        );
    if changed {
        set_field(e, this, MAX_DISTANCE, x87_quiet_bits(max.to_bits()));
        set_field(e, this, MIN_DISTANCE, x87_quiet_bits(min.to_bits()));
        if e.mem.u8(base + POSITION_SET) != 0 {
            let point = [
                field(e, this, POSITION),
                field(e, this, POSITION + 4),
                field(e, this, POSITION + 8),
            ];
            e.vcall(base, 0x4c, &point);
        }
    }
    if !e.call(IS_3D, &args![this]).bool()
        || field(e, this, BUFFER_3D) == 0
        || field(e, this, DIRECT_SOUND) == 0
    {
        return;
    }
    let zero = double_at(e, ZERO_01012060);
    let distance = x87_quiet_bits(e.mem.u32(0x010a_3dcc));
    com(e, this, BUFFER_3D, 0x44, &[distance, 0]);
    let new_min = if fpu_compare(x87_load32(min.to_bits()), zero) & 0x41 == 0 {
        x87_quiet_bits(min.to_bits())
    } else {
        x87_quiet_bits(e.mem.u32(0x010a_3e58))
    };
    set_field(e, this, MIN_DISTANCE, new_min);
    let distance = x87_quiet_bits(e.mem.u32(0x010a_3dcc));
    com(e, this, BUFFER_3D, 0x40, &[distance, 0]);
    let new_max = if fpu_compare(x87_load32(max.to_bits()), zero) & 0x41 == 0 {
        x87_quiet_bits(max.to_bits())
    } else {
        x87_quiet_bits(e.mem.u32(0x010a_3dcc))
    };
    set_field(e, this, MAX_DISTANCE, new_max);
}

// Translated from 00aeff60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x60` (`SetAttenuationCurve`, `RET 0x14`: five words, their low halves read):
/// each non-zero value `v` becomes `truncate(log10(v / 100) * c)` with the `double` `c` at
/// `010a3e68` (the quotient and the logarithm stored as `float`s), a zero one the `double` at
/// `010a3e60` truncated; the low half-words go to `+0x20c` to `+0x214`.
pub fn fn_00aeff60(e: &mut Engine, this: Ptr, a: u32, b: u32, c: u32, d: u32, f: u32) {
    for (index, value) in [a, b, c, d, f].iter().enumerate() {
        let value = *value as u16;
        let level = if value != 0 {
            let ratio = x87_store32(x87_div(f64::from(value), double_at(e, HUNDRED_01017A40)));
            let log = x87_store32(e.call(0x00ec_bc80, &args![x87_load32(ratio)]).f64());
            x87_mul(x87_load32(log), double_at(e, 0x010a_3e68))
        } else {
            double_at(e, 0x010a_3e60)
        };
        let truncated = fistp_chop(level);
        e.mem.set_u16(
            this.addr()
                .wrapping_add(ATTENUATION_CURVE + 2 * index as u32),
            truncated as u16,
        );
    }
}

// Translated from 00af01e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x64` (`SetFrequency`, `RET 4`): `iModSamplesPerSecond * ratio` truncated (a
/// 64-bit `FISTP`, its low word), clamped to `[100, 200000]`, goes to the buffer's
/// `SetFrequency` (slot `0x44`) when there are a buffer and a device. Answers whether that
/// succeeded (`E_FAIL` without them).
pub fn fn_00af01e0(e: &mut Engine, this: Ptr, ratio: f32) -> u32 {
    let mut result = E_FAIL;
    let rate = f64::from(e.mem.u16(this.addr() + MOD_SAMPLES_PER_SECOND));
    let product = x87_mul(rate, x87_m32(ratio.to_bits()));
    let frequency = (x87_fistp64_chop(X87Ext::load64(product)) as u32).clamp(100, 200_000);
    if field(e, this, BUFFER) != 0 && field(e, this, DIRECT_SOUND) != 0 {
        result = com(e, this, BUFFER, 0x44, &[frequency]);
    }
    u32::from(result as i32 >= 0)
}

// Translated from 00af0290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x68` (`GetFrequency`): the buffer's frequency (`GetFrequency`, slot `0x20`)
/// over `iModSamplesPerSecond`, stored as a `float`; 1.0 without a buffer and a device or when
/// the call fails. The result is in `ST0`.
pub fn fn_00af0290(e: &mut Engine, this: Ptr) -> f64 {
    let mut result = E_FAIL;
    let mut ratio = 1.0f32.to_bits();
    e.with_stack(4, |e, frequency| {
        let frequency = frequency.addr();
        if field(e, this, BUFFER) != 0 && field(e, this, DIRECT_SOUND) != 0 {
            result = com(e, this, BUFFER, 0x20, &[frequency]);
        }
        if result as i32 >= 0 {
            let value = f64::from(e.mem.u32(frequency));
            let rate = f64::from(e.mem.u16(this.addr() + MOD_SAMPLES_PER_SECOND));
            ratio = x87_store32(x87_div(value, rate));
        }
    });
    x87_load32(ratio)
}

// Translated from 00af0320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x6c` (`Seek`, `RET 4`: milliseconds). Without a buffer, or at or past the
/// duration (`+0x14`, unsigned), it prints `"Attempted to seek to %.2fs in a sound of only %.2fs
/// duration."` (both `/ 1000.0` as `double`s). Otherwise the buffer's format is read
/// (`GetFormat`, slot `0x14`, into a zeroed 0x12-byte block) and the position set to `ms *
/// _ftol2(nAvgBytesPerSec / 1000)` (`SetCurrentPosition`, slot `0x34`).
pub fn fn_00af0320(e: &mut Engine, this: Ptr, milliseconds: u32) {
    let thousand = double_at(e, 0x0101_7b70);
    if field(e, this, BUFFER) == 0 || milliseconds >= field(e, this, DURATION) {
        let duration = x87_div(f64::from(field(e, this, DURATION) as i32), thousand);
        let position = x87_div(f64::from(milliseconds), thousand);
        e.call(PRINT, &args![0x010a_3e70u32, position, duration]);
        return;
    }
    e.with_stack(0x14, |e, format| {
        let format = format.addr();
        e.call(MEMSET, &args![format, 0u32, 0x12u32]);
        com(e, this, BUFFER, 0x14, &[format, 0x12, 0]);
        let average = f64::from(e.mem.u32(format + 8));
        let per_millisecond = e
            .call(0x00ec_62c0, &args![x87_div(average, thousand)])
            .u32();
        com(
            e,
            this,
            BUFFER,
            0x34,
            &[milliseconds.wrapping_mul(per_millisecond)],
        );
    });
}

/// WinMM imports `Open` uses (by import slot).
const MMIO_OPEN: u32 = 0x00fd_f318;
const MMIO_DESCEND: u32 = 0x00fd_f31c;
const MMIO_CLOSE: u32 = 0x00fd_f320;
const MMIO_READ: u32 = 0x00fd_f324;
const MMIO_ASCEND: u32 = 0x00fd_f328;
const MMIO_GET_INFO: u32 = 0x00fd_f32c;
const MMIO_ADVANCE: u32 = 0x00fd_f330;
/// `MMIOERR_CHUNKNOTFOUND`.
const CHUNK_NOT_FOUND: u32 = 0x109;
/// `"Data\\Sound\\FX\\silent_lp.wav"`, `"Data\\Sound\\FX\\TEMP_Missing_Voice_Asset.wav"`,
/// `"Data\\Sound\\FX\\UI\\RADIO\\TEMP_radioassetmissing.wav"`.
const SILENT_LOOP_010A3D20: u32 = 0x010a_3d20;
const MISSING_VOICE_010A3D3C: u32 = 0x010a_3d3c;
const MISSING_RADIO_010A3D68: u32 = 0x010a_3d68;
/// `"Audio: Could not open file %s\n"`.
const OPEN_FAILED_010A3D9C: u32 = 0x010a_3d9c;
/// `"Audio: Attempted to play multi-channel sound (%s) in 3D. Forcing 2D playback."`.
const MULTI_CHANNEL_010A3CD0: u32 = 0x010a_3cd0;
/// `"Audio: Loop envelope sound (%s) missing smpl chunk. ..."`.
const MISSING_SMPL_010A3C68: u32 = 0x010a_3c68;
/// `".mp3"`, `".ogg"`, `".wav"`.
const MP3_01033AD8: u32 = 0x0103_3ad8;
const OGG_010A3DBC: u32 = 0x010a_3dbc;
const WAV_0107CE2C: u32 = 0x0107_ce2c;
/// The null GUID and `DS3DALG_HRTF_FULL` (16 bytes each).
const GUID_NULL_01093F04: u32 = 0x0109_3f04;
const HRTF_GUID_01100FE4: u32 = 0x0110_0fe4;
/// `0.2` (`double`, the `float` 0.2 widened).
const FIFTH_01018A90: u32 = 0x0101_8a90;

/// The size of `Open`'s modelled stack frame: every local whose address the code passes on
/// lives at `frame + OPEN_FRAME - offset` (`offset` below `EBP`).
const OPEN_FRAME: u32 = 0x1a0;

fn copy_guid(e: &mut Engine, this: Ptr, source: u32) {
    for word in 0..4 {
        let value = e.mem.u32(source + 4 * word);
        set_field(e, this, ALGORITHM_GUID + 4 * word, value);
    }
}

/// `mmioOpenA(pFilename, info, MMIO_ALLOCBUF)` after `strcpy_s(pFilename, 0x40, replacement)`
/// and `CalculateHashes(1)`.
fn reopen_as(e: &mut Engine, this: Ptr, replacement: u32, info: u32) -> u32 {
    let filename = this.addr().wrapping_add(FILENAME);
    e.call(STRCPY_S, &args![filename, 0x40u32, replacement]);
    e.call(CALCULATE_HASHES, &args![this, 1u32]);
    e.call(MMIO_OPEN, &args![filename, info, 0x10000u32]).u32()
}

// Translated from 00aeb270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x24` (`Open`). Sets bit 0x1000 of `iStateFlags`; without a device
/// (`00af1ba0`) the open fails (`00ae9c50(0)`). A sound with any of the type bits 0x1000, 0x2000,
/// 4, 0x400000, 0x100000 or 0x800 is first tried as an Ogg Vorbis stream (`00aebff0`; success is
/// `00ae9c50(1)`). Otherwise the `.mp3`/`.ogg` name is turned into `.wav` and read through WinMM
/// with this unit's I/O procedure (`00af0400`, four-character code `"WAV "`): a missing file is
/// reported and replaced by the radio (types 0x400000/0x100000), voice (type 4) or, for a
/// looping or streaming sound, the silent loop placeholder (the streaming bits cleared); others
/// fail. The `RIFF` chunk, the `fmt ` chunk (at least 16 bytes, PCM) and the `data` chunk give
/// the format (copied to the `WAVEFORMATEX`, `cbSize` 0), the sample rates, the data size and
/// the duration; a multi-channel 3D sound is forced to 2D (reported unless type bit 0x8000000).
/// The buffer flags are 0x40 (2D) or 0x30010 (3D, with the HRTF algorithm when the audio
/// manager's settings word `+0x60` is set), plus 0x2a0. A non-streaming sound is padded: up to
/// 250 ms for a sound of 150 ms or less (dropping flag 0x200), plus `min(1000 ms, max(500 ms,
/// size * 0.2))` of samples. The samples are copied through `mmioGetInfo`/`mmioAdvance` into a
/// zeroed block from the memory manager (a failed advance fails the open), the sizes and
/// duration updated, and a streaming sound reads its loop points from the `smpl` chunk (missing:
/// reported and failed). The answer is `00ae9c50`'s, or 0 (in `AL`, the rest of `EAX` from
/// `mmioClose`) for a bad chunk.
pub fn fn_00aeb270(e: &mut Engine, this: Ptr) -> u32 {
    audio_check::opening(e, this.addr());
    update_state(e, this, |flags| flags | 0x1000);
    if fn_00af1ba0(e, this) == 0 {
        return set_opened(e, this, 0);
    }
    let wants_stream = [0x1000u32, 0x2000, 4, 0x40_0000, 0x10_0000, 0x800]
        .iter()
        .any(|bit| flags8(e, this) & bit != 0);
    if wants_stream && fn_00aebff0(e, this) as u8 != 0 {
        return set_opened(e, this, 1);
    }
    e.with_stack(OPEN_FRAME, |e, frame| {
        let at = |offset: u32| frame.addr() + OPEN_FRAME - offset;
        open_wave(e, this, &at)
    })
}

/// The WinMM part of `Open` (`00aeb270`).
fn open_wave(e: &mut Engine, this: Ptr, at: &dyn Fn(u32) -> u32) -> u32 {
    let filename = this.addr().wrapping_add(FILENAME);
    let mut extension = e.call(STRSTR, &args![filename, MP3_01033AD8]).u32();
    if extension == 0 {
        extension = e.call(STRSTR, &args![filename, OGG_010A3DBC]).u32();
    }
    if extension != 0 {
        e.mem.set_u8(extension + 1, b'w');
        e.mem.set_u8(extension + 2, b'a');
        e.mem.set_u8(extension + 3, b'v');
    }
    set_field(e, this, CREATED_BUFFER, 0);
    let info = at(0xf0);
    e.call(MEMSET, &args![info, 0u32, 0x48u32]);
    e.mem.set_u32(info + 4, 0x0056_4157);
    e.mem.set_u32(info + 8, 0x00af_0400);
    let mut file = e.call(MMIO_OPEN, &args![filename, info, 0x10000u32]).u32();
    if file == 0 {
        e.call(PRINT, &args![OPEN_FAILED_010A3D9C, filename]);
        let flags = flags8(e, this);
        file = if flags & 0x40_0000 != 0 || flags & 0x10_0000 != 0 {
            reopen_as(e, this, MISSING_RADIO_010A3D68, info)
        } else if flags & 4 != 0 {
            reopen_as(e, this, MISSING_VOICE_010A3D3C, info)
        } else if flags8(e, this) & 0x10 != 0 || is_streaming(e, this) {
            update_flags8(e, this, |flags| flags & 0xfdff_ffff);
            update_flags8(e, this, |flags| flags & 0xfbff_ffff);
            reopen_as(e, this, SILENT_LOOP_010A3D20, info)
        } else {
            0
        };
        if file == 0 {
            return set_opened(e, this, 0);
        }
    }
    let riff = at(0x9c);
    let chunk = at(0x38);
    if e.call(MMIO_DESCEND, &args![file, riff, 0u32, 0u32]).u32() == CHUNK_NOT_FOUND {
        e.call(MMIO_CLOSE, &args![file, 0u32]);
        return set_opened(e, this, 0);
    }
    e.mem.set_u32(chunk, 0x2074_6d66);
    if e.call(MMIO_DESCEND, &args![file, chunk, riff, 0x10u32])
        .u32()
        == CHUNK_NOT_FOUND
        || e.mem.u32(chunk + 4) < 0x10
    {
        return e.call(MMIO_CLOSE, &args![file, 0u32]).u32() & 0xffff_ff00;
    }
    let wave = at(0x14);
    e.call(MMIO_READ, &args![file, wave, 0x10u32]);
    if e.mem.u16(wave) != 1 {
        return e.call(MMIO_CLOSE, &args![file, 0u32]).u32() & 0xffff_ff00;
    }
    e.call(MMIO_ASCEND, &args![file, chunk, 0u32]);
    e.mem.set_u32(chunk, 0x6174_6164);
    e.call(MMIO_DESCEND, &args![file, chunk, riff, 0x10u32]);
    let data_size = e.mem.u32(chunk + 4);
    set_field(e, this, DATA_BYTES, data_size);
    set_field(e, this, SOUND_SIZE, data_size);
    set_field(e, this, BUFFER_DESC, 0x24);
    if i32::from(e.mem.u16(wave + 2)) > 1 && (flags8(e, this) & 1 == 0 || flags8(e, this) & 2 != 0)
    {
        update_flags8(e, this, |flags| flags | 1);
        update_flags8(e, this, |flags| flags & 0xffff_fffd);
        if flags8(e, this) & 0x0800_0000 == 0 {
            e.call(PRINT, &args![MULTI_CHANNEL_010A3CD0, filename]);
        }
    }
    if flags8(e, this) & 1 != 0 {
        set_field(e, this, BUFFER_FLAGS, 0x40);
        copy_guid(e, this, GUID_NULL_01093F04);
    } else {
        set_field(e, this, BUFFER_FLAGS, 0x30010);
        // The exe copies 0x18 words of the audio manager's settings (`+0x40`) to its stack and
        // tests the ninth.
        let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
        if e.mem.u32(manager.wrapping_add(0x60)) != 0 {
            copy_guid(e, this, HRTF_GUID_01100FE4);
        } else {
            copy_guid(e, this, GUID_NULL_01093F04);
        }
    }
    let flags = field(e, this, BUFFER_FLAGS) | 0x2a0;
    set_field(e, this, BUFFER_FLAGS, flags);
    let bytes = field(e, this, DATA_BYTES);
    set_field(e, this, BUFFER_BYTES, bytes);
    let format = field(e, this, WAVE_FORMAT);
    e.call(MEMCPY, &args![format, wave, 0x10u32]);
    let format = field(e, this, WAVE_FORMAT);
    e.mem.set_u16(format + 0x10, 0);
    let rate = e.mem.u16(format + 4);
    e.mem.set_u16(this.addr() + SAMPLES_PER_SECOND, rate);
    e.mem.set_u16(this.addr() + MOD_SAMPLES_PER_SECOND, rate);
    let per_millisecond = (i32::from(rate) / 1000) as u32;
    let bits = i32::from(e.mem.u16(format + 0xe));
    let bytes_per_sample = (bits / 8) as u32;
    let duration = field(e, this, DATA_BYTES) / bytes_per_sample / per_millisecond;
    set_field(e, this, DURATION, duration);
    let mut padding = 0u32;
    let extra = 0u32;
    if field(e, this, DURATION) as i32 <= 0x96
        && !(flags8(e, this) & 0x10 != 0 || is_streaming(e, this))
    {
        let flags = field(e, this, BUFFER_FLAGS) & 0xffff_fdff;
        set_field(e, this, BUFFER_FLAGS, flags);
        padding = 0xfau32
            .wrapping_sub(field(e, this, DURATION))
            .wrapping_mul(bytes_per_sample)
            .wrapping_mul(per_millisecond);
    }
    if !(flags8(e, this) & 0x10 != 0 || is_streaming(e, this)) {
        padding = padding.wrapping_add(open_padding(e, this, bytes_per_sample, per_millisecond));
    }
    let total = data_size.wrapping_add(padding).wrapping_add(extra);
    let samples = e.call(ALLOCATE, &args![MEMORY_MANAGER, total]).u32();
    set_field(e, this, SAMPLE_DATA, samples);
    if field(e, this, SAMPLE_DATA) == 0 {
        e.call(MMIO_CLOSE, &args![file, 0u32]);
        return set_opened(e, this, 0);
    }
    let samples = field(e, this, SAMPLE_DATA);
    e.call(MEMSET, &args![samples, 0u32, total]);
    let reader = at(0x88);
    e.call(MMIO_GET_INFO, &args![file, reader, 0u32]);
    let mut index = extra;
    while index < e.mem.u32(chunk + 4) {
        if e.mem.u32(reader + 0x1c) == e.mem.u32(reader + 0x20)
            && e.call(MMIO_ADVANCE, &args![file, reader, 0u32]).u32() != 0
        {
            e.call(MMIO_CLOSE, &args![file, 0u32]);
            return set_opened(e, this, 0);
        }
        let next = e.mem.u32(reader + 0x1c);
        let byte = e.mem.u8(next);
        let samples = field(e, this, SAMPLE_DATA);
        e.mem.set_u8(samples.wrapping_add(index), byte);
        e.mem.set_u32(reader + 0x1c, next.wrapping_add(1));
        index = index.wrapping_add(1);
    }
    let bytes = padding
        .wrapping_add(extra)
        .wrapping_add(field(e, this, DATA_BYTES));
    set_field(e, this, DATA_BYTES, bytes);
    set_field(e, this, BUFFER_BYTES, bytes);
    set_field(e, this, SOUND_SIZE, bytes);
    let duration = field(e, this, DATA_BYTES) / bytes_per_sample / per_millisecond;
    set_field(e, this, DURATION, duration);
    if flags8(e, this) & 0x0200_0000 != 0 || flags8(e, this) & 0x0400_0000 != 0 {
        e.call(MMIO_ASCEND, &args![file, chunk, 0u32]);
        e.mem.set_u32(chunk, 0x6c70_6d73);
        if e.call(MMIO_DESCEND, &args![file, chunk, riff, 0x10u32])
            .u32()
            == CHUNK_NOT_FOUND
        {
            e.mem.set_u32(chunk, 0x2074_6d66);
            e.call(MMIO_CLOSE, &args![file, 0u32]);
            e.call(PRINT, &args![MISSING_SMPL_010A3C68, filename]);
            return set_opened(e, this, 0);
        }
        let sample_chunk = at(0x130);
        e.call(MMIO_READ, &args![file, sample_chunk, 0x3cu32]);
        let start = e.mem.u32(sample_chunk + 0x2c);
        let end = e.mem.u32(sample_chunk + 0x30);
        set_field(e, this, LOOP_START, start);
        set_field(e, this, LOOP_END, end);
    }
    e.call(MMIO_CLOSE, &args![file, 0u32]);
    set_opened(e, this, 1)
}

/// The tail padding of `Open`: `min(1000 ms, max(500 ms, size * 0.2))` of samples, compared
/// and truncated in extended precision (the 64-bit `FISTP`'s low word).
fn open_padding(e: &mut Engine, this: Ptr, bytes_per_sample: u32, per_millisecond: u32) -> u32 {
    let fifth = double_at(e, FIFTH_01018A90);
    let half_second = f64::from(
        bytes_per_sample
            .wrapping_mul(0x1f4)
            .wrapping_mul(per_millisecond),
    );
    let second = f64::from(
        bytes_per_sample
            .wrapping_mul(0x3e8)
            .wrapping_mul(per_millisecond),
    );
    let at_least_half = |e: &Engine| {
        let scaled = x87_mul(f64::from(field(e, this, DATA_BYTES)), fifth);
        if !parity_even(fpu_compare(half_second, scaled) & 0x05) {
            x87_mul(f64::from(field(e, this, DATA_BYTES)), fifth)
        } else {
            half_second
        }
    };
    let longer = at_least_half(e);
    let amount = if fpu_compare(second, longer) & 0x41 != 0 {
        second
    } else {
        at_least_half(e)
    };
    x87_fistp64_chop(X87Ext::load64(amount)) as u32
}

// Translated from 00aebff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aebff0 - the C++ exception-unwinding frame (FS:[0]) is not translated
/// Unnamed in the engine map: opens the sound as an Ogg Vorbis stream. A reader (0x2e0 bytes
/// from the memory manager, `00af1d20`) goes to `+0x22c` and opens `pFilename` (`00af1e00`); a
/// failure destroys and frees it and answers 0. A streaming sound with type bit 0x2000 or 0x1000
/// gets type bit 0x10; the streaming bits are cleared. The `WAVEFORMATEX` becomes 16-bit PCM
/// with the stream's rate and channel count (`vorbis_info` at the reader's `+0x2d8`: `+8` rate,
/// `+4` channels; 0 and 1 without it), the buffer size `pcm_total * 2 * channels` (`+0x2d4`),
/// and the decoded samples are read into a zeroed block (`00af2100`), half a second larger for a
/// sound with type bit 0x400000 or 0x100000 (whose duration also gains 500 ms). The duration is
/// `size / (channels * 2) / (rate / 1000)`. The buffer flags are 0x40 (forcing 2D) for a 2D or
/// multi-channel sound, else 0x30010, plus 0x2a0. Answers `00ae9c50(this, 1)`.
pub fn fn_00aebff0(e: &mut Engine, this: Ptr) -> u32 {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x2e0u32]).u32();
    let reader = if block != 0 {
        fn_00af1d20(e, Ptr::new(block)).addr()
    } else {
        0
    };
    audio_check::reader_made(e, this.addr(), reader, field(e, this, OGG_READER));
    set_field(e, this, OGG_READER, reader);
    let reader = Ptr::new(field(e, this, OGG_READER));
    if fn_00af1e00(e, reader, this.addr().wrapping_add(FILENAME)) == 0 {
        let reader = field(e, this, OGG_READER);
        if reader != 0 {
            fn_00af1d80(e, Ptr::new(reader));
            e.call(FREE, &args![MEMORY_MANAGER, reader]);
        }
        set_field(e, this, OGG_READER, 0);
        return 0;
    }
    if is_streaming(e, this) && (flags8(e, this) & 0x2000 != 0 || flags8(e, this) & 0x1000 != 0) {
        update_flags8(e, this, |flags| flags | 0x10);
    }
    update_flags8(e, this, |flags| flags & 0xfdff_ffff);
    update_flags8(e, this, |flags| flags & 0xfbff_ffff);
    let preload = flags8(e, this) & 0x40_0000 != 0 || flags8(e, this) & 0x10_0000 != 0;
    set_field(e, this, BUFFER_DESC, 0x24);
    let format = field(e, this, WAVE_FORMAT);
    e.mem.set_u16(format + 0x10, 0);
    e.mem.set_u16(format, 1);
    let info = |e: &Engine, this: Ptr| {
        let reader = field(e, this, OGG_READER);
        e.mem.u32(reader.wrapping_add(0x2d8))
    };
    let rate = match info(e, this) {
        0 => 0,
        info => e.mem.u32(info + 8),
    };
    let format = field(e, this, WAVE_FORMAT);
    e.mem.set_u32(format + 4, rate);
    e.mem.set_u16(format + 0xe, 0x10);
    let channels = |e: &Engine, this: Ptr| match info(e, this) {
        0 => 1,
        info => e.mem.u32(info + 4),
    };
    let count = channels(e, this);
    let format = field(e, this, WAVE_FORMAT);
    e.mem.set_u16(format + 2, count as u16);
    let bits = i32::from(e.mem.u16(format + 0xe));
    let block_align = (bits / 8).wrapping_mul(i32::from(e.mem.u16(format + 2)));
    e.mem.set_u16(format + 0xc, block_align as u16);
    let average = u32::from(e.mem.u16(format + 0xc)).wrapping_mul(e.mem.u32(format + 4));
    e.mem.set_u32(format + 8, average);
    let count = channels(e, this);
    let reader = field(e, this, OGG_READER);
    let total = e.mem.u32(reader.wrapping_add(0x2d4));
    set_field(
        e,
        this,
        BUFFER_BYTES,
        total.wrapping_shl(1).wrapping_mul(count),
    );
    let bytes = field(e, this, BUFFER_BYTES);
    set_field(e, this, DATA_BYTES, bytes);
    let size = if preload {
        let format = field(e, this, WAVE_FORMAT);
        (e.mem.u32(format + 8) >> 1).wrapping_add(field(e, this, BUFFER_BYTES))
    } else {
        field(e, this, BUFFER_BYTES)
    };
    let data = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    set_field(e, this, SAMPLE_DATA, data);
    let data = field(e, this, SAMPLE_DATA);
    e.call(MEMSET, &args![data, 0u32, size]);
    let format = field(e, this, WAVE_FORMAT);
    let rate = e.mem.u16(format + 4);
    e.mem.set_u16(this.addr() + SAMPLES_PER_SECOND, rate);
    e.mem.set_u16(this.addr() + MOD_SAMPLES_PER_SECOND, rate);
    let count = channels(e, this);
    let per_frame = field(e, this, DATA_BYTES) / count.wrapping_shl(1);
    let per_millisecond = (i32::from(e.mem.u16(this.addr() + SAMPLES_PER_SECOND)) / 1000) as u32;
    set_field(e, this, DURATION, per_frame / per_millisecond);
    let reader = field(e, this, OGG_READER);
    let data = field(e, this, SAMPLE_DATA);
    let bytes = field(e, this, BUFFER_BYTES);
    fn_00af2100(e, Ptr::new(reader), data, bytes);
    if flags8(e, this) & 1 != 0 || channels(e, this) > 1 {
        set_field(e, this, BUFFER_FLAGS, 0x40);
        update_flags8(e, this, |flags| flags & 0xffff_fffd);
        update_flags8(e, this, |flags| flags | 1);
    } else {
        set_field(e, this, BUFFER_FLAGS, 0x30010);
    }
    let flags = field(e, this, BUFFER_FLAGS) | 0x2a0;
    set_field(e, this, BUFFER_FLAGS, flags);
    if preload {
        let format = field(e, this, WAVE_FORMAT);
        let bytes = (e.mem.u32(format + 8) >> 1).wrapping_add(field(e, this, BUFFER_BYTES));
        set_field(e, this, BUFFER_BYTES, bytes);
        set_field(e, this, DATA_BYTES, bytes);
        let duration = field(e, this, DURATION).wrapping_add(500);
        set_field(e, this, DURATION, duration);
    }
    set_opened(e, this, 1)
}

// Translated from 00af0400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the WinMM I/O procedure (`stdcall`: `info`, `message`, `first`,
/// `second`) over the game's file system; `MMIOINFO::adwInfo[0]` (`+0x30`) holds the file and
/// `lDiskOffset` (`+0x2c`) the position. Open (3) gets the file by name (`FileFinder::GetFile_ov2
/// (name, 0, 0x4000, 8)`; 0x101 when missing or not open, deleting it); close (4) deletes it
/// (0x104 and `"Audio: Could not close file."` without one); read (0) calls the file's read
/// function (`+8`: file, buffer, size, &1, 1) and advances both positions by its result, which
/// is answered (0x105 without a file); seek (2) seeks the file (vtable slot `0x14` with the
/// offset and the mode from the table at `010a2480`, 0 for `SEEK_SET`, 2 for `SEEK_END`, else
/// the middle entry; 0x107 without a file) and stores its position (`+0x38`, or `+0x150` when
/// that is -1). Write (1) and anything above 4 answer 0x10c.
pub fn fn_00af0400(e: &mut Engine, info: u32, message: u32, first: u32, second: u32) -> u32 {
    match message {
        3 => {
            let file = e
                .call(0x00af_df20, &args![first, 0u32, 0x4000u32, 8u32])
                .u32();
            if file == 0 || e.mem.u8(file + 0x2c) == 0 {
                if file != 0 {
                    e.vcall(file, 0, &args![1u32]);
                }
                return 0x101;
            }
            e.mem.set_u32(info + 0x2c, 0);
            e.mem.set_u32(info + 0x30, file);
            0
        }
        4 => {
            let file = e.mem.u32(info + 0x30);
            if file == 0 {
                e.call(PRINT, &args![0x010a_3eb0u32]);
                return 0x104;
            }
            e.vcall(file, 0, &args![1u32]);
            0
        }
        0 => {
            let file = e.mem.u32(info + 0x30);
            if file == 0 {
                return 0x105;
            }
            e.with_stack(4, |e, count| {
                e.mem.set_u32(count.addr(), 1);
                let read = e.mem.u32(file + 8);
                let result = e.call(read, &args![file, first, second, count, 1u32]).u32();
                let position = e.mem.u32(file + 4);
                e.mem.set_u32(file + 4, position.wrapping_add(result));
                let offset = e.mem.u32(info + 0x2c);
                e.mem.set_u32(info + 0x2c, offset.wrapping_add(result));
                result
            })
        }
        2 => {
            let mode = match second {
                0 => e.mem.u32(0x010a_2480),
                2 => e.mem.u32(0x010a_2488),
                _ => e.mem.u32(0x010a_2484),
            };
            let file = e.mem.u32(info + 0x30);
            if file == 0 {
                return 0x107;
            }
            e.vcall(file, 0x14, &args![first, mode]);
            let position = match e.mem.u32(file + 0x38) {
                u32::MAX => e.mem.u32(file + 0x150),
                position => position,
            };
            e.mem.set_u32(info + 0x2c, position);
            0
        }
        _ => 0x10c,
    }
}

// Translated from 00af1d20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: constructor of the Ogg Vorbis reader: clears `+0x2d4`, `+0x2d8`,
/// the `OggVorbis_File` (0x2d0 bytes, `memset`), the opened byte (`+0x2dc`) and the file
/// (`+0x2d0`). Returns `this`.
pub fn fn_00af1d20(e: &mut Engine, this: Ptr) -> Ptr {
    set_field(e, this, 0x2d4, 0);
    set_field(e, this, 0x2d8, 0);
    e.call(MEMSET, &args![this, 0u32, 0x2d0u32]);
    e.mem.set_u8(this.addr().wrapping_add(0x2dc), 0);
    set_field(e, this, 0x2d0, 0);
    this
}

// Translated from 00af1d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: destructor body of the Ogg Vorbis reader: an opened stream is
/// cleared (`ov_clear`, the import thunk `00ee81e6`); the file (`+0x2d0`) is deleted through
/// its vtable slot 0 with 1 and cleared.
pub fn fn_00af1d80(e: &mut Engine, this: Ptr) {
    audio_check::reader_destroyed(e, this.addr());
    if e.mem.u8(this.addr().wrapping_add(0x2dc)) != 0 {
        e.call(0x00ee_81e6, &args![this]);
    }
    if field(e, this, 0x2d0) != 0 {
        let file = field(e, this, 0x2d0);
        if file != 0 {
            audio_check::file_delete(e, this.addr(), file);
            e.vcall(file, 0, &args![1u32]);
        }
        set_field(e, this, 0x2d0, 0);
    }
}

// Translated from 00af1e00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af1e00 - the /GS cookie check (00ec408c) is not translated
/// Unnamed in the engine map (`RET 4`, `name`; answer in `AL`): opens an Ogg Vorbis stream. The
/// name is copied and lowered (`tolower` on its first 0x103 bytes); `.ogg` is looked for, else
/// `.mp3` or `.wav` is renamed to `.ogg` (neither: 0). The file (`FileFinder::GetFile_ov2(name,
/// 0, 0x4000, 0x10)`) must be open (`+0x2c`), else it is deleted and the answer is 0. The stream
/// opens through `ov_open_callbacks(file, this, 0, 0, {00af2180, 00af2200, 0, 00af2290})`
/// (`00ee81fe`); a failure closes (`BSFile::Close`), deletes and clears the file and answers 0.
/// Otherwise the stream is marked open, its info (`00ee81f8(this, -1)`) kept at `+0x2d8` and,
/// when `00ee81f2(this)` is non-zero, `00ee81ec(this, 0)` at `+0x2d4`. Answers 1.
pub fn fn_00af1e00(e: &mut Engine, this: Ptr, name: u32) -> u8 {
    e.with_stack(0x110, |e, frame| {
        let path = frame.addr();
        e.call(0x00ec_6370, &args![path, name]);
        let mut index = 0i32;
        while index < 0x103 {
            let byte = e.mem.u8(path + index as u32) as i8 as i32 as u32;
            let lowered = e.call(0x00ec_67aa, &args![byte]).u8();
            e.mem.set_u8(path + index as u32, lowered);
            index += 1;
        }
        if e.call(STRSTR, &args![path, OGG_010A3DBC]).u32() == 0 {
            let mut found = e.call(STRSTR, &args![path, MP3_01033AD8]).u32();
            if found == 0 {
                found = e.call(STRSTR, &args![path, WAV_0107CE2C]).u32();
            }
            if found == 0 {
                return 0;
            }
            e.mem.set_u8(found + 1, b'o');
            e.mem.set_u8(found + 2, b'g');
            e.mem.set_u8(found + 3, b'g');
        }
        let file = e
            .call(0x00af_df20, &args![path, 0u32, 0x4000u32, 0x10u32])
            .u32();
        set_field(e, this, 0x2d0, file);
        let file = field(e, this, 0x2d0);
        if file == 0 || e.mem.u8(file + 0x2c) == 0 {
            if field(e, this, 0x2d0) != 0 {
                let file = field(e, this, 0x2d0);
                if file != 0 {
                    e.vcall(file, 0, &args![1u32]);
                }
                set_field(e, this, 0x2d0, 0);
            }
            return 0;
        }
        let file = field(e, this, 0x2d0);
        let failed = e
            .call(
                0x00ee_81fe,
                &args![
                    file,
                    this,
                    0u32,
                    0u32,
                    0x00af_2180u32,
                    0x00af_2200u32,
                    0u32,
                    0x00af_2290u32
                ],
            )
            .u32();
        if failed != 0 {
            let file = field(e, this, 0x2d0);
            e.call(0x00af_fd10, &args![file]);
            let file = field(e, this, 0x2d0);
            if file != 0 {
                e.vcall(file, 0, &args![1u32]);
            }
            set_field(e, this, 0x2d0, 0);
            return 0;
        }
        audio_check::reader_opened(e, this.addr(), field(e, this, 0x2d0));
        e.mem.set_u8(this.addr().wrapping_add(0x2dc), 1);
        let info = e.call(0x00ee_81f8, &args![this, u32::MAX]).u32();
        set_field(e, this, 0x2d8, info);
        if e.call(0x00ee_81f2, &args![this]).u32() != 0 {
            let total = e.call(0x00ee_81ec, &args![this, 0u32]).u32();
            set_field(e, this, 0x2d4, total);
        }
        1
    })
}

// Translated from 00af2100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`RET 8`: `buffer`, `length`): decodes the Ogg Vorbis stream
/// (`this`) into `buffer` with `ov_read(this, at, length - done, 0, 2, 1, &section)` (`00ee8204`)
/// until more than `length` bytes are done; a read of 0 sets `length` to 0. Returns the bytes
/// done. (A stream that yields nothing at all loops for ever, as in the exe.)
pub fn fn_00af2100(e: &mut Engine, this: Ptr, buffer: u32, length: u32) -> u32 {
    let mut length = length;
    e.with_stack(4, |e, section| {
        let mut done = 0u32;
        let mut at = buffer;
        while done <= length {
            let read = e
                .call(
                    0x00ee_8204,
                    &args![
                        this,
                        at,
                        length.wrapping_sub(done),
                        0u32,
                        2u32,
                        1u32,
                        section
                    ],
                )
                .u32();
            done = done.wrapping_add(read);
            at = at.wrapping_add(read);
            if read == 0 {
                length = 0;
            }
        }
        done
    })
}

// Translated from 00af2180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the Ogg read callback (`cdecl`: `buffer`, `size`, `count`, `file`):
/// 0 without a file; otherwise the file's read function (`+8`: file, buffer, size * count, &1,
/// 1) runs, its position (`+4`) advances by the result, which is returned. (The exe reads the
/// file's open byte `+0x2c` and keeps the same file either way.)
pub fn fn_00af2180(e: &mut Engine, buffer: u32, size: u32, count: u32, file: u32) -> u32 {
    if file == 0 {
        return 0;
    }
    let _ = e.mem.u8(file + 0x2c);
    e.with_stack(4, |e, one| {
        e.mem.set_u32(one.addr(), 1);
        let read = e.mem.u32(file + 8);
        let result = e
            .call(
                read,
                &args![file, buffer, size.wrapping_mul(count), one, 1u32],
            )
            .u32();
        let position = e.mem.u32(file + 4);
        e.mem.set_u32(file + 4, position.wrapping_add(result));
        result
    })
}

// Translated from 00af2200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the Ogg seek callback (`cdecl`: `file`, a 64-bit offset (its high
/// word is not read), `whence`): -1 without a file; otherwise the file's vtable slot `0x14`
/// (seek) with the offset's low word and `whence`, and 0.
pub fn fn_00af2200(e: &mut Engine, file: u32, offset: u32, _unused_3: u32, whence: u32) -> u32 {
    if file == 0 {
        return u32::MAX;
    }
    e.vcall(file, 0x14, &args![offset, whence]);
    0
}

// Translated from 00af2290 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the Ogg tell callback (`cdecl`, `file`): 0 without a file, else the
/// file's position (`+0x38`, or `+0x150` when that is -1).
pub fn fn_00af2290(e: &mut Engine, file: u32) -> u32 {
    if file == 0 {
        return 0;
    }
    match e.mem.u32(file + 0x38) {
        u32::MAX => e.mem.u32(file + 0x150),
        position => position,
    }
}

/// The effect object (`+0x208`) and its parameters (`+0x1f0`, five `float`s).
const EFFECT: u32 = 0x208;
const EFFECT_PARAMETERS: u32 = 0x1f0;
const DISTANCE_TO_LISTENER: u32 = 0x150;

// Translated from 00aec530 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00aec530 - the /GS cookie check (00ec408c) is not translated
/// Vtable slot `0x28` (`Prepare`). Without a device the prepare fails (`00ae9c50(0)`). A 3D sound
/// gets distance attenuation 10000. With sample data: a streaming sound splits it at its loop
/// points into a streaming state (0x24 bytes: intro, loop and tail blocks of `loop start * bps`,
/// the rest, and `size - loop end * bps` bytes plus 0x400 zero bytes) and plays from a buffer of
/// half a second; a short sound (below 150 ms) with bit 0x10 or streaming drops buffer flag
/// 0x200. The buffer is made (`IDirectSound8::CreateSoundBuffer`, slot `0xc`; a failure prints
/// the 3D multi-channel or "too small" message (`0x887800b4`), frees the data and fails), the
/// `IDirectSoundBuffer8` queried (IID at `01100f94`) and, for a 3D sound, its 3D buffer (IID at
/// `01100fb4`) set up (slot `0x44` with the `float` at `010a3dcc`, the position from the
/// `float` at `010a3dc8` through vtable slot `0x50`, the distance `FLT_MAX`, the created buffer's
/// volume -10000). The buffer is locked and filled (the stream's intro, state 1 or 2, or all the
/// data), unlocked, the data freed and the volume applied (vtable slot `0x3c`). A sound of 150 ms
/// or more runs `00af1430`. Type bit 0x80000 adds an effect (`SetFX`, `GetObjectInPath`) whose
/// parameters are the `float` at `011f7158`, 0, and the `float`s at `01030020`, `010a3dc4`,
/// `01030020`. `BSGameSound::Prepare` (`00ae9de0`) runs last.
///
/// Answers `AL` of the call it ends on (`Prepare` is `bool` in the Xbox PDB): `00ae9c50(0)`'s
/// on each failure, `BSGameSound::Prepare`'s otherwise.
pub fn fn_00aec530(e: &mut Engine, this: Ptr) -> bool {
    if field(e, this, DIRECT_SOUND) == 0 {
        return set_opened(e, this, 0) as u8 != 0;
    }
    let format = field(e, this, WAVE_FORMAT);
    let bytes_per_sample = (i32::from(e.mem.u16(format + 0xe)) / 8) as u32;
    if e.call(IS_3D, &args![this]).bool() || e.call(IS_3D_OTHER, &args![this]).bool() {
        e.mem.set_u16(this.addr() + DISTANCE_ATTENUATION, 10000);
    }
    if field(e, this, SAMPLE_DATA) != 0 {
        if let Err(answer) = prepare_buffer(e, this, bytes_per_sample) {
            return answer;
        }
    }
    if field(e, this, DURATION) as i32 >= 0x96 {
        fn_00af1430(e, this);
    }
    if flags8(e, this) & 0x8_0000 != 0 {
        prepare_effect(e, this);
    }
    e.call(0x00ae_9de0, &args![this]).bool()
}

/// The buffer part of `Prepare` (`00aec530`); when the prepare failed, the `AL` of the
/// `00ae9c50(0)` it ends on.
fn prepare_buffer(e: &mut Engine, this: Ptr, bytes_per_sample: u32) -> Result<(), bool> {
    if is_streaming(e, this) {
        prepare_stream(e, this, bytes_per_sample);
    }
    if (field(e, this, DURATION) as i32) < 0x96
        && (flags8(e, this) & 0x10 != 0 || is_streaming(e, this))
    {
        let flags = field(e, this, BUFFER_FLAGS) & 0xffff_fdff;
        set_field(e, this, BUFFER_FLAGS, flags);
    }
    let base = this.addr();
    let created = com(
        e,
        this,
        DIRECT_SOUND,
        0xc,
        &[base + BUFFER_DESC, base + CREATED_BUFFER, 0],
    );
    if (created as i32) < 0 {
        let format = field(e, this, WAVE_FORMAT);
        let channels = e.mem.u16(format + 2);
        if flags8(e, this) & 2 != 0 && i32::from(channels) > 1 {
            e.call(
                PRINT,
                &args![0x010a_3e10u32, u32::from(channels), base + FILENAME],
            );
        } else if created == 0x8878_00b4 {
            e.call(PRINT, &args![0x010a_3dd0u32, base + FILENAME]);
        }
        if field(e, this, SAMPLE_DATA) != 0 {
            let data = field(e, this, SAMPLE_DATA);
            e.call(FREE, &args![MEMORY_MANAGER, data]);
            set_field(e, this, SAMPLE_DATA, 0);
        }
        return Err(set_opened(e, this, 0) as u8 != 0);
    }
    com(e, this, CREATED_BUFFER, 0, &[0x0110_0f94, base + BUFFER]);
    if field(e, this, BUFFER) != 0 && flags8(e, this) & 2 != 0 {
        com(e, this, BUFFER, 0, &[0x0110_0fb4, base + BUFFER_3D]);
        let distance = x87_quiet_bits(e.mem.u32(0x010a_3dcc));
        com(e, this, BUFFER_3D, 0x44, &[distance, 1]);
        let coordinate = x87_quiet_bits(e.mem.u32(0x010a_3dc8));
        e.vcall(base, 0x50, &args![coordinate, coordinate, coordinate]);
        let far = x87_quiet_bits(e.mem.u32(0x0101_6970));
        set_field(e, this, DISTANCE_TO_LISTENER, far);
        com(e, this, CREATED_BUFFER, 0x3c, &[(-10000i32) as u32]);
    }
    let locked = e.with_stack(8, |e, locals| {
        let region = locals.addr();
        let region_bytes = region + 4;
        let bytes = field(e, this, DATA_BYTES);
        let result = com(
            e,
            this,
            BUFFER,
            0x2c,
            &[0, bytes, region, region_bytes, 0, 0, 0],
        );
        if (result as i32) < 0 {
            return false;
        }
        let target = e.mem.u32(region);
        let stream = field(e, this, STREAM);
        if is_streaming(e, this) && stream != 0 && e.mem.u32(stream + 0x18) != 0 {
            let intro = e.mem.u32(stream + 8);
            if intro > field(e, this, DATA_BYTES) {
                let bytes = field(e, this, DATA_BYTES);
                let source = e.mem.u32(stream + 0x18);
                e.call(MEMCPY, &args![target, source, bytes]);
                let bytes = field(e, this, DATA_BYTES);
                let stream = field(e, this, STREAM);
                e.mem.set_u32(stream + 4, bytes);
                e.mem.set_u32(stream, 1);
            } else {
                let source = e.mem.u32(stream + 0x18);
                e.call(MEMCPY, &args![target, source, intro]);
                let stream = field(e, this, STREAM);
                e.mem.set_u32(stream + 4, 0);
                let intro = e.mem.u32(stream + 8);
                e.mem.set_u32(stream + 0x14, intro);
                e.mem.set_u32(stream, 2);
            }
        } else {
            let bytes = field(e, this, DATA_BYTES);
            let data = field(e, this, SAMPLE_DATA);
            e.call(MEMCPY, &args![target, data, bytes]);
        }
        let pointer = e.mem.u32(region);
        let length = e.mem.u32(region_bytes);
        com(e, this, BUFFER, 0x4c, &[pointer, length, 0, 0]);
        true
    });
    if !locked {
        return Err(set_opened(e, this, 0) as u8 != 0);
    }
    if field(e, this, SAMPLE_DATA) != 0 {
        let data = field(e, this, SAMPLE_DATA);
        e.call(FREE, &args![MEMORY_MANAGER, data]);
        set_field(e, this, SAMPLE_DATA, 0);
    }
    let volume = x87_quiet_bits(field(e, this, CURRENT_VOLUME));
    e.vcall(base, 0x3c, &args![volume]);
    Ok(())
}

/// The streaming split of `Prepare` (`00aec530`).
fn prepare_stream(e: &mut Engine, this: Ptr, bytes_per_sample: u32) {
    let extra = 0x400u32;
    let stream = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x24u32]).u32();
    set_field(e, this, STREAM, stream);
    let intro = field(e, this, LOOP_START).wrapping_mul(bytes_per_sample);
    e.mem.set_u32(stream + 8, intro);
    let tail = field(e, this, DATA_BYTES)
        .wrapping_sub(field(e, this, LOOP_END).wrapping_mul(bytes_per_sample));
    e.mem.set_u32(stream + 0x10, tail);
    let middle = field(e, this, DATA_BYTES)
        .wrapping_sub(e.mem.u32(stream + 8).wrapping_add(e.mem.u32(stream + 0x10)));
    e.mem.set_u32(stream + 0xc, middle);
    let format = field(e, this, WAVE_FORMAT);
    let half_second = e.mem.u32(format + 8) >> 1;
    set_field(e, this, DATA_BYTES, half_second);
    set_field(e, this, BUFFER_BYTES, half_second);
    e.mem.set_u32(stream + 0x14, 0);
    e.mem.set_u32(stream, 0);
    e.mem.set_u32(stream + 4, 0);
    for (block, size) in [(0x18u32, 8u32), (0x1c, 0xc)] {
        let bytes = e.mem.u32(stream + size);
        let memory = e.call(ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
        e.mem.set_u32(stream + block, memory);
    }
    let bytes = e.mem.u32(stream + 0x10).wrapping_add(extra);
    let memory = e.call(ALLOCATE, &args![MEMORY_MANAGER, bytes]).u32();
    e.mem.set_u32(stream + 0x20, memory);
    let mut source = field(e, this, SAMPLE_DATA);
    for (block, size) in [(0x18u32, 8u32), (0x1c, 0xc), (0x20, 0x10)] {
        let bytes = e.mem.u32(stream + size);
        let target = e.mem.u32(stream + block);
        e.call(MEMCPY, &args![target, source, bytes]);
        source = source.wrapping_add(e.mem.u32(stream + size));
    }
    let tail_end = e
        .mem
        .u32(stream + 0x20)
        .wrapping_add(e.mem.u32(stream + 0x10));
    e.call(MEMSET, &args![tail_end, 0u32, extra]);
    let tail = e.mem.u32(stream + 0x10).wrapping_add(extra);
    e.mem.set_u32(stream + 0x10, tail);
}

/// The effect part of `Prepare` (`00aec530`).
fn prepare_effect(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.with_stack(0x28, |e, locals| {
        let description = locals.addr();
        let results = description + 0x20;
        e.mem.set_u32(description, 0x20);
        e.mem.set_u32(description + 4, 0);
        for word in 0..4 {
            let value = e.mem.u32(0x0110_0ff4 + 4 * word);
            e.mem.set_u32(description + 8 + 4 * word, value);
        }
        e.mem.set_u32(description + 0x18, 0);
        e.mem.set_u32(description + 0x1c, 0);
        com(e, this, BUFFER, 0x54, &[1, description, results]);
    });
    let found = com(
        e,
        this,
        BUFFER,
        0x5c,
        &[0x0110_0ff4, 0, 0x0110_0fc4, base + EFFECT],
    );
    if (found as i32) < 0 {
        return;
    }
    let source = if 0x011f_7154u32 != 0 {
        0x011f_7158
    } else {
        // Not reached: the tested address is a constant that is never 0.
        e.mem.set_u32(0x0120_2800, 0);
        0x0120_2800
    };
    let first = x87_quiet_bits(e.mem.u32(source));
    set_field(e, this, EFFECT_PARAMETERS, first);
    set_field(e, this, EFFECT_PARAMETERS + 4, 0);
    let wide = x87_quiet_bits(e.mem.u32(0x0103_0020));
    set_field(e, this, EFFECT_PARAMETERS + 8, wide);
    let middle = x87_quiet_bits(e.mem.u32(0x010a_3dc4));
    set_field(e, this, EFFECT_PARAMETERS + 0xc, middle);
    let wide = x87_quiet_bits(e.mem.u32(0x0103_0020));
    set_field(e, this, EFFECT_PARAMETERS + 0x10, wide);
    com(e, this, EFFECT, 0xc, &[base + EFFECT_PARAMETERS]);
}

/// The reverb buffer's effect object (`+0x1e0`).
const REVERB_EFFECT: u32 = 0x1e0;
const ENVIRONMENT_TYPE: u32 = 0x154;
/// The count of second buffers (`011f7128`) and its limit (`011f7140`), and the byte that
/// allows them (`011f7134`).
const SECOND_BUFFER_COUNT_011F7128: u32 = 0x011f_7128;

// Translated from 00af1430 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af1430 - the /GS cookie check (00ec408c) is not translated
/// Unnamed in the engine map (answer in `AL`): adds the second (reverb) buffer. Only for a sound
/// without type bit 0x20 or streaming, when the byte at `011f7134` allows it, with a device and
/// a buffer, when the audio manager's environment (`+0x10`, vtable slot `0x2c`) is non-zero and
/// fewer second buffers than the limit at `011f7140` exist. A buffer of the same format (read
/// through `GetFormat` into a block from the memory manager) and a quarter longer
/// (`DSBUFFERDESC` flags 0x40 or 0x30010, plus 0x2a0) is made (`CreateSoundBuffer`; the count at
/// `011f7128` goes up first), its `IDirectSoundBuffer8` (`+0x1a0`) and 3D buffer (`+0x1d8`)
/// queried; an effect (GUID at `01101004`) is set and its object (`+0x1e0`) told the environment
/// minus 1 (the audio manager's, else `+0x154`; slot `0x14`). Both buffers are locked whole
/// (`DSBLOCK_ENTIREBUFFER`), the new one cleared and given the sound's bytes, and unlocked.
/// Answers 1 when the new buffer was locked; failures free the format block and answer 0.
pub fn fn_00af1430(e: &mut Engine, this: Ptr) -> u8 {
    if flags8(e, this) & 0x20 != 0 {
        return 0;
    }
    let allowed = if 0x011f_7130u32 != 0 {
        0x011f_7134
    } else {
        // Not reached: the tested address is a constant that is never 0.
        e.mem.set_u8(0x0120_2800, 0);
        0x0120_2800
    };
    if e.mem.u8(allowed) == 0 || field(e, this, DIRECT_SOUND) == 0 || field(e, this, BUFFER) == 0 {
        return 0;
    }
    let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
    if e.mem.u32(manager.wrapping_add(0x10)) == 0 {
        return 0;
    }
    let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
    let environment = e.mem.u32(manager.wrapping_add(0x10));
    if e.vcall(environment, 0x2c, &args![]).u32() == 0 || is_streaming(e, this) {
        return 0;
    }
    let limit = if 0x011f_713cu32 != 0 {
        0x011f_7140
    } else {
        // Not reached: the tested address is a constant that is never 0.
        e.mem.set_u32(0x0120_2800, 0);
        0x0120_2800
    };
    if e.mem.u32(limit) <= e.mem.u32(SECOND_BUFFER_COUNT_011F7128) {
        return 0;
    }
    e.with_stack(0x80, |e, frame| {
        let at = |offset: u32| frame.addr() + 0x80 - offset;
        second_buffer(e, this, &at)
    })
}

/// The buffer part of `00af1430`.
fn second_buffer(e: &mut Engine, this: Ptr, at: &dyn Fn(u32) -> u32) -> u8 {
    let base = this.addr();
    let description = at(0x38);
    e.call(MEMSET, &args![description, 0u32, 0x24u32]);
    e.mem.set_u32(description, 0x24);
    let flags = if flags8(e, this) & 1 != 0 {
        0x40
    } else {
        0x30010
    };
    e.mem.set_u32(description + 4, flags | 0x2a0);
    let bytes = field(e, this, DATA_BYTES);
    let quarter = x87_mul(f64::from(bytes), double_at(e, 0x0102_90b0));
    let longer =
        (x87_fistp64_chop(X87Ext::load64(quarter)) as u32).wrapping_add(field(e, this, DATA_BYTES));
    e.mem.set_u32(description + 8, longer);
    for word in 0..4 {
        let value = e.mem.u32(GUID_NULL_01093F04 + 4 * word);
        e.mem.set_u32(description + 0x14 + 4 * word, value);
    }
    let format = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x12u32]).u32();
    e.mem.set_u32(description + 0x10, format);
    let new_lock = at(0x68);
    let new_bytes = at(0x3c);
    let new_second = at(0x60);
    let new_second_bytes = at(0x6c);
    com(e, this, BUFFER, 0x14, &[format, 0x12, new_bytes]);
    let count = e.mem.u32(SECOND_BUFFER_COUNT_011F7128);
    e.mem
        .set_u32(SECOND_BUFFER_COUNT_011F7128, count.wrapping_add(1));
    if (com(
        e,
        this,
        DIRECT_SOUND,
        0xc,
        &[description, base + BUFFER_1D0, 0],
    ) as i32)
        < 0
    {
        e.call(FREE, &args![MEMORY_MANAGER, format]);
        return 0;
    }
    com(e, this, BUFFER_1D0, 0, &[0x0110_0f94, base + SECOND_BUFFER]);
    if field(e, this, SECOND_BUFFER) == 0 {
        e.call(FREE, &args![MEMORY_MANAGER, format]);
        return 0;
    }
    com(e, this, SECOND_BUFFER, 0, &[0x0110_0fb4, base + BUFFER_1D8]);
    let effect = at(0x5c);
    e.mem.set_u32(effect, 0x20);
    e.mem.set_u32(effect + 4, 0);
    for word in 0..4 {
        let value = e.mem.u32(0x0110_1004 + 4 * word);
        e.mem.set_u32(effect + 8 + 4 * word, value);
    }
    e.mem.set_u32(effect + 0x18, 0);
    e.mem.set_u32(effect + 0x1c, 0);
    com(e, this, SECOND_BUFFER, 0x54, &[1, effect, 0]);
    let found = com(
        e,
        this,
        SECOND_BUFFER,
        0x5c,
        &[0x0110_1004, 0, 0x0110_0fd4, base + REVERB_EFFECT],
    );
    if found as i32 >= 0 {
        let mut environment = field(e, this, ENVIRONMENT_TYPE);
        let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
        if e.mem.u32(manager.wrapping_add(0x10)) != 0 {
            let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
            let object = e.mem.u32(manager.wrapping_add(0x10));
            environment = e.vcall(object, 0x2c, &args![]).u32();
        }
        if environment != 0 {
            com(e, this, REVERB_EFFECT, 0x14, &[environment.wrapping_sub(1)]);
        }
    }
    e.mem.set_u32(new_bytes, 0);
    let locked = com(
        e,
        this,
        SECOND_BUFFER,
        0x2c,
        &[0, 0, new_lock, new_bytes, new_second, new_second_bytes, 2],
    );
    if (locked as i32) < 0 {
        e.call(FREE, &args![MEMORY_MANAGER, format]);
        return 0;
    }
    let old_lock = at(0x10);
    let old_bytes = at(4);
    let old_second = at(8);
    let old_second_bytes = at(0xc);
    let source = com(
        e,
        this,
        BUFFER,
        0x2c,
        &[0, 0, old_lock, old_bytes, old_second, old_second_bytes, 2],
    );
    if source as i32 >= 0 {
        let target = e.mem.u32(new_lock);
        e.call(MEMSET, &args![target, 0u32, longer]);
        let bytes = field(e, this, BUFFER_BYTES);
        let from = e.mem.u32(old_lock);
        let target = e.mem.u32(new_lock);
        e.call(MEMCPY, &args![target, from, bytes]);
        let words = [
            e.mem.u32(old_lock),
            e.mem.u32(old_bytes),
            e.mem.u32(old_second),
            e.mem.u32(old_second_bytes),
        ];
        com(e, this, BUFFER, 0x4c, &words);
    }
    let words = [
        e.mem.u32(new_lock),
        e.mem.u32(new_bytes),
        e.mem.u32(new_second),
        e.mem.u32(new_second_bytes),
    ];
    com(e, this, SECOND_BUFFER, 0x4c, &words);
    e.call(FREE, &args![MEMORY_MANAGER, format]);
    1
}

/// `NiPoint3` difference (`00439ef0`, `thiscall` on the left point, `RET 8`: result, right
/// point), length (`00457990`) and dot product (`NiPoint3::Dot`, `004b6190`, `RET 4`).
const POINT_SUBTRACT: u32 = 0x0043_9ef0;
const POINT_LENGTH: u32 = 0x0045_7990;
const POINT_DOT: u32 = 0x004b_6190;
/// `BSGameSound::vBeamEnd` (Xbox PDB) `+0x17c`.
const BEAM_END: u32 = 0x17c;
/// The scale from game units to DirectSound's (`double` at `010a3be8`).
const UNIT_SCALE_010A3BE8: u32 = 0x010a_3be8;

/// `IDirectSound3DBuffer::SetPosition` (slot `0x4c`) on the buffer at `this + offset` with the
/// point's `x`, `z`, `y` scaled (each stored as a `float`) and `DS3D_DEFERRED` (1).
fn set_3d_position(e: &mut Engine, this: Ptr, offset: u32, point: [u32; 3]) {
    let scale = double_at(e, UNIT_SCALE_010A3BE8);
    let scaled = |value: u32| x87_store32(x87_mul(x87_load32(value), scale));
    let (y, z, x) = (scaled(point[1]), scaled(point[2]), scaled(point[0]));
    com(e, this, offset, 0x4c, &[x, z, y, 1]);
}

/// The listener object's position (`[[audio manager] + 0x10]`, vtable slot `0xc`), as three
/// words.
fn listener_position(e: &mut Engine) -> [u32; 3] {
    let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
    let listener = e.mem.u32(manager.wrapping_add(0x10));
    let point = e.vcall(listener, 0xc, &args![]).u32();
    [e.mem.u32(point), e.mem.u32(point + 4), e.mem.u32(point + 8)]
}

fn listener_present(e: &Engine) -> bool {
    let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
    e.mem.u32(manager.wrapping_add(0x10)) != 0
}

/// Whether the sound follows a beam (type bit 0x10000000 and state bit 0x8000).
fn is_beam(e: &Engine, this: Ptr) -> bool {
    flags8(e, this) & 0x1000_0000 != 0 && field(e, this, STATE_FLAGS) & 0x8000 != 0
}

// Translated from 00aef090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x50` (`SetEmitterPosition`, `RET 0xc`: `x`, `y`, `z`). Only for a 3D sound with
/// a device. With a listener (the audio manager's `+0x10`), a sound with type bit 0x40000 takes
/// the listener's position; a position different from the stored one (`+0x218`, `FUCOM`) is
/// stored. The sound is marked placed. Its 3D buffers get the position (scaled, axes `x`, `z`,
/// `y`) when it moved and it is not a beam; a beam sound (type bit 0x10000000, state bit 0x8000)
/// with a listener gets the closest point of its beam to the listener instead (the position
/// when the listener is behind it, the beam end when past it, else the projection through
/// `004a3760` and `00439e90`). The distance to the listener (`+0x150`) is updated (a beam
/// measures from that point), and the distance attenuation (`+0x1e`) set: 10000 inside
/// `fMaxDist`; when the range is over 10, interpolated over the attenuation curve between the
/// quarter points of `[fMinDist, fMaxDist]` (truncated), else 0. The volume is applied (vtable
/// slot `0x3c`) after setting it and once more at the end.
pub fn fn_00aef090(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32) {
    let default = [
        e.mem.u32(0x011f_426c),
        e.mem.u32(0x011f_4270),
        e.mem.u32(0x011f_4274),
    ];
    let mut point = default;
    let mut listener = default;
    let mut changed = false;
    if !(e.call(IS_3D, &args![this]).bool() || e.call(IS_3D_OTHER, &args![this]).bool()) {
        return;
    }
    if field(e, this, DIRECT_SOUND) == 0 {
        return;
    }
    let mut position = [x.to_bits(), y.to_bits(), z.to_bits()];
    if listener_present(e) {
        listener = listener_position(e);
        if flags8(e, this) & 0x4_0000 != 0 {
            position = [
                x87_quiet_bits(listener[0]),
                x87_quiet_bits(listener[1]),
                x87_quiet_bits(listener[2]),
            ];
        }
        let differs = (0..3).any(|axis| {
            let stored = field(e, this, POSITION + 4 * axis as u32);
            parity_even(fpu_compare(x87_load32(position[axis]), x87_load32(stored)) & 0x44)
        });
        if differs {
            for (axis, value) in position.iter().enumerate() {
                set_field(e, this, POSITION + 4 * axis as u32, x87_quiet_bits(*value));
            }
            changed = true;
        }
    }
    e.mem.set_u8(this.addr() + POSITION_SET, 1);
    if field(e, this, BUFFER_3D) != 0 {
        if !is_beam(e, this) && changed {
            set_3d_position(e, this, BUFFER_3D, position);
            if field(e, this, BUFFER_1D8) != 0 {
                set_3d_position(e, this, BUFFER_1D8, position);
            }
        } else if listener_present(e) && is_beam(e, this) {
            point = beam_point(e, this, point);
            set_3d_position(e, this, BUFFER_3D, point);
            if field(e, this, BUFFER_1D8) != 0 {
                set_3d_position(e, this, BUFFER_1D8, point);
            }
        }
    }
    let distance = e.with_stack(0x20, |e, locals| {
        let start = locals.addr();
        let result = start + 0xc;
        for (axis, value) in listener.iter().enumerate() {
            e.mem.set_u32(start + 4 * axis as u32, *value);
        }
        let difference = e
            .call(
                POINT_SUBTRACT,
                &args![start, result, this.addr() + POSITION],
            )
            .u32();
        let length = x87_store32(e.call(POINT_LENGTH, &args![difference]).f64());
        set_field(e, this, DISTANCE_TO_LISTENER, x87_quiet_bits(length));
        let mut distance = x87_quiet_bits(field(e, this, DISTANCE_TO_LISTENER));
        if is_beam(e, this) && listener_present(e) {
            let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
            let object = e.mem.u32(manager.wrapping_add(0x10));
            let heard = e.vcall(object, 0xc, &args![]).u32();
            for (axis, value) in point.iter().enumerate() {
                e.mem.set_u32(start + 4 * axis as u32, *value);
            }
            let difference = e.call(POINT_SUBTRACT, &args![start, result, heard]).u32();
            distance = x87_store32(e.call(POINT_LENGTH, &args![difference]).f64());
        }
        distance
    });
    set_distance_attenuation(e, this, distance, false);
    let volume = x87_quiet_bits(field(e, this, CURRENT_VOLUME));
    e.vcall(this.addr(), 0x3c, &args![volume]);
}

/// The point of the beam (`+0x218` to `vBeamEnd`) closest to the listener, as `SetEmitterPosition`
/// (`00aef090`) finds it.
fn beam_point(e: &mut Engine, this: Ptr, point: [u32; 3]) -> [u32; 3] {
    let mut point = point;
    let base = this.addr();
    e.with_stack(0x48, |e, locals| {
        let heard = locals.addr();
        let direction = heard + 0xc;
        let relative = heard + 0x18;
        let scratch = heard + 0x24;
        let product = heard + 0x30;
        let projected = heard + 0x3c;
        let listener = listener_position(e);
        for (axis, value) in listener.iter().enumerate() {
            e.mem.set_u32(heard + 4 * axis as u32, *value);
        }
        let result = e
            .call(
                POINT_SUBTRACT,
                &args![base + BEAM_END, scratch, base + POSITION],
            )
            .u32();
        for axis in 0..3 {
            let value = e.mem.u32(result + 4 * axis);
            e.mem.set_u32(direction + 4 * axis, value);
        }
        let result = e
            .call(POINT_SUBTRACT, &args![heard, scratch, base + POSITION])
            .u32();
        for axis in 0..3 {
            let value = e.mem.u32(result + 4 * axis);
            e.mem.set_u32(relative + 4 * axis, value);
        }
        let along = x87_store32(e.call(POINT_DOT, &args![relative, direction]).f64());
        if !parity_even(fpu_compare(x87_load32(along), double_at(e, ZERO_01012060)) & 0x41) {
            point = [
                field(e, this, POSITION),
                field(e, this, POSITION + 4),
                field(e, this, POSITION + 8),
            ];
            return;
        }
        let length = x87_store32(e.call(POINT_DOT, &args![direction, direction]).f64());
        if fpu_compare(x87_load32(along), x87_load32(length)) & 0x01 == 0 {
            point = [
                field(e, this, BEAM_END),
                field(e, this, BEAM_END + 4),
                field(e, this, BEAM_END + 8),
            ];
            return;
        }
        let fraction = x87_store32(x87_div(x87_load32(along), x87_m32(length)));
        let scaled = e
            .call(
                0x004a_3760,
                &args![product, x87_quiet_bits(fraction), direction],
            )
            .u32();
        let sum = e
            .call(0x0043_9e90, &args![base + POSITION, projected, scaled])
            .u32();
        point = [e.mem.u32(sum), e.mem.u32(sum + 4), e.mem.u32(sum + 8)];
    });
    point
}

/// Stores the distance attenuation `value` (`+0x1e`) and applies the volume (vtable slot `0x3c`).
fn set_distance_attenuation_to(e: &mut Engine, this: Ptr, value: u16) {
    e.mem.set_u16(this.addr() + DISTANCE_ATTENUATION, value);
    let volume = x87_quiet_bits(field(e, this, CURRENT_VOLUME));
    e.vcall(this.addr(), 0x3c, &args![volume]);
}

/// The attenuation of `SetEmitterPosition` (`00aef090`) and `UpdateEmitterPosition` (`00aed990`)
/// for the listener at `distance`; `near_only` (the latter) also requires the listener to be
/// nearer than 1.1 times `fMaxDist` (the `double` at `010290f8`) for the curve.
fn set_distance_attenuation(e: &mut Engine, this: Ptr, distance: u32, near_only: bool) {
    let max = field(e, this, MAX_DISTANCE);
    let min = field(e, this, MIN_DISTANCE);
    let range = x87_store32(x87_sub(x87_m32(max), x87_load32(min)));
    let absolute = x87_store32(e.call(0x00ec_6cde, &args![x87_load32(range)]).f64());
    let d = x87_load32(distance);
    let inside = parity_even(fpu_compare(x87_load32(max), d) & 0x41);
    let far = near_only && {
        let limit = x87_mul(x87_load32(max), double_at(e, 0x0102_90f8));
        parity_even(fpu_compare(d, limit) & 0x05)
    };
    if fpu_compare(x87_load32(absolute), double_at(e, 0x0102_0758)) & 0x41 != 0 || far {
        set_distance_attenuation_to(e, this, if inside { 0 } else { 10000 });
        return;
    }
    if !inside {
        set_distance_attenuation_to(e, this, 10000);
        return;
    }
    let quarter = |e: &Engine, factor: u32| {
        let span = x87_sub(x87_load32(max), x87_m32(min));
        x87_store32(x87_add(x87_mul(span, double_at(e, factor)), x87_m32(min)))
    };
    let first = quarter(e, 0x0102_90b0);
    let second = quarter(e, 0x0101_1588);
    let third = quarter(e, 0x0101_de30);
    let zero = double_at(e, ZERO_01012060);
    let base = this.addr();
    // From the top: the band (lower, upper) and the curve words it interpolates between.
    let bands = [
        (third, max, 0x212u32, 0x214u32),
        (second, third, 0x210, 0x212),
        (first, second, 0x20e, 0x210),
        (min, first, 0x20c, 0x20e),
    ];
    for (lower, upper, from, to) in bands {
        let above = !parity_even(fpu_compare(x87_load32(lower), d) & 0x05);
        if !above {
            continue;
        }
        let width = x87_sub(x87_load32(upper), x87_m32(lower));
        if fpu_compare(width, zero) & 0x41 != 0 {
            continue;
        }
        let span = x87_sub(x87_m32(upper), x87_load32(lower));
        let offset = x87_sub(x87_m32(distance), x87_load32(lower));
        let ratio = x87_div(offset, span);
        let start = i32::from(e.mem.u16(base + from));
        let difference = i32::from(e.mem.u16(base + to)) - start;
        let value = x87_add(x87_mul(ratio, f64::from(difference)), f64::from(start));
        set_distance_attenuation_to(e, this, fistp_chop(value) as u16);
        return;
    }
    set_distance_attenuation_to(e, this, 0);
}

// Translated from 00aed990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x40` (`UpdateEmitterPosition`). Only for a 3D sound with a listener (the audio
/// manager's `+0x10`). A sound with type bit 0x40000 puts its 3D buffers at the listener's
/// position; a beam sound (type bit 0x10000000, state bit 0x8000) at the point of its beam
/// closest to the listener. The distance to the listener (`+0x150`) is measured again (from that
/// point for a beam, else from the position `+0x218`) and the distance attenuation set as in
/// `SetEmitterPosition`, the curve only applying while the listener is nearer than 1.1 times
/// `fMaxDist`. The volume is applied after setting it and once more at the end.
pub fn fn_00aed990(e: &mut Engine, this: Ptr) {
    let mut point = [
        e.mem.u32(0x011f_426c),
        e.mem.u32(0x011f_4270),
        e.mem.u32(0x011f_4274),
    ];
    if !(e.call(IS_3D, &args![this]).bool() || e.call(IS_3D_OTHER, &args![this]).bool()) {
        return;
    }
    if !listener_present(e) {
        return;
    }
    if flags8(e, this) & 0x4_0000 != 0 {
        let listener = listener_position(e);
        let position = [
            x87_quiet_bits(listener[0]),
            x87_quiet_bits(listener[1]),
            x87_quiet_bits(listener[2]),
        ];
        if field(e, this, BUFFER_3D) != 0 {
            set_3d_position(e, this, BUFFER_3D, position);
            if field(e, this, BUFFER_1D8) != 0 {
                set_3d_position(e, this, BUFFER_1D8, position);
            }
        }
    } else if is_beam(e, this) {
        point = beam_point(e, this, point);
        if field(e, this, BUFFER_3D) != 0 {
            set_3d_position(e, this, BUFFER_3D, point);
            if field(e, this, BUFFER_1D8) != 0 {
                set_3d_position(e, this, BUFFER_1D8, point);
            }
        }
    }
    let mut distance = x87_quiet_bits(field(e, this, DISTANCE_TO_LISTENER));
    let beam = is_beam(e, this) && listener_present(e);
    if beam || listener_present(e) {
        distance = e.with_stack(0x18, |e, locals| {
            let start = locals.addr();
            let result = start + 0xc;
            let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
            let object = e.mem.u32(manager.wrapping_add(0x10));
            let heard = e.vcall(object, 0xc, &args![]).u32();
            let from = if beam {
                for (axis, value) in point.iter().enumerate() {
                    e.mem.set_u32(start + 4 * axis as u32, *value);
                }
                start
            } else {
                this.addr() + POSITION
            };
            let difference = e.call(POINT_SUBTRACT, &args![from, result, heard]).u32();
            x87_store32(e.call(POINT_LENGTH, &args![difference]).f64())
        });
        set_field(e, this, DISTANCE_TO_LISTENER, x87_quiet_bits(distance));
    }
    set_distance_attenuation(e, this, distance, true);
    let volume = x87_quiet_bits(field(e, this, CURRENT_VOLUME));
    e.vcall(this.addr(), 0x3c, &args![volume]);
}

/// `BSGameSound::iSynchStartTime` (Xbox PDB) `+0x15c` and `iCleanupDelayInMS` `+0x160`.
const SYNCH_START_TIME: u32 = 0x15c;
const CLEANUP_DELAY: u32 = 0x160;

/// `00af1c00(this, written, cursor, length, block)` with the stream's written count (`+4`) and
/// cursor (`+0x14`).
fn stream_fill(e: &mut Engine, this: Ptr, length: u32, block: u32) {
    let stream = field(e, this, STREAM);
    let written = e.mem.u32(stream + 4);
    let at = e.mem.u32(stream + 0x14);
    fn_00af1c00(e, this, written, at, length, block);
}

// Translated from 00aee580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Vtable slot `0x44` (`Update`, `RET 4`, the word is not read; answer in `AL`). With a buffer
/// and a device: the buffer's status (`GetStatus`, slot `0x24`) sets or clears state bit 0x20.
/// A playing streaming sound feeds its buffer from the play cursor (`GetCurrentPosition`, slot
/// `0x10`; an unmoved cursor answers 0): state 1 copies the intro until it is done (then state
/// 2), state 2 the loop (wrapping), state 4 restarts from the tail (state 6, or 5 when the tail
/// fits; answer 0) for type bit 0x2000000 or stops into state 3 for 0x4000000, state 5 copies the
/// tail until it is done (then stops into state 6, clearing state bits 0x20 and 0x40); the
/// stream's cursor becomes the play cursor (0 after a wrap). Otherwise a playing sound with type
/// bit 0x1000000 pauses or resumes with the audio manager's pause flag (`+0x1c` bit 0, state bit
/// 0x100000) and, unpaused, resynchronises its play cursor with the manager's clock (`+0x14`
/// minus `iSynchStartTime`) when it drifted more than 50 ms, stopping (and dropping the bit) when
/// the target is past 1.1 times the duration. `UpdateEmitterPosition` (vtable slot `0x40`) runs
/// last; the answer is 1.
pub fn fn_00aee580(e: &mut Engine, this: Ptr, _unused_1: u32) -> u8 {
    if field(e, this, BUFFER) != 0 && field(e, this, DIRECT_SOUND) != 0 {
        let status = e.with_stack(4, |e, status| {
            com(e, this, BUFFER, 0x24, &[status.addr()]);
            e.mem.u32(status.addr())
        });
        if status & 1 != 0 {
            update_state(e, this, |flags| flags | 0x20);
        } else {
            update_state(e, this, |flags| flags & 0xffff_ffdf);
        }
        let streaming = e.vcall(this.addr(), 4, &args![]).bool()
            && is_streaming(e, this)
            && field(e, this, STREAM) != 0;
        if streaming {
            if !update_stream(e, this) {
                return 0;
            }
        } else {
            update_synchronised(e, this);
        }
    }
    e.vcall(this.addr(), 0x40, &args![]);
    1
}

/// The streaming part of `Update` (`00aee580`); false when it answers 0 at once.
fn update_stream(e: &mut Engine, this: Ptr) -> bool {
    let (cursor, _) = e.with_stack(8, |e, cursors| {
        let play = cursors.addr();
        com(e, this, BUFFER, 0x10, &[play, play + 4]);
        (e.mem.u32(play), e.mem.u32(play + 4))
    });
    let stream = field(e, this, STREAM);
    let at = e.mem.u32(stream + 0x14);
    let mut wrapped = false;
    let mut count = if at > cursor {
        wrapped = true;
        field(e, this, DATA_BYTES).wrapping_sub(at)
    } else if at < cursor {
        cursor.wrapping_sub(at)
    } else {
        return false;
    };
    let state = e.mem.u32(stream);
    let length_of = |e: &Engine, offset: u32| {
        let stream = field(e, this, STREAM);
        e.mem
            .u32(stream + offset)
            .wrapping_sub(e.mem.u32(stream + 4))
    };
    match state {
        1 => {
            if count < length_of(e, 8) {
                let block = e.mem.u32(stream + 0x18);
                stream_fill(e, this, count, block);
                let stream = field(e, this, STREAM);
                let written = e.mem.u32(stream + 4).wrapping_add(count);
                e.mem.set_u32(stream + 4, written);
            } else if count >= length_of(e, 8) {
                count = length_of(e, 8);
                let block = e.mem.u32(stream + 0x18);
                stream_fill(e, this, count, block);
                let stream = field(e, this, STREAM);
                e.mem.set_u32(stream + 4, 0);
                e.mem.set_u32(stream, 2);
            }
        }
        2 if e.mem.u32(stream + 0xc) != 0 => {
            let mut rest = 0u32;
            let overflow = count > length_of(e, 0xc);
            if overflow {
                rest = count.wrapping_sub(length_of(e, 0xc));
                count = length_of(e, 0xc);
            }
            let block = e.mem.u32(stream + 0x1c);
            stream_fill(e, this, count, block);
            let stream = field(e, this, STREAM);
            let written = e.mem.u32(stream + 4).wrapping_add(count) % e.mem.u32(stream + 0xc);
            e.mem.set_u32(stream + 4, written);
            if overflow {
                let at = e.mem.u32(stream + 0x14).wrapping_add(count);
                e.mem.set_u32(stream + 0x14, at);
                let block = e.mem.u32(stream + 0x1c);
                stream_fill(e, this, rest, block);
                let stream = field(e, this, STREAM);
                e.mem.set_u32(stream + 4, rest);
            }
        }
        4 => {
            if flags8(e, this) & 0x0200_0000 != 0 {
                e.mem.set_u32(stream + 4, 0);
                let tail = e.mem.u32(stream + 0x10);
                let tail_block = e.mem.u32(stream + 0x20);
                if field(e, this, DATA_BYTES) > tail {
                    com(e, this, BUFFER, 0x48, &[]);
                    com(e, this, BUFFER, 0x34, &[0]);
                    fn_00af1c00(e, this, 0, 0, tail, tail_block);
                    let rest = field(e, this, DATA_BYTES).wrapping_sub(tail);
                    fn_00af1c00(e, this, 0, tail, rest, 0);
                    e.mem.set_u32(field(e, this, STREAM), 6);
                    com(e, this, BUFFER, 0x30, &[0, 0, 0]);
                } else {
                    com(e, this, BUFFER, 0x48, &[]);
                    com(e, this, BUFFER, 0x34, &[0]);
                    let bytes = field(e, this, DATA_BYTES);
                    fn_00af1c00(e, this, 0, 0, bytes, tail_block);
                    com(e, this, BUFFER, 0x30, &[0, 0, 1]);
                    let stream = field(e, this, STREAM);
                    e.mem.set_u32(stream, 5);
                    let written = e.mem.u32(stream + 4).wrapping_add(bytes);
                    e.mem.set_u32(stream + 4, written);
                }
                let stream = field(e, this, STREAM);
                e.mem.set_u32(stream + 0x14, 0);
                return false;
            }
            if flags8(e, this) & 0x0400_0000 != 0 {
                e.mem.set_u32(stream, 3);
                com(e, this, BUFFER, 0x48, &[]);
                com(e, this, BUFFER, 0x30, &[0, 0, 0]);
            }
        }
        5 => {
            if count < length_of(e, 0x10) {
                let block = e.mem.u32(stream + 0x20);
                stream_fill(e, this, count, block);
                let stream = field(e, this, STREAM);
                let written = e.mem.u32(stream + 4).wrapping_add(count);
                e.mem.set_u32(stream + 4, written);
            } else if count >= length_of(e, 0x10) {
                com(e, this, BUFFER, 0x48, &[]);
                count = length_of(e, 0x10);
                let stream = field(e, this, STREAM);
                let block = e.mem.u32(stream + 0x20);
                stream_fill(e, this, count, block);
                com(e, this, BUFFER, 0x30, &[0, 0, 0]);
                let stream = field(e, this, STREAM);
                e.mem.set_u32(stream, 6);
                update_state(e, this, |flags| flags & 0xffff_ffdf);
                update_state(e, this, |flags| flags & 0xffff_ffbf);
            }
        }
        _ => {}
    }
    let stream = field(e, this, STREAM);
    e.mem
        .set_u32(stream + 0x14, if wrapped { 0 } else { cursor });
    true
}

/// The pause and resynchronisation part of `Update` (`00aee580`).
fn update_synchronised(e: &mut Engine, this: Ptr) {
    if !e.vcall(this.addr(), 4, &args![]).bool() || flags8(e, this) & 0x0100_0000 == 0 {
        return;
    }
    let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
    let paused = e.mem.u32(manager.wrapping_add(0x1c)) & 1 != 0;
    let held = field(e, this, STATE_FLAGS) & 0x10_0000 != 0;
    if !held && paused {
        // The exe tests a constant here; only the setting branch is reached.
        let set = true;
        if set {
            update_state(e, this, |flags| flags | 0x10_0000);
        } else {
            update_state(e, this, |flags| flags & 0xffef_ffff);
        }
        e.vcall(this.addr(), 0x34, &args![]);
    } else if held && !paused {
        // The exe tests a constant here; only the clearing branch is reached.
        let set = false;
        if set {
            update_state(e, this, |flags| flags | 0x10_0000);
        } else {
            update_state(e, this, |flags| flags & 0xffef_ffff);
        }
        e.vcall(this.addr(), 0x30, &args![0u32]);
    }
    if field(e, this, STATE_FLAGS) & 0x10_0000 != 0 {
        return;
    }
    let (cursor, average) = e.with_stack(0x18, |e, locals| {
        let format = locals.addr();
        let cursor = format + 0x14;
        e.call(MEMSET, &args![format, 0u32, 0x12u32]);
        e.mem.set_u32(cursor, 0);
        com(e, this, BUFFER, 0x10, &[cursor, 0]);
        com(e, this, BUFFER, 0x14, &[format, 0x12, 0]);
        (e.mem.u32(cursor), e.mem.u32(format + 8))
    });
    let thousand = double_at(e, 0x0101_7b70);
    let per_millisecond = e
        .call(0x00ec_62c0, &args![x87_div(f64::from(average), thousand)])
        .u32();
    let rate = x87_div(f64::from(average), thousand);
    let played = x87_div(f64::from(cursor), rate);
    let played = x87_fistp64_chop(X87Ext::load64(played)) as u32;
    let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
    let now = e.mem.u32(manager.wrapping_add(0x14));
    let elapsed = now.wrapping_sub(field(e, this, SYNCH_START_TIME));
    let drift = if played > elapsed {
        played.wrapping_sub(elapsed).wrapping_neg() as i32
    } else if played < elapsed {
        elapsed.wrapping_sub(played) as i32
    } else {
        0
    };
    if drift <= 0x32 && drift.wrapping_neg() <= 0x32 {
        return;
    }
    let mut target = drift
        .wrapping_mul(per_millisecond as i32)
        .wrapping_add(cursor as i32);
    let length = (field(e, this, DURATION) as i32).wrapping_mul(per_millisecond as i32);
    let limit = x87_mul(f64::from(length), double_at(e, 0x0102_90f8));
    if !parity_even(fpu_compare(limit, f64::from(target)) & 0x05) {
        set_field(e, this, CLEANUP_DELAY, 100);
        set_field(e, this, SYNCH_START_TIME, 0);
        update_flags8(e, this, |flags| flags & 0xfeff_ffff);
        e.vcall(this.addr(), 0x38, &args![]);
        return;
    }
    let length = (field(e, this, DURATION) as i32).wrapping_mul(per_millisecond as i32);
    if target > length {
        target = (field(e, this, DURATION) as i32).wrapping_mul(per_millisecond as i32);
    } else if target < 0 {
        target = 0;
    }
    com(e, this, BUFFER, 0x34, &[target as u32]);
}

/// `BSGameSound::Copy` (`00ae97a0`, `RET 8`: the source and a flag).
const GAME_SOUND_COPY: u32 = 0x00ae_97a0;
/// `BSGameSound::uiAudioDataSize` and `uiSoundDataSize` (Xbox PDB) `+0x148` and `+0x14c`.
const AUDIO_DATA_SIZE: u32 = 0x148;
/// The maximum distance handed to a copied 3D buffer (`010a3dcc`).
const COPY_MAX_DISTANCE_010A3DCC: u32 = 0x010a_3dcc;

/// The message `00af0630` prints for a failed `CreateSoundBuffer`.
fn create_buffer_message(result: u32) -> u32 {
    match result {
        // DSERR_ALLOCATED
        0x8878_000a => 0x010a_4168,
        // DSERR_CONTROLUNAVAIL
        0x8878_001e => 0x010a_40b0,
        // E_OUTOFMEMORY
        0x8007_000e => 0x010a_3f84,
        // E_NOTIMPL (DSERR_UNSUPPORTED)
        0x8000_4001 => 0x010a_3f0c,
        // CLASS_E_NOAGGREGATION
        0x8004_0110 => 0x010a_3fc0,
        // E_INVALIDARG
        0x8007_0057 => 0x010a_3ffc,
        // DSERR_INVALIDCALL
        0x8878_0032 => 0x010a_4038,
        // DSERR_BADFORMAT
        0x8878_0064 => 0x010a_4130,
        // DSERR_UNINITIALIZED
        0x8878_00aa => 0x010a_3f48,
        // DSERR_BUFFERTOOSMALL
        0x8878_00b4 => 0x010a_40f0,
        // DSERR_DS8_REQUIRED
        0x8878_00be => 0x010a_4074,
        // "- unknown warning"
        _ => 0x010a_3ed0,
    }
}

// Translated from 00af0630 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00af0630 - the /GS cookie check (00ec408c) is not translated
/// Vtable slot `0x2c` (`Copy`, `RET 8`: the source sound and a byte). The source's vtable slot
/// `0x28` runs first when it holds sample data (`+0x1c8`), then `BSGameSound::Copy` (`00ae97a0`,
/// flag 1). The second, `1d0` and `1d8` buffers, the buffer and the 3D buffer are cleared; the
/// sizes (`+0x148`, `+0x1dc`, `+0x14c`), sample rates (`+0x34`, `+0x15a`) and device come from the
/// source. With a source buffer: type bit 1 is set unless bits 1 or 2 are; the `DSBUFFERDESC`
/// (`+0x1a4`) gets the source's byte count and format (`GetFormat`, slot `0x14`) and flags 0x40
/// (type bit 1 or more than one channel) or 0x30010 (type bit 2), plus 0x2a0; a sound of at most
/// 150 ms without type bits 0x10, 0x4000000 and 0x2000000 is padded to 250 ms (and loses flag
/// 0x200). The duration is recomputed from the bytes, the 3D algorithm (`+0x1b8`) is
/// `GUID_NULL` for type bit 1 or without the audio manager's HRTF word (`+0x60`), the sample rate
/// becomes the format's and the modified rate keeps its ratio (truncated). `CreateSoundBuffer`
/// (device slot `0xc`, into `+0x1cc`) failing prints the matching message. Otherwise both
/// buffers are locked whole, the source's bytes copied and both unlocked (the new buffer even
/// when the source's lock fails; a failed lock of the new buffer ends it), the buffer and its 3D
/// buffer (type bit 2; max distance from `010a3dcc`) queried, and a streaming source's stream
/// (`+0x228`) duplicated with its three blocks. State flags clear, the sound is opened
/// (`00ae9c50`) and, with the byte set, vtable slot `0x28` runs.
pub fn fn_00af0630(e: &mut Engine, this: Ptr, source: Ptr, load: u32) {
    if field(e, source, SAMPLE_DATA) != 0 {
        e.vcall(source.addr(), 0x28, &args![]);
    }
    e.call(GAME_SOUND_COPY, &args![this, source, 1u32]);
    set_field(e, this, SECOND_BUFFER, 0);
    set_field(e, this, BUFFER_1D0, 0);
    set_field(e, this, BUFFER_1D8, 0);
    let audio = field(e, source, AUDIO_DATA_SIZE);
    set_field(e, this, AUDIO_DATA_SIZE, audio);
    let bytes = field(e, source, DATA_BYTES);
    set_field(e, this, DATA_BYTES, bytes);
    set_field(e, this, SOUND_SIZE, bytes);
    let rate = e.mem.u16(source.addr() + SAMPLES_PER_SECOND);
    e.mem.set_u16(this.addr() + SAMPLES_PER_SECOND, rate);
    let modified = e.mem.u16(source.addr() + MOD_SAMPLES_PER_SECOND);
    e.mem
        .set_u16(this.addr() + MOD_SAMPLES_PER_SECOND, modified);
    set_field(e, this, BUFFER, 0);
    set_field(e, this, BUFFER_3D, 0);
    let device = field(e, source, DIRECT_SOUND);
    set_field(e, this, DIRECT_SOUND, device);
    if source.addr() == 0 || field(e, source, BUFFER) == 0 {
        return;
    }
    if flags8(e, this) & 1 == 0 && flags8(e, this) & 2 == 0 {
        update_flags8(e, this, |flags| flags | 1);
    }
    e.with_stack(0x24, |e, locals| {
        copy_buffer(e, this, source, load, locals.addr())
    });
}

/// Everything of `00af0630` from the buffer description on; `locals` holds the eight lock words.
fn copy_buffer(e: &mut Engine, this: Ptr, source: Ptr, load: u32, locals: u32) {
    let (new_lock, new_bytes, new_second, new_second_bytes) =
        (locals, locals + 4, locals + 8, locals + 0xc);
    let (old_lock, old_bytes, old_second, old_second_bytes) =
        (locals + 0x10, locals + 0x14, locals + 0x18, locals + 0x1c);
    let written = locals + 0x20;
    set_field(e, this, BUFFER_DESC, 0x24);
    let bytes = field(e, source, BUFFER_BYTES);
    set_field(e, this, BUFFER_BYTES, bytes);
    let format = field(e, this, WAVE_FORMAT);
    com(e, source, BUFFER, 0x14, &[format, 0x12, written]);
    let format = field(e, this, WAVE_FORMAT);
    if flags8(e, this) & 1 != 0 || (format != 0 && (e.mem.u16(format + 2) as i32) > 1) {
        set_field(e, this, BUFFER_FLAGS, 0x40);
    } else if flags8(e, this) & 2 != 0 {
        set_field(e, this, BUFFER_FLAGS, 0x30010);
    }
    let flags = field(e, this, BUFFER_FLAGS) | 0x2a0;
    set_field(e, this, BUFFER_FLAGS, flags);
    let format = field(e, this, WAVE_FORMAT);
    let sample_bytes = u32::from(e.mem.u16(format + 0xe)) >> 3;
    let per_millisecond = u32::from(e.mem.u16(this.addr() + SAMPLES_PER_SECOND)) / 1000;
    let mut padding = 0u32;
    if field(e, this, DURATION) as i32 <= 0x96 {
        let flags = field(e, this, BUFFER_FLAGS) & 0xffff_fdff;
        set_field(e, this, BUFFER_FLAGS, flags);
        let kind = flags8(e, this);
        let excluded = kind & 0x10 != 0 || kind & 0x0400_0000 != 0 || kind & 0x0200_0000 != 0;
        if !excluded {
            padding = 0xfau32
                .wrapping_sub(field(e, this, DURATION))
                .wrapping_mul(sample_bytes)
                .wrapping_mul(per_millisecond);
        }
    }
    let total = padding.wrapping_add(field(e, this, DATA_BYTES));
    set_field(e, this, DATA_BYTES, total);
    set_field(e, this, BUFFER_BYTES, total);
    set_field(e, this, SOUND_SIZE, total);
    set_field(e, this, DURATION, total / sample_bytes / per_millisecond);
    let guid = if flags8(e, this) & 1 != 0 {
        GUID_NULL_01093F04
    } else {
        let manager = e.mem.u32(AUDIO_MANAGER_011F6D98);
        if e.mem.u32(manager.wrapping_add(0x60)) != 0 {
            HRTF_GUID_01100FE4
        } else {
            GUID_NULL_01093F04
        }
    };
    for word in 0..4 {
        let value = e.mem.u32(guid + 4 * word);
        set_field(e, this, ALGORITHM_GUID + 4 * word, value);
    }
    let modified = e.mem.u16(this.addr() + MOD_SAMPLES_PER_SECOND);
    let rate = e.mem.u16(this.addr() + SAMPLES_PER_SECOND);
    let ratio = x87_store32(x87_div(f64::from(modified), f64::from(rate)));
    let format = field(e, this, WAVE_FORMAT);
    let rate = e.mem.u16(format + 4);
    e.mem.set_u16(this.addr() + SAMPLES_PER_SECOND, rate);
    let scaled = fistp_chop(x87_mul(f64::from(rate), x87_m32(ratio)));
    e.mem
        .set_u16(this.addr() + MOD_SAMPLES_PER_SECOND, scaled as u16);
    let sample_bytes = u32::from(e.mem.u16(format + 0xe)) >> 3;
    let per_millisecond = u32::from(rate) / 1000;
    let duration = field(e, this, DATA_BYTES) / sample_bytes / per_millisecond;
    set_field(e, this, DURATION, duration);
    let base = this.addr();
    let result = com(
        e,
        this,
        DIRECT_SOUND,
        0xc,
        &[base + BUFFER_DESC, base + CREATED_BUFFER, 0],
    );
    if (result as i32) < 0 {
        e.with_stack(0x100, |e, message| {
            let text = create_buffer_message(result);
            e.call(STRCPY_S, &args![message.addr(), 0xffu32, text]);
            e.call(PRINT, &args![message.addr()]);
        });
        return;
    }
    let locked = com(
        e,
        this,
        CREATED_BUFFER,
        0x2c,
        &[0, 0, new_lock, new_bytes, new_second, new_second_bytes, 2],
    );
    if (locked as i32) < 0 {
        return;
    }
    let source_locked = com(
        e,
        source,
        BUFFER,
        0x2c,
        &[0, 0, old_lock, old_bytes, old_second, old_second_bytes, 2],
    );
    if source_locked as i32 >= 0 {
        let count = field(e, this, BUFFER_BYTES);
        let from = e.mem.u32(old_lock);
        let to = e.mem.u32(new_lock);
        e.call(MEMCPY, &args![to, from, count]);
        let words = [
            e.mem.u32(old_lock),
            e.mem.u32(old_bytes),
            e.mem.u32(old_second),
            e.mem.u32(old_second_bytes),
        ];
        com(e, source, BUFFER, 0x4c, &words);
    }
    let words = [
        e.mem.u32(new_lock),
        e.mem.u32(new_bytes),
        e.mem.u32(new_second),
        e.mem.u32(new_second_bytes),
    ];
    com(e, this, CREATED_BUFFER, 0x4c, &words);
    com(e, this, CREATED_BUFFER, 0, &[0x0110_0f94, base + BUFFER]);
    if field(e, this, BUFFER) != 0 && flags8(e, this) & 2 != 0 {
        com(e, this, BUFFER, 0, &[0x0110_0fb4, base + BUFFER_3D]);
        if field(e, this, BUFFER_3D) != 0 {
            let distance = x87_quiet_bits(e.mem.u32(COPY_MAX_DISTANCE_010A3DCC));
            com(e, this, BUFFER_3D, 0x44, &[distance, 1]);
        }
    }
    let kind = flags8(e, source);
    let streamed = kind & 0x0400_0000 != 0 || kind & 0x0200_0000 != 0;
    if streamed && field(e, source, STREAM) != 0 {
        copy_stream(e, this, source);
    }
    set_field(e, this, STATE_FLAGS, 0);
    set_opened(e, this, 1);
    if load & 0xff != 0 {
        e.vcall(this.addr(), 0x28, &args![]);
    }
}

/// The stream part of `00af0630`: a new 0x24-byte stream with the source's intro, loop and
/// tail lengths (`+8`, `+0xc`, `+0x10`), cleared state and cursors, and copies of its blocks.
fn copy_stream(e: &mut Engine, this: Ptr, source: Ptr) {
    let stream = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x24u32]).u32();
    set_field(e, this, STREAM, stream);
    for offset in [8, 0x10, 0xc] {
        let value = e.mem.u32(field(e, source, STREAM) + offset);
        e.mem.set_u32(field(e, this, STREAM) + offset, value);
    }
    for offset in [0x14, 0, 4] {
        e.mem.set_u32(field(e, this, STREAM) + offset, 0);
    }
    for (length, block) in [(8, 0x18), (0xc, 0x1c), (0x10, 0x20)] {
        let size = e.mem.u32(field(e, this, STREAM) + length);
        let memory = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
        e.mem.set_u32(field(e, this, STREAM) + block, memory);
    }
    for (length, block) in [(8, 0x18), (0xc, 0x1c), (0x10, 0x20)] {
        let size = e.mem.u32(field(e, this, STREAM) + length);
        let from = e.mem.u32(field(e, source, STREAM) + block);
        let to = e.mem.u32(field(e, this, STREAM) + block);
        e.call(MEMCPY, &args![to, from, size]);
    }
}

/// `NV_AUDIO_CHECK` (diagnostic only, not exe behaviour): follows every `BSWin32GameSound` and
/// its Ogg Vorbis reader (`+0x22c`), and stops the run at the first use that does not fit what
/// came before, naming both events (game thread, the exe frames of [`Engine::call_stack`]) and
/// the sequence number of each:
///
/// - a sound opened (`00aeb270`) or destroyed (`00aeafb0`) that is not live (never constructed
///   while the check ran, or destroyed already);
/// - a sound given a second reader while it still has one (`00aebff0`);
/// - a sound destroyed with a reader that is not its own (another sound's, or one already
///   destroyed), or whose reader's file word (`+0x2d0`) is not the file it opened;
/// - between the open (`00af1e00`) and the reader's destruction (`00af1d80`), a change to the
///   reader's file word or to the file's vtable word, or a free or re-allocation of either's
///   block (the write watch, `Mem::watch`; at the next call or return).
///
/// Any value but `0` turns it on. It writes no game memory; off, it reads none either.
pub mod audio_check {
    use crate::prelude::*;
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};

    /// One event: what, its number, the game thread and the exe frames.
    #[derive(Clone, Debug)]
    pub struct Event {
        pub what: String,
        pub seq: u64,
        pub thread: u32,
        pub frames: Vec<u32>,
    }

    impl std::fmt::Display for Event {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let frames: Vec<String> = self
                .frames
                .iter()
                .rev()
                .map(|a| format!("{a:08x}"))
                .collect();
            write!(
                f,
                "#{} {} (thread {}, in {})",
                self.seq,
                self.what,
                self.thread,
                frames.join(" < ")
            )
        }
    }

    /// What the check knows; each method answers the stop's text when the event does not fit.
    #[derive(Default)]
    pub struct State {
        seq: u64,
        /// Live sounds: their construction.
        sounds: HashMap<u32, Event>,
        /// Destroyed sounds (the latest destruction at each address).
        dead_sounds: HashMap<u32, Event>,
        /// Live readers: the sound that made it, the file it opened (0 until open), the making.
        readers: HashMap<u32, (u32, u32, Event)>,
        /// Destroyed readers: the destruction.
        dead_readers: HashMap<u32, Event>,
    }

    fn report(now: &Event, why: String, earlier: &[Option<&Event>]) -> String {
        let mut text = format!("NV_AUDIO_CHECK: {why}\n  now: {now}");
        for event in earlier.iter().flatten() {
            text += &format!("\n  earlier: {event}");
        }
        text
    }

    impl State {
        pub fn event(&mut self, what: String, thread: u32, frames: Vec<u32>) -> Event {
            self.seq += 1;
            Event {
                what,
                seq: self.seq,
                thread,
                frames,
            }
        }

        pub fn constructed(&mut self, sound: u32, now: Event) {
            self.dead_sounds.remove(&sound);
            self.sounds.insert(sound, now);
        }

        pub fn opening(&mut self, sound: u32, now: Event) -> Result<(), String> {
            if self.sounds.contains_key(&sound) {
                return Ok(());
            }
            Err(report(
                &now,
                format!("sound {sound:08x} opened while it is not live"),
                &[self.dead_sounds.get(&sound)],
            ))
        }

        pub fn reader_made(
            &mut self,
            sound: u32,
            reader: u32,
            previous: u32,
            now: Event,
        ) -> Result<(), String> {
            if previous != 0 {
                let made = self.readers.get(&previous).map(|r| &r.2);
                return Err(report(
                    &now,
                    format!("sound {sound:08x} already has the reader {previous:08x}"),
                    &[made, self.sounds.get(&sound)],
                ));
            }
            self.dead_readers.remove(&reader);
            self.readers.insert(reader, (sound, 0, now));
            Ok(())
        }

        pub fn reader_opened(&mut self, reader: u32, file: u32) {
            if let Some(r) = self.readers.get_mut(&reader) {
                r.1 = file;
            }
        }

        /// Answers the file the reader opened (0: none).
        pub fn reader_destroyed(&mut self, reader: u32, now: Event) -> u32 {
            let file = self.readers.remove(&reader).map_or(0, |r| r.1);
            self.dead_readers.insert(reader, now);
            file
        }

        /// The text naming a watched word (`reader_opened`) that changed: the reader and sound
        /// it belongs to and the reader's making.
        pub fn watched(&self, at: u32) -> String {
            for (reader, (sound, file, made)) in &self.readers {
                if at.wrapping_sub(reader.wrapping_add(0x2d0)) < 4 {
                    return format!(
                        "the file word (+0x2d0) of reader {reader:08x} (sound {sound:08x}, file \
                         {file:08x}; {made})"
                    );
                }
                if *file != 0 && at.wrapping_sub(*file) < 4 {
                    return format!(
                        "the vtable word of file {file:08x} (reader {reader:08x}, sound \
                         {sound:08x}; {made})"
                    );
                }
            }
            format!("{at:08x} (no live reader's word)")
        }

        pub fn destroying(
            &mut self,
            sound: u32,
            reader: u32,
            file: u32,
            now: Event,
        ) -> Result<(), String> {
            if !self.sounds.contains_key(&sound) {
                return Err(report(
                    &now,
                    format!("sound {sound:08x} destroyed while it is not live"),
                    &[self.dead_sounds.get(&sound)],
                ));
            }
            if reader != 0 {
                match self.readers.get(&reader) {
                    None => {
                        return Err(report(
                            &now,
                            format!(
                                "sound {sound:08x} holds the reader {reader:08x}, which is not \
                                 live"
                            ),
                            &[self.dead_readers.get(&reader), self.sounds.get(&sound)],
                        ))
                    }
                    Some((owner, _, made)) if *owner != sound => {
                        return Err(report(
                            &now,
                            format!(
                                "sound {sound:08x} holds the reader {reader:08x} of sound \
                                 {owner:08x}"
                            ),
                            &[
                                Some(made),
                                self.sounds.get(owner),
                                self.dead_sounds.get(owner),
                            ],
                        ))
                    }
                    Some((_, opened, made)) if *opened != file => {
                        return Err(report(
                            &now,
                            format!(
                                "the reader {reader:08x} of sound {sound:08x} holds the file \
                                 word {file:08x}, not the file {opened:08x} it opened"
                            ),
                            &[Some(made)],
                        ))
                    }
                    Some(_) => {}
                }
            }
            self.sounds.remove(&sound);
            self.dead_sounds.insert(sound, now);
            Ok(())
        }
    }

    /// `NV_AUDIO_CHECK=fault`: only the report at a file delete that would fault (`00af1d80`),
    /// with the frees seen before it: nothing is followed or watched, so the run's timing stays
    /// close to one without the check. Any other value but `0`: everything.
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Mode {
        Off,
        Fault,
        Full,
    }

    fn mode() -> Mode {
        static MODE: OnceLock<Mode> = OnceLock::new();
        *MODE.get_or_init(|| match std::env::var("NV_AUDIO_CHECK").as_deref() {
            Err(_) | Ok("") | Ok("0") => Mode::Off,
            Ok("fault") => Mode::Fault,
            Ok(_) => Mode::Full,
        })
    }

    fn on() -> bool {
        mode() == Mode::Full
    }

    /// The frees counted so far, and the latest free of each block: its number, thread and exe
    /// frames.
    type Frees = (u64, HashMap<u32, (u64, u32, Vec<u32>)>);

    /// The latest free of each block (`Engine::free_hook`): its number, thread and exe frames.
    fn frees() -> std::sync::MutexGuard<'static, Frees> {
        static FREES: OnceLock<Mutex<Frees>> = OnceLock::new();
        FREES
            .get_or_init(Default::default)
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// `Engine::free_hook`: remembers who freed `block`.
    fn on_free(e: &mut Engine, block: u32) {
        let thread = engine_core::thread_host::current_id(e);
        let frames: Vec<u32> = e.call_stack.iter().rev().take(12).copied().collect();
        let mut f = frees();
        f.0 += 1;
        let n = f.0;
        f.1.insert(block, (n, thread, frames));
    }

    /// `00af1d80` is about to delete `file` through its vtable (`reader`'s `+0x2d0`): when the
    /// file or its vtable is not mapped (the delete would fault), stops the run with what is
    /// known: the words, whether the reader and the file are live heap blocks, and who freed
    /// each last.
    pub fn file_delete(e: &mut Engine, reader: u32, file: u32) {
        if mode() == Mode::Off {
            return;
        }
        let word = |at: u32| {
            let mut b = [0u8; 4];
            e.mem
                .try_read(at, &mut b)
                .ok()
                .map(|_| u32::from_le_bytes(b))
        };
        let vtable = word(file);
        if vtable.is_some_and(|v| word(v).is_some()) {
            return;
        }
        let words = |at: u32, n: u32| -> String {
            (0..n)
                .map(|k| {
                    word(at.wrapping_add(4 * k)).map_or("--------".into(), |w| format!("{w:08x}"))
                })
                .collect::<Vec<_>>()
                .join(" ")
        };
        let live = |block: u32| e.kernel.crt_block_sizes.get(&block).copied();
        let f = frees();
        let freed = |block: u32| match f.1.get(&block) {
            None => "never freed while the check ran".to_string(),
            Some((n, thread, frames)) => {
                let path: Vec<String> = frames.iter().map(|a| format!("{a:08x}")).collect();
                format!(
                    "last freed as free #{n} of {} (thread {thread}, in {})",
                    f.0,
                    path.join(" < ")
                )
            }
        };
        let path: Vec<String> = e
            .call_stack
            .iter()
            .rev()
            .map(|a| format!("{a:08x}"))
            .collect();
        let text = format!(
            "NV_AUDIO_CHECK: reader {reader:08x} deletes its file {file:08x}, whose vtable word is \
             {} (thread {}, in {}) | reader: live block {:?}; {} | reader +0: {} | reader \
             +0x2c0: {} | file: live block {:?}; {} | file +0: {}",
            vtable.map_or("unmapped".into(), |v| format!("{v:08x}")),
            engine_core::thread_host::current_id(e),
            path.join(" < "),
            live(reader),
            freed(reader),
            words(reader, 8),
            words(reader.wrapping_add(0x2c0), 8),
            live(file),
            freed(file),
            words(file, 8),
        );
        drop(f);
        // One line: the run's stop record keeps only the first.
        println!("{text}");
        panic!("{text}");
    }

    /// The process's state.
    fn state() -> std::sync::MutexGuard<'static, State> {
        static STATE: OnceLock<Mutex<State>> = OnceLock::new();
        STATE
            .get_or_init(Default::default)
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    /// Runs `body` on the process's state with the event `what` when the check is on; stops the
    /// run with the text it answers.
    fn with_state(
        e: &Engine,
        what: impl FnOnce() -> String,
        body: impl FnOnce(&mut State, Event) -> Result<(), String>,
    ) {
        if !on() {
            return;
        }
        let mut st = state();
        let now = st.event(
            what(),
            engine_core::thread_host::current_id(e),
            e.call_stack.clone(),
        );
        if let Err(text) = body(&mut st, now) {
            drop(st);
            panic!("{text}");
        }
    }

    /// `00aeac20` / `00aeae10` made `sound`.
    pub fn constructed(e: &mut Engine, sound: u32) {
        if mode() != Mode::Off && e.free_hook.is_none() {
            e.free_hook = Some(on_free);
        }
        with_state(
            e,
            || format!("sound {sound:08x} constructed"),
            |st, now| {
                st.constructed(sound, now);
                Ok(())
            },
        );
    }

    /// `00aeb270` opens `sound`.
    pub fn opening(e: &Engine, sound: u32) {
        with_state(
            e,
            || format!("sound {sound:08x} opened"),
            |st, now| st.opening(sound, now),
        );
    }

    /// `00aebff0` gives `sound` the reader `reader` (`previous`: its `+0x22c` before).
    pub fn reader_made(e: &Engine, sound: u32, reader: u32, previous: u32) {
        if reader == 0 {
            return;
        }
        with_state(
            e,
            || format!("reader {reader:08x} made for sound {sound:08x}"),
            |st, now| st.reader_made(sound, reader, previous, now),
        );
    }

    /// `00af1e00` opened `file` for `reader`.
    /// From here until `00af1d80` the reader's file word (`+0x2d0`) and the file's vtable word
    /// are watched ([`Mem::watch`](engine_core::mem::Mem::watch)): a store that changes either,
    /// or a free or re-allocation of its block, stops the run at the next call or return,
    /// naming the calls it happened in (`NV_WATCH` takes the same events: use one at a time).
    pub fn reader_opened(e: &mut Engine, reader: u32, file: u32) {
        if !on() {
            return;
        }
        with_state(
            e,
            || format!("reader {reader:08x} opened file {file:08x}"),
            |st, _| {
                st.reader_opened(reader, file);
                Ok(())
            },
        );
        install(e);
        e.mem.watch(reader.wrapping_add(0x2d0), 4, false);
        if file != 0 && e.mem.is_mapped(file) {
            e.mem.watch(file, 4, false);
        }
    }

    /// `00af1d80` destroys `reader` (its words are no longer watched).
    pub fn reader_destroyed(e: &mut Engine, reader: u32) {
        if !on() {
            return;
        }
        let mut file = 0;
        with_state(
            e,
            || format!("reader {reader:08x} destroyed"),
            |st, now| {
                file = st.reader_destroyed(reader, now);
                Ok(())
            },
        );
        e.mem.unwatch(reader.wrapping_add(0x2d0));
        if file != 0 {
            e.mem.unwatch(file);
        }
    }

    /// `Engine::call_hook`'s type.
    type CallHook = fn(&mut Engine, u32, bool);
    static PREVIOUS: Mutex<Option<CallHook>> = Mutex::new(None);

    /// Puts [`on_call`] in front of the engine's call hook (once).
    fn install(e: &mut Engine) {
        if e.call_hook
            .is_some_and(|h| h as usize == on_call as CallHook as usize)
        {
            return;
        }
        *PREVIOUS
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = e.call_hook;
        e.call_hook = Some(on_call);
    }

    /// `Engine::call_hook`: a watched word changed (`reader_opened`): stops the run naming it,
    /// the calls it happened in and the reader's making.
    fn on_call(e: &mut Engine, addr: u32, exit: bool) {
        // The previous call event (address, return or not, thread): the write happened between
        // it and this one.
        static LAST: Mutex<(u32, bool, u32)> = Mutex::new((0, false, 0));
        let thread = engine_core::thread_host::current_id(e);
        let last = std::mem::replace(
            &mut *LAST
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
            (addr, exit, thread),
        );
        if e.mem.has_watch_hits() {
            let hits = e.mem.take_watch_hits();
            let mut frames: Vec<u32> = Vec::new();
            if exit {
                frames.push(addr);
            }
            frames.extend(e.call_stack.iter().rev().take(16));
            let path: Vec<String> = frames.iter().map(|a| format!("{a:08x}")).collect();
            let event = |(addr, exit, thread): (u32, bool, u32)| {
                format!(
                    "{} {addr:08x} on thread {thread}",
                    if exit { "the return of" } else { "the call of" }
                )
            };
            let path = format!(
                "{} (between {} and {})",
                path.join(" < "),
                event(last),
                event((addr, exit, thread))
            );
            let st = state();
            let mut text = String::from("NV_AUDIO_CHECK: a watched word changed");
            for hit in &hits {
                text += &format!(
                    " | {}: {}",
                    st.watched(hit.addr),
                    engine_core::watch_check::report_text(hit, &path)
                );
            }
            drop(st);
            // One line: the run's stop record keeps only the first.
            println!("{text}");
            panic!("{text}");
        }
        let previous = *PREVIOUS
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(hook) = previous {
            hook(e, addr, exit);
        }
    }

    /// `00aeafb0` begins on `sound` (its reader at `+0x22c`, the reader's file at `+0x2d0`,
    /// read without faulting).
    pub fn destroying(e: &Engine, sound: u32) {
        if !on() {
            return;
        }
        let word = |at: u32| {
            let mut b = [0u8; 4];
            e.mem
                .try_read(at, &mut b)
                .map_or(0xdead_dead, |_| u32::from_le_bytes(b))
        };
        let reader = word(sound.wrapping_add(0x22c));
        let file = if reader == 0 {
            0
        } else {
            word(reader.wrapping_add(0x2d0))
        };
        with_state(
            e,
            || format!("sound {sound:08x} destroyed (reader {reader:08x}, file {file:08x})"),
            |st, now| st.destroying(sound, reader, file, now),
        );
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn ev(st: &mut State, what: &str) -> Event {
            st.event(what.into(), 1, vec![0x00ad_dc00, 0x00af_1cc0])
        }

        #[test]
        fn a_sound_with_its_own_reader_passes() {
            let mut st = State::default();
            let now = ev(&mut st, "make");
            st.constructed(0x100, now);
            let now = ev(&mut st, "open");
            assert!(st.opening(0x100, now).is_ok());
            let now = ev(&mut st, "reader");
            assert!(st.reader_made(0x100, 0x500, 0, now).is_ok());
            st.reader_opened(0x500, 0x900);
            let now = ev(&mut st, "destroy");
            assert!(st.destroying(0x100, 0x500, 0x900, now).is_ok());
            // Opened again after its destruction: a stop naming the destruction.
            let now = ev(&mut st, "open again");
            let text = st.opening(0x100, now).unwrap_err();
            assert!(
                text.contains("not live") && text.contains("#4 destroy"),
                "{text}"
            );
        }

        #[test]
        fn a_watched_word_is_named_by_its_reader() {
            let mut st = State::default();
            let now = ev(&mut st, "make");
            st.constructed(0x100, now);
            let now = ev(&mut st, "reader");
            st.reader_made(0x100, 0x500, 0, now).unwrap();
            st.reader_opened(0x500, 0x900);
            let text = st.watched(0x500 + 0x2d0 + 1);
            assert!(
                text.starts_with("the file word (+0x2d0) of reader 00000500"),
                "{text}"
            );
            let text = st.watched(0x900);
            assert!(
                text.starts_with("the vtable word of file 00000900"),
                "{text}"
            );
            let now = ev(&mut st, "reader gone");
            assert_eq!(st.reader_destroyed(0x500, now), 0x900);
            assert!(st.watched(0x900).contains("no live reader"));
        }

        #[test]
        fn a_foreign_dead_or_changed_reader_stops() {
            let mut st = State::default();
            for sound in [0x100, 0x200] {
                let now = ev(&mut st, "make");
                st.constructed(sound, now);
            }
            let now = ev(&mut st, "reader of 100");
            st.reader_made(0x100, 0x500, 0, now).unwrap();
            st.reader_opened(0x500, 0x900);
            // Sound 200 holds sound 100's reader.
            let now = ev(&mut st, "destroy 200");
            let text = st.destroying(0x200, 0x500, 0x900, now).unwrap_err();
            assert!(text.contains("of sound 00000100"), "{text}");
            // The file word changed under the reader.
            let now = ev(&mut st, "destroy 100");
            let text = st.destroying(0x100, 0x500, 0xb, now).unwrap_err();
            assert!(text.contains("file word 0000000b"), "{text}");
            // A destroyed reader still held.
            let now = ev(&mut st, "reader gone");
            st.reader_destroyed(0x500, now);
            let now = ev(&mut st, "destroy 100");
            let text = st.destroying(0x100, 0x500, 0x900, now).unwrap_err();
            assert!(
                text.contains("not live") && text.contains("reader gone"),
                "{text}"
            );
        }
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ae_ac20, fn_00aeac20(Ptr, u32) -> Ptr),
        entry!(0x00ae_afb0, fn_00aeafb0(Ptr)),
        entry!(0x00af_1cc0, fn_00af1cc0(Ptr, u32) -> Ptr),
        entry!(0x00af_1ba0, fn_00af1ba0(Ptr) -> u8),
        entry!(0x00af_13f0, fn_00af13f0(Ptr, u32)),
        entry!(0x00ae_fcf0, fn_00aefcf0(Ptr, f32, f32, f32)),
        entry!(0x00ae_fd20, fn_00aefd20(Ptr, Ptr) -> Ptr),
        entry!(0x00af_1c00, fn_00af1c00(Ptr, u32, u32, u32, u32) -> u8),
        entry!(0x00ae_d420, fn_00aed420(Ptr) -> u32),
        entry!(0x00ae_d4c0, fn_00aed4c0(Ptr) -> u32),
        entry!(0x00ae_d080, fn_00aed080(Ptr, u32) -> u32),
        entry!(0x00ae_d660, fn_00aed660(Ptr, f32) -> u8),
        entry!(0x00ae_fda0, fn_00aefda0(Ptr, f32, f32)),
        entry!(0x00ae_ff60, fn_00aeff60(Ptr, u32, u32, u32, u32, u32)),
        entry!(0x00af_01e0, fn_00af01e0(Ptr, f32) -> u32),
        entry!(0x00af_0290, fn_00af0290(Ptr) -> f64),
        entry!(0x00af_0320, fn_00af0320(Ptr, u32)),
        entry!(0x00ae_b270, fn_00aeb270(Ptr) -> u32),
        entry!(0x00ae_bff0, fn_00aebff0(Ptr) -> u32),
        entry!(0x00af_0400, fn_00af0400(u32, u32, u32, u32) -> u32),
        entry!(0x00af_1d20, fn_00af1d20(Ptr) -> Ptr),
        entry!(0x00af_1d80, fn_00af1d80(Ptr)),
        entry!(0x00af_1e00, fn_00af1e00(Ptr, u32) -> u8),
        entry!(0x00af_2100, fn_00af2100(Ptr, u32, u32) -> u32),
        entry!(0x00af_2180, fn_00af2180(u32, u32, u32, u32) -> u32),
        entry!(0x00af_2200, fn_00af2200(u32, u32, u32, u32) -> u32),
        entry!(0x00af_2290, fn_00af2290(u32) -> u32),
        entry!(0x00ae_c530, fn_00aec530(Ptr) -> bool),
        entry!(0x00af_1430, fn_00af1430(Ptr) -> u8),
        entry!(0x00ae_f090, fn_00aef090(Ptr, f32, f32, f32)),
        entry!(0x00ae_d990, fn_00aed990(Ptr)),
        entry!(0x00ae_e580, fn_00aee580(Ptr, u32) -> u8),
        entry!(0x00af_0630, fn_00af0630(Ptr, Ptr, u32)),
        entry!(0x00ae_abf0, fn_00aeabf0(u32, u32, u32, u32) -> i32),
        entry!(0x00ae_ae10, fn_00aeae10(Ptr) -> Ptr),
        entry!(0x00ae_fd90, fn_00aefd90(Ptr, u32, u32, u32)),
        entry!(0x00af_1af0, fn_00af1af0(Ptr)),
        entry!(0x00af_1cf0, fn_00af1cf0(u32, u32, u32, u32) -> i32),
        entry!(0x00af_2230, fn_00af2230(u32) -> i32),
    ]
}

#[cfg(test)]
mod b035_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Log = Vec<(u32, Vec<u32>)>;

    fn calls(log: &Log, address: u32) -> Vec<Vec<u32>> {
        log.iter()
            .filter(|(a, _)| *a == address)
            .map(|(_, w)| w.clone())
            .collect()
    }

    const SOUND_SLOTS: u32 = 0x7100_0000;
    const BUFFER_SLOTS: u32 = 0x7200_0000;
    const DEVICE_SLOTS: u32 = 0x7300_0000;
    const BUFFER_3D_SLOTS: u32 = 0x7400_0000;
    const SECOND_SLOTS: u32 = 0x7500_0000;
    const CREATED_SLOTS: u32 = 0x7600_0000;
    const OTHER_SLOTS: u32 = 0x7700_0000;
    const LISTENER_SLOTS: u32 = 0x7800_0000;

    /// An object whose vtable slot `n` calls `base + n` (no-op doubles answering 0).
    fn object(e: &mut Engine, base: u32) -> u32 {
        let vtable = e.mem.alloc(0x100);
        for slot in (0..0x100).step_by(4) {
            e.mem.set_u32(vtable + slot, base + slot);
            if !e.is_translated(base + slot) {
                e.register(base + slot, |_, _| Ret::default());
            }
        }
        let object = e.mem.alloc(0x200);
        e.mem.set_u32(object, vtable);
        object
    }

    fn set_f64(e: &mut Engine, address: u32, value: f64) {
        e.mem.set_u64(address, value.to_bits());
    }

    fn set_f32(e: &mut Engine, address: u32, value: f32) {
        e.mem.set_u32(address, value.to_bits());
    }

    fn put_string(e: &mut Engine, address: u32, text: &str) {
        for (index, byte) in text.bytes().enumerate() {
            e.mem.set_u8(address + index as u32, byte);
        }
        e.mem.set_u8(address + text.len() as u32, 0);
    }

    fn string_at(e: &Engine, address: u32) -> String {
        let mut text = String::new();
        let mut at = address;
        while e.mem.u8(at) != 0 {
            text.push(e.mem.u8(at) as char);
            at += 1;
        }
        text
    }

    /// The data section, the globals, the memory manager, the audio manager (returned) and
    /// the doubles every test shares.
    fn engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x10_0000);
        e.map(0x011f_4000, 0x4000);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1].max(4)).into_ret());
        e.register(FREE, |_, _| Ret::default());
        e.register(SET_OPENED, |_, a| a[1].into_ret());
        e.register(IS_3D, |_, _| 0u32.into_ret());
        e.register(IS_3D_OTHER, |_, _| 0u32.into_ret());
        e.register(PRINT, |_, _| Ret::default());
        let manager = e.mem.alloc(0x100);
        e.mem.set_u32(AUDIO_MANAGER_011F6D98, manager);
        set_f64(&mut e, ZERO_01012060, 0.0);
        set_f64(&mut e, ONE_01012070, 1.0);
        set_f64(&mut e, HUNDRED_01017A40, 100.0);
        set_f64(&mut e, 0x0101_7b70, 1000.0);
        set_f64(&mut e, 0x0102_90f8, 1.1);
        set_f64(&mut e, 0x0102_90b0, 0.25);
        set_f64(&mut e, 0x0101_1588, 0.5);
        set_f64(&mut e, 0x0101_de30, 0.75);
        set_f64(&mut e, 0x0102_0758, 10.0);
        set_f64(&mut e, FIFTH_01018A90, f64::from(0.2f32));
        (e, manager)
    }

    /// A sound with its vtable doubles, a device and (when `buffer`) a buffer object.
    fn sound(e: &mut Engine, buffer: bool) -> (Ptr, u32) {
        let vtable_object = object(e, SOUND_SLOTS);
        let vtable = e.mem.u32(vtable_object);
        let this = e.mem.alloc(0x240);
        e.mem.set_u32(this, vtable);
        let device = object(e, DEVICE_SLOTS);
        e.mem.set_u32(this + DIRECT_SOUND, device);
        let buffer_object = if buffer { object(e, BUFFER_SLOTS) } else { 0 };
        e.mem.set_u32(this + BUFFER, buffer_object);
        (Ptr::new(this), buffer_object)
    }

    /// A buffer `Lock` (slot `0x2c`) handing out `region` (and its length) in the first pair.
    fn lockable(e: &mut Engine, base: u32, region: u32) {
        e.register_double(base + 0x2c, move |e, a| {
            e.mem.set_u32(a[3], region);
            e.mem.set_u32(a[4], a[2].max(0x40));
            0u32.into_ret()
        });
    }

    /// `QueryInterface` (slot 0) storing `result` through its second argument.
    fn queries(e: &mut Engine, base: u32, result: u32) {
        e.register_double(base, move |e, a| {
            e.mem.set_u32(a[2], result);
            0u32.into_ret()
        });
    }

    fn point_doubles(e: &mut Engine) {
        e.register(POINT_SUBTRACT, |e, a| {
            for axis in 0..3 {
                let left = f32::from_bits(e.mem.u32(a[0] + 4 * axis));
                let right = f32::from_bits(e.mem.u32(a[2] + 4 * axis));
                e.mem.set_u32(a[1] + 4 * axis, (left - right).to_bits());
            }
            a[1].into_ret()
        });
        e.register(POINT_LENGTH, |e, a| {
            let squares: f32 = (0..3)
                .map(|axis| f32::from_bits(e.mem.u32(a[0] + 4 * axis)).powi(2))
                .sum();
            f64::from(squares.sqrt()).into_ret()
        });
        e.register(0x00ec_6cde, |_, a| {
            f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32))
                .abs()
                .into_ret()
        });
    }

    #[test]
    fn fn_00aeac20_builds_the_sound() {
        let (mut e, _) = engine();
        e.register(0x00ae_9360, |_, _| Ret::default());
        let this = e.mem.alloc(0x240);
        e.mem.set_u32(this + BUFFER, 5);
        e.mem.set_u32(this + BUFFER_DESC + 8, 9);
        let name = e.mem.alloc(0x20);
        put_string(&mut e, name, "fx\\door.wav");
        let result = e.call(0x00ae_ac20, &args![this, name]).u32();
        assert_eq!(result, this);
        assert_eq!(e.mem.u32(this), WIN32_GAME_SOUND_VTABLE);
        assert_eq!(string_at(&e, this + FILENAME), "fx\\door.wav");
        assert_eq!(e.mem.u32(this + BUFFER), 0);
        assert_eq!(e.mem.u32(this + BUFFER_DESC + 8), 0);
        assert_ne!(e.mem.u32(this + WAVE_FORMAT), 0);
        let curve: Vec<u16> = (0..5)
            .map(|i| e.mem.u16(this + ATTENUATION_CURVE + 2 * i))
            .collect();
        assert_eq!(curve, vec![0, 600, 1400, 2600, 5000]);
    }

    #[test]
    fn fn_00aeafb0_releases_and_frees() {
        let (mut e, _) = engine();
        e.register(0x00ae_93f0, |_, _| Ret::default());
        let (this, _) = sound(&mut e, true);
        let this = this.addr();
        let counts = Rc::new(RefCell::new(vec![2u32, 1, 0]));
        let next = counts.clone();
        e.register_double(BUFFER_SLOTS + 8, move |_, _| {
            next.borrow_mut().remove(0).into_ret()
        });
        let second = object(&mut e, SECOND_SLOTS);
        e.mem.set_u32(this + SECOND_BUFFER, second);
        e.mem.set_u32(SECOND_BUFFER_COUNT_011F7128, 3);
        e.mem.set_u32(this + WAVE_FORMAT, 0x40);
        e.mem.set_u32(this + SAMPLE_DATA, 0x44);
        let stream = e.mem.alloc(0x24);
        for (offset, block) in [(0x18, 0x48), (0x1c, 0x4c), (0x20, 0x50)] {
            e.mem.set_u32(stream + offset, block);
        }
        e.mem.set_u32(this + STREAM, stream);
        let reader = e.mem.alloc(0x2e0);
        e.mem.set_u32(this + OGG_READER, reader);
        e.call_log = Some(vec![]);
        e.call(0x00ae_afb0, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, BUFFER_SLOTS + 8).len(), 3);
        assert_eq!(calls(&log, SECOND_SLOTS + 8).len(), 1);
        assert_eq!(e.mem.u32(SECOND_BUFFER_COUNT_011F7128), 2);
        let freed: Vec<u32> = calls(&log, FREE).iter().map(|a| a[1]).collect();
        assert_eq!(freed, vec![0x40, 0x44, 0x48, 0x50, 0x4c, stream, reader]);
        assert_eq!(e.mem.u32(this + BUFFER), 0);
        assert_eq!(e.mem.u32(this + OGG_READER), 0);
        assert_eq!(calls(&log, 0x00ae_93f0), vec![vec![this]]);
    }

    #[test]
    fn fn_00af1cc0_frees_with_bit_0() {
        let (mut e, _) = engine();
        e.register(0x00ae_93f0, |_, _| Ret::default());
        let (this, _) = sound(&mut e, false);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00af_1cc0, &args![this, 0u32]).u32(), this.addr());
        assert_eq!(e.call(0x00af_1cc0, &args![this, 1u32]).u32(), this.addr());
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, FREE), vec![vec![MEMORY_MANAGER, this.addr()]]);
    }

    #[test]
    fn fn_00af1ba0_takes_the_device() {
        let (mut e, manager) = engine();
        let this: Ptr = Ptr::new(e.mem.alloc(0x240));
        assert_eq!(e.call(0x00af_1ba0, &args![this]).u8(), 0);
        e.mem.set_u32(manager + 0x38, 0x99);
        assert_eq!(e.call(0x00af_1ba0, &args![this]).u8(), 1);
        assert_eq!(e.mem.u32(this.addr() + DIRECT_SOUND), 0x99);
    }

    #[test]
    fn fn_00af13f0_sets_and_clears_inaudible() {
        let (mut e, _) = engine();
        let this = e.mem.alloc(0x240);
        e.mem.set_u32(this + STATE_FLAGS, 1);
        e.call(0x00af_13f0, &args![this, 0x100u32 | 1]);
        assert_eq!(e.mem.u32(this + STATE_FLAGS), 0x20001);
        e.call(0x00af_13f0, &args![this, 0x100u32]);
        assert_eq!(e.mem.u32(this + STATE_FLAGS), 1);
    }

    #[test]
    fn fn_00aefcf0_forwards_the_point() {
        let (mut e, _) = engine();
        let (this, _) = sound(&mut e, false);
        e.call_log = Some(vec![]);
        e.call(0x00ae_fcf0, &args![this, 1.0f32, 2.0f32, 3.0f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, SOUND_SLOTS + 0x50),
            vec![vec![
                this.addr(),
                1.0f32.to_bits(),
                2.0f32.to_bits(),
                3.0f32.to_bits()
            ]]
        );
    }

    #[test]
    fn fn_00aefd20_copies_a_3d_position() {
        let (mut e, _) = engine();
        let this = e.mem.alloc(0x240);
        for axis in 0..3 {
            e.mem.set_u32(this + POSITION + 4 * axis, 10 + axis);
        }
        let result = e.mem.alloc(0xc);
        e.mem.set_u32(result, 7);
        assert_eq!(e.call(0x00ae_fd20, &args![this, result]).u32(), result);
        assert_eq!(e.mem.u32(result), 0);
        e.register(IS_3D_OTHER, |_, _| 1u32.into_ret());
        e.call(0x00ae_fd20, &args![this, result]);
        assert_eq!(
            (0..3)
                .map(|i| e.mem.u32(result + 4 * i))
                .collect::<Vec<_>>(),
            vec![10, 11, 12]
        );
    }

    #[test]
    fn fn_00af1c00_fills_a_region() {
        let (mut e, _) = engine();
        let (this, buffer) = sound(&mut e, true);
        let region = e.mem.alloc(0x40);
        lockable(&mut e, BUFFER_SLOTS, region);
        let data = e.mem.alloc(0x10);
        put_string(&mut e, data, "abcdefgh");
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00af_1c00, &args![this, 2u32, 0x10u32, 3u32, data])
                .u8(),
            1
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, BUFFER_SLOTS + 0x2c)[0][1..3], [0x10, 3]);
        assert_eq!(string_at(&e, region), "cde");
        assert_eq!(
            calls(&log, BUFFER_SLOTS + 0x4c),
            vec![vec![buffer, region, 0x40, 0, 0]]
        );
        e.call(0x00af_1c00, &args![this, 0u32, 0u32, 2u32, 0u32]);
        assert_eq!(string_at(&e, region), "");
        e.register(BUFFER_SLOTS + 0x2c, |_, _| E_FAIL.into_ret());
        assert_eq!(
            e.call(0x00af_1c00, &args![this, 0u32, 0u32, 2u32, 0u32])
                .u8(),
            0
        );
    }

    #[test]
    fn fn_00aed420_pauses() {
        let (mut e, _) = engine();
        let (this, _) = sound(&mut e, true);
        let second = object(&mut e, SECOND_SLOTS);
        e.mem.set_u32(this.addr() + SECOND_BUFFER, second);
        e.mem.set_u32(this.addr() + STATE_FLAGS, 0x21);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_d420, &args![this]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SECOND_SLOTS + 0x48), vec![vec![second]]);
        assert_eq!(e.mem.u32(this.addr() + STATE_FLAGS), 0x41);
        let (other, _) = sound(&mut e, false);
        assert_eq!(e.call(0x00ae_d420, &args![other]).u32(), 0);
        assert_eq!(e.mem.u32(other.addr() + STATE_FLAGS), 0);
    }

    #[test]
    fn fn_00aed4c0_stops_or_parks_the_stream() {
        let (mut e, _) = engine();
        let (this, buffer) = sound(&mut e, true);
        e.mem.set_u32(this.addr() + STATE_FLAGS, 0x60);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_d4c0, &args![this]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, BUFFER_SLOTS + 0x34), vec![vec![buffer, 0]]);
        assert_eq!(e.mem.u32(this.addr() + STATE_FLAGS), 0);
        let stream = e.mem.alloc(0x24);
        e.mem.set_u32(stream, 3);
        e.mem.set_u32(this.addr() + STREAM, stream);
        e.mem.set_u32(this.addr() + SOUND_TYPE_FLAGS, 0x0200_0000);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_d4c0, &args![this]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(stream), 4);
        assert!(calls(&log, BUFFER_SLOTS + 0x48).is_empty());
    }

    #[test]
    fn fn_00aed080_plays() {
        let (mut e, _) = engine();
        e.register(AFTER_PLAY, |_, _| Ret::default());
        let (this, buffer) = sound(&mut e, true);
        e.mem.set_u32(this.addr() + STATE_FLAGS, 0x40);
        e.mem
            .set_u32(this.addr() + CURRENT_VOLUME, 0.5f32.to_bits());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_d080, &args![this, 0x100u32]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, BUFFER_SLOTS + 0x30),
            vec![vec![buffer, 0, 0, 0]]
        );
        assert_eq!(e.mem.u32(this.addr() + STATE_FLAGS), 0x20);
        assert_eq!(
            calls(&log, SOUND_SLOTS + 0x3c),
            vec![vec![this.addr(), 0.5f32.to_bits()]]
        );
        assert!(calls(&log, SOUND_SLOTS + 0x44).is_empty());
        // A random start loops.
        e.mem.set_u32(this.addr() + SOUND_TYPE_FLAGS, 0x10);
        e.register(0x00ec_adb8, |_, _| 7u32.into_ret());
        e.mem.set_u32(this.addr() + SOUND_SIZE, 5);
        e.call_log = Some(vec![]);
        e.call(0x00ae_d080, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, BUFFER_SLOTS + 0x34), vec![vec![buffer, 3]]);
        assert_eq!(
            calls(&log, BUFFER_SLOTS + 0x30),
            vec![vec![buffer, 0, 0, 1]]
        );
    }

    #[test]
    fn fn_00aed080_fills_a_stream() {
        let (mut e, _) = engine();
        e.register(AFTER_PLAY, |_, _| Ret::default());
        let (this, _) = sound(&mut e, true);
        let region = e.mem.alloc(0x40);
        lockable(&mut e, BUFFER_SLOTS, region);
        e.mem.set_u32(this.addr() + SOUND_TYPE_FLAGS, 0x0400_0000);
        e.mem.set_u32(this.addr() + DATA_BYTES, 4);
        let stream = e.mem.alloc(0x24);
        let intro = e.mem.alloc(0x10);
        put_string(&mut e, intro, "INTRO");
        e.mem.set_u32(stream + 8, 6);
        e.mem.set_u32(stream + 0x18, intro);
        e.mem.set_u32(this.addr() + STREAM, stream);
        e.call(0x00ae_d080, &args![this, 0u32]);
        assert_eq!(e.mem.u32(stream), 1);
        assert_eq!(e.mem.u32(stream + 4), 4);
        assert_eq!(string_at(&e, region), "INTR");
    }

    #[test]
    fn fn_00aed660_sets_hundredths_of_decibels() {
        let (mut e, manager) = engine();
        let (this, buffer) = sound(&mut e, true);
        set_f32(&mut e, 0x0107_18c0, 0.0);
        set_f64(&mut e, 0x0101_dae8, 0.0001);
        set_f64(&mut e, 0x0107_3490, 2000.0);
        set_f64(&mut e, 0x0102_d268, -10000.0);
        e.register_double(0x00ad_9060, move |_, _| manager.into_ret());
        e.register(0x00ad_ac30, |_, a| {
            f64::from(f32::from_bits(a[1])).into_ret()
        });
        e.register(0x00ec_bc80, |_, a| {
            f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32))
                .log10()
                .into_ret()
        });
        e.mem.set_u16(this.addr() + STATIC_ATTENUATION, 100);
        e.mem.set_u16(this.addr() + REVERB_ATTENUATION, 10);
        let second = object(&mut e, SECOND_SLOTS);
        e.mem.set_u32(this.addr() + SECOND_BUFFER, second);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_d660, &args![this, 0.5f32]).u8(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr() + CURRENT_VOLUME), 0.5f32.to_bits());
        assert_eq!(
            calls(&log, BUFFER_SLOTS + 0x3c),
            vec![vec![buffer, (-702i32) as u32]]
        );
        assert_eq!(
            calls(&log, SECOND_SLOTS + 0x3c),
            vec![vec![second, (-1702i32) as u32]]
        );
        // Muted.
        e.mem.set_u32(this.addr() + STATE_FLAGS, 0x400);
        e.call_log = Some(vec![]);
        e.call(0x00ae_d660, &args![this, 0.5f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, BUFFER_SLOTS + 0x3c),
            vec![vec![buffer, (-10000i32) as u32]]
        );
    }

    #[test]
    fn fn_00aefda0_sets_the_distances() {
        let (mut e, _) = engine();
        e.register(IS_3D, |_, _| 1u32.into_ret());
        let (this, _) = sound(&mut e, true);
        let buffer_3d = object(&mut e, BUFFER_3D_SLOTS);
        e.mem.set_u32(this.addr() + BUFFER_3D, buffer_3d);
        e.mem.set_u8(this.addr() + POSITION_SET, 1);
        set_f32(&mut e, 0x010a_3dcc, 1.0e9);
        e.call_log = Some(vec![]);
        e.call(0x00ae_fda0, &args![this, 5.0f32, 50.0f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr() + MIN_DISTANCE), 5.0f32.to_bits());
        assert_eq!(e.mem.u32(this.addr() + MAX_DISTANCE), 50.0f32.to_bits());
        assert_eq!(calls(&log, SOUND_SLOTS + 0x4c).len(), 1);
        let far = 1.0e9f32.to_bits();
        assert_eq!(
            calls(&log, BUFFER_3D_SLOTS + 0x44),
            vec![vec![buffer_3d, far, 0]]
        );
        assert_eq!(
            calls(&log, BUFFER_3D_SLOTS + 0x40),
            vec![vec![buffer_3d, far, 0]]
        );
        // Unchanged distances do not move the sound again.
        e.call_log = Some(vec![]);
        e.call(0x00ae_fda0, &args![this, 5.0f32, 50.0f32]);
        let log = e.call_log.take().unwrap();
        assert!(calls(&log, SOUND_SLOTS + 0x4c).is_empty());
    }

    #[test]
    fn fn_00aeff60_builds_the_curve() {
        let (mut e, _) = engine();
        set_f64(&mut e, 0x010a_3e68, 2000.0);
        set_f64(&mut e, 0x010a_3e60, -10000.0);
        e.register(0x00ec_bc80, |_, a| {
            f64::from_bits(u64::from(a[0]) | (u64::from(a[1]) << 32))
                .log10()
                .into_ret()
        });
        let this = e.mem.alloc(0x240);
        e.call(
            0x00ae_ff60,
            &args![this, 100u32, 0x1_0000u32 | 1000, 0u32, 100u32, 100u32],
        );
        let curve: Vec<u16> = (0..5)
            .map(|i| e.mem.u16(this + ATTENUATION_CURVE + 2 * i))
            .collect();
        assert_eq!(curve, vec![0, 2000, (-10000i16) as u16, 0, 0]);
    }

    #[test]
    fn fn_00af01e0_sets_a_clamped_frequency() {
        let (mut e, _) = engine();
        let (this, buffer) = sound(&mut e, true);
        e.mem.set_u16(this.addr() + MOD_SAMPLES_PER_SECOND, 22050);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00af_01e0, &args![this, 2.0f32]).u32(), 1);
        e.call(0x00af_01e0, &args![this, 100.0f32]);
        e.call(0x00af_01e0, &args![this, 0.0f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, BUFFER_SLOTS + 0x44),
            vec![
                vec![buffer, 44100],
                vec![buffer, 200_000],
                vec![buffer, 100]
            ]
        );
    }

    #[test]
    fn fn_00af0290_reads_the_ratio() {
        let (mut e, _) = engine();
        let (this, _) = sound(&mut e, true);
        e.mem.set_u16(this.addr() + MOD_SAMPLES_PER_SECOND, 22050);
        e.register(BUFFER_SLOTS + 0x20, |e, a| {
            e.mem.set_u32(a[1], 44100);
            0u32.into_ret()
        });
        assert_eq!(e.call(0x00af_0290, &args![this]).f64(), 2.0);
        e.register(BUFFER_SLOTS + 0x20, |_, _| E_FAIL.into_ret());
        assert_eq!(e.call(0x00af_0290, &args![this]).f64(), 1.0);
    }

    #[test]
    fn fn_00af0320_seeks_or_complains() {
        let (mut e, _) = engine();
        let (this, buffer) = sound(&mut e, true);
        e.mem.set_u32(this.addr() + DURATION, 1000);
        e.register(BUFFER_SLOTS + 0x14, |e, a| {
            e.mem.set_u32(a[1] + 8, 88200);
            0u32.into_ret()
        });
        e.call_log = Some(vec![]);
        e.call(0x00af_0320, &args![this, 500u32]);
        e.call(0x00af_0320, &args![this, 1000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, BUFFER_SLOTS + 0x34), vec![vec![buffer, 44000]]);
        let printed = calls(&log, PRINT);
        assert_eq!(printed.len(), 1);
        assert_eq!(printed[0][0], 0x010a_3e70);
        assert_eq!(
            f64::from_bits(u64::from(printed[0][1]) | (u64::from(printed[0][2]) << 32)),
            1.0
        );
    }

    fn strstr_double(e: &mut Engine) {
        e.register(STRSTR, |e, a| {
            let text = string_at(e, a[0]);
            let needle = string_at(e, a[1]);
            match text.find(&needle) {
                Some(at) => (a[0] + at as u32).into_ret(),
                None => 0u32.into_ret(),
            }
        });
    }

    #[test]
    fn fn_00aeb270_opens_a_wave() {
        let (mut e, _) = engine();
        strstr_double(&mut e);
        put_string(&mut e, MP3_01033AD8, ".mp3");
        put_string(&mut e, OGG_010A3DBC, ".ogg");
        let (this, _) = sound(&mut e, false);
        let base = this.addr();
        put_string(&mut e, base + FILENAME, "a.mp3");
        let format = e.mem.alloc(0x12);
        e.mem.set_u32(base + WAVE_FORMAT, format);
        e.mem.set_u32(base + SOUND_TYPE_FLAGS, 1);
        e.register(MMIO_OPEN, |_, _| 0x55u32.into_ret());
        e.register(MMIO_DESCEND, |e, a| {
            match e.mem.u32(a[1]) {
                0x2074_6d66 => e.mem.set_u32(a[1] + 4, 0x10),
                0x6174_6164 => e.mem.set_u32(a[1] + 4, 8),
                _ => {}
            }
            0u32.into_ret()
        });
        e.register(MMIO_READ, |e, a| {
            e.mem.set_u16(a[1], 1);
            e.mem.set_u16(a[1] + 2, 1);
            e.mem.set_u32(a[1] + 4, 22050);
            e.mem.set_u16(a[1] + 0xe, 16);
            0x10u32.into_ret()
        });
        e.register(MMIO_ASCEND, |_, _| Ret::default());
        e.register(MMIO_CLOSE, |_, _| Ret::default());
        let source = e.mem.alloc(0x10);
        put_string(&mut e, source, "SAMPLES!");
        e.register_double(MMIO_GET_INFO, move |e, a| {
            e.mem.set_u32(a[1] + 0x1c, source);
            e.mem.set_u32(a[1] + 0x20, source + 8);
            0u32.into_ret()
        });
        e.register(MMIO_ADVANCE, |_, _| 1u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_b270, &args![this]).u32(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(string_at(&e, base + FILENAME), "a.wav");
        assert_eq!(e.mem.u32(base + STATE_FLAGS), 0x1000);
        assert_eq!(e.mem.u32(base + BUFFER_FLAGS), 0xe0);
        // 8 bytes, 250 ms of padding (11000) and half a second of tail (22000).
        assert_eq!(e.mem.u32(base + DATA_BYTES), 33008);
        assert_eq!(e.mem.u32(base + DURATION), 33008 / 2 / 22);
        assert_eq!(e.mem.u16(base + SAMPLES_PER_SECOND), 22050);
        assert_eq!(string_at(&e, e.mem.u32(base + SAMPLE_DATA)), "SAMPLES!");
        assert_eq!(calls(&log, MMIO_CLOSE).len(), 1);
        assert_eq!(calls(&log, SET_OPENED), vec![vec![base, 1]]);
    }

    #[test]
    fn fn_00aeb270_fails_without_a_device() {
        let (mut e, _) = engine();
        let this = e.mem.alloc(0x240);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_b270, &args![this]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SET_OPENED), vec![vec![this, 0]]);
    }

    fn ogg_name_doubles(e: &mut Engine) {
        strstr_double(e);
        e.register(0x00ec_6370, |e, a| {
            let text = string_at(e, a[1]);
            put_string(e, a[0], &text);
            a[0].into_ret()
        });
        e.register(0x00ec_67aa, |_, a| {
            u32::from((a[0] as u8).to_ascii_lowercase()).into_ret()
        });
        put_string(e, MP3_01033AD8, ".mp3");
        put_string(e, OGG_010A3DBC, ".ogg");
        put_string(e, WAV_0107CE2C, ".wav");
    }

    #[test]
    fn fn_00aebff0_drops_a_reader_that_fails() {
        let (mut e, _) = engine();
        ogg_name_doubles(&mut e);
        let (this, _) = sound(&mut e, false);
        put_string(&mut e, this.addr() + FILENAME, "a.txt");
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_bff0, &args![this]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(this.addr() + OGG_READER), 0);
        assert_eq!(calls(&log, FREE).len(), 1);
        assert!(calls(&log, SET_OPENED).is_empty());
    }

    #[test]
    fn fn_00aebff0_reads_a_stream() {
        let (mut e, _) = engine();
        ogg_name_doubles(&mut e);
        let (this, _) = sound(&mut e, false);
        let base = this.addr();
        put_string(&mut e, base + FILENAME, "a.wav");
        let format = e.mem.alloc(0x12);
        e.mem.set_u32(base + WAVE_FORMAT, format);
        e.mem.set_u32(base + SOUND_TYPE_FLAGS, 0x0200_1000);
        let file = object(&mut e, OTHER_SLOTS);
        e.mem.set_u8(file + 0x2c, 1);
        e.register_double(0x00af_df20, move |_, _| file.into_ret());
        e.register(0x00ee_81fe, |_, _| 0u32.into_ret());
        let info = e.mem.alloc(0x10);
        e.mem.set_u32(info + 4, 1);
        e.mem.set_u32(info + 8, 44100);
        e.register_double(0x00ee_81f8, move |_, _| info.into_ret());
        e.register(0x00ee_81f2, |_, _| 1u32.into_ret());
        e.register(0x00ee_81ec, |_, _| 44100u32.into_ret());
        e.register(0x00ee_8204, |_, a| a[2].min(0x1000).into_ret());
        assert_eq!(e.call(0x00ae_bff0, &args![this]).u32(), 1);
        assert_eq!(e.mem.u32(base + SOUND_TYPE_FLAGS), 0x1010);
        assert_eq!(e.mem.u32(base + DATA_BYTES), 88200);
        assert_eq!(e.mem.u32(base + DURATION), 1002);
        assert_eq!(e.mem.u32(format + 8), 88200);
        assert_eq!(e.mem.u32(base + BUFFER_FLAGS), 0x30010 | 0x2a0);
    }

    #[test]
    fn fn_00af0400_handles_the_messages() {
        let (mut e, _) = engine();
        let info = e.mem.alloc(0x48);
        let file = object(&mut e, OTHER_SLOTS);
        e.mem.set_u8(file + 0x2c, 1);
        e.register_double(0x00af_df20, move |_, _| file.into_ret());
        assert_eq!(
            e.call(0x00af_0400, &args![info, 3u32, 0x10u32, 0u32]).u32(),
            0
        );
        assert_eq!(e.mem.u32(info + 0x30), file);
        e.register(0x7f00_0000, |_, a| a[2].into_ret());
        e.mem.set_u32(file + 8, 0x7f00_0000);
        assert_eq!(
            e.call(0x00af_0400, &args![info, 0u32, 0x20u32, 6u32]).u32(),
            6
        );
        assert_eq!(e.mem.u32(info + 0x2c), 6);
        e.mem.set_u32(0x010a_2488, 9);
        e.mem.set_u32(file + 0x38, u32::MAX);
        e.mem.set_u32(file + 0x150, 77);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00af_0400, &args![info, 2u32, 4u32, 2u32]).u32(), 0);
        assert_eq!(e.call(0x00af_0400, &args![info, 4u32, 0u32, 0u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, OTHER_SLOTS + 0x14), vec![vec![file, 4, 9]]);
        assert_eq!(calls(&log, OTHER_SLOTS), vec![vec![file, 1]]);
        assert_eq!(e.mem.u32(info + 0x2c), 77);
        assert_eq!(
            e.call(0x00af_0400, &args![info, 1u32, 0u32, 0u32]).u32(),
            0x10c
        );
        e.mem.set_u32(info + 0x30, 0);
        assert_eq!(
            e.call(0x00af_0400, &args![info, 0u32, 0u32, 0u32]).u32(),
            0x105
        );
        assert_eq!(
            e.call(0x00af_0400, &args![info, 4u32, 0u32, 0u32]).u32(),
            0x104
        );
    }

    #[test]
    fn fn_00af1d20_and_00af1d80_build_and_close_a_reader() {
        let (mut e, _) = engine();
        let reader = e.mem.alloc(0x2e0);
        e.mem.set_u32(reader + 0x10, 5);
        e.mem.set_u32(reader + 0x2d4, 5);
        assert_eq!(e.call(0x00af_1d20, &args![reader]).u32(), reader);
        assert_eq!(e.mem.u32(reader + 0x10), 0);
        assert_eq!(e.mem.u32(reader + 0x2d4), 0);
        let file = object(&mut e, OTHER_SLOTS);
        e.mem.set_u32(reader + 0x2d0, file);
        e.mem.set_u8(reader + 0x2dc, 1);
        e.register(0x00ee_81e6, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x00af_1d80, &args![reader]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, 0x00ee_81e6), vec![vec![reader]]);
        assert_eq!(calls(&log, OTHER_SLOTS), vec![vec![file, 1]]);
        assert_eq!(e.mem.u32(reader + 0x2d0), 0);
    }

    #[test]
    fn fn_00af1e00_opens_the_ogg_name() {
        let (mut e, _) = engine();
        ogg_name_doubles(&mut e);
        let reader = e.mem.alloc(0x2e0);
        let name = e.mem.alloc(0x110);
        put_string(&mut e, name, "Sound\\A.WAV");
        let file = object(&mut e, OTHER_SLOTS);
        e.mem.set_u8(file + 0x2c, 1);
        let opened = Rc::new(RefCell::new(String::new()));
        let seen = opened.clone();
        e.register_double(0x00af_df20, move |e, a| {
            *seen.borrow_mut() = string_at(e, a[0]);
            file.into_ret()
        });
        e.register(0x00ee_81fe, |_, _| 0u32.into_ret());
        e.register(0x00ee_81f8, |_, _| 0x1234u32.into_ret());
        e.register(0x00ee_81f2, |_, _| 0u32.into_ret());
        assert_eq!(e.call(0x00af_1e00, &args![reader, name]).u8(), 1);
        assert_eq!(*opened.borrow(), "sound\\a.ogg");
        assert_eq!(e.mem.u8(reader + 0x2dc), 1);
        assert_eq!(e.mem.u32(reader + 0x2d8), 0x1234);
        assert_eq!(e.mem.u32(reader + 0x2d4), 0);
        // A failed open closes and drops the file.
        e.register(0x00ee_81fe, |_, _| 1u32.into_ret());
        e.register(0x00af_fd10, |_, _| Ret::default());
        assert_eq!(e.call(0x00af_1e00, &args![reader, name]).u8(), 0);
        assert_eq!(e.mem.u32(reader + 0x2d0), 0);
    }

    #[test]
    fn fn_00af2100_reads_until_done() {
        let (mut e, _) = engine();
        let reads = Rc::new(RefCell::new(vec![4u32, 4, 0]));
        let next = reads.clone();
        e.register_double(0x00ee_8204, move |_, _| {
            next.borrow_mut().remove(0).into_ret()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00af_2100, &args![0x10u32, 0x100u32, 8u32]).u32(),
            8
        );
        let log = e.call_log.take().unwrap();
        let read = calls(&log, 0x00ee_8204);
        assert_eq!(read.len(), 3);
        assert_eq!(read[1][1..6], [0x104, 4, 0, 2, 1]);
    }

    #[test]
    fn fn_00af2180_00af2200_00af2290_are_the_callbacks() {
        let (mut e, _) = engine();
        let file = object(&mut e, OTHER_SLOTS);
        e.register(0x7f00_0000, |_, a| a[2].into_ret());
        e.mem.set_u32(file + 8, 0x7f00_0000);
        e.mem.set_u32(file + 4, 10);
        assert_eq!(
            e.call(0x00af_2180, &args![0x20u32, 3u32, 4u32, file]).u32(),
            12
        );
        assert_eq!(e.mem.u32(file + 4), 22);
        assert_eq!(
            e.call(0x00af_2180, &args![0x20u32, 3u32, 4u32, 0u32]).u32(),
            0
        );
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00af_2200, &args![file, 5u32, 0u32, 1u32]).u32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, OTHER_SLOTS + 0x14), vec![vec![file, 5, 1]]);
        assert_eq!(
            e.call(0x00af_2200, &args![0u32, 5u32, 0u32, 1u32]).u32(),
            u32::MAX
        );
        e.mem.set_u32(file + 0x38, 40);
        assert_eq!(e.call(0x00af_2290, &args![file]).u32(), 40);
        e.mem.set_u32(file + 0x38, u32::MAX);
        e.mem.set_u32(file + 0x150, 41);
        assert_eq!(e.call(0x00af_2290, &args![file]).u32(), 41);
        assert_eq!(e.call(0x00af_2290, &args![0u32]).u32(), 0);
    }

    #[test]
    fn fn_00aec530_prepares_the_buffer() {
        let (mut e, _) = engine();
        e.register(0x00ae_9de0, |_, _| Ret::default());
        let (this, _) = sound(&mut e, false);
        let base = this.addr();
        let format = e.mem.alloc(0x12);
        e.mem.set_u16(format + 0xe, 16);
        e.mem.set_u32(base + WAVE_FORMAT, format);
        let data = e.mem.alloc(0x10);
        put_string(&mut e, data, "WAVE");
        e.mem.set_u32(base + SAMPLE_DATA, data);
        e.mem.set_u32(base + DATA_BYTES, 4);
        e.mem.set_u32(base + DURATION, 10);
        let created = object(&mut e, CREATED_SLOTS);
        let buffer = object(&mut e, BUFFER_SLOTS);
        e.register_double(DEVICE_SLOTS + 0xc, move |e, a| {
            e.mem.set_u32(a[2], created);
            0u32.into_ret()
        });
        queries(&mut e, CREATED_SLOTS, buffer);
        let region = e.mem.alloc(0x40);
        lockable(&mut e, BUFFER_SLOTS, region);
        e.call_log = Some(vec![]);
        e.call(0x00ae_c530, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(base + BUFFER), buffer);
        assert_eq!(string_at(&e, region), "WAVE");
        assert_eq!(e.mem.u32(base + SAMPLE_DATA), 0);
        assert_eq!(calls(&log, FREE), vec![vec![MEMORY_MANAGER, data]]);
        assert_eq!(calls(&log, SOUND_SLOTS + 0x3c).len(), 1);
        assert_eq!(calls(&log, 0x00ae_9de0), vec![vec![base]]);
        // Without a device the prepare fails.
        e.mem.set_u32(base + DIRECT_SOUND, 0);
        e.call_log = Some(vec![]);
        e.call(0x00ae_c530, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SET_OPENED), vec![vec![base, 0]]);
        assert!(calls(&log, 0x00ae_9de0).is_empty());
    }

    #[test]
    fn fn_00aec530_answers_the_call_it_ends_on() {
        let (mut e, _) = engine();
        e.register(0x00ae_9de0, |_, _| 0x0000_0301u32.into_ret());
        let (this, _) = sound(&mut e, false);
        let base = this.addr();
        let format = e.mem.alloc(0x12);
        e.mem.set_u16(format + 0xe, 16);
        e.mem.set_u32(base + WAVE_FORMAT, format);
        e.mem.set_u32(base + DURATION, 10);
        // No sample data: straight to `BSGameSound::Prepare`, whose `AL` is the answer.
        assert!(e.call(0x00ae_c530, &args![this]).bool());
        // Without a device: `00ae9c50(0)`'s `AL`.
        e.register(SET_OPENED, |_, _| 0x0000_0201u32.into_ret());
        e.mem.set_u32(base + DIRECT_SOUND, 0);
        assert!(e.call(0x00ae_c530, &args![this]).bool());
        e.register(SET_OPENED, |_, a| a[1].into_ret());
        assert!(!e.call(0x00ae_c530, &args![this]).bool());
    }

    #[test]
    fn fn_00af1430_adds_the_reverb_buffer() {
        let (mut e, manager) = engine();
        let (this, _) = sound(&mut e, true);
        let base = this.addr();
        assert_eq!(e.call(0x00af_1430, &args![this]).u8(), 0);
        e.mem.set_u8(0x011f_7134, 1);
        e.mem.set_u32(0x011f_7140, 4);
        let environment = object(&mut e, LISTENER_SLOTS);
        e.register(LISTENER_SLOTS + 0x2c, |_, _| 3u32.into_ret());
        e.mem.set_u32(manager + 0x10, environment);
        e.mem.set_u32(base + DATA_BYTES, 8);
        e.mem.set_u32(base + BUFFER_BYTES, 8);
        let created = object(&mut e, CREATED_SLOTS);
        let second = object(&mut e, SECOND_SLOTS);
        let effect = object(&mut e, OTHER_SLOTS);
        e.register_double(DEVICE_SLOTS + 0xc, move |e, a| {
            e.mem.set_u32(a[2], created);
            0u32.into_ret()
        });
        queries(&mut e, CREATED_SLOTS, second);
        e.register_double(SECOND_SLOTS + 0x5c, move |e, a| {
            e.mem.set_u32(a[4], effect);
            0u32.into_ret()
        });
        let new_region = e.mem.alloc(0x40);
        let old_region = e.mem.alloc(0x40);
        put_string(&mut e, old_region, "REVERB!!");
        lockable(&mut e, SECOND_SLOTS, new_region);
        lockable(&mut e, BUFFER_SLOTS, old_region);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00af_1430, &args![this]).u8(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(SECOND_BUFFER_COUNT_011F7128), 1);
        assert_eq!(e.mem.u32(base + SECOND_BUFFER), second);
        assert_eq!(e.mem.u32(base + REVERB_EFFECT), effect);
        assert_eq!(calls(&log, OTHER_SLOTS + 0x14), vec![vec![effect, 2]]);
        assert_eq!(string_at(&e, new_region), "REVERB!!");
        // The description: flags 0x30010 | 0x2a0 and a quarter more bytes.
        let description = calls(&log, DEVICE_SLOTS + 0xc)[0][1];
        assert_eq!(e.mem.u32(description + 4), 0x302b0);
        assert_eq!(e.mem.u32(description + 8), 10);
        assert_eq!(calls(&log, FREE).len(), 1);
    }

    #[test]
    fn fn_00aef090_places_the_sound() {
        let (mut e, manager) = engine();
        point_doubles(&mut e);
        e.register(IS_3D, |_, _| 1u32.into_ret());
        set_f64(&mut e, UNIT_SCALE_010A3BE8, 0.5);
        let (this, _) = sound(&mut e, true);
        let base = this.addr();
        let buffer_3d = object(&mut e, BUFFER_3D_SLOTS);
        e.mem.set_u32(base + BUFFER_3D, buffer_3d);
        set_f32(&mut e, base + MAX_DISTANCE, 100.0);
        set_f32(&mut e, base + MIN_DISTANCE, 10.0);
        for (index, value) in [0u16, 600, 1400, 2600, 5000].iter().enumerate() {
            e.mem
                .set_u16(base + ATTENUATION_CURVE + 2 * index as u32, *value);
        }
        let listener = object(&mut e, LISTENER_SLOTS);
        let heard = e.mem.alloc(0xc);
        e.register_double(LISTENER_SLOTS + 0xc, move |_, _| heard.into_ret());
        e.mem.set_u32(manager + 0x10, listener);
        e.call_log = Some(vec![]);
        e.call(0x00ae_f090, &args![this, 50.0f32, 2.0f32, 4.0f32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(base + POSITION), 50.0f32.to_bits());
        assert_eq!(e.mem.u8(base + POSITION_SET), 1);
        assert_eq!(
            calls(&log, BUFFER_3D_SLOTS + 0x4c),
            vec![vec![
                buffer_3d,
                25.0f32.to_bits(),
                2.0f32.to_bits(),
                1.0f32.to_bits(),
                1
            ]]
        );
        let distance = f32::from_bits(e.mem.u32(base + DISTANCE_TO_LISTENER));
        assert!((distance - 50.199_6).abs() < 0.001);
        // Between the first two quarter points: 600 + (50.2 - 32.5) / 22.5 * 800.
        assert_eq!(e.mem.u16(base + DISTANCE_ATTENUATION), 1229);
        assert_eq!(calls(&log, SOUND_SLOTS + 0x3c).len(), 2);
    }

    #[test]
    fn fn_00aed990_follows_the_listener() {
        let (mut e, manager) = engine();
        point_doubles(&mut e);
        e.register(IS_3D, |_, _| 1u32.into_ret());
        set_f64(&mut e, UNIT_SCALE_010A3BE8, 1.0);
        let (this, _) = sound(&mut e, true);
        let base = this.addr();
        e.mem.set_u32(base + SOUND_TYPE_FLAGS, 0x4_0000);
        let buffer_3d = object(&mut e, BUFFER_3D_SLOTS);
        e.mem.set_u32(base + BUFFER_3D, buffer_3d);
        set_f32(&mut e, base + MAX_DISTANCE, 100.0);
        set_f32(&mut e, base + MIN_DISTANCE, 10.0);
        set_f32(&mut e, base + POSITION, 200.0);
        let listener = object(&mut e, LISTENER_SLOTS);
        let heard = e.mem.alloc(0xc);
        set_f32(&mut e, heard, 3.0);
        e.register_double(LISTENER_SLOTS + 0xc, move |_, _| heard.into_ret());
        e.mem.set_u32(manager + 0x10, listener);
        e.call_log = Some(vec![]);
        e.call(0x00ae_d990, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, BUFFER_3D_SLOTS + 0x4c),
            vec![vec![buffer_3d, 3.0f32.to_bits(), 0, 0, 1]]
        );
        assert_eq!(e.mem.u32(base + DISTANCE_TO_LISTENER), 197.0f32.to_bits());
        assert_eq!(e.mem.u16(base + DISTANCE_ATTENUATION), 10000);
        assert_eq!(calls(&log, SOUND_SLOTS + 0x3c).len(), 2);
    }

    #[test]
    fn fn_00aee580_feeds_the_stream() {
        let (mut e, _) = engine();
        let (this, buffer) = sound(&mut e, true);
        let base = this.addr();
        e.call_log = Some(vec![]);
        let (other, _) = sound(&mut e, false);
        assert_eq!(e.call(0x00ae_e580, &args![other, 0u32]).u8(), 1);
        e.register(SOUND_SLOTS + 4, |_, _| 1u32.into_ret());
        e.register(BUFFER_SLOTS + 0x24, |e, a| {
            e.mem.set_u32(a[1], 1);
            0u32.into_ret()
        });
        e.register(BUFFER_SLOTS + 0x10, |e, a| {
            e.mem.set_u32(a[1], 40);
            0u32.into_ret()
        });
        let region = e.mem.alloc(0x40);
        lockable(&mut e, BUFFER_SLOTS, region);
        e.mem.set_u32(base + SOUND_TYPE_FLAGS, 0x0400_0000);
        e.mem.set_u32(base + DATA_BYTES, 0x100);
        let stream = e.mem.alloc(0x24);
        e.mem.set_u32(stream, 1);
        e.mem.set_u32(stream + 4, 2);
        e.mem.set_u32(stream + 8, 100);
        e.mem.set_u32(stream + 0x14, 0);
        let intro = e.mem.alloc(0x80);
        e.mem.set_u32(stream + 0x18, intro);
        e.mem.set_u32(base + STREAM, stream);
        assert_eq!(e.call(0x00ae_e580, &args![this, 0u32]).u8(), 1);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SOUND_SLOTS + 0x40).len(), 2);
        assert_eq!(e.mem.u32(base + STATE_FLAGS), 0x20);
        assert_eq!(calls(&log, BUFFER_SLOTS + 0x2c)[0][1..3], [0, 40]);
        assert_eq!(calls(&log, MEMCPY)[0][1], intro + 2);
        assert_eq!(e.mem.u32(stream + 4), 42);
        assert_eq!(e.mem.u32(stream + 0x14), 40);
        assert_eq!(e.mem.u32(stream), 1);
        let _ = buffer;
        // An unmoved cursor answers 0.
        assert_eq!(e.call(0x00ae_e580, &args![this, 0u32]).u8(), 0);
    }

    #[test]
    fn fn_00aee580_resynchronises() {
        let (mut e, manager) = engine();
        let (this, buffer) = sound(&mut e, true);
        let base = this.addr();
        e.register(SOUND_SLOTS + 4, |_, _| 1u32.into_ret());
        e.register(BUFFER_SLOTS + 0x10, |e, a| {
            e.mem.set_u32(a[1], 9600);
            0u32.into_ret()
        });
        e.register(BUFFER_SLOTS + 0x14, |e, a| {
            e.mem.set_u32(a[1] + 8, 96000);
            0u32.into_ret()
        });
        e.mem.set_u32(base + SOUND_TYPE_FLAGS, 0x0100_0000);
        e.mem.set_u32(base + DURATION, 1000);
        e.mem.set_u32(base + SYNCH_START_TIME, 100);
        e.mem.set_u32(manager + 0x14, 300);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_e580, &args![this, 0u32]).u8(), 1);
        let log = e.call_log.take().unwrap();
        // 100 ms played, 200 ms elapsed: 100 ms * 96 bytes ahead of 9600.
        assert_eq!(calls(&log, BUFFER_SLOTS + 0x34), vec![vec![buffer, 19200]]);
        // Paused by the manager: the sound pauses and stays put.
        e.mem.set_u32(manager + 0x1c, 1);
        e.call_log = Some(vec![]);
        e.call(0x00ae_e580, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(e.mem.u32(base + STATE_FLAGS) & 0x10_0000, 0x10_0000);
        assert_eq!(calls(&log, SOUND_SLOTS + 0x34), vec![vec![base]]);
        assert!(calls(&log, BUFFER_SLOTS + 0x34).is_empty());
        // Far past the end: stopped.
        e.mem.set_u32(manager + 0x1c, 0);
        e.mem.set_u32(manager + 0x14, 100_000);
        e.call_log = Some(vec![]);
        e.call(0x00ae_e580, &args![this, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, SOUND_SLOTS + 0x30), vec![vec![base, 0]]);
        assert_eq!(calls(&log, SOUND_SLOTS + 0x38), vec![vec![base]]);
        assert_eq!(e.mem.u32(base + CLEANUP_DELAY), 100);
        assert_eq!(e.mem.u32(base + SOUND_TYPE_FLAGS), 0);
    }

    fn copy_source(e: &mut Engine) -> (Ptr, Ptr) {
        e.register(GAME_SOUND_COPY, |_, _| Ret::default());
        let (this, _) = sound(e, false);
        let (source, _) = sound(e, true);
        let format = e.mem.alloc(0x12);
        e.mem.set_u32(this.addr() + WAVE_FORMAT, format);
        e.register(BUFFER_SLOTS + 0x14, |e, a| {
            e.mem.set_u16(a[1] + 2, 1);
            e.mem.set_u32(a[1] + 4, 22050);
            e.mem.set_u16(a[1] + 0xe, 16);
            0u32.into_ret()
        });
        let base = source.addr();
        e.mem.set_u32(base + DATA_BYTES, 8);
        e.mem.set_u32(base + BUFFER_BYTES, 8);
        e.mem.set_u16(base + SAMPLES_PER_SECOND, 44100);
        e.mem.set_u16(base + MOD_SAMPLES_PER_SECOND, 22050);
        (this, source)
    }

    #[test]
    fn fn_00af0630_copies_the_buffer() {
        let (mut e, _) = engine();
        let (this, source) = copy_source(&mut e);
        let base = this.addr();
        e.mem.set_u32(base + SOUND_TYPE_FLAGS, 2);
        e.mem.set_u32(base + DURATION, 1000);
        let created = object(&mut e, CREATED_SLOTS);
        let buffer = object(&mut e, SECOND_SLOTS);
        let buffer_3d = object(&mut e, BUFFER_3D_SLOTS);
        e.register_double(DEVICE_SLOTS + 0xc, move |e, a| {
            e.mem.set_u32(a[2], created);
            0u32.into_ret()
        });
        queries(&mut e, CREATED_SLOTS, buffer);
        queries(&mut e, SECOND_SLOTS, buffer_3d);
        let new_region = e.mem.alloc(0x40);
        let old_region = e.mem.alloc(0x40);
        put_string(&mut e, old_region, "COPYDATA");
        lockable(&mut e, CREATED_SLOTS, new_region);
        lockable(&mut e, BUFFER_SLOTS, old_region);
        e.mem.set_u32(base + STATE_FLAGS, 0x20);
        e.call_log = Some(vec![]);
        e.call(0x00af_0630, &args![this, source, 1u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            calls(&log, GAME_SOUND_COPY),
            vec![vec![base, source.addr(), 1]]
        );
        assert_eq!(e.mem.u32(base + BUFFER), buffer);
        assert_eq!(e.mem.u32(base + BUFFER_3D), buffer_3d);
        assert_eq!(e.mem.u32(base + BUFFER_FLAGS), 0x302b0);
        assert_eq!(string_at(&e, new_region), "COPYDATA");
        assert_eq!(e.mem.u16(base + SAMPLES_PER_SECOND), 22050);
        // The modified rate keeps its ratio of one half.
        assert_eq!(e.mem.u16(base + MOD_SAMPLES_PER_SECOND), 11025);
        assert_eq!(e.mem.u32(base + DURATION), 8 / 2 / 22);
        assert_eq!(e.mem.u32(base + STATE_FLAGS), 0);
        assert_eq!(calls(&log, SET_OPENED), vec![vec![base, 1]]);
        assert_eq!(calls(&log, SOUND_SLOTS + 0x28), vec![vec![base]]);
    }

    #[test]
    fn fn_00af0630_reports_a_failed_buffer() {
        let (mut e, _) = engine();
        let (this, source) = copy_source(&mut e);
        e.register(DEVICE_SLOTS + 0xc, |_, _| 0x8878_0064u32.into_ret());
        e.call_log = Some(vec![]);
        e.call(0x00af_0630, &args![this, source, 0u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(calls(&log, STRCPY_S)[0][1..3], [0xff, 0x010a_4130]);
        assert_eq!(calls(&log, PRINT).len(), 1);
        assert!(calls(&log, SET_OPENED).is_empty());
        // Short sounds are padded to 250 ms (2 bytes * 44 per ms).
        assert_eq!(e.mem.u32(this.addr() + DATA_BYTES), 8 + 250 * 2 * 44);
    }
}

/// `fseek` (CRT, cdecl: file, offset, origin).
const FSEEK: u32 = 0x00ec_b65a;
/// `BSFile::Close` (engine map), `00affd10` (thiscall).
const BS_FILE_CLOSE: u32 = 0x00af_fd10;
/// The `BSGameSound` constructor body `00ae9360` (thiscall; placed in
/// `BSAudio` by the engine map).
const GAME_SOUND_CONSTRUCTOR: u32 = 0x00ae_9360;
/// The point the emitter position starts at (`011f426c`, three floats).
const START_POSITION_011F426C: u32 = 0x011f_426c;

// Translated from 00aeabf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A seek callback (cdecl: the file, a 64-bit offset as two words, the
/// origin; stored in the callback tables at `011acc1c` and `011acc2c`): -1
/// for a null file, otherwise `fseek(file, offset's low word, origin)`,
/// whose answer it passes on.
pub fn fn_00aeabf0(
    e: &mut Engine,
    file: u32,
    offset_low: u32,
    _offset_high: u32,
    origin: u32,
) -> i32 {
    if file == 0 {
        return -1;
    }
    e.call(FSEEK, &[file, offset_low, origin]).i32()
}

// Translated from 00aeae10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWin32GameSound` constructor (it stores the class's vtable; no caller
/// or vtable entry in the exe): the `BSGameSound` constructor body
/// `00ae9360`, the vtable, the PC fields cleared (data size, the buffers
/// +0x19C, +0x1A0, +0x1D8, +0x1D0, +0x1D4, the DirectSound +0x198, the
/// words +0x204 and +0x208, the position-set byte), the position from the
/// point at `011f426c`, the 0x24-byte `DSBUFFERDESC` cleared, a 0x12-byte
/// format block from the memory manager, no sample data, the attenuation
/// curve 0, 600, 1400, 2600, 5000, and no stream or Ogg reader. Answers
/// `this`.
// Translation incomplete: 00aeae10 - C++ exception unwinding (SEH frame state) is not translated.
pub fn fn_00aeae10(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(GAME_SOUND_CONSTRUCTOR, &[this.addr()]);
    set_field(e, this, 0, WIN32_GAME_SOUND_VTABLE);
    for offset in [
        DATA_BYTES,
        BUFFER,
        SECOND_BUFFER,
        BUFFER_1D8,
        BUFFER_1D0,
        BUFFER_3D,
        DIRECT_SOUND,
        WORD_204,
        WORD_208,
    ] {
        set_field(e, this, offset, 0);
    }
    e.mem.set_u8(this.addr() + POSITION_SET, 0);
    for i in 0..3 {
        let word = e.mem.u32(START_POSITION_011F426C + 4 * i);
        set_field(e, this, POSITION + 4 * i, word);
    }
    e.call(MEMSET, &[this.addr() + BUFFER_DESC, 0, 0x24]);
    let format = e.call(ALLOCATE, &[MEMORY_MANAGER, 0x12]).u32();
    set_field(e, this, WAVE_FORMAT, format);
    set_field(e, this, SAMPLE_DATA, 0);
    for (i, value) in [0u16, 600, 1400, 2600, 5000].into_iter().enumerate() {
        e.mem
            .set_u16(this.addr() + ATTENUATION_CURVE + 2 * i as u32, value);
    }
    set_field(e, this, STREAM, 0);
    set_field(e, this, OGG_READER, 0);
    audio_check::constructed(e, this.addr());
    this
}

// Translated from 00aefd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Empty method taking three words it does not read (slot +0x58 of
/// `BSWin32GameSound`'s vtable, by its RTTI).
pub fn fn_00aefd90(_e: &mut Engine, _this: Ptr, _unused_1: u32, _unused_2: u32, _unused_3: u32) {}

// Translated from 00af1af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Lets go of the second buffer and the buffer at +0x1D8 (no caller or
/// vtable in the exe): with a second buffer (+0x1A0), counts the global at
/// `011f7128` down, stops it (slot +0x48) and releases it (slot +8) until
/// its count reaches 0, and clears the field; then releases +0x1D8 the same
/// way when it is set.
pub fn fn_00af1af0(e: &mut Engine, this: Ptr) {
    if field(e, this, SECOND_BUFFER) != 0 {
        let count = e.mem.u32(SECOND_BUFFER_COUNT_011F7128).wrapping_sub(1);
        e.mem.set_u32(SECOND_BUFFER_COUNT_011F7128, count);
        com(e, this, SECOND_BUFFER, 0x48, &[]);
        release_all(e, this, SECOND_BUFFER);
    }
    if field(e, this, BUFFER_1D8) != 0 {
        release_all(e, this, BUFFER_1D8);
    }
}

// Translated from 00af1cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The same seek callback as [`fn_00aeabf0`] (cdecl; stored in the callback
/// tables at `011acc7c` and `011acc8c`): -1 for a null file, otherwise
/// `fseek(file, offset's low word, origin)`, whose answer it passes on.
pub fn fn_00af1cf0(
    e: &mut Engine,
    file: u32,
    offset_low: u32,
    _offset_high: u32,
    origin: u32,
) -> i32 {
    if file == 0 {
        return -1;
    }
    e.call(FSEEK, &[file, offset_low, origin]).i32()
}

// Translated from 00af2230 (decompiled, FalloutNV.exe 1.4.0.525)
/// A close callback for a `BSFile` (cdecl; no caller or vtable in the
/// exe): -1 for a null file; otherwise `BSFile::Close` (`00affd10`), then
/// its scalar deleting destructor (vtable slot 0) with 1, and answers 0.
pub fn fn_00af2230(e: &mut Engine, file: u32) -> i32 {
    if file == 0 {
        return -1;
    }
    e.call(BS_FILE_CLOSE, &[file]);
    if file != 0 {
        e.vcall(file, 0, &[1]);
    }
    0
}

#[cfg(test)]
mod n079_tests {
    use super::*;

    #[test]
    fn seek_callbacks_skip_a_null_file() {
        for addr in [0x00ae_abf0u32, 0x00af_1cf0] {
            let mut e = Engine::new();
            e.register(FSEEK, |_, a| (a[1] + a[2]).into_ret());
            assert_eq!(e.call(addr, &[0, 5, 0, 1]).i32(), -1);
            e.call_log = Some(vec![]);
            assert_eq!(e.call(addr, &[0x1000, 5, 9, 1]).i32(), 6);
            assert_eq!(e.call_log.take().unwrap()[1], (FSEEK, vec![0x1000, 5, 1]));
        }
    }

    #[test]
    fn constructor_sets_the_pc_fields() {
        let mut e = Engine::new();
        e.map(0x011f_4000, 0x1000);
        for (i, v) in [1.0f32, 2.0, 3.0].into_iter().enumerate() {
            e.mem
                .set_u32(START_POSITION_011F426C + 4 * i as u32, v.to_bits());
        }
        e.register(GAME_SOUND_CONSTRUCTOR, |_, a| a[0].into_ret());
        e.register(MEMSET, |e, a| {
            e.mem.write(a[0], &vec![a[1] as u8; a[2] as usize]);
            a[0].into_ret()
        });
        e.register(ALLOCATE, |_, _| 0x7700u32.into_ret());
        let this = e.mem.alloc(0x230);
        e.mem.write(this, &[0xaa; 0x230]);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_ae10, &[this]).u32(), this);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (GAME_SOUND_CONSTRUCTOR, vec![this]));
        assert!(log.contains(&(ALLOCATE, vec![MEMORY_MANAGER, 0x12])));
        assert_eq!(e.mem.u32(this), WIN32_GAME_SOUND_VTABLE);
        for offset in [
            0x198u32, 0x19c, 0x1a0, 0x1a4, 0x1c4, 0x1c8, 0x1d0, 0x1d4, 0x1d8, 0x1dc, 0x204, 0x208,
            0x228, 0x22c,
        ] {
            assert_eq!(e.mem.u32(this + offset), 0, "+{offset:#x}");
        }
        assert_eq!(e.mem.u32(this + 0x1b4), 0x7700);
        assert_eq!(e.mem.u32(this + 0x1cc), 0xaaaa_aaaa);
        assert_eq!(e.mem.f32(this + 0x21c), 2.0);
        assert_eq!(e.mem.u8(this + 0x224), 0);
        let curve: Vec<u16> = (0..5).map(|i| e.mem.u16(this + 0x20c + 2 * i)).collect();
        assert_eq!(curve, vec![0, 600, 1400, 2600, 5000]);
    }

    #[test]
    fn empty_slot_does_nothing() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.call(0x00ae_fd90, &[0x1000, 1, 2, 3]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
    }

    #[test]
    fn buffers_are_released_until_gone() {
        const STOP: u32 = 0x00de_ad48;
        const RELEASE: u32 = 0x00de_ad08;
        let mut e = Engine::new();
        e.map(0x011f_7000, 0x1000);
        e.mem.set_u32(SECOND_BUFFER_COUNT_011F7128, 3);
        e.register(STOP, |_, _| 0u32.into_ret());
        e.register(RELEASE, |e, a| {
            let n = e.mem.u32(a[0] + 4) - 1;
            e.mem.set_u32(a[0] + 4, n);
            n.into_ret()
        });
        let mut slots = vec![0u32; 0x13];
        slots[2] = RELEASE;
        slots[0x12] = STOP;
        e.put_vtable(0x0072_0000, &slots);
        let second = e.mem.alloc(8);
        e.mem.set_u32(second, 0x0072_0000);
        e.mem.set_u32(second + 4, 2);
        let other = e.mem.alloc(8);
        e.mem.set_u32(other, 0x0072_0000);
        e.mem.set_u32(other + 4, 1);
        let this = e.mem.alloc(0x230);
        e.mem.set_u32(this + SECOND_BUFFER, second);
        e.mem.set_u32(this + BUFFER_1D8, other);
        e.call_log = Some(vec![]);
        e.call(0x00af_1af0, &[this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (STOP, vec![second]));
        assert_eq!(log.iter().filter(|(a, _)| *a == RELEASE).count(), 3);
        assert_eq!(e.mem.u32(SECOND_BUFFER_COUNT_011F7128), 2);
        assert_eq!(e.mem.u32(this + SECOND_BUFFER), 0);
        assert_eq!(e.mem.u32(this + BUFFER_1D8), 0);
        // Nothing set: nothing happens.
        e.call_log = Some(vec![]);
        e.call(0x00af_1af0, &[this]);
        assert_eq!(e.call_log.take().unwrap().len(), 1);
        assert_eq!(e.mem.u32(SECOND_BUFFER_COUNT_011F7128), 2);
    }

    #[test]
    fn close_callback_closes_and_deletes() {
        const DELETE: u32 = 0x00de_ad00;
        let mut e = Engine::new();
        e.register(BS_FILE_CLOSE, |_, _| 0u32.into_ret());
        e.register(DELETE, |_, a| a[0].into_ret());
        e.put_vtable(0x0073_0000, &[DELETE]);
        let file = e.mem.alloc(8);
        e.mem.set_u32(file, 0x0073_0000);
        assert_eq!(e.call(0x00af_2230, &[0]).i32(), -1);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00af_2230, &[file]).i32(), 0);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [(BS_FILE_CLOSE, vec![file]), (DELETE, vec![file, 1])]
        );
    }
}
