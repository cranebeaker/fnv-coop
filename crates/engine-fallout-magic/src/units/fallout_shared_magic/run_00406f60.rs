//! `fallout shared/magic/run_00406f60` (Xbox PDB source unit), subsystem `fallout shared/magic`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The `va_list` form of the formatter: `__thiscall` on the string object,
/// arguments (format, `va_list`), `RET 8`. It formats into a 1 KiB buffer
/// and stores the text in the string (`004037f0`).
const FORMAT_VA_LIST: u32 = 0x0040_6f90;
/// `_vsprintf` (LIBCMT, `00ec6cc4`), `cdecl`: (buffer, format, `va_list`),
/// returns the number of characters written. C runtime library code: the
/// lead's `crt.rs` provides it, it is not translated here.
const VSPRINTF: u32 = 0x00ec_6cc4;

/// `004037f0(this, text, 0)`: stores the text in the string object (the
/// `BSString` assignment; not translated here).
const STRING_ASSIGN: u32 = 0x0040_37f0;

// Translated from 00406f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `printf`-style string formatter (`cdecl`): `string` is the string
/// object that receives the text, `format` the format string, and
/// `variable_arguments` the words the caller pushed after it. They are copied
/// to memory because the `va_list` handed to `00406f90` is the address of
/// the first one. Returns what `00406f90` returns (`EAX`).
pub fn fn_00406f60(e: &mut Engine, string: Ptr, format: Ptr, variable_arguments: &[u32]) -> u32 {
    let list = e.mem.alloc(4 * variable_arguments.len().max(1) as u32);
    for (i, word) in variable_arguments.iter().enumerate() {
        e.mem.set_u32(list + 4 * i as u32, *word);
    }
    let result = e.call(FORMAT_VA_LIST, &args![string, format, list]).u32();
    e.mem.free(list);
    result
}

/// The uniform form of [`fn_00406f60`]: it takes any number of argument
/// words, so it cannot go through `entry!`. The first two words are the
/// string and the format; the rest are the variable arguments.
fn fn_00406f60_uniform(e: &mut Engine, a: &[u32]) -> Ret {
    fn_00406f60(e, Ptr::new(a[0]), Ptr::new(a[1]), &a[2..]).into_ret()
}

// Translated from 00407000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The game's `vsprintf` wrapper (`cdecl`): hands (`buffer`, `format`,
/// `va_list`) to the runtime's `_vsprintf` and leaves its result in `EAX`
/// (the decompiler shows `void`, but `00406f90` stores the register as its
/// own result, so it is returned here).
pub fn fn_00407000(e: &mut Engine, buffer: Ptr, format: Ptr, arguments: Ptr) -> u32 {
    e.call(VSPRINTF, &args![buffer, format, arguments]).u32()
}

// Translated from 00406f90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `va_list` string formatter (`__thiscall`, `RET 8`): formats `format`
/// with the `va_list` at `arguments` into a 0x400-byte stack buffer with the
/// `vsprintf` wrapper `00407000`, stores the text in the string object `this`
/// (`004037f0(this, buffer, 0)`) and returns the character count.
// Translation incomplete: 00406f90 - the /GS stack-cookie check (00ec408c) is not translated.
pub fn fn_00406f90(e: &mut Engine, this: Ptr, format: Ptr, arguments: Ptr) -> u32 {
    e.with_stack(0x400, |e, buffer| {
        let count = fn_00407000(e, buffer, format, arguments);
        e.call(STRING_ASSIGN, &args![this, buffer, 0u32]);
        count
    })
}

// Translated from 00407020 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00407020 - the exception-unwinding frame (FS:[0]) and the security-cookie bookkeeping are not translated.
/// Constructor of the form `CreateFormOfType` (`00465110`) makes for form type 0x10 (0xb0 bytes); `ECX` is the object.
/// The engine map gives it no name; the RTTI name of the vtable it stores at `+0` (`0x0101_2834`) is `EffectSetting`, so this is the constructor of `EffectSetting`.
/// It runs the base and component constructors by address, stores the vtable pointers and sets the fields to their start values. It sets `+0x5c` to 0.0 and `+0x74` to 1.0, and fills `+0x90` and `+0x94` from the floats the settings at `011c36c0` and `011c37e8` give (`00403e20` returns a pointer to the value).
/// The exception-unwinding frame is not translated.
pub fn fn_00407020(e: &mut Engine, this: Ptr) -> Ptr {
    let base = this.addr();
    e.call(0x0048_3370, &args![this]);
    e.call(0x0048_8f50, &args![base + 0x18]);
    e.call(0x0048_2c50, &args![base + 0x30]);
    e.call(0x0040_2d00, &args![base + 0x38]);
    e.call(0x0040_2c20, &args![base + 0x44]);
    e.mem.set_u32(base, 0x0101_2834);
    e.mem.set_u32(base + 0x18, 0x0101_280c);
    e.mem.set_u32(base + 0x30, 0x0101_27f4);
    e.mem.set_u32(base + 0x38, 0x0101_27e0);
    e.mem.set_u32(base + 0x44, 0x0101_27c0);
    e.call(0x0096_a2d0, &args![base + 0xa0]);
    e.mem.set_u32(base + 0x58, 0x0);
    e.mem.set_f32(base + 0x5c, 0.0);
    e.mem.set_u32(base + 0x60, 0x0);
    e.mem.set_u32(base + 0x9c, 0xffff_ffff);
    e.mem.set_u32(base + 0x64, 0xffff_ffff);
    e.mem.set_u32(base + 0x68, 0xffff_ffff);
    e.mem.set_u16(base + 0x6c, 0x0);
    e.mem.set_u32(base + 0x70, 0x0);
    e.mem.set_u32(base + 0x78, 0x0);
    e.mem.set_u32(base + 0x7c, 0x0);
    e.mem.set_f32(base + 0x74, 1.0);
    e.mem.set_u32(base + 0x80, 0x0);
    e.mem.set_u32(base + 0x84, 0x0);
    e.mem.set_u32(base + 0x88, 0x0);
    e.mem.set_u32(base + 0x8c, 0x0);
    let v7 = e.call(0x0040_3e20, &args![0x011c_36c0u32]).u32();
    let f1 = e.mem.f32(v7);
    e.mem.set_f32(base + 0x90, f1);
    let v8 = e.call(0x0040_3e20, &args![0x011c_37e8u32]).u32();
    let f2 = e.mem.f32(v8);
    e.mem.set_f32(base + 0x94, f2);
    e.mem.set_u32(base + 0x98, 0x0);
    e.mem.set_u32(base + 0xa8, 0x0);
    e.mem.set_u32(base + 0xac, 0x0);
    e.mem.set_u32(base + 0x50, 0x0);
    e.mem.set_u32(base + 0x54, 0x0);
    e.call(0x004f_15a0, &args![this, 0x10u32]);
    this
}

// Translated from 00407200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00407230` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00407200(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0040_7230, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        (0x00406f60, fn_00406f60_uniform as AbiFn),
        entry!(0x00406f90, fn_00406f90(Ptr, Ptr, Ptr) -> u32),
        entry!(0x00407000, fn_00407000(Ptr, Ptr, Ptr) -> u32),
        entry!(0x0040_7020, fn_00407020(Ptr) -> Ptr),
        entry!(0x0040_7200, fn_00407200(Ptr, u32) -> Ptr),
        entry!(0x0040_7230, fn_00407230(Ptr)),
    ]
}

/// The destructor steps of `fn_00407230`, in the exe's order, with the offset of `this` each
/// runs on (all `thiscall`, no stack words): `00483710` (unit `tesform.cpp`), the chained
/// container's `0046ffb0`, `00402c40`, `00591300`, `00489070` (unit
/// `tesmodel.cpp`) and `TESForm::~TESForm` (`00483630`).
const EFFECT_SETTING_DESTRUCT_STEPS: [(u32, u32); 6] = [
    (0x0048_3710, 0),
    (0x0046_ffb0, 0xa0),
    (0x0040_2c40, 0x44),
    (0x0059_1300, 0x38),
    (0x0048_9070, 0x18),
    (0x0048_3630, 0),
];

// Translated from 00407230 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00407230 - the C++ exception frame (FS:[0] chain, unwind states 4 to 0 for the members and the base) is not translated.
/// Unnamed in the engine map (`thiscall`, no stack words): the destructor body of
/// `EffectSetting` (the RTTI name of the vtable `01012834` it stores at +0, as the
/// constructor [`fn_00407020`] does). Stores the five vtables the constructor stores (+0,
/// +0x18, +0x30, +0x38, +0x44), then runs `00483710` on `this`, the member destructors on
/// +0xa0, +0x44, +0x38 and +0x18 and `TESForm::~TESForm` on `this`.
pub fn fn_00407230(e: &mut Engine, this: Ptr) {
    let base = this.addr();
    e.mem.set_u32(base, 0x0101_2834);
    e.mem.set_u32(base + 0x18, 0x0101_280c);
    e.mem.set_u32(base + 0x30, 0x0101_27f4);
    e.mem.set_u32(base + 0x38, 0x0101_27e0);
    e.mem.set_u32(base + 0x44, 0x0101_27c0);
    for (step, offset) in EFFECT_SETTING_DESTRUCT_STEPS {
        e.call(step, &args![base + offset]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn passes_a_va_list_over_the_extra_words() {
        let mut e = Engine::new();
        // The double reads the words the va_list points at.
        e.register(FORMAT_VA_LIST, |e, a| {
            assert_eq!((a[0], a[1]), (0x1000, 0x2000));
            Ret {
                eax: e.mem.u32(a[2]) + e.mem.u32(a[2] + 8),
                ..Ret::default()
            }
        });
        let r = e.call(0x0040_6f60, &args![0x1000u32, 0x2000u32, 5u32, 6u32, 7u32]);
        assert_eq!(r.u32(), 12);
    }

    #[test]
    fn the_vsprintf_wrapper_forwards_its_three_words_and_returns_the_count() {
        let mut e = Engine::new();
        e.register(VSPRINTF, |_, a| Ret {
            eax: a[0] + a[1] + a[2],
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let r = e.call(0x0040_7000, &args![100u32, 20u32, 3u32]);
        assert_eq!(r.u32(), 123);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[1], (VSPRINTF, vec![100, 20, 3]));
    }

    #[test]
    fn the_va_list_formatter_stores_the_text_and_returns_the_count() {
        let mut e = Engine::new();
        e.register(VSPRINTF, |e, a| {
            assert_eq!((a[1], a[2]), (0x3000, 0x4000));
            e.mem.write(a[0], b"hi\0");
            Ret {
                eax: 2,
                ..Ret::default()
            }
        });
        e.register(STRING_ASSIGN, |e, a| {
            assert_eq!(a[2], 0);
            assert_eq!(e.mem.bytes(a[1], 3), b"hi\0");
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let r = e.call(0x0040_6f90, &args![0x2000u32, 0x3000u32, 0x4000u32]);
        assert_eq!(r.u32(), 2);
        let log = e.call_log.take().unwrap();
        assert_eq!(log[2].0, STRING_ASSIGN);
        assert_eq!(log[2].1[0], 0x2000, "the string object is this");
        assert_eq!(log[1].1[1..], [0x3000, 0x4000], "format and va_list");
    }

    #[test]
    fn without_variable_arguments_the_list_is_still_a_valid_address() {
        let mut e = Engine::new();
        e.register(FORMAT_VA_LIST, |_, a| Ret {
            eax: a[2],
            ..Ret::default()
        });
        assert_ne!(e.call(0x0040_6f60, &args![1u32, 2u32]).u32(), 0);
    }
}

#[cfg(test)]
mod ctor_tests {
    use crate::units::crt::ctor_test::{self, A};

    #[test]
    fn fn_00407020_builds_the_object() {
        ctor_test::run(&ctor_test::Case {
            addr: 0x0040_7020,
            size: 0xb0,
            globals: &[(0x2000_2000, 0x0200_0002)],
            returns: &[(0x0040_3e20, 0x2000_2000)],
            calls: &[
                (0x0048_3370, &[A::T(0x0)]),
                (0x0048_8f50, &[A::T(0x18)]),
                (0x0048_2c50, &[A::T(0x30)]),
                (0x0040_2d00, &[A::T(0x38)]),
                (0x0040_2c20, &[A::T(0x44)]),
                (0x0096_a2d0, &[A::T(0xa0)]),
                (0x0040_3e20, &[A::V(0x011c_36c0)]),
                (0x0040_3e20, &[A::V(0x011c_37e8)]),
                (0x004f_15a0, &[A::T(0x0), A::V(0x10)]),
            ],
            stores: &[
                (0x0, 4, 0x0101_2834),
                (0x18, 4, 0x0101_280c),
                (0x30, 4, 0x0101_27f4),
                (0x38, 4, 0x0101_27e0),
                (0x44, 4, 0x0101_27c0),
                (0x58, 4, 0x0),
                (0x5c, 4, 0x0),
                (0x60, 4, 0x0),
                (0x9c, 4, 0xffff_ffff),
                (0x64, 4, 0xffff_ffff),
                (0x68, 4, 0xffff_ffff),
                (0x6c, 2, 0x0),
                (0x70, 4, 0x0),
                (0x78, 4, 0x0),
                (0x7c, 4, 0x0),
                (0x74, 4, 0x3f80_0000),
                (0x80, 4, 0x0),
                (0x84, 4, 0x0),
                (0x88, 4, 0x0),
                (0x8c, 4, 0x0),
                (0x90, 4, 0x0200_0002),
                (0x94, 4, 0x0200_0002),
                (0x98, 4, 0x0),
                (0xa8, 4, 0x0),
                (0xac, 4, 0x0),
                (0x50, 4, 0x0),
                (0x54, 4, 0x0),
            ],
        });
    }
    #[test]
    fn at_00407200_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0040_7230, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0040_7200, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0040_7200)
                .collect();
            let mut expected = vec![(0x0040_7230, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod destructor_tests {
    use super::*;

    #[test]
    fn fn_00407230_stores_the_vtables_then_runs_the_member_destructors() {
        let mut e = Engine::new();
        for (step, _) in EFFECT_SETTING_DESTRUCT_STEPS {
            e.register(step, |_, _| Ret::default());
        }
        let this = e.mem.alloc(0xb0);
        e.call_log = Some(vec![]);
        e.call(0x0040_7230, &args![this]);
        for (offset, vtable) in [
            (0u32, 0x0101_2834u32),
            (0x18, 0x0101_280c),
            (0x30, 0x0101_27f4),
            (0x38, 0x0101_27e0),
            (0x44, 0x0101_27c0),
        ] {
            assert_eq!(e.mem.u32(this + offset), vtable, "{offset:#x}");
        }
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..].to_vec(),
            vec![
                (0x0048_3710, vec![this]),
                (0x0046_ffb0, vec![this + 0xa0]),
                (0x0040_2c40, vec![this + 0x44]),
                (0x0059_1300, vec![this + 0x38]),
                (0x0048_9070, vec![this + 0x18]),
                (0x0048_3630, vec![this]),
            ]
        );
    }
}
