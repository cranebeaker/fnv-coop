//! The floating-point part of the C runtime's start-up (MSVC 2008 LIBCMT, statically linked
//! into FalloutNV.exe 1.4.0.525), translated from the exe: `_fpmath` (`00ec5f4d`, which
//! `__cinit` calls through `_FPinit`), the Pentium `FDIV` test (`_ms_p5_mp_test_fdiv`,
//! `_ms_p5_test_fdiv`) and `__setdefaultprecision` with the `_controlfp_s` / `_control87`
//! chain.
//!
//! The x87 control word is the engine's (`super::x87_control_word` for `FSTCW`,
//! `super::set_x87_control_word` for `FLDCW`: the running thread's word in its TLS block and
//! the FPU register the x87 helpers round with). `MXCSR` is not tracked by the engine: every
//! game thread runs with `0x1f80` (measured; `crate::sse` relies on it), which `STMXCSR`
//! reads here ([`super::MXCSR`]); a load of any other value stops the run.

use super::*;

/// `_fpmath`'s callees and data.
const FP_TEST_FDIV_RESULT: u32 = 0x0126_fd94;
/// `GetModuleHandleA` and `GetProcAddress` (import slots), and the strings `KERNEL32` and
/// `IsProcessorFeaturePresent` `_ms_p5_mp_test_fdiv` passes them.
const IMPORT_GET_MODULE_HANDLE_A: u32 = 0x00fd_f118;
const IMPORT_GET_PROC_ADDRESS: u32 = 0x00fd_f0ac;
const KERNEL32_ANSI: u32 = 0x010f_2a7c;
const IS_PROCESSOR_FEATURE_PRESENT_NAME: u32 = 0x010f_2a60;
/// The Pentium division test's operands (`010f2a50` 4195835.0 and `010f2a58` 3145727.0, the
/// pair the original Pentium divided wrongly).
const FDIV_DIVIDEND: u32 = 0x010f_2a50;
const FDIV_DIVISOR: u32 = 0x010f_2a58;
/// `__sse2_available` (`01270a6c`) and the flag `___set_fpsr_sse2` reads before it keeps the
/// denormals-are-zero bit (`011c2538`).
const SSE2_AVAILABLE: u32 = 0x0127_0a6c;
const DAZ_SUPPORTED: u32 = 0x011c_2538;
/// `__invoke_watson` (`00ec7b08`, five words).
const FP_INVOKE_WATSON: u32 = 0x00ec_7b08;

// Translated from 00ec5f4d (decompiled, FalloutNV.exe 1.4.0.525)
/// `_fpmath` (LIBCMT/fpinit.c; cdecl: whether to set the default precision): fills
/// `_cfltcvt_tab` (`__cfltcvt_init`), keeps the Pentium `FDIV` test's answer at `0126fd94`
/// (`_adjust_fdiv`), sets the default precision when asked (`__setdefaultprecision`) and
/// clears the x87 exception flags (`FNCLEX`; the engine keeps no exception flags, so there is
/// nothing to clear).
pub fn fpmath(e: &mut Engine, init_precision: u32) {
    super::scanf::cfltcvt_init(e);
    let fdiv = ms_p5_mp_test_fdiv(e);
    e.mem.set_u32(FP_TEST_FDIV_RESULT, fdiv);
    if init_precision != 0 {
        setdefaultprecision(e);
    }
}

// Translated from 00ed2518 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_ms_p5_mp_test_fdiv` (LIBCMT/testfdiv.c; no arguments): asks Windows
/// `IsProcessorFeaturePresent(PF_FLOATING_POINT_PRECISION_ERRATA)` when `KERNEL32` has it
/// (`GetModuleHandleA`, `GetProcAddress`) and returns its answer (EAX); otherwise
/// [`ms_p5_test_fdiv`].
pub fn ms_p5_mp_test_fdiv(e: &mut Engine) -> u32 {
    let module = e.call(IMPORT_GET_MODULE_HANDLE_A, &[KERNEL32_ANSI]).u32();
    if module != 0 {
        let present = e
            .call(
                IMPORT_GET_PROC_ADDRESS,
                &[module, IS_PROCESSOR_FEATURE_PRESENT_NAME],
            )
            .u32();
        if present != 0 {
            return e.call(present, &[0]).u32();
        }
    }
    ms_p5_test_fdiv(e)
}

// Translated from 00ed24da (disassembled, FalloutNV.exe 1.4.0.525)
/// `_ms_p5_test_fdiv` (LIBCMT/testfdiv.c; no arguments): computes `x - (x / y) * y` for the
/// Pentium's bad pair on the x87 (at the running thread's precision, stored as a double) and
/// returns 1 when the result is above 1.0 (`FCOMP` with `C0`/`C2` tested: unordered gives 0),
/// else 0.
pub fn ms_p5_test_fdiv(e: &mut Engine) -> u32 {
    let divisor = x87_load64(e.mem.u64(FDIV_DIVISOR));
    let dividend = x87_load64(e.mem.u64(FDIV_DIVIDEND));
    let quotient = x87_div(dividend, divisor);
    let product = x87_mul(quotient, divisor);
    let rest = x87_sub(dividend, product);
    u32::from(1.0 < rest)
}

// Translated from 00ed24af (disassembled, FalloutNV.exe 1.4.0.525)
/// `__setdefaultprecision` (LIBCMT/fp8.c; no arguments): `_controlfp_s(NULL, _PC_53,
/// _MCW_PC)`; a failure calls `__invoke_watson`.
pub fn setdefaultprecision(e: &mut Engine) {
    if controlfp_s(e, 0, 0x1_0000, 0x3_0000) != 0 {
        e.call(FP_INVOKE_WATSON, &[0, 0, 0, 0, 0]);
    }
}

// Translated from 00ee1a98 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_controlfp_s` (LIBCMT/contrlfp.c; cdecl: current out, new, mask): the mask without
/// `_EM_DENORMAL` (0x80000); new bits under it outside the valid ones (`0xfcf0fce0`) are an
/// invalid parameter (the current state stored when asked, `EINVAL`, the handler, 0x16).
/// Otherwise `_control87(new, mask)`, stored at `current` when given, and 0.
pub fn controlfp_s(e: &mut Engine, current: u32, new: u32, mask: u32) -> u32 {
    let mask = mask & 0xfff7_ffff;
    if new & mask & 0xfcf0_fce0 != 0 {
        if current != 0 {
            let state = control87(e, 0, 0);
            e.mem.set_u32(current, state);
        }
        let slot = errno(e);
        e.mem.set_u32(slot.addr(), 0x16);
        fn_00ec7c30(e, 0, 0, 0, 0, 0);
        return 0x16;
    }
    let state = control87(e, new, mask);
    if current != 0 {
        e.mem.set_u32(current, state);
    }
    0
}

/// The x87 control word as `_control87`'s abstract bits (inlined twice): the exception masks
/// (invalid 0x10, zero-divide 8, overflow 4, underflow 2, inexact 1, denormal 0x80000), the
/// rounding (0x100 down, 0x200 up, 0x300 chop), the precision (0x20000 for 24 bits, 0x10000
/// for 53, none for 64 or the reserved value) and infinity control (0x40000).
fn x87_abstract(word: u32) -> u32 {
    let mut a = 0;
    if word & 1 != 0 {
        a = 0x10;
    }
    if word & 4 != 0 {
        a |= 8;
    }
    if word & 8 != 0 {
        a |= 4;
    }
    if word & 0x10 != 0 {
        a |= 2;
    }
    if word & 0x20 != 0 {
        a |= 1;
    }
    if word & 2 != 0 {
        a |= 0x8_0000;
    }
    match word & 0xc00 {
        0x400 => a |= 0x100,
        0x800 => a |= 0x200,
        0xc00 => a |= 0x300,
        _ => {}
    }
    match word & 0x300 {
        0 => a |= 0x2_0000,
        0x200 => a |= 0x1_0000,
        _ => {}
    }
    if word & 0x1000 != 0 {
        a |= 0x4_0000;
    }
    a
}

/// `MXCSR` as `_control87`'s abstract bits (inlined twice): the exception masks (0x80
/// invalid, 0x200 zero-divide, 0x400 overflow, 0x800 underflow, 0x1000 inexact, 0x100
/// denormal), the rounding (0x2000, 0x4000, 0x6000) and the denormal control (0x40 DAZ:
/// 0x2000000, 0x8000 FTZ: 0x3000000, both: 0x1000000).
fn sse_abstract(mxcsr: u32) -> u32 {
    let mut a = 0;
    if (mxcsr as u8 as i8) < 0 {
        a = 0x10;
    }
    if mxcsr & 0x200 != 0 {
        a |= 8;
    }
    if mxcsr & 0x400 != 0 {
        a |= 4;
    }
    if mxcsr & 0x800 != 0 {
        a |= 2;
    }
    if mxcsr & 0x1000 != 0 {
        a |= 1;
    }
    if mxcsr & 0x100 != 0 {
        a |= 0x8_0000;
    }
    match mxcsr & 0x6000 {
        0x2000 => a |= 0x100,
        0x4000 => a |= 0x200,
        0x6000 => a |= 0x300,
        _ => {}
    }
    match mxcsr & 0x8040 {
        0x40 => a |= 0x200_0000,
        0x8000 => a |= 0x300_0000,
        0x8040 => a |= 0x100_0000,
        _ => {}
    }
    a
}

/// `STMXCSR`: the engine's `MXCSR` (every game thread's, see the module documentation).
fn stmxcsr() -> u32 {
    MXCSR
}

// Translated from 00ee5bf0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_control87` (LIBCMT/ieee87.c; cdecl: new, mask): reads the x87 control word (`FSTCW`) as
/// abstract bits; when `(old & !mask) | (new & mask)` differs, loads it (`_hw_cw`, `FLDCW`)
/// and reads it back. With `__sse2_available` the same for `MXCSR` under the mask's SSE bits
/// (`0x308031f`: `___hw_cw_sse2`, `___set_fpsr_sse2`), and the answer is both states or'd,
/// with `_EM_AMBIGUOUS` (0x80000000) when they differ in the shared bits (`0x8031f`).
/// Without SSE2 the answer is the x87 state.
pub fn control87(e: &mut Engine, new: u32, mask: u32) -> u32 {
    let old = x87_abstract(u32::from(x87_control_word(e)));
    let mut x87 = (!mask & old) | (new & mask);
    if x87 != old {
        let word = hw_cw(e, x87) & 0xffff;
        set_x87_control_word(e, word as u16);
        x87 = x87_abstract(u32::from(x87_control_word(e)));
    }
    if e.mem.u32(SSE2_AVAILABLE) == 0 {
        return x87;
    }
    let sse_mask = mask & 0x308_031f;
    let mut sse = sse_abstract(stmxcsr());
    let wanted = (!sse_mask & sse) | (sse_mask & new);
    if wanted != sse {
        let mxcsr = hw_cw_sse2(e, wanted);
        set_fpsr_sse2(e, mxcsr);
        sse = sse_abstract(stmxcsr());
    }
    let mut result = sse | x87;
    if (sse ^ x87) & 0x8_031f != 0 {
        result |= 0x8000_0000;
    }
    result
}

// Translated from 00ee5ac2 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_hw_cw` (LIBCMT/ieee87.c; the abstract bits in EBX, passed first): the x87 control word
/// for them (the inverse of the mapping `_control87` reads with).
pub fn hw_cw(_e: &mut Engine, abstract_bits: u32) -> u32 {
    let a = abstract_bits;
    let mut word = 0;
    if a & 0x10 != 0 {
        word += 1;
    }
    if a & 8 != 0 {
        word |= 4;
    }
    if a & 4 != 0 {
        word |= 8;
    }
    if a & 2 != 0 {
        word |= 0x10;
    }
    if a & 1 != 0 {
        word |= 0x20;
    }
    if a & 0x8_0000 != 0 {
        word |= 2;
    }
    match a & 0x300 {
        0x100 => word |= 0x400,
        0x200 => word |= 0x800,
        0x300 => word |= 0xc00,
        _ => {}
    }
    match a & 0x3_0000 {
        0 => word |= 0x300,
        0x1_0000 => word |= 0x200,
        _ => {}
    }
    if a & 0x4_0000 != 0 {
        word |= 0x1000;
    }
    word
}

// Translated from 00ee5b50 (disassembled, FalloutNV.exe 1.4.0.525)
/// `___hw_cw_sse2` (LIBCMT/ieee87.c; the abstract bits in EDX, passed first): the `MXCSR`
/// value for them.
pub fn hw_cw_sse2(_e: &mut Engine, abstract_bits: u32) -> u32 {
    let a = abstract_bits;
    let mut mxcsr = 0;
    if a & 0x10 != 0 {
        mxcsr = 0x80;
    }
    if a & 8 != 0 {
        mxcsr |= 0x200;
    }
    if a & 4 != 0 {
        mxcsr |= 0x400;
    }
    if a & 2 != 0 {
        mxcsr |= 0x800;
    }
    if a & 1 != 0 {
        mxcsr |= 0x1000;
    }
    if a & 0x8_0000 != 0 {
        mxcsr |= 0x100;
    }
    match a & 0x300 {
        0x100 => mxcsr |= 0x2000,
        0x200 => mxcsr |= 0x4000,
        0x300 => mxcsr |= 0x6000,
        _ => {}
    }
    match a & 0x300_0000 {
        0x100_0000 => mxcsr |= 0x8040,
        0x200_0000 => mxcsr |= 0x40,
        0x300_0000 => mxcsr |= 0x8000,
        _ => {}
    }
    mxcsr
}

// Translated from 00ed5b3c (disassembled, FalloutNV.exe 1.4.0.525)
/// `___set_fpsr_sse2` (LIBCMT/ieee87.c; cdecl: the `MXCSR` value): with `__sse2_available`,
/// loads it (`LDMXCSR`), without the denormals-are-zero bit (0x40) unless the processor takes
/// it (`011c2538`). The engine's `MXCSR` is `0x1f80` on every thread; loading that value
/// changes nothing, any other stops the run.
// Translation incomplete: 00ed5b3c - the SEH frame (__SEH_prolog4/__SEH_epilog4) is not translated, and MXCSR values other than 0x1f80 are not modelled.
pub fn set_fpsr_sse2(e: &mut Engine, value: u32) {
    if e.mem.u32(SSE2_AVAILABLE) == 0 {
        return;
    }
    let loaded = if value & 0x40 != 0 && e.mem.u32(DAZ_SUPPORTED) != 0 {
        value
    } else {
        value & 0xffff_ffbf
    };
    assert!(
        loaded == MXCSR,
        "LDMXCSR {loaded:#x}: MXCSR values other than {MXCSR:#x} are not modelled"
    );
}

/// This file's registrations (added to [`super::funcs`]).
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_5f4d, fpmath(u32)),
        entry!(0x00ed_2518, ms_p5_mp_test_fdiv() -> u32),
        entry!(0x00ed_24da, ms_p5_test_fdiv() -> u32),
        entry!(0x00ed_24af, setdefaultprecision()),
        entry!(0x00ee_1a98, controlfp_s(u32, u32, u32) -> u32),
        entry!(0x00ee_5bf0, control87(u32, u32) -> u32),
        entry!(0x00ee_5ac2, hw_cw(u32) -> u32),
        entry!(0x00ee_5b50, hw_cw_sse2(u32) -> u32),
        entry!(0x00ed_5b3c, set_fpsr_sse2(u32)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::platform::loader;

    /// An engine with the imports `_ms_p5_mp_test_fdiv` calls answered by the platform's
    /// loader, the Pentium test's operands, and errno in its static slot.
    fn engine() -> Engine {
        let mut e = Engine::new();
        e.map(0x0126_f000, 0x2000);
        e.map(0x011c_1000, 0x2000);
        e.map(0x010f_2000, 0x1000);
        e.mem.set_cstr(KERNEL32_ANSI, b"KERNEL32");
        e.mem.set_cstr(
            IS_PROCESSOR_FEATURE_PRESENT_NAME,
            b"IsProcessorFeaturePresent",
        );
        e.mem.set_u64(FDIV_DIVIDEND, 4_195_835f64.to_bits());
        e.mem.set_u64(FDIV_DIVISOR, 3_145_727f64.to_bits());
        e.register(
            IMPORT_GET_MODULE_HANDLE_A,
            entry!(0, loader::get_module_handle_a(Ptr) -> u32).1,
        );
        e.register(
            IMPORT_GET_PROC_ADDRESS,
            entry!(0, loader::get_proc_address(u32, u32) -> u32).1,
        );
        e.register(GETPTD_NOEXIT, |_, _| 0u32.into_ret());
        e
    }

    #[test]
    fn the_fdiv_test_asks_windows_and_falls_back_to_dividing() {
        let mut e = engine();
        assert_eq!(ms_p5_mp_test_fdiv(&mut e), 0);
        // Without the procedure: the division itself, which a correct FPU gets right.
        assert_eq!(ms_p5_test_fdiv(&mut e), 0);
        e.register(IMPORT_GET_MODULE_HANDLE_A, |_, _| 0u32.into_ret());
        assert_eq!(ms_p5_mp_test_fdiv(&mut e), 0);
    }

    #[test]
    fn control87_maps_the_control_word_both_ways() {
        let mut e = engine();
        set_x87_control_word(&mut e, 0x027f);
        // 0x027f: all masked, nearest, 53 bits.
        assert_eq!(control87(&mut e, 0, 0), 0x9_001f);
        // 24 bits, chop: loaded as 0x0c7f and read back.
        assert_eq!(control87(&mut e, 0x2_0300, 0x3_0300), 0x8_031f | 0x2_0000);
        // `_hw_cw` builds 0x0c3f (it never sets the reserved bit 6); the FPU keeps bit 6 set, so
        // `FSTCW` reads 0x0c7f (`set_x87_control_word`, measured).
        assert_eq!(x87_control_word(&mut e), 0x0c7f);
        assert_eq!(crate::x87::control_word(), 0x0c7f);
        for a in [0x9_001f, 0x8_031f | 0x2_0000, 0x4_0115, 0x0_0200] {
            assert_eq!(x87_abstract(hw_cw(&mut e, a)), a);
        }
        // With SSE2 the answer joins MXCSR's state (0x1f80: all masked, nearest).
        e.mem.set_u32(SSE2_AVAILABLE, 1);
        set_x87_control_word(&mut e, 0x027f);
        assert_eq!(control87(&mut e, 0, 0), 0x9_001f);
        assert_eq!(sse_abstract(hw_cw_sse2(&mut e, 0x8_001f)), 0x8_001f);
        assert_eq!(hw_cw_sse2(&mut e, 0x8_001f), 0x1f80);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn controlfp_s_checks_its_bits() {
        let mut e = engine();
        set_x87_control_word(&mut e, 0x007f);
        let out = e.mem.alloc(4);
        assert_eq!(controlfp_s(&mut e, out, 0x1_0000, 0x3_0000), 0);
        assert_eq!(x87_control_word(&mut e), 0x027f);
        assert_eq!(e.mem.u32(out), 0x9_001f);
        // A bit outside the valid ones: EINVAL through the handler.
        e.mem.set_u32(0x0126_fd9c, 0x0077_0000);
        e.register(DECODE_POINTER, |_, a| a[0].into_ret());
        e.register(0x0077_0000, |_, _| Ret::default());
        assert_eq!(controlfp_s(&mut e, out, 0x20, 0x20), 0x16);
        assert_eq!(e.mem.u32(ERRNO_FALLBACK), 0x16);
        assert_eq!(e.mem.u32(out), 0x9_001f);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }
}
