//! `fallout shared/run_00488f10` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00488ff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00489620`, which frees the block held at `+4`
/// (through `00401030`) when it is non-null, zeroes `+4`, clears the byte at `+0` and
/// answers whether a block was freed, and returns its result (a `bool`). Parameters: one
/// word, `this`; the disassembly of the function ends in a plain `RET` and passes only `ECX`
/// on. The return type (`bool`, AL) is from `00489620`'s disassembly, which ends `MOV
/// AL,[EBP-1]` (the flag it sets to 1 only on the freeing path); the wrapper leaves AL as it
/// came back.
pub fn fn_00488ff0(e: &mut Engine, this: Ptr) -> bool {
    e.call(0x0048_9620, &args![this]).bool()
}

// Translated from 00488f50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00488f50 - the exception-unwinding frame (FS:[0]) is not translated.
/// The constructor of the form component `TESModel` (the engine map gives it
/// no name; the RTTI name of the vtable it stores, `0101cf44`, is
/// `.?AVTESModel@@`): runs the base constructor (`00402d30`), stores the
/// vtable at `+0`, constructs the string at `+4` (`004037b0`), runs `00488fd0`
/// on the member at `+0xc`, then `TESModel::InitializeDataComponent` (Xbox PDB
/// name of `00489040`). Returns `this`. The exception-unwinding frame is not
/// translated.
pub fn fn_00488f50(e: &mut Engine, this: Ptr) -> Ptr {
    e.call(0x0040_2d30, &args![this]);
    e.mem.set_u32(this.addr(), 0x0101_cf44);
    e.call(0x0040_37b0, &args![this.byte_add(4)]);
    e.call(0x0048_8fd0, &args![this.byte_add(0xc)]);
    e.call(0x0048_9040, &args![this]);
    this
}

// Translated from 00489010 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00489070` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00489010(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0048_9070, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00488fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Constructor of the two-word member at `TESModel+0xc` (class
/// unconfirmed; `00488f50` runs it): clears the byte at `+0` and the word
/// at `+4`. Returns `this`.
pub fn fn_00488fd0(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u8(this.addr(), 0);
    e.mem.set_u32(this.addr() + 4, 0);
    this
}

/// `0042ce30` (`thiscall`, `RET 4`; unit `extradatalist.cpp` by the engine
/// map): the changed-form data of the load buffer, written to the given
/// out word; answers the object `004280f0` is asked on.
const CHANGE_DATA: u32 = 0x0042_ce30;
/// `004280f0` (`thiscall`, `RET 4`): whether the change flags hold the
/// given bits (`AL`).
const HAS_CHANGE_FLAGS: u32 = 0x0042_80f0;
/// The `TESLeveledList` change flag of script-added entries (as
/// `run_00487620` names `0x80000000`).
const CHANGE_LEVELED_LIST_ADDED: u32 = 0x8000_0000;
/// `00487980` (`thiscall`, no stack word; `run_00487620`): takes the
/// script-added entries out of the leveled list.
const REMOVE_SCRIPT_ADDED: u32 = 0x0048_7980;

// Translated from 00488f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// A leveled list's revert step (`thiscall`, `RET 4`, the load buffer; the
/// engine map gives it no name): when the buffer's change data (`0042ce30`,
/// with a stack out word) has flag `0x80000000` (`004280f0`), the
/// script-added entries are taken out (`00487980` on `this`).
pub fn fn_00488f10(e: &mut Engine, this: Ptr, buffer: u32) {
    let flagged = e.with_stack(4, |e, out| {
        let data = e.call(CHANGE_DATA, &args![buffer, out]).u32();
        e.call(HAS_CHANGE_FLAGS, &args![data, CHANGE_LEVELED_LIST_ADDED])
            .bool()
    });
    if flagged {
        e.call(REMOVE_SCRIPT_ADDED, &args![this]);
    }
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0048_8f10, fn_00488f10(Ptr, u32)),
        entry!(0x0048_8ff0, fn_00488ff0(Ptr) -> bool),
        entry!(0x00488f50, fn_00488f50(Ptr) -> Ptr),
        entry!(0x0048_9010, fn_00489010(Ptr, u32) -> Ptr),
        entry!(0x00488fd0, fn_00488fd0(Ptr) -> Ptr),
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
    fn fwd_a03_00488ff0_returns_the_callees_al() {
        let mut e = Engine::new();
        e.register(0x0048_9620, |_, _| Ret {
            eax: 0x1234_5601,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(e.call(0x0048_8ff0, &args![0x0000_5000u32]).bool());
        assert_eq!(calls_to(&e, 0x0048_9620), vec![vec![0x5000]]);
        // Only AL counts: a non-zero upper part with AL clear is false.
        e.register(0x0048_9620, |_, _| Ret {
            eax: 0x1234_5600,
            ..Ret::default()
        });
        assert!(!e.call(0x0048_8ff0, &args![0x0000_5000u32]).bool());
    }
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00488f50_builds_the_component() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0048_8f50,
            size: 0x20,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_2d30, &[A::T(0x0)]),
                (0x0040_37b0, &[A::T(0x4)]),
                (0x0048_8fd0, &[A::T(0xc)]),
                (0x0048_9040, &[A::T(0x0)]),
            ],
            stores: &[(0x0, 4, 0x0101_cf44)],
        });
    }
    #[test]
    fn at_00489010_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0048_9070, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0048_9010, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0048_9010)
                .collect();
            let mut expected = vec![(0x0048_9070, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn at_00488fd0_clears_the_byte_and_the_word() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this = e.mem.alloc(8);
        e.mem.set_u32(this, 0xffff_ffff);
        e.mem.set_u32(this + 4, 0xffff_ffff);
        assert_eq!(e.call(0x0048_8fd0, &args![this]).u32(), this);
        // Only the low byte of the first word is cleared.
        assert_eq!(e.mem.u32(this), 0xffff_ff00);
        assert_eq!(e.mem.u32(this + 4), 0);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_00488f10_removes_when_flagged() {
        let mut e = Engine::new();
        e.mem.map(0x0f00_0000, 4);
        e.register(CHANGE_DATA, |_, _| Ret {
            eax: 0x6000,
            ..Ret::default()
        });
        e.register(HAS_CHANGE_FLAGS, |e, _| Ret {
            eax: e.mem.u32(0x0f00_0000),
            ..Ret::default()
        });
        e.register(REMOVE_SCRIPT_ADDED, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0048_8f10, &args![0x4000u32, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1].0, CHANGE_DATA);
        assert_eq!(log[1].1[0], 0x5000);
        assert_eq!(log[2], (HAS_CHANGE_FLAGS, vec![0x6000, 0x8000_0000]));
        assert_eq!(log.len(), 3);
        e.mem.set_u32(0x0f00_0000, 1);
        e.call_log = Some(vec![]);
        e.call(0x0048_8f10, &args![0x4000u32, 0x5000u32]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[3], (REMOVE_SCRIPT_ADDED, vec![0x4000]));
    }
}
