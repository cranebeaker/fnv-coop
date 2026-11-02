//! The platform's Bink (`binkw32.dll`, RAD Game Tools' library, version shipped with the
//! game): a platform layer like D3DX (the lead's decision, 2026-10-23). Each import the game
//! reaches is written here from its documented meaning, answered by the repository's own
//! Bink 1 decoder (`crates/bink`, docs/MOVIES.md), and matched against what the player's
//! `binkw32.dll` (SHA-256 `c6c06e2d...185bd`) answers to the same calls. The DLL's code is
//! never read for this layer: its answers are measured with `research/nv-oracle`'s `nv-bink`
//! (`--game`, `--wait`, `--audio`, `--sound-system`), which runs the player's DLL in a 32-bit
//! process; the recordings are private (`build43-scratch/measure/bink-*-2026-10-23`).
//!
//! The imports (FalloutNV.exe 1.4.0.525's import table, slot and every call site):
//! - `_BinkSetSoundSystem@8` (slot `00fdf3a0`): `00ec1680` passes the word in the
//!   `BinkOpenDirectSound` slot and 0 when the game's sound is on.
//! - `_BinkOpenDirectSound@4` (slot `00fdf3b4`): never called by the game; only its address
//!   is handed to `BinkSetSoundSystem`.
//! - `_BinkOpen@8` (slot `00fdf3b8`): `00ec0ac0`, the path (`Data\Video\` and the name) and
//!   the flags 0, `BINKPRELOADALL` (0x2000) and/or `BINKNOTHREADEDIO` (0x0800_0000).
//! - `_BinkWait@4` (slot `00fdf3ac`), `_BinkNextFrame@4` (`00fdf3a8`): `00ec1060`.
//! - `_BinkDoFrame@4` (slot `00fdf3bc`), `_BinkCopyToBufferRect@44` (`00fdf3c0`): `00ec14d0`,
//!   one copy per 256x256 texture tile with `(bink, bits, pitch, bink->Height, 0, 0, x, y, w,
//!   h, 3)` (`| 0x8000_0000` when its flag is set; `00ec1060` passes 0).
//! - `_BinkPause@8` (slot `00fdf3b0`): `00ec1fb0` (1) and `00ec1fd0` (0).
//! - `_BinkClose@4` (slot `00fdf3a4`): `00ec1020`.
//!
//! The game reads the handle's `Width` (+0), `Height` (+4) and `Frames` (+8) only
//! (`00ec0cd0`, `00ec14d0`, `00ec1060`, `00ec0e00`).
//!
//! What a case this layer has no measurement for does: it stops the run ("not provided").

use crate::{Engine, Ptr};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

/// The handle's fields (`HBINK`, bink.h order), as the DLL fills them for the intro
/// (measured, `words.tsv`): `Width`, `Height`, `Frames`, `FrameNum` (1-based, the frame
/// `BinkDoFrame` decodes next), `LastFrameNum` (the frame decoded last, `0xffffffff` after
/// `BinkOpen`), `FrameRate`, `FrameRateDiv`, `ReadError` (0), `OpenFlags` (the flags given
/// to `BinkOpen`), `BinkType` (0) and `Size` (the file's size field, file size - 8). The
/// DLL's structure goes on with its own state (pointers into its heap and code), which the
/// game never reads and this layer does not have.
pub const WIDTH: u32 = 0;
pub const HEIGHT: u32 = 4;
pub const FRAMES: u32 = 8;
pub const FRAME_NUM: u32 = 0xc;
pub const LAST_FRAME_NUM: u32 = 0x10;
pub const FRAME_RATE: u32 = 0x14;
pub const FRAME_RATE_DIV: u32 = 0x18;
pub const READ_ERROR: u32 = 0x1c;
pub const OPEN_FLAGS: u32 = 0x20;
pub const BINK_TYPE: u32 = 0x24;
pub const SIZE: u32 = 0x28;
/// The bytes of the handle this layer fills.
pub const HANDLE_BYTES: u32 = 0x2c;

/// `BinkOpen` flags the game passes: `BINKPRELOADALL` and `BINKNOTHREADEDIO` (bink.h). Both
/// only say how the DLL reads the file; this layer reads it whole either way.
pub const BINKPRELOADALL: u32 = 0x2000;
pub const BINKNOTHREADEDIO: u32 = 0x0800_0000;
/// `BinkCopyToBuffer(Rect)` flags: the surface type in the low 4 bits (`BINKSURFACE32` = 3,
/// 32 bits per pixel, blue, green, red, then a byte the DLL writes as 0) and `BINKCOPYALL`.
pub const BINKSURFACE32: u32 = 3;
pub const BINKSURFACEMASK: u32 = 0xf;
pub const BINKCOPYALL: u32 = 0x8000_0000;

/// The import slot holding `BinkOpenDirectSound`'s address (`00ec1691` reads it). The loader
/// (`Engine::register_imports`) puts each slot's own address in it.
pub const OPEN_DIRECT_SOUND_SLOT: u32 = 0x00fd_f3b4;

/// One page of engine memory for this layer's per-engine word: which engine this is (a
/// number handed out once per engine, [`engine_id`]). Like the other platform layers' code
/// addresses (`0x7e00_0000` up), no exe code uses it.
pub const STATE_PAGE: u32 = 0x7e14_0000;
/// The word after it: 1 once `BinkSetSoundSystem` chose the DirectSound system.
const SOUND_SYSTEM: u32 = 4;

/// After how many `BinkWait` calls in a row that answer "wait" at the same platform clock the
/// run stops: with the real clock off (tests) the clock (`Engine::clock_now_ms`) moves only with
/// `Sleep` and the test, so a movie the game waits on without sleeping would loop for ever.
pub const WAITS_WITHOUT_TIME: u32 = 100_000;

/// A movie this layer has open.
struct Playing {
    data: Vec<u8>,
    decoder: ::bink::Decoder,
    /// The frame index (0-based) decoded last, and its pixels as `Decoder::to_bgrx` gives
    /// them (made on the first copy).
    decoded: Option<u32>,
    pixels: Option<Vec<u8>>,
    /// `BinkNextFrame` went past the last frame (the DLL starts again at frame 1).
    wrapped: bool,
    rate: u32,
    rate_div: u32,
    /// The schedule ([`wait`]): the clock when it starts and the frame index it starts at.
    start: Option<u32>,
    base: u32,
    /// When `BinkPause(1)` paused it.
    paused: Option<u32>,
    /// The clock of the last `BinkWait` that answered "wait", and how many did in a row.
    waiting: (u32, u32),
    /// The movie's sound (with the DirectSound sound system chosen): its decoded track as
    /// 16-bit samples, channels and rate; played from the first `BinkDoFrame` ([`do_frame`]).
    track: Option<(Vec<u8>, u16, u32)>,
}

/// The open movies, by (engine, handle). The movie data and the decoder are host-side state
/// (like the decoder state of the DLL, which the game never sees).
static PLAYING: Mutex<BTreeMap<(u32, u32), Playing>> = Mutex::new(BTreeMap::new());
static NEXT_ENGINE: AtomicU32 = AtomicU32::new(1);

fn playing() -> std::sync::MutexGuard<'static, BTreeMap<(u32, u32), Playing>> {
    // A stop elsewhere while the lock was held must not stop every later engine.
    PLAYING.lock().unwrap_or_else(|p| p.into_inner())
}

/// This engine's number in [`PLAYING`] (kept at [`STATE_PAGE`]).
fn engine_id(e: &mut Engine) -> u32 {
    if !e.mem.is_mapped(STATE_PAGE) {
        e.mem.map(STATE_PAGE, 0x1000);
    }
    let mut id = e.mem.u32(STATE_PAGE);
    if id == 0 {
        id = NEXT_ENGINE.fetch_add(1, Ordering::Relaxed);
        e.mem.set_u32(STATE_PAGE, id);
    }
    id
}

/// Takes the movie behind `bink` out of [`PLAYING`] (put back with [`put`]), so a stop in the
/// middle of a call leaves no lock held.
fn take(e: &mut Engine, bink: u32, call: &str) -> Playing {
    let id = engine_id(e);
    playing()
        .remove(&(id, bink))
        .unwrap_or_else(|| panic!("{call}: {bink:#x} is not a Bink handle this layer opened"))
}

fn put(e: &mut Engine, bink: u32, movie: Playing) {
    let id = engine_id(e);
    playing().insert((id, bink), movie);
}

// Platform: replaces binkw32.dll _BinkSetSoundSystem@8 (import)
/// `BinkSetSoundSystem(open, param)`: chooses the sound system later `BinkOpen`s play through
/// (`open` is the system's open function, `param` its argument: for `BinkOpenDirectSound`,
/// an `IDirectSound` or 0 for one of the library's own). The game passes
/// `BinkOpenDirectSound` and 0 at start-up (`BGSMoviePlayer`, `008670e0`), whether or not a
/// movie plays. The answer is 1, as the player's DLL answers on this machine (measured
/// 2026-10-24, `nv-bink --sound-system`, with and without a window, `bink-sound-2026-10-24`;
/// the 2026-10-23 measurement that gave 0 had passed the DLL a NULL function). The layer
/// remembers that the DirectSound system is chosen: a movie with a sound track then plays it
/// through the platform's DirectSound (heard when the host turned the output on,
/// `super::dsound::start_output`). Measured with the DLL (`nv-bink --wait --sound --window
/// --hook-dsound`, docs/SOUND.md): it makes its own device (priority level), a looping 48 kHz
/// 16-bit stereo buffer of 115,712 bytes at volume, pan and frequency defaults, fills it whole
/// at the first `BinkDoFrame`, plays it, and refills what was played every two frames; the
/// frames then keep the clock schedule [`wait`] keeps, started when the sound starts. Here the
/// whole decoded track is one voice of the mixer started at the first `BinkDoFrame`, the
/// moment the schedule starts: the same samples on the same schedule. (When the DLL's sound
/// starts after its fill varies from run to run with the device, 50 to 540 ms here; that
/// delay is not reproduced.) Any other sound system is not provided.
pub fn set_sound_system(e: &mut Engine, open: u32, param: u32) -> i32 {
    assert!(
        open == OPEN_DIRECT_SOUND_SLOT && param == 0,
        "BinkSetSoundSystem({open:#x}, {param:#x}) is not provided (only BinkOpenDirectSound, 0)"
    );
    engine_id(e);
    e.mem.set_u32(STATE_PAGE + SOUND_SYSTEM, 1);
    1
}

// Platform: replaces binkw32.dll _BinkOpenDirectSound@4 (import)
/// `BinkOpenDirectSound(param)`: the DirectSound system's open function, which the library
/// itself calls from `BinkSetSoundSystem`. The game only passes its address on; a direct call
/// is not provided.
pub fn open_direct_sound(_e: &mut Engine, param: u32) -> u32 {
    panic!("BinkOpenDirectSound({param:#x}) called directly is not provided")
}

// Platform: replaces binkw32.dll _BinkOpen@8 (import)
/// `BinkOpen(name, flags)`: opens a movie and answers its handle, or 0 when it cannot (no such
/// file: the documented failure). The file is read from the player's install (read only,
/// through [`super::files`]); a relative name is in the current directory, as Windows opens
/// it. The handle's fields are those listed at [`WIDTH`]; `FrameNum` is 1. Flags other than
/// [`BINKPRELOADALL`] and [`BINKNOTHREADEDIO`], a file the decoder cannot read, a revision
/// other than `i` or video flags other than 0 (the only measured movie: the game's intro)
/// are not provided.
pub fn open(e: &mut Engine, name: Ptr, flags: u32) -> u32 {
    let text = e.mem.cstr(name.addr());
    let shown = String::from_utf8_lossy(&text).into_owned();
    assert!(
        flags & !(BINKPRELOADALL | BINKNOTHREADEDIO) == 0,
        "BinkOpen({shown:?}, {flags:#x}): these flags are not provided"
    );
    let path = super::files::full_path(e, &text);
    let Some(data) = super::files::read_whole(e, &path) else {
        return 0;
    };
    engine_id(e);
    let sound = e.mem.u32(STATE_PAGE + SOUND_SYSTEM) == 1;
    let (handle_fields, decoder, track) = {
        let movie = ::bink::Movie::parse(&data)
            .unwrap_or_else(|err| panic!("BinkOpen({shown:?}): {err:?} is not provided"));
        let track = if sound && !movie.audio.is_empty() {
            Some(sound_track(&movie, &shown))
        } else {
            None
        };
        assert!(
            movie.revision == b'i' && movie.video_flags == 0,
            "BinkOpen({shown:?}): revision {:?} with video flags {:#x} is not provided",
            movie.revision as char,
            movie.video_flags
        );
        let size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]);
        (
            [
                movie.width,
                movie.height,
                movie.frame_count(),
                1,
                0xffff_ffff,
                movie.fps_num,
                movie.fps_den,
                0,
                flags,
                0,
                size,
            ],
            ::bink::Decoder::new(movie.width, movie.height, movie.revision, movie.video_flags),
            track,
        )
    };
    // The handle follows the layer's own word (unused: kept so the handle is never the
    // block's start, which only this layer frees).
    let block = e.mem.alloc(4 + HANDLE_BYTES);
    let bink = block + 4;
    for (i, v) in handle_fields.iter().enumerate() {
        e.mem.set_u32(bink + 4 * i as u32, *v);
    }
    let movie = Playing {
        data,
        decoder,
        decoded: None,
        pixels: None,
        wrapped: false,
        rate: handle_fields[5],
        rate_div: handle_fields[6],
        start: None,
        base: 0,
        paused: None,
        waiting: (0, 0),
        track,
    };
    put(e, bink, movie);
    bink
}

/// The movie's sound track decoded whole (`crates/bink`'s audio decoder; within one step of
/// the DLL's, docs/MOVIES.md): 16-bit samples, channels, rate. More than one track is not
/// provided.
fn sound_track(movie: &::bink::Movie<'_>, shown: &str) -> (Vec<u8>, u16, u32) {
    assert!(
        movie.audio.len() == 1,
        "BinkOpen({shown:?}): {} sound tracks are not provided",
        movie.audio.len()
    );
    let t = &movie.audio[0];
    let channels = t.channels();
    let rate = u32::from(t.sample_rate);
    let mut decoder = ::bink::AudioDecoder::new(rate, channels, t.flags)
        .unwrap_or_else(|err| panic!("BinkOpen({shown:?}): sound track {err:?} is not provided"));
    let mut samples = Vec::new();
    for i in 0..movie.frame_count() {
        let p = movie
            .packet(i)
            .unwrap_or_else(|err| panic!("BinkOpen({shown:?}): frame {i}: {err:?}"));
        decoder
            .decode_packet(p.audio[0], &mut samples)
            .unwrap_or_else(|err| panic!("BinkOpen({shown:?}): frame {i} sound: {err:?}"));
    }
    let pcm = samples.iter().flat_map(|s| s.to_le_bytes()).collect();
    (pcm, channels, rate)
}

// Platform: replaces binkw32.dll _BinkClose@4 (import)
/// `BinkClose(bink)`: closes the movie and frees its handle.
pub fn close(e: &mut Engine, bink: u32) {
    drop(take(e, bink, "BinkClose"));
    super::dsound::stop_stream(e, bink);
    e.mem.free(bink - 4);
}

/// The frame index `FrameNum` names (0-based).
fn current(e: &Engine, bink: u32) -> u32 {
    e.mem.u32(bink + FRAME_NUM).wrapping_sub(1)
}

// Platform: replaces binkw32.dll _BinkDoFrame@4 (import)
/// `BinkDoFrame(bink)`: decodes the frame `FrameNum` names and answers 0 (the DLL answered 0
/// for every frame of the intro). `LastFrameNum` becomes `FrameNum`. The first one starts
/// the schedule [`wait`] answers by (measured: a pause after `BinkWait` and before it moves
/// the whole schedule; a pause after it makes the next frame late). Frames are decoded in
/// order: decoding one again, out of order, or after the movie went past its end is not
/// provided, nor is a frame the decoder cannot read.
pub fn do_frame(e: &mut Engine, bink: u32) -> i32 {
    let mut movie = take(e, bink, "BinkDoFrame");
    let index = current(e, bink);
    let expected = movie.decoded.map_or(0, |d| d + 1);
    assert!(
        !movie.wrapped && index == expected,
        "BinkDoFrame on frame index {index} (decoded last: {:?}, past the end: {}) is not \
         provided",
        movie.decoded,
        movie.wrapped
    );
    {
        let parsed = ::bink::Movie::parse(&movie.data).expect("parsed at BinkOpen");
        let packet = parsed
            .packet(index)
            .unwrap_or_else(|err| panic!("BinkDoFrame on frame index {index}: {err:?}"));
        movie
            .decoder
            .decode(packet.video)
            .unwrap_or_else(|err| panic!("BinkDoFrame on frame index {index}: {err:?}"));
    }
    movie.decoded = Some(index);
    movie.pixels = None;
    if movie.start.is_none() {
        movie.start = Some(e.clock_now_ms());
        movie.base = index;
        if let Some((pcm, channels, rate)) = movie.track.take() {
            super::dsound::note(
                e,
                &format!("movie {bink:08x}: sound track, {rate} Hz, {channels} ch"),
            );
            super::dsound::play_stream(e, bink, pcm, channels, 16, rate);
        }
    }
    let num = e.mem.u32(bink + FRAME_NUM);
    e.mem.set_u32(bink + LAST_FRAME_NUM, num);
    put(e, bink, movie);
    0
}

// Platform: replaces binkw32.dll _BinkNextFrame@4 (import)
/// `BinkNextFrame(bink)`: moves `FrameNum` on to the next frame; after the last frame it is 1
/// again (measured). Moving on from a frame that was not decoded is not provided.
pub fn next_frame(e: &mut Engine, bink: u32) {
    let mut movie = take(e, bink, "BinkNextFrame");
    let index = current(e, bink);
    assert!(
        !movie.wrapped && movie.decoded == Some(index),
        "BinkNextFrame on frame index {index}, which was not decoded, is not provided"
    );
    let frames = e.mem.u32(bink + FRAMES);
    let num = e.mem.u32(bink + FRAME_NUM);
    if num >= frames {
        e.mem.set_u32(bink + FRAME_NUM, 1);
        movie.wrapped = true;
    } else {
        e.mem.set_u32(bink + FRAME_NUM, num + 1);
    }
    put(e, bink, movie);
}

/// When the frame `index` is due, in milliseconds after the schedule's start: `(index -
/// base) * 1000 * FrameRateDiv / FrameRate`, rounded down.
fn due(movie: &Playing, index: u32) -> u64 {
    let frames = index
        .checked_sub(movie.base)
        .unwrap_or_else(|| panic!("BinkWait on frame index {index} before the schedule's start"));
    u64::from(frames) * 1000 * u64::from(movie.rate_div) / u64::from(movie.rate)
}

// Platform: replaces binkw32.dll _BinkWait@4 (import)
/// `BinkWait(bink)`: 1 while the frame `FrameNum` names is not due yet, 0 once it is, against
/// the platform clock (`Engine::clock_now_ms`, which `GetTickCount`, `timeGetTime` and
/// `QueryPerformanceCounter` answer). Measured with the DLL timing the intro without sound
/// (`bink-wait*-2026-10-23`, ticks of a 1 ms clock):
/// - before the first `BinkDoFrame` it answers 0 (also 500 ms after `BinkOpen`);
/// - the first `BinkDoFrame` starts the schedule: frame `n` is due `floor(n * 1000 *
///   FrameRateDiv / FrameRate)` ms after it (33, 66, 99, 133, 166, 199, ... for the intro);
/// - a frame waited for late keeps the schedule (the frames due meanwhile are answered 0 at
///   once) unless it is more than two frames late: when the time since the start is past the
///   due time of the frame two after it (`> due(n + 2)`), the schedule starts again at that
///   moment with that frame as its frame 1, so the next frame is due two frame times later
///   (66 ms for the intro). Measured to within a millisecond of the boundary.
/// - while paused ([`pause`]) it answers 1.
///
/// Waiting on a movie past its end is not provided. The run stops after
/// [`WAITS_WITHOUT_TIME`] answers of 1 in a row at the same clock (no time passes for it).
pub fn wait(e: &mut Engine, bink: u32) -> i32 {
    let mut movie = take(e, bink, "BinkWait");
    let index = current(e, bink);
    assert!(
        !movie.wrapped,
        "BinkWait on a movie past its last frame is not provided"
    );
    let now = e.clock_now_ms();
    let answer = match (movie.paused, movie.start) {
        (Some(_), _) => 1,
        (None, None) => 0,
        (None, Some(start)) => {
            assert!(
                movie.rate != 0,
                "BinkWait on a movie with a frame rate of 0 is not provided"
            );
            let elapsed = u64::from(now.wrapping_sub(start));
            if elapsed < due(&movie, index) {
                1
            } else {
                if elapsed > due(&movie, index + 2) {
                    movie.start = Some(now);
                    movie.base = index.checked_sub(1).unwrap_or_else(|| {
                        panic!("BinkWait late on frame index 0 is not provided")
                    });
                }
                0
            }
        }
    };
    if answer == 1 {
        movie.waiting = match movie.waiting {
            (clock, n) if clock == now && n > 0 => (now, n + 1),
            _ => (now, 1),
        };
        assert!(
            movie.waiting.1 < WAITS_WITHOUT_TIME,
            "BinkWait: {WAITS_WITHOUT_TIME} waits in a row at clock {now} ms; the platform \
             clock does not move while the game waits on the movie"
        );
    } else {
        movie.waiting = (0, 0);
    }
    put(e, bink, movie);
    answer
}

// Platform: replaces binkw32.dll _BinkPause@8 (import)
/// `BinkPause(bink, pause)`: pauses (non-zero) or resumes (0) the movie and answers the new
/// state (1 or 0, measured). Resuming moves the schedule on by the time it was paused
/// (measured: 10, 200 and 500 ms pauses). Pausing a paused movie or resuming a running one is
/// not provided.
pub fn pause(e: &mut Engine, bink: u32, pause: i32) -> i32 {
    let mut movie = take(e, bink, "BinkPause");
    let now = e.clock_now_ms();
    let answer = if pause != 0 {
        assert!(
            movie.paused.is_none(),
            "BinkPause(1) on a paused movie is not provided"
        );
        movie.paused = Some(now);
        1
    } else {
        let since = movie
            .paused
            .take()
            .unwrap_or_else(|| panic!("BinkPause(0) on a running movie is not provided"));
        if let Some(start) = movie.start {
            movie.start = Some(start.wrapping_add(now.wrapping_sub(since)));
        }
        0
    };
    put(e, bink, movie);
    super::dsound::pause_stream(e, bink, pause != 0);
    answer
}

// Platform: replaces binkw32.dll _BinkCopyToBufferRect@44 (import)
/// `BinkCopyToBufferRect(bink, dest, pitch, dest_height, dest_x, dest_y, src_x, src_y, src_w,
/// src_h, flags)`: copies the source rectangle of the decoded frame to `dest` (rows `pitch`
/// bytes apart) and answers 0 (the DLL answered 0 for every copy, also with the frame late).
/// With surface type 3 each pixel is blue, green, red and 0, the colours of the DLL's
/// 32-bit blitter (`Decoder::to_bgrx`). Rows below the movie's bottom are not written
/// (measured: the intro's last tile row asks for 256 rows from row 512 and gets 208). With
/// `BINKCOPYALL` or without, the whole rectangle is written: into the game's textures, which
/// keep the previous frame, both give the same pixels for all 8,692 frames of the intro
/// (measured). Other surface types or flags, a destination position other than 0, 0, a
/// rectangle past the right edge or starting below the bottom, a pitch too small for it, a
/// destination height below the rows written, and a frame that was not decoded are not
/// provided.
pub fn copy_to_buffer_rect(
    e: &mut Engine,
    bink: u32,
    dest: Ptr,
    pitch: i32,
    dest_height: u32,
    dest_x: u32,
    dest_y: u32,
    src_x: u32,
    src_y: u32,
    src_w: u32,
    src_h: u32,
    flags: u32,
) -> i32 {
    let mut movie = take(e, bink, "BinkCopyToBufferRect");
    let call = format!(
        "BinkCopyToBufferRect(dest {:#x}, pitch {pitch}, height {dest_height}, at {dest_x},{dest_y}, \
         source {src_x},{src_y} {src_w}x{src_h}, flags {flags:#x})",
        dest.addr()
    );
    assert!(
        flags & BINKSURFACEMASK == BINKSURFACE32 && flags & !(BINKSURFACEMASK | BINKCOPYALL) == 0,
        "{call}: these flags are not provided"
    );
    let width = e.mem.u32(bink + WIDTH);
    let height = e.mem.u32(bink + HEIGHT);
    let index = current(e, bink);
    assert!(
        !movie.wrapped && movie.decoded == Some(index),
        "{call}: frame index {index} was not decoded; not provided"
    );
    assert!(
        dest_x == 0 && dest_y == 0,
        "{call}: a destination position is not provided"
    );
    assert!(
        u64::from(src_x) + u64::from(src_w) <= u64::from(width) && src_y < height,
        "{call}: a rectangle outside the {width}x{height} movie is not provided"
    );
    let rows = src_h.min(height - src_y);
    assert!(
        pitch > 0 && pitch as u64 >= u64::from(src_w) * 4 && rows <= dest_height,
        "{call}: this pitch or destination height is not provided"
    );
    if movie.pixels.is_none() {
        let mut out = Vec::new();
        movie.decoder.to_bgrx(&mut out);
        movie.pixels = Some(out);
    }
    let pixels = movie.pixels.as_deref().unwrap_or_default();
    for row in 0..rows {
        let from = (((src_y + row) * width + src_x) * 4) as usize;
        let to = dest.addr().wrapping_add(row.wrapping_mul(pitch as u32));
        e.mem.write(to, &pixels[from..from + (src_w * 4) as usize]);
    }
    put(e, bink, movie);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A Bink 1 file (revision `i`) of `width` x `height`, `frames` frames at 30000/1001
    /// frames per second, no audio; each frame's video data is `video(index)`.
    fn movie_bytes(
        width: u32,
        height: u32,
        frames: u32,
        video: &dyn Fn(u32) -> Vec<u8>,
    ) -> Vec<u8> {
        let packets: Vec<Vec<u8>> = (0..frames).map(video).collect();
        let largest = packets.iter().map(Vec::len).max().unwrap_or(0) as u32;
        let mut d = Vec::new();
        d.extend_from_slice(b"BIKi");
        d.extend_from_slice(&0u32.to_le_bytes());
        for v in [frames, largest, frames, width, height, 30000, 1001, 0, 0] {
            d.extend_from_slice(&v.to_le_bytes());
        }
        let mut o = d.len() + 4 * (frames as usize + 1);
        for (i, p) in packets.iter().enumerate() {
            d.extend_from_slice(&((o as u32) | u32::from(i == 0)).to_le_bytes());
            o += p.len();
        }
        d.extend_from_slice(&(o as u32).to_le_bytes());
        for p in &packets {
            d.extend_from_slice(p);
        }
        let size = (d.len() - 8) as u32;
        d[4..8].copy_from_slice(&size.to_le_bytes());
        d
    }

    /// [`movie_bytes`] with one sound track (`rate`, `flags`; each frame's audio packet empty).
    fn movie_with_track(frames: u32, rate: u16, flags: u16) -> Vec<u8> {
        let packets: Vec<Vec<u8>> = (0..frames)
            .map(|f| [vec![0u8; 4], zeros(f)].concat())
            .collect();
        let mut d = b"BIKi".to_vec();
        d.extend_from_slice(&0u32.to_le_bytes());
        for v in [frames, 4100, frames, 64, 48, 30000, 1001, 0, 1, 0] {
            d.extend_from_slice(&v.to_le_bytes());
        }
        d.extend_from_slice(&rate.to_le_bytes());
        d.extend_from_slice(&flags.to_le_bytes());
        d.extend_from_slice(&0u32.to_le_bytes());
        let mut o = d.len() + 4 * (frames as usize + 1);
        for (i, p) in packets.iter().enumerate() {
            d.extend_from_slice(&((o as u32) | u32::from(i == 0)).to_le_bytes());
            o += p.len();
        }
        d.extend_from_slice(&(o as u32).to_le_bytes());
        for p in &packets {
            d.extend_from_slice(p);
        }
        let size = (d.len() - 8) as u32;
        d[4..8].copy_from_slice(&size.to_le_bytes());
        d
    }

    /// Video data that decodes: 4 KiB of zeros (every bundle empty, every block a skip).
    fn zeros(_: u32) -> Vec<u8> {
        vec![0; 4096]
    }

    const PATH: &[u8] = b"C:\\Game\\Data\\Video\\Test.bik";

    /// An engine with `bytes` as `PATH` among the run's files, and its name in memory.
    fn engine(bytes: Vec<u8>) -> (Engine, Ptr) {
        let mut e = Engine::new();
        e.kernel.current_directory = b"C:\\Game".to_vec();
        e.kernel.disk.files.insert(PATH.to_ascii_lowercase(), bytes);
        let name = e.mem.alloc(64);
        e.mem.set_cstr(name, b"Data\\Video\\Test.bik");
        (e, Ptr::new(name))
    }

    fn frame(e: &mut Engine, bink: u32) {
        do_frame(e, bink);
        next_frame(e, bink);
    }

    #[test]
    fn open_fills_the_handle_as_the_dll_does() {
        let bytes = movie_bytes(64, 48, 3, &zeros);
        let size = (bytes.len() - 8) as u32;
        let (mut e, name) = engine(bytes);
        let bink = open(&mut e, name, BINKPRELOADALL | BINKNOTHREADEDIO);
        assert_ne!(bink, 0);
        let words: Vec<u32> = (0..11).map(|i| e.mem.u32(bink + 4 * i)).collect();
        assert_eq!(
            words,
            [
                64,
                48,
                3,
                1,
                0xffff_ffff,
                30000,
                1001,
                0,
                0x0800_2000,
                0,
                size
            ]
        );
        close(&mut e, bink);
    }

    #[test]
    fn a_missing_file_answers_0() {
        let (mut e, _) = engine(movie_bytes(64, 48, 1, &zeros));
        let name = e.mem.alloc(16);
        e.mem.set_cstr(name, b"Nothing.bik");
        assert_eq!(open(&mut e, Ptr::new(name), 0), 0);
    }

    #[test]
    #[should_panic(expected = "these flags are not provided")]
    fn other_open_flags_stop() {
        let (mut e, name) = engine(movie_bytes(64, 48, 1, &zeros));
        open(&mut e, name, 0x4000);
    }

    #[test]
    fn frames_count_on_and_start_again_after_the_last() {
        let (mut e, name) = engine(movie_bytes(64, 48, 2, &zeros));
        let bink = open(&mut e, name, 0);
        assert_eq!(do_frame(&mut e, bink), 0);
        assert_eq!(e.mem.u32(bink + LAST_FRAME_NUM), 1);
        next_frame(&mut e, bink);
        assert_eq!(e.mem.u32(bink + FRAME_NUM), 2);
        do_frame(&mut e, bink);
        assert_eq!(e.mem.u32(bink + LAST_FRAME_NUM), 2);
        next_frame(&mut e, bink);
        assert_eq!(e.mem.u32(bink + FRAME_NUM), 1);
        assert_eq!(e.mem.u32(bink + LAST_FRAME_NUM), 2);
        close(&mut e, bink);
    }

    #[test]
    #[should_panic(expected = "is not provided")]
    fn decoding_a_frame_twice_stops() {
        let (mut e, name) = engine(movie_bytes(64, 48, 2, &zeros));
        let bink = open(&mut e, name, 0);
        do_frame(&mut e, bink);
        do_frame(&mut e, bink);
    }

    /// The intro's rate: 10000000 / 333333.
    fn intro_rate(e: &mut Engine, bink: u32) {
        let mut movie = take(e, bink, "test");
        movie.rate = 10_000_000;
        movie.rate_div = 333_333;
        put(e, bink, movie);
    }

    /// Steps the clock until `BinkWait` answers 0 and answers that clock.
    fn due_at(e: &mut Engine, bink: u32) -> u32 {
        while wait(e, bink) != 0 {
            e.clock_ms += 1;
        }
        e.clock_ms
    }

    #[test]
    fn frames_are_due_on_the_measured_schedule() {
        let (mut e, name) = engine(movie_bytes(64, 48, 8, &zeros));
        let bink = open(&mut e, name, 0);
        intro_rate(&mut e, bink);
        e.clock_ms = 5000;
        // Frame 0 at once, however long after the open.
        assert_eq!(wait(&mut e, bink), 0);
        frame(&mut e, bink);
        let mut times = Vec::new();
        for _ in 1..7 {
            times.push(due_at(&mut e, bink) - 5000);
            frame(&mut e, bink);
        }
        assert_eq!(times, [33, 66, 99, 133, 166, 199]);
    }

    #[test]
    fn a_late_frame_keeps_the_schedule_up_to_two_frames() {
        let (mut e, name) = engine(movie_bytes(64, 48, 16, &zeros));
        let bink = open(&mut e, name, 0);
        intro_rate(&mut e, bink);
        frame(&mut e, bink);
        for _ in 1..10 {
            due_at(&mut e, bink);
            frame(&mut e, bink);
        }
        // Frame 10 is due at 333; waited for at 399 (= due(12)) it keeps the schedule:
        // frames 11 and 12 are due at once, 13 at 433.
        e.clock_ms = 399;
        assert_eq!(wait(&mut e, bink), 0);
        frame(&mut e, bink);
        assert_eq!(wait(&mut e, bink), 0);
        frame(&mut e, bink);
        assert_eq!(wait(&mut e, bink), 0);
        frame(&mut e, bink);
        assert_eq!(due_at(&mut e, bink), 433);
    }

    #[test]
    fn a_frame_more_than_two_frames_late_starts_the_schedule_again() {
        let (mut e, name) = engine(movie_bytes(64, 48, 16, &zeros));
        let bink = open(&mut e, name, 0);
        intro_rate(&mut e, bink);
        frame(&mut e, bink);
        for _ in 1..10 {
            due_at(&mut e, bink);
            frame(&mut e, bink);
        }
        // Waited for at 450: the schedule starts again with frame 10 as its frame 1, so the
        // next frames are due 66, 99 and 133 ms later (measured with a 150 ms delay).
        e.clock_ms = 450;
        assert_eq!(wait(&mut e, bink), 0);
        frame(&mut e, bink);
        let mut times = Vec::new();
        for _ in 0..3 {
            times.push(due_at(&mut e, bink) - 450);
            frame(&mut e, bink);
        }
        assert_eq!(times, [66, 99, 133]);
    }

    #[test]
    fn a_pause_moves_the_schedule_on() {
        let (mut e, name) = engine(movie_bytes(64, 48, 4, &zeros));
        let bink = open(&mut e, name, 0);
        intro_rate(&mut e, bink);
        frame(&mut e, bink);
        e.clock_ms = 10;
        assert_eq!(pause(&mut e, bink, 1), 1);
        e.clock_ms = 100;
        assert_eq!(wait(&mut e, bink), 1);
        e.clock_ms = 210;
        assert_eq!(pause(&mut e, bink, 0), 0);
        assert_eq!(due_at(&mut e, bink), 233);
    }

    #[test]
    #[should_panic(expected = "the platform clock does not move")]
    fn waiting_without_time_stops() {
        let (mut e, name) = engine(movie_bytes(64, 48, 4, &zeros));
        let bink = open(&mut e, name, 0);
        frame(&mut e, bink);
        loop {
            wait(&mut e, bink);
        }
    }

    /// Frame 0 of `bytes` as the decoder gives it in 32 bits (what a copy must write).
    fn decoded_frame(bytes: &[u8]) -> Vec<u8> {
        let m = ::bink::Movie::parse(bytes).unwrap();
        let mut d = ::bink::Decoder::new(m.width, m.height, m.revision, m.video_flags);
        d.decode(m.packet(0).unwrap().video).unwrap();
        let mut out = Vec::new();
        d.to_bgrx(&mut out);
        out
    }

    #[test]
    fn copies_a_tile_and_stops_at_the_bottom() {
        let bytes = movie_bytes(64, 48, 1, &zeros);
        let frame0 = decoded_frame(&bytes);
        let (mut e, name) = engine(bytes);
        let bink = open(&mut e, name, 0);
        do_frame(&mut e, bink);
        // A 32x32 tile at 32, 32 of the 64x48 movie: 16 rows are picture.
        let tile = e.mem.alloc(32 * 128);
        e.mem.write(tile, &vec![0xee; 32 * 128]);
        let r = copy_to_buffer_rect(
            &mut e,
            bink,
            Ptr::new(tile),
            128,
            48,
            0,
            0,
            32,
            32,
            32,
            32,
            3,
        );
        assert_eq!(r, 0);
        for row in 0..32u32 {
            let got = e.mem.bytes(tile + row * 128, 128);
            if row < 16 {
                let from = (((32 + row) * 64 + 32) * 4) as usize;
                assert_eq!(&got[..], &frame0[from..from + 128], "row {row}");
                assert!(got.iter().skip(3).step_by(4).all(|&x| x == 0));
            } else {
                assert!(got.iter().all(|&b| b == 0xee), "row {row}");
            }
        }
        // BINKCOPYALL writes the same.
        let again = e.mem.alloc(32 * 128);
        copy_to_buffer_rect(
            &mut e,
            bink,
            Ptr::new(again),
            128,
            48,
            0,
            0,
            32,
            32,
            32,
            32,
            BINKCOPYALL | 3,
        );
        assert_eq!(e.mem.bytes(again, 16 * 128), e.mem.bytes(tile, 16 * 128));
        close(&mut e, bink);
    }

    #[test]
    fn each_row_comes_from_its_place_in_the_frame() {
        let (mut e, name) = engine(movie_bytes(64, 48, 1, &zeros));
        let bink = open(&mut e, name, 0);
        do_frame(&mut e, bink);
        // A frame whose every byte differs from its neighbours' (the decoder's own zero frame
        // is one colour), put where the copy takes the frame's pixels from.
        let pattern: Vec<u8> = (0..64 * 48 * 4u32)
            .map(|i| (i * 7 + i / 251) as u8)
            .collect();
        let mut movie = take(&mut e, bink, "test");
        movie.pixels = Some(pattern.clone());
        put(&mut e, bink, movie);
        // 16x40 from 48, 8 with a pitch of 100: 40 rows, all picture.
        let tile = e.mem.alloc(40 * 100);
        copy_to_buffer_rect(
            &mut e,
            bink,
            Ptr::new(tile),
            100,
            40,
            0,
            0,
            48,
            8,
            16,
            40,
            3,
        );
        for row in 0..40u32 {
            let from = (((8 + row) * 64 + 48) * 4) as usize;
            assert_eq!(e.mem.bytes(tile + row * 100, 64), &pattern[from..from + 64]);
            assert_eq!(e.mem.bytes(tile + row * 100 + 64, 36), vec![0; 36]);
        }
        close(&mut e, bink);
    }

    #[test]
    #[should_panic(expected = "these flags are not provided")]
    fn other_surface_types_stop() {
        let (mut e, name) = engine(movie_bytes(64, 48, 1, &zeros));
        let bink = open(&mut e, name, 0);
        do_frame(&mut e, bink);
        let tile = e.mem.alloc(64 * 64 * 4);
        copy_to_buffer_rect(&mut e, bink, Ptr::new(tile), 256, 48, 0, 0, 0, 0, 64, 48, 4);
    }

    #[test]
    #[should_panic(expected = "a destination position is not provided")]
    fn a_destination_position_stops() {
        let (mut e, name) = engine(movie_bytes(64, 48, 1, &zeros));
        let bink = open(&mut e, name, 0);
        do_frame(&mut e, bink);
        let tile = e.mem.alloc(64 * 64 * 4);
        copy_to_buffer_rect(&mut e, bink, Ptr::new(tile), 256, 64, 1, 0, 0, 0, 32, 32, 3);
    }

    #[test]
    fn the_direct_sound_system_is_chosen_as_measured() {
        let mut e = Engine::new();
        assert_eq!(set_sound_system(&mut e, OPEN_DIRECT_SOUND_SLOT, 0), 1);
    }

    #[test]
    fn a_movie_with_a_sound_track_plays_with_the_sound_system_chosen() {
        // 48 kHz 16-bit stereo DCT, as the intro's; the output is off (the default), so nothing
        // is heard and the movie runs as without sound.
        let (mut e, name) = engine(movie_with_track(2, 48000, 0x7000));
        set_sound_system(&mut e, OPEN_DIRECT_SOUND_SLOT, 0);
        let bink = open(&mut e, name, 0);
        assert_ne!(bink, 0);
        do_frame(&mut e, bink);
        next_frame(&mut e, bink);
        assert_eq!(pause(&mut e, bink, 1), 1);
        assert_eq!(pause(&mut e, bink, 0), 0);
        close(&mut e, bink);
    }

    /// The frames of a movie with a sound track come due on the platform clock alone, as
    /// without sound, whether or not the output plays: nothing waits on an audio position
    /// (the DLL's frames with sound keep the clock schedule too, docs/SOUND.md).
    #[test]
    fn a_movie_with_a_sound_track_keeps_the_clock_schedule_without_output() {
        let (mut e, name) = engine(movie_with_track(2, 48000, 0x7000));
        set_sound_system(&mut e, OPEN_DIRECT_SOUND_SLOT, 0);
        let bink = open(&mut e, name, 0);
        intro_rate(&mut e, bink);
        e.clock_ms = 5000;
        assert_eq!(wait(&mut e, bink), 0);
        frame(&mut e, bink);
        assert_eq!(wait(&mut e, bink), 1);
        assert_eq!(due_at(&mut e, bink) - 5000, 33);
        close(&mut e, bink);
    }

    #[test]
    #[should_panic(expected = "sound track")]
    fn a_sound_track_the_decoder_cannot_read_stops() {
        // The test track has no DCT flag: the decoder does not provide it.
        let (mut e, name) = engine(movie_with_track(2, 22050, 0));
        set_sound_system(&mut e, OPEN_DIRECT_SOUND_SLOT, 0);
        open(&mut e, name, 0);
    }

    #[test]
    fn a_silent_movie_opens_with_the_sound_system_chosen() {
        let (mut e, name) = engine(movie_bytes(64, 48, 2, &zeros));
        set_sound_system(&mut e, OPEN_DIRECT_SOUND_SLOT, 0);
        assert_ne!(open(&mut e, name, 0), 0);
        let (mut e, name) = engine(movie_with_track(2, 22050, 0));
        assert_ne!(open(&mut e, name, 0), 0);
    }

    #[test]
    #[should_panic(expected = "is not provided")]
    fn another_sound_system_stops() {
        let mut e = Engine::new();
        set_sound_system(&mut e, 0x1234, 0);
    }

    #[test]
    fn the_imports_are_registered() {
        let table = super::super::imports();
        for name in [
            "_BinkSetSoundSystem@8",
            "_BinkClose@4",
            "_BinkNextFrame@4",
            "_BinkWait@4",
            "_BinkPause@8",
            "_BinkOpenDirectSound@4",
            "_BinkOpen@8",
            "_BinkDoFrame@4",
            "_BinkCopyToBufferRect@44",
        ] {
            assert!(
                table
                    .iter()
                    .any(|(dll, n, _)| dll.eq_ignore_ascii_case("binkw32.dll") && *n == name),
                "{name}"
            );
        }
    }
}
