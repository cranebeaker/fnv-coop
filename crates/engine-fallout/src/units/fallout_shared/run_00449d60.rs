//! `fallout shared/run_00449d60` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00449e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00449d60` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00449e70(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_9d60, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `0044c480` (`thiscall`, `RET 4`): run with 1 first.
const CLEAR_ENTRIES: u32 = 0x0044_c480;
/// `0044c3f0` (`thiscall`, no stack word): the base destructor.
const BASE_DESTRUCTOR_0044C3F0: u32 = 0x0044_c3f0;

// Translated from 00449d60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00449d60 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eece28, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): a destructor. Sets the vtable
/// `0101732c`, calls `0044c480(1)` on `this`, then the base destructor `0044c3f0`.
pub fn fn_00449d60(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_732c);
    e.call(CLEAR_ENTRIES, &args![this, 1u32]);
    e.call(BASE_DESTRUCTOR_0044C3F0, &args![this]);
}

/// The 4-byte memory-context guard the game keeps on its stack
/// (`00404eb0`: `thiscall`, four words; `00404ee0` ends it).
const GUARD_BEGIN: u32 = 0x0040_4eb0;
const GUARD_END: u32 = 0x0040_4ee0;
/// `"d:\_fallout3\platforms\common\code\bsmain\ThreadSafeStructures.h"`.
const SOURCE_FILE: u32 = 0x0101_71a0;
/// `0044a670` (`cdecl`, one word): the string's length.
const STRING_LENGTH: u32 = 0x0044_a670;
/// `operator new` (`00401000`, `cdecl`, one word).
const OPERATOR_NEW: u32 = 0x0040_1000;
/// `00406d30` (`cdecl`: destination, size, source): the bounded string copy.
const STRING_COPY: u32 = 0x0040_6d30;

// Translated from 00449dc0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00449dc0 - the exception-unwinding frame (FS:[0]) that ends the guard on a throw is not translated.
/// Copies a string key into a new block (`thiscall`, `RET 4`; `this` is not
/// read; the engine map gives it no name): inside the memory-context guard
/// (`00404eb0` with 6, 1, the header's name and line 0x1d7), takes length +
/// 1 bytes from `operator new` and copies the key into them (`00406d30`).
/// Returns the block.
pub fn fn_00449dc0(e: &mut Engine, _this: Ptr, key: u32) -> u32 {
    e.with_stack(4, |e, guard| {
        e.call(
            GUARD_BEGIN,
            &args![guard, 6u32, 1u32, SOURCE_FILE, 0x1d7u32],
        );
        let size = e.call(STRING_LENGTH, &args![key]).u32().wrapping_add(1);
        let block = e.call(OPERATOR_NEW, &args![size]).u32();
        e.call(STRING_COPY, &args![block, size, key]);
        e.call(GUARD_END, &args![guard]);
        block
    })
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_9dc0, fn_00449dc0(Ptr, u32) -> u32),
        entry!(0x0044_9e70, fn_00449e70(Ptr, u32) -> Ptr),
        entry!(0x0044_9d60, fn_00449d60(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_00449e70_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_9d60, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_9e70, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_9e70)
                .collect();
            let mut expected = vec![(0x0044_9d60, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00449d60_clears_then_runs_the_base() {
        let mut e = Engine::new();
        e.register(CLEAR_ENTRIES, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR_0044C3F0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0044_9d60, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (CLEAR_ENTRIES, vec![this, 1]),
                (BASE_DESTRUCTOR_0044C3F0, vec![this])
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0101_732c);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_00449dc0_copies_the_key_inside_the_guard() {
        let mut e = Engine::new();
        for addr in [GUARD_BEGIN, GUARD_END, STRING_COPY] {
            e.register(addr, |_, _| Ret::default());
        }
        e.register(STRING_LENGTH, |_, _| Ret {
            eax: 4,
            ..Ret::default()
        });
        e.register(OPERATOR_NEW, |_, _| Ret {
            eax: 0x6000,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0044_9dc0, &args![0u32, 0x5000u32]).u32(), 0x6000);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != 0x0044_9dc0)
            .collect();
        let guard = log[0].1[0];
        assert_eq!(
            log,
            vec![
                (GUARD_BEGIN, vec![guard, 6, 1, SOURCE_FILE, 0x1d7]),
                (STRING_LENGTH, vec![0x5000]),
                (OPERATOR_NEW, vec![5]),
                (STRING_COPY, vec![0x6000, 5, 0x5000]),
                (GUARD_END, vec![guard]),
            ]
        );
    }
}
