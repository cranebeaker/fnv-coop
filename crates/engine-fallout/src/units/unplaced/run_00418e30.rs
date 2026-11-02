//! `(unplaced)/run_00418e30` (Xbox PDB source unit), subsystem `(unplaced)`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00418f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `0045cec0`, the smart-pointer destructor of unit
/// run_0045cb90, and returns nothing. Parameters: one word, `this`; the disassembly of the
/// function ends in a plain `RET` and passes only `ECX` on. `0045cec0`'s translated
/// signature `fn_0045cec0(Ptr)` returns nothing.
pub fn fn_00418f40(e: &mut Engine, this: Ptr) {
    e.call(0x0045_cec0, &args![this]);
}

// Translated from 00418f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00418f40` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00418f10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0041_8f40, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `BaseExtraList::GetExtraData` (`00410220`, thiscall, type byte).
const GET_EXTRA_DATA: u32 = 0x0041_0220;
/// `BaseExtraList::AddExtra` (`0040ff60`, thiscall, extra data).
const ADD_EXTRA: u32 = 0x0040_ff60;
/// `00401000` (cdecl, byte count): allocates a block.
const ALLOCATE_BLOCK: u32 = 0x0040_1000;
/// `004308f0` (thiscall, word): the constructor of the type-0x29 extra data
/// (0x10 bytes), holding the word at +0xC.
const EXTRA_CONSTRUCT: u32 = 0x0043_08f0;
/// The extra data type this function handles.
const EXTRA_TYPE: u32 = 0x29;

// Translated from 00418e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: sets the word held by the extra list's
/// type-0x29 extra data to `value` and returns that extra data. An existing
/// one first releases its old word (`00418f10(old, 1)` when non-null); a
/// missing one is created (0x10 bytes, `004308f0(value)`) and added to the
/// list.
// Translation incomplete: 00418e30 - SEH exception unwinding (the allocation's cleanup on unwind) is unsupported.
pub fn fn_00418e30(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let extra = e.call(GET_EXTRA_DATA, &args![this, EXTRA_TYPE]).u32();
    if extra != 0 {
        let old = e.mem.u32(extra + 0xc);
        if old != 0 {
            fn_00418f10(e, Ptr::new(old), 1);
        }
        e.mem.set_u32(extra + 0xc, value);
        return extra;
    }
    let block = e.call(ALLOCATE_BLOCK, &args![0x10u32]).u32();
    let created = if block != 0 {
        e.call(EXTRA_CONSTRUCT, &args![block, value]).u32()
    } else {
        0
    };
    e.call(ADD_EXTRA, &args![this, created]);
    created
}

/// `004f15a0` (thiscall, one word): stores a type byte into the object (the stage and
/// extra-data constructors run it with their type).
const SET_TYPE_004F15A0: u32 = 0x004f_15a0;

// Translated from 00418f60 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00418f60 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00f00bdb, state variable around the constructor) and the frame's cookie word are not translated.
/// Unnamed in the engine map (`ECX` = an extra list, one stack word, `RET 4`; called by
/// `00572770`): sets the word held by the list's type-0x40 extra data to `value` and returns
/// that extra data. An existing one first releases its old word (`00418f10(old, 1)` when
/// non-null); a missing one is made (0x10 bytes from `00401000`, `004308f0(value)` when the
/// block is not null), given type 0x40 (`004f15a0`, also on a null result) and added to the
/// list (`BaseExtraList::AddExtra`).
pub fn fn_00418f60(e: &mut Engine, this: Ptr, value: u32) -> u32 {
    let extra = e.call(GET_EXTRA_DATA, &args![this, 0x40u32]).u32();
    if extra != 0 {
        let old = e.mem.u32(extra + 0xc);
        if old != 0 {
            fn_00418f10(e, Ptr::new(old), 1);
        }
        e.mem.set_u32(extra + 0xc, value);
        return extra;
    }
    let block = e.call(ALLOCATE_BLOCK, &args![0x10u32]).u32();
    let created = if block != 0 {
        e.call(EXTRA_CONSTRUCT, &args![block, value]).u32()
    } else {
        0
    };
    e.call(SET_TYPE_004F15A0, &args![created, 0x40u32]);
    e.call(ADD_EXTRA, &args![this, created]);
    created
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0041_8f60, fn_00418f60(Ptr, u32) -> u32),
        entry!(0x0041_8f40, fn_00418f40(Ptr)),
        entry!(0x0041_8f10, fn_00418f10(Ptr, u32) -> Ptr),
        entry!(0x0041_8e30, fn_00418e30(Ptr, u32) -> u32),
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
    fn fwd_a03_00418f40_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0045_cec0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0041_8f40, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0045_cec0), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_00418f10_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0041_8f40, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0041_8f10, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0041_8f10)
                .collect();
            let mut expected = vec![(0x0041_8f40, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn fn_00418e30_replaces_the_held_word_or_adds_the_extra() {
        let mut e = Engine::new();
        // The list's extra (if any) is its word at +0.
        e.register(GET_EXTRA_DATA, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        e.register(ADD_EXTRA, |e, a| {
            e.mem.set_u32(a[0], a[1]);
            Ret::default()
        });
        e.register(ALLOCATE_BLOCK, |e, a| Ret {
            eax: e.mem.alloc(a[0]),
            ..Ret::default()
        });
        e.register(EXTRA_CONSTRUCT, |e, a| {
            e.mem.set_u32(a[0] + 0xc, a[1]);
            Ret {
                eax: a[0],
                ..Ret::default()
            }
        });
        e.register(0x0041_8f40, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        let list = e.mem.alloc(4);
        e.call_log = Some(vec![]);
        let extra = e.call(0x0041_8e30, &args![list, 0x5000u32]).u32();
        assert_ne!(extra, 0);
        assert_eq!(e.mem.u32(list), extra);
        assert_eq!(e.mem.u32(extra + 0xc), 0x5000);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(GET_EXTRA_DATA, vec![list, 0x29])));
        assert!(!log.iter().any(|c| c.0 == 0x0041_8f40));

        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0041_8e30, &args![list, 0x6000u32]).u32(), extra);
        assert_eq!(e.mem.u32(extra + 0xc), 0x6000);
        let log = e.call_log.take().unwrap();
        assert!(
            log.contains(&(0x0040_1030, vec![0x5000])),
            "old word released"
        );
        assert!(!log.iter().any(|c| c.0 == ADD_EXTRA));

        e.mem.set_u32(extra + 0xc, 0);
        e.call_log = Some(vec![]);
        e.call(0x0041_8e30, &args![list, 0x7000u32]);
        assert!(!e
            .call_log
            .take()
            .unwrap()
            .iter()
            .any(|c| c.0 == 0x0041_8f40));
        assert_eq!(e.mem.u32(extra + 0xc), 0x7000);
    }
}

#[cfg(test)]
mod n075_tests {
    use super::*;

    fn engine(existing: u32) -> Engine {
        let mut e = Engine::new();
        e.register_double(GET_EXTRA_DATA, move |_, _| existing.into_ret());
        e.register(ALLOCATE_BLOCK, |_, _| 0x0090_0000u32.into_ret());
        e.register(EXTRA_CONSTRUCT, |_, a| a[0].into_ret());
        e.register(SET_TYPE_004F15A0, |_, _| Ret::default());
        e.register(ADD_EXTRA, |_, _| Ret::default());
        e.register(0x0041_8f40, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    #[test]
    fn fn_00418f60_replaces_the_word_of_an_existing_extra() {
        let mut e = engine(0);
        let extra = e.mem.alloc(0x10);
        e.register_double(GET_EXTRA_DATA, move |_, _| extra.into_ret());
        e.mem.set_u32(extra + 0xc, 0x4444);
        assert_eq!(
            e.call(0x0041_8f60, &args![0x1000u32, 0x5555u32]).u32(),
            extra
        );
        assert_eq!(e.mem.u32(extra + 0xc), 0x5555);
        let log = e.call_log.take().unwrap();
        assert!(log.contains(&(GET_EXTRA_DATA, vec![0x1000, 0x40])));
        assert!(log.contains(&(0x0041_8f40, vec![0x4444])));
        assert!(log.contains(&(0x0040_1030, vec![0x4444])));
        assert!(!log.iter().any(|(a, _)| *a == ADD_EXTRA));
    }

    #[test]
    fn fn_00418f60_makes_and_adds_a_missing_extra() {
        let mut e = engine(0);
        assert_eq!(
            e.call(0x0041_8f60, &args![0x1000u32, 0x5555u32]).u32(),
            0x0090_0000
        );
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[2..].to_vec(),
            vec![
                (ALLOCATE_BLOCK, vec![0x10]),
                (EXTRA_CONSTRUCT, vec![0x0090_0000, 0x5555]),
                (SET_TYPE_004F15A0, vec![0x0090_0000, 0x40]),
                (ADD_EXTRA, vec![0x1000, 0x0090_0000]),
            ]
        );
    }
}
