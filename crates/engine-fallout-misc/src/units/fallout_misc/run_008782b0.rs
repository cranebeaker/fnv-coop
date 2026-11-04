//! `fallout/misc/run_008782b0` (Xbox PDB source unit), subsystem `fallout/misc`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! A run of small functions after `memorylevelmanager.cpp`:
//!
//! - `008782b0`: opens a memory-freeing scope (`FreeScope`, in
//!   `run_00877fc0.rs`), frees memory and closes it;
//! - `00878340`: reads a byte of the object at `011dea3c` (the global the
//!   player character is read from elsewhere);
//! - `00878360`: whether a range of two words of its object is small;
//! - `008783a0`: forwards the address `this + 0x14` to `0044fb00`;
//! - `008783c0`: the constructor of the 0x18-byte file object used by the
//!   memory statistics commands (`tesscriptfunctions.rs` calls it).

#[allow(unused_imports)]
use crate::prelude::*;

use crate::units::fallout_misc::run_00877fc0::FreeScope;

/// `00878160`: opens a `FreeScope` (cdecl: the scope, three flag bytes).
const FREE_SCOPE_OPEN: u32 = 0x0087_8160;
/// `00878200`: closes the scope.
const FREE_SCOPE_CLOSE: u32 = 0x0087_8200;
/// `00878250` (`MemoryLevelManager::FreeReleasedObjects`, Xbox PDB; cdecl,
/// one word: the byte `begin_result` of the scope).
const FREE_RELEASED_OBJECTS: u32 = 0x0087_8250;
/// The global pointer whose object `00878340` is called on.
const PLAYER_POINTER: u32 = 0x011d_ea3c;
/// `00652110`: returns the object the checks below are made on.
const CHECKED_OBJECT: u32 = 0x0065_2110;
/// `00529ea0`: a test on that object (returns a byte).
const OBJECT_TEST: u32 = 0x0052_9ea0;
/// `00a09030`: cdecl, one word (`0x40c`); returns a word.
const LOOKUP_040C: u32 = 0x00a0_9030;
/// `00650a30`: thiscall on the object with one word.
const OBJECT_ACTION: u32 = 0x0065_0a30;
/// `0044fb00`: called on the address `this + 0x14`.
const MEMBER_CALL: u32 = 0x0044_fb00;
/// `008784b0`: the base constructor `008783c0` runs first (thiscall).
const FILE_BASE_CONSTRUCTOR: u32 = 0x0087_84b0;
/// `00b00900`: thiscall on `this + 4`; four stack words (the name and
/// three constants).
const FILE_OPEN: u32 = 0x00b0_0900;
/// The vtable `008783c0` installs.
const FILE_VTABLE: u32 = 0x0108_3ae8;
/// The range at most this long is small for `00878360`.
const SMALL_RANGE: u32 = 0x0050_0000;

// Translated from 008782b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Opens a memory-freeing scope with all three flags set (`00878160`), has
/// `00878250` act on the scope's `begin_result` byte, then, when the object
/// at `011dea3c` has the byte `00878340` reads set and the object `00652110`
/// returns passes `00529ea0` and `00a09030(0x40c)` returns 0, calls
/// `00650a30(0)` on that object. The scope is closed (`00878200`) at the
/// end.
pub fn fn_008782b0(e: &mut Engine) {
    e.with_stack(0xc, |e, scope| {
        e.call(FREE_SCOPE_OPEN, &args![scope, 1u32, 1u32, 1u32]);
        let begin_result = e.get(scope.cast::<FreeScope>(), FreeScope::begin_result);
        e.call(FREE_RELEASED_OBJECTS, &args![begin_result]);
        let player = Ptr::new(e.global::<u32>(PLAYER_POINTER));
        if fn_00878340(e, player) != 0 && e.call(CHECKED_OBJECT, &[]).u32() != 0 {
            let object = e.call(CHECKED_OBJECT, &[]).u32();
            if e.call(OBJECT_TEST, &args![object]).u8() != 0
                && e.call(LOOKUP_040C, &args![0x40cu32]).u32() == 0
            {
                let object = e.call(CHECKED_OBJECT, &[]).u32();
                e.call(OBJECT_ACTION, &args![object, 0u32]);
            }
        }
        e.call(FREE_SCOPE_CLOSE, &args![scope]);
    });
}

// Translated from 00878340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The byte at `+0x75d` of `this` (called on the object at `011dea3c`).
pub fn fn_00878340(e: &mut Engine, this: Ptr) -> u8 {
    e.mem.u8(this.addr().wrapping_add(0x75d))
}

// Translated from 00878360 (decompiled, FalloutNV.exe 1.4.0.525)
/// 1 unless the word at `+0x10` is above the word at `+0x8` by more than
/// 5 MB (both unsigned); 0 then.
pub fn fn_00878360(e: &mut Engine, this: Ptr) -> u8 {
    let high = e.mem.u32(this.addr().wrapping_add(0x10));
    let low = e.mem.u32(this.addr().wrapping_add(0x8));
    if high > low && high.wrapping_sub(low) > SMALL_RANGE {
        0
    } else {
        1
    }
}

// Translated from 008783a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `0044fb00` on `this + 0x14` and returns what it returns.
pub fn fn_008783a0(e: &mut Engine, this: Ptr) -> u32 {
    e.call(MEMBER_CALL, &args![this.addr().wrapping_add(0x14)])
        .u32()
}

// Translated from 008783c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the file object: runs the base constructor `008784b0`,
/// installs the vtable `01083ae8` and opens the file at `this + 4` through
/// `00b00900(name, 1, 2, 0)`. Returns `this`.
// Translation incomplete: 008783c0 - the compiler's exception-unwind frame (FS:[0]) is not translated.
pub fn fn_008783c0(e: &mut Engine, this: Ptr, name: u32) -> Ptr {
    e.call(FILE_BASE_CONSTRUCTOR, &args![this]);
    e.mem.set_u32(this.addr(), FILE_VTABLE);
    e.call(
        FILE_OPEN,
        &args![this.addr().wrapping_add(4), name, 1u32, 2u32, 0u32],
    );
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x008782b0, fn_008782b0()),
        entry!(0x00878340, fn_00878340(Ptr) -> u8),
        entry!(0x00878360, fn_00878360(Ptr) -> u8),
        entry!(0x008783a0, fn_008783a0(Ptr) -> u32),
        entry!(0x008783c0, fn_008783c0(Ptr, u32) -> Ptr),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word_ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    /// Doubles for everything `008782b0` calls; the scope's `begin_result`
    /// byte is set by the open double.
    fn scope_engine() -> (Engine, u32) {
        let mut e = Engine::new();
        e.register(FREE_SCOPE_OPEN, |e, a| {
            e.mem.set_u8(a[0] + 5, 0x7e);
            Ret::default()
        });
        for addr in [FREE_RELEASED_OBJECTS, FREE_SCOPE_CLOSE, OBJECT_ACTION] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(CHECKED_OBJECT, |_, _| word_ret(0x0e00_0000));
        e.register(OBJECT_TEST, |_, _| word_ret(1));
        e.register(LOOKUP_040C, |_, _| word_ret(0));
        e.map(PLAYER_POINTER, 4);
        let player = e.mem.alloc(0x800);
        e.set_global(PLAYER_POINTER, player);
        e.mem.set_u8(player + 0x75d, 1);
        (e, player)
    }

    #[test]
    fn scope_frees_and_acts_when_every_check_passes() {
        let (mut e, _) = scope_engine();
        e.call_log = Some(vec![]);
        e.call(0x0087_82b0, &[]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<(u32, Vec<u32>)> = log[1..].to_vec();
        assert_eq!(calls[0].0, FREE_SCOPE_OPEN);
        assert_eq!(calls[0].1[1..], [1, 1, 1]);
        let scope = calls[0].1[0];
        assert_eq!(calls[1], (FREE_RELEASED_OBJECTS, vec![0x7e]));
        assert!(calls.contains(&(LOOKUP_040C, vec![0x40c])));
        assert!(calls.contains(&(OBJECT_ACTION, vec![0x0e00_0000, 0])));
        assert_eq!(*calls.last().unwrap(), (FREE_SCOPE_CLOSE, vec![scope]));
    }

    #[test]
    fn scope_skips_the_action_when_a_check_fails() {
        // The byte is clear: no object lookup at all.
        let (mut e, player) = scope_engine();
        e.mem.set_u8(player + 0x75d, 0);
        e.call_log = Some(vec![]);
        e.call(0x0087_82b0, &[]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log.len(), 4);
        // No object.
        let (mut e, _) = scope_engine();
        e.register(CHECKED_OBJECT, |_, _| word_ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0087_82b0, &[]);
        assert_eq!(e.call_log.take().unwrap().len(), 5);
        // The object test fails.
        let (mut e, _) = scope_engine();
        e.register(OBJECT_TEST, |_, _| word_ret(0));
        e.call_log = Some(vec![]);
        e.call(0x0087_82b0, &[]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == LOOKUP_040C));
        // The lookup finds something.
        let (mut e, _) = scope_engine();
        e.register(LOOKUP_040C, |_, _| word_ret(9));
        e.call_log = Some(vec![]);
        e.call(0x0087_82b0, &[]);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|c| c.0 == OBJECT_ACTION));
        assert_eq!(log.last().unwrap().0, FREE_SCOPE_CLOSE);
    }

    #[test]
    fn byte_getter_reads_offset_75d() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x800);
        e.mem.set_u8(object + 0x75d, 0x42);
        assert_eq!(e.call(0x0087_8340, &[object]).u8(), 0x42);
    }

    #[test]
    fn range_is_small_unless_it_exceeds_five_megabytes() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x20);
        for (low, high, small) in [
            (0x1000u32, 0x1000u32, 1u32),
            (0x2000, 0x1000, 1),
            (0x1000, 0x1000 + 0x50_0000, 1),
            (0x1000, 0x1001 + 0x50_0000, 0),
            (0, 0xffff_ffff, 0),
        ] {
            e.mem.set_u32(object + 8, low);
            e.mem.set_u32(object + 0x10, high);
            assert_eq!(e.call(0x0087_8360, &[object]).u32() & 0xff, small);
        }
    }

    #[test]
    fn member_call_gets_the_address_of_the_member_and_returns_its_result() {
        let mut e = Engine::new();
        e.register(MEMBER_CALL, |_, a| word_ret(a[0] + 1));
        assert_eq!(e.call(0x0087_83a0, &[0x1000]).u32(), 0x1015);
    }

    #[test]
    fn file_constructor_runs_the_base_installs_the_vtable_and_opens() {
        let mut e = Engine::new();
        e.register(FILE_BASE_CONSTRUCTOR, |_, _| Ret::default());
        e.register(FILE_OPEN, |_, _| Ret::default());
        let object = e.mem.alloc(0x18);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0087_83c0, &[object, 0x2222]).u32(), object);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (FILE_BASE_CONSTRUCTOR, vec![object]));
        assert_eq!(log[2], (FILE_OPEN, vec![object + 4, 0x2222, 1, 2, 0]));
        assert_eq!(e.mem.u32(object), 0x0108_3ae8);
    }
}
