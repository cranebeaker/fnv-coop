//! `fallout shared/run_005173d0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 005173e0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 005173e0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x1c (0x90 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_005173e0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_f6f0, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0048_cc50, &args![base + 0x5c]);
    e.call(0x0047_5350, &args![base + 0x68]);
    e.call(0x0051_0620, &args![base + 0x70]);
    e.mem.set_u32(base, 0x0102_b1fc);
    e.mem.set_u32(base + 0x30, 0x0102_b1e4);
    e.mem.set_u32(base + 0x3c, 0x0102_b1c0);
    e.mem.set_u32(base + 0x5c, 0x0102_b1ac);
    e.mem.set_u32(base + 0x68, 0x0102_b198);
    e.mem.set_u32(base + 0x70, 0x0102_b188);
    e.call(0x0096_a2d0, &args![base + 0x88]);
    e.call(0x004f_15a0, &args![this, 0x1c]);
    e.call(0x0051_75d0, &args![this]);
    this
}

// Translated from 00517610 (decompiled, FalloutNV.exe 1.4.0.525)
/// A forwarder: it hands `this` (`ECX`) to `00519550`, which calls `00470470` on the member
/// at `this + 0x88`, and returns nothing. Parameters: one word, `this`; the disassembly of
/// the function ends in a plain `RET` and passes only `ECX` on. `00519550` ends right after
/// that call with no result of its own; `00470470` ends on a store (`MOV [EDX],0`), and
/// Ghidra types both void.
pub fn fn_00517610(e: &mut Engine, this: Ptr) {
    e.call(0x0051_9550, &args![this]);
}

// Translated from 005175d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start values of the form `005173e0` builds (form type 0x1c; class unconfirmed): zeroes the
/// words at `+0x78`, `+0x7c` and `+0x80` and the byte at `+0x84`.
pub fn fn_005175d0(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base + 0x78, 0);
    e.mem.set_u32(base + 0x7c, 0);
    e.mem.set_u32(base + 0x80, 0);
    e.mem.set_u8(base + 0x84, 0);
}

// Translated from 005174d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00517500` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_005174d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0051_7500, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `004ae750` (thiscall): null for a null `this`, otherwise `00620b80`'s
/// result (engine map name `bhkCharacterProxy::operator*`, unconfirmed).
const OBJECT_PROXY: u32 = 0x004a_e750;
/// `009d9f40` (thiscall): `this + 0xe0`.
const PLUS_E0: u32 = 0x009d_9f40;
/// `00519b00` `PackageTarget::GetTargType` (Xbox PDB name in the engine
/// map; thiscall).
const PACKAGE_TARGET_GET_TARG_TYPE: u32 = 0x0051_9b00;

// Translated from 00517690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `PackageTarget::GetTargType` on `this + 8`.
pub fn fn_00517690(e: &mut Engine, this: Ptr) -> u32 {
    e.call(PACKAGE_TARGET_GET_TARG_TYPE, &args![this.byte_add(8)])
        .u32()
}

// Translated from 00517670 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: `00517690` on what `009d9f40` answers
/// (`this + 0xe0`).
pub fn fn_00517670(e: &mut Engine, this: Ptr) -> u32 {
    let member = e.call(PLUS_E0, &args![this]).u32();
    fn_00517690(e, Ptr::new(member))
}

// Translated from 00517630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: 5 when `004ae750` answers null, otherwise
/// `00517670` on its answer.
pub fn fn_00517630(e: &mut Engine, this: Ptr) -> u32 {
    let proxy = e.call(OBJECT_PROXY, &args![this]).u32();
    if proxy == 0 {
        5
    } else {
        fn_00517670(e, Ptr::new(proxy))
    }
}

// Translated from 00517500 (decompiled, FalloutNV.exe 1.4.0.525)
/// The destructor body of the form `005173e0` builds (the vtables it stores
/// are `TESObjectDOOR`'s by their RTTI): stores the six vtables at +0,
/// +0x30, +0x3C, +0x5C, +0x68, +0x70, then `00517610`, and destroys the
/// members in reverse order: +0x88 (`0046ffb0`), +0x68 (`00475380`), +0x3C
/// (`0048a490`), +0x30 (`00591300`), and the base (`0050f740`).
// Translation incomplete: 00517500 - SEH exception unwinding is unsupported.
pub fn fn_00517500(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_b1fc);
    e.mem.set_u32(base + 0x30, 0x0102_b1e4);
    e.mem.set_u32(base + 0x3c, 0x0102_b1c0);
    e.mem.set_u32(base + 0x5c, 0x0102_b1ac);
    e.mem.set_u32(base + 0x68, 0x0102_b198);
    e.mem.set_u32(base + 0x70, 0x0102_b188);
    fn_00517610(e, this);
    e.call(0x0046_ffb0, &args![base + 0x88]);
    e.call(0x0047_5380, &args![base + 0x68]);
    e.call(0x0048_a490, &args![base + 0x3c]);
    e.call(0x0059_1300, &args![base + 0x30]);
    e.call(0x0050_f740, &args![this]);
}

// Translated from 005173d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x48 (`SUB ECX,0x48`) and jumps to
/// `00516210` (the deleting destructor in `tesobjectcont.rs`, `RET 4`),
/// passing its stack word on; returns what it returns.
pub fn fn_005173d0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    let base = this.addr().wrapping_sub(0x48);
    e.call(0x0051_6210, &args![base, flags]).u32()
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0051_7610, fn_00517610(Ptr)),
        entry!(0x0051_73e0, fn_005173e0(Ptr) -> Ptr),
        entry!(0x0051_75d0, fn_005175d0(Ptr)),
        entry!(0x0051_74d0, fn_005174d0(Ptr, u32) -> Ptr),
        entry!(0x0051_7690, fn_00517690(Ptr) -> u32),
        entry!(0x0051_7670, fn_00517670(Ptr) -> u32),
        entry!(0x0051_7630, fn_00517630(Ptr) -> u32),
        entry!(0x0051_7500, fn_00517500(Ptr)),
        entry!(0x0051_73d0, fn_005173d0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_005173e0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0051_73e0,
            size: 0x90,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_f6f0, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0048_cc50, &[A::T(0x5c)]),
                (0x0047_5350, &[A::T(0x68)]),
                (0x0051_0620, &[A::T(0x70)]),
                (0x0096_a2d0, &[A::T(0x88)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x1c)]),
                (0x0051_75d0, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_b1fc),
                (0x30, 4, 0x0102_b1e4),
                (0x3c, 4, 0x0102_b1c0),
                (0x5c, 4, 0x0102_b1ac),
                (0x68, 4, 0x0102_b198),
                (0x70, 4, 0x0102_b188),
            ],
        });
    }
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
    fn fwd_a03_00517610_passes_this_to_the_callee() {
        let mut e = Engine::new();
        e.register(0x0051_9550, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0051_7610, &args![0x0000_5000u32]);
        assert_eq!(calls_to(&e, 0x0051_9550), vec![vec![0x5000]]);
    }
}

#[cfg(test)]
mod movement_path_tests {
    use crate::prelude::*;

    #[test]
    fn fn_005175d0_zeroes_the_start_values() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0x90);
        e.mem.write(this, &[0xcd; 0x90]);
        e.call(0x0051_75d0, &args![Ptr::<()>::new(this)]);
        let mut expected = vec![0xcd_u8; 0x90];
        expected[0x78..0x84].fill(0);
        expected[0x84] = 0;
        assert_eq!(e.mem.bytes(this, 0x90), expected);
    }
    #[test]
    fn at_005174d0_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0051_7500, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0051_74d0, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0051_74d0)
                .collect();
            let mut expected = vec![(0x0051_7500, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod b036_tests {
    use super::*;

    #[test]
    fn fn_00517690_asks_the_package_target_at_8() {
        let mut e = Engine::new();
        e.register(PACKAGE_TARGET_GET_TARG_TYPE, |_, a| Ret {
            eax: a[0] ^ 0xff,
            ..Ret::default()
        });
        assert_eq!(e.call(0x0051_7690, &args![0x1000u32]).u32(), 0x10f7);
    }

    #[test]
    fn fn_00517670_goes_through_the_member_at_e0() {
        let mut e = Engine::new();
        e.register(PLUS_E0, |_, a| Ret {
            eax: a[0] + 0xe0,
            ..Ret::default()
        });
        e.register(PACKAGE_TARGET_GET_TARG_TYPE, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        assert_eq!(e.call(0x0051_7670, &args![0x1000u32]).u32(), 0x10e8);
    }

    #[test]
    fn fn_00517630_answers_5_without_a_proxy() {
        let mut e = Engine::new();
        e.register(OBJECT_PROXY, |_, a| Ret {
            eax: if a[0] == 0x2000 { 0 } else { 0x3000 },
            ..Ret::default()
        });
        e.register(PLUS_E0, |_, a| Ret {
            eax: a[0] + 0xe0,
            ..Ret::default()
        });
        e.register(PACKAGE_TARGET_GET_TARG_TYPE, |_, a| Ret {
            eax: a[0],
            ..Ret::default()
        });
        assert_eq!(e.call(0x0051_7630, &args![0x2000u32]).u32(), 5);
        assert_eq!(e.call(0x0051_7630, &args![0x1000u32]).u32(), 0x30e8);
    }

    #[test]
    fn fn_00517500_restores_the_vtables_and_destroys_in_reverse() {
        let mut e = Engine::new();
        for addr in [
            0x0051_9550u32,
            0x0046_ffb0,
            0x0047_5380,
            0x0048_a490,
            0x0059_1300,
            0x0050_f740,
        ] {
            e.register(addr, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0x90);
        e.call_log = Some(vec![]);
        e.call(0x0051_7500, &args![this]);
        for (off, vtable) in [
            (0u32, 0x0102_b1fcu32),
            (0x30, 0x0102_b1e4),
            (0x3c, 0x0102_b1c0),
            (0x5c, 0x0102_b1ac),
            (0x68, 0x0102_b198),
            (0x70, 0x0102_b188),
        ] {
            assert_eq!(e.mem.u32(this + off), vtable);
        }
        let calls: Vec<_> = e.call_log.take().unwrap().into_iter().skip(1).collect();
        assert_eq!(
            calls,
            vec![
                (0x0051_9550, vec![this]),
                (0x0046_ffb0, vec![this + 0x88]),
                (0x0047_5380, vec![this + 0x68]),
                (0x0048_a490, vec![this + 0x3c]),
                (0x0059_1300, vec![this + 0x30]),
                (0x0050_f740, vec![this]),
            ]
        );
    }
}

#[cfg(test)]
mod n040_tests {
    use super::*;

    #[test]
    fn fn_005173d0_moves_this_back_to_the_destructor() {
        let mut e = Engine::new();
        e.register(0x0051_6210, |_, a| Ret {
            eax: a[0] + a[1],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0051_73d0, &[0x2048, 1]).u32(), 0x2001);
        let log: Vec<_> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a == 0x0051_6210)
            .collect();
        assert_eq!(log, vec![(0x0051_6210, vec![0x2000, 1])]);
    }
}
