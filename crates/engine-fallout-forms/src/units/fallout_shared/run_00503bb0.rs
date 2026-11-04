//! `fallout shared/run_00503bb0` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).
//!
//! The object is a `TESCasinoChips` (the RTTI name of the vtable stored at
//! `+0`, `0x010263dc`).

#[allow(unused_imports)]
use crate::prelude::*;

/// The scalar deleting destructor `00503bb0` jumps to (`00502850`, `RET 4`).
const SECOND_BASE_DESTRUCTOR: u32 = 0x0050_2850;
/// The scalar deleting destructor `00503bc0` jumps to (`005039f0`, `RET 4`).
const THIRD_BASE_DESTRUCTOR: u32 = 0x0050_39f0;
/// `operator delete` (`00401030`, `cdecl`, one stack word).
const OPERATOR_DELETE: u32 = 0x0040_1030;
/// Sets a string-like object (`004037f0`, `thiscall`, two stack words).
const SET_STRING: u32 = 0x0040_37f0;
/// Destroys the string at `+0x94` (`004037d0`).
const DESTROY_STRING: u32 = 0x0040_37d0;

// Translated from 00503bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: the destructor `00502850` for the base that sits at `+0x3c` (`this`
/// is moved back by `0x3c`, the flags word is passed on). Returns what it
/// returns.
pub fn fn_00503bb0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(
        SECOND_BASE_DESTRUCTOR,
        &args![this.addr().wrapping_sub(0x3c), flags],
    )
    .u32()
}

// Translated from 00503bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: the destructor `005039f0` for the base that sits at `+0x78` (`this`
/// is moved back by `0x78`, the flags word is passed on). Returns what it
/// returns.
pub fn fn_00503bc0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(
        THIRD_BASE_DESTRUCTOR,
        &args![this.addr().wrapping_sub(0x78), flags],
    )
    .u32()
}

// Translated from 00503bd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00503bd0 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x6c (0x9c bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0102_63dc`) is `TESCasinoChips`, so this is the constructor of `TESCasinoChips`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values.
/// The exception-unwinding frame is not translated.
pub fn fn_00503bd0(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0050_e820, &args![this]);
    e.call(0x0040_2d00, &args![base + 0x30]);
    e.call(0x0048_a3d0, &args![base + 0x3c]);
    e.call(0x0040_2c20, &args![base + 0x5c]);
    e.call(0x0047_a130, &args![base + 0x68]);
    e.call(0x0048_e7a0, &args![base + 0x78]);
    e.call(0x0047_5350, &args![base + 0x80]);
    e.call(0x0047_b4f0, &args![base + 0x88]);
    e.mem.set_u32(base, 0x0102_63dc);
    e.mem.set_u32(base + 0x30, 0x0102_63c4);
    e.mem.set_u32(base + 0x3c, 0x0102_63a0);
    e.mem.set_u32(base + 0x5c, 0x0102_6380);
    e.mem.set_u32(base + 0x68, 0x0102_636c);
    e.mem.set_u32(base + 0x78, 0x0102_634c);
    e.mem.set_u32(base + 0x80, 0x0102_6338);
    e.mem.set_u32(base + 0x88, 0x0102_6324);
    e.call(0x0040_37b0, &args![base + 0x94]);
    e.call(0x004f_15a0, &args![this, 0x6cu32]);
    e.call(0x0050_3e70, &args![this]);
    this
}

// Translated from 00503d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor of `TESCasinoChips`: runs the destructor body
/// (`00503d40`), then frees the object (`00401030`) when bit 0 of `flags` is
/// set. Returns `this`. `__thiscall`, one stack word.
pub fn fn_00503d10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_00503d40(e, this);
    if flags & 1 != 0 {
        e.call(OPERATOR_DELETE, &args![this]);
    }
    this
}

// Translated from 00503d40 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00503d40 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Destructor body of `TESCasinoChips`: puts the eight vtables back (`+0`,
/// `+0x30`, `+0x3c`, `+0x5c`, `+0x68`, `+0x78`, `+0x80`, `+0x88`), empties the
/// string at `+0x94` (`00503e70`) and destroys it (`004037d0`), then the
/// members at `+0x88` (`0047b520`), `+0x80` (`00475380`), `+0x78`
/// (`0048e7d0`), `+0x68` (`0047a1a0`), `+0x5c` (`00402c40`) and `+0x3c`
/// (`0048a490`), the component at `+0x30` (`00591300`) and the base
/// (`0050e9f0`). The exception-unwinding frame is not translated.
pub fn fn_00503d40(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0102_63dc);
    e.mem.set_u32(base + 0x30, 0x0102_63c4);
    e.mem.set_u32(base + 0x3c, 0x0102_63a0);
    e.mem.set_u32(base + 0x5c, 0x0102_6380);
    e.mem.set_u32(base + 0x68, 0x0102_636c);
    e.mem.set_u32(base + 0x78, 0x0102_634c);
    e.mem.set_u32(base + 0x80, 0x0102_6338);
    e.mem.set_u32(base + 0x88, 0x0102_6324);
    fn_00503e70(e, this);
    e.call(DESTROY_STRING, &args![base + 0x94]);
    e.call(0x0047_b520, &args![base + 0x88]);
    e.call(0x0047_5380, &args![base + 0x80]);
    e.call(0x0048_e7d0, &args![base + 0x78]);
    e.call(0x0047_a1a0, &args![base + 0x68]);
    e.call(0x0040_2c40, &args![base + 0x5c]);
    e.call(0x0048_a490, &args![base + 0x3c]);
    e.call(0x0059_1300, &args![base + 0x30]);
    e.call(0x0050_e9f0, &args![this]);
}

// Translated from 00503e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Calls `004037f0` on the string at `+0x94` of `this` with two zero words.
/// `__fastcall`.
pub fn fn_00503e70(e: &mut Engine, this: Ptr) {
    e.call(SET_STRING, &args![this.addr() + 0x94, 0u32, 0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00503bb0, fn_00503bb0(Ptr, u32) -> u32),
        entry!(0x00503bc0, fn_00503bc0(Ptr, u32) -> u32),
        entry!(0x00503bd0, fn_00503bd0(Ptr) -> Ptr),
        entry!(0x00503d10, fn_00503d10(Ptr, u32) -> Ptr),
        entry!(0x00503d40, fn_00503d40(Ptr)),
        entry!(0x00503e70, fn_00503e70(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::crt::ctor_test::{self, A};

    fn ret(value: u32) -> Ret {
        Ret {
            eax: value,
            ..Ret::default()
        }
    }

    #[test]
    fn fn_00503bd0_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0050_3bd0,
            size: 0x9c,
            globals: &[],
            returns: &[],
            calls: &[
                (0x0050_e820, &[A::T(0x0)]),
                (0x0040_2d00, &[A::T(0x30)]),
                (0x0048_a3d0, &[A::T(0x3c)]),
                (0x0040_2c20, &[A::T(0x5c)]),
                (0x0047_a130, &[A::T(0x68)]),
                (0x0048_e7a0, &[A::T(0x78)]),
                (0x0047_5350, &[A::T(0x80)]),
                (0x0047_b4f0, &[A::T(0x88)]),
                (0x0040_37b0, &[A::T(0x94)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x6c)]),
                (0x0050_3e70, &[A::T(0x0)]),
            ],
            stores: &[
                (0x0, 4, 0x0102_63dc),
                (0x30, 4, 0x0102_63c4),
                (0x3c, 4, 0x0102_63a0),
                (0x5c, 4, 0x0102_6380),
                (0x68, 4, 0x0102_636c),
                (0x78, 4, 0x0102_634c),
                (0x80, 4, 0x0102_6338),
                (0x88, 4, 0x0102_6324),
            ],
        });
    }

    #[test]
    fn the_thunks_move_this_back_and_pass_the_flags() {
        let mut e = Engine::new();
        e.register(SECOND_BASE_DESTRUCTOR, |_, a| ret(a[0] * 16 + a[1]));
        e.register(THIRD_BASE_DESTRUCTOR, |_, a| ret(a[0] * 16 + a[1]));
        let object = Ptr::<()>::new(0x1000);
        assert_eq!(
            e.call(0x0050_3bb0, &args![object, 1u32]).u32(),
            (0x1000 - 0x3c) * 16 + 1
        );
        assert_eq!(
            e.call(0x0050_3bc0, &args![object, 0u32]).u32(),
            (0x1000 - 0x78) * 16
        );
    }

    #[test]
    fn the_string_is_set_with_two_zero_words() {
        let mut e = Engine::new();
        e.register(SET_STRING, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0050_3e70, &args![Ptr::<()>::new(0x2000)]);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (SET_STRING, vec![0x2094, 0, 0]));
    }

    fn destructor_doubles(e: &mut Engine) {
        for address in [
            SET_STRING,
            DESTROY_STRING,
            OPERATOR_DELETE,
            0x0047_b520,
            0x0047_5380,
            0x0048_e7d0,
            0x0047_a1a0,
            0x0040_2c40,
            0x0048_a490,
            0x0059_1300,
            0x0050_e9f0,
        ] {
            e.register(address, |_, _| Ret::default());
        }
    }

    #[test]
    fn the_destructor_body_restores_vtables_and_destroys_the_members_in_order() {
        let mut e = Engine::new();
        destructor_doubles(&mut e);
        let object = e.mem.alloc(0x9c);
        e.call_log = Some(vec![]);
        e.call(0x0050_3d40, &args![Ptr::<()>::new(object)]);
        let log = e.call_log.take().unwrap();
        let addresses: Vec<u32> = log.iter().map(|(a, _)| *a).collect();
        assert_eq!(
            addresses[1..],
            [
                SET_STRING,
                DESTROY_STRING,
                0x0047_b520,
                0x0047_5380,
                0x0048_e7d0,
                0x0047_a1a0,
                0x0040_2c40,
                0x0048_a490,
                0x0059_1300,
                0x0050_e9f0,
            ]
        );
        assert_eq!(log[1].1, vec![object + 0x94, 0, 0]);
        assert_eq!(log[10].1, vec![object]);
        assert_eq!(e.mem.u32(object), 0x0102_63dc);
        assert_eq!(e.mem.u32(object + 0x88), 0x0102_6324);
    }

    #[test]
    fn the_scalar_deleting_destructor_frees_only_with_bit_0() {
        let mut e = Engine::new();
        destructor_doubles(&mut e);
        let object = e.mem.alloc(0x9c);
        e.call_log = Some(vec![]);
        let result = e.call(0x0050_3d10, &args![Ptr::<()>::new(object), 0u32]);
        assert_eq!(result.u32(), object);
        let log = e.call_log.take().unwrap();
        assert!(!log.iter().any(|(a, _)| *a == OPERATOR_DELETE));
        e.call_log = Some(vec![]);
        let result = e.call(0x0050_3d10, &args![Ptr::<()>::new(object), 3u32]);
        assert_eq!(result.u32(), object);
        let log = e.call_log.take().unwrap();
        assert_eq!(*log.last().unwrap(), (OPERATOR_DELETE, vec![object]));
    }
}
