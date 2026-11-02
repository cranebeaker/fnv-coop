//! `fallout shared/run_0055a240` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0055a290 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0055a2b0`, which calls `006208d0` on the member
/// at `this + 8`, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `0055a2b0` ends right after that
/// call, and `006208d0` ends with its exception-frame unlink after `CALL 0045cec0`; no
/// result is computed and Ghidra types both void.
pub fn fn_0055a290(e: &mut Engine, this: Ptr) {
    e.call(0x0055_a2b0, &args![this]);
}

/// The game's `free` (`00401030`, cdecl, one word).
const FREE: u32 = 0x0040_1030;
/// `0054acf0` (`tesobjectcell.cpp`, thiscall on the member at +8).
const MEMBER_CONSTRUCT_0054ACF0: u32 = 0x0054_acf0;
/// `CameraCaster::~CameraCaster` (`006208d0`, engine map), run on the
/// member at +8.
const MEMBER_DESTRUCT: u32 = 0x0062_08d0;
/// Import slot of `CloseHandle` (`00fdf07c`).
const CLOSE_HANDLE: u32 = 0x00fd_f07c;
/// The member at +8 the constructor and destructor work on, and the handle
/// word at +4 `0055a2d0` closes.
const MEMBER: u32 = 0x08;
const HANDLE: u32 = 0x04;

// Translated from 0055a240 (decompiled, FalloutNV.exe 1.4.0.525)
/// A scalar deleting destructor (unnamed in the engine map): the
/// destructor body (`0055a290`), then `00401030(this)` when bit 0 of
/// `flags` is set. Returns `this`.
pub fn fn_0055a240(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0055a290(e, this);
    if flags & 1 != 0 {
        e.call(FREE, &args![this]);
    }
    this
}

// Translated from 0055a270 (decompiled, FalloutNV.exe 1.4.0.525)
/// A constructor (unnamed in the engine map): `0054acf0` on the member at
/// +8. Returns `this`.
pub fn fn_0055a270(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(MEMBER_CONSTRUCT_0054ACF0, &args![this.addr() + MEMBER]);
    this
}

// Translated from 0055a2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor of the member at +8 (`006208d0`, which the engine map
/// names `CameraCaster::~CameraCaster`). Returns nothing.
pub fn fn_0055a2b0(e: &mut Engine, this: Ptr) {
    e.call(MEMBER_DESTRUCT, &args![this.addr() + MEMBER]);
}

// Translated from 0055a2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Closes the handle at +4 (`CloseHandle`). Returns nothing.
pub fn fn_0055a2d0(e: &mut Engine, this: Ptr) {
    let handle = e.mem.u32(this.addr() + HANDLE);
    e.call(CLOSE_HANDLE, &args![handle]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0055_a290, fn_0055a290(Ptr)),
        entry!(0x0055_a240, fn_0055a240(Ptr, u32) -> Ptr),
        entry!(0x0055_a270, fn_0055a270(Ptr) -> Ptr),
        entry!(0x0055_a2b0, fn_0055a2b0(Ptr)),
        entry!(0x0055_a2d0, fn_0055a2d0(Ptr)),
    ]
}

#[cfg(test)]
mod thunk_tests {
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
    fn fwd_a03_0055a290_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0055_a2b0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0055_a290, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0055_a2b0), vec![vec![0x5000]]);
    }

    #[test]
    fn fn_0055a240_destroys_then_frees_only_with_bit_zero() {
        let mut e = Engine::new();
        e.register(0x0055_a2b0, |_, _| Ret::default());
        e.register(FREE, |_, _| Ret::default());
        for (flags, frees) in [(0u32, false), (2, false), (1, true), (3, true)] {
            e.call_log = Some(vec![]);
            assert_eq!(e.call(0x0055_a240, &args![0x5000u32, flags]).u32(), 0x5000);
            assert_eq!(calls_to(&e, 0x0055_a2b0), vec![vec![0x5000]]);
            let frees_seen = calls_to(&e, FREE);
            if frees {
                assert_eq!(frees_seen, vec![vec![0x5000]]);
            } else {
                assert!(frees_seen.is_empty());
            }
        }
    }

    #[test]
    fn fn_0055a270_constructs_the_member_at_8() {
        let mut e = Engine::new();
        e.register(MEMBER_CONSTRUCT_0054ACF0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0055_a270, &args![0x5000u32]).u32(), 0x5000);
        assert_eq!(calls_to(&e, MEMBER_CONSTRUCT_0054ACF0), vec![vec![0x5008]]);
    }

    #[test]
    fn fn_0055a2b0_destroys_the_member_at_8() {
        let mut e = Engine::new();
        e.register(MEMBER_DESTRUCT, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0055_a2b0, &args![0x5000u32]);
        assert_eq!(calls_to(&e, MEMBER_DESTRUCT), vec![vec![0x5008]]);
    }

    #[test]
    fn fn_0055a2d0_closes_the_handle_at_4() {
        let mut e = Engine::new();
        e.register(CLOSE_HANDLE, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        let this = e.mem.alloc(8);
        e.mem.set_u32(this + 4, 0x0000_0abc);
        e.call_log = Some(vec![]);
        e.call(0x0055_a2d0, &args![this]);
        assert_eq!(calls_to(&e, CLOSE_HANDLE), vec![vec![0x0abc]]);
    }
}
