//! The platform's DirectSound 8 layer: `DirectSoundCreate8` (DSOUND import by ordinal 11, slot
//! `00fdf034`, behind the thunk `00ee81e0`), `IDirectSound8`, the primary buffer and its
//! `IDirectSound3DListener`, secondary buffers (`IDirectSoundBuffer`/`IDirectSoundBuffer8`), their
//! `IDirectSound3DBuffer` and the distortion effect object, plus `CoInitialize` and
//! `CoCreateInstance`. Every call the game makes is answered as DirectSound answers it.
//!
//! Sound reaches the player's device only when the host asks for it ([`start_output`]): each
//! secondary buffer is then also a [`super::mixer::Voice`] (its samples copied at every
//! `Unlock`, its volume, pan and looping kept with the buffer's), mixed newest first by the
//! measured exact path on the output thread ([`super::sound_out`]). The cursors the game reads
//! stay on the platform clock (above), and the output follows that clock (`Pace`): within
//! 100 ms at the device's own pace, beyond that skipping on or holding. The host turns it on with
//! [`want_output`] (`NV_SOUND=out`), and it starts when the game makes its device. A 3D buffer
//! goes through DirectSound's 3D stage (`super::ds3d`, translated from the player's
//! `dsound.dll`), with `DS3D_DEFERRED` settings held until `CommitDeferredSettings`; a buffer
//! with the distortion through the effect (`super::dsfx`, from the player's `dsdmo.dll`). A buffer
//! at another rate goes through DirectSound's converter (`super::resampler`, translated from
//! the player's `dsound.dll` and `RESAMPLEDMO.DLL`): the first one, until a frequency moves by
//! more than 1/500 of a buffer's rate ([`refresh_voice`]), then the second one for every buffer.
//! Without the player's `RESAMPLEDMO.DLL` (another build) a buffer that needs it is not heard
//! (named once on stderr). A device that does not mix two channels is not provided.
//!
//! The answers that depend on the player's sound device and Windows come from the sound table,
//! captured on the player's machine by `scripts/dsound-capture` (git-ignored: `NV_DSOUND_TABLE`,
//! default `%LOCALAPPDATA%\nv-rs\dsound.tsv`): `DirectSoundCreate8`'s result, the cooperative
//! level's, `GetCaps`, the primary buffer, the listener's defaults, and the secondary buffers,
//! effects and settings the game asks for. Where an answer follows a rule (which buffer sizes
//! and formats are refused, the ranges of volume, pan, frequency and the distortion's
//! parameters), the layer answers by the rule and checks the rule against every captured line
//! when it loads the table: a table that disagrees stops the run. Which secondary buffers are
//! made follows that rule for every description measured (`MEASURED_FLAGS`, PCM mono or stereo,
//! 8 or 16 bits, 100 to 200000 Hz, any size), whether the capture asked for it or not; another
//! description is not provided. The state of buffers, the listener and the effects is this
//! layer's own.
//!
//! Measured on this machine (2026-10-23, Windows 11, `dsound-capture`, 3,500 lines):
//! - `CreateSoundBuffer` for the game's four flag sets, 7 rates, mono and stereo, 8 and 16 bit
//!   and sizes around each limit (3,360 buffers): under 4 bytes `DSERR_INVALIDPARAM`; with
//!   `DSBCAPS_CTRLFX` and fewer than `floor(rate * 150 / 1000)` frames `DSERR_BUFFERTOOSMALL`;
//!   then 3D and not mono `DSERR_INVALIDPARAM`; 3D with `DS3DALG_HRTF_LIGHT` succeeds with
//!   `DS_NO_VIRTUALIZATION`; else `DS_OK`. A new buffer reports its flags plus
//!   `DSBCAPS_LOCSOFTWARE`, its size, its format (0x12 bytes), its rate, status 0, cursors 0;
//!   `IDirectSoundBuffer8` is there and `IDirectSound3DBuffer` only with `DSBCAPS_CTRL3D`.
//! - Volume -10000..0, pan -10000..10000, frequency 0 (the format's rate) or 100..200000; out of
//!   range `DSERR_INVALIDPARAM` (checked before the control: a 3D buffer's in-range pan is
//!   `DSERR_CONTROLUNAVAIL`). `Lock(0, size)` and `Lock(.., DSBLOCK_ENTIREBUFFER)` give one
//!   region of the whole buffer.
//! - `SetFX` with the distortion: `DS_OK`, result `DSFXR_LOCSOFTWARE`; the I3DL2 reverb:
//!   `REGDB_E_CLASSNOTREG`, result `DSFXR_UNKNOWN`, and its `GetObjectInPath`
//!   `DSERR_OBJECTNOTFOUND`; without `DSBCAPS_CTRLFX` both `DSERR_CONTROLUNAVAIL`. The
//!   distortion's gain must be -60..0.
//! - The cursors (seven buffers sampled at every change for 600 ms after `Play`; the first
//!   `Play` of the run took longer than that): DirectSound moves them in steps of the device's
//!   10 ms period. After `n` periods the write cursor is `floor(n * frequency / 100) + 1`
//!   frames on and the play cursor `floor(3 * rate / 100)` frames behind it (three periods at
//!   the format's rate, whatever the frequency). With the first period placed where the
//!   capture saw it, this gives every sample of the 44100 and 48000 Hz stereo and the 11025 Hz
//!   8-bit buffers exactly; the 16-bit mono buffers and the ones with another frequency are
//!   one frame off at some samples (Windows' resampler). The first period begins 1.7 to 7.7 ms
//!   after `Play`, varying from run to run; here the periods are the platform clock's 10 ms
//!   steps (`Engine::clock_now_us`), so runs repeat. A non-looping buffer stops when the write
//!   cursor reaches its end (both cursors then read 0, status 0); `Stop` leaves both cursors at
//!   the write cursor (at 0, for a buffer that stopped by itself, which `Play` then plays again
//!   from its start). A later `Play` of the same buffer (after its end, a `Stop`, a
//!   `SetCurrentPosition`) puts the write cursor `ceil(n * frequency / 100)` frames on, and the
//!   buffer stops when that reaches its end (`dsound-capture replays`, 2026-10-30: all ten
//!   cases at 22050 Hz mono and 44100 Hz stereo). A buffer's first `Play` that day led by 33
//!   frames, not the one frame of the earlier capture: the write cursor is the frames the
//!   converter DirectSound runs the buffer through has taken, so it steps with that converter
//!   (`dsound-capture endstop`, 2026-10-31, and its `more` formats, 2026-11-01: 44100 and
//!   22050 Hz stereo and 48000 Hz mono, both converters; [`Steps`]). With the first converter
//!   (`RESAMPLEDMO`) at a frequency other than the device's rate it is the buffer's position
//!   rounded up and a first `Play` leads by 33 frames from its first period; at the device's
//!   rate (48000 Hz here: the converter passes the samples through) it is the position rounded
//!   up with no lead. With the second converter on (below) it is exactly the frames that
//!   converter takes, its `CalcInputFrames` (`dsound` `100636a0`) in its own `f32` arithmetic
//!   (`super::resampler::AplFrames`), counted from a new converter at the buffer's first `Play`
//!   and kept by the buffer through `Stop`, the end and `SetFrequency`, and, on the DLL's word,
//!   `SetCurrentPosition` (`dsound` `100580b0` only moves the source and sets the cursor fields,
//!   the converter untouched; no measured case moves a buffer with it on).
//!   For an 8-bit or stereo buffer at its own rate that is the frames played and one more (the
//!   earlier capture's `floor(n * frequency / 100) + 1`), but no rule of hundredths fits it
//!   everywhere: a 16-bit mono buffer (the four-lane kernel) at 22050 Hz reads 221, 442, 663
//!   frames after 1, 2, 3 periods, at 48000 Hz 481, 961, at 11025 Hz 111, 221, 331, 441. All 246
//!   measured cases with the second converter on match it at every sample (`endstop`, its `more`
//!   formats, `freqphase`); a new counter at each `Play` matches 75 of them. The layer takes the
//!   device's rate from the output when it runs, else from the default device's mix format
//!   (`super::sound_out::device_rate`).
//! - The steps of a later `Play` (`dsound-capture endphase`, 2026-10-31, 162 cases in three runs,
//!   11025 Hz 8-bit and 22050 Hz 16-bit, ends after 20 to 47 periods and stops after 3 to 10):
//!   the write cursor goes on in the steps of the periods the buffer had played, through its end
//!   (counted to the period its write cursor reached the end) and through `Stop`: `p` periods
//!   played, after `n` more it is `ceil((p + n) * frequency / 100) - ceil(p * frequency / 100)`
//!   frames on (the `ceil(n * frequency / 100)` above whenever `p * frequency / 100` is whole,
//!   which fits every case of `replays`). All 162 match; starting the steps over is refuted by
//!   105 of them. Also after an end then `Stop` (`freqphase`, ends after 21 to 23 periods at
//!   11025 Hz: 12 of 12). In other words DirectSound keeps the buffer's position in hundredths
//!   of a frame, each period adding the frequency, and the write cursor is it rounded up (these
//!   runs had the first converter; the second's steps are above).
//! - `SetFrequency` (`dsound-capture freqphase`, 2026-10-31, 156 cases in three runs: 11025 Hz
//!   8-bit and 22050 Hz 16-bit buffers retuned to 11040, 22071, 16537 and 11025 Hz, then stopped
//!   and played, played to their end and played, or stopped, retuned and played). With
//!   DirectSound's second converter on (a change of more than 1/500 of a rate turns it on for
//!   the device, `super::resampler`), the frames it takes go on through the change, the new
//!   frequency counting from the next period (the converter's own retune), through `Stop` and
//!   the end: 72 of the 72 cases with it on, `22071 6 1 apl` and `11025 6 1 big` among them,
//!   where a first `Play` retuned on a whole frame goes on as a later `Play` would. Right after
//!   the call both cursors read as before it (72 retunes with both converters), and the play
//!   cursor goes on trailing the write cursor by three periods at the format's rate (every
//!   case). A `SetFrequency` to the frequency the buffer has (0 being the format's rate) changes
//!   nothing (`endstop` `loopoff_f`, every format, both converters). Deliberately not modelled
//!   here: with the first converter (`RESAMPLEDMO`) and a frequency other than the format's rate
//!   the write cursor follows that converter's own `GetRequiredInputFrames`, which no rule of
//!   hundredths fits (36 retunes, and the 6 buffers played at 11040 or 22071 Hz from the start;
//!   a change of 1/500 or less reaches the cursor a period late there, at the converter's
//!   `SetRate`), and the layer carries the position's hundredths through the change there (a
//!   first `Play` past its first period going on from its write cursor as a later `Play`):
//!   following the converter would need its filter design from the player's `RESAMPLEDMO.DLL`,
//!   which the cursors must not wait for. When the second converter turns on, every buffer
//!   playing goes on from its write cursor with a new one (`dsound` `1005713d` reads the write
//!   cursor, builds the new converter from there and sets both cursor fields to it, as `Play`
//!   and `SetCurrentPosition` do; the output goes on from there too): both cursors read the
//!   write cursor, the play cursor held there until the write cursor is three periods past. Not
//!   measured.
//! - The end (`dsound-capture endstop`, 2026-10-31, 168 cases in three runs, 22050 Hz 16-bit and
//!   11025 Hz 8-bit, lengths around 20 periods' frames, both converters): a buffer that does not
//!   loop never reads its write cursor at its length while it plays; it ends in the period its
//!   write cursor would reach it, a first `Play`'s too. Stopped in the period before, both
//!   cursors stay on the write cursor and `Play` plays the rest (it ends when its write cursor
//!   reaches the length). A looping buffer turned not looping (`Play(0, 0, 0)` while it plays)
//!   plays out the lap it is in and ends there; a `SetFrequency` to its own rate just before
//!   changes nothing. 168 of 168 (`tools/check-endstop.pl`).
//! - `SetCurrentPosition` (`dsound-capture setpos`, 2026-10-31: 22050 Hz mono and 44100 Hz
//!   stereo 16-bit and 11025 Hz 8-bit, the game's 2D and 3D flags, 412 calls in four runs, 272 of
//!   them on playing buffers; `build43-scratch/measure/dss1-work`): the byte is taken down to a
//!   whole frame; the end of the buffer or past it is `DSERR_INVALIDPARAM` and changes nothing,
//!   whether the buffer plays, is stopped or played to its end. A buffer that does not play is
//!   stopped there, and plays from there as a later `Play` in the steps it was taking. A playing
//!   buffer, looping or not, has both cursors there at once, keeps its status and goes on from
//!   there as after a later `Play`, in the steps it was taking (at 22050 Hz 221, 441, 662 ...
//!   after an even number of periods, 220, 441, 661 ... after an odd one). The play cursor stays
//!   there until the write cursor is three periods past; a buffer that does not loop ends when
//!   the write cursor reaches its end; a following `Play` only takes its looping flag. A first
//!   `Play` moved before its first period still has its lead to come. All 140 calls on buffers
//!   that do not play match at every sample, the first `Play` before them included (the 80
//!   after an end and the 10 after a `Stop` among them). Of the 272 on playing buffers, 268
//!   match at every sample. Of the other four, three came 5 to 24 us after a period began: the
//!   cursors read the new place right after the call, then the old ones again until the next
//!   period. The fourth (`setpos-run3`,
//!   `5622 1 10 ac44 2e0 loop_half 103300`) came 10.02 ms after the last cursor change: the
//!   cursors read the new place right after the call, then the old track's next period showed
//!   53 us after the call, and the move counted its steps from that period on. In all four, at
//!   the next period one read of the write cursor on the new place with the play cursor three
//!   periods before it, then for two periods the play cursor three periods behind the write
//!   cursor, behind the new place, before the cursors are back on the track above. A race inside
//!   DirectSound; the layer leaves it out.
//! - Every measured line replayed through the layer (`measured_replay`, ignored: the same calls
//!   after the same number of periods, the cursors compared at every read, the first `Play`
//!   included; 1,058 lines): all match but the first converter at frequencies other than the
//!   format's rate (42 lines, above) and the four races. docs/SOUND.md has the counts.
//!
//! As in the DirectInput layer (`super::dinput`), the objects live in engine memory with a vtable
//! pointer at `+0`, each provided slot a platform function (code addresses from [`CODE_BASE`]);
//! a slot not provided is left unregistered, so a call to it stops the run as an open function
//! and is recorded with its address.
//!
//! What the game calls: `BSXenonAudio::Init` (`00aea230`), the sounds of `BSWin32GameSound`
//! (`00aeb270`, `00aec530`, `00af0630`, `00af1430` and the playing functions around them) and the
//! listener (`00aea8b0`); `CoCreateInstance` only from the DirectShow music thread (`00830b30`)
//! and the filter registration (`009fba40`, `009fbab0`), which this layer does not provide.

use super::mixer::Voice;
use crate::{Engine, Ptr};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, Mutex};

/// Where the platform's DirectSound functions sit (code addresses no exe code uses).
pub const CODE_BASE: u32 = 0x7e00_0400;
pub const SOUND_CODE: u32 = CODE_BASE;
pub const BUFFER_CODE: u32 = CODE_BASE + 0x100;
pub const LISTENER_CODE: u32 = CODE_BASE + 0x200;
pub const BUFFER3D_CODE: u32 = CODE_BASE + 0x300;
pub const EFFECT_CODE: u32 = CODE_BASE + 0x380;

/// `IDirectSound8`'s slots (dsound.h order).
pub const SOUND_SLOTS: [&str; 12] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "CreateSoundBuffer",
    "GetCaps",
    "DuplicateSoundBuffer",
    "SetCooperativeLevel",
    "Compact",
    "GetSpeakerConfig",
    "SetSpeakerConfig",
    "Initialize",
    "VerifyCertification",
];
/// `IDirectSoundBuffer8`'s slots (the primary buffer is an `IDirectSoundBuffer`, the first 21).
pub const BUFFER_SLOTS: [&str; 24] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetCaps",
    "GetCurrentPosition",
    "GetFormat",
    "GetVolume",
    "GetPan",
    "GetFrequency",
    "GetStatus",
    "Initialize",
    "Lock",
    "Play",
    "SetCurrentPosition",
    "SetFormat",
    "SetVolume",
    "SetPan",
    "SetFrequency",
    "Stop",
    "Unlock",
    "Restore",
    "SetFX",
    "AcquireResources",
    "GetObjectInPath",
];
/// `IDirectSound3DListener`'s slots.
pub const LISTENER_SLOTS: [&str; 18] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetAllParameters",
    "GetDistanceFactor",
    "GetDopplerFactor",
    "GetOrientation",
    "GetPosition",
    "GetRolloffFactor",
    "GetVelocity",
    "SetAllParameters",
    "SetDistanceFactor",
    "SetDopplerFactor",
    "SetOrientation",
    "SetPosition",
    "SetRolloffFactor",
    "SetVelocity",
    "CommitDeferredSettings",
];
/// `IDirectSound3DBuffer`'s slots.
pub const BUFFER3D_SLOTS: [&str; 21] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "GetAllParameters",
    "GetConeAngles",
    "GetConeOrientation",
    "GetConeOutsideVolume",
    "GetMaxDistance",
    "GetMinDistance",
    "GetMode",
    "GetPosition",
    "GetVelocity",
    "SetAllParameters",
    "SetConeAngles",
    "SetConeOrientation",
    "SetConeOutsideVolume",
    "SetMaxDistance",
    "SetMinDistance",
    "SetMode",
    "SetPosition",
    "SetVelocity",
];
/// `IDirectSoundFXDistortion8`'s slots.
pub const DISTORTION_SLOTS: [&str; 5] = [
    "QueryInterface",
    "AddRef",
    "Release",
    "SetAllParameters",
    "GetAllParameters",
];

/// Result codes (dsound.h / winerror.h).
pub const DS_OK: u32 = 0;
pub const S_FALSE: u32 = 1;
pub const E_NOINTERFACE: u32 = 0x8000_4002;
pub const DSERR_INVALIDPARAM: u32 = 0x8007_0057;
pub const DSERR_UNSUPPORTED: u32 = 0x8000_4001;
pub const DSERR_ALREADYINITIALIZED: u32 = 0x8878_0082;
pub const DSERR_CONTROLUNAVAIL: u32 = 0x8878_001e;
pub const DSERR_BUFFERTOOSMALL: u32 = 0x8878_00b4;
pub const DS_NO_VIRTUALIZATION: u32 = 0x0878_000a;

/// Interface and class ids.
const IID_LISTENER: [u8; 16] = [
    0x84, 0xfa, 0x9a, 0x27, 0x81, 0x49, 0xce, 0x11, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60,
];
const IID_BUFFER3D: [u8; 16] = [
    0x86, 0xfa, 0x9a, 0x27, 0x81, 0x49, 0xce, 0x11, 0xa5, 0x21, 0x00, 0x20, 0xaf, 0x0b, 0xe5, 0x60,
];
const IID_BUFFER8: [u8; 16] = [
    0x49, 0xa4, 0x25, 0x68, 0x24, 0x75, 0x82, 0x4d, 0x92, 0x0f, 0x50, 0xe3, 0x6a, 0xb3, 0xab, 0x1e,
];
/// `GUID_DSFX_STANDARD_DISTORTION` and `IID_IDirectSoundFXDistortion8` (the values at the exe's
/// `01100ff4` and `01100fc4`), and the I3DL2 reverb's (`01101004`, `01100fd4`).
const GUID_DISTORTION: [u8; 16] = [
    0x90, 0x4c, 0x11, 0xef, 0x1d, 0xcd, 0x4e, 0x48, 0x96, 0xe5, 0x09, 0xcf, 0xaf, 0x91, 0x2a, 0x21,
];
const IID_DISTORTION8: [u8; 16] = [
    0x26, 0x43, 0xcf, 0x8e, 0x5f, 0x45, 0x8b, 0x4d, 0xbd, 0xa9, 0x8d, 0x5d, 0x3e, 0x9e, 0x3e, 0x0b,
];
const GUID_I3DL2: [u8; 16] = [
    0x71, 0x5e, 0x98, 0xef, 0xc7, 0xd5, 0xd4, 0x42, 0xba, 0x4d, 0x2d, 0x07, 0x3e, 0x2e, 0x96, 0xf4,
];
const IID_I3DL2_8: [u8; 16] = [
    0x6a, 0x6a, 0x16, 0x4b, 0x66, 0x0d, 0xf3, 0x43, 0x80, 0xe3, 0xee, 0x62, 0x80, 0xde, 0xe1, 0xa4,
];
/// `DS3DALG_HRTF_LIGHT` as the game stores it (`00aeb270`).
const ALG_HRTF_LIGHT: [u32; 4] = [0xc241_3342, 0x11d2_1c1b, 0xc000_f594, 0xca8a_c24f];

/// `DSSCL_*` cooperative levels.
const DSSCL_NORMAL: u32 = 1;
const DSSCL_WRITEPRIMARY: u32 = 4;
/// `DSBCAPS_*` flags.
const DSBCAPS_PRIMARYBUFFER: u32 = 0x1;
const DSBCAPS_LOCSOFTWARE: u32 = 0x8;
const DSBCAPS_CTRL3D: u32 = 0x10;
const DSBCAPS_CTRLFREQUENCY: u32 = 0x20;
const DSBCAPS_CTRLPAN: u32 = 0x40;
const DSBCAPS_CTRLVOLUME: u32 = 0x80;
const DSBCAPS_CTRLFX: u32 = 0x200;
const DSBCAPS_MUTE3DATMAXDISTANCE: u32 = 0x2_0000;
/// The secondary-buffer flag sets measured (`refmeasure sweep`, 2026-10-27): the game's four
/// (2D and 3D, with and without `DSBCAPS_CTRLFX`) and `0x80e0` (`DSBCAPS_GLOBALFOCUS` 2D), with
/// rates 100 to 200000 Hz, mono and stereo, 8 and 16 bits, and sizes around every limit.
const MEASURED_FLAGS: [u32; 5] = [0x2e0, 0xe0, 0x302b0, 0x300b0, 0x80e0];
/// `DSBPLAY_LOOPING`, `DSBLOCK_ENTIREBUFFER`, `DSBSTATUS_PLAYING`, `DSBSTATUS_LOOPING`.
const DSBPLAY_LOOPING: u32 = 1;
const DSBLOCK_ENTIREBUFFER: u32 = 2;
const DSBSTATUS_PLAYING: u32 = 1;
const DSBSTATUS_LOOPING: u32 = 4;
/// `DSCAPS` (0x60 bytes), `DSBCAPS` (0x14), `DSBUFFERDESC` (0x24, DirectX 7 and later),
/// `WAVEFORMATEX` (0x12), `DSEFFECTDESC` (0x20), `DS3DLISTENER` (0x40).
const DSCAPS_SIZE: u32 = 0x60;
const DSBCAPS_SIZE: u32 = 0x14;
const BUFFER_DESC_SIZE: u32 = 0x24;
const FORMAT_SIZE: u32 = 0x12;
const EFFECT_DESC_SIZE: u32 = 0x20;
const LISTENER_SIZE: u32 = 0x40;
/// The device's period (measured: the cursors move every 10 ms) and the smallest buffer.
const PERIOD_US: u64 = 10_000;
const MIN_BUFFER_BYTES: u32 = 4;

/// How to make the sound table, for the stop when it is missing.
pub const CAPTURE_HINT: &str = "run `cargo run --release --target i686-pc-windows-msvc \
     --manifest-path scripts/dsound-capture/Cargo.toml > <file>` on this machine and point \
     NV_DSOUND_TABLE at the file (default: %LOCALAPPDATA%/nv-rs/dsound.tsv)";

/// What a secondary buffer request is: flags, whether the HRTF algorithm is asked, and the
/// format's tag, channels, rate and bits.
type Kind = (u32, bool, u16, u16, u32, u16);

/// The captured sound table (`scripts/dsound-capture`), checked against the layer's rules.
#[derive(Debug, Clone, Default)]
pub struct Table {
    create: u32,
    cooperative: BTreeMap<u32, u32>,
    caps: (u32, Vec<u8>),
    primary: u32,
    listener: (u32, Vec<u8>),
    /// `SetFX` / `GetObjectInPath` by (effect, buffer has `DSBCAPS_CTRLFX`): `SetFX`'s result,
    /// its result code, `GetObjectInPath`'s result.
    effects: BTreeMap<(String, bool), (u32, u32, u32)>,
}

fn hex(text: &str) -> u32 {
    u32::from_str_radix(text, 16)
        .unwrap_or_else(|_| panic!("a hex word in the sound table: {text}"))
}

fn words(text: &str) -> Vec<u32> {
    text.split(' ').filter(|w| !w.is_empty()).map(hex).collect()
}

fn bytes(text: &str) -> Vec<u8> {
    (0..text.len() / 2)
        .map(|i| u8::from_str_radix(&text[2 * i..2 * i + 2], 16).expect("hex bytes"))
        .collect()
}

fn disagree(line: &str, rule: impl std::fmt::Display) -> ! {
    panic!("the sound table disagrees with the layer ({rule}): {line}; re-capture it or report it")
}

/// The answer the layer gives a secondary `CreateSoundBuffer` (see the module documentation).
fn create_result(kind: Kind, bytes: u32) -> u32 {
    let (flags, hrtf, _, channels, rate, bits) = kind;
    let align = u32::from(channels) * u32::from(bits) / 8;
    if bytes < MIN_BUFFER_BYTES {
        DSERR_INVALIDPARAM
    } else if flags & DSBCAPS_CTRLFX != 0 && u64::from(bytes / align) < u64::from(rate) * 150 / 1000
    {
        DSERR_BUFFERTOOSMALL
    } else if flags & DSBCAPS_CTRL3D != 0 && channels != 1 {
        DSERR_INVALIDPARAM
    } else if flags & DSBCAPS_CTRL3D != 0 && hrtf {
        DS_NO_VIRTUALIZATION
    } else {
        DS_OK
    }
}

fn volume_ok(v: i32) -> bool {
    (-10000..=0).contains(&v)
}

fn pan_ok(v: i32) -> bool {
    (-10000..=10000).contains(&v)
}

fn frequency_ok(v: u32) -> bool {
    v == 0 || (100..=200_000).contains(&v)
}

/// `DSFXDistortion`'s ranges (dsound.h: gain -60..0, edge 0..100, the frequencies 100..8000).
fn distortion_ok(p: [f32; 5]) -> bool {
    (-60.0..=0.0).contains(&p[0])
        && (0.0..=100.0).contains(&p[1])
        && p[2..].iter().all(|f| (100.0..=8000.0).contains(f))
}

impl Table {
    /// Parses and checks a capture.
    pub fn parse(text: &str) -> Table {
        let mut t = Table::default();
        let mut ended = false;
        for line in text.lines().filter(|l| !l.is_empty()) {
            let f: Vec<&str> = line.split('\t').collect();
            let field = |i: usize| *f.get(i).unwrap_or(&"");
            match f[0] {
                "DsCreate" => t.create = hex(field(2)),
                "DsCooperativeLevel" => {
                    t.cooperative.insert(hex(field(1)), hex(field(2)));
                }
                "DsCaps" => t.caps = (hex(field(2)), bytes(field(3))),
                "DsPrimary" => t.primary = hex(field(2)),
                "DsListener" => {
                    let r = words(field(2));
                    t.listener = (r[0], bytes(field(3)));
                }
                "DsBuffer" => t.check_buffer(line, &f),
                "DsFx" => t.check_effect(line, &f),
                "DsSet" => check_setting(line, &f),
                "DsCursor" => {}
                "DsEnd" => ended = true,
                other => panic!("the sound table has a line this layer does not know: {other}"),
            }
        }
        assert!(
            ended,
            "the sound table is cut short (no DsEnd): {CAPTURE_HINT}"
        );
        t
    }

    fn check_buffer(&mut self, line: &str, f: &[&str]) {
        let w = words(f[1]);
        let kind: Kind = (w[0], w[1] != 0, w[2] as u16, w[3] as u16, w[4], w[5] as u16);
        let size = w[6];
        let result = hex(f[2]);
        let want = create_result(kind, size);
        if result != want {
            disagree(line, format!("CreateSoundBuffer answers {want:x}"));
        }
        if (result as i32) < 0 {
            return;
        }
        // The new buffer as the layer makes it.
        let caps = bytes(f[3]);
        let read = |b: &[u8], o: usize| u32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
        let (flags, _, tag, channels, rate, bits) = kind;
        let align = u32::from(channels) * u32::from(bits) / 8;
        let format = format_bytes(tag, channels, rate, bits);
        let state = words(f[5]);
        let queries = words(f[6]);
        let three_d = if flags & DSBCAPS_CTRL3D != 0 {
            DS_OK
        } else {
            E_NOINTERFACE
        };
        let ok = read(&caps, 0) == DSBCAPS_SIZE
            && read(&caps, 4) == flags | DSBCAPS_LOCSOFTWARE
            && read(&caps, 8) == size
            && read(&caps, 12) == 0
            && read(&caps, 16) == 0
            && f[4] == format!("{} 12", hex_string(&format))
            && state == [rate, 0, 0, 0]
            && queries == [DS_OK, three_d];
        if !ok || align == 0 {
            disagree(line, "a new buffer's caps, format, state and interfaces");
        }
    }

    fn check_effect(&mut self, line: &str, f: &[&str]) {
        let w: Vec<&str> = f[1].split(' ').collect();
        if f[2].starts_with("create") {
            return;
        }
        let flags = hex(w[1]);
        let key = (w[0].to_string(), flags & DSBCAPS_CTRLFX != 0);
        let value = (hex(f[2]), hex(f[3]), hex(f[4]));
        if let Some(old) = self.effects.insert(key, value) {
            if old != value {
                disagree(line, "the same effect answers the same for every format");
            }
        }
    }

    /// The captured answer to `SetFX`/`GetObjectInPath` for this effect.
    fn effect(&self, name: &str, fx: bool) -> (u32, u32, u32) {
        *self
            .effects
            .get(&(name.to_string(), fx))
            .unwrap_or_else(|| {
                panic!("SetFX({name}) was not captured in the sound table: not provided")
            })
    }
}

fn hex_string(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

fn format_bytes(tag: u16, channels: u16, rate: u32, bits: u16) -> Vec<u8> {
    let align = channels * bits / 8;
    let mut b = Vec::new();
    b.extend_from_slice(&tag.to_le_bytes());
    b.extend_from_slice(&channels.to_le_bytes());
    b.extend_from_slice(&rate.to_le_bytes());
    b.extend_from_slice(&(rate * u32::from(align)).to_le_bytes());
    b.extend_from_slice(&align.to_le_bytes());
    b.extend_from_slice(&bits.to_le_bytes());
    b.extend_from_slice(&0u16.to_le_bytes());
    b
}

/// Checks a `DsSet` line against the layer's rules.
fn check_setting(line: &str, f: &[&str]) {
    let w: Vec<&str> = f[1].split(' ').collect();
    // The 3D buffer's defaults line has two results (the query's and `GetAllParameters`').
    let result = hex(f[2].split(' ').next().unwrap_or_default());
    let expect = |want: u32| {
        if result != want {
            disagree(line, format!("answers {want:x}"));
        }
    };
    let ok = |good: bool| if good { DS_OK } else { DSERR_INVALIDPARAM };
    match w.as_slice() {
        ["distortion", gain] => {
            let g = f32::from_bits(hex(gain));
            expect(ok(distortion_ok([g, 0.0, 5000.0, 1135.0, 5000.0])));
        }
        [_, "volume", v] => expect(ok(volume_ok(hex(v) as i32))),
        [flags, "pan", v] => {
            let has_pan = hex(flags) & DSBCAPS_CTRLPAN != 0;
            let v = hex(v) as i32;
            expect(match (pan_ok(v), has_pan) {
                (false, _) => DSERR_INVALIDPARAM,
                (true, false) => DSERR_CONTROLUNAVAIL,
                (true, true) => DS_OK,
            });
        }
        [_, "frequency", v] => expect(ok(frequency_ok(hex(v)))),
        ["3dbuffer", "mindistance" | "maxdistance", v] => {
            expect(ok(f32::from_bits(hex(v)) > 0.0));
        }
        [_, "lock", ..]
        | [_, "unlock"]
        | ["3dbuffer", ..]
        | ["listener", ..]
        | ["i3dl2preset", _] => {}
        _ => panic!("the sound table has a setting this layer does not know: {line}"),
    }
}

/// Gives the layer the player's sound table (`scripts/dsound-capture`'s output).
pub fn load_table(e: &mut Engine, text: &str) {
    e.kernel.sound.table = Some(Table::parse(text));
}

/// The loaded table; loaded from `NV_DSOUND_TABLE` (default `%LOCALAPPDATA%\nv-rs\dsound.tsv`)
/// the first time it is needed. Without one the run stops and says how to capture it.
fn table(e: &mut Engine) -> &Table {
    if e.kernel.sound.table.is_none() {
        let path = std::env::var_os("NV_DSOUND_TABLE")
            .map(std::path::PathBuf::from)
            .or_else(|| {
                std::env::var_os("LOCALAPPDATA")
                    .map(|d| std::path::PathBuf::from(d).join("nv-rs").join("dsound.tsv"))
            });
        let text = path
            .as_ref()
            .and_then(|p| std::fs::read_to_string(p).ok())
            .unwrap_or_else(|| panic!("the sound table ({path:?}) is not there: {CAPTURE_HINT}"));
        load_table(e, &text);
    }
    e.kernel.sound.table.as_ref().expect("loaded")
}

/// The DirectSound 8 `DS3DLISTENER` state: the settings in effect, and those set with
/// `DS3D_DEFERRED` that wait for `CommitDeferredSettings` (each setting on its own, as the
/// DLL's deferred copy with its dirty flags).
#[derive(Debug, Clone, Copy)]
struct Listener {
    position: [f32; 3],
    velocity: [f32; 3],
    front: [f32; 3],
    top: [f32; 3],
    distance_factor: f32,
    rolloff: f32,
    doppler: f32,
    pending: ListenerPending,
}

#[derive(Debug, Clone, Copy, Default)]
struct ListenerPending {
    position: Option<[f32; 3]>,
    velocity: Option<[f32; 3]>,
    orientation: Option<([f32; 3], [f32; 3])>,
    distance_factor: Option<f32>,
    rolloff: Option<f32>,
    doppler: Option<f32>,
}

/// A 3D buffer's settings set with `DS3D_DEFERRED`, waiting for the listener's
/// `CommitDeferredSettings`.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
struct BufferPending {
    position: Option<[f32; 3]>,
    velocity: Option<[f32; 3]>,
    min_distance: Option<f32>,
    max_distance: Option<f32>,
}

/// `DS3D_DEFERRED`.
const DS3D_DEFERRED: u32 = 1;

/// `DSFXDistortion`'s defaults (dsound.h: gain -18 dB, edge 15 %, post-EQ centre and bandwidth
/// 2400 Hz, pre-lowpass cutoff 8000 Hz): the effect's parameters until the game sets them.
const DISTORTION_DEFAULTS: [f32; 5] = [-18.0, 15.0, 2400.0, 2400.0, 8000.0];

/// How a buffer's write cursor steps: the frames DirectSound's converter takes for it (measured,
/// module documentation), by the converter it goes through:
/// - `Dmo`: the first converter (`RESAMPLEDMO`) converting: the buffer's position, kept in
///   hundredths of a frame (each period adding the frequency), rounded up; a first `Play` leads
///   by 33 frames;
/// - `Pass`: the first converter at the device's own rate, passing the samples through: the
///   position, no lead;
/// - `Apl`: the second converter: the frames it takes ([`super::resampler::AplFrames`], kept
///   by the buffer, `Buffer::apl`), `fed` of them in the `done` periods counted from `start`.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Steps {
    Dmo,
    Pass,
    Apl { fed: u64, done: u64 },
}

impl Steps {
    /// A first `Play`'s lead after `n` periods (the first converter's).
    fn lead(self, n: u64) -> u64 {
        match self {
            Steps::Dmo => 33 * u64::from(n >= 1),
            _ => 0,
        }
    }
}

/// Whether a buffer plays.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Play {
    /// Stopped with both cursors at byte `at`, its position `part` hundredths of a frame past a
    /// whole frame (see `Playing`).
    Stopped { at: u32, part: u32 },
    /// Playing since the clock read `start` (microseconds): the write cursor at frame `base`
    /// then, the play cursor held on frame `from` (where `Play` or `SetCurrentPosition` put it)
    /// until the write cursor is three periods past it (frames counted on round a looping
    /// buffer, not wrapped); `first` for the buffer's first `Play` since it was made, before a
    /// `SetFrequency` past its first period (it leads, [`Steps::lead`]). `part`: DirectSound keeps
    /// the buffer's position in hundredths of a frame, each period adding the frequency, and the
    /// write cursor follows it as `steps` says (the converter when it was played, moved or
    /// retuned); `part` is how far past a whole frame it was at `start` (0 to 99), so the write
    /// cursor goes on in the steps it was taking: carried through `SetCurrentPosition`,
    /// `SetFrequency`, `Stop`, the end and the next `Play` (measured, module documentation); 0
    /// for a buffer never played. (With the second converter the frames it takes count instead,
    /// [`Steps::Apl`].)
    /// `end`: the frame (counted the same way) at which it ends if it does not loop: its length,
    /// or the end of the lap it was in when `Play` turned its looping off (measured).
    Playing {
        looping: bool,
        start: u64,
        base: u64,
        from: u64,
        first: bool,
        part: u32,
        end: u64,
        steps: Steps,
    },
    /// A non-looping buffer that played to its end, its position `part` hundredths of a frame
    /// past a whole frame (see `Playing`).
    Ended { part: u32 },
}

/// A buffer's `WAVEFORMATEX` as the bytes it was made with (none for the primary buffer): kept in
/// place, so a copy of a buffer's settings allocates nothing (the buffer's calls copy them at
/// every call, tens a frame). Reads as the byte slice.
#[derive(Clone, Copy, Default)]
struct Format {
    len: usize,
    bytes: [u8; FORMAT_SIZE as usize],
}

impl Format {
    fn of(bytes: &[u8]) -> Format {
        let mut f = Format {
            len: bytes.len(),
            ..Format::default()
        };
        f.bytes[..bytes.len()].copy_from_slice(bytes);
        f
    }
}

impl std::ops::Deref for Format {
    type Target = [u8];
    fn deref(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
}

impl std::fmt::Debug for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (**self).fmt(f)
    }
}

#[derive(Debug, Clone)]
struct Buffer {
    references: u32,
    /// The `IDirectSound3DBuffer` interface's own count (measured: separate from the buffer's).
    references3d: u32,
    flags: u32,
    /// The listener this primary buffer gave out (0: none yet).
    listener: u32,
    /// Secondary buffers: the size, the format, the sample memory (engine memory), the 3D
    /// interface (0: not asked yet).
    bytes: u32,
    format: Format,
    data: u32,
    three_d: u32,
    volume: i32,
    pan: i32,
    frequency: u32,
    play: Play,
    /// `Play` was called on it before (its next `Play` is not its first).
    played: bool,
    /// The frames the second converter takes for it ([`Steps::Apl`]): made at its first `Play`
    /// with the second converter on (or when it turns on while the buffer plays), then kept
    /// through `Stop`, the end and `SetFrequency` (measured, module documentation) and
    /// `SetCurrentPosition` (the DLL: no measured case moves a buffer with it on).
    apl: Option<super::resampler::AplFrames>,
    locks: u32,
    effect: Option<&'static str>,
    min_distance: f32,
    max_distance: f32,
    position: [f32; 3],
    velocity: [f32; 3],
    pending3d: BufferPending,
    /// The distortion's parameters (`DSFXDistortion`), when it is set.
    fx_params: [f32; 5],
}

impl Buffer {
    fn align(&self) -> u32 {
        u32::from(u16::from_le_bytes([self.format[12], self.format[13]]))
    }

    fn rate(&self) -> u32 {
        u32::from_le_bytes([
            self.format[4],
            self.format[5],
            self.format[6],
            self.format[7],
        ])
    }

    fn frequency(&self) -> u32 {
        if self.frequency == 0 {
            self.rate()
        } else {
            self.frequency
        }
    }

    fn frames(&self) -> u64 {
        u64::from(self.bytes / self.align())
    }
}

/// The DirectSound objects (`Kernel::sound`).
#[derive(Debug, Default)]
pub struct Sound {
    sound_vtable: u32,
    buffer_vtable: u32,
    listener_vtable: u32,
    buffer3d_vtable: u32,
    distortion_vtable: u32,
    devices: BTreeMap<u32, (u32, u32)>,
    buffers: BTreeMap<u32, Buffer>,
    listeners: BTreeMap<u32, (u32, Listener)>,
    /// The 3D interfaces: the buffer each belongs to (each counts its own references,
    /// `Buffer::references3d`).
    buffers3d: BTreeMap<u32, u32>,
    /// 3D interfaces whose buffer is gone, with their reference count (released past 0 it wraps,
    /// as measured).
    dead3d: BTreeMap<u32, u32>,
    /// The distortion objects: reference count and the buffer whose effect each one is.
    distortions: BTreeMap<u32, (u32, u32)>,
    /// `CoInitialize` calls on the main thread (the first answers `S_OK`, later ones `S_FALSE`).
    com_initialized: u32,
    /// The sound table ([`load_table`]).
    table: Option<Table>,
    /// The DirectShow filter graphs ([`super::dshow`]).
    pub(crate) show: super::dshow::Show,
    /// The open Ogg Vorbis files ([`super::vorbisfile`]).
    pub(crate) vorbis: super::vorbisfile::Files,
    /// The voices heard on the device, once the host started the output ([`start_output`]).
    mix: Option<Mix>,
    /// The host asked for the output when the game makes its device ([`want_output`]), and
    /// where to record what goes to the device.
    output_wanted: bool,
    device_paced: bool,
    record: Option<std::path::PathBuf>,
    /// What was already named on stderr (once each).
    noted: BTreeSet<&'static str>,
    /// DirectSound has turned on its second converter for the device (a frequency moved by more
    /// than 1/500 of a rate, [`set_frequency`], [`refresh_voice`]): it steps the cursors
    /// ([`Steps`]) and converts the voices (`Voices::apl`).
    apl: bool,
}

/// The output thread and the voices it plays (the buffers' engine addresses, newest `Play`
/// first).
struct Mix {
    voices: Arc<Mutex<Voices>>,
    rate: u32,
    /// The thread (none in the tests, which render by hand).
    _output: Option<super::sound_out::Output>,
}

impl std::fmt::Debug for Mix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Mix {{ rate: {} }}", self.rate)
    }
}

#[derive(Default)]
struct Voices {
    /// Every secondary buffer's voice, by address.
    all: BTreeMap<u32, Voice>,
    /// The voices started and not stopped since, newest `Play` first. One the output plays to its
    /// end stays in its place, silent, until it is stopped, played again or released (moved by
    /// `SetCurrentPosition` while its buffer still plays, it starts again there).
    playing: Vec<u32>,
    /// What goes to the device, recorded ([`want_output`]).
    record: Option<Record>,
    /// The game's clock the output follows (none in the tests, which render by hand).
    pace: Option<Pace>,
    /// The device has turned on DirectSound's second converter (`super::resampler`): a buffer's
    /// frequency moved by more than 1/500 of its rate.
    apl: bool,
}

/// A recording of what the mixer sends to the device: a WAV file (32-bit float, the device's
/// rate and channels) whose sizes are written again every second, so a run that is killed
/// leaves a readable file; and a log of what was started when, beside it (`.log`).
pub struct Record {
    wav: std::fs::File,
    log: std::fs::File,
    frames: u64,
    rate: u32,
    channels: u16,
    since_header: u64,
    started: std::time::Instant,
}

impl Record {
    fn create(path: &std::path::Path, rate: u32, channels: u16) -> std::io::Result<Record> {
        let wav = std::fs::File::create(path)?;
        let log = std::fs::File::create(path.with_extension("log"))?;
        let mut r = Record {
            wav,
            log,
            frames: 0,
            rate,
            channels,
            since_header: 0,
            started: std::time::Instant::now(),
        };
        r.header()?;
        Ok(r)
    }

    fn header(&mut self) -> std::io::Result<()> {
        use std::io::{Seek, SeekFrom, Write};
        let block = 4 * u32::from(self.channels);
        let data = (self.frames * u64::from(block)).min(u64::from(u32::MAX - 64)) as u32;
        let mut h = Vec::with_capacity(44);
        h.extend_from_slice(b"RIFF");
        h.extend_from_slice(&(36 + data).to_le_bytes());
        h.extend_from_slice(b"WAVEfmt ");
        h.extend_from_slice(&16u32.to_le_bytes());
        h.extend_from_slice(&3u16.to_le_bytes());
        h.extend_from_slice(&self.channels.to_le_bytes());
        h.extend_from_slice(&self.rate.to_le_bytes());
        h.extend_from_slice(&(self.rate * block).to_le_bytes());
        h.extend_from_slice(&(block as u16).to_le_bytes());
        h.extend_from_slice(&32u16.to_le_bytes());
        h.extend_from_slice(b"data");
        h.extend_from_slice(&data.to_le_bytes());
        let end = self.wav.stream_position()?.max(44);
        self.wav.seek(SeekFrom::Start(0))?;
        self.wav.write_all(&h)?;
        self.wav.seek(SeekFrom::Start(end))?;
        Ok(())
    }

    fn write(&mut self, out: &[f32]) {
        use std::io::Write;
        let bytes: Vec<u8> = out.iter().flat_map(|s| s.to_le_bytes()).collect();
        let _ = self.wav.write_all(&bytes);
        let n = (out.len() / usize::from(self.channels)) as u64;
        self.frames += n;
        self.since_header += n;
        if self.since_header >= u64::from(self.rate) {
            self.since_header = 0;
            let _ = self.header();
        }
    }

    /// One line of the log: the time since the output started (ms), the frames recorded so
    /// far, and `what`.
    fn note(&mut self, what: &str) {
        use std::io::Write;
        let ms = self.started.elapsed().as_millis();
        let _ = writeln!(self.log, "{ms}\t{}\t{what}", self.frames);
    }
}

impl Voices {
    /// Mixes the listed voices into `out` (newest first); one that ends stays listed, silent.
    fn mix(&mut self, out: &mut [f32]) {
        let Voices { all, playing, .. } = self;
        // `playing` holds distinct addresses (each `Play` takes its own out first): each voice
        // is rendered once, in its order, as `mixer::render` renders a list.
        for at in playing.iter() {
            if let Some(v) = all.get_mut(at) {
                super::mixer::render_voice(v, out);
            }
        }
        playing.retain(|at| all.contains_key(at));
    }
}

/// How the output follows the game's clock (`Engine::clock_now_us`, which `Sleep` moves on
/// beyond the real time): the clock as the engine last read it and when, and how far the
/// mixer is. The game reads its buffers' cursors and the music's position from that clock, so
/// the voices are kept where that clock puts them: when it runs more than 100 ms ahead of what
/// was mixed, the mixer skips on (mixing the frames and dropping them); when it falls more than
/// 100 ms behind, the device gets silence until it catches up. Within 100 ms the device's own
/// pace is kept.
struct Pace {
    game_us: u64,
    at: std::time::Instant,
    start_us: u64,
    mixed: u64,
}

impl super::sound_out::Source for Voices {
    fn render(&mut self, rate: u32, channels: u16, out: &mut [f32]) {
        assert!(
            channels == 2,
            "a sound device mixing {channels} channels is not provided"
        );
        let frames = (out.len() / 2) as u64;
        let mut hold = false;
        if let Some(p) = &mut self.pace {
            let game = p.game_us + p.at.elapsed().as_micros() as u64;
            let target =
                (u128::from(game.saturating_sub(p.start_us)) * u128::from(rate) / 1_000_000) as i64;
            let lag = target - (p.mixed + frames) as i64;
            let cap = i64::from(rate / 10);
            if lag > cap {
                let skip = (lag - cap / 2) as u64;
                p.mixed += skip;
                let mut scratch = vec![0f32; 2 * 4800];
                let mut left = skip as usize;
                while left > 0 {
                    let n = left.min(4800);
                    scratch[..2 * n].fill(0.0);
                    self.mix(&mut scratch[..2 * n]);
                    left -= n;
                }
                if let Some(r) = &mut self.record {
                    r.note(&format!(
                        "pace: skipped {skip} frames (the game's clock ran ahead)"
                    ));
                }
            } else if lag < -cap {
                hold = true;
            }
        }
        if hold {
            if let Some(r) = &mut self.record {
                r.note(&format!(
                    "pace: {frames} frames of silence (the game's clock is behind)"
                ));
            }
        } else {
            if let Some(p) = &mut self.pace {
                p.mixed += frames;
            }
            self.mix(out);
        }
        if let Some(r) = &mut self.record {
            r.write(out);
        }
    }
}

/// Has the output started ([`start_output`]) when the game makes its DirectSound device
/// (`DirectSoundCreate8`, from `BSWin32Audio::Init` `00aea230`), so what the game plays from then
/// on is heard. With `record`, what the mixer sends to the device is also written there as a
/// WAV file, with a log of what was started when beside it (`.log`). The host's switch
/// (`NV_SOUND=out`); off by default and in the tests. With `device_paced` the output keeps the
/// device's own pace instead of following the game's clock (`Pace`; `NV_SOUND=device`, for
/// listening while the game's clock runs ahead of the real time).
pub fn want_output(e: &mut Engine, record: Option<std::path::PathBuf>, device_paced: bool) {
    e.kernel.sound.output_wanted = true;
    e.kernel.sound.device_paced = device_paced;
    e.kernel.sound.record = record;
}

/// Starts playing the game's sound on the player's default device; false when there is no
/// device (or not on Windows). Buffers made before keep playing silently until their next
/// `Play`.
pub fn start_output(e: &mut Engine) -> bool {
    if e.kernel.sound.mix.is_some() {
        return true;
    }
    // The first converter's tables, read now rather than on the output thread.
    let _ = super::resampler::tables();
    let voices = Arc::new(Mutex::new(Voices::default()));
    {
        let mut v = voices.lock().expect("the voices");
        for (&at, b) in &e.kernel.sound.buffers {
            if b.flags & DSBCAPS_PRIMARYBUFFER == 0 {
                v.all.insert(at, voice_of(e, b));
            }
        }
        // The device's converter as the game has left it ([`Sound::apl`]).
        v.apl = e.kernel.sound.apl;
        let apl = v.apl;
        for voice in v.all.values_mut() {
            voice.apl = apl;
        }
    }
    let source: Arc<Mutex<dyn super::sound_out::Source>> = voices.clone();
    let Some(output) = super::sound_out::start(source) else {
        eprintln!("sound: no output device; the game's sound is not heard");
        return false;
    };
    if let Some(path) = e.kernel.sound.record.clone() {
        match Record::create(&path, output.rate, output.channels) {
            Ok(r) => voices.lock().expect("the voices").record = Some(r),
            Err(err) => eprintln!("sound: cannot record to {}: {err}", path.display()),
        }
    }
    if !e.kernel.sound.device_paced {
        let now = e.clock_now_us();
        voices.lock().expect("the voices").pace = Some(Pace {
            game_us: now,
            at: std::time::Instant::now(),
            start_us: now,
            mixed: 0,
        });
    }
    eprintln!(
        "sound: output on the default device, {} Hz, {} channels",
        output.rate, output.channels
    );
    e.kernel.sound.mix = Some(Mix {
        voices,
        rate: output.rate,
        _output: Some(output),
    });
    true
}

/// Writes `what` to the recording's log, when the output is recorded, with the game's clock
/// (ms) beside the real time.
pub fn note(e: &mut Engine, what: &str) {
    if e.kernel.sound.mix.is_none() {
        return;
    }
    let game = e.clock_now_ms();
    if let Some(m) = &e.kernel.sound.mix {
        if let Some(r) = &mut m.voices.lock().expect("the voices").record {
            r.note(&format!("game {game}\t{what}"));
        }
    }
}

/// Names on stderr, once, that a buffer at another rate is not heard: the player's
/// `RESAMPLEDMO.DLL` is another build than the one `super::resampler` reads.
fn unheard(e: &mut Engine, example: &str) {
    if e.kernel.sound.noted.insert("resampling") {
        eprintln!("sound: not heard: another rate without this build of RESAMPLEDMO.DLL (first: {example}); see docs/SOUND.md");
    }
}

/// A buffer's voice: its samples as they are in engine memory now, its settings, not playing.
fn voice_of(e: &Engine, b: &Buffer) -> Voice {
    let mut v = Voice::new(
        e.mem.bytes(b.data, b.bytes),
        u16::from_le_bytes([b.format[2], b.format[3]]),
        u16::from_le_bytes([b.format[14], b.format[15]]),
    );
    v.volume = b.volume;
    v.pan = b.pan;
    v.rate = b.frequency();
    v.device = e.kernel.sound.mix.as_ref().map_or(0, |m| m.rate);
    if b.effect == Some("distortion") {
        v.fx = Some(Box::new(fx_chain(b, v.data.clone())));
    }
    v
}

/// The distortion's chain for buffer `b` on `source` (the buffer's samples): the effect at the
/// format's rate (the rate `dsound.dll` gives the effect, whatever the frequency) with the
/// buffer's parameters and the player's edge table (`super::dsfx::edge_table`). Without that
/// table the distortion is not provided.
fn fx_chain(b: &Buffer, source: Vec<u8>) -> super::dsfx::Chain {
    let c = super::dsfx::Coefficients::new(b.rate(), b.fx_params, edge_table());
    super::dsfx::Chain::new(
        source,
        super::dsfx::Distortion::new(c),
        u16::from_le_bytes([b.format[2], b.format[3]]),
        u16::from_le_bytes([b.format[14], b.format[15]]),
    )
}

/// The player's edge table for the distortion (`super::dsfx::edge_table`); the tests' own
/// made-up one under `cfg(test)`.
fn edge_table() -> &'static [f64; 32] {
    #[cfg(test)]
    {
        &tests::EDGE_TABLE
    }
    #[cfg(not(test))]
    {
        super::dsfx::edge_table().expect(
            "the distortion's edge table could not be measured from the player's dsdmo.dll: \
             the distortion is not provided",
        )
    }
}

/// Runs `f` on the voice of the buffer at `at`, when the output is on.
fn with_voice(e: &mut Engine, at: u32, f: impl FnOnce(&mut Voices, u32)) {
    tick(e);
    if let Some(m) = &e.kernel.sound.mix {
        let mut v = m.voices.lock().expect("the voices");
        f(&mut v, at);
    }
}

/// Tells the output where the game's clock is now (`Pace`); called at every sound call that
/// reads the clock or changes a voice.
pub fn tick(e: &mut Engine) {
    if e.kernel.sound.mix.is_none() {
        return;
    }
    let now = e.clock_now_us();
    if let Some(m) = &e.kernel.sound.mix {
        if let Some(p) = &mut m.voices.lock().expect("the voices").pace {
            p.game_us = now;
            p.at = std::time::Instant::now();
        }
    }
}

/// For a platform layer that plays sound of its own through DirectSound (Bink's DirectSound
/// sound system, `super::bink`): starts `pcm` (`channels`, `bits`, `rate`) as a voice under
/// `key` (an engine address of the caller's, distinct from every buffer's), heard when the
/// output is on, with volume, pan and frequency at their defaults. A rate other than the
/// device's goes through the first converter (the stream is its own DirectSound device).
pub fn play_stream(e: &mut Engine, key: u32, pcm: Vec<u8>, channels: u16, bits: u16, rate: u32) {
    play_stream_from(e, key, pcm, channels, bits, rate, 0);
}

/// [`play_stream`] from the stream's frame `from`.
pub fn play_stream_from(
    e: &mut Engine,
    key: u32,
    pcm: Vec<u8>,
    channels: u16,
    bits: u16,
    rate: u32,
    from: usize,
) {
    play_voice(e, key, Voice::new(pcm, channels, bits), rate, from);
}

/// [`play_stream_from`] of a 16-bit stream another thread decodes (`super::dshow`'s music): the
/// voice takes its samples from `feed` as it reads them ([`super::mixer::Feed`]).
pub fn play_stream_fed(
    e: &mut Engine,
    key: u32,
    feed: Arc<super::mixer::Feed>,
    channels: u16,
    rate: u32,
    from: usize,
) {
    if e.kernel.sound.mix.is_none() {
        return;
    }
    play_voice(e, key, Voice::fed_by(feed, channels), rate, from);
}

fn play_voice(e: &mut Engine, key: u32, mut voice: Voice, rate: u32, from: usize) {
    let Some(device) = e.kernel.sound.mix.as_ref().map(|m| m.rate) else {
        return;
    };
    // A stream is its own DirectSound device (Bink's, DirectShow's): the first converter.
    if rate != device && super::resampler::tables().is_none() {
        unheard(e, &format!("a stream at {rate} Hz on {device} Hz"));
    }
    let Some(m) = &e.kernel.sound.mix else {
        return;
    };
    let mut v = m.voices.lock().expect("the voices");
    voice.start(from);
    voice.rate = rate;
    voice.device = device;
    v.all.insert(key, voice);
    v.playing.retain(|&p| p != key);
    v.playing.insert(0, key);
}

/// Sets the volume (hundredths of a decibel, -10000..0) of a stream from [`play_stream`].
pub fn stream_volume(e: &mut Engine, key: u32, volume: i32) {
    with_voice(e, key, |m, at| {
        if let Some(v) = m.all.get_mut(&at) {
            v.volume = volume;
        }
    });
}

/// Whether a stream from [`play_stream`] exists (playing or held).
pub fn has_stream(e: &Engine, key: u32) -> bool {
    e.kernel
        .sound
        .mix
        .as_ref()
        .is_some_and(|m| m.voices.lock().expect("the voices").all.contains_key(&key))
}

/// Moves a stream from [`play_stream`] to its frame `at` (it keeps playing or held).
pub fn seek_stream(e: &mut Engine, key: u32, at: usize) {
    with_voice(e, key, |m, k| {
        if let Some(v) = m.all.get_mut(&k) {
            v.start(at);
        }
    });
}

/// Holds (`paused`) or goes on with a stream from [`play_stream`] where it is.
pub fn pause_stream(e: &mut Engine, key: u32, paused: bool) {
    with_voice(e, key, |m, at| {
        m.playing.retain(|&p| p != at);
        if !paused && m.all.get(&at).is_some_and(|v| v.position.is_some()) {
            m.playing.insert(0, at);
        }
    });
}

/// Ends a stream from [`play_stream`].
pub fn stop_stream(e: &mut Engine, key: u32) {
    with_voice(e, key, |m, at| {
        m.all.remove(&at);
        m.playing.retain(|&p| p != at);
    });
}

/// The tests' output (other units' tests too): on at `rate` without a device, rendered by hand
/// ([`mixed`]).
#[cfg(test)]
pub(crate) fn hear_by_hand(e: &mut Engine, rate: u32) {
    e.kernel.sound.mix = Some(Mix {
        voices: Arc::default(),
        rate,
        _output: None,
    });
}

/// The next `frames` frames the tests' output gives ([`hear_by_hand`]).
#[cfg(test)]
pub(crate) fn mixed(e: &Engine, frames: usize) -> Vec<f32> {
    let mut out = vec![0f32; 2 * frames];
    if let Some(m) = &e.kernel.sound.mix {
        m.voices.lock().expect("the voices").mix(&mut out);
    }
    out
}

type Slots<const N: usize> = [(u32, crate::AbiFn); N];

fn register_code(e: &mut Engine) {
    let sound: Slots<6> = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, sound_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, sound_release(Ptr) -> u32).1),
        (
            3,
            crate::entry!(0, create_sound_buffer(Ptr, Ptr, Ptr, u32) -> u32).1,
        ),
        (4, crate::entry!(0, get_caps(Ptr, Ptr) -> u32).1),
        (
            6,
            crate::entry!(0, set_cooperative_level(Ptr, u32, u32) -> u32).1,
        ),
    ];
    for (slot, f) in sound {
        e.register(SOUND_CODE + 4 * slot, f);
    }
    let buffer: Slots<20> = [
        (
            0,
            crate::entry!(0, buffer_query_interface(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (1, crate::entry!(0, buffer_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, buffer_release(Ptr) -> u32).1),
        (3, crate::entry!(0, buffer_get_caps(Ptr, Ptr) -> u32).1),
        (
            4,
            crate::entry!(0, get_current_position(Ptr, Ptr, Ptr) -> u32).1,
        ),
        (5, crate::entry!(0, get_format(Ptr, Ptr, u32, Ptr) -> u32).1),
        (6, crate::entry!(0, get_volume(Ptr, Ptr) -> u32).1),
        (7, crate::entry!(0, get_pan(Ptr, Ptr) -> u32).1),
        (8, crate::entry!(0, get_frequency(Ptr, Ptr) -> u32).1),
        (9, crate::entry!(0, get_status(Ptr, Ptr) -> u32).1),
        (
            11,
            crate::entry!(0, lock(Ptr, u32, u32, Ptr, Ptr, Ptr, Ptr, u32) -> u32).1,
        ),
        (12, crate::entry!(0, play(Ptr, u32, u32, u32) -> u32).1),
        (
            13,
            crate::entry!(0, set_current_position(Ptr, u32) -> u32).1,
        ),
        (15, crate::entry!(0, set_volume(Ptr, i32) -> u32).1),
        (16, crate::entry!(0, set_pan(Ptr, i32) -> u32).1),
        (17, crate::entry!(0, set_frequency(Ptr, u32) -> u32).1),
        (18, crate::entry!(0, stop(Ptr) -> u32).1),
        (
            19,
            crate::entry!(0, unlock(Ptr, u32, u32, u32, u32) -> u32).1,
        ),
        (21, crate::entry!(0, set_fx(Ptr, u32, Ptr, Ptr) -> u32).1),
        (
            23,
            crate::entry!(0, get_object_in_path(Ptr, Ptr, u32, Ptr, Ptr) -> u32).1,
        ),
    ];
    for (slot, f) in buffer {
        e.register(BUFFER_CODE + 4 * slot, f);
    }
    let listener: Slots<13> = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, listener_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, listener_release(Ptr) -> u32).1),
        (3, crate::entry!(0, listener_get_all_parameters(Ptr, Ptr) -> u32).1),
        (7, crate::entry!(0, listener_get_position(Ptr, Ptr) -> u32).1),
        (10, crate::entry!(0, listener_set_all_parameters(Ptr, Ptr, u32) -> u32).1),
        (11, crate::entry!(0, listener_set_distance_factor(Ptr, f32, u32) -> u32).1),
        (12, crate::entry!(0, listener_set_doppler_factor(Ptr, f32, u32) -> u32).1),
        (
            13,
            crate::entry!(0, listener_set_orientation(Ptr, f32, f32, f32, f32, f32, f32, u32) -> u32).1,
        ),
        (14, crate::entry!(0, listener_set_position(Ptr, f32, f32, f32, u32) -> u32).1),
        (15, crate::entry!(0, listener_set_rolloff_factor(Ptr, f32, u32) -> u32).1),
        (16, crate::entry!(0, listener_set_velocity(Ptr, f32, f32, f32, u32) -> u32).1),
        (17, crate::entry!(0, listener_commit(Ptr) -> u32).1),
    ];
    for (slot, f) in listener {
        e.register(LISTENER_CODE + 4 * slot, f);
    }
    let three_d: Slots<7> = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, buffer3d_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, buffer3d_release(Ptr) -> u32).1),
        (
            16,
            crate::entry!(0, set_max_distance(Ptr, f32, u32) -> u32).1,
        ),
        (
            17,
            crate::entry!(0, set_min_distance(Ptr, f32, u32) -> u32).1,
        ),
        (
            19,
            crate::entry!(0, buffer3d_set_position(Ptr, f32, f32, f32, u32) -> u32).1,
        ),
        (
            20,
            crate::entry!(0, buffer3d_set_velocity(Ptr, f32, f32, f32, u32) -> u32).1,
        ),
    ];
    for (slot, f) in three_d {
        e.register(BUFFER3D_CODE + 4 * slot, f);
    }
    let distortion: Slots<4> = [
        (0, crate::entry!(0, no_interface(Ptr, Ptr, Ptr) -> u32).1),
        (1, crate::entry!(0, distortion_add_ref(Ptr) -> u32).1),
        (2, crate::entry!(0, distortion_release(Ptr) -> u32).1),
        (
            3,
            crate::entry!(0, distortion_set_all_parameters(Ptr, Ptr) -> u32).1,
        ),
    ];
    for (slot, f) in distortion {
        e.register(EFFECT_CODE + 4 * slot, f);
    }
}

fn vtable(e: &mut Engine, code: u32, slots: u32) -> u32 {
    let table = e.mem.alloc(4 * slots);
    for i in 0..slots {
        e.mem.set_u32(table + 4 * i, code + 4 * i);
    }
    table
}

fn object(e: &mut Engine, vtable: u32) -> u32 {
    let object = e.mem.alloc(8);
    e.mem.set_u32(object, vtable);
    object
}

// Platform: replaces OLE32.DLL CoInitialize (import)
/// `CoInitialize(NULL)`: COM on the calling thread; `S_OK` the first time, `S_FALSE` after.
pub fn co_initialize(e: &mut Engine, reserved: u32) -> u32 {
    if reserved != 0 {
        return DSERR_INVALIDPARAM;
    }
    e.kernel.sound.com_initialized += 1;
    if e.kernel.sound.com_initialized == 1 {
        DS_OK
    } else {
        S_FALSE
    }
}

// Platform: replaces OLE32.DLL CoCreateInstance (import)
/// `CoCreateInstance(clsid, outer, context, iid, out)`: the game asks only for DirectShow's
/// filter graph (the music thread `00830b30`), which [`super::dshow`] provides, and the filter
/// mapper (the filter registration `009fba40`/`009fbab0`), which stops the run as any other class.
pub fn co_create_instance(
    e: &mut Engine,
    clsid: Ptr,
    outer: u32,
    context: u32,
    iid: Ptr,
    out: Ptr,
) -> u32 {
    super::dshow::co_create_instance(e, clsid, outer, context, iid, out)
}

// Platform: replaces DSOUND.DLL DirectSoundCreate8 (import by ordinal 11)
/// `DirectSoundCreate8(device, out, outer)`: the default device (`device` NULL) as a new
/// `IDirectSound8` through `out` with the captured result; a failure the capture recorded is
/// answered as captured with `out` NULL. A chosen device or an outer object is not provided.
pub fn direct_sound_create8(e: &mut Engine, device: u32, out: Ptr, outer: u32) -> u32 {
    if device != 0 || outer != 0 {
        panic!("DirectSoundCreate8 with a device GUID or an outer object is not provided");
    }
    let result = table(e).create;
    if result != DS_OK {
        e.mem.set_u32(out.addr(), 0);
        return result;
    }
    if e.kernel.sound.sound_vtable == 0 {
        register_code(e);
        e.kernel.sound.sound_vtable = vtable(e, SOUND_CODE, SOUND_SLOTS.len() as u32);
        e.kernel.sound.buffer_vtable = vtable(e, BUFFER_CODE, BUFFER_SLOTS.len() as u32);
        e.kernel.sound.listener_vtable = vtable(e, LISTENER_CODE, LISTENER_SLOTS.len() as u32);
        e.kernel.sound.buffer3d_vtable = vtable(e, BUFFER3D_CODE, BUFFER3D_SLOTS.len() as u32);
        e.kernel.sound.distortion_vtable = vtable(e, EFFECT_CODE, DISTORTION_SLOTS.len() as u32);
    }
    let table = e.kernel.sound.sound_vtable;
    let device = object(e, table);
    e.kernel.sound.devices.insert(device, (1, 0));
    e.mem.set_u32(out.addr(), device);
    // The host's switch (`want_output`): the game's device is made, so its sound is heard
    // from here on.
    if e.kernel.sound.output_wanted {
        start_output(e);
    }
    DS_OK
}

fn no_interface(e: &mut Engine, _this: Ptr, _interface: Ptr, out: Ptr) -> u32 {
    if !out.is_null() {
        e.mem.set_u32(out.addr(), 0);
    }
    E_NOINTERFACE
}

fn device(e: &mut Engine, this: Ptr) -> &mut (u32, u32) {
    e.kernel
        .sound
        .devices
        .get_mut(&this.addr())
        .expect("an IDirectSound8 of this layer")
}

fn sound_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    d.0 += 1;
    d.0
}

fn sound_release(e: &mut Engine, this: Ptr) -> u32 {
    let d = device(e, this);
    d.0 -= 1;
    let left = d.0;
    if left == 0 {
        e.kernel.sound.devices.remove(&this.addr());
        e.mem.free(this.addr());
    }
    left
}

/// `SetCooperativeLevel(window, level)`: the captured answer for that level (the game asks
/// `DSSCL_PRIORITY`); a level outside `DSSCL_NORMAL..DSSCL_WRITEPRIMARY` is `DSERR_INVALIDPARAM`;
/// a level the capture didn't ask is not provided.
fn set_cooperative_level(e: &mut Engine, this: Ptr, _window: u32, level: u32) -> u32 {
    if !(DSSCL_NORMAL..=DSSCL_WRITEPRIMARY).contains(&level) {
        return DSERR_INVALIDPARAM;
    }
    let result = *table(e)
        .cooperative
        .get(&level)
        .unwrap_or_else(|| panic!("SetCooperativeLevel({level}) was not captured: not provided"));
    if result == DS_OK {
        device(e, this).1 = level;
    }
    result
}

/// `GetCaps(caps)`: `caps->dwSize` must be 0x60; the captured `DSCAPS`.
fn get_caps(e: &mut Engine, this: Ptr, caps: Ptr) -> u32 {
    let _ = device(e, this);
    let at = caps.addr();
    if e.mem.u32(at) != DSCAPS_SIZE {
        return DSERR_INVALIDPARAM;
    }
    let (result, bytes) = table(e).caps.clone();
    if result == DS_OK {
        e.mem.write(at, &bytes);
    }
    result
}

fn new_buffer(flags: u32) -> Buffer {
    Buffer {
        references: 1,
        references3d: 0,
        flags,
        listener: 0,
        bytes: 0,
        format: Format::default(),
        data: 0,
        three_d: 0,
        volume: 0,
        pan: 0,
        frequency: 0,
        play: Play::Stopped { at: 0, part: 0 },
        played: false,
        apl: None,
        locks: 0,
        effect: None,
        // DS3D_DEFAULTMINDISTANCE and DS3D_DEFAULTMAXDISTANCE (the capture's defaults).
        min_distance: 1.0,
        max_distance: 1.0e9,
        position: [0.0; 3],
        velocity: [0.0; 3],
        pending3d: BufferPending::default(),
        fx_params: DISTORTION_DEFAULTS,
    }
}

/// `CreateSoundBuffer(desc, out, outer)`: the primary buffer (flags `DSBCAPS_PRIMARYBUFFER`
/// with `DSBCAPS_CTRL3D` and `DSBCAPS_CTRLVOLUME` at most, size and format 0) with the captured
/// result, or a secondary buffer of a kind the capture made, answered by the measured rule
/// (module documentation). `out` is written only on success.
fn create_sound_buffer(e: &mut Engine, this: Ptr, desc: Ptr, out: Ptr, outer: u32) -> u32 {
    let _ = device(e, this);
    if outer != 0 {
        return DSERR_INVALIDPARAM;
    }
    let d = desc.addr();
    assert!(
        e.mem.u32(d) == BUFFER_DESC_SIZE,
        "CreateSoundBuffer with a description of {:#x} bytes is not provided",
        e.mem.u32(d)
    );
    let flags = e.mem.u32(d + 4);
    let size = e.mem.u32(d + 8);
    let format = e.mem.u32(d + 0x10);
    let mut buffer = if flags & DSBCAPS_PRIMARYBUFFER != 0 {
        assert!(
            flags & !(DSBCAPS_PRIMARYBUFFER | DSBCAPS_CTRL3D | DSBCAPS_CTRLVOLUME) == 0
                && flags == 0x91,
            "IDirectSound8::CreateSoundBuffer of a primary buffer with flags {flags:#x} is not provided"
        );
        if size != 0 || format != 0 {
            return DSERR_INVALIDPARAM;
        }
        let result = table(e).primary;
        if result != DS_OK {
            return result;
        }
        new_buffer(flags)
    } else {
        assert!(
            format != 0,
            "CreateSoundBuffer without a format is not provided"
        );
        let f = e.mem.bytes(format, FORMAT_SIZE);
        let w16 = |o: usize| u16::from_le_bytes([f[o], f[o + 1]]);
        let w32 = |o: usize| u32::from_le_bytes([f[o], f[o + 1], f[o + 2], f[o + 3]]);
        let (tag, channels, rate, bits) = (w16(0), w16(2), w32(4), w16(14));
        let alg: Vec<u32> = (0..4).map(|i| e.mem.u32(d + 0x14 + 4 * i)).collect();
        let hrtf = alg == ALG_HRTF_LIGHT;
        let kind: Kind = (flags, hrtf, tag, channels, rate, bits);
        let align = u32::from(channels) * u32::from(bits) / 8;
        // The rule (`create_result`) is general DirectSound behaviour, measured over every
        // description below (`refmeasure sweep`, 2026-10-27: 6,256 descriptions, all as the
        // rule); the capture's own lines are checked against it when the table loads.
        if flags == DSBCAPS_MUTE3DATMAXDISTANCE | 0xe0 {
            // Measured: muting at the maximum distance without 3D is refused.
            return DSERR_INVALIDPARAM;
        }
        assert!(
            (hrtf || alg == [0; 4])
                && MEASURED_FLAGS.contains(&flags)
                && tag == 1
                && matches!(channels, 1 | 2)
                && matches!(bits, 8 | 16)
                && (100..=200_000).contains(&rate)
                && f == format_bytes(tag, channels, rate, bits),
            "CreateSoundBuffer of a buffer description not measured (flags {flags:#x}, \
             algorithm {alg:x?}, format {}, {size} bytes) is not provided",
            hex_string(&f)
        );
        let result = create_result(kind, size);
        if (result as i32) < 0 {
            return result;
        }
        let mut b = new_buffer(flags);
        // Measured: a size that is not a whole number of blocks is rounded up to one (what
        // `GetCaps` reports).
        let size = size.div_ceil(align) * align;
        b.bytes = size;
        b.format = Format::of(&f);
        b.data = e.mem.alloc(size);
        if result == DS_NO_VIRTUALIZATION {
            let at = insert_buffer(e, b);
            e.mem.set_u32(out.addr(), at);
            return result;
        }
        b
    };
    buffer.references = 1;
    let at = insert_buffer(e, buffer);
    e.mem.set_u32(out.addr(), at);
    DS_OK
}

fn insert_buffer(e: &mut Engine, b: Buffer) -> u32 {
    let table = e.kernel.sound.buffer_vtable;
    let at = object(e, table);
    if b.flags & DSBCAPS_PRIMARYBUFFER == 0 {
        let v = voice_of(e, &b);
        with_voice(e, at, |m, at| {
            m.all.insert(at, v);
        });
    }
    e.kernel.sound.buffers.insert(at, b);
    at
}

fn buffer(e: &mut Engine, this: Ptr) -> &mut Buffer {
    e.kernel
        .sound
        .buffers
        .get_mut(&this.addr())
        .expect("a buffer of this layer")
}

fn secondary(e: &mut Engine, this: Ptr, call: &str) -> Buffer {
    let b = buffer(e, this).clone();
    assert!(
        b.flags & DSBCAPS_PRIMARYBUFFER == 0,
        "IDirectSoundBuffer::{call} on the primary buffer is not provided"
    );
    b
}

/// `QueryInterface(iid, out)` on a buffer: the primary buffer made with `DSBCAPS_CTRL3D` gives
/// its `IDirectSound3DListener` (one per buffer, a new reference each time); a secondary buffer
/// gives itself as `IDirectSoundBuffer8` and, with `DSBCAPS_CTRL3D`, its `IDirectSound3DBuffer`
/// (sharing its reference count); any other interface is `E_NOINTERFACE`.
fn buffer_query_interface(e: &mut Engine, this: Ptr, iid: Ptr, out: Ptr) -> u32 {
    let mut id = [0u8; 16];
    e.mem.read(iid.addr(), &mut id);
    let b = buffer(e, this).clone();
    if b.flags & DSBCAPS_PRIMARYBUFFER == 0 {
        if id == IID_BUFFER8 {
            buffer(e, this).references += 1;
            e.mem.set_u32(out.addr(), this.addr());
            return DS_OK;
        }
        if id == IID_BUFFER3D && b.flags & DSBCAPS_CTRL3D != 0 {
            let three_d = if b.three_d == 0 {
                let table = e.kernel.sound.buffer3d_vtable;
                let at = object(e, table);
                e.kernel.sound.buffers3d.insert(at, this.addr());
                buffer(e, this).three_d = at;
                at
            } else {
                b.three_d
            };
            buffer(e, this).references3d += 1;
            e.mem.set_u32(out.addr(), three_d);
            return DS_OK;
        }
        return no_interface(e, this, iid, out);
    }
    let is_3d_primary = b.flags & DSBCAPS_CTRL3D != 0;
    if id != IID_LISTENER || !is_3d_primary {
        return no_interface(e, this, iid, out);
    }
    let listener = if b.listener != 0 {
        let l = e
            .kernel
            .sound
            .listeners
            .get_mut(&b.listener)
            .expect("its listener");
        l.0 += 1;
        b.listener
    } else {
        let defaults = listener_defaults(e);
        let table = e.kernel.sound.listener_vtable;
        let listener = object(e, table);
        e.kernel.sound.listeners.insert(listener, (1, defaults));
        buffer(e, this).listener = listener;
        listener
    };
    e.mem.set_u32(out.addr(), listener);
    DS_OK
}

/// The listener's state when the primary buffer gives it out: the captured `DS3DLISTENER`.
fn listener_defaults(e: &mut Engine) -> Listener {
    let (result, b) = table(e).listener.clone();
    assert!(
        result == DS_OK && b.len() == LISTENER_SIZE as usize,
        "the sound table has no listener: {CAPTURE_HINT}"
    );
    let f = |o: usize| f32::from_le_bytes([b[o], b[o + 1], b[o + 2], b[o + 3]]);
    let v = |o: usize| [f(o), f(o + 4), f(o + 8)];
    Listener {
        position: v(4),
        velocity: v(0x10),
        front: v(0x1c),
        top: v(0x28),
        distance_factor: f(0x34),
        rolloff: f(0x38),
        doppler: f(0x3c),
        pending: ListenerPending::default(),
    }
}

fn buffer_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let b = buffer(e, this);
    b.references += 1;
    b.references
}

/// Drops one reference of the buffer at `at` (its `IDirectSoundBuffer`/`IDirectSoundBuffer8`
/// count). Measured (`refmeasure`, 2026-10-27): the buffer interfaces and the
/// `IDirectSound3DBuffer` count their references apart (create 1, `QueryInterface` of the 8
/// interface +1 on the buffer's; of the 3D interface 1 on its own); a `Release` of the buffer at
/// 0 answers 0 and takes one from the 3D interface's count instead, so a later `Release` of the
/// 3D interface answers `0xffffffff` (the game's loading-menu sound, `00af1cc0`, releases the
/// buffer three times and then the 3D interface). The buffer goes (its memory, its voice) once
/// both counts are 0.
fn release_buffer(e: &mut Engine, at: u32) -> u32 {
    let b = e
        .kernel
        .sound
        .buffers
        .get_mut(&at)
        .expect("a buffer of this layer");
    if b.references > 0 {
        b.references -= 1;
    } else {
        b.references3d = b.references3d.wrapping_sub(1);
    }
    let left = b.references;
    if b.references == 0 && b.references3d == 0 {
        free_buffer(e, at);
    }
    left
}

/// Frees the buffer at `at`: its memory, its voice; its 3D interface stays as a gone one
/// (`Sound::dead3d`).
fn free_buffer(e: &mut Engine, at: u32) {
    let b = e.kernel.sound.buffers.remove(&at).expect("the buffer");
    with_voice(e, at, |m, at| {
        m.all.remove(&at);
        m.playing.retain(|&p| p != at);
    });
    if b.data != 0 {
        e.mem.free(b.data);
    }
    if b.three_d != 0 {
        e.kernel.sound.buffers3d.remove(&b.three_d);
        e.kernel.sound.dead3d.insert(b.three_d, 0);
    }
    e.mem.free(at);
}

fn buffer_release(e: &mut Engine, this: Ptr) -> u32 {
    release_buffer(e, this.addr())
}

/// `GetCaps(caps)`: `caps->dwSize` must be 0x14; the flags (with `DSBCAPS_LOCSOFTWARE`) and
/// size, transfer rate and processor overhead 0 (measured).
fn buffer_get_caps(e: &mut Engine, this: Ptr, caps: Ptr) -> u32 {
    let b = secondary(e, this, "GetCaps");
    let at = caps.addr();
    if e.mem.u32(at) != DSBCAPS_SIZE {
        return DSERR_INVALIDPARAM;
    }
    e.mem.set_u32(at + 4, b.flags | DSBCAPS_LOCSOFTWARE);
    e.mem.set_u32(at + 8, b.bytes);
    e.mem.set_u32(at + 12, 0);
    e.mem.set_u32(at + 16, 0);
    DS_OK
}

/// `GetFormat(format, size, written)`: the 0x12-byte `WAVEFORMATEX` (when `format` is given and
/// `size` holds it) and its size through `written`. A smaller `size` is not provided.
fn get_format(e: &mut Engine, this: Ptr, format: Ptr, size: u32, written: Ptr) -> u32 {
    let b = secondary(e, this, "GetFormat");
    if !format.is_null() {
        assert!(
            size >= FORMAT_SIZE,
            "GetFormat into {size} bytes is not provided"
        );
        e.mem.write(format.addr(), &b.format);
    }
    if !written.is_null() {
        e.mem.set_u32(written.addr(), FORMAT_SIZE);
    }
    DS_OK
}

fn get_volume(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let v = secondary(e, this, "GetVolume").volume;
    e.mem.set_i32(out.addr(), v);
    DS_OK
}

fn get_pan(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let v = secondary(e, this, "GetPan").pan;
    e.mem.set_i32(out.addr(), v);
    DS_OK
}

/// `GetFrequency(out)`: the frequency set, or the format's rate.
fn get_frequency(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let v = secondary(e, this, "GetFrequency").frequency();
    e.mem.set_u32(out.addr(), v);
    DS_OK
}

/// `SetVolume(v)`: -10000..0, else `DSERR_INVALIDPARAM`. Without `DSBCAPS_CTRLVOLUME` not
/// provided (only the pan's order was measured).
fn set_volume(e: &mut Engine, this: Ptr, v: i32) -> u32 {
    let b = secondary(e, this, "SetVolume");
    if !volume_ok(v) {
        return DSERR_INVALIDPARAM;
    }
    assert!(
        b.flags & DSBCAPS_CTRLVOLUME != 0,
        "SetVolume without DSBCAPS_CTRLVOLUME is not provided"
    );
    buffer(e, this).volume = v;
    with_voice(e, this.addr(), |m, at| {
        if let Some(voice) = m.all.get_mut(&at) {
            voice.volume = v;
        }
    });
    // A 3D buffer's volume goes through the 3D stage (`1002a7d0`).
    refresh_voice(e, this.addr());
    DS_OK
}

/// `SetPan(v)`: -10000..10000, else `DSERR_INVALIDPARAM`; then without `DSBCAPS_CTRLPAN`
/// `DSERR_CONTROLUNAVAIL` (measured order).
fn set_pan(e: &mut Engine, this: Ptr, v: i32) -> u32 {
    let b = secondary(e, this, "SetPan");
    if !pan_ok(v) {
        return DSERR_INVALIDPARAM;
    }
    if b.flags & DSBCAPS_CTRLPAN == 0 {
        return DSERR_CONTROLUNAVAIL;
    }
    buffer(e, this).pan = v;
    with_voice(e, this.addr(), |m, at| {
        if let Some(voice) = m.all.get_mut(&at) {
            voice.pan = v;
        }
    });
    DS_OK
}

/// `SetFrequency(v)`: 0 (the format's rate) or 100..200000, else `DSERR_INVALIDPARAM`; the
/// frequency the buffer has changes nothing (measured, `endstop` `loopoff_f`). A playing buffer
/// goes on from where it is at the new speed from the next period, both cursors as they were
/// right after the call and its play cursor trailing as before (measured, `dsound-capture
/// freqphase`, module documentation): with the second converter on, its frame counter takes the
/// new frequency ([`Steps::Apl`], every case); with the first, its position's hundredths of a
/// frame are carried and a first `Play` past its first period goes on from its write cursor as a
/// later `Play`. Deliberately not modelled: with the first converter at a frequency other than
/// the format's rate the steps are that converter's own input requests (and a small change
/// reaches them a period late); the layer keeps the same rule there. A change of more than 1/500
/// of the rate turns on the second converter ([`second_converter_on`]). Without
/// `DSBCAPS_CTRLFREQUENCY` not provided.
fn set_frequency(e: &mut Engine, this: Ptr, v: u32) -> u32 {
    let b = secondary(e, this, "SetFrequency");
    if !frequency_ok(v) {
        return DSERR_INVALIDPARAM;
    }
    assert!(
        b.flags & DSBCAPS_CTRLFREQUENCY != 0,
        "SetFrequency without DSBCAPS_CTRLFREQUENCY is not provided"
    );
    tick(e);
    let now = e.clock_now_us();
    let _ = cursors_at(e, this, now);
    let (old, new) = (b.frequency(), if v == 0 { b.rate() } else { v });
    buffer(e, this).frequency = v;
    if old == new {
        return DS_OK;
    }
    let device = device_rate(e);
    if let Some(counter) = buffer(e, this).apl.as_mut() {
        counter.retune(new, device);
    }
    if let Play::Playing {
        looping,
        start,
        base,
        from,
        first,
        part,
        end,
        steps: steps @ (Steps::Dmo | Steps::Pass),
    } = buffer(e, this).play
    {
        // The write cursor goes on from where it is at the new frequency from the next period,
        // its position's hundredths carried; the play cursor goes on trailing it. A first `Play`
        // past its first period goes on from its write cursor (its lead included) as a later
        // `Play`; before its first period it stays a first `Play`.
        let n = periods(start, now);
        let write = write_after(base, first, part, old, steps, n);
        let held = u64::from(part) + n * u64::from(old);
        let (base, first) = if first && n == 0 {
            (base, true)
        } else {
            (write, false)
        };
        let steps = steps_of(e, this);
        buffer(e, this).play = Play::Playing {
            looping,
            start: now,
            base,
            from,
            first,
            part: (held % 100) as u32,
            end,
            steps,
        };
    }
    // DirectSound's rule for its converters (`dsound` `100581e0`): a move of more than 1/500 of
    // the rate turns on the device's second converter for every buffer, from then on.
    if old.abs_diff(new) > old / 500 {
        second_converter_on(e, now);
    }
    if matches!(buffer(e, this).play, Play::Playing { .. }) {
        heard(e, this, "SetFrequency");
    } else {
        refresh_voice(e, this.addr());
    }
    DS_OK
}

/// DirectSound turns on its second converter for the device (`dsound` `100581e0`): from then on
/// every buffer goes through it, and each one playing goes on from its write cursor with a new
/// one (`1005713d`, `super::resampler`: the write cursor read, the new chain built from there,
/// both cursor fields set to it as `Play` and `SetCurrentPosition` set them): both cursors on
/// the write cursor, the write cursor going on by the frames the new converter takes
/// ([`Steps::Apl`]) and the play cursor held there until the write cursor is three periods
/// past. The buffer whose `SetFrequency` turns it on goes the same way. (Read from the DLL; the
/// cursors of a buffer playing when it turns on are not measured.)
fn second_converter_on(e: &mut Engine, now: u64) {
    if e.kernel.sound.apl {
        return;
    }
    e.kernel.sound.apl = true;
    let device = device_rate(e);
    let playing: Vec<u32> = e
        .kernel
        .sound
        .buffers
        .iter()
        .filter(|(_, b)| matches!(b.play, Play::Playing { .. }))
        .map(|(&at, _)| at)
        .collect();
    for at in playing {
        let this = Ptr::new(at);
        let _ = cursors_at(e, this, now);
        let b = buffer(e, this);
        if let Play::Playing {
            looping,
            start,
            base,
            first,
            part,
            end,
            steps,
            ..
        } = b.play
        {
            let write = write_after(base, first, part, b.frequency(), steps, periods(start, now));
            b.apl = Some(super::resampler::AplFrames::new(
                b.frequency(),
                device,
                lanes(b),
            ));
            b.play = Play::Playing {
                looping,
                start: now,
                base: write,
                from: write,
                first: false,
                part: 0,
                end,
                steps: Steps::Apl { fed: 0, done: 0 },
            };
        }
    }
}

/// A buffer about to be heard, or heard with new settings: with the output on, its voice is
/// brought up to date ([`refresh_voice`]), and a rate the converter path does not translate is
/// named once on stderr (docs/SOUND.md keeps it open for the maintainer).
fn heard(e: &mut Engine, this: Ptr, call: &str) {
    let Some(rate) = e.kernel.sound.mix.as_ref().map(|m| m.rate) else {
        return;
    };
    let b = buffer(e, this).clone();
    let f = b.frequency();
    // Written out only where it is told (a `SetFrequency` each frame tells nothing).
    let what = |b: &Buffer| {
        format!(
            "{call} {:08x}: {} ch {} bit {} Hz, {} bytes{}{}",
            this.addr(),
            u16::from_le_bytes([b.format[2], b.format[3]]),
            u16::from_le_bytes([b.format[14], b.format[15]]),
            f,
            b.bytes,
            if b.flags & DSBCAPS_CTRL3D != 0 {
                ", 3D"
            } else {
                ""
            },
            b.effect.map_or(String::new(), |fx| format!(", {fx}")),
        )
    };
    let apl = e
        .kernel
        .sound
        .mix
        .as_ref()
        .is_some_and(|m| m.voices.lock().expect("the voices").apl);
    if f != rate && !apl && super::resampler::tables().is_none() {
        unheard(e, &what(&b));
    }
    refresh_voice(e, this.addr());
    if call == "Play" {
        // The log also names the level it is heard at: the volume, and for a 3D buffer its
        // position from the listener and the 3D stage's channel multipliers.
        let what = what(&b);
        let b = buffer(e, this).clone();
        let mut what = format!("{what}, volume {}", b.volume);
        if b.flags & DSBCAPS_CTRL3D != 0 {
            let l = listener_now(e);
            let h = super::ds3d::hear(&source3d(&b), &l);
            what += &format!(
                ", at {:?} (listener {:?}, min {} max {}), 3D gains {}/{}, {} Hz",
                b.position,
                l.position,
                b.min_distance,
                b.max_distance,
                h.gains.0,
                h.gains.1,
                h.frequency
            );
        }
        note(e, &what);
    }
}

/// Brings a buffer's voice up to date with its frequency and, for a 3D buffer, the 3D stage's
/// channel multipliers and Doppler frequency (`super::ds3d::hear`), and the device's choice of
/// converter: a frequency that moves by more than 1/500 of the rate turns on the second one
/// for every buffer (`super::resampler`).
fn refresh_voice(e: &mut Engine, at: u32) {
    let Some(rate) = e.kernel.sound.mix.as_ref().map(|m| m.rate) else {
        return;
    };
    let Some(b) = e.kernel.sound.buffers.get(&at) else {
        return;
    };
    if b.flags & DSBCAPS_PRIMARYBUFFER != 0 {
        return;
    }
    let (frequency, spatial) = if b.flags & DSBCAPS_CTRL3D != 0 {
        let h = super::ds3d::hear(&source3d(b), &listener_now(e));
        (h.frequency, Some(h.gains))
    } else {
        (b.frequency(), None)
    };
    // DirectSound's rule for its converters (`dsound` `100581e0`): a move of more than 1/500 of
    // the rate turns on the device's second converter for every buffer; a smaller one is the
    // playing converter's `SetRate` (`super::resampler`, at its next period). (`with_voice`
    // written out, so the buffers are read in place.)
    tick(e);
    let sound = &e.kernel.sound;
    let Some(mix) = &sound.mix else {
        return;
    };
    let mut voices = mix.voices.lock().expect("the voices");
    let m = &mut *voices;
    let Some(v) = m.all.get_mut(&at) else {
        return;
    };
    let old = v.rate;
    v.device = rate;
    v.spatial = spatial;
    v.rate = frequency;
    if old == 0 || old == frequency {
        return;
    }
    if old.abs_diff(frequency) > old / 500 && !m.apl {
        m.apl = true;
        for (b, _) in sound
            .buffers
            .iter()
            .filter(|(_, b)| b.flags & DSBCAPS_PRIMARYBUFFER == 0)
        {
            if let Some(v) = m.all.get_mut(b) {
                v.apl = true;
            }
        }
        // The cursors step by it too ([`Steps`]).
        drop(voices);
        let now = e.clock_now_us();
        second_converter_on(e, now);
    }
}

/// The 3D buffers' voices brought up to date (after the listener changed).
fn refresh_3d(e: &mut Engine) {
    if e.kernel.sound.mix.is_none() {
        return;
    }
    // In address order, found one after another, with no list: `refresh_voice` adds, removes
    // and reflags no buffer (when it turns the second converter on, it only moves the cursors
    // of the buffers that play).
    let mut from = 0;
    while let Some(at) = e
        .kernel
        .sound
        .buffers
        .range(from..)
        .find(|(_, b)| b.flags & DSBCAPS_CTRL3D != 0 && b.flags & DSBCAPS_PRIMARYBUFFER == 0)
        .map(|(&at, _)| at)
    {
        refresh_voice(e, at);
        let Some(next) = at.checked_add(1) else {
            break;
        };
        from = next;
    }
}

/// The listener's settings in effect, as the 3D stage reads them; without a listener the DLL's
/// defaults (at the origin, facing +z with +y up, factors 1), which leave positions as they are.
fn listener_now(e: &Engine) -> super::ds3d::Listener {
    let l = e.kernel.sound.listeners.values().next().map(|(_, l)| *l);
    match l {
        Some(l) => super::ds3d::Listener {
            position: l.position,
            velocity: l.velocity,
            front: l.front,
            top: l.top,
            distance_factor: l.distance_factor,
            rolloff: l.rolloff,
            doppler: l.doppler,
        },
        None => super::ds3d::Listener {
            position: [0.0; 3],
            velocity: [0.0; 3],
            front: [0.0, 0.0, 1.0],
            top: [0.0, 1.0, 0.0],
            distance_factor: 1.0,
            rolloff: 1.0,
            doppler: 1.0,
        },
    }
}

/// A 3D buffer's settings in effect, as the 3D stage reads them.
fn source3d(b: &Buffer) -> super::ds3d::Source {
    super::ds3d::Source {
        position: b.position,
        velocity: b.velocity,
        min: b.min_distance,
        max: b.max_distance,
        mute_at_max: b.flags & DSBCAPS_MUTE3DATMAXDISTANCE != 0,
        volume: b.volume,
        frequency: b.frequency(),
    }
}

/// Device periods from `start` to `now` (the platform clock's 10 ms steps).
fn periods(start: u64, now: u64) -> u64 {
    now / PERIOD_US - start / PERIOD_US
}

/// The hundredths of a frame a position `part` hundredths past a whole frame is past one after
/// `n` periods at `frequency`.
fn part_after(part: u32, n: u64, frequency: u32) -> u32 {
    ((u64::from(part) + n * u64::from(frequency)) % 100) as u32
}

/// The rate DirectSound mixes at: the output's when it runs, else the default device's
/// (`super::sound_out::device_rate`; the measured device's 48000 when Windows names none, where
/// DirectSound would make no device); the tests' 48000.
fn device_rate(e: &Engine) -> u32 {
    if let Some(m) = &e.kernel.sound.mix {
        return m.rate;
    }
    #[cfg(test)]
    {
        48000
    }
    #[cfg(not(test))]
    {
        super::sound_out::device_rate().unwrap_or(48000)
    }
}

/// Whether the second converter takes a buffer through its four-lane kernel (16-bit mono,
/// `super::resampler::Apl`).
fn lanes(b: &Buffer) -> bool {
    let (channels, bits) = (
        u16::from_le_bytes([b.format[2], b.format[3]]),
        u16::from_le_bytes([b.format[14], b.format[15]]),
    );
    channels == 1 && bits == 16
}

/// How a buffer's write cursor steps from a `Play` or a move now ([`Steps`]): by the device's
/// converter and, for the first, whether it converts the buffer's frequency. With the second, the
/// buffer gets its frame counter at its first `Play`.
fn steps_of(e: &mut Engine, this: Ptr) -> Steps {
    let device = device_rate(e);
    let apl = e.kernel.sound.apl;
    let b = buffer(e, this);
    if apl {
        if b.apl.is_none() {
            b.apl = Some(super::resampler::AplFrames::new(
                b.frequency(),
                device,
                lanes(b),
            ));
        }
        Steps::Apl { fed: 0, done: 0 }
    } else if b.frequency() == device {
        Steps::Pass
    } else {
        Steps::Dmo
    }
}

/// A buffer playing through the first converter: its write cursor `n` periods after `start`
/// (frames counted on round it): from `base`, its position `part` hundredths past a whole frame,
/// at `frequency`, rounded up, with the lead of a `first` Play ([`Steps`]). Measured: the first
/// Play (`dsound-capture` `cursors`, `replays`, `setpos`, `endphase`, `endstop`, `freqphase`), a
/// later Play after it ended, a `Stop` or a `SetCurrentPosition` (`replays`, 2026-10-30), moved
/// while it plays, stopped, ended or retuned (2026-10-31, `setpos`, `endphase`, `freqphase`): on
/// by the steps it was taking.
fn write_after(base: u64, first: bool, part: u32, frequency: u32, steps: Steps, n: u64) -> u64 {
    let (f, part) = (u64::from(frequency), u64::from(part));
    let lead = if first { steps.lead(n) } else { 0 };
    base + (part + n * f).div_ceil(100) - part.div_ceil(100) + lead
}

/// The period of its `Play` in which a playing buffer that does not loop ends: the first in which
/// its write cursor reaches `end` (measured, [`cursors`]), at most `now_n` (where it is known to
/// have).
fn end_period(at: impl Fn(u64) -> u64, end: u64, now_n: u64) -> u64 {
    let (mut lo, mut hi) = (0, now_n);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if at(mid) >= end {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}

/// Brings a playing non-looping buffer that reached its end to `Ended`; answers the play and
/// write cursors in bytes.
fn cursors(e: &mut Engine, this: Ptr) -> (u32, u32) {
    tick(e);
    let now = e.clock_now_us();
    cursors_at(e, this, now)
}

/// [`cursors`] at the clock reading `now`.
fn cursors_at(e: &mut Engine, this: Ptr, now: u64) -> (u32, u32) {
    let b = buffer(e, this);
    match b.play {
        Play::Stopped { at, .. } => (at, at),
        Play::Ended { .. } => (0, 0),
        Play::Playing {
            looping,
            start,
            base,
            from,
            first,
            part,
            end,
            steps,
        } => {
            let (f, len, align) = (b.frequency(), b.frames(), b.align());
            let n = periods(start, now);
            // It ends in the period its write cursor reaches its end, which it never reads while
            // it plays (measured, `dsound-capture endstop`); its steps go on from that period
            // (`endphase`).
            let write = if let Steps::Apl { mut fed, mut done } = steps {
                // The second converter's frames, period by period from where they were counted.
                let counter = b
                    .apl
                    .as_mut()
                    .expect("the second converter's frame counter");
                while done < n {
                    fed += counter.period();
                    done += 1;
                    if !looping && base + fed >= end {
                        b.play = Play::Ended { part: 0 };
                        return (0, 0);
                    }
                }
                if let Play::Playing { steps, .. } = &mut b.play {
                    *steps = Steps::Apl { fed, done };
                }
                base + fed
            } else {
                let at = |k: u64| write_after(base, first, part, f, steps, k);
                let write = at(n);
                if !looping && write >= end {
                    let period = end_period(at, end, n);
                    b.play = Play::Ended {
                        part: part_after(part, period, f),
                    };
                    return (0, 0);
                }
                write
            };
            // The play cursor trails the write cursor by three periods at the format's rate, held
            // where it was played from until then.
            let lag = 3 * u64::from(b.rate()) / 100;
            let play = if write - from > lag {
                write - lag
            } else {
                from
            };
            // Frames counted on round the buffer: wrapped (a buffer that does not loop stops short
            // of its `end`).
            let wrap = |x: u64| x % len;
            ((wrap(play) as u32) * align, (wrap(write) as u32) * align)
        }
    }
}

/// `GetCurrentPosition(play, write)`: the cursors (module documentation); either pointer may be
/// NULL.
fn get_current_position(e: &mut Engine, this: Ptr, play: Ptr, write: Ptr) -> u32 {
    let _ = secondary(e, this, "GetCurrentPosition");
    let (p, w) = cursors(e, this);
    if !play.is_null() {
        e.mem.set_u32(play.addr(), p);
    }
    if !write.is_null() {
        e.mem.set_u32(write.addr(), w);
    }
    DS_OK
}

/// `GetStatus(out)`: `DSBSTATUS_PLAYING`, with `DSBSTATUS_LOOPING` when looping; 0 when stopped
/// or played to the end (measured).
fn get_status(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let _ = secondary(e, this, "GetStatus");
    let _ = cursors(e, this);
    let status = match buffer(e, this).play {
        Play::Playing { looping, .. } => {
            DSBSTATUS_PLAYING | if looping { DSBSTATUS_LOOPING } else { 0 }
        }
        _ => 0,
    };
    e.mem.set_u32(out.addr(), status);
    DS_OK
}

/// `Lock(offset, bytes, p1, n1, p2, n2, flags)`: with `DSBLOCK_ENTIREBUFFER` (offset 0) or 0,
/// the region from `offset`, wrapping into a second region at the start. `bytes` 0 or larger
/// than the buffer without `DSBLOCK_ENTIREBUFFER` is `DSERR_INVALIDPARAM`; the write cursor flag
/// and a second region the caller has no pointer for are not provided.
#[allow(clippy::too_many_arguments)] // The interface's receiver and its seven words.
fn lock(
    e: &mut Engine,
    this: Ptr,
    offset: u32,
    bytes: u32,
    p1: Ptr,
    n1: Ptr,
    p2: Ptr,
    n2: Ptr,
    flags: u32,
) -> u32 {
    let b = secondary(e, this, "Lock");
    let (offset, bytes) = match flags {
        DSBLOCK_ENTIREBUFFER => {
            assert!(
                offset == 0,
                "Lock of the whole buffer from {offset} is not provided"
            );
            (0, b.bytes)
        }
        0 => {
            if bytes == 0 || bytes > b.bytes || offset >= b.bytes {
                return DSERR_INVALIDPARAM;
            }
            (offset, bytes)
        }
        _ => panic!("IDirectSoundBuffer::Lock flags {flags:#x} are not provided"),
    };
    let first = bytes.min(b.bytes - offset);
    let second = bytes - first;
    assert!(
        second == 0 || !p2.is_null(),
        "Lock wrapping round without a second pointer is not provided"
    );
    e.mem.set_u32(p1.addr(), b.data + offset);
    e.mem.set_u32(n1.addr(), first);
    if !p2.is_null() {
        e.mem
            .set_u32(p2.addr(), if second == 0 { 0 } else { b.data });
        e.mem.set_u32(n2.addr(), second);
    }
    buffer(e, this).locks += 1;
    DS_OK
}

/// `Unlock(p1, n1, p2, n2)`: ends a `Lock`; an unlock without a lock is not provided.
fn unlock(e: &mut Engine, this: Ptr, _p1: u32, _n1: u32, _p2: u32, _n2: u32) -> u32 {
    let b = buffer(e, this);
    assert!(
        b.locks > 0,
        "IDirectSoundBuffer::Unlock without a Lock is not provided"
    );
    b.locks -= 1;
    let (data, bytes) = (b.data, b.bytes);
    if e.kernel.sound.mix.is_some() {
        // `with_voice` written out: the samples are read straight into the voice's copy, which
        // has the buffer's size (a new copy otherwise), instead of into a new one each time.
        tick(e);
        if let Some(m) = &e.kernel.sound.mix {
            let mut voices = m.voices.lock().expect("the voices");
            if let Some(voice) = voices.all.get_mut(&this.addr()) {
                // With the distortion the samples are the effect's input; the voice plays its
                // processed copy.
                let to = match voice.fx.as_mut() {
                    Some(fx) => &mut fx.source,
                    None => &mut voice.data,
                };
                if to.len() == bytes as usize {
                    e.mem.read(data, to);
                } else {
                    *to = e.mem.bytes(data, bytes);
                }
            }
        }
    }
    DS_OK
}

/// `Play(0, 0, flags)` with flags 0 or `DSBPLAY_LOOPING`: plays from where the buffer stopped
/// (on the platform clock), in the steps it was taking (its `part`, measured); a playing buffer
/// only takes the looping flag (turned not looping, it plays out the lap it is in and ends there,
/// measured with `dsound-capture endstop`); a buffer that played to its end plays from its
/// start, measured. A stopped buffer is never at its end: `SetCurrentPosition` refuses it, and a
/// playing buffer ends in the period its write cursor reaches its end, so `Stop` never leaves it
/// there (measured, `endstop`: stopped in the period before, `Play` plays the rest).
fn play(e: &mut Engine, this: Ptr, reserved: u32, priority: u32, flags: u32) -> u32 {
    let b = secondary(e, this, "Play");
    assert!(
        reserved == 0 && priority == 0 && flags & !DSBPLAY_LOOPING == 0,
        "IDirectSoundBuffer::Play({reserved:#x}, {priority:#x}, {flags:#x}) is not provided"
    );
    let looping = flags & DSBPLAY_LOOPING != 0;
    tick(e);
    let now = e.clock_now_us();
    let _ = cursors_at(e, this, now);
    let first = !buffer(e, this).played;
    let steps = steps_of(e, this);
    let play = match buffer(e, this).play {
        Play::Stopped { at, part } => Play::Playing {
            looping,
            start: now,
            base: u64::from(at / b.align()),
            from: u64::from(at / b.align()),
            first,
            part,
            end: b.frames(),
            steps,
        },
        Play::Playing {
            looping: was,
            start,
            base,
            from,
            first,
            part,
            end,
            steps,
        } => {
            // Turned not looping, it plays out the lap it is in (measured, `endstop`).
            let end = if was && !looping {
                let w = match steps {
                    // Counted up to now above.
                    Steps::Apl { fed, .. } => base + fed,
                    _ => write_after(base, first, part, b.frequency(), steps, periods(start, now)),
                };
                (w / b.frames() + 1) * b.frames()
            } else {
                end
            };
            Play::Playing {
                looping,
                start,
                base,
                from,
                first,
                part,
                end,
                steps,
            }
        }
        // Measured (2026-10-30, `dsound-capture replays`): a buffer that played to its end
        // plays again from its start (2026-10-31, `endphase`: in the steps it was taking).
        Play::Ended { part } => Play::Playing {
            looping,
            start: now,
            base: 0,
            from: 0,
            first,
            part,
            end: b.frames(),
            steps,
        },
    };
    buffer(e, this).played = true;
    let was_playing = matches!(buffer(e, this).play, Play::Playing { .. });
    buffer(e, this).play = play;
    heard(e, this, "Play");
    let Play::Playing { base, .. } = play else {
        unreachable!()
    };
    with_voice(e, this.addr(), |m, at| {
        if let Some(voice) = m.all.get_mut(&at) {
            voice.looping = looping;
            if !was_playing {
                // The effect starts over 32 frames before the play position (`10046fb8`).
                if let Some(fx) = voice.fx.as_mut() {
                    fx.start(&mut voice.data, base as usize);
                }
                voice.apl = m.apl;
                voice.start(base as usize);
                m.playing.retain(|&p| p != at);
                m.playing.insert(0, at);
            }
        }
    });
    DS_OK
}

/// `Stop()`: both cursors stay at the write cursor (at 0 for a buffer that played to its end),
/// measured; where its position was in hundredths of a frame is kept for its next `Play` (`part`,
/// measured with `dsound-capture endphase` after stops at 3 to 10 periods).
fn stop(e: &mut Engine, this: Ptr) -> u32 {
    let b = secondary(e, this, "Stop");
    if matches!(b.play, Play::Playing { .. }) {
        note(e, &format!("Stop {:08x}", this.addr()));
    }
    tick(e);
    let now = e.clock_now_us();
    let (_, write) = cursors_at(e, this, now);
    buffer(e, this).play = match (b.play, buffer(e, this).play) {
        (Play::Stopped { at, part }, _) => Play::Stopped { at, part },
        // Measured: a buffer that played to its end keeps both cursors at 0.
        (_, Play::Ended { part }) => Play::Stopped { at: 0, part },
        (Play::Playing { start, part, .. }, _) => Play::Stopped {
            at: write,
            part: part_after(part, periods(start, now), b.frequency()),
        },
        (Play::Ended { part }, _) => Play::Stopped { at: 0, part },
    };
    with_voice(e, this.addr(), |m, at| {
        if let Some(voice) = m.all.get_mut(&at) {
            voice.position = None;
        }
        m.playing.retain(|&p| p != at);
    });
    DS_OK
}

/// `SetCurrentPosition(at)` (measured 2026-10-31, `dsound-capture setpos`, module documentation):
/// `at` down to a whole frame. At or past the end `DSERR_INVALIDPARAM` and nothing changes,
/// whether the buffer plays or not. A stopped buffer, or one that played to its end, stays
/// stopped with both cursors there, keeping the steps it was taking for its next `Play`. A
/// playing buffer has both cursors there at once and goes on playing from there, looping as it
/// was: the write cursor moves on from it by the steps the buffer was taking (`part`,
/// [`cursors`]; a first `Play` moved before its first period still has its lead to come), the
/// play cursor stays on it until the write cursor is three periods past, and a buffer that does
/// not loop ends when the write cursor reaches its end.
///
/// What is heard (`dsmix-measure` `DSMIX_SETPOS` with `DSMIX_HOOK`, 2026-10-31, 42 captures at
/// 48000, 22050 and 44100 Hz, mono and stereo, looping and not, with the distortion and
/// without, every sample): what DirectSound had mixed (up to the write cursor) plays out, and
/// its next period mixes from the new place through the buffer's converter as it was, kept
/// with all it holds ([`super::mixer::Voice::move_to`]); a distortion starts over there as at
/// `Play` (cleared, from 32 frames before). Here the voice moves at its next period. The moved
/// voice keeps its place in the sum (`DSMIX_SETPOS_ORDER`, 12 captures, three runs, of three
/// voices played A, B, C, newest first in the sum, one of them moved: moved, the oldest, A, is
/// still added last, so not to the front; moved, B or C is not added after A, so not to the
/// end; with three voices the order of the two added first cannot show).
///
/// Not measured: a voice the output has already played to its end (the output runs up to
/// 100 ms ahead of the platform clock, `Pace`) while the buffer still plays on the platform
/// clock starts again there, with a new converter, in its place in the sum.
fn set_current_position(e: &mut Engine, this: Ptr, at: u32) -> u32 {
    let b = secondary(e, this, "SetCurrentPosition");
    tick(e);
    let now = e.clock_now_us();
    let _ = cursors_at(e, this, now);
    if at >= b.bytes {
        return DSERR_INVALIDPARAM;
    }
    let at = at - at % b.align();
    let frame = at / b.align();
    let state = buffer(e, this).play;
    buffer(e, this).play = match state {
        Play::Playing {
            looping,
            start,
            first,
            part,
            ..
        } => {
            let n = periods(start, now);
            // The second converter's frames counted from here, the counter going on.
            let steps = steps_of(e, this);
            Play::Playing {
                looping,
                start: now,
                base: u64::from(frame),
                from: u64::from(frame),
                // A first `Play` moved before its first period still has its lead to come.
                first: first && n == 0,
                part: part_after(part, n, b.frequency()),
                end: b.frames(),
                steps,
            }
        }
        Play::Stopped { part, .. } | Play::Ended { part } => Play::Stopped { at, part },
    };
    if matches!(state, Play::Playing { .. }) {
        note(
            e,
            &format!("SetCurrentPosition {:08x} to {at:#x}", this.addr()),
        );
        with_voice(e, this.addr(), |m, key| {
            let listed = m.playing.contains(&key);
            if let Some(voice) = m.all.get_mut(&key).filter(|_| listed) {
                let frame = frame as usize;
                if let Some(fx) = voice.fx.as_mut() {
                    fx.start(&mut voice.data, frame);
                }
                if voice.position.is_some() {
                    voice.move_to(frame);
                } else {
                    voice.apl = m.apl;
                    voice.start(frame);
                }
            }
        });
    }
    DS_OK
}

fn effect_name(e: &Engine, class: u32) -> Option<&'static str> {
    let id = e.mem.bytes(class, 16);
    if id == GUID_DISTORTION {
        Some("distortion")
    } else if id == GUID_I3DL2 {
        Some("i3dl2")
    } else {
        None
    }
}

/// `SetFX(count, descriptions, results)` with one effect, the distortion or the I3DL2 reverb,
/// on a stopped, unlocked buffer: the captured answer (and result code when `results` is
/// given). Other effects, counts or descriptions are not provided.
fn set_fx(e: &mut Engine, this: Ptr, count: u32, descriptions: Ptr, results: Ptr) -> u32 {
    let b = secondary(e, this, "SetFX");
    let d = descriptions.addr();
    let name = effect_name(e, d + 8);
    assert!(
        count == 1
            && e.mem.u32(d) == EFFECT_DESC_SIZE
            && e.mem.u32(d + 4) == 0
            && e.mem.u32(d + 0x18) == 0
            && e.mem.u32(d + 0x1c) == 0
            && name.is_some(),
        "IDirectSoundBuffer8::SetFX with these effects is not provided"
    );
    let name = name.expect("checked");
    assert!(
        b.locks == 0 && matches!(b.play, Play::Stopped { .. }),
        "SetFX on a playing or locked buffer is not provided"
    );
    let fx = b.flags & DSBCAPS_CTRLFX != 0;
    let (result, code, _) = table(e).effect(name, fx);
    if !results.is_null() && fx {
        e.mem.set_u32(results.addr(), code);
    }
    if result == DS_OK {
        let b = buffer(e, this);
        b.effect = Some(name);
        b.fx_params = DISTORTION_DEFAULTS;
        // The voice now plays the effect's output (`super::dsfx`).
        if name == "distortion" && e.kernel.sound.mix.is_some() {
            let b = buffer(e, this).clone();
            with_voice(e, this.addr(), |m, at| {
                if let Some(v) = m.all.get_mut(&at) {
                    let source = std::mem::take(&mut v.data);
                    v.data = source.clone();
                    v.fx = Some(Box::new(fx_chain(&b, source)));
                }
            });
        }
    }
    result
}

/// `GetObjectInPath(class, 0, iid, out)` for the effect `SetFX` set: the captured answer, and for
/// the distortion a new `IDirectSoundFXDistortion8` on the buffer's effect.
fn get_object_in_path(
    e: &mut Engine,
    this: Ptr,
    class: Ptr,
    index: u32,
    iid: Ptr,
    out: Ptr,
) -> u32 {
    let b = secondary(e, this, "GetObjectInPath");
    let name = effect_name(e, class.addr());
    let interface = e.mem.bytes(iid.addr(), 16);
    let fits = match name {
        Some("distortion") => interface == IID_DISTORTION8,
        Some("i3dl2") => interface == IID_I3DL2_8,
        _ => false,
    };
    assert!(
        index == 0 && fits,
        "GetObjectInPath for this object is not provided"
    );
    let name = name.expect("checked");
    let fx = b.flags & DSBCAPS_CTRLFX != 0;
    let (_, _, result) = table(e).effect(name, fx);
    if result != DS_OK {
        return result;
    }
    assert!(
        b.effect == Some(name) && name == "distortion",
        "GetObjectInPath({name}) without SetFX is not provided"
    );
    let table = e.kernel.sound.distortion_vtable;
    let at = object(e, table);
    e.kernel.sound.distortions.insert(at, (1, this.addr()));
    e.mem.set_u32(out.addr(), at);
    DS_OK
}

fn distortion(e: &mut Engine, this: Ptr) -> &mut (u32, u32) {
    e.kernel
        .sound
        .distortions
        .get_mut(&this.addr())
        .expect("a distortion of this layer")
}

fn distortion_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let d = distortion(e, this);
    d.0 += 1;
    d.0
}

fn distortion_release(e: &mut Engine, this: Ptr) -> u32 {
    let d = distortion(e, this);
    d.0 -= 1;
    let left = d.0;
    if left == 0 {
        e.kernel.sound.distortions.remove(&this.addr());
        e.mem.free(this.addr());
    }
    left
}

/// `SetAllParameters(params)`: `DSFXDistortion` within its ranges (module documentation), else
/// `DSERR_INVALIDPARAM`. A NaN is not provided. The effect takes the new coefficients and keeps
/// its state (`1000dea0`).
fn distortion_set_all_parameters(e: &mut Engine, this: Ptr, params: Ptr) -> u32 {
    let p: [f32; 5] = std::array::from_fn(|i| e.mem.f32(params.addr() + 4 * i as u32));
    assert!(
        p.iter().all(|f| !f.is_nan()),
        "a NaN distortion parameter is not provided"
    );
    if !distortion_ok(p) {
        return DSERR_INVALIDPARAM;
    }
    let owner = distortion(e, this).1;
    if let Some(b) = e.kernel.sound.buffers.get_mut(&owner) {
        b.fx_params = p;
        let b = b.clone();
        if e.kernel.sound.mix.is_some() {
            let c = super::dsfx::Coefficients::new(b.rate(), p, edge_table());
            with_voice(e, owner, |m, at| {
                if let Some(fx) = m.all.get_mut(&at).and_then(|v| v.fx.as_mut()) {
                    fx.fx.c = c;
                }
            });
        }
    }
    DS_OK
}

/// The buffer a 3D interface belongs to.
fn owner(e: &Engine, this: Ptr) -> u32 {
    *e.kernel
        .sound
        .buffers3d
        .get(&this.addr())
        .expect("an IDirectSound3DBuffer of this layer")
}

/// `AddRef()` on the 3D interface: its own count (measured, [`release_buffer`]).
fn buffer3d_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    if let Some(n) = e.kernel.sound.dead3d.get_mut(&this.addr()) {
        *n = n.wrapping_add(1);
        return *n;
    }
    let at = owner(e, this);
    let b = buffer(e, Ptr::new(at));
    b.references3d = b.references3d.wrapping_add(1);
    b.references3d
}

/// `Release()` on the 3D interface: its own count, which wraps below 0 (measured,
/// [`release_buffer`]); the buffer goes once both counts are 0.
fn buffer3d_release(e: &mut Engine, this: Ptr) -> u32 {
    if let Some(n) = e.kernel.sound.dead3d.get_mut(&this.addr()) {
        *n = n.wrapping_sub(1);
        return *n;
    }
    let at = owner(e, this);
    let b = buffer(e, Ptr::new(at));
    b.references3d = b.references3d.wrapping_sub(1);
    let left = b.references3d;
    if b.references == 0 && left == 0 {
        free_buffer(e, at);
    }
    left
}

/// Whether a 3D setting waits for `CommitDeferredSettings` (`DS3D_DEFERRED`) or applies at once
/// (`DS3D_IMMEDIATE`); other flags are not provided.
fn deferred(apply: u32) -> bool {
    assert!(
        apply <= DS3D_DEFERRED,
        "a 3D setting with apply {apply:#x} is not provided"
    );
    apply == DS3D_DEFERRED
}

/// Sets one of a 3D buffer's settings: at once (and heard), or kept for the commit.
fn set_3d(
    e: &mut Engine,
    this: Ptr,
    apply: u32,
    now: impl FnOnce(&mut Buffer),
    later: impl FnOnce(&mut BufferPending),
) {
    let at = owner(e, this);
    if deferred(apply) {
        later(&mut buffer(e, Ptr::new(at)).pending3d);
    } else {
        now(buffer(e, Ptr::new(at)));
        refresh_voice(e, at);
    }
}

/// `SetMinDistance(v, apply)`: above 0, else `DSERR_INVALIDPARAM` (measured: 0 refused). NaN is
/// not provided.
fn set_min_distance(e: &mut Engine, this: Ptr, v: f32, apply: u32) -> u32 {
    assert!(!v.is_nan(), "a NaN distance is not provided");
    if v <= 0.0 {
        let _ = deferred(apply);
        return DSERR_INVALIDPARAM;
    }
    set_3d(
        e,
        this,
        apply,
        |b| b.min_distance = v,
        |p| p.min_distance = Some(v),
    );
    DS_OK
}

/// `SetMaxDistance(v, apply)`: above 0, else `DSERR_INVALIDPARAM` (measured: -1 refused).
fn set_max_distance(e: &mut Engine, this: Ptr, v: f32, apply: u32) -> u32 {
    assert!(!v.is_nan(), "a NaN distance is not provided");
    if v <= 0.0 {
        let _ = deferred(apply);
        return DSERR_INVALIDPARAM;
    }
    set_3d(
        e,
        this,
        apply,
        |b| b.max_distance = v,
        |p| p.max_distance = Some(v),
    );
    DS_OK
}

/// `SetPosition(x, y, z, apply)`: the position (a vector longer than `f32::MAX` shortened,
/// `super::ds3d::clamp_length`).
fn buffer3d_set_position(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32, apply: u32) -> u32 {
    assert!(
        ![x, y, z].iter().any(|f| f.is_nan()),
        "a NaN position is not provided"
    );
    let v = super::ds3d::clamp_length([x, y, z]);
    set_3d(e, this, apply, |b| b.position = v, |p| p.position = Some(v));
    DS_OK
}

/// `SetVelocity(x, y, z, apply)`: the velocity, for Doppler (`super::ds3d::doppler_frequency`).
fn buffer3d_set_velocity(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32, apply: u32) -> u32 {
    assert!(
        ![x, y, z].iter().any(|f| f.is_nan()),
        "a NaN velocity is not provided"
    );
    let v = super::ds3d::clamp_length([x, y, z]);
    set_3d(e, this, apply, |b| b.velocity = v, |p| p.velocity = Some(v));
    DS_OK
}

fn listener(e: &mut Engine, this: Ptr) -> &mut (u32, Listener) {
    e.kernel
        .sound
        .listeners
        .get_mut(&this.addr())
        .expect("an IDirectSound3DListener of this layer")
}

fn listener_add_ref(e: &mut Engine, this: Ptr) -> u32 {
    let l = listener(e, this);
    l.0 += 1;
    l.0
}

fn listener_release(e: &mut Engine, this: Ptr) -> u32 {
    let l = listener(e, this);
    l.0 -= 1;
    let left = l.0;
    if left == 0 {
        e.kernel.sound.listeners.remove(&this.addr());
        for b in e.kernel.sound.buffers.values_mut() {
            if b.listener == this.addr() {
                b.listener = 0;
            }
        }
        e.mem.free(this.addr());
    }
    left
}

fn put3(e: &mut Engine, at: u32, v: [f32; 3]) {
    for (i, f) in v.into_iter().enumerate() {
        e.mem.set_f32(at + 4 * i as u32, f);
    }
}

fn get3(e: &Engine, at: u32) -> [f32; 3] {
    [e.mem.f32(at), e.mem.f32(at + 4), e.mem.f32(at + 8)]
}

/// `GetAllParameters(params)`: `params->dwSize` must be 0x40; the listener's state.
fn listener_get_all_parameters(e: &mut Engine, this: Ptr, params: Ptr) -> u32 {
    let p = params.addr();
    if e.mem.u32(p) != LISTENER_SIZE {
        return DSERR_INVALIDPARAM;
    }
    let l = listener(e, this).1;
    put3(e, p + 4, l.position);
    put3(e, p + 0x10, l.velocity);
    put3(e, p + 0x1c, l.front);
    put3(e, p + 0x28, l.top);
    e.mem.set_f32(p + 0x34, l.distance_factor);
    e.mem.set_f32(p + 0x38, l.rolloff);
    e.mem.set_f32(p + 0x3c, l.doppler);
    DS_OK
}

/// `GetPosition(out)`.
fn listener_get_position(e: &mut Engine, this: Ptr, out: Ptr) -> u32 {
    let position = listener(e, this).1.position;
    put3(e, out.addr(), position);
    DS_OK
}

/// Sets one of the listener's settings: at once (and every 3D buffer heard again), or kept for
/// the commit.
fn set_listener(
    e: &mut Engine,
    this: Ptr,
    apply: u32,
    now: impl FnOnce(&mut Listener),
    later: impl FnOnce(&mut ListenerPending),
) {
    if deferred(apply) {
        later(&mut listener(e, this).1.pending);
    } else {
        now(&mut listener(e, this).1);
        refresh_3d(e);
    }
}

/// The factors' ranges (the DLL's checks, the limits dsound.h names): distance factor
/// `FLT_MIN..=FLT_MAX`, rolloff and Doppler factors 0..=10.
fn distance_factor_ok(v: f32) -> bool {
    (f32::MIN_POSITIVE..=f32::MAX).contains(&v)
}

fn factor_ok(v: f32) -> bool {
    (0.0..=10.0).contains(&v)
}

/// `SetAllParameters(params, apply)`: `params->dwSize` must be 0x40 and the factors and the
/// orientation valid (`10034620`), else `DSERR_INVALIDPARAM`; the vectors are stored as
/// `SetPosition`, `SetVelocity` and `SetOrientation` store them.
fn listener_set_all_parameters(e: &mut Engine, this: Ptr, params: Ptr, apply: u32) -> u32 {
    let p = params.addr();
    let deferred = deferred(apply);
    if e.mem.u32(p) != LISTENER_SIZE {
        return DSERR_INVALIDPARAM;
    }
    let (distance_factor, rolloff, doppler) = (
        e.mem.f32(p + 0x34),
        e.mem.f32(p + 0x38),
        e.mem.f32(p + 0x3c),
    );
    let (front, top) = (get3(e, p + 0x1c), get3(e, p + 0x28));
    if !distance_factor_ok(distance_factor) || !factor_ok(doppler) {
        return DSERR_INVALIDPARAM;
    }
    if front.iter().chain(&top).any(|f| f.is_nan()) {
        return DSERR_INVALIDPARAM;
    }
    let Some(orientation) = super::ds3d::orientation(front, top) else {
        return DSERR_INVALIDPARAM;
    };
    if !factor_ok(rolloff) {
        return DSERR_INVALIDPARAM;
    }
    let position = super::ds3d::clamp_length(get3(e, p + 4));
    let velocity = super::ds3d::clamp_length(get3(e, p + 0x10));
    let pending = ListenerPending {
        position: Some(position),
        velocity: Some(velocity),
        orientation: Some(orientation),
        distance_factor: Some(distance_factor),
        rolloff: Some(rolloff),
        doppler: Some(doppler),
    };
    let l = &mut listener(e, this).1;
    if deferred {
        l.pending = pending;
    } else {
        apply_listener(l, pending);
        refresh_3d(e);
    }
    DS_OK
}

/// The settings in `p` made the listener's.
fn apply_listener(l: &mut Listener, p: ListenerPending) {
    if let Some(v) = p.position {
        l.position = v;
    }
    if let Some(v) = p.velocity {
        l.velocity = v;
    }
    if let Some((front, top)) = p.orientation {
        l.front = front;
        l.top = top;
    }
    if let Some(v) = p.distance_factor {
        l.distance_factor = v;
    }
    if let Some(v) = p.rolloff {
        l.rolloff = v;
    }
    if let Some(v) = p.doppler {
        l.doppler = v;
    }
}

fn listener_set_distance_factor(e: &mut Engine, this: Ptr, value: f32, apply: u32) -> u32 {
    if !distance_factor_ok(value) {
        let _ = deferred(apply);
        return DSERR_INVALIDPARAM;
    }
    set_listener(
        e,
        this,
        apply,
        |l| l.distance_factor = value,
        |p| p.distance_factor = Some(value),
    );
    DS_OK
}

fn listener_set_doppler_factor(e: &mut Engine, this: Ptr, value: f32, apply: u32) -> u32 {
    if !factor_ok(value) {
        let _ = deferred(apply);
        return DSERR_INVALIDPARAM;
    }
    set_listener(
        e,
        this,
        apply,
        |l| l.doppler = value,
        |p| p.doppler = Some(value),
    );
    DS_OK
}

fn listener_set_rolloff_factor(e: &mut Engine, this: Ptr, value: f32, apply: u32) -> u32 {
    if !factor_ok(value) {
        let _ = deferred(apply);
        return DSERR_INVALIDPARAM;
    }
    set_listener(
        e,
        this,
        apply,
        |l| l.rolloff = value,
        |p| p.rolloff = Some(value),
    );
    DS_OK
}

/// `SetOrientation(front, top, apply)`: both made unit length and the top made square to the
/// front (`super::ds3d::orientation`, the DLL's `SetOrientation` checks); a zero, parallel or
/// NaN pair `DSERR_INVALIDPARAM`.
#[allow(clippy::too_many_arguments)] // The interface's receiver, six floats and the apply word.
fn listener_set_orientation(
    e: &mut Engine,
    this: Ptr,
    fx: f32,
    fy: f32,
    fz: f32,
    tx: f32,
    ty: f32,
    tz: f32,
    apply: u32,
) -> u32 {
    let _ = deferred(apply);
    if [fx, fy, fz, tx, ty, tz].iter().any(|f| f.is_nan()) {
        return DSERR_INVALIDPARAM;
    }
    let Some(o) = super::ds3d::orientation([fx, fy, fz], [tx, ty, tz]) else {
        return DSERR_INVALIDPARAM;
    };
    set_listener(
        e,
        this,
        apply,
        |l| (l.front, l.top) = o,
        |p| p.orientation = Some(o),
    );
    DS_OK
}

/// `SetPosition(x, y, z, apply)` (a vector longer than `f32::MAX` shortened).
fn listener_set_position(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32, apply: u32) -> u32 {
    let v = super::ds3d::clamp_length([x, y, z]);
    set_listener(e, this, apply, |l| l.position = v, |p| p.position = Some(v));
    DS_OK
}

/// `SetVelocity(x, y, z, apply)` (for Doppler).
fn listener_set_velocity(e: &mut Engine, this: Ptr, x: f32, y: f32, z: f32, apply: u32) -> u32 {
    let v = super::ds3d::clamp_length([x, y, z]);
    set_listener(e, this, apply, |l| l.velocity = v, |p| p.velocity = Some(v));
    DS_OK
}

/// `CommitDeferredSettings()`: the listener's and every 3D buffer's settings kept with
/// `DS3D_DEFERRED` take effect (each one that was set; the others stay), then every 3D buffer is
/// heard again. `DS_OK`.
fn listener_commit(e: &mut Engine, this: Ptr) -> u32 {
    let l = &mut listener(e, this).1;
    let pending = std::mem::take(&mut l.pending);
    apply_listener(l, pending);
    for b in e.kernel.sound.buffers.values_mut() {
        let p = std::mem::take(&mut b.pending3d);
        if let Some(v) = p.position {
            b.position = v;
        }
        if let Some(v) = p.velocity {
            b.velocity = v;
        }
        if let Some(v) = p.min_distance {
            b.min_distance = v;
        }
        if let Some(v) = p.max_distance {
            b.max_distance = v;
        }
    }
    refresh_3d(e);
    DS_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A made-up capture (not a real machine's) in `dsound-capture`'s form: the start-up
    /// answers, a few buffers of the kinds the tests make, the effects and settings.
    fn table_text() -> String {
        let mut t = String::new();
        t += "DsCreate\t\t0\t\nDsCooperativeLevel\t2\t0\t\n";
        let mut caps = vec![0u8; 0x60];
        caps[0] = 0x60;
        caps[4..8].copy_from_slice(&0xf1fu32.to_le_bytes());
        t += &format!("DsCaps\t\t0\t{}\n", hex_string(&caps));
        t += "DsPrimary\t91\t0\t010002002256000044ac0000020008000000 12\n";
        let mut listener = vec![0u8; 0x40];
        listener[0] = 0x40;
        for o in [0x24usize, 0x2c, 0x34, 0x38, 0x3c] {
            listener[o..o + 4].copy_from_slice(&1.0f32.to_le_bytes());
        }
        t += &format!("DsListener\t\t0 0\t{}\n", hex_string(&listener));
        for (flags, alg, channels, size) in [
            (0x2e0u32, 0u32, 1u16, 3u32),
            (0x2e0, 0, 1, 6612),
            (0x2e0, 0, 1, 44100),
            (0xe0, 0, 1, 200),
            (0x302b0, 0, 1, 44100),
            (0x302b0, 0, 2, 88200),
            (0x302b0, 1, 1, 44100),
        ] {
            let kind: Kind = (flags, alg != 0, 1, channels, 22050, 16);
            let result = create_result(kind, size);
            let words = format!("{flags:x} {alg} 1 {channels:x} 5622 10 {size:x}");
            if (result as i32) < 0 {
                t += &format!("DsBuffer\t{words}\t{result:x}\t\t\t\t\n");
            } else {
                let mut caps = Vec::new();
                for w in [0x14, flags | 8, size, 0, 0] {
                    caps.extend_from_slice(&u32::to_le_bytes(w));
                }
                let three_d = if flags & 0x10 != 0 { 0 } else { E_NOINTERFACE };
                t += &format!(
                    "DsBuffer\t{words}\t{result:x}\t{}\t{} 12\t5622 0 0 0\t0 {three_d:x}\n",
                    hex_string(&caps),
                    hex_string(&format_bytes(1, channels, 22050, 16))
                );
            }
        }
        t += "DsFx\tdistortion 2e0 1 5622 ac44\t0\t2\t0\t00\n";
        t += "DsFx\tdistortion e0 1 5622 ac44\t8878001e\tffffffff\t8878001e\t00\n";
        t += "DsFx\ti3dl2 2e0 1 5622 ac44\t80040154\t5\t88781161\t00\n";
        t += "DsFx\ti3dl2 e0 1 5622 ac44\t8878001e\tffffffff\t8878001e\t00\n";
        t += "DsSet\tdistortion c2740000\t80070057\t00\nDsSet\tdistortion 00000000\t0\t00\n";
        t += "DsSet\t302b0 pan 0\t8878001e\t0\nDsSet\t2e0 pan 2711\t80070057\t2710\n";
        t += "DsEnd\n";
        t
    }

    fn engine() -> Engine {
        let mut e = Engine::new();
        load_table(&mut e, &table_text());
        e.mem.map(0x1000, 0x2000);
        e
    }

    /// `BSXenonAudio::Init`'s calls in its order; the device; answers 0x1000.. scratch.
    fn start_up(e: &mut Engine) -> u32 {
        assert_eq!(direct_sound_create8(e, 0, Ptr::new(0x1000), 0), DS_OK);
        e.mem.u32(0x1000)
    }

    /// A secondary buffer of 22050 Hz 16-bit PCM through `CreateSoundBuffer`.
    fn create(
        e: &mut Engine,
        ds: u32,
        flags: u32,
        hrtf: bool,
        channels: u16,
        size: u32,
    ) -> (u32, u32) {
        let format = 0x1600;
        e.mem.write(format, &format_bytes(1, channels, 22050, 16));
        let desc = 0x1700;
        for (i, w) in [0x24, flags, size, 0, format].into_iter().enumerate() {
            e.mem.set_u32(desc + 4 * i as u32, w);
        }
        let alg = if hrtf { ALG_HRTF_LIGHT } else { [0; 4] };
        for (i, w) in alg.into_iter().enumerate() {
            e.mem.set_u32(desc + 0x14 + 4 * i as u32, w);
        }
        e.mem.set_u32(0x1004, 0);
        let r = e.vcall(ds, 0xc, &[desc, 0x1004, 0]).u32();
        (r, e.mem.u32(0x1004))
    }

    #[test]
    fn the_audio_start_up_gets_a_device_a_primary_buffer_and_a_listener() {
        let mut e = engine();
        assert_eq!(co_initialize(&mut e, 0), DS_OK);
        assert_eq!(co_initialize(&mut e, 0), S_FALSE);
        let ds = start_up(&mut e);
        assert_eq!(e.vcall(ds, 0x18, &[0x55, 2]).u32(), DS_OK);
        assert_eq!(e.vcall(ds, 0x18, &[0x55, 9]).u32(), DSERR_INVALIDPARAM);
        e.mem.set_u32(0x1100, DSCAPS_SIZE);
        assert_eq!(e.vcall(ds, 0x10, &[0x1100]).u32(), DS_OK);
        assert_eq!(e.mem.u32(0x1104), 0xf1f, "the captured flags");
        e.mem.set_u32(0x1200, 0x24);
        e.mem.set_u32(0x1204, 0x91);
        assert_eq!(e.vcall(ds, 0xc, &[0x1200, 0x1004, 0]).u32(), DS_OK);
        let primary = e.mem.u32(0x1004);
        e.mem.write(0x1300, &IID_LISTENER);
        assert_eq!(e.vcall(primary, 0, &[0x1300, 0x1008]).u32(), DS_OK);
        let listener = e.mem.u32(0x1008);
        assert_ne!(listener, 0);
        assert_eq!(
            e.vcall(
                listener,
                0x38,
                &[1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 1]
            )
            .u32(),
            DS_OK
        );
        // Set with DS3D_DEFERRED: the position in effect changes at the commit (the DLL's
        // `GetPosition` reads the settings in effect, `100167f0`).
        assert_eq!(e.vcall(listener, 0x1c, &[0x1400]).u32(), DS_OK);
        assert_eq!([0, 4, 8].map(|o| e.mem.f32(0x1400 + o)), [0.0; 3]);
        assert_eq!(e.vcall(listener, 0x44, &[]).u32(), DS_OK);
        assert_eq!(e.vcall(listener, 0x1c, &[0x1400]).u32(), DS_OK);
        assert_eq!([0, 4, 8].map(|o| e.mem.f32(0x1400 + o)), [1.0, 2.0, 3.0]);
        e.mem.set_u32(0x1500, LISTENER_SIZE);
        assert_eq!(e.vcall(listener, 0xc, &[0x1500]).u32(), DS_OK);
        assert_eq!(e.mem.f32(0x1500 + 0x34), 1.0, "distance factor");
        assert_eq!(e.mem.f32(0x1500 + 0x28 + 4), 1.0, "top is +y");
        e.mem.write(0x1300, &[9u8; 16]);
        assert_eq!(e.vcall(primary, 0, &[0x1300, 0x100c]).u32(), E_NOINTERFACE);
        assert_eq!(e.vcall(listener, 8, &[]).u32(), 0);
        assert_eq!(e.vcall(primary, 8, &[]).u32(), 0);
        assert_eq!(e.vcall(ds, 8, &[]).u32(), 0);
    }

    #[test]
    fn buffers_are_refused_and_made_as_measured() {
        let mut e = engine();
        let ds = start_up(&mut e);
        assert_eq!(create(&mut e, ds, 0x2e0, false, 1, 3).0, DSERR_INVALIDPARAM);
        assert_eq!(
            create(&mut e, ds, 0x2e0, false, 1, 6612).0,
            DSERR_BUFFERTOOSMALL
        );
        assert_eq!(
            create(&mut e, ds, 0x302b0, false, 2, 88200).0,
            DSERR_INVALIDPARAM
        );
        let (r, hrtf) = create(&mut e, ds, 0x302b0, true, 1, 44100);
        assert_eq!(r, DS_NO_VIRTUALIZATION);
        assert_ne!(hrtf, 0);
        let (r, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        assert_eq!(r, DS_OK);
        e.mem.set_u32(0x1800, 0x14);
        assert_eq!(e.vcall(b, 0xc, &[0x1800]).u32(), DS_OK);
        assert_eq!([4, 8].map(|o| e.mem.u32(0x1800 + o)), [0x2e8, 44100]);
        assert_eq!(e.vcall(b, 0x14, &[0x1900, 0x12, 0x1920]).u32(), DS_OK);
        assert_eq!(e.mem.bytes(0x1900, 0x12), format_bytes(1, 1, 22050, 16));
        assert_eq!(e.mem.u32(0x1920), 0x12);
        assert_eq!(e.vcall(b, 0x20, &[0x1924]).u32(), DS_OK);
        assert_eq!(e.mem.u32(0x1924), 22050);
        // IDirectSoundBuffer8 yes, IDirectSound3DBuffer no (2D).
        e.mem.write(0x1300, &IID_BUFFER8);
        assert_eq!(e.vcall(b, 0, &[0x1300, 0x1928]).u32(), DS_OK);
        e.mem.write(0x1300, &IID_BUFFER3D);
        assert_eq!(e.vcall(b, 0, &[0x1300, 0x192c]).u32(), E_NOINTERFACE);
        assert_eq!(e.vcall(b, 8, &[]).u32(), 1);
        assert_eq!(e.vcall(b, 8, &[]).u32(), 0);
    }

    #[test]
    #[should_panic(expected = "not measured")]
    fn a_buffer_description_not_measured_stops() {
        let mut e = engine();
        let ds = start_up(&mut e);
        create(&mut e, ds, 0x2a0, false, 1, 44100);
    }

    /// Measured (`refmeasure sweep`): any rate, channel count and width of the measured flag
    /// sets is made by the rule, the capture or not; a size that is not whole blocks is
    /// rounded up; muting at the maximum distance without 3D is refused. The loading menu's
    /// sound asked for 212275 bytes of 32000 Hz 16-bit mono 3D (`00aec530`).
    #[test]
    fn buffers_follow_the_measured_rule_without_a_capture_line() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let make = |e: &mut Engine, flags: u32, channels: u16, rate: u32, bits: u16, size: u32| {
            e.mem.write(0x1600, &format_bytes(1, channels, rate, bits));
            for (i, w) in [0x24, flags, size, 0, 0x1600, 0, 0, 0, 0]
                .into_iter()
                .enumerate()
            {
                e.mem.set_u32(0x1700 + 4 * i as u32, w);
            }
            e.mem.set_u32(0x1004, 0);
            let r = e.vcall(ds, 0xc, &[0x1700, 0x1004, 0]).u32();
            (r, e.mem.u32(0x1004))
        };
        let (r, b) = make(&mut e, 0x302b0, 1, 32000, 16, 212_275);
        assert_eq!(r, DS_OK);
        e.mem.set_u32(0x1800, 0x14);
        assert_eq!(e.vcall(b, 0xc, &[0x1800]).u32(), DS_OK);
        assert_eq!([4, 8].map(|o| e.mem.u32(0x1800 + o)), [0x302b8, 212_276]);
        let (r, b) = make(&mut e, 0xe0, 2, 96000, 8, 1001);
        assert_eq!(r, DS_OK);
        assert_eq!(e.vcall(b, 0xc, &[0x1800]).u32(), DS_OK);
        assert_eq!(e.mem.u32(0x1808), 1002);
        assert_eq!(
            make(&mut e, 0x2e0, 2, 48000, 16, 28796).0,
            DSERR_BUFFERTOOSMALL
        );
        assert_eq!(make(&mut e, 0x2e0, 2, 48000, 16, 28800).0, DS_OK);
        assert_eq!(
            make(&mut e, 0x300b0, 2, 44100, 16, 4000).0,
            DSERR_INVALIDPARAM
        );
        assert_eq!(
            make(&mut e, 0x200e0, 1, 22050, 16, 4000).0,
            DSERR_INVALIDPARAM
        );
    }

    #[test]
    fn a_3d_buffer_keeps_its_settings() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x302b0, false, 1, 44100);
        e.mem.write(0x1300, &IID_BUFFER3D);
        assert_eq!(e.vcall(b, 0, &[0x1300, 0x1928]).u32(), DS_OK);
        let three_d = e.mem.u32(0x1928);
        assert_eq!(
            e.vcall(three_d, 0x44, &[1.0e9f32.to_bits(), 1]).u32(),
            DS_OK
        );
        assert_eq!(e.vcall(three_d, 0x44, &[0, 1]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(
            e.vcall(three_d, 0x40, &[(-1.0f32).to_bits(), 1]).u32(),
            DSERR_INVALIDPARAM
        );
        let p = (-3.4e37f32).to_bits();
        assert_eq!(e.vcall(three_d, 0x4c, &[p, p, p, 1]).u32(), DS_OK);
        // Pan: in range but no DSBCAPS_CTRLPAN.
        assert_eq!(e.vcall(b, 0x40, &[0]).u32(), DSERR_CONTROLUNAVAIL);
        assert_eq!(e.vcall(b, 0x40, &[10001]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(e.vcall(b, 0x3c, &[(-10000i32) as u32]).u32(), DS_OK);
        // The 3D interface counts its own references (measured).
        assert_eq!(e.vcall(three_d, 8, &[]).u32(), 0);
        assert_eq!(e.vcall(b, 8, &[]).u32(), 0);
        assert!(e.kernel.sound.buffers.is_empty());
    }

    /// Measured (`refmeasure`, 2026-10-27): the buffer and its 3D interface count apart; the
    /// game's order (the buffer released three times, then the 3D interface) answers 1, 0, 0,
    /// then `0xffffffff`.
    #[test]
    fn a_3d_buffer_and_its_3d_interface_count_their_references_apart() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let three_d_of = |e: &mut Engine, b: u32| {
            e.mem.write(0x1300, &IID_BUFFER3D);
            assert_eq!(e.vcall(b, 0, &[0x1300, 0x1928]).u32(), DS_OK);
            e.mem.u32(0x1928)
        };
        let buffer8_of = |e: &mut Engine, b: u32| {
            e.mem.write(0x1300, &IID_BUFFER8);
            assert_eq!(e.vcall(b, 0, &[0x1300, 0x192c]).u32(), DS_OK);
        };
        // The game's order.
        let (_, b) = create(&mut e, ds, 0x302b0, false, 1, 44100);
        buffer8_of(&mut e, b);
        let t = three_d_of(&mut e, b);
        let answers: Vec<u32> = (0..3).map(|_| e.vcall(b, 8, &[]).u32()).collect();
        assert_eq!(answers, [1, 0, 0]);
        assert_eq!(e.vcall(t, 8, &[]).u32(), 0xffff_ffff);
        // AddRef on each, then the 3D interface first.
        let (_, b) = create(&mut e, ds, 0x302b0, false, 1, 44100);
        buffer8_of(&mut e, b);
        let t = three_d_of(&mut e, b);
        assert_eq!(e.vcall(t, 4, &[]).u32(), 2);
        assert_eq!(e.vcall(b, 4, &[]).u32(), 3);
        let answers: Vec<u32> = [t, t, b, b, b]
            .iter()
            .map(|&o| e.vcall(o, 8, &[]).u32())
            .collect();
        assert_eq!(answers, [1, 0, 2, 1, 0]);
        assert!(e.kernel.sound.buffers.is_empty());
        // The buffer first: it stays until the 3D interface goes.
        let (_, b) = create(&mut e, ds, 0x302b0, false, 1, 44100);
        buffer8_of(&mut e, b);
        let t = three_d_of(&mut e, b);
        assert_eq!(e.vcall(b, 8, &[]).u32(), 1);
        assert_eq!(e.vcall(b, 8, &[]).u32(), 0);
        assert!(e.kernel.sound.buffers.contains_key(&b));
        assert_eq!(e.vcall(t, 8, &[]).u32(), 0);
        assert!(e.kernel.sound.buffers.is_empty());
    }

    #[test]
    fn lock_gives_the_buffer_s_memory() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        assert_eq!(
            e.vcall(b, 0x2c, &[0, 44100, 0x1a00, 0x1a04, 0, 0, 0]).u32(),
            DS_OK
        );
        let p = e.mem.u32(0x1a00);
        assert_eq!(e.mem.u32(0x1a04), 44100);
        e.mem.write(p, &[7; 16]);
        assert_eq!(e.vcall(b, 0x4c, &[p, 44100, 0, 0]).u32(), DS_OK);
        assert_eq!(
            e.vcall(b, 0x2c, &[0, 0, 0x1a00, 0x1a04, 0x1a08, 0x1a0c, 2])
                .u32(),
            DS_OK
        );
        assert_eq!(
            [e.mem.u32(0x1a00), e.mem.u32(0x1a04), e.mem.u32(0x1a08)],
            [p, 44100, 0]
        );
        assert_eq!(e.mem.bytes(p, 2), [7, 7]);
        // Wrapping round.
        assert_eq!(
            e.vcall(b, 0x2c, &[44000, 300, 0x1a10, 0x1a14, 0x1a18, 0x1a1c, 0])
                .u32(),
            DS_OK
        );
        assert_eq!(
            [e.mem.u32(0x1a14), e.mem.u32(0x1a18), e.mem.u32(0x1a1c)],
            [100, p, 200]
        );
    }

    #[test]
    fn volume_pan_and_frequency_keep_their_ranges() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        assert_eq!(
            e.vcall(b, 0x3c, &[(-10001i32) as u32]).u32(),
            DSERR_INVALIDPARAM
        );
        assert_eq!(e.vcall(b, 0x3c, &[1]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(e.vcall(b, 0x3c, &[(-2500i32) as u32]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x18, &[0x1a00]).u32(), DS_OK);
        assert_eq!(e.mem.i32(0x1a00), -2500);
        assert_eq!(e.vcall(b, 0x40, &[10000]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x44, &[99]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(e.vcall(b, 0x44, &[200_001]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(e.vcall(b, 0x44, &[11025]).u32(), DS_OK);
        e.vcall(b, 0x20, &[0x1a04]);
        assert_eq!(e.mem.u32(0x1a04), 11025);
        assert_eq!(e.vcall(b, 0x44, &[0]).u32(), DS_OK);
        e.vcall(b, 0x20, &[0x1a04]);
        assert_eq!(e.mem.u32(0x1a04), 22050, "0 is the format's rate");
    }

    /// The game's `SetFX` (`00aec530`): one `DSEFFECTDESC` of 0x20 bytes.
    fn effect_desc(e: &mut Engine, class: &[u8; 16]) -> u32 {
        let d = 0x1b00;
        e.mem.set_u32(d, 0x20);
        e.mem.set_u32(d + 4, 0);
        e.mem.write(d + 8, class);
        e.mem.set_u32(d + 0x18, 0);
        e.mem.set_u32(d + 0x1c, 0);
        d
    }

    #[test]
    fn the_distortion_works_and_the_reverb_is_missing_as_captured() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        let d = effect_desc(&mut e, &GUID_DISTORTION);
        assert_eq!(e.vcall(b, 0x54, &[1, d, 0x1c00]).u32(), DS_OK);
        assert_eq!(e.mem.u32(0x1c00), 2, "DSFXR_LOCSOFTWARE");
        e.mem.write(0x1c10, &GUID_DISTORTION);
        e.mem.write(0x1c20, &IID_DISTORTION8);
        assert_eq!(e.vcall(b, 0x5c, &[0x1c10, 0, 0x1c20, 0x1c30]).u32(), DS_OK);
        let fx = e.mem.u32(0x1c30);
        for (i, v) in [-18.0f32, 0.0, 5000.0, 1135.0, 5000.0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(0x1c40 + 4 * i as u32, v);
        }
        assert_eq!(e.vcall(fx, 0xc, &[0x1c40]).u32(), DS_OK);
        e.mem.set_f32(0x1c40, -61.0);
        assert_eq!(e.vcall(fx, 0xc, &[0x1c40]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(e.vcall(fx, 8, &[]).u32(), 0);
        // The reverb, on a second buffer.
        let (_, b2) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        let d = effect_desc(&mut e, &GUID_I3DL2);
        assert_eq!(e.vcall(b2, 0x54, &[1, d, 0]).u32(), 0x8004_0154);
        e.mem.write(0x1c10, &GUID_I3DL2);
        e.mem.write(0x1c20, &IID_I3DL2_8);
        assert_eq!(
            e.vcall(b2, 0x5c, &[0x1c10, 0, 0x1c20, 0x1c30]).u32(),
            0x8878_1161
        );
        // Without DSBCAPS_CTRLFX.
        let (_, b3) = create(&mut e, ds, 0xe0, false, 1, 200);
        let d = effect_desc(&mut e, &GUID_DISTORTION);
        e.mem.set_u32(0x1c00, 0xffff_ffff);
        assert_eq!(
            e.vcall(b3, 0x54, &[1, d, 0x1c00]).u32(),
            DSERR_CONTROLUNAVAIL
        );
        assert_eq!(e.mem.u32(0x1c00), 0xffff_ffff);
    }

    fn position(e: &mut Engine, b: u32) -> (u32, u32, u32) {
        e.vcall(b, 0x10, &[0x1d00, 0x1d04]);
        e.vcall(b, 0x24, &[0x1d08]);
        (e.mem.u32(0x1d00), e.mem.u32(0x1d04), e.mem.u32(0x1d08))
    }

    /// Turns on DirectSound's second converter for the device, as a large frequency change on any
    /// buffer does.
    fn second_converter(e: &mut Engine, ds: u32) {
        let (_, x) = create(e, ds, 0x2e0, false, 1, 44100);
        assert_eq!(e.vcall(x, 0x44, &[30000]).u32(), DS_OK);
        assert!(e.kernel.sound.apl);
    }

    /// Measured: the first `Play` of a 22050 Hz mono 16-bit looping buffer on the platform clock's
    /// 10 ms periods, with DirectSound's first converter (`setpos-run1.tsv` `loop_half_play`: the
    /// frames played rounded up and 33 more) and with its second (`freqphase-run1.tsv` `5622 1 10
    /// ac44 steady 0 0 0 apl`: the frames its converter takes, here rounded up and one more from
    /// the second period).
    #[test]
    fn the_cursors_move_by_10_ms_periods_of_the_platform_clock() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.clock_ms = 5;
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 1]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (0, 0, 5));
        let dmo = [
            (0, 254),
            (0, 474),
            (34, 695),
            (254, 915),
            (475, 1136),
            (695, 1356),
        ];
        for (i, (p, w)) in dmo.into_iter().enumerate() {
            e.clock_ms = 10 * (i as u32 + 1);
            assert_eq!(position(&mut e, b), (2 * p, 2 * w, 5), "period {}", i + 1);
        }
        // Past the end of the buffer: round again (101 periods: 33 + 22271 frames).
        e.clock_ms = 1010;
        assert_eq!(position(&mut e, b).1, 2 * (33 + 22271 - 22050));
        assert_eq!(e.vcall(b, 0x48, &[]).u32(), DS_OK);
        assert_eq!(
            position(&mut e, b),
            (2 * 254, 2 * 254, 0),
            "both at the write cursor"
        );
        second_converter(&mut e, ds);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.clock_ms = 2000;
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 1]).u32(), DS_OK);
        let lanes = [
            (0, 221),
            (0, 442),
            (2, 663),
            (222, 883),
            (443, 1104),
            (663, 1324),
        ];
        for (i, (p, w)) in lanes.into_iter().enumerate() {
            e.clock_ms = 2000 + 10 * (i as u32 + 1);
            assert_eq!(position(&mut e, b), (2 * p, 2 * w, 5), "period {}", i + 1);
        }
    }

    /// Measured (`dsound-capture replays`): a later Play of a buffer puts the write cursor at the
    /// frames played rounded up, its first Play one frame past them.
    #[test]
    fn a_buffer_played_again_leads_by_the_frames_rounded_up() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        e.clock_ms = 20; // two periods: 441 frames played, 33 ahead (the first converter)
        assert_eq!(position(&mut e, b).1, 2 * 474, "first Play");
        assert_eq!(e.vcall(b, 0x48, &[]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x34, &[0]).u32(), DS_OK);
        e.clock_ms = 100;
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        e.clock_ms = 120;
        assert_eq!(position(&mut e, b).1, 2 * 441, "a later Play");
        e.clock_ms = 130; // 661.5 frames: rounded up either way
        assert_eq!(position(&mut e, b).1, 2 * 662);
    }

    #[test]
    fn a_buffer_played_once_stops_at_its_end() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.clock_ms = 990;
        assert_eq!(position(&mut e, b).2, 1);
        e.clock_ms = 1000; // 100 periods: 22050 frames, the whole buffer
        assert_eq!(position(&mut e, b), (0, 0, 0));
        // Measured (`dsound-capture replays`): Stop keeps both cursors at 0, and Play plays the
        // buffer again from its start.
        assert_eq!(e.vcall(b, 0x48, &[]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (0, 0, 0), "at 0 after Stop");
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        assert_eq!(position(&mut e, b).2, 1, "playing again");
        e.clock_ms = 1500;
        let (play, write, status) = position(&mut e, b);
        assert!(
            status == 1 && play < write && write < 44100,
            "{play} {write} {status}"
        );
        e.clock_ms = 2010;
        assert_eq!(position(&mut e, b), (0, 0, 0), "ended again");
        // Straight after the end, without a Stop: from the start too.
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        assert_eq!(position(&mut e, b).2, 1);
        assert_eq!(e.vcall(b, 0x48, &[]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x34, &[0]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (0, 0, 0));
    }

    /// A `0x2e0` buffer of `size` bytes, `channels`, `rate` and `bits`.
    fn create_format(e: &mut Engine, ds: u32, format: (u16, u32, u16), size: u32) -> u32 {
        let (channels, rate, bits) = format;
        e.mem.write(0x1600, &format_bytes(1, channels, rate, bits));
        for (i, w) in [0x24, 0x2e0, size, 0, 0x1600, 0, 0, 0, 0]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(0x1700 + 4 * i as u32, w);
        }
        e.mem.set_u32(0x1004, 0);
        assert_eq!(e.vcall(ds, 0xc, &[0x1700, 0x1004, 0]).u32(), DS_OK);
        e.mem.u32(0x1004)
    }

    /// Checks the play and write cursors and the status at each clock reading (ms).
    fn expect_cursors(e: &mut Engine, b: u32, expect: &[(u32, (u32, u32, u32))]) {
        for &(ms, want) in expect {
            e.clock_ms = ms;
            assert_eq!(position(e, b), want, "at {ms} ms");
        }
    }

    /// Measured (`dsound-capture setpos`, 2026-10-31, `setpos-run1.tsv` `loop_half_play`): the
    /// game's call (`00aed080`: `SetCurrentPosition` then `Play(DSBPLAY_LOOPING)`) on a buffer
    /// playing for 10 periods. Both cursors at the new place at once, then the write cursor on
    /// from it by 221, 441, 662 ... frames (the steps the buffer was taking), the play cursor
    /// held there until the write cursor is three periods past.
    #[test]
    fn a_playing_buffer_moved_goes_on_from_there() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        assert_eq!(e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]).u32(), DS_OK);
        e.clock_ms = 100;
        assert_eq!(e.vcall(b, 0x34, &[0x5622]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (0x5622, 0x5622, 5));
        assert_eq!(e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (0x5622, 0x5622, 5));
        expect_cursors(
            &mut e,
            b,
            &[
                (110, (0x5622, 0x57dc, 5)),
                (120, (0x5622, 0x5994, 5)),
                (130, (0x5624, 0x5b4e, 5)),
                (140, (0x57dc, 0x5d06, 5)),
                (150, (0x5996, 0x5ec0, 5)),
            ],
        );
    }

    /// Measured (`setpos-run2.tsv` `loop_half 106600`, `setpos-run3.tsv` 11025 Hz): moved after
    /// an odd number of periods, the 22050 Hz buffer's steps go on 220, 221, 220 ...; an 11025 Hz
    /// 8-bit buffer's after 10 and after 11 periods.
    #[test]
    fn a_moved_buffer_keeps_the_steps_it_was_taking() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        e.clock_ms = 110;
        assert_eq!(e.vcall(b, 0x34, &[0x5622]).u32(), DS_OK);
        expect_cursors(
            &mut e,
            b,
            &[
                (110, (0x5622, 0x5622, 5)),
                (120, (0x5622, 0x57da, 5)),
                (130, (0x5622, 0x5994, 5)),
                (140, (0x5622, 0x5b4c, 5)),
                (150, (0x57dc, 0x5d06, 5)),
                (160, (0x5994, 0x5ebe, 5)),
            ],
        );
        for (played, expect) in [
            (
                100,
                [
                    (0x1588, 0x15f6),
                    (0x1588, 0x1664),
                    (0x1589, 0x16d3),
                    (0x15f7, 0x1741),
                    (0x1665, 0x17af),
                    (0x16d3, 0x181d),
                ],
            ),
            (
                110,
                [
                    (0x1588, 0x15f6),
                    (0x1588, 0x1665),
                    (0x1589, 0x16d3),
                    (0x15f7, 0x1741),
                    (0x1665, 0x17af),
                    (0x16d4, 0x181e),
                ],
            ),
        ] {
            let b = create_format(&mut e, ds, (1, 11025, 8), 11025);
            e.clock_ms = 1000;
            e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
            e.clock_ms = 1000 + played;
            assert_eq!(e.vcall(b, 0x34, &[0x1588]).u32(), DS_OK);
            for (i, (p, w)) in expect.into_iter().enumerate() {
                e.clock_ms = 1000 + played + 10 * (i as u32 + 1);
                assert_eq!(position(&mut e, b), (p, w, 5), "{played}: period {}", i + 1);
            }
            e.vcall(b, 0x48, &[]);
        }
    }

    /// Measured (`setpos-run1.tsv` `loop_near_end`, `once_late`, `loop_odd`, `loop_end`,
    /// `loop_past`, `loop_half_play_once`): a looping buffer moved to its last frame goes round;
    /// one that does not loop ends when the write cursor reaches its end; an odd byte is taken
    /// down to its frame; the end and past it are refused and change nothing; `Play` without the
    /// looping flag after the move takes the flag off.
    #[test]
    fn a_moved_buffer_goes_round_or_ends() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        e.clock_ms = 100;
        let before = position(&mut e, b);
        for refused in [0xac44, 0xac46] {
            assert_eq!(e.vcall(b, 0x34, &[refused]).u32(), DSERR_INVALIDPARAM);
            assert_eq!(position(&mut e, b), before);
        }
        assert_eq!(e.vcall(b, 0x34, &[0xac43]).u32(), DS_OK);
        expect_cursors(
            &mut e,
            b,
            &[
                (100, (0xac42, 0xac42, 5)),
                (110, (0xac42, 0x1b8, 5)),
                (120, (0xac42, 0x370, 5)),
                (130, (0, 0x52a, 5)),
                (140, (0x1b8, 0x6e2, 5)),
            ],
        );
        // Not looping: from frame 16537 it ends at the 25th period.
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.clock_ms = 1000;
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.clock_ms = 1100;
        assert_eq!(e.vcall(b, 0x34, &[0x8132]).u32(), DS_OK);
        expect_cursors(
            &mut e,
            b,
            &[
                (1100, (0x8132, 0x8132, 1)),
                (1110, (0x8132, 0x82ec, 1)),
                (1130, (0x8134, 0x865e, 1)),
                (1340, (0xa560, 0xaa8a, 1)),
                (1350, (0, 0, 0)),
            ],
        );
        // Looping, moved, then played without the flag: it ends.
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.clock_ms = 2000;
        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        e.clock_ms = 2100;
        assert_eq!(e.vcall(b, 0x34, &[0x5623]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        expect_cursors(
            &mut e,
            b,
            &[
                (2100, (0x5622, 0x5622, 1)),
                (2110, (0x5622, 0x57dc, 1)),
                (2550, (0x9e7e, 0xa3a8, 1)),
                (2600, (0, 0, 0)),
            ],
        );
    }

    /// Measured (`setpos-run1.tsv` `ended_half`, `ended_end`): a buffer that played to its end,
    /// moved, stays stopped there and plays from there as a later `Play`; moved to its end it is
    /// refused and plays again from its start.
    #[test]
    fn a_buffer_that_ended_moved_plays_from_there() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 8820);
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.clock_ms = 300;
        assert_eq!(position(&mut e, b), (0, 0, 0), "ended");
        assert_eq!(e.vcall(b, 0x34, &[0x113a]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (0x113a, 0x113a, 0));
        e.clock_ms = 350;
        assert_eq!(position(&mut e, b), (0x113a, 0x113a, 0));
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        expect_cursors(
            &mut e,
            b,
            &[
                (360, (0x113a, 0x12f4, 1)),
                (370, (0x113a, 0x14ac, 1)),
                (380, (0x113c, 0x1666, 1)),
            ],
        );
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 8820);
        e.clock_ms = 1000;
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.clock_ms = 1300;
        assert_eq!(e.vcall(b, 0x34, &[0x2274]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(position(&mut e, b), (0, 0, 0));
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.clock_ms = 1310;
        assert_eq!(position(&mut e, b), (0, 0x1ba, 1), "from the start");
    }

    /// Measured (`setpos-run4.tsv` `fresh_end`, `stopped_end`, and its first `loop_half`): the end
    /// of a stopped buffer is refused too; a first `Play` moved before its first period still has
    /// its lead to come (33 frames with the first converter: `0x5622 + 0x1fc` a period later).
    #[test]
    fn the_end_is_refused_and_a_first_lead_kept() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 8820);
        for refused in [0x2274, 0x2276] {
            assert_eq!(e.vcall(b, 0x34, &[refused]).u32(), DSERR_INVALIDPARAM);
            assert_eq!(position(&mut e, b), (0, 0, 0));
        }
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.clock_ms = 50;
        e.vcall(b, 0x48, &[]);
        let stopped = position(&mut e, b);
        assert_eq!(e.vcall(b, 0x34, &[0x2274]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(position(&mut e, b), stopped);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.clock_ms = 1001;
        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        e.clock_ms = 1005;
        assert_eq!(e.vcall(b, 0x34, &[0x5622]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (0x5622, 0x5622, 5));
        e.clock_ms = 1010;
        assert_eq!(position(&mut e, b), (0x5622, 0x5622 + 2 * (33 + 221), 5));
    }

    /// Measured (`setpos-run4.tsv` `2b11 1 8 89d 102e0 stopped_half`): an 11025 Hz 8-bit buffer
    /// stopped after 6 periods, moved and played again goes on in the steps it was taking (110,
    /// 220, 331 ... frames, not 111, 221, 331).
    #[test]
    fn a_stopped_buffer_plays_on_in_its_steps() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let b = create_format(&mut e, ds, (1, 11025, 8), 0x89d);
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.clock_ms = 60;
        assert_eq!(e.vcall(b, 0x48, &[]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x34, &[0x89d]).u32(), DSERR_INVALIDPARAM);
        assert_eq!(e.vcall(b, 0x34, &[0x44e]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (0x44e, 0x44e, 0));
        e.clock_ms = 100;
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        expect_cursors(
            &mut e,
            b,
            &[
                (100, (0x44e, 0x44e, 1)),
                (110, (0x44e, 0x4bc, 1)),
                (120, (0x44e, 0x52a, 1)),
                (130, (0x44f, 0x599, 1)),
                (140, (0x4bd, 0x607, 1)),
                (150, (0x52b, 0x675, 1)),
                (160, (0x599, 0x6e3, 1)),
                (170, (0x608, 0x752, 1)),
                (180, (0x676, 0x7c0, 1)),
            ],
        );
    }

    /// Measured (`endphase-run1.tsv` `2b11 1 8 8d4 ended` and `again`): a buffer played to its end
    /// plays again in the steps of the periods it had played, to the period its write cursor
    /// reached the end (21, then 42).
    #[test]
    fn a_buffer_that_ended_plays_again_in_its_steps() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let b = create_format(&mut e, ds, (1, 11025, 8), 2260);
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.clock_ms = 210;
        assert_eq!(position(&mut e, b), (0, 0, 0), "ended in its 21st period");
        e.clock_ms = 300;
        e.vcall(b, 0x30, &[0, 0, 0]);
        let steps = [
            (0, 0x6e),
            (0, 0xdc),
            (0, 0x14a),
            (0x6f, 0x1b9),
            (0xdd, 0x227),
            (0x14b, 0x295),
        ];
        for (i, (p, w)) in steps.into_iter().enumerate() {
            e.clock_ms = 310 + 10 * i as u32;
            assert_eq!(position(&mut e, b), (p, w, 1), "period {}", i + 1);
        }
        e.clock_ms = 520;
        assert_eq!(position(&mut e, b), (0, 0, 0), "ended again, 21 periods on");
        e.clock_ms = 600;
        e.vcall(b, 0x30, &[0, 0, 0]);
        let steps = [
            (0, 0x6e),
            (0, 0xdc),
            (1, 0x14b),
            (0x6f, 0x1b9),
            (0xdd, 0x227),
            (0x14b, 0x295),
        ];
        for (i, (p, w)) in steps.into_iter().enumerate() {
            e.clock_ms = 610 + 10 * i as u32;
            assert_eq!(position(&mut e, b), (p, w, 1), "again: period {}", i + 1);
        }
    }

    /// Measured (`freqphase-run1..3.tsv` `2b11 1 8 2b11 retune_stop 11040 5 3 apl`, the same in
    /// all three runs): an 11025 Hz 8-bit buffer retuned to 11040 Hz after 5 periods, stopped 3
    /// periods later and played again: the position's hundredths go on through the change and
    /// the stop, and the play cursor goes on trailing after the change.
    #[test]
    fn a_retuned_buffer_keeps_its_position_hundredths() {
        let mut e = engine();
        let ds = start_up(&mut e);
        second_converter(&mut e, ds);
        let b = create_format(&mut e, ds, (1, 11025, 8), 0x2b11);
        e.vcall(b, 0x30, &[0, 0, 0]);
        let first = [(0, 111), (0, 221), (1, 331), (112, 442), (222, 552)];
        for (i, (p, w)) in first.into_iter().enumerate() {
            e.clock_ms = 10 * (i as u32 + 1);
            assert_eq!(position(&mut e, b), (p, w, 1), "period {}", i + 1);
        }
        assert_eq!(e.vcall(b, 0x44, &[11040]).u32(), DS_OK);
        // Measured 1 us after the call (`43569:de:228:1`, all 72 retunes): unchanged.
        assert_eq!(position(&mut e, b), (222, 552, 1), "right after the call");
        expect_cursors(
            &mut e,
            b,
            &[
                (60, (332, 662, 1)),
                (70, (443, 773, 1)),
                (80, (553, 883, 1)),
            ],
        );
        assert_eq!(e.vcall(b, 0x48, &[]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (883, 883, 0));
        e.clock_ms = 100;
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        expect_cursors(
            &mut e,
            b,
            &[
                (110, (883, 993, 1)),
                (120, (883, 1104, 1)),
                (130, (884, 1214, 1)),
                (140, (995, 1325, 1)),
                (150, (1105, 1435, 1)),
                (160, (1215, 1545, 1)),
                (170, (1326, 1656, 1)),
            ],
        );
    }

    /// A move right after `SetFrequency`, in the same period, of a first `Play` past its first
    /// period goes on as a later `Play` (the same as without the `SetFrequency`, measured with
    /// `setpos-run1.tsv` `loop_half_play`: 5994 at 120 ms, then 57dc:5d06 at 140 ms).
    #[test]
    fn a_move_right_after_set_frequency_takes_the_measured_steps() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        e.clock_ms = 100;
        assert_eq!(e.vcall(b, 0x44, &[22050]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x34, &[0x5622]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]).u32(), DS_OK);
        expect_cursors(
            &mut e,
            b,
            &[
                (110, (0x5622, 0x57dc, 5)),
                (120, (0x5622, 0x5994, 5)),
                (140, (0x57dc, 0x5d06, 5)),
            ],
        );
    }

    /// One replay program (`tools/programs.pl` in `build43-scratch/measure/dss1-work`, from the
    /// measured `dsound-capture` lines: `id<TAB>rate channels bits bytes<TAB>frequency set before
    /// the first Play (0: none)<TAB>the first Play's flags (`none`: not played)<TAB>tokens`, the
    /// tokens the measured reads `S:time:play:write:status` and the calls `C:stop`, `C:play:F`,
    /// `C:f:HZ`, `C:scp:BYTE`, `C:wait:PERIODS`, `C:waitend`) replayed through the layer: each
    /// call after as many periods as the measured write cursor changed before it (or ended), the
    /// cursors read at every period, as `segment:period:play:write:status` (bytes, hex).
    fn replay_program(line: &str) -> String {
        let f: Vec<&str> = line.split('\t').collect();
        let fmt: Vec<u32> = f[1].split(' ').map(|x| x.parse().unwrap()).collect();
        let (rate, channels, bits, bytes) = (fmt[0], fmt[1] as u16, fmt[2] as u16, fmt[3]);
        let init: u32 = f[2].parse().unwrap();
        let first = f[3];
        let hex = |s: &str| u32::from_str_radix(s, 16).unwrap();
        let mut periods = 0u32;
        let mut last: Option<(u32, Option<bool>)> = None;
        let mut calls: Vec<(u32, String)> = Vec::new();
        for t in f[4].split(' ') {
            if let Some(c) = t.strip_prefix("C:") {
                calls.push((periods, c.to_string()));
                last = last.map(|(w, _)| (w, None));
                continue;
            }
            let v: Vec<u32> = t[2..].split(':').skip(1).map(hex).collect();
            let playing = v[2] & 1 != 0;
            if let Some((w, Some(true))) = last {
                if v[1] != w || !playing {
                    periods += 1;
                }
            }
            last = Some((v[1], Some(playing)));
        }
        let mut e = engine();
        let ds = start_up(&mut e);
        // Measured with the second converter on: the `apl` and `big` parts.
        e.kernel.sound.apl = f[0].ends_with(" apl") || f[0].ends_with(" big");
        let b = create_format(&mut e, ds, (channels, rate, bits), bytes);
        let mut out = Vec::new();
        let (mut at, mut off, mut seg, mut seg_at) = (0u32, 0u32, 0u32, 0u32);
        let read = |e: &mut Engine, out: &mut Vec<String>, seg: u32, j: u32| {
            let (p, w, s) = position(e, b);
            out.push(format!("{seg}:{j}:{p:x}:{w:x}:{s:x}"));
        };
        e.clock_ms = 0;
        if init != 0 {
            e.vcall(b, 0x44, &[init]);
        }
        if first != "none" {
            e.vcall(b, 0x30, &[0, 0, first.parse().unwrap()]);
        }
        read(&mut e, &mut out, 0, 0);
        let n = calls.len();
        for (k, (p, c)) in calls.into_iter().enumerate() {
            while at < p {
                at += 1;
                e.clock_ms = 10 * (at + off);
                read(&mut e, &mut out, seg, at - seg_at);
            }
            let parts: Vec<&str> = c.split(':').collect();
            match parts[0] {
                "stop" => {
                    e.vcall(b, 0x48, &[]);
                }
                "play" => {
                    e.vcall(b, 0x30, &[0, 0, parts[1].parse().unwrap()]);
                }
                "f" => {
                    e.vcall(b, 0x44, &[parts[1].parse().unwrap()]);
                }
                "scp" => {
                    let r = e.vcall(b, 0x34, &[hex(parts[1])]).u32();
                    out.push(format!("scp={r:x}"));
                }
                "wait" => {
                    off += parts[1].parse::<u32>().unwrap();
                    e.clock_ms = 10 * (at + off);
                }
                "waitend" => {
                    for _ in 0..200 {
                        off += 1;
                        e.clock_ms = 10 * (at + off);
                        if position(&mut e, b).2 & 1 == 0 {
                            break;
                        }
                    }
                    off += 2;
                    e.clock_ms = 10 * (at + off);
                }
                _ => {}
            }
            seg += 1;
            seg_at = at;
            read(&mut e, &mut out, seg, 0);
            if k + 1 == n {
                for j in 1..=45 {
                    e.clock_ms = 10 * (at + off + j);
                    read(&mut e, &mut out, seg, j);
                }
            }
        }
        if n == 0 {
            for j in 1..=45 {
                e.clock_ms = 10 * j;
                read(&mut e, &mut out, 0, j);
            }
        }
        out.join(" ")
    }

    /// Every replay program in the file `NV_REPLAY_PROGRAMS` (`tools/programs.pl`) through the
    /// layer, each line `id<TAB>reads` (or `PANIC message`) written to `NV_REPLAY_OUT`, for
    /// `tools/cmp2.pl` to set against the measured reads period by period (`cargo test -p
    /// engine-core --lib -- --ignored measured_replay`; `cmp2.pl programs out abs 0` compares
    /// every read). A second converter turned on in the measurement (the `apl` and `big` parts)
    /// is turned on here by the line's part. The counts per data set and the lines that differ
    /// are in docs/SOUND.md.
    #[test]
    #[ignore]
    fn measured_replay() {
        let Ok(file) = std::env::var("NV_REPLAY_PROGRAMS") else {
            return;
        };
        let text = std::fs::read_to_string(file).unwrap();
        let mut o = String::new();
        std::panic::set_hook(Box::new(|_| {}));
        for line in text.lines() {
            let id = line.split('\t').next().unwrap();
            let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| replay_program(line)));
            match r {
                Ok(s) => o += &format!("{id}\t{s}\n"),
                Err(p) => {
                    let msg = p
                        .downcast_ref::<String>()
                        .cloned()
                        .or_else(|| p.downcast_ref::<&str>().map(|s| s.to_string()))
                        .unwrap_or_default();
                    o += &format!("{id}\tPANIC {}\n", msg.replace(['\t', '\n'], " "));
                }
            }
        }
        let _ = std::panic::take_hook();
        std::fs::write(std::env::var("NV_REPLAY_OUT").unwrap(), o).unwrap();
    }

    /// Every `dsound-capture freqphase` case (the file in `NV_DSOUND_FREQPHASE`) replayed through
    /// the layer: the same calls after the same number of periods (the measured write cursor's
    /// changes), the cursors read at every period. Checks that nothing stops and that right
    /// after `SetFrequency` both cursors read as just before it, as measured in every case
    /// (`cargo test -p engine-core --lib -- --ignored freqphase_replay --nocapture`).
    #[test]
    #[ignore]
    fn freqphase_replay() {
        let Ok(file) = std::env::var("NV_DSOUND_FREQPHASE") else {
            return;
        };
        let text = std::fs::read_to_string(file).unwrap();
        let mut tally: BTreeMap<String, (u32, u32)> = BTreeMap::new();
        for line in text.lines().filter(|l| l.starts_with("DsFreqPhase")) {
            let f: Vec<&str> = line.split('\t').collect();
            let w: Vec<&str> = f[1].split(' ').collect();
            let hex = |s: &str| u32::from_str_radix(s, 16).unwrap();
            let (rate, channels, bits, bytes) = (hex(w[0]), hex(w[1]) as u16, hex(w[2]), hex(w[3]));
            let (variant, to, part): (&str, u32, &str) = (w[4], w[5].parse().unwrap(), w[8]);
            let align = u32::from(channels) * bits / 8;
            // The calls, each after the number of periods the write cursor had moved by then; and
            // the measured reads after each call (play and write frames, distinct, `None` once
            // ended), the first the read right after it.
            // A period: a change of the write cursor while it plays, or its end (the status going
            // to 0 without a call).
            let (mut periods, mut calls) = (0u32, Vec::new());
            let mut last: Option<(u32, Option<bool>)> = None;
            let mut measured: Vec<Vec<Option<(u32, u32)>>> = vec![Vec::new()];
            for x in f[3].split(' ') {
                if let Some(m) = x.strip_prefix('|') {
                    calls.push((periods, m.split('@').next().unwrap().to_string()));
                    measured.push(Vec::new());
                    last = last.map(|(w, _)| (w, None));
                    continue;
                }
                let v: Vec<u32> = x.split(':').skip(1).map(hex).collect();
                let seg = measured.last_mut().unwrap();
                let playing = v[2] & 1 != 0;
                let read = playing.then_some((v[0] / align, v[1] / align));
                if seg.last() != Some(&read) && seg.last() != Some(&None) {
                    seg.push(read);
                }
                if let Some((w, Some(true))) = last {
                    if v[1] != w || !playing {
                        periods += 1;
                    }
                }
                last = Some((v[1], Some(playing)));
            }
            let mut e = engine();
            let ds = start_up(&mut e);
            let b = create_format(&mut e, ds, (channels, rate, bits as u16), bytes);
            if variant == "steady" && to != 0 {
                e.vcall(b, 0x44, &[to]);
            }
            e.vcall(b, 0x30, &[0, 0, 0]);
            let read = |e: &mut Engine| {
                let (p, w, s) = position(e, b);
                (s & 1 != 0).then_some((p / align, w / align))
            };
            let mut layer: Vec<Vec<Option<(u32, u32)>>> = vec![Vec::new()];
            let mut at = 0;
            let n = calls.len();
            for (k, (p, what)) in calls.into_iter().enumerate() {
                while at < p {
                    at += 1;
                    e.clock_ms = 10 * at;
                    let r = read(&mut e);
                    layer.last_mut().unwrap().push(r);
                }
                let before = position(&mut e, b);
                match what.as_str() {
                    "f" => {
                        assert_eq!(e.vcall(b, 0x44, &[to]).u32(), DS_OK);
                        if before.2 & 1 != 0 {
                            assert_eq!(position(&mut e, b), before, "{}: right after", f[1]);
                        }
                    }
                    "stop" => {
                        e.vcall(b, 0x48, &[]);
                    }
                    "play" => {
                        e.vcall(b, 0x30, &[0, 0, 0]);
                    }
                    _ => {}
                }
                layer.push(vec![read(&mut e)]);
                if k + 1 == n {
                    for j in 1..=40 {
                        e.clock_ms = 10 * (at + j);
                        let r = read(&mut e);
                        layer.last_mut().unwrap().push(r);
                    }
                }
            }
            // The steps after `SetFrequency` while it plays and after the last `Play`, from the
            // read right after the call, to the end or the next call.
            let steps = |s: &[Option<(u32, u32)>]| -> Vec<Option<(i64, i64)>> {
                let Some(Some((p0, w0))) = s.first().copied() else {
                    return Vec::new();
                };
                let mut out = Vec::new();
                for r in s {
                    out.push(r.map(|(p, w)| {
                        (i64::from(p) - i64::from(p0), i64::from(w) - i64::from(w0))
                    }));
                    if r.is_none() {
                        break;
                    }
                }
                out
            };
            let segments: Vec<usize> = (1..measured.len())
                .filter(|&i| {
                    let name = &f[3]
                        .split(' ')
                        .filter(|x| x.starts_with('|'))
                        .nth(i - 1)
                        .unwrap()[1..];
                    name.starts_with("f@") && !measured[i].is_empty() || name.starts_with("play@")
                })
                .collect();
            let mut ok = true;
            for i in segments {
                let (m, l) = (steps(&measured[i]), steps(&layer[i]));
                let k = m.len().min(l.len());
                if m[..k] != l[..k] {
                    ok = false;
                    let at = (0..k).find(|&j| m[j] != l[j]).unwrap_or(0);
                    eprintln!(
                        "  after call {i} from step {at}: measured {:?}, layer {:?}",
                        &m[at..k.min(at + 4)],
                        &l[at..k.min(at + 4)]
                    );
                }
            }
            let entry = tally.entry(format!("{part} {variant}")).or_default();
            entry.0 += u32::from(ok);
            entry.1 += 1;
            if !ok {
                eprintln!("differs: {}", f[1]);
            }
        }
        let total: u32 = tally.values().map(|t| t.1).sum();
        for (k, (ok, n)) in &tally {
            eprintln!("{k}: {ok} of {n} as measured after the calls");
        }
        eprintln!("{total} cases replayed");
        assert!(total > 0);
    }

    /// Measured (`endstop-run1..3.tsv` `5622 1 10 2274 loopoff dmo` and `2b11 1 8 89d loopoff_f
    /// apl`, the same in all three runs): a looping buffer turned not looping 30 periods into its
    /// first `Play` plays out the lap it is in and ends there; a `SetFrequency` to the frequency it
    /// has changes nothing. Then the reviewer's sequence: a later `Play` past its end, retuned,
    /// turned not looping.
    #[test]
    fn a_looping_buffer_turned_not_looping_plays_out_its_lap() {
        let mut e = engine();
        let ds = start_up(&mut e);
        // 4410 frames at 22050 Hz, the first converter: 33 + 6615 frames on after 30 periods.
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 8820);
        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        e.clock_ms = 300;
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        let writes = [2238, 2459, 2679, 2900, 3120, 3341, 3561, 3782, 4002, 4223];
        for (i, w) in writes.into_iter().enumerate() {
            e.clock_ms = 300 + 10 * i as u32;
            assert_eq!(
                position(&mut e, b),
                (2 * (w - 661), 2 * w, 1),
                "period {}",
                30 + i
            );
        }
        e.clock_ms = 400;
        assert_eq!(position(&mut e, b), (0, 0, 0));
        // 2205 frames at 11025 Hz 8-bit, the second converter.
        second_converter(&mut e, ds);
        let b = create_format(&mut e, ds, (1, 11025, 8), 2205);
        e.clock_ms = 1000;
        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        e.clock_ms = 1300;
        assert_eq!(e.vcall(b, 0x44, &[11025]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        let writes = [
            0x44f, 0x4bd, 0x52c, 0x59a, 0x608, 0x676, 0x6e5, 0x753, 0x7c1, 0x82f,
        ];
        for (i, w) in writes.into_iter().enumerate() {
            e.clock_ms = 1300 + 10 * i as u32;
            assert_eq!(position(&mut e, b), (w - 330, w, 1), "period {}", 30 + i);
        }
        e.clock_ms = 1400;
        assert_eq!(position(&mut e, b), (0, 0, 0));
        // A later Play 100 periods on (442 + 22050 frames, the second converter's), retuned to
        // 22000 Hz, then not looping: it ends at 44100, 99 periods of 220 on.
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.clock_ms = 2005;
        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        e.clock_ms = 2025;
        e.vcall(b, 0x48, &[]);
        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        e.clock_ms = 3025;
        assert_eq!(e.vcall(b, 0x44, &[22000]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        expect_cursors(
            &mut e,
            b,
            &[
                (3025, (2 * 21831, 2 * 442, 1)),
                (3035, (2, 2 * 662, 1)),
                (4005, (2 * 21341, 2 * 22002, 1)),
                (4015, (0, 0, 0)),
            ],
        );
    }

    /// Measured (`freqphase-run1..3.tsv` `5622 1 10 ac44 retune_stop 22071 6 1 apl`): a first
    /// `Play` retuned on a whole frame (6 periods at 22050 Hz) goes on in a later `Play`'s steps
    /// from its write cursor: 884:1545 a period later, then after `Stop` and `Play` 1766, 1987,
    /// 2207 (the first `Play`'s rounding would give 1544 and 1765, 1986, 2206).
    #[test]
    fn a_first_play_retuned_on_a_whole_frame_takes_later_steps() {
        let mut e = engine();
        let ds = start_up(&mut e);
        second_converter(&mut e, ds);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.clock_ms = 60;
        assert_eq!(position(&mut e, b), (2 * 663, 2 * 1324, 1));
        assert_eq!(e.vcall(b, 0x44, &[22071]).u32(), DS_OK);
        assert_eq!(
            position(&mut e, b),
            (2 * 663, 2 * 1324, 1),
            "right after the call"
        );
        e.clock_ms = 70;
        assert_eq!(position(&mut e, b), (2 * 884, 2 * 1545, 1));
        assert_eq!(e.vcall(b, 0x48, &[]).u32(), DS_OK);
        e.clock_ms = 80;
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        expect_cursors(
            &mut e,
            b,
            &[
                (90, (2 * 1545, 2 * 1766, 1)),
                (100, (2 * 1545, 2 * 1987, 1)),
                (110, (2 * 1546, 2 * 2207, 1)),
                (120, (2 * 1767, 2 * 2428, 1)),
            ],
        );
    }

    /// Measured (`endstop-run1..3.tsv`): a buffer that does not loop ends in the period its write
    /// cursor would reach its length, a first `Play`'s too, so it is never read there playing;
    /// stopped in the period before, `Play` plays the rest (here at once: `last` with both
    /// converters). 22051 frames at 22050 Hz.
    #[test]
    fn a_buffer_is_never_stopped_at_its_end() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 2 * 22051);
        e.vcall(b, 0x30, &[0, 0, 0]);
        // The first converter: 33 + 21830 frames after 99 periods, 33 + 22050 after 100.
        e.clock_ms = 990;
        assert_eq!(position(&mut e, b), (2 * 21202, 2 * 21863, 1));
        e.clock_ms = 1000;
        assert_eq!(
            position(&mut e, b),
            (0, 0, 0),
            "ended, not read at its length"
        );
        assert_eq!(e.vcall(b, 0x48, &[]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        assert_eq!(position(&mut e, b).2, 1, "from its start");
        // Stopped after 99 periods; Play ends in its first period (21863 + 220 >= 22051).
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 2 * 22051);
        e.clock_ms = 2000;
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.clock_ms = 2990;
        assert_eq!(e.vcall(b, 0x48, &[]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (2 * 21863, 2 * 21863, 0));
        e.clock_ms = 3000;
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (2 * 21863, 2 * 21863, 1));
        e.clock_ms = 3010;
        assert_eq!(position(&mut e, b), (0, 0, 0));
        // The second converter: 21830 + 1 after 99 periods, then 220 more ends it.
        second_converter(&mut e, ds);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 2 * 22051);
        e.clock_ms = 4000;
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.clock_ms = 4990;
        assert_eq!(e.vcall(b, 0x48, &[]).u32(), DS_OK);
        assert_eq!(position(&mut e, b), (2 * 21831, 2 * 21831, 0));
        e.clock_ms = 5000;
        assert_eq!(e.vcall(b, 0x30, &[0, 0, 0]).u32(), DS_OK);
        e.clock_ms = 5010;
        assert_eq!(position(&mut e, b), (0, 0, 0));
    }

    /// Measured (`freqphase-run1.tsv` `2b11 1 8 8d4 ended_stop 0 0 2260 dmo`): played to its end
    /// (in its 21st period), stopped and played again, a buffer goes on in the steps it was
    /// taking.
    #[test]
    fn a_buffer_ended_then_stopped_plays_on_in_its_steps() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let b = create_format(&mut e, ds, (1, 11025, 8), 2260);
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.clock_ms = 300;
        assert_eq!(position(&mut e, b), (0, 0, 0));
        assert_eq!(e.vcall(b, 0x48, &[]).u32(), DS_OK);
        e.clock_ms = 400;
        e.vcall(b, 0x30, &[0, 0, 0]);
        let steps = [(0, 110), (0, 220), (0, 330), (111, 441), (221, 551)];
        for (i, (p, w)) in steps.into_iter().enumerate() {
            e.clock_ms = 410 + 10 * i as u32;
            assert_eq!(position(&mut e, b), (p, w, 1), "period {}", i + 1);
        }
    }

    /// A change of 1/500 of the rate or less: the first converter goes on from where the buffer
    /// is at the new frequency, its position's hundredths carried. A larger one turns on the
    /// second converter, and the buffer goes on from its write cursor with a new one, both cursors
    /// there (`dsound` `1005713d`; not measured): its frames from there (`resampler::AplFrames`).
    #[test]
    fn a_new_frequency_goes_on_from_where_the_buffer_is() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.vcall(b, 0x30, &[0, 0, 1]);
        e.clock_ms = 20; // 441 frames, 33 ahead
        e.vcall(b, 0x44, &[22080]);
        e.clock_ms = 30; // one period at 22080: 220.8 more, 0.8 of them past a whole frame
        assert_eq!(position(&mut e, b).1, 2 * (33 + 441 + 221));
        e.clock_ms = 40; // 441.6
        assert_eq!(position(&mut e, b).1, 2 * (33 + 441 + 442));
        assert!(!e.kernel.sound.apl);
        e.vcall(b, 0x44, &[44100]);
        assert!(e.kernel.sound.apl);
        assert_eq!(
            position(&mut e, b),
            (2 * 916, 2 * 916, 5),
            "both on the write cursor"
        );
        e.clock_ms = 50; // the new converter's first period at 44100
        assert_eq!(position(&mut e, b), (2 * 916, 2 * (916 + 442), 5));
    }

    /// Another buffer's large change turns on the second converter while these play (the DLL,
    /// `dsound` `1005713d`; not measured): each goes on from its write cursor with a new converter,
    /// both cursors there, the play cursor held until the write cursor is three periods past.
    #[test]
    fn a_buffer_playing_when_the_second_converter_turns_on_goes_on_from_its_write_cursor() {
        let mut e = engine();
        let ds = start_up(&mut e);
        // 22050 Hz mono 16-bit: 33 + 2205 frames after 10 periods (0x117c bytes), then 221, 442,
        // 663 more; three periods at 22050 Hz are 661 frames.
        let (_, mono) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        // 44100 Hz stereo 16-bit: 33 + 4410 frames (0x456c bytes), then 442, 883, 1324 more; three
        // periods are 1323 frames.
        let stereo = create_format(&mut e, ds, (2, 44100, 16), 4 * 44100);
        e.vcall(mono, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        e.vcall(stereo, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        e.clock_ms = 100;
        assert_eq!(position(&mut e, mono), (0xc52, 0x117c, 5));
        second_converter(&mut e, ds);
        let expected = [
            (100, (0x117c, 0x117c), (0x456c, 0x456c)),
            (110, (0x117c, 0x1336), (0x456c, 0x4c54)),
            (120, (0x117c, 0x14f0), (0x456c, 0x5338)),
            (130, (0x1180, 0x16aa), (0x4570, 0x5a1c)),
        ];
        for (at, (mp, mw), (sp, sw)) in expected {
            e.clock_ms = at;
            assert_eq!(position(&mut e, mono), (mp, mw, 5), "mono at {at} ms");
            assert_eq!(position(&mut e, stereo), (sp, sw, 5), "stereo at {at} ms");
        }
    }

    /// The output on, at the test format's rate, without a device: the tests render by hand.
    fn hear(e: &mut Engine) -> Arc<Mutex<Voices>> {
        let voices = Arc::new(Mutex::new(Voices::default()));
        e.kernel.sound.mix = Some(Mix {
            voices: voices.clone(),
            rate: 22050,
            _output: None,
        });
        voices
    }

    /// Fills the buffer `b` (`bytes` long) with 16-bit `samples` through `Lock`/`Unlock`.
    fn fill(e: &mut Engine, b: u32, bytes: u32, samples: &[i16]) {
        let (p1, n1) = (0x1800, 0x1804);
        e.vcall(b, 0x2c, &[0, bytes, p1, n1, 0, 0, DSBLOCK_ENTIREBUFFER]);
        let at = e.mem.u32(p1);
        let data: Vec<u8> = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
        e.mem.write(at, &data);
        let n = e.mem.u32(n1);
        e.vcall(b, 0x4c, &[at, n, 0, 0]);
    }

    /// The bits the output gets and the cursors the game reads over 2 s of buffers played, moved,
    /// refilled, retuned and stopped through the interface, as one hash; the same as the build
    /// before the allocation work (`cargo test --release -p engine-core --lib output_bits --
    /// --ignored --nocapture`), but for the cursors, changed 2026-10-31 and 2026-11-01 (measured,
    /// module documentation): a buffer played again after its end goes on in the steps it was
    /// taking, after `SetFrequency` the play cursor goes on trailing, the buffers here, at the
    /// output's rate, lead by nothing (the first converter passes them through), and with the
    /// second converter on the write cursor is the frames it takes. `plain[1]`'s change to
    /// 22094 Hz (in its first `Play`, 40 periods in) is one the first converter takes at a
    /// frequency other than the format's rate, where the layer keeps the rule of hundredths, and
    /// `plain[2]`'s change to 30000 Hz turns the second converter on while all seven play (not
    /// measured: as the DLL reads, each goes on from its write cursor with a new converter, both
    /// cursors there), so the combined hash rests on those two there. The samples alone hash as
    /// before.
    #[test]
    #[ignore]
    fn output_bits_are_unchanged() {
        let mut e = engine();
        let ds = start_up(&mut e);
        e.mem.set_u32(0x1200, 0x24);
        e.mem.set_u32(0x1204, 0x91);
        e.vcall(ds, 0xc, &[0x1200, 0x1004, 0]);
        let primary = e.mem.u32(0x1004);
        e.mem.write(0x1300, &IID_LISTENER);
        e.vcall(primary, 0, &[0x1300, 0x1008]);
        let listener = e.mem.u32(0x1008);
        let voices = hear(&mut e);
        let mut x = 0x2545_f491u32;
        let mut noise = |n: usize| -> Vec<i16> {
            (0..n)
                .map(|_| {
                    x ^= x << 13;
                    x ^= x >> 17;
                    x ^= x << 5;
                    (x % 20001) as i16 - 10000
                })
                .collect()
        };
        let mut plain = Vec::new();
        for _ in 0..4 {
            let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
            fill(&mut e, b, 44100, &noise(22050));
            plain.push(b);
        }
        let mut three_d = Vec::new();
        for _ in 0..3 {
            let (_, b) = create(&mut e, ds, 0x302b0, true, 1, 44100);
            fill(&mut e, b, 44100, &noise(22050));
            e.mem.write(0x1300, &IID_BUFFER3D);
            e.vcall(b, 0, &[0x1300, 0x100c]);
            three_d.push((b, e.mem.u32(0x100c)));
        }
        let mut hash = 0xcbf2_9ce4_8422_2325u64;
        let (mut hc, mut hs) = (hash, hash);
        let eat = |hash: &mut u64, w: u32| {
            for b in w.to_le_bytes() {
                *hash = (*hash ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
            }
        };
        let mut out = vec![0f32; 2 * 220];
        for period in 0..200u32 {
            let t = period as f32 * 0.1;
            match period {
                0 => {
                    for (i, &b) in plain.iter().enumerate() {
                        let flags = if i % 2 == 0 { DSBPLAY_LOOPING } else { 0 };
                        e.vcall(b, 0x30, &[0, 0, flags]);
                    }
                    for &(b, _) in &three_d {
                        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
                    }
                }
                40 => {
                    e.vcall(plain[1], 0x44, &[22094]);
                }
                60 => {
                    // A refill while playing (the voice's copy follows the Unlock).
                    let b = plain[0];
                    e.vcall(b, 0x2c, &[0, 4410, 0x1800, 0x1804, 0x1808, 0x180c, 0]);
                    let (p, n) = (e.mem.u32(0x1800), e.mem.u32(0x1804));
                    let fresh: Vec<u8> = noise(2205).iter().flat_map(|s| s.to_le_bytes()).collect();
                    e.mem.write(p, &fresh[..n as usize]);
                    e.vcall(b, 0x4c, &[p, n, 0, 0]);
                }
                90 => {
                    e.vcall(plain[2], 0x44, &[30000]);
                }
                120 => {
                    e.vcall(plain[3], 0x48, &[]);
                    e.vcall(plain[1], 0x30, &[0, 0, DSBPLAY_LOOPING]);
                }
                _ => {}
            }
            e.vcall(plain[0], 0x3c, &[(-((period % 50) as i32) * 20) as u32]);
            e.vcall(plain[2], 0x40, &[(period % 40) * 100]);
            for (k, &(_, b3)) in three_d.iter().enumerate() {
                let p = [t + k as f32, 1.0, (t * 0.5).sin() * 4.0];
                e.vcall(
                    b3,
                    0x4c,
                    &[p[0].to_bits(), p[1].to_bits(), p[2].to_bits(), 0],
                );
            }
            let l = [t.cos() * 2.0, 0.0, t.sin() * 2.0];
            e.vcall(
                listener,
                0x38,
                &[l[0].to_bits(), l[1].to_bits(), l[2].to_bits(), 1],
            );
            e.vcall(listener, 0x44, &[]);
            e.clock_ms += 10;
            for &b in plain.iter().chain(three_d.iter().map(|(b, _)| b)) {
                e.vcall(b, 0x10, &[0x1d00, 0x1d04]);
                e.vcall(b, 0x24, &[0x1d08]);
                for w in [0x1d00, 0x1d04, 0x1d08] {
                    eat(&mut hash, e.mem.u32(w));
                    eat(&mut hc, e.mem.u32(w));
                }
            }
            out.fill(0.0);
            super::super::sound_out::Source::render(
                &mut *voices.lock().unwrap(),
                22050,
                2,
                &mut out,
            );
            for s in &out {
                eat(&mut hash, s.to_bits());
                eat(&mut hs, s.to_bits());
            }
        }
        println!("output hash {hash:016x} cursors {hc:016x} samples {hs:016x}");
        assert_eq!(hs, 0x4323_299c_daf5_af1d, "the samples");
        assert_eq!(hash, 0x771c_dd2d_987a_add3);
    }

    /// Not a check: the host heap allocations of the buffers' per-frame calls with the output on
    /// (the game's thread) and of the mixer's 10 ms periods (the output's thread), for the
    /// frame-rate work (`cargo test --release -p engine-core --lib allocations_per -- --ignored
    /// --nocapture`).
    #[test]
    #[ignore]
    fn allocations_per_operation_of_directsound() {
        use crate::alloc_count::{per_run, row};
        let mut e = engine();
        let ds = start_up(&mut e);
        e.mem.set_u32(0x1200, 0x24);
        e.mem.set_u32(0x1204, 0x91);
        e.vcall(ds, 0xc, &[0x1200, 0x1004, 0]);
        let primary = e.mem.u32(0x1004);
        e.mem.write(0x1300, &IID_LISTENER);
        e.vcall(primary, 0, &[0x1300, 0x1008]);
        let listener = e.mem.u32(0x1008);
        let voices = hear(&mut e);
        // Eight plain buffers and four 3D ones, playing, as a room's sounds.
        let mut plain = Vec::new();
        for i in 0..8i16 {
            let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
            fill(&mut e, b, 44100, &vec![i * 100; 22050]);
            e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
            plain.push(b);
        }
        let mut three_d = Vec::new();
        for i in 0..4i16 {
            let (_, b) = create(&mut e, ds, 0x302b0, true, 1, 44100);
            fill(&mut e, b, 44100, &vec![i * 100 + 50; 22050]);
            e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
            e.mem.write(0x1300, &IID_BUFFER3D);
            e.vcall(b, 0, &[0x1300, 0x100c]);
            three_d.push(e.mem.u32(0x100c));
        }
        let b = plain[0];
        // One plain buffer at another rate: its voice goes through the converter.
        e.vcall(plain[1], 0x44, &[11025]);
        let ms = |e: &mut Engine| e.clock_ms += 1;
        row(
            "IDirectSoundBuffer GetCurrentPosition",
            per_run(1000, || {
                ms(&mut e);
                e.vcall(b, 0x10, &[0x1d00, 0x1d04]);
            }),
        );
        row(
            "IDirectSoundBuffer GetStatus",
            per_run(1000, || {
                e.vcall(b, 0x24, &[0x1d08]);
            }),
        );
        let mut volume = 0u32;
        row(
            "IDirectSoundBuffer SetVolume",
            per_run(1000, || {
                volume = (volume + 1) % 100;
                e.vcall(b, 0x3c, &[(-(volume as i32)) as u32]);
            }),
        );
        row(
            "IDirectSoundBuffer SetPan",
            per_run(1000, || {
                volume = (volume + 1) % 100;
                e.vcall(b, 0x40, &[volume]);
            }),
        );
        row(
            "IDirectSoundBuffer SetFrequency (playing)",
            per_run(1000, || {
                volume = (volume + 1) % 100;
                e.vcall(plain[1], 0x44, &[11025 + volume]);
            }),
        );
        row(
            "IDirectSoundBuffer Lock + Unlock, 4410 of 44100 bytes",
            per_run(1000, || {
                e.vcall(b, 0x2c, &[0, 4410, 0x1800, 0x1804, 0x1808, 0x180c, 0]);
                let (p, n) = (e.mem.u32(0x1800), e.mem.u32(0x1804));
                e.vcall(b, 0x4c, &[p, n, 0, 0]);
            }),
        );
        let mut x = 0f32;
        row(
            "IDirectSound3DBuffer SetPosition (at once)",
            per_run(1000, || {
                x += 0.25;
                e.vcall(three_d[0], 0x4c, &[x.to_bits(), 0, 0, 0]);
            }),
        );
        row(
            "IDirectSound3DListener SetPosition (deferred) + CommitDeferredSettings",
            per_run(1000, || {
                x += 0.25;
                e.vcall(listener, 0x38, &[x.to_bits(), 0, 0, 1]);
                e.vcall(listener, 0x44, &[]);
            }),
        );
        let mut out = vec![0f32; 2 * 220];
        row(
            "mixer: 10 ms of 12 voices (one converted)",
            per_run(200, || {
                out.fill(0.0);
                super::super::sound_out::Source::render(
                    &mut *voices.lock().unwrap(),
                    22050,
                    2,
                    &mut out,
                );
            }),
        );
    }

    #[test]
    fn heard_buffers_are_mixed_newest_first_with_their_settings() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let voices = hear(&mut e);
        let size = 44100;
        let (_, a) = create(&mut e, ds, 0x2e0, false, 1, size);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, size);
        let mut sa = vec![0i16; (size / 2) as usize];
        let mut sb = sa.clone();
        sa[..3].copy_from_slice(&[1000, -2000, 3000]);
        sb[..3].copy_from_slice(&[7, 11, -13]);
        fill(&mut e, a, size, &sa);
        fill(&mut e, b, size, &sb);
        e.vcall(a, 0x3c, &[(-600i32) as u32]);
        e.vcall(b, 0x40, &[1000]);
        e.vcall(a, 0x30, &[0, 0, 0]);
        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        assert_eq!(voices.lock().unwrap().playing, vec![b, a]);
        let mut out = [0f32; 6];
        super::super::sound_out::Source::render(&mut *voices.lock().unwrap(), 22050, 2, &mut out);
        let x = |v: i16| f32::from(v) / 32768.0;
        let (ga, gb) = (
            super::super::mixer::gain(-600),
            super::super::mixer::gain(-1000),
        );
        for i in 0..3 {
            // b first (newest), its left side attenuated by the pan; then a at -6 dB.
            assert_eq!(out[2 * i], x(sb[i]) * gb + x(sa[i]) * ga);
            assert_eq!(out[2 * i + 1], x(sb[i]) + x(sa[i]) * ga);
        }
        e.vcall(a, 0x48, &[]);
        assert_eq!(voices.lock().unwrap().playing, vec![b]);
        e.vcall(b, 0x8, &[]);
        let v = voices.lock().unwrap();
        assert!(v.playing.is_empty() && !v.all.contains_key(&b) && v.all.contains_key(&a));
    }

    #[test]
    fn a_heard_buffer_at_another_rate_is_resampled() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let voices = hear(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        e.vcall(b, 0x30, &[0, 0, 0]);
        e.vcall(b, 0x44, &[11025]);
        let v = voices.lock().unwrap();
        assert_eq!((v.all[&b].rate, v.all[&b].device), (11025, 22050));
        assert!(!v.all[&b].exact());
    }

    /// DirectSound's converters (`dsound` `100581e0`): a move of more than 1/500 of the rate turns
    /// on the second converter for every buffer of the device; a smaller one keeps the first.
    #[test]
    fn a_large_frequency_change_turns_on_the_second_converter() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let voices = hear(&mut e);
        let (_, a) = create(&mut e, ds, 0x2e0, false, 1, 22050);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 22050);
        e.vcall(a, 0x30, &[0, 0, 0]);
        // 44 Hz is 1/500 of 22050 (in whole numbers): the first converter stays.
        e.vcall(a, 0x44, &[22094]);
        {
            let v = voices.lock().unwrap();
            assert!(!v.apl && !v.all[&a].apl && !v.all[&b].apl);
            assert_eq!(v.all[&a].rate, 22094);
        }
        e.vcall(b, 0x44, &[22140]);
        let v = voices.lock().unwrap();
        assert!(v.apl && v.all[&a].apl && v.all[&b].apl);
        assert_eq!((v.all[&a].rate, v.all[&b].rate), (22094, 22140));
    }

    #[test]
    fn a_stream_plays_pauses_and_stops() {
        let mut e = engine();
        let voices = hear(&mut e);
        let pcm: Vec<u8> = [1000i16, -1000, 2000, -2000, 3000, -3000]
            .iter()
            .flat_map(|s| s.to_le_bytes())
            .collect();
        play_stream(&mut e, 0x9000, pcm, 2, 16, 22050);
        let mut out = [0f32; 2];
        super::super::sound_out::Source::render(&mut *voices.lock().unwrap(), 22050, 2, &mut out);
        assert_eq!(out, [1000.0 / 32768.0, -1000.0 / 32768.0]);
        pause_stream(&mut e, 0x9000, true);
        assert!(voices.lock().unwrap().playing.is_empty());
        pause_stream(&mut e, 0x9000, false);
        let mut out = [0f32; 2];
        super::super::sound_out::Source::render(&mut *voices.lock().unwrap(), 22050, 2, &mut out);
        assert_eq!(out, [2000.0 / 32768.0, -2000.0 / 32768.0]);
        stop_stream(&mut e, 0x9000);
        assert!(voices.lock().unwrap().all.is_empty());
    }

    #[test]
    fn a_stream_at_another_rate_is_resampled() {
        let mut e = engine();
        let voices = hear(&mut e);
        play_stream(&mut e, 0x9000, vec![0; 4], 2, 16, 44100);
        assert_eq!(voices.lock().unwrap().all[&0x9000].rate, 44100);
        seek_stream(&mut e, 0x9000, 1);
        stream_volume(&mut e, 0x9000, -100);
        let v = voices.lock().unwrap();
        assert_eq!(
            (v.all[&0x9000].position, v.all[&0x9000].volume),
            (Some(1), -100)
        );
    }

    #[test]
    fn a_heard_3d_buffer_follows_its_position() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let voices = hear(&mut e);
        let (_, b) = create(&mut e, ds, 0x302b0, false, 1, 44100);
        e.vcall(b, 0x30, &[0, 0, 0]);
        // At the listener: an even split, 5 dB down (`gain(-500) * gain(-301)`), as measured
        // straight ahead at the minimum distance.
        let gains = |voices: &Arc<Mutex<Voices>>| {
            let (l, r) = voices.lock().unwrap().all[&b].spatial.unwrap();
            (l.to_bits(), r.to_bits())
        };
        assert_eq!(gains(&voices), (0x3ed0_9516, 0x3ed0_9516));
        let three_d = buffer3d_of(&mut e, b);
        // Far to the left, at once: the right side is silent (measured).
        e.vcall(three_d, 0x4c, &[(-4.0f32).to_bits(), 0, 0, 0]);
        assert_eq!(gains(&voices), (0x3e12_5092, 0));
        // Deferred to the right: nothing changes until the listener commits.
        e.vcall(three_d, 0x4c, &[4.0f32.to_bits(), 0, 0, DS3D_DEFERRED]);
        assert_eq!(gains(&voices), (0x3e12_5092, 0));
        let listener = listener_of(&mut e, ds);
        assert_eq!(e.vcall(listener, 0x44, &[]).u32(), DS_OK);
        assert_eq!(gains(&voices), (0, 0x3e12_5092));
        // The listener turned to face +x (at once): the source at +x is now straight ahead.
        let (one, zero) = (1.0f32.to_bits(), 0u32);
        let r = e.vcall(listener, 0x34, &[one, zero, zero, zero, one, zero, 0]);
        assert_eq!(r.u32(), DS_OK);
        // (The rotation reads whole degrees from the table, so "ahead" is within a degree.)
        let (l, r) = gains(&voices);
        let (l, r) = (f32::from_bits(l), f32::from_bits(r));
        assert!((l - r).abs() < 0.03 * l, "{l} {r}");
        // A parallel front and top are refused.
        let r = e.vcall(listener, 0x34, &[one, zero, zero, one, zero, zero, 0]);
        assert_eq!(r.u32(), DSERR_INVALIDPARAM);
    }

    /// The buffer's `IDirectSound3DBuffer`.
    fn buffer3d_of(e: &mut Engine, b: u32) -> u32 {
        let three_d = e.kernel.sound.buffers[&b].three_d;
        if three_d != 0 {
            return three_d;
        }
        let (out, iid) = (0x1900, 0x1910);
        e.mem.write(iid, &IID_BUFFER3D);
        e.vcall(b, 0, &[iid, out]);
        e.mem.u32(out)
    }

    /// The device's listener (a 3D primary buffer's).
    fn listener_of(e: &mut Engine, ds: u32) -> u32 {
        e.mem.set_u32(0x1200, 0x24);
        e.mem.set_u32(0x1204, 0x91);
        assert_eq!(e.vcall(ds, 0xc, &[0x1200, 0x1004, 0]).u32(), DS_OK);
        let primary = e.mem.u32(0x1004);
        e.mem.write(0x1300, &IID_LISTENER);
        assert_eq!(e.vcall(primary, 0, &[0x1300, 0x1008]).u32(), DS_OK);
        e.mem.u32(0x1008)
    }

    /// A made-up edge table for the distortion (not the player's: the tests check the chain).
    pub(super) const EDGE_TABLE: [f64; 32] = [1.25; 32];

    #[test]
    fn a_heard_buffer_with_the_distortion_plays_the_effect() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let voices = hear(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        // Samples: a ramp, written through Lock/Unlock.
        let data = e.kernel.sound.buffers[&b].data;
        let samples: Vec<u8> = (0..22050i32)
            .flat_map(|i| (((i * 7) % 2000 - 1000) as i16).to_le_bytes())
            .collect();
        e.mem.write(data, &samples);
        e.vcall(b, 0x2c, &[0, 44100, 0x1a00, 0x1a04, 0, 0, 0]);
        e.vcall(b, 0x4c, &[e.mem.u32(0x1a00), 44100, 0, 0]);
        let d = effect_desc(&mut e, &GUID_DISTORTION);
        assert_eq!(e.vcall(b, 0x54, &[1, d, 0x1c00]).u32(), DS_OK);
        e.mem.write(0x1c10, &GUID_DISTORTION);
        e.mem.write(0x1c20, &IID_DISTORTION8);
        assert_eq!(e.vcall(b, 0x5c, &[0x1c10, 0, 0x1c20, 0x1c30]).u32(), DS_OK);
        let fx = e.mem.u32(0x1c30);
        let p = [-18.2f32, 0.0, 5000.0, 1135.0, 5000.0];
        for (i, v) in p.into_iter().enumerate() {
            e.mem.set_f32(0x1c40 + 4 * i as u32, v);
        }
        assert_eq!(e.vcall(fx, 0xc, &[0x1c40]).u32(), DS_OK);
        e.vcall(b, 0x30, &[0, 0, 0]);
        let mut out = vec![0f32; 2 * 64];
        super::super::sound_out::Source::render(&mut *voices.lock().unwrap(), 22050, 2, &mut out);
        // By hand: the effect from 32 frames before the start (the end of the buffer).
        let c = super::super::dsfx::Coefficients::new(22050, p, &EDGE_TABLE);
        let mut d = super::super::dsfx::Distortion::new(c);
        let mut expect = samples.clone();
        for f in (22050 - 32..22050).chain(0..64) {
            d.frame(&mut expect[2 * f..2 * f + 2], 1, 16);
        }
        for f in 0..64 {
            let s = i16::from_le_bytes([expect[2 * f], expect[2 * f + 1]]);
            assert_eq!(out[2 * f], f32::from(s) / 32768.0, "frame {f}");
        }
    }

    /// Measured (`DSMIX_SETPOS`): a heard buffer moved while it plays plays out what was mixed
    /// (the rest of the period here), then goes on from the new place.
    #[test]
    fn a_heard_buffer_moved_goes_on_from_there() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let voices = hear(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        let samples: Vec<i16> = (0..22050i32)
            .map(|i| ((i * 7) % 2000 - 1000) as i16)
            .collect();
        fill(&mut e, b, 44100, &samples);
        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        let render = |n: usize| {
            let mut out = vec![0f32; 2 * n];
            super::super::sound_out::Source::render(
                &mut *voices.lock().unwrap(),
                22050,
                2,
                &mut out,
            );
            out
        };
        // Half of the first period (the device's 10 ms: 220 frames).
        let first = render(110);
        assert_eq!(e.vcall(b, 0x34, &[2 * 5000]).u32(), DS_OK);
        assert_eq!(e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]).u32(), DS_OK);
        let then = render(330);
        let x = |f: usize| f32::from(samples[f]) / 32768.0;
        for f in 0..110 {
            assert_eq!(first[2 * f], x(f), "frame {f}");
            assert_eq!(then[2 * f], x(110 + f), "the period's frame {}", 110 + f);
        }
        for f in 110..330 {
            assert_eq!(
                then[2 * f],
                x(5000 + f - 110),
                "moved: frame {}",
                5000 + f - 110
            );
        }
        // Not measured: a buffer the output already played to its end (ahead of the platform
        // clock) starts again there, in its place in the sum (`a` older, still playing).
        let a = b;
        let (_, b) = create(&mut e, ds, 0xe0, false, 1, 440);
        fill(&mut e, b, 440, &samples[..220]);
        e.vcall(b, 0x30, &[0, 0, 0]);
        let _ = render(440);
        {
            let v = voices.lock().unwrap();
            assert_eq!(v.all[&b].position, None, "played to its end");
            assert_eq!(v.playing, vec![b, a], "still in its place");
        }
        assert_eq!(e.vcall(b, 0x34, &[2 * 100]).u32(), DS_OK);
        assert_eq!(voices.lock().unwrap().playing, vec![b, a]);
        let again = render(120);
        // `a` has played 440 + 440 frames since it moved to 5000.
        for f in 0..120 {
            assert_eq!(
                again[2 * f],
                x(100 + f) + x(5000 + 220 + 440 + f),
                "again: frame {}",
                100 + f
            );
        }
        // Stopped, a voice leaves the list.
        e.vcall(b, 0x48, &[]);
        assert_eq!(voices.lock().unwrap().playing, vec![a]);
    }

    /// Measured (`DSMIX_SETPOS`, the captures with the distortion): moved while it plays, the
    /// effect starts over at the new place as at `Play` (cleared, from 32 frames before).
    #[test]
    fn a_heard_buffer_with_the_distortion_moved_starts_the_effect_there() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let voices = hear(&mut e);
        let (_, b) = create(&mut e, ds, 0x2e0, false, 1, 44100);
        let samples: Vec<u8> = (0..22050i32)
            .flat_map(|i| (((i * 7) % 2000 - 1000) as i16).to_le_bytes())
            .collect();
        let data = e.kernel.sound.buffers[&b].data;
        e.mem.write(data, &samples);
        e.vcall(b, 0x2c, &[0, 44100, 0x1a00, 0x1a04, 0, 0, 0]);
        e.vcall(b, 0x4c, &[e.mem.u32(0x1a00), 44100, 0, 0]);
        let d = effect_desc(&mut e, &GUID_DISTORTION);
        assert_eq!(e.vcall(b, 0x54, &[1, d, 0x1c00]).u32(), DS_OK);
        e.mem.write(0x1c10, &GUID_DISTORTION);
        e.mem.write(0x1c20, &IID_DISTORTION8);
        assert_eq!(e.vcall(b, 0x5c, &[0x1c10, 0, 0x1c20, 0x1c30]).u32(), DS_OK);
        let fx = e.mem.u32(0x1c30);
        let p = [-18.2f32, 0.0, 5000.0, 1135.0, 5000.0];
        for (i, v) in p.into_iter().enumerate() {
            e.mem.set_f32(0x1c40 + 4 * i as u32, v);
        }
        assert_eq!(e.vcall(fx, 0xc, &[0x1c40]).u32(), DS_OK);
        e.vcall(b, 0x30, &[0, 0, DSBPLAY_LOOPING]);
        let render = |n: usize| {
            let mut out = vec![0f32; 2 * n];
            super::super::sound_out::Source::render(
                &mut *voices.lock().unwrap(),
                22050,
                2,
                &mut out,
            );
            out
        };
        let first = render(110);
        assert_eq!(e.vcall(b, 0x34, &[2 * 5000]).u32(), DS_OK);
        let then = render(330);
        // By hand: the effect from 32 frames before each start.
        let processed = |from: usize, count: usize| {
            let c = super::super::dsfx::Coefficients::new(22050, p, &EDGE_TABLE);
            let mut d = super::super::dsfx::Distortion::new(c);
            let mut out = samples.clone();
            for f in (0..32 + count).map(|k| (from + 22050 - 32 + k) % 22050) {
                d.frame(&mut out[2 * f..2 * f + 2], 1, 16);
            }
            move |f: usize| f32::from(i16::from_le_bytes([out[2 * f], out[2 * f + 1]])) / 32768.0
        };
        let (old, new) = (processed(0, 220), processed(5000, 220));
        for f in 0..110 {
            assert_eq!(first[2 * f], old(f), "frame {f}");
            assert_eq!(then[2 * f], old(110 + f), "the period's frame {}", 110 + f);
        }
        for f in 110..330 {
            assert_eq!(
                then[2 * f],
                new(5000 + f - 110),
                "moved: frame {}",
                5000 + f - 110
            );
        }
    }

    #[test]
    #[should_panic(expected = "the sound table disagrees")]
    fn a_table_that_disagrees_stops() {
        let text = table_text().replace(
            "DsBuffer\t2e0 0 1 1 5622 10 3\t80070057",
            "DsBuffer\t2e0 0 1 1 5622 10 3\t0",
        );
        Table::parse(&text);
    }

    #[test]
    #[should_panic(expected = "is not provided")]
    fn co_create_instance_stops() {
        let mut e = engine();
        co_create_instance(
            &mut e,
            Ptr::new(0x1000),
            0,
            1,
            Ptr::new(0x1010),
            Ptr::new(0x1020),
        );
    }

    #[test]
    fn a_slot_not_provided_stops() {
        let mut e = engine();
        let ds = start_up(&mut e);
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            e.vcall(ds, 0x1c, &[]); // Compact
        }));
        assert!(stopped.is_err());
    }
}
