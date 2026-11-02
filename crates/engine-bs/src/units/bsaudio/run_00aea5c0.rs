//! `BSAudio/run_00aea5c0` (Xbox PDB source unit), subsystem `BSAudio`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! Two virtual methods of `BSWin32Audio` (RTTI `.?AVBSWin32Audio@@`, PC
//! vtable `010a39d8`): slot +0x14 (`00aea5c0`) and slot +0x18
//! (`00aea640`). By their bodies they create a game sound and fix a sound
//! path; the Xbox PDB's `BSAudio` has `CreateNewGameSound` and
//! `FixSoundPath` one slot later each, so those names are not confirmed.

#[allow(unused_imports)]
use crate::prelude::*;

/// The game's memory manager object: `ECX` of `MemoryManager::Allocate`.
const MEMORY_MANAGER: u32 = 0x011f_6238;
/// `MemoryManager::Allocate` (platform): `(manager, size)`.
const ALLOCATE: u32 = 0x00aa_3e40;
/// The constructor run on the new 0x230-byte sound (`00aeac20`: `this`,
/// one argument).
const GAME_SOUND_CONSTRUCT: u32 = 0x00ae_ac20;
/// `_strcpy_s` (CRT, cdecl: destination, size, source).
const STRCPY_S: u32 = 0x00ec_65a6;
/// `_strcat_s` (CRT, cdecl: destination, size, source).
const STRCAT_S: u32 = 0x00ec_6bd2;
/// `"data\\sound\\"`.
const SOUND_FOLDER: u32 = 0x0107_ce20;

// Translated from 00aea5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWin32Audio`'s virtual slot +0x14: allocates 0x230 bytes and runs the
/// sound constructor `00aeac20` on them with `argument`; returns the new
/// object, or null when the allocation failed. `this` is not read.
// Translation incomplete: 00aea5c0 - SEH exception frame (FS:[0]) that frees the block if the constructor throws is not modelled.
pub fn fn_00aea5c0(e: &mut Engine, _this: Ptr, argument: u32) -> Ptr {
    let block = e.call(ALLOCATE, &args![MEMORY_MANAGER, 0x230u32]).u32();
    if block == 0 {
        Ptr::NULL
    } else {
        e.call(GAME_SOUND_CONSTRUCT, &args![block, argument]).ptr()
    }
}

// Translated from 00aea640 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BSWin32Audio`'s virtual slot +0x18: a path starting with `fx` or
/// `song` gets `data\sound\` put in front of it (copied through a
/// 260-byte buffer on the stack; the path buffer is taken to be 260 bytes).
/// Null or other paths are left alone. `this` is not read.
// Translation incomplete: 00aea640 - the /GS stack cookie check (00ec408c) is not modelled.
pub fn fn_00aea640(e: &mut Engine, _this: Ptr, path: Ptr) {
    if path.is_null() {
        return;
    }
    let p = path.addr();
    let fx = e.mem.u8(p) == b'f' && e.mem.u8(p.wrapping_add(1)) == b'x';
    let song = !fx
        && e.mem.u8(p) == b's'
        && e.mem.u8(p.wrapping_add(1)) == b'o'
        && e.mem.u8(p.wrapping_add(2)) == b'n'
        && e.mem.u8(p.wrapping_add(3)) == b'g';
    if fx || song {
        e.with_stack(0x10c, |e, copy| {
            e.call(STRCPY_S, &args![copy, 0x104u32, path]);
            e.call(STRCPY_S, &args![path, 0x104u32, SOUND_FOLDER]);
            e.call(STRCAT_S, &args![path, 0x104u32, copy]);
        });
    }
}

/// `BSWin32Audio`'s destructor body (`00aea210`, `units/bsaudio/run_00aea020.rs`;
/// `thiscall`, no stack words) and the heap free (`00aa4060`, `thiscall` on the heap
/// object, one word).
const WIN32_AUDIO_DESTRUCTOR: u32 = 0x00ae_a210;
const HEAP_FREE: u32 = 0x00aa_4060;

// Translated from 00aea700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `BSWin32Audio`'s scalar deleting destructor (`thiscall`, one
/// word of flags): the destructor body `00aea210`, then with flag bit 0 the block freed into
/// the heap at `011f6238` (`00aa4060`). Answers `this`.
pub fn fn_00aea700(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(WIN32_AUDIO_DESTRUCTOR, &args![this]);
    if flags & 1 != 0 {
        e.call(HEAP_FREE, &args![MEMORY_MANAGER, this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ae_a700, fn_00aea700(Ptr, u32) -> Ptr),
        entry!(0x00ae_a5c0, fn_00aea5c0(Ptr, u32) -> Ptr),
        entry!(0x00ae_a640, fn_00aea640(Ptr, Ptr)),
    ]
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn creates_a_sound_or_null() {
        let mut e = Engine::new();
        e.register(ALLOCATE, |_, a| Ret {
            eax: if a[1] == 0x230 { 0x0092_0000 } else { 1 },
            ..Ret::default()
        });
        e.register(GAME_SOUND_CONSTRUCT, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_a5c0, &args![0u32, 7u32]).u32(), 0x0092_0000);
        assert_eq!(
            calls_to(&e, GAME_SOUND_CONSTRUCT),
            vec![vec![0x0092_0000, 7]]
        );
        assert_eq!(calls_to(&e, ALLOCATE), vec![vec![MEMORY_MANAGER, 0x230]]);
        e.register(ALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_a5c0, &args![0u32, 7u32]).u32(), 0);
        assert!(calls_to(&e, GAME_SOUND_CONSTRUCT).is_empty());
    }

    #[test]
    fn sound_paths_get_the_folder() {
        let mut e = Engine::new();
        e.map(0x0107_c000, 0x1000);
        e.mem.set_cstr(SOUND_FOLDER, b"data\\sound\\");
        let path = e.mem.alloc(0x104);
        for (input, output) in [
            (&b"fx\\a.wav"[..], &b"data\\sound\\fx\\a.wav"[..]),
            (b"song\\b.mp3", b"data\\sound\\song\\b.mp3"),
            (b"sonx", b"sonx"),
            (b"voice", b"voice"),
        ] {
            e.mem.set_cstr(path, input);
            e.call(0x00ae_a640, &args![0u32, path]);
            assert_eq!(e.mem.cstr(path), output);
        }
        e.call_log = Some(vec![]);
        e.call(0x00ae_a640, &args![0u32, 0u32]);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }
}

#[cfg(test)]
mod n082_tests {
    use super::*;

    #[test]
    fn fn_00aea700_destroys_then_frees_only_with_flag_bit_0() {
        let mut e = Engine::new();
        e.register(WIN32_AUDIO_DESTRUCTOR, |_, _| Ret::default());
        e.register(HEAP_FREE, |_, _| Ret::default());
        let calls_to = |e: &Engine, addr: u32| -> Vec<Vec<u32>> {
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|c| c.0 == addr)
                .map(|c| c.1.clone())
                .collect()
        };
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00ae_a700, &args![0x5000u32, 2u32]).u32(), 0x5000);
        assert!(calls_to(&e, HEAP_FREE).is_empty());
        assert_eq!(e.call(0x00ae_a700, &args![0x5000u32, 1u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, HEAP_FREE), vec![vec![MEMORY_MANAGER, 0x5000]]);
        assert_eq!(calls_to(&e, WIN32_AUDIO_DESTRUCTOR), vec![vec![0x5000]; 2]);
    }
}
