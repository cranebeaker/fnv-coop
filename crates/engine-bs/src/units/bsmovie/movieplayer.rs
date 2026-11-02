//! `BSMovie/movieplayer.obj` (Xbox PDB source unit), subsystem `BSMovie`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The movie object's destructor (`00ec1f40`, `thiscall`), `MemoryManager::Deallocate`
/// (`00aa4060`, `thiscall` on the manager `011f6238`, one word) and the player's own call after
/// the stop (`00ec2a60`, `thiscall`).
const MOVIE_DESTRUCT: u32 = 0x00ec_1f40;
const DEALLOCATE: u32 = 0x00aa_4060;
const MEMORY_MANAGER: u32 = 0x011f_6238;
const AFTER_STOP: u32 = 0x00ec_2a60;
/// The flag the inlined deleting destructor tests (`MOV ECX,1; AND ECX,1; JZ`): the constant 1,
/// so the free always runs.
const DELETE_FLAG: u32 = 1;

// Translated from 00ec25e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `MoviePlayer::Stop` (engine map; `thiscall`, no stack words, plain `RET`): a movie at `+0xc`
/// is destroyed (`00ec1f40`), its memory given back (`MemoryManager::Deallocate` on
/// `011f6238`), and the word cleared; then `00ec2a60`.
pub fn movie_player_stop(e: &mut Engine, this: Ptr) {
    let t = this.addr();
    if e.mem.u32(t + 0xc) != 0 {
        let movie = e.mem.u32(t + 0xc);
        if movie != 0 {
            e.call(MOVIE_DESTRUCT, &args![movie]);
            if DELETE_FLAG & 1 != 0 {
                e.call(DEALLOCATE, &args![MEMORY_MANAGER, movie]);
            }
        }
        e.mem.set_u32(t + 0xc, 0);
    }
    e.call(AFTER_STOP, &args![this]);
}

/// `MemoryManager::Allocate` (`00aa3e40`, `thiscall` on the manager `011f6238`, one word).
const ALLOCATE: u32 = 0x00aa_3e40;
/// The C runtime by address (`cdecl`): `strlen` (`00ec6130`), `strcpy` (`00ec6370`),
/// `_mbstok` (`00ece23e`: string, delimiters) and `_mbsicmp` (`00ec858a`).
const STRLEN: u32 = 0x00ec_6130;
const STRCPY: u32 = 0x00ec_6370;
const MBSTOK: u32 = 0x00ec_e23e;
const MBSICMP: u32 = 0x00ec_858a;
/// `GetCurrentThreadId` (KERNEL32 import slot `00fdf1d0`) and `BinkPause` (binkw32 import
/// slot `00fdf3b0`, `stdcall`: the Bink handle, pause).
const GET_CURRENT_THREAD_ID: u32 = 0x00fd_f1d0;
const BINK_PAUSE: u32 = 0x00fd_f3b0;
/// The release build's logging stub (`005b5e40`, `cdecl`, answers 0; the format and its
/// arguments are pushed and ignored).
const LOG: u32 = 0x005b_5e40;
/// The comma (`01063cd8`), `bik` (`010f1d48`), `DEFAULT: Unknown file type on file %s`
/// (`010f1d20`) and `DEFAULT: Trying to pause a movie while another movie is already
/// paused.  This is not currently supported.` (`010f1d50`), in `.rdata`.
const TEXT_COMMA: u32 = 0x0106_3cd8;
const TEXT_BIK: u32 = 0x010f_1d48;
const TEXT_UNKNOWN_FILE_TYPE: u32 = 0x010f_1d20;
const TEXT_PAUSE_WHILE_PAUSED: u32 = 0x010f_1d50;
/// The byte setting at `0126facc` (inside the object at `0126fac8`) passed to each movie's
/// play, and the fallback byte `01202800` the exe uses when that object's address is null
/// (it never is).
const PLAY_SETTING_HOLDER: u32 = 0x0126_fac8;
const PLAY_SETTING: u32 = 0x0126_facc;
const NULL_SETTING: u32 = 0x0120_2800;
/// The movie's play (`00ec0e00`, `Movie::Play`, `thiscall`, nine words), the sequence
/// thread's release of its starter (`00ec1a00`, `thiscall`), the movie's close (`00ec1020`,
/// `thiscall`) and its field reset (`00ec1570`, `thiscall`).
const MOVIE_PLAY: u32 = 0x00ec_0e00;
const SEQUENCE_RELEASE: u32 = 0x00ec_1a00;
const MOVIE_CLOSE: u32 = 0x00ec_1020;
const MOVIE_RESET: u32 = 0x00ec_1570;
/// The thread block's word at `+0x2b4` (the allocation context), which `Play` and
/// `PlaySequence` set to 0x14 and restore.
const TLS_CONTEXT: u32 = 0x2b4;
const MOVIE_CONTEXT: u32 = 0x14;
/// The player's vtable slots called here (the object is a `BGSMoviePlayer`, vtable
/// `01082564`): lock (+0x20) and unlock (+0x24) the renderer, the main thread's id (+0x3C)
/// and whether to go on (+0x40, one byte).
const LOCK_RENDERER: u32 = 0x20;
const UNLOCK_RENDERER: u32 = 0x24;
const MAIN_THREAD_ID: u32 = 0x3c;
const CHECK_CONTINUE_PLAYING: u32 = 0x40;

// Translated from 00ec1f20 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no unit; `thiscall`, no stack words): the movie's constructor,
/// its field reset (`00ec1570`); answers `this`.
pub fn fn_00ec1f20(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MOVIE_RESET, &args![this]);
    this
}

// Translated from 00ec1f40 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): the movie's destructor, its close
/// (`00ec1020`).
pub fn fn_00ec1f40(e: &mut Engine, this: Ptr) {
    e.call(MOVIE_CLOSE, &args![this]);
}

// Translated from 00ec1fb0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): pauses the movie,
/// `BinkPause(handle at +0, 1)`; its answer stays in `EAX`.
pub fn fn_00ec1fb0(e: &mut Engine, this: Ptr) -> u32 {
    let bink = e.mem.u32(this.addr());
    e.call(BINK_PAUSE, &args![bink, 1u32]).u32()
}

// Translated from 00ec1fd0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (no unit; `thiscall`, no stack words): resumes the movie,
/// `BinkPause(handle at +0, 0)`; its answer stays in `EAX`.
pub fn fn_00ec1fd0(e: &mut Engine, this: Ptr) -> u32 {
    let bink = e.mem.u32(this.addr());
    e.call(BINK_PAUSE, &args![bink, 0u32]).u32()
}

// Translated from 00ec2a10 (disassembled, FalloutNV.exe 1.4.0.525)
/// `MoviePlayer::Pause` (engine map; `thiscall`, no stack words): with a movie (+0xC), logs
/// (the stub `005b5e40`) when one is already paused (+0x10), pauses it (`00ec1fb0`), moves
/// it to +0x10 and clears +0xC.
pub fn movie_player_pause(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    if e.mem.u32(at + 0xc) == 0 {
        return;
    }
    if e.mem.u32(at + 0x10) != 0 {
        e.call(LOG, &args![TEXT_PAUSE_WHILE_PAUSED]);
    }
    let movie = e.mem.u32(at + 0xc);
    fn_00ec1fb0(e, Ptr::new(movie));
    let movie = e.mem.u32(at + 0xc);
    e.mem.set_u32(at + 0x10, movie);
    e.mem.set_u32(at + 0xc, 0);
}

// Translated from 00ec2a60 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, no stack words): what `Stop` runs last. Without a
/// movie (+0xC) but with a paused one (+0x10), the paused one moves back to +0xC, +0x10 is
/// cleared and it resumes (`00ec1fd0`).
pub fn fn_00ec2a60(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    if e.mem.u32(at + 0xc) != 0 || e.mem.u32(at + 0x10) == 0 {
        return;
    }
    let paused = e.mem.u32(at + 0x10);
    e.mem.set_u32(at + 0xc, paused);
    e.mem.set_u32(at + 0x10, 0);
    let movie = e.mem.u32(at + 0xc);
    fn_00ec1fd0(e, Ptr::new(movie));
}

// Translation incomplete: 00ec2320 - the FS:[0] exception frame (unwinding the new movie's allocation when its constructor throws) is not translated
// Translated from 00ec2320 (disassembled, FalloutNV.exe 1.4.0.525)
/// `MoviePlayer::Play` (engine map; `thiscall`, `RET 0x24`): plays one movie file. Sets the
/// thread block's allocation context (+0x2b4) to 0x14 (restored at the end). Unless
/// `keep_movie` is set: the playing movie is paused (`Pause`), then a remembered movie
/// (+0x14, with the byte +0x18 set) becomes the movie (+0xC; +0x14 and +0x18 cleared), or a
/// new one is made (`MemoryManager::Allocate(0x28)` on `011f6238`, then `00ec1f20` when that
/// is not null; null otherwise). The movie (+0xC) plays (`Movie::Play` `00ec0e00`: the file,
/// the bytes `b0c`, `b10`, `b14`, `b18`, `b1c`, `b24`, `b28` and the float at +0x28, copied
/// through `FLD`/`FSTP`), the player stops (`Stop`) and the play's answer is returned (`AL`).
#[allow(clippy::too_many_arguments)]
pub fn movie_player_play(
    e: &mut Engine,
    this: Ptr,
    file: u32,
    b0c: u8,
    b10: u8,
    b14: u8,
    b18: u8,
    b1c: u8,
    keep_movie: u8,
    b24: u8,
    b28: u8,
) -> bool {
    let at = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_CONTEXT, MOVIE_CONTEXT);
    if keep_movie == 0 {
        movie_player_pause(e, this);
        if e.mem.u8(at + 0x18) != 0 && e.mem.u32(at + 0x14) != 0 {
            let remembered = e.mem.u32(at + 0x14);
            e.mem.set_u32(at + 0xc, remembered);
            e.mem.set_u32(at + 0x14, 0);
            e.mem.set_u8(at + 0x18, 0);
        } else {
            let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x28u32]).u32();
            let movie = if block != 0 {
                fn_00ec1f20(e, Ptr::new(block)).addr()
            } else {
                0
            };
            e.mem.set_u32(at + 0xc, movie);
        }
    }
    let value = x87_quiet_bits(e.mem.u32(at + 0x28));
    let movie = e.mem.u32(at + 0xc);
    let played = e
        .call(
            MOVIE_PLAY,
            &args![
                movie,
                file,
                u32::from(b0c),
                u32::from(b10),
                u32::from(b14),
                u32::from(b18),
                u32::from(b1c),
                u32::from(b24),
                u32::from(b28),
                value
            ],
        )
        .u8();
    movie_player_stop(e, this);
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
    played != 0
}

// Translation incomplete: 00ec2760 - the FS:[0] exception frame (one state over the whole body, nothing to unwind) is not translated
// Translated from 00ec2760 (disassembled, FalloutNV.exe 1.4.0.525)
/// `MoviePlayer::PlaySequence` (engine map; `thiscall`, `RET 4`: the comma-separated file
/// list), run on the sequence thread (`00ec16b0`). Sets the thread block's allocation
/// context (+0x2b4) to 0x14 (restored at the end) and locks the renderer (vtable +0x20).
/// Copies the list (`strlen` + 1 bytes from `MemoryManager::Allocate` on `011f6238`,
/// `strcpy`), clears the released byte (+0x3C) and walks the copy with `_mbstok` on the
/// comma: while the stop byte (+0x21) is clear and the player goes on (vtable +0x40 with 1),
/// a file whose last three characters are `bik` (`_mbsicmp`) plays (`Play` with 1, 1, 0,
/// the byte at `0126facc`, +0x22, 0, +0x23, +0x24); any other file is logged (the stub
/// `005b5e40`). Then the copy is freed, the stop byte cleared, +8 (the thread handle)
/// cleared unless +0x20 is set, the starter released (`00ec1a00`) when this is not the main
/// thread (`GetCurrentThreadId` against vtable +0x3C) and that has not happened yet (+0x3C),
/// and the renderer unlocked (vtable +0x24).
pub fn movie_player_play_sequence(e: &mut Engine, this: Ptr, files: u32) {
    let at = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_CONTEXT, MOVIE_CONTEXT);
    e.vcall(at, LOCK_RENDERER, &[]);
    let size = e.call(STRLEN, &args![files]).u32().wrapping_add(1);
    let copy = e.call(ALLOCATE, &args![MEMORY_MANAGER, size]).u32();
    e.call(STRCPY, &args![copy, files]);
    let mut file = e.call(MBSTOK, &args![copy, TEXT_COMMA]).u32();
    // [EBP-0xD]: set to 1 once, pushed as the go-on argument and as `Play`'s first byte.
    let one: u8 = 1;
    e.mem.set_u8(at + 0x3c, 0);
    while file != 0 {
        if e.mem.u8(at + 0x21) == 0
            && e.vcall(at, CHECK_CONTINUE_PLAYING, &args![u32::from(one)])
                .u8()
                != 0
        {
            let length = e.call(STRLEN, &args![file]).u32();
            let extension = file.wrapping_add(length).wrapping_sub(3);
            if e.call(MBSICMP, &args![extension, TEXT_BIK]).u32() == 0 {
                let setting = if PLAY_SETTING_HOLDER != 0 {
                    PLAY_SETTING
                } else {
                    // Never taken: the exe tests a constant address.
                    e.mem.set_u8(NULL_SETTING, 0);
                    NULL_SETTING
                };
                let b24 = e.mem.u8(at + 0x24);
                let b23 = e.mem.u8(at + 0x23);
                let b22 = e.mem.u8(at + 0x22);
                let setting = e.mem.u8(setting);
                movie_player_play(e, this, file, one, 1, 0, setting, b22, 0, b23, b24);
            } else {
                e.call(LOG, &args![TEXT_UNKNOWN_FILE_TYPE, file]);
            }
        }
        file = e.call(MBSTOK, &args![0u32, TEXT_COMMA]).u32();
    }
    e.call(DEALLOCATE, &args![MEMORY_MANAGER, copy]);
    e.mem.set_u8(at + 0x21, 0);
    if e.mem.u8(at + 0x20) == 0 {
        e.mem.set_u32(at + 8, 0);
    }
    let current = e.call(GET_CURRENT_THREAD_ID, &[]).u32();
    let main = e.vcall(at, MAIN_THREAD_ID, &[]).u32();
    if current != main && e.mem.u8(at + 0x3c) == 0 {
        e.call(SEQUENCE_RELEASE, &args![this]);
    }
    e.vcall(at, UNLOCK_RENDERER, &[]);
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
}

/// The movie's open (`00ec0cd0`, `thiscall`, four words; answer `AL`), its update
/// (`00ec1060`, `thiscall`, three words; answer `AL`) and the `Sleep` import slot.
const MOVIE_OPEN: u32 = 0x00ec_0cd0;
const MOVIE_UPDATE: u32 = 0x00ec_1060;
const SLEEP: u32 = 0x00fd_f104;

// Translated from 00ec1f60 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`movieplayer.obj`; `thiscall` on a movie, two words: a file and
/// a flag whose low byte is passed zero-extended, `RET 8`): the word at +0x20 becomes 2 and the
/// byte at +0x24 is set while the movie is opened (`00ec0cd0(file, 1, flag, 0)`), then cleared.
/// Answers whether the open answered nonzero (`AL`).
pub fn fn_00ec1f60(e: &mut Engine, this: Ptr, file: u32, flag: u32) -> bool {
    let at = this.addr();
    e.mem.set_u32(at + 0x20, 2);
    e.mem.set_u8(at + 0x24, 1);
    let flag = u32::from(flag as u8);
    let opened = e
        .call(MOVIE_OPEN, &args![this, file, 1u32, flag, 0u32])
        .u8()
        != 0;
    e.mem.set_u8(at + 0x24, 0);
    opened
}

// Translated from 00ec2490 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec2490 - the SEH frame (FS:[0] handler 00f34bf8, states 0/1 around the allocation and the movie's construction) is not translated.
/// Unnamed in the engine map (`movieplayer.obj`; `thiscall`, two words: a file and a flag,
/// `RET 8`): with the thread block's allocation context (+0x2b4) set to 0x14 the player pauses
/// its movie ([`movie_player_pause`]); a movie kept at +0x14 with the byte at +0x18 set
/// becomes the current one (+0xc), the word and the byte cleared; otherwise a new 0x28-byte
/// movie (`00aa3e40`, made by [`fn_00ec1f20`]; null when the allocation fails) does. That movie
/// opens the file ([`fn_00ec1f60`] with the flag's low byte); when that fails the player stops
/// ([`movie_player_stop`]). The context is restored; answers whether the open succeeded.
pub fn fn_00ec2490(e: &mut Engine, this: Ptr, file: u32, flag: u32) -> bool {
    let at = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(tls + TLS_CONTEXT, MOVIE_CONTEXT);
    movie_player_pause(e, this);
    if e.mem.u8(at + 0x18) != 0 && e.mem.u32(at + 0x14) != 0 {
        let kept = e.mem.u32(at + 0x14);
        e.mem.set_u32(at + 0xc, kept);
        e.mem.set_u32(at + 0x14, 0);
        e.mem.set_u8(at + 0x18, 0);
    } else {
        let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x28u32]).u32();
        let movie = if block != 0 {
            fn_00ec1f20(e, Ptr::new(block)).addr()
        } else {
            0
        };
        e.mem.set_u32(at + 0xc, movie);
    }
    let movie = e.mem.u32(at + 0xc);
    let opened = fn_00ec1f60(e, Ptr::new(movie), file, u32::from(flag as u8));
    if !opened {
        movie_player_stop(e, this);
    }
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
    opened
}

// Translated from 00ec2650 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ec2650 - the SEH frame (FS:[0] handler 00f34c1b, state 0 while the context is set) is not translated.
/// Unnamed in the engine map (`movieplayer.obj`; `thiscall`, two words whose low bytes are
/// passed zero-extended, `RET 8`): with the thread block's allocation context (+0x2b4) set to
/// 0x14, a current movie (+0xc) whose word at +0x20 is not 0 is updated (`00ec1060(first,
/// second, 0)`), whose `AL` is answered; otherwise 0. The context is restored either way.
pub fn fn_00ec2650(e: &mut Engine, this: Ptr, first: u32, second: u32) -> bool {
    let at = this.addr();
    let tls = e.tls();
    let saved = e.mem.u32(tls + TLS_CONTEXT);
    e.mem.set_u32(tls + TLS_CONTEXT, MOVIE_CONTEXT);
    let movie = e.mem.u32(at + 0xc);
    let answer = if movie != 0 && e.mem.u32(movie + 0x20) != 0 {
        let movie = e.mem.u32(at + 0xc);
        let (first, second) = (u32::from(first as u8), u32::from(second as u8));
        e.call(MOVIE_UPDATE, &args![movie, first, second, 0u32])
            .u8()
            != 0
    } else {
        false
    };
    let tls = e.tls();
    e.mem.set_u32(tls + TLS_CONTEXT, saved);
    answer
}

// Translated from 00ec29b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `MoviePlayer::StopSequence` (engine map; `thiscall`, plain `RET`): sets the byte at +0x21.
pub fn movie_player_stop_sequence(e: &mut Engine, this: Ptr) {
    e.mem.set_u8(this.addr() + 0x21, 1);
}

// Translated from 00ec29d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`movieplayer.obj`; `thiscall`, plain `RET`): while a thread
/// handle is set (+8) and there is no current movie (+0xc) or its byte at +0x24 is set, sleeps
/// 1 ms (`Sleep(1)`), everything read again each time.
pub fn fn_00ec29d0(e: &mut Engine, this: Ptr) {
    let at = this.addr();
    loop {
        if e.mem.u32(at + 8) == 0 {
            break;
        }
        let movie = e.mem.u32(at + 0xc);
        if movie != 0 && e.mem.u8(movie + 0x24) == 0 {
            break;
        }
        e.call(SLEEP, &args![1u32]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_1f60, fn_00ec1f60(Ptr, u32, u32) -> bool),
        entry!(0x00ec_2490, fn_00ec2490(Ptr, u32, u32) -> bool),
        entry!(0x00ec_2650, fn_00ec2650(Ptr, u32, u32) -> bool),
        entry!(0x00ec_29b0, movie_player_stop_sequence(Ptr)),
        entry!(0x00ec_29d0, fn_00ec29d0(Ptr)),
        entry!(0x00ec_25e0, movie_player_stop(Ptr)),
        entry!(0x00ec_1f20, fn_00ec1f20(Ptr) -> Ptr),
        entry!(0x00ec_1f40, fn_00ec1f40(Ptr)),
        entry!(0x00ec_1fb0, fn_00ec1fb0(Ptr) -> u32),
        entry!(0x00ec_1fd0, fn_00ec1fd0(Ptr) -> u32),
        entry!(0x00ec_2a10, movie_player_pause(Ptr)),
        entry!(0x00ec_2a60, fn_00ec2a60(Ptr)),
        entry!(
            0x00ec_2320,
            movie_player_play(Ptr, u32, u8, u8, u8, u8, u8, u8, u8, u8) -> bool
        ),
        entry!(0x00ec_2760, movie_player_play_sequence(Ptr, u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    #[test]
    fn stop_destroys_and_frees_the_movie_then_runs_00ec2a60() {
        let mut e = Engine::new();
        for addr in [MOVIE_DESTRUCT, DEALLOCATE, AFTER_STOP] {
            e.register(addr, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        let player = e.mem.alloc(0x40);
        e.mem.set_u32(player + 0xc, 0x0500_0000);
        e.call(0x00ec_25e0, &args![player]);
        assert_eq!(calls(&e, MOVIE_DESTRUCT), vec![vec![0x0500_0000]]);
        assert_eq!(
            calls(&e, DEALLOCATE),
            vec![vec![MEMORY_MANAGER, 0x0500_0000]]
        );
        assert_eq!(e.mem.u32(player + 0xc), 0);
        assert_eq!(calls(&e, AFTER_STOP), vec![vec![player]]);
        // Without a movie only 00ec2a60 runs.
        e.call(0x00ec_25e0, &args![player]);
        assert_eq!(calls(&e, MOVIE_DESTRUCT).len(), 1);
        assert_eq!(calls(&e, DEALLOCATE).len(), 1);
        assert_eq!(calls(&e, AFTER_STOP).len(), 2);
    }
}

#[cfg(test)]
mod play_tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn calls_at(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for &addr in addrs {
            e.register(addr, |_, _| Ret::default());
        }
    }

    #[test]
    fn movie_construct_destruct_pause_and_resume_forward() {
        let mut e = Engine::new();
        quiet(&mut e, &[MOVIE_RESET, MOVIE_CLOSE]);
        e.register(BINK_PAUSE, |_, a| (a[1] + 0x10).into_ret());
        e.call_log = Some(vec![]);
        let movie = e.mem.alloc(0x28);
        e.mem.set_u32(movie, 0x0bb0_0000);
        assert_eq!(e.call(0x00ec_1f20, &args![movie]).u32(), movie);
        assert_eq!(calls_at(&e, MOVIE_RESET), vec![vec![movie]]);
        e.call(0x00ec_1f40, &args![movie]);
        assert_eq!(calls_at(&e, MOVIE_CLOSE), vec![vec![movie]]);
        assert_eq!(e.call(0x00ec_1fb0, &args![movie]).u32(), 0x11);
        assert_eq!(e.call(0x00ec_1fd0, &args![movie]).u32(), 0x10);
        assert_eq!(
            calls_at(&e, BINK_PAUSE),
            vec![vec![0x0bb0_0000, 1], vec![0x0bb0_0000, 0]]
        );
    }

    #[test]
    fn pause_parks_the_movie_and_the_stop_tail_brings_it_back() {
        let mut e = Engine::new();
        quiet(&mut e, &[BINK_PAUSE, LOG]);
        e.call_log = Some(vec![]);
        let player = e.mem.alloc(0x40);
        let movie = e.mem.alloc(0x28);
        e.mem.set_u32(movie, 0x0bb0_0000);
        // No movie: nothing.
        e.call(0x00ec_2a10, &args![player]);
        assert!(calls_at(&e, BINK_PAUSE).is_empty());
        e.mem.set_u32(player + 0xc, movie);
        e.call(0x00ec_2a10, &args![player]);
        assert_eq!(calls_at(&e, BINK_PAUSE), vec![vec![0x0bb0_0000, 1]]);
        assert!(calls_at(&e, LOG).is_empty());
        assert_eq!(
            (e.mem.u32(player + 0xc), e.mem.u32(player + 0x10)),
            (0, movie)
        );
        // A second pause while one is parked logs and replaces it.
        e.mem.set_u32(player + 0xc, movie);
        e.call(0x00ec_2a10, &args![player]);
        assert_eq!(calls_at(&e, LOG), vec![vec![TEXT_PAUSE_WHILE_PAUSED]]);
        // The stop tail: with a movie nothing happens; without one the parked one resumes.
        e.mem.set_u32(player + 0xc, movie);
        e.call(0x00ec_2a60, &args![player]);
        assert_eq!(calls_at(&e, BINK_PAUSE).len(), 2);
        e.mem.set_u32(player + 0xc, 0);
        e.call(0x00ec_2a60, &args![player]);
        assert_eq!(
            (e.mem.u32(player + 0xc), e.mem.u32(player + 0x10)),
            (movie, 0)
        );
        assert_eq!(calls_at(&e, BINK_PAUSE)[2], vec![0x0bb0_0000, 0]);
        // Neither: nothing.
        e.mem.set_u32(player + 0xc, 0);
        e.call(0x00ec_2a60, &args![player]);
        assert_eq!(calls_at(&e, BINK_PAUSE).len(), 3);
    }

    /// An engine with the play's callees doubled: `Allocate` hands out 0x0c00_0000,
    /// `Movie::Play` checks the context word and answers 1.
    fn play_engine() -> Engine {
        let mut e = Engine::new();
        quiet(
            &mut e,
            &[
                MOVIE_RESET,
                MOVIE_DESTRUCT,
                DEALLOCATE,
                AFTER_STOP,
                BINK_PAUSE,
                LOG,
            ],
        );
        e.register(ALLOCATE, |_, _| 0x0c00_0000u32.into_ret());
        e.register(MOVIE_PLAY, |e, _| {
            let tls = e.tls();
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), MOVIE_CONTEXT);
            1u32.into_ret()
        });
        e
    }

    #[test]
    fn play_makes_a_movie_plays_it_and_stops() {
        let mut e = play_engine();
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 7);
        let player = e.mem.alloc(0x40);
        // TEST DATA: a signalling NaN at +0x28 is passed quiet.
        e.mem.set_u32(player + 0x28, 0x7f80_0001);
        e.call_log = Some(vec![]);
        let words = args![
            player,
            0x0f11_e000u32,
            1u32,
            2u32,
            3u32,
            4u32,
            5u32,
            0u32,
            7u32,
            8u32
        ];
        assert!(e.call(0x00ec_2320, &words).bool());
        assert_eq!(calls_at(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 0x28]]);
        assert_eq!(calls_at(&e, MOVIE_RESET), vec![vec![0x0c00_0000]]);
        assert_eq!(
            calls_at(&e, MOVIE_PLAY),
            vec![vec![
                0x0c00_0000,
                0x0f11_e000,
                1,
                2,
                3,
                4,
                5,
                7,
                8,
                0x7fc0_0001
            ]]
        );
        // Stop destroyed and freed the movie.
        assert_eq!(calls_at(&e, MOVIE_DESTRUCT), vec![vec![0x0c00_0000]]);
        assert_eq!(e.mem.u32(player + 0xc), 0);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 7);
    }

    #[test]
    fn play_takes_the_remembered_movie_or_keeps_the_current_one() {
        let mut e = play_engine();
        let player = e.mem.alloc(0x40);
        e.mem.set_u32(player + 0x14, 0x0d00_0000);
        e.mem.set_u8(player + 0x18, 1);
        e.call_log = Some(vec![]);
        let words = |keep: u32| args![player, 0u32, 0u32, 0u32, 0u32, 0u32, 0u32, keep, 0u32, 0u32];
        e.call(0x00ec_2320, &words(0));
        assert!(calls_at(&e, ALLOCATE).is_empty());
        assert_eq!(calls_at(&e, MOVIE_PLAY)[0][0], 0x0d00_0000);
        assert_eq!((e.mem.u32(player + 0x14), e.mem.u8(player + 0x18)), (0, 0));
        // Keeping the movie: no pause, no new movie; the one at +0xC plays.
        e.mem.set_u32(player + 0xc, 0x0e00_0000);
        e.call(0x00ec_2320, &words(1));
        assert!(calls_at(&e, ALLOCATE).is_empty());
        assert!(calls_at(&e, BINK_PAUSE).is_empty());
        assert_eq!(calls_at(&e, MOVIE_PLAY)[1][0], 0x0e00_0000);
        // A failed allocation leaves a null movie.
        e.register(ALLOCATE, |_, _| Ret::default());
        e.register(MOVIE_PLAY, |_, _| Ret::default());
        assert!(!e.call(0x00ec_2320, &words(0)).bool());
        assert_eq!(calls_at(&e, MOVIE_PLAY)[2][0], 0);
        assert!(calls_at(&e, MOVIE_RESET).is_empty());
    }

    /// The `BGSMoviePlayer` stand-in: vtable slots +0x20, +0x24, +0x3C and +0x40 at
    /// `0x0030_0000 + slot`; +0x3C answers `main`, +0x40 answers 1.
    fn sequence_player(e: &mut Engine, main: u32) -> u32 {
        let vtable = e.mem.alloc(0x44);
        for slot in [
            LOCK_RENDERER,
            UNLOCK_RENDERER,
            MAIN_THREAD_ID,
            CHECK_CONTINUE_PLAYING,
        ] {
            e.mem.set_u32(vtable + slot, 0x0030_0000 + slot);
        }
        quiet(e, &[0x0030_0020, 0x0030_0024]);
        e.register_double(0x0030_003c, move |_, _| main.into_ret());
        e.register(0x0030_0040, |_, _| 1u32.into_ret());
        let player = e.mem.alloc(0x40);
        e.mem.set_u32(player, vtable);
        player
    }

    /// `strlen`, `strcpy`, `_mbsicmp` and a `_mbstok` on commas, on the engine memory.
    fn crt(e: &mut Engine) {
        e.register(STRLEN, |e, a| (e.mem.cstr(a[0]).len() as u32).into_ret());
        e.register(STRCPY, |e, a| {
            let s = e.mem.cstr(a[1]);
            e.mem.set_cstr(a[0], &s);
            a[0].into_ret()
        });
        e.register(MBSICMP, |e, a| {
            let (x, y) = (e.mem.cstr(a[0]), e.mem.cstr(a[1]));
            u32::from(!x.eq_ignore_ascii_case(&y)).into_ret()
        });
        let next = Rc::new(RefCell::new(0u32));
        e.register_double(MBSTOK, move |e, a| {
            let mut at = if a[0] != 0 { a[0] } else { *next.borrow() };
            while at != 0 && e.mem.u8(at) == b',' {
                at += 1;
            }
            if at == 0 || e.mem.u8(at) == 0 {
                *next.borrow_mut() = 0;
                return Ret::default();
            }
            let start = at;
            while e.mem.u8(at) != 0 && e.mem.u8(at) != b',' {
                at += 1;
            }
            if e.mem.u8(at) == b',' {
                e.mem.set_u8(at, 0);
                at += 1;
            }
            *next.borrow_mut() = at;
            start.into_ret()
        });
    }

    #[test]
    fn play_sequence_plays_each_bik_and_logs_the_rest() {
        let mut e = play_engine();
        crt(&mut e);
        e.map(0x0106_3000, 0x1000);
        e.map(0x010f_1000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        e.mem.set_cstr(TEXT_COMMA, b",");
        e.mem.set_cstr(TEXT_BIK, b"bik");
        e.mem.set_u8(PLAY_SETTING, 9);
        e.register(GET_CURRENT_THREAD_ID, |_, _| 0x44u32.into_ret());
        quiet(&mut e, &[SEQUENCE_RELEASE]);
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let player = sequence_player(&mut e, 0x10);
        e.mem.set_u8(player + 0x22, 0x22);
        e.mem.set_u8(player + 0x23, 0x23);
        e.mem.set_u8(player + 0x24, 0x24);
        e.mem.set_u32(player + 8, 0x0123);
        e.mem.set_u8(player + 0x21, 1);
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 3);
        let files = e.mem.alloc(0x40);
        e.mem
            .set_cstr(files, b"Intro.BIK,readme.txt,Fallout INTRO Vsk.bik");
        e.call_log = Some(vec![]);
        e.call(0x00ec_2760, &args![player, files]);
        // The stop byte was set: nothing played, and it is cleared after.
        assert!(calls_at(&e, MOVIE_PLAY).is_empty());
        assert!(calls_at(&e, 0x0030_0040).is_empty());
        assert_eq!(e.mem.u8(player + 0x21), 0);
        assert_eq!(calls_at(&e, MBSTOK).len(), 4);
        // Then the whole list.
        e.call_log = Some(vec![]);
        e.call(0x00ec_2760, &args![player, files]);
        assert_eq!(calls_at(&e, 0x0030_0020).len(), 1);
        assert_eq!(calls_at(&e, 0x0030_0040), vec![vec![player, 1]; 3]);
        let plays = calls_at(&e, MOVIE_PLAY);
        assert_eq!(plays.len(), 2);
        assert_eq!(e.mem.cstr(plays[0][1]), b"Intro.BIK");
        assert_eq!(e.mem.cstr(plays[1][1]), b"Fallout INTRO Vsk.bik");
        let logged = calls_at(&e, LOG);
        assert_eq!(logged.len(), 1);
        assert_eq!(logged[0][0], TEXT_UNKNOWN_FILE_TYPE);
        assert_eq!(e.mem.cstr(logged[0][1]), b"readme.txt");
        let copy = calls_at(&e, STRCPY)[0][0];
        assert_eq!(
            calls_at(&e, DEALLOCATE).last().unwrap(),
            &vec![MEMORY_MANAGER, copy]
        );
        // Not the main thread and not yet released: the starter is released; +8 cleared.
        assert_eq!(calls_at(&e, SEQUENCE_RELEASE), vec![vec![player]]);
        assert_eq!(e.mem.u32(player + 8), 0);
        assert_eq!(calls_at(&e, 0x0030_0024).len(), 1);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 3);
    }

    #[test]
    fn play_sequence_passes_the_player_bytes_to_play() {
        let mut e = play_engine();
        crt(&mut e);
        e.map(0x0106_3000, 0x1000);
        e.map(0x010f_1000, 0x1000);
        e.map(0x0126_f000, 0x1000);
        e.mem.set_cstr(TEXT_COMMA, b",");
        e.mem.set_cstr(TEXT_BIK, b"bik");
        e.mem.set_u8(PLAY_SETTING, 9);
        e.register(GET_CURRENT_THREAD_ID, |_, _| 0x10u32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let player = sequence_player(&mut e, 0x10);
        e.mem.set_u8(player + 0x22, 0x22);
        e.mem.set_u8(player + 0x23, 0x23);
        e.mem.set_u8(player + 0x24, 0x24);
        // A stand-in play that records the bytes `Play` hands `Movie::Play`.
        let files = e.mem.alloc(0x10);
        e.mem.set_cstr(files, b"a.bik");
        e.call_log = Some(vec![]);
        e.call(0x00ec_2760, &args![player, files]);
        let plays = calls_at(&e, MOVIE_PLAY);
        assert_eq!(plays.len(), 1);
        // Play(file, 1, 1, 0, setting, +0x22, keep 0, +0x23, +0x24): Movie::Play gets the
        // bytes but the keep flag.
        assert_eq!(&plays[0][2..9], &[1, 1, 0, 9, 0x22, 0x23, 0x24]);
    }

    #[test]
    fn play_sequence_on_the_main_thread_or_already_released_keeps_the_starter() {
        let mut e = play_engine();
        crt(&mut e);
        e.map(0x0106_3000, 0x1000);
        e.map(0x010f_1000, 0x1000);
        e.mem.set_cstr(TEXT_COMMA, b",");
        quiet(&mut e, &[SEQUENCE_RELEASE]);
        e.register(GET_CURRENT_THREAD_ID, |_, _| 0x10u32.into_ret());
        e.register(ALLOCATE, |e, a| e.mem.alloc(a[1]).into_ret());
        let player = sequence_player(&mut e, 0x10);
        e.mem.set_u8(player + 0x20, 1);
        e.mem.set_u32(player + 8, 0x0123);
        let files = e.mem.alloc(4);
        e.mem.set_cstr(files, b"");
        e.call_log = Some(vec![]);
        e.call(0x00ec_2760, &args![player, files]);
        assert!(calls_at(&e, SEQUENCE_RELEASE).is_empty());
        assert_eq!(e.mem.u32(player + 8), 0x0123);
        // Another thread, but the starter was released while the list ran (+0x3C set, as
        // 00ec1a00 sets it): the walk's last `_mbstok` sets it here.
        e.register(GET_CURRENT_THREAD_ID, |_, _| 0x44u32.into_ret());
        e.register(MBSTOK, |e, a| {
            if a[0] != 0 {
                return a[0].into_ret();
            }
            let player = e.mem.u32(0x0106_3f00);
            e.mem.set_u8(player + 0x3c, 1);
            Ret::default()
        });
        e.mem.set_u32(0x0106_3f00, player);
        e.mem.set_cstr(files, b"x.t");
        e.call(0x00ec_2760, &args![player, files]);
        assert_eq!(calls_at(&e, LOG).len(), 1);
        assert!(calls_at(&e, SEQUENCE_RELEASE).is_empty());
    }
}

#[cfg(test)]
mod finish_tests {
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

    #[test]
    fn a_movie_opens_with_its_busy_byte_set() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let movie = e.mem.alloc(0x28);
        e.register_double(MOVIE_OPEN, move |e, a| {
            assert_eq!(e.mem.u8(movie + 0x24), 1);
            u32::from(a[1] == 0x10).into_ret()
        });
        assert!(e.call(0x00ec_1f60, &args![movie, 0x10u32, 0x201u32]).bool());
        assert_eq!(e.mem.u32(movie + 0x20), 2);
        assert_eq!(e.mem.u8(movie + 0x24), 0);
        assert!(!fn_00ec1f60(&mut e, Ptr::new(movie), 0x11, 0));
        assert_eq!(
            calls(&e, MOVIE_OPEN),
            vec![vec![movie, 0x10, 1, 1, 0], vec![movie, 0x11, 1, 0, 0]]
        );
    }

    #[test]
    fn open_reuses_a_kept_movie_or_makes_one_and_restores_the_context() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        for slot in [MOVIE_RESET, AFTER_STOP, MOVIE_DESTRUCT] {
            e.register(slot, |_, _| Ret::default());
        }
        let tls = e.tls();
        e.mem.set_u32(tls + TLS_CONTEXT, 7);
        e.register_double(MOVIE_OPEN, move |e, a| {
            assert_eq!(e.mem.u32(tls + TLS_CONTEXT), MOVIE_CONTEXT);
            u32::from(a[1] == 1).into_ret()
        });
        let player = e.mem.alloc(0x40);
        let kept = e.mem.alloc(0x28);
        e.mem.set_u32(player + 0x14, kept);
        e.mem.set_u8(player + 0x18, 1);
        assert!(e.call(0x00ec_2490, &args![player, 1u32, 0u32]).bool());
        assert_eq!(e.mem.u32(player + 0xc), kept);
        assert_eq!((e.mem.u32(player + 0x14), e.mem.u8(player + 0x18)), (0, 0));
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 7);
        assert!(calls(&e, ALLOCATE).is_empty());
        // No kept movie: a new 0x28-byte one; a failed open stops the player.
        e.mem.set_u32(player + 0xc, 0);
        assert!(!fn_00ec2490(&mut e, Ptr::new(player), 2, 0));
        assert_eq!(calls(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 0x28]]);
        assert_eq!(calls(&e, MOVIE_RESET).len(), 1);
        assert_eq!(calls(&e, MOVIE_DESTRUCT).len(), 1);
        assert_eq!(e.mem.u32(player + 0xc), 0);
        assert_eq!(e.mem.u32(tls + TLS_CONTEXT), 7);
    }

    #[test]
    fn update_needs_a_movie_with_its_word_at_0x20() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(MOVIE_UPDATE, |_, _| 0x101u32.into_ret());
        let player = e.mem.alloc(0x40);
        assert!(!e.call(0x00ec_2650, &args![player, 1u32, 1u32]).bool());
        let movie = e.mem.alloc(0x28);
        e.mem.set_u32(player + 0xc, movie);
        assert!(!fn_00ec2650(&mut e, Ptr::new(player), 1, 1));
        e.mem.set_u32(movie + 0x20, 1);
        assert!(fn_00ec2650(&mut e, Ptr::new(player), 0x301, 0x200));
        assert_eq!(calls(&e, MOVIE_UPDATE), vec![vec![movie, 1, 0, 0]]);
    }

    #[test]
    fn stop_sequence_and_the_wait_for_a_movie() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        let player = e.mem.alloc(0x40);
        e.call(0x00ec_29b0, &args![player]);
        assert_eq!(e.mem.u8(player + 0x21), 1);
        // No thread: no wait.
        e.call(0x00ec_29d0, &args![player]);
        assert!(calls(&e, SLEEP).is_empty());
        e.mem.set_u32(player + 8, 0x55);
        let movie = e.mem.alloc(0x28);
        e.mem.set_u8(movie + 0x24, 1);
        let mut n = 0;
        e.register_double(SLEEP, move |e, _| {
            n += 1;
            if n == 2 {
                e.mem.set_u32(player + 0xc, movie);
            }
            if n == 3 {
                e.mem.set_u8(movie + 0x24, 0);
            }
            Ret::default()
        });
        fn_00ec29d0(&mut e, Ptr::new(player));
        assert_eq!(calls(&e, SLEEP), vec![vec![1], vec![1], vec![1]]);
    }
}
