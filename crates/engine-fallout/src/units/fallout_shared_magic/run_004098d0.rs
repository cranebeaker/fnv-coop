//! `fallout shared/magic/run_004098d0` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 004098f0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 004098f0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x1d (0xa4 bytes); `ECX` is the object.
/// The engine map gives it no name (a `range-sub` function), so the class is unconfirmed.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_004098f0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0040_2b40, &args![this]);
    e.call(0x0048_a3d0, &args![base + 0x4c]);
    e.call(0x0040_2c20, &args![base + 0x6c]);
    e.call(0x0048_cc50, &args![base + 0x78]);
    e.call(0x0048_eac0, &args![base + 0x84]);
    e.call(0x0047_9320, &args![base + 0x8c]);
    e.call(0x0047_5350, &args![base + 0x94]);
    e.mem.set_u32(base, 0x0101_3284);
    e.mem.set_u32(base + 0x30, 0x0101_323c);
    e.mem.set_u32(base + 0x3c, 0x0101_3224);
    e.mem.set_u32(base + 0x4c, 0x0101_3200);
    e.mem.set_u32(base + 0x6c, 0x0101_31e0);
    e.mem.set_u32(base + 0x78, 0x0101_31cc);
    e.mem.set_u32(base + 0x84, 0x0101_31b8);
    e.mem.set_u32(base + 0x8c, 0x0101_31a4);
    e.mem.set_u32(base + 0x94, 0x0101_3190);
    e.call(0x004f_15a0, &args![this, 0x1d]);
    e.call(0x0040_9c40, &args![this]);
    this
}

/// The function `fn_004098d0` calls on the member (`005a6b10`).
const CALLEE_005A6B10: u32 = 0x005a_6b10;

// Translated from 004098d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thin wrapper: calls `005a6b10` with `ECX` = `this + 0x34` and passes on nothing (the callee returns no value).
pub fn fn_004098d0(e: &mut Engine, this: Ptr) {
    e.call(CALLEE_005A6B10, &args![this.byte_add(0x34)]);
}

// Translated from 00409b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00409b40` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00409b10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_9b40, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 00409b40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00409b40 - the exception-unwinding frame (FS:[0]) is not translated.
/// Destructor body of the form `fn_004098f0` constructs (the engine map gives it no name;
/// the class is unconfirmed): stores the class's nine vtable pointers back, then runs the
/// component destructors in reverse construction order: `00402ef0` on `this`, `00475380`
/// on `this + 0x94`, `0048eaf0` on `this + 0x84`, `00402c40` on `this + 0x6c`, `0048a490`
/// on `this + 0x4c` and the base destructor `00402bc0` on `this`.
pub fn fn_00409b40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0101_3284);
    e.mem.set_u32(base + 0x30, 0x0101_323c);
    e.mem.set_u32(base + 0x3c, 0x0101_3224);
    e.mem.set_u32(base + 0x4c, 0x0101_3200);
    e.mem.set_u32(base + 0x6c, 0x0101_31e0);
    e.mem.set_u32(base + 0x78, 0x0101_31cc);
    e.mem.set_u32(base + 0x84, 0x0101_31b8);
    e.mem.set_u32(base + 0x8c, 0x0101_31a4);
    e.mem.set_u32(base + 0x94, 0x0101_3190);
    e.call(0x0040_2ef0, &args![this]);
    e.call(0x0047_5380, &args![base + 0x94]);
    e.call(0x0048_eaf0, &args![base + 0x84]);
    e.call(0x0040_2c40, &args![base + 0x6c]);
    e.call(0x0048_a490, &args![base + 0x4c]);
    e.call(0x0040_2bc0, &args![this]);
}

// Translated from 00409c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Start values of the form `fn_004098f0` constructs (its last call): the word
/// at +0x9c becomes -1 and the byte at +0xa0 zero, then `00483710`
/// (`fallout shared/tesform.cpp`, unnamed) runs on `this`.
pub fn fn_00409c40(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr() + 0x9c, 0xffff_ffff);
    e.mem.set_u8(this.addr() + 0xa0, 0);
    e.call(0x0048_3710, &args![this]);
}

// Translated from 00409a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall` on the component at +0x3c of the
/// form `fn_004098f0` constructs): whether the form's flags byte (+0xa0)
/// has flag 2 (`00409a30`).
pub fn fn_00409a10(e: &mut Engine, this: Ptr) -> bool {
    fn_00409a30(e, Ptr::new(this.addr().wrapping_sub(0x3c)), 2)
}

// Translated from 00409a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): whether the flags byte
/// at +0xa0 has a bit of `mask`.
pub fn fn_00409a30(e: &mut Engine, this: Ptr, mask: u32) -> bool {
    u32::from(e.mem.u8(this.addr() + 0xa0)) & mask != 0
}

// Translated from 00409a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`): whether the flags byte at +0xa0
/// lacks flag 1 (`00409a30`).
pub fn fn_00409a50(e: &mut Engine, this: Ptr) -> bool {
    !fn_00409a30(e, this, 1)
}

// Translated from 00409a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 4`): flag 1 of the flags
/// byte at +0xa0 set when the byte `clear` is zero, cleared otherwise
/// (`00409aa0`).
pub fn fn_00409a70(e: &mut Engine, this: Ptr, clear: u8) {
    fn_00409aa0(e, this, 1, u8::from(clear == 0));
}

// Translated from 00409aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map (`thiscall`, `RET 8`): sets (when the byte
/// `set` is non-zero) or clears the bits of `mask` in the flags byte at
/// +0xa0.
pub fn fn_00409aa0(e: &mut Engine, this: Ptr, mask: u32, set: u8) {
    let flags = u32::from(e.mem.u8(this.addr() + 0xa0));
    let flags = if set != 0 {
        flags | mask
    } else {
        flags & !mask
    };
    e.mem.set_u8(this.addr() + 0xa0, flags as u8);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0040_98f0, fn_004098f0(Ptr) -> Ptr),
        entry!(0x0040_98d0, fn_004098d0(Ptr)),
        entry!(0x0040_9b40, fn_00409b40(Ptr)),
        entry!(0x0040_9c40, fn_00409c40(Ptr)),
        entry!(0x0040_9b10, fn_00409b10(Ptr, u32) -> Ptr),
        entry!(0x0040_9a10, fn_00409a10(Ptr) -> bool),
        entry!(0x0040_9a30, fn_00409a30(Ptr, u32) -> bool),
        entry!(0x0040_9a50, fn_00409a50(Ptr) -> bool),
        entry!(0x0040_9a70, fn_00409a70(Ptr, u8)),
        entry!(0x0040_9aa0, fn_00409aa0(Ptr, u32, u8)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_004098f0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_98f0,
            size: 0xa4,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0040_2b40, &[A::T(0x0)]),
                (0x0048_a3d0, &[A::T(0x4c)]),
                (0x0040_2c20, &[A::T(0x6c)]),
                (0x0048_cc50, &[A::T(0x78)]),
                (0x0048_eac0, &[A::T(0x84)]),
                (0x0047_9320, &[A::T(0x8c)]),
                (0x0047_5350, &[A::T(0x94)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x1d)]),
                (0x0040_9c40, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0101_3284),
                (0x30, 4, 0x0101_323c),
                (0x3c, 4, 0x0101_3224),
                (0x4c, 4, 0x0101_3200),
                (0x6c, 4, 0x0101_31e0),
                (0x78, 4, 0x0101_31cc),
                (0x84, 4, 0x0101_31b8),
                (0x8c, 4, 0x0101_31a4),
                (0x94, 4, 0x0101_3190),
            ],
        });
    }

    #[test]
    fn l68d_004098d0_passes_this_plus_34_to_the_callee() {
        let mut e = Engine::new();
        e.register(CALLEE_005A6B10, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0040_98d0, &args![0x4100u32]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == CALLEE_005A6B10).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![0x4100 + 0x34]);
    }
    #[test]
    fn at_00409b10_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_9b40, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_9b10, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_9b10)
                .collect();
            let mut expected = vec![(0x0040_9b40, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }

    #[test]
    fn fn_00409b40_restores_vtables_and_destroys_components() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa4);
        for callee in [
            0x0040_2ef0u32,
            0x0047_5380,
            0x0048_eaf0,
            0x0040_2c40,
            0x0048_a490,
            0x0040_2bc0,
        ] {
            e.register(callee, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        e.call(0x0040_9b40, &args![this]);
        let log: Vec<(u32, Vec<u32>)> = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|c| c.0 != 0x0040_9b40)
            .collect();
        assert_eq!(
            log,
            vec![
                (0x0040_2ef0, vec![this]),
                (0x0047_5380, vec![this + 0x94]),
                (0x0048_eaf0, vec![this + 0x84]),
                (0x0040_2c40, vec![this + 0x6c]),
                (0x0048_a490, vec![this + 0x4c]),
                (0x0040_2bc0, vec![this]),
            ]
        );
        for (off, vtable) in [
            (0x0u32, 0x0101_3284u32),
            (0x30, 0x0101_323c),
            (0x3c, 0x0101_3224),
            (0x4c, 0x0101_3200),
            (0x6c, 0x0101_31e0),
            (0x78, 0x0101_31cc),
            (0x84, 0x0101_31b8),
            (0x8c, 0x0101_31a4),
            (0x94, 0x0101_3190),
        ] {
            assert_eq!(e.mem.u32(this + off), vtable);
        }
    }

    #[test]
    fn fn_00409c40_sets_start_values_then_calls_the_form_helper() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xa4);
        e.mem.set_u8(this + 0xa0, 7);
        e.register(0x0048_3710, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0040_9c40, &args![this]);
        let log = e.call_log.take().unwrap();
        let inner: Vec<_> = log.iter().filter(|c| c.0 == 0x0048_3710).collect();
        assert_eq!(inner.len(), 1);
        assert_eq!(inner[0].1, vec![this]);
        assert_eq!(e.mem.u32(this + 0x9c), 0xffff_ffff);
        assert_eq!(e.mem.u8(this + 0xa0), 0);
    }
}

#[cfg(test)]
mod flag_tests {
    use super::*;

    #[test]
    fn flag_queries() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0xa4);
        e.mem.set_u8(form + 0xa0, 0x02);
        assert!(e.call(0x0040_9a30, &args![form, 0x102u32]).bool());
        assert!(!fn_00409a30(&mut e, Ptr::new(form), 0x101));
        assert!(e.call(0x0040_9a10, &args![form + 0x3c]).bool());
        assert!(e.call(0x0040_9a50, &args![form]).bool());
        e.mem.set_u8(form + 0xa0, 0x01);
        assert!(!fn_00409a10(&mut e, Ptr::new(form + 0x3c)));
        assert!(!fn_00409a50(&mut e, Ptr::new(form)));
    }

    #[test]
    fn flag_updates() {
        let mut e = Engine::new();
        let form = e.mem.alloc(0xa4);
        e.mem.set_u8(form + 0xa0, 0x10);
        e.mem.set_u8(form + 0xa1, 0x77);
        e.call(0x0040_9aa0, &args![form, 0x0306u32, 0x100u32]);
        // The set byte is 0 (only its low byte is read): clear 0x06.
        assert_eq!(e.mem.u8(form + 0xa0), 0x10);
        e.call(0x0040_9aa0, &args![form, 0x0306u32, 1u32]);
        assert_eq!(e.mem.u8(form + 0xa0), 0x16);
        assert_eq!(e.mem.u8(form + 0xa1), 0x77);
        e.call(0x0040_9a70, &args![form, 0x200u32]);
        assert_eq!(e.mem.u8(form + 0xa0), 0x17);
        fn_00409a70(&mut e, Ptr::new(form), 5);
        assert_eq!(e.mem.u8(form + 0xa0), 0x16);
    }
}
