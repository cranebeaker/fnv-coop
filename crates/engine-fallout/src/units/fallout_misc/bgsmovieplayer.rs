//! `fallout/misc/bgsmovieplayer.cpp` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00867180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores the vtable `01082564` (RTTI `.?AVBGSMoviePlayer@@`) at `+0` of `this` (`ECX`), then
/// hands `this` to `00ec15e0`. One word, plain `RET`, returns nothing.
pub fn fn_00867180(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0108_2564);
    e.call(0x00ec_15e0, &args![this]);
}

/// The renderer getter (`004bc3f0`, no arguments) and the renderer calls
/// the movie player forwards to: lock (`004a0370`), unlock (`004a03c0`) and
/// the device (`004dc020`), all `thiscall` on the renderer, and the
/// inside-frame state (`004e9530`, `thiscall`, 1 inside a frame).
const RENDERER: u32 = 0x004b_c3f0;
const RENDERER_LOCK: u32 = 0x004a_0370;
const RENDERER_UNLOCK: u32 = 0x004a_03c0;
const RENDERER_GET_DEVICE: u32 = 0x004d_c020;
const RENDERER_FRAME_STATE: u32 = 0x004e_9530;
/// The object at `011dea0c` (the process manager the movie player asks for
/// the main thread, `0044edb0`, and for the control map, `00877720`).
const MANAGER: u32 = 0x011d_ea0c;
const MANAGER_MAIN_THREAD: u32 = 0x0044_edb0;
const MANAGER_CONTROLS: u32 = 0x0087_7720;
/// The screen-size queries (no arguments, `renderer.cpp`): whether a
/// display mode is set (`004de070`, `004de080`, bytes), its width
/// (`004de0e0`) and height (`004de0f0`) as floats in ST0, and the fallback
/// width (`004dc1f0`) and height (`004dc200`) as words.
const MODE_SET: u32 = 0x004d_e070;
const MODE_VALID: u32 = 0x004d_e080;
const MODE_WIDTH: u32 = 0x004d_e0e0;
const MODE_HEIGHT: u32 = 0x004d_e0f0;
const FALLBACK_WIDTH: u32 = 0x004d_c1f0;
const FALLBACK_HEIGHT: u32 = 0x004d_c200;
/// The free `00401030` (`cdecl`) and the allocator `00401000` (`cdecl`).
const FREE: u32 = 0x0040_1030;
const ALLOCATE: u32 = 0x0040_1000;
/// The memory scope guard `Create` keeps on its stack (`00404eb0`,
/// `thiscall (guard, 0x14, 1, 01082518, 0x1C)`; `00404ee0` ends it), the
/// player's constructor (`008670e0`) and the global it fills (`0126fac4`).
const SCOPE_BEGIN: u32 = 0x0040_4eb0;
const SCOPE_END: u32 = 0x0040_4ee0;
const SCOPE_NAME: u32 = 0x0108_2518;
const PLAYER_CONSTRUCTOR: u32 = 0x0086_70e0;
const MOVIE_PLAYER: u32 = 0x0126_fac4;
/// The audio manager getter (`00453a70`, no arguments) and its calls: the
/// muted state (`005bb4d0`, a byte), the mute (`00ad7230`, one byte), the
/// pause (`007eb8a0`) and resume (`007ec380`), and `00ad7740` (one word);
/// the flag the movie player keeps when it muted the audio (`011de714`).
const AUDIO: u32 = 0x0045_3a70;
const AUDIO_IS_MUTED: u32 = 0x005b_b4d0;
const AUDIO_SET_MUTE: u32 = 0x00ad_7230;
const AUDIO_PAUSE: u32 = 0x007e_b8a0;
const AUDIO_RESUME: u32 = 0x007e_c380;
const AUDIO_00AD7740: u32 = 0x00ad_7740;
const MUTED_BY_MOVIE: u32 = 0x011d_e714;
/// The movie's own calls (`BSMoviePlayer`, `thiscall`): continue
/// (`00ec1840`, one byte), playing (`00ec17c0`) and stop (`00ec29b0`), the
/// frame-state getter (`0043c4b0`), the control map's refresh (`00a23010`)
/// and its key test (`00a24660`, `(control, 1)`, answers a word); the
/// device's slot +0xC (`TestCooperativeLevel`, a COM call).
const MOVIE_CONTINUE: u32 = 0x00ec_1840;
const MOVIE_IS_PLAYING: u32 = 0x00ec_17c0;
const MOVIE_STOP: u32 = 0x00ec_29b0;
const RENDERER_FOR_DEVICE: u32 = 0x0043_c4b0;
const CONTROLS_REFRESH: u32 = 0x00a2_3010;
const CONTROLS_PRESSED: u32 = 0x00a2_4660;
/// `BGSMoviePlayer`'s byte at +0x20 (a skippable flag the continue check
/// reads).
const SKIPPABLE: u32 = 0x20;

/// `FISTP qword` under a control word with truncation (the exe ORs 0xC00
/// into it around the store): the low word of the 64-bit integer, the
/// integer indefinite for NaN and values out of range.
fn truncate_to_word(value: f64) -> u32 {
    if value.is_nan()
        || value >= 9_223_372_036_854_775_808.0
        || value < -9_223_372_036_854_775_808.0
    {
        0
    } else {
        value.trunc() as i64 as u32
    }
}

// Translated from 00867150 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMoviePlayer::_scalar_deleting_destructor_` (Xbox PDB): the destructor
/// body (`fn_00867180`), then the free `00401030` when bit 0 of `flags` is
/// set. Answers `this`.
pub fn bgs_movie_player_scalar_deleting_destructor(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00867180(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 00866ff0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00866ff0 - the exception-unwinding frame (FS:[0]) is not translated.
/// `BGSMoviePlayer::Create` (Xbox PDB): inside the memory scope (`00404eb0`
/// / `00404ee0` on a stack guard) a new 0x54-byte player (`00401000`,
/// constructed by `008670e0` when allocated, else null) into the global at
/// `0126fac4`. The exception-unwinding frame is not translated.
pub fn bgs_movie_player_create(e: &mut Engine) {
    e.with_stack(4, |e, guard| {
        e.call(
            SCOPE_BEGIN,
            &args![guard, 0x14u32, 1u32, SCOPE_NAME, 0x1cu32],
        );
        let block = e.call(ALLOCATE, &args![0x54u32]).u32();
        let player = if block != 0 {
            e.call(PLAYER_CONSTRUCTOR, &args![block]).u32()
        } else {
            0
        };
        e.mem.set_u32(MOVIE_PLAYER, player);
        e.call(SCOPE_END, &args![guard]);
    });
}

// Translated from 008672b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMoviePlayer::LockRenderer` (Xbox PDB): locks the renderer
/// (`004a0370`).
pub fn bgs_movie_player_lock_renderer(e: &mut Engine, _this: Ptr) -> u32 {
    let renderer = e.call(RENDERER, &args![]).u32();
    e.call(RENDERER_LOCK, &args![renderer]).u32()
}

// Translated from 008672d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMoviePlayer::UnlockRenderer` (Xbox PDB): unlocks the renderer
/// (`004a03c0`).
pub fn bgs_movie_player_unlock_renderer(e: &mut Engine, _this: Ptr) -> u32 {
    let renderer = e.call(RENDERER, &args![]).u32();
    e.call(RENDERER_UNLOCK, &args![renderer]).u32()
}

// Translated from 00867290 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMoviePlayer::GetDevice` (Xbox PDB): the renderer's device
/// (`004dc020`).
pub fn bgs_movie_player_get_device(e: &mut Engine, _this: Ptr) -> u32 {
    let renderer = e.call(RENDERER, &args![]).u32();
    e.call(RENDERER_GET_DEVICE, &args![renderer]).u32()
}

// Translated from 008673e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMoviePlayer::IsRendererInsideFrame` (Xbox PDB): whether the
/// renderer's frame state (`004e9530`) is 1.
pub fn bgs_movie_player_is_renderer_inside_frame(e: &mut Engine, _this: Ptr) -> bool {
    let renderer = e.call(RENDERER, &args![]).u32();
    e.call(RENDERER_FRAME_STATE, &args![renderer]).u32() == 1
}

// Translated from 00867410 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMoviePlayer::GetMainThreadID` (Xbox PDB): `0044edb0` of the object
/// at `011dea0c`, 0 without one.
pub fn bgs_movie_player_get_main_thread_id(e: &mut Engine, _this: Ptr) -> u32 {
    let manager = e.mem.u32(MANAGER);
    if manager == 0 {
        return 0;
    }
    e.call(MANAGER_MAIN_THREAD, &args![manager]).u32()
}

/// The screen size the movie player answers: the display mode's (`size`,
/// truncated) when one is set and valid, else `fallback()` (0 when there is
/// none).
fn screen_size(e: &mut Engine, size: u32, fallback: Option<u32>) -> u32 {
    if e.call(MODE_SET, &args![]).u8() != 0 && e.call(MODE_VALID, &args![]).u8() != 0 {
        let value = e.call(size, &args![]).f64();
        return truncate_to_word(value);
    }
    match fallback {
        Some(function) => e.call(function, &args![]).u32(),
        None => 0,
    }
}

// Translated from 008672f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMoviePlayer::GetScreenWidth` (Xbox PDB): the display mode's width
/// (`004de0e0`, truncated) when a mode is set and valid (`004de070`,
/// `004de080`), else `004dc1f0`.
pub fn bgs_movie_player_get_screen_width(e: &mut Engine, _this: Ptr) -> u32 {
    screen_size(e, MODE_WIDTH, Some(FALLBACK_WIDTH))
}

// Translated from 00867340 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMoviePlayer::GetScreenHeight` (Xbox PDB): the display mode's height
/// (`004de0f0`, truncated) when a mode is set and valid, else `004dc200`.
pub fn bgs_movie_player_get_screen_height(e: &mut Engine, _this: Ptr) -> u32 {
    screen_size(e, MODE_HEIGHT, Some(FALLBACK_HEIGHT))
}

// Translated from 00867390 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: the display mode's width (`004de0e0`,
/// truncated) when a mode is set and valid, else 0.
pub fn fn_00867390(e: &mut Engine, _this: Ptr) -> u32 {
    screen_size(e, MODE_WIDTH, None)
}

// Translated from 008671a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMoviePlayer::MuteAudio` (Xbox PDB): when the audio is not muted, mutes
/// it (`00ad7230` with 1) and remembers that; pauses (`007eb8a0`); and when
/// it is still not muted, forgets it and calls `00ad7740` with 1 twice.
pub fn bgs_movie_player_mute_audio(e: &mut Engine, _this: Ptr) {
    let audio = e.call(AUDIO, &args![]).u32();
    if e.call(AUDIO_IS_MUTED, &args![audio]).u8() == 0 {
        let audio = e.call(AUDIO, &args![]).u32();
        e.call(AUDIO_SET_MUTE, &args![audio, 1u32]);
        e.mem.set_u8(MUTED_BY_MOVIE, 1);
    }
    let audio = e.call(AUDIO, &args![]).u32();
    e.call(AUDIO_PAUSE, &args![audio]);
    let audio = e.call(AUDIO, &args![]).u32();
    if e.call(AUDIO_IS_MUTED, &args![audio]).u8() == 0 {
        e.mem.set_u8(MUTED_BY_MOVIE, 0);
        for _ in 0..2 {
            let audio = e.call(AUDIO, &args![]).u32();
            e.call(AUDIO_00AD7740, &args![audio, 1u32]);
        }
    }
}

// Translated from 00867220 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMoviePlayer::UnmuteAudio` (Xbox PDB): when the movie muted the audio
/// and it is muted, unmutes it (`00ad7230` with 0) and forgets that; then
/// resumes (`007ec380`).
pub fn bgs_movie_player_unmute_audio(e: &mut Engine, _this: Ptr) {
    if e.mem.u8(MUTED_BY_MOVIE) != 0 {
        let audio = e.call(AUDIO, &args![]).u32();
        if e.call(AUDIO_IS_MUTED, &args![audio]).u8() != 0 {
            let audio = e.call(AUDIO, &args![]).u32();
            e.call(AUDIO_SET_MUTE, &args![audio, 0u32]);
            e.mem.set_u8(MUTED_BY_MOVIE, 0);
        }
    }
    let audio = e.call(AUDIO, &args![]).u32();
    e.call(AUDIO_RESUME, &args![audio]);
}

// Translated from 00867440 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMoviePlayer::CheckContinuePlaying` (Xbox PDB): false when the movie
/// does not continue (`00ec1840` with `skip`) or the device is lost (its
/// `TestCooperativeLevel`, vtable +0xC, negative). Otherwise, when `skip`
/// is set or the movie is skippable (+0x20), the controls are refreshed
/// and, when control 5 or 0x1C is pressed (`00a24660` with 1), the movie
/// is stopped (when playing), the controls refreshed again and the answer
/// is false; true otherwise.
pub fn bgs_movie_player_check_continue_playing(e: &mut Engine, this: Ptr, skip: u8) -> bool {
    if e.call(MOVIE_CONTINUE, &args![this, u32::from(skip)]).u8() == 0 {
        return false;
    }
    let renderer = e.call(RENDERER_FOR_DEVICE, &args![]).u32();
    let device = e.call(RENDERER_GET_DEVICE, &args![renderer]).u32();
    if (e.vcall(device, 0xc, &args![]).u32() as i32) < 0 {
        return false;
    }
    if skip == 0 && e.mem.u8(this.addr().wrapping_add(SKIPPABLE)) == 0 {
        return true;
    }
    let manager = e.mem.u32(MANAGER);
    let controls = e.call(MANAGER_CONTROLS, &args![manager]).u32();
    e.call(CONTROLS_REFRESH, &args![controls]);
    if e.call(CONTROLS_PRESSED, &args![controls, 5u32, 1u32]).u32() != 0
        || e.call(CONTROLS_PRESSED, &args![controls, 0x1cu32, 1u32])
            .u32()
            != 0
    {
        if e.call(MOVIE_IS_PLAYING, &args![this]).u8() != 0 {
            e.call(MOVIE_STOP, &args![this]);
        }
        e.call(CONTROLS_REFRESH, &args![controls]);
        return false;
    }
    true
}

/// `FalloutAudioMedia::MediaPause` (`008305e0`) and `MediaUnPause`
/// (`00830610`): static, no arguments.
const MEDIA_PAUSE: u32 = 0x0083_05e0;
const MEDIA_UNPAUSE: u32 = 0x0083_0610;

// Translated from 00867270 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMoviePlayer::PauseMusic` (Xbox PDB; `ECX` only, not read):
/// `FalloutAudioMedia::MediaPause`.
pub fn bgs_movie_player_pause_music(e: &mut Engine, _this: Ptr) {
    e.call(MEDIA_PAUSE, &args![]);
}

// Translated from 00867280 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BGSMoviePlayer::UnPauseMusic` (Xbox PDB; `ECX` only, not read):
/// `FalloutAudioMedia::MediaUnPause`.
pub fn bgs_movie_player_unpause_music(e: &mut Engine, _this: Ptr) {
    e.call(MEDIA_UNPAUSE, &args![]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0086_7270, bgs_movie_player_pause_music(Ptr)),
        entry!(0x0086_7280, bgs_movie_player_unpause_music(Ptr)),
        entry!(0x0086_7180, fn_00867180(Ptr)),
        entry!(0x00867150, bgs_movie_player_scalar_deleting_destructor(Ptr, u32) -> Ptr),
        entry!(0x00866ff0, bgs_movie_player_create()),
        entry!(0x008672b0, bgs_movie_player_lock_renderer(Ptr) -> u32),
        entry!(0x008672d0, bgs_movie_player_unlock_renderer(Ptr) -> u32),
        entry!(0x00867290, bgs_movie_player_get_device(Ptr) -> u32),
        entry!(0x008673e0, bgs_movie_player_is_renderer_inside_frame(Ptr) -> bool),
        entry!(0x00867410, bgs_movie_player_get_main_thread_id(Ptr) -> u32),
        entry!(0x008672f0, bgs_movie_player_get_screen_width(Ptr) -> u32),
        entry!(0x00867340, bgs_movie_player_get_screen_height(Ptr) -> u32),
        entry!(0x00867390, fn_00867390(Ptr) -> u32),
        entry!(0x008671a0, bgs_movie_player_mute_audio(Ptr)),
        entry!(0x00867220, bgs_movie_player_unmute_audio(Ptr)),
        entry!(0x00867440, bgs_movie_player_check_continue_playing(Ptr, u8) -> bool),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn fn_00867180_stores_the_vtable_then_forwards_this() {
        use crate::prelude::*;
        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.write(this, &[0xcd; 8]);
        // The double copies the vtable word it finds when it runs to `+4`.
        e.register(0x00ec_15e0, |e, a| {
            let seen = e.mem.u32(a[0]);
            e.mem.set_u32(a[0] + 4, seen);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        e.call(0x0086_7180, &args![Ptr::<()>::new(this)]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(addr, _)| *addr == 0x00ec_15e0)
            .collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1[0], this);
        assert_eq!(e.mem.u32(this), 0x0108_2564);
        assert_eq!(
            e.mem.u32(this + 4),
            0x0108_2564,
            "the vtable was stored before the call"
        );
    }
}

#[cfg(test)]
mod b014_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    /// The doubles' switches live at `0x7300_0000` (one word each).
    const STATE: u32 = 0x7300_0000;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(STATE, 0x40);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        e.register(RENDERER, |_, _| 0x4000u32.into_ret());
        e.register(AUDIO, |_, _| 0x5000u32.into_ret());
        e
    }

    #[test]
    fn scalar_deleting_destructor_frees_on_bit_0() {
        let mut e = engine();
        e.register(0x00ec_15e0, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        let this = e.mem.alloc(0x54);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00867150, &args![this, 0u32]).u32(), this);
        assert!(calls(&e, FREE).is_empty());
        assert_eq!(e.call(0x00867150, &args![this, 1u32]).u32(), this);
        assert_eq!(calls(&e, FREE), vec![vec![this]]);
        assert_eq!(e.mem.u32(this), 0x0108_2564);
    }

    #[test]
    fn create_makes_the_player_inside_the_scope() {
        for allocated in [true, false] {
            let mut e = engine();
            e.mem.set_u32(STATE, u32::from(allocated));
            e.register(SCOPE_BEGIN, |_, _| Ret::default());
            e.register(SCOPE_END, |_, _| Ret::default());
            e.register(ALLOCATE, |e, _| {
                if e.mem.u32(STATE) != 0 { 0x6000u32 } else { 0 }.into_ret()
            });
            e.register(PLAYER_CONSTRUCTOR, |_, a| (a[0] + 1).into_ret());
            e.call_log = Some(vec![]);
            e.call(0x00866ff0, &args![]);
            let begin = calls(&e, SCOPE_BEGIN);
            assert_eq!(&begin[0][1..], [0x14, 1, SCOPE_NAME, 0x1c]);
            assert_eq!(calls(&e, SCOPE_END), vec![vec![begin[0][0]]]);
            assert_eq!(calls(&e, ALLOCATE), vec![vec![0x54]]);
            let expected = if allocated { 0x6001 } else { 0 };
            assert_eq!(e.mem.u32(MOVIE_PLAYER), expected);
        }
    }

    #[test]
    fn renderer_forwarders() {
        let mut e = engine();
        e.register(RENDERER_LOCK, |_, a| (a[0] + 1).into_ret());
        e.register(RENDERER_UNLOCK, |_, a| (a[0] + 2).into_ret());
        e.register(RENDERER_GET_DEVICE, |_, a| (a[0] + 3).into_ret());
        e.register(RENDERER_FRAME_STATE, |e, _| e.mem.u32(STATE).into_ret());
        assert_eq!(e.call(0x008672b0, &args![0u32]).u32(), 0x4001);
        assert_eq!(e.call(0x008672d0, &args![0u32]).u32(), 0x4002);
        assert_eq!(e.call(0x00867290, &args![0u32]).u32(), 0x4003);
        for (state, inside) in [(0u32, false), (1, true), (2, false)] {
            e.mem.set_u32(STATE, state);
            assert_eq!(e.call(0x008673e0, &args![0u32]).bool(), inside);
        }
    }

    #[test]
    fn main_thread_id_needs_the_manager() {
        let mut e = engine();
        e.register(MANAGER_MAIN_THREAD, |_, a| (a[0] + 7).into_ret());
        assert_eq!(e.call(0x00867410, &args![0u32]).u32(), 0);
        e.mem.set_u32(MANAGER, 0x100);
        assert_eq!(e.call(0x00867410, &args![0u32]).u32(), 0x107);
    }

    #[test]
    fn screen_sizes_from_the_mode_or_the_fallback() {
        let mut e = engine();
        e.register(MODE_SET, |e, _| (e.mem.u32(STATE) & 1).into_ret());
        e.register(MODE_VALID, |e, _| ((e.mem.u32(STATE) >> 1) & 1).into_ret());
        e.register(MODE_WIDTH, |_, _| Ret {
            st0: 1279.9,
            ..Ret::default()
        });
        e.register(MODE_HEIGHT, |_, _| Ret {
            st0: 719.5,
            ..Ret::default()
        });
        e.register(FALLBACK_WIDTH, |_, _| 800u32.into_ret());
        e.register(FALLBACK_HEIGHT, |_, _| 600u32.into_ret());
        e.mem.set_u32(STATE, 3);
        assert_eq!(e.call(0x008672f0, &args![0u32]).u32(), 1279);
        assert_eq!(e.call(0x00867340, &args![0u32]).u32(), 719);
        assert_eq!(e.call(0x00867390, &args![0u32]).u32(), 1279);
        for state in [0u32, 1, 2] {
            e.mem.set_u32(STATE, state);
            assert_eq!(e.call(0x008672f0, &args![0u32]).u32(), 800);
            assert_eq!(e.call(0x00867340, &args![0u32]).u32(), 600);
            assert_eq!(e.call(0x00867390, &args![0u32]).u32(), 0);
        }
    }

    fn audio_engine(muted: bool) -> Engine {
        let mut e = engine();
        e.mem.set_u32(STATE, u32::from(muted));
        e.register(AUDIO_IS_MUTED, |e, _| e.mem.u32(STATE).into_ret());
        e.register(AUDIO_SET_MUTE, |e, a| {
            e.mem.set_u32(STATE, a[1]);
            Ret::default()
        });
        e.register(AUDIO_PAUSE, |_, _| Ret::default());
        e.register(AUDIO_RESUME, |_, _| Ret::default());
        e.register(AUDIO_00AD7740, |_, _| Ret::default());
        e
    }

    #[test]
    fn mute_audio_mutes_once_and_remembers_it() {
        let mut e = audio_engine(false);
        e.call_log = Some(vec![]);
        e.call(0x008671a0, &args![0u32]);
        assert_eq!(calls(&e, AUDIO_SET_MUTE), vec![vec![0x5000, 1]]);
        assert_eq!(e.mem.u8(MUTED_BY_MOVIE), 1);
        assert_eq!(calls(&e, AUDIO_PAUSE).len(), 1);
        assert!(calls(&e, AUDIO_00AD7740).is_empty());
        // Already muted by someone else: nothing remembered; the mute stays
        // set, so no `00ad7740` either. A mute that does not stick: both.
        let mut e = audio_engine(true);
        e.call(0x008671a0, &args![0u32]);
        assert_eq!(e.mem.u8(MUTED_BY_MOVIE), 0);
        let mut e = audio_engine(false);
        e.register(AUDIO_SET_MUTE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x008671a0, &args![0u32]);
        assert_eq!(e.mem.u8(MUTED_BY_MOVIE), 0);
        assert_eq!(calls(&e, AUDIO_00AD7740), vec![vec![0x5000, 1]; 2]);
    }

    #[test]
    fn unmute_audio_undoes_only_its_own_mute() {
        let mut e = audio_engine(true);
        e.mem.set_u8(MUTED_BY_MOVIE, 1);
        e.call_log = Some(vec![]);
        e.call(0x00867220, &args![0u32]);
        assert_eq!(calls(&e, AUDIO_SET_MUTE), vec![vec![0x5000, 0]]);
        assert_eq!(e.mem.u8(MUTED_BY_MOVIE), 0);
        assert_eq!(calls(&e, AUDIO_RESUME).len(), 1);
        e.call_log = Some(vec![]);
        e.call(0x00867220, &args![0u32]);
        assert!(calls(&e, AUDIO_SET_MUTE).is_empty());
        assert_eq!(calls(&e, AUDIO_RESUME).len(), 1);
    }

    #[test]
    fn check_continue_playing() {
        const COOPERATIVE: u32 = 0x7200_000c;
        // (continues, device level, skip, skippable, pressed) -> answer, stopped
        for (continues, level, skip, skippable, pressed, answer, stopped) in [
            (false, 0u32, 0u8, 0u8, false, false, false),
            (true, 0x8000_0000, 0, 0, false, false, false),
            (true, 0, 0, 0, true, true, false),
            (true, 0, 1, 0, false, true, false),
            (true, 0, 1, 0, true, false, true),
            (true, 0, 0, 1, true, false, true),
        ] {
            let mut e = engine();
            e.mem.set_u32(STATE, u32::from(continues));
            e.mem.set_u32(STATE + 4, level);
            e.mem.set_u32(STATE + 8, u32::from(pressed));
            e.register(MOVIE_CONTINUE, |e, _| e.mem.u32(STATE).into_ret());
            e.register(RENDERER_FOR_DEVICE, |_, _| 0x4000u32.into_ret());
            e.put_vtable(0x7000_0000, &[0, 0, 0, COOPERATIVE]);
            let device = e.mem.alloc(4);
            e.mem.set_u32(device, 0x7000_0000);
            e.mem.set_u32(STATE + 12, device);
            e.register(RENDERER_GET_DEVICE, |e, _| e.mem.u32(STATE + 12).into_ret());
            e.register(COOPERATIVE, |e, _| e.mem.u32(STATE + 4).into_ret());
            e.register(MANAGER_CONTROLS, |_, _| 0x9000u32.into_ret());
            e.register(CONTROLS_REFRESH, |_, _| Ret::default());
            e.register(CONTROLS_PRESSED, |e, a| {
                (a[1] == 0x1c && e.mem.u32(STATE + 8) != 0).into_ret()
            });
            e.register(MOVIE_IS_PLAYING, |_, _| true.into_ret());
            e.register(MOVIE_STOP, |_, _| Ret::default());
            let this = e.mem.alloc(0x54);
            e.mem.set_u8(this + 0x20, skippable);
            e.call_log = Some(vec![]);
            let got = e.call(0x00867440, &args![this, u32::from(skip)]).bool();
            assert_eq!(got, answer);
            assert_eq!(!calls(&e, MOVIE_STOP).is_empty(), stopped);
            assert_eq!(calls(&e, MOVIE_CONTINUE), vec![vec![this, u32::from(skip)]]);
        }
    }
}

#[cfg(test)]
mod n072_tests {
    use super::*;

    #[test]
    fn pause_and_unpause_forward_to_the_media() {
        let mut e = Engine::new();
        e.register(MEDIA_PAUSE, |_, _| Ret::default());
        e.register(MEDIA_UNPAUSE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0086_7270, &args![0x10u32]);
        e.call(0x0086_7280, &args![0x10u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log,
            vec![
                (0x0086_7270, vec![0x10]),
                (MEDIA_PAUSE, vec![]),
                (0x0086_7280, vec![0x10]),
                (MEDIA_UNPAUSE, vec![]),
            ]
        );
    }
}
