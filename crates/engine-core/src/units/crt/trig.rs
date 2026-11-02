//! The C runtime's sine, cosine and tangent (MSVC 2008 LIBCMT, statically linked into
//! FalloutNV.exe 1.4.0.525), translated from the exe: the register-argument entries `_CIcos`
//! (`00ec9f70`), `_CIsin` (`00eca0a0`) and `_CItan` (`00eca5d0`), the stack-argument entries
//! `cos` (`00ec9f30`), `sin` (`00eca060`) and `tan` (`00eca590`), their x87 bodies (`00ec9fc8`,
//! `00eca0f8`, `00eca628`) with the stack loaders the stack entries fall into (`00ec9fbf`,
//! `00eca0ef`, `00eca61f`), their SSE2 bodies (`00ed9780`, `00ed9930`, `00edb880` for `ST0`;
//! `00ed9798`, `00ed9948`, `00edb898` for the stack) and the SSE2 cores those reach (`00ed979e`,
//! `00ed994e`, `00edb89e`), and the x87 error machinery the bodies end in that nothing had
//! translated: `00ed2808` (the stored double's exponent, `ZF`), `00ed27ac` (the NaN's error
//! type), `__math_exit` (`00ed282b`), `__startOneArgErrorHandling` (`00ed2737`) and
//! `__87except` (`00ee1b04`). Which is which comes from the code: each entry jumps to its own
//! SSE2 body and calls (or falls into) its own x87 body, and each body's error report names
//! the function (`"cos"` `011c1950`, `"sin"` `011c1960`, `"tan"` `011c1980`).
//!
//! Each entry dispatches as the exe does: with `__use_sse2_mathfcns` (`01270a64`) set and
//! every `MXCSR` and x87 exception masked (`super::sse2_math_available`, the same three tests
//! in the same order) it jumps to its SSE2 body, otherwise it runs its x87 body.
//!
//! x87: `FSIN`, `FCOS`, `FPTAN` and `FPREM1` run through `crate::x87` (the host's FPU, bit for
//! bit). The engine holds `ST0` as an `f64`: the 80-bit result of the instruction is rounded
//! as `FSTP qword` rounds it under the control word of that moment ([`X87Ext::store64`]),
//! which is exact wherever the exe stores it to a double itself (the error path does). The
//! instructions' C2 flag (operand out of range, `|x| >= 2^63`, `crate::x87::X87_STATUS_C2`)
//! is decided from the operand ([`out_of_range`]); `FPREM1` gives its own status word.
//!
//! SSE2: `crate::sse` has single-precision lanes only, so the double-precision instructions
//! (`ADDSD`/`SUBSD`/`MULSD`/`DIVSD` and their packed forms, `ANDPD`, `CVTSD2SI`) are written
//! out here with the instructions' rules ([`sd_op`]): a NaN in the destination wins, quieted,
//! else a NaN in the source, quieted; an invalid operation on numbers gives the default NaN
//! `0xfff8_0000_0000_0000`; numbers round to nearest with denormals kept (`MXCSR` 0x1f80,
//! `super::MXCSR`), which is IEEE `f64` arithmetic. A register lane the code only carries
//! along (the high lane a `MOVLPD`/`MOVSD` keeps) is left out where no later instruction
//! reads it into the result; each core says which. The constant tables are read from the
//! exe's `.rdata` in engine memory at their addresses.
//!
//! Not modelled: the x87 status word's sticky precision flag `__math_exit` tests (see
//! [`math_exit`]) and the `/GS` cookie check of `__87except`. `__raise_exc_ex` (`00ed5255`),
//! which an unhandled exception reaches through `__raise_exc`, is not translated (`math.rs`).

use super::*;

/// `_ctrlfp` (`00ed5abb`), `__fload_withFB` (`00ed27c5`, `ECX` and `EDX` words, giving `EAX`
/// and `ST0`): in `crt.rs`. `__handle_exc` (`00ed5554`), `__raise_exc` (`00ed5531`),
/// `__set_errno_from_matherr` (`00ed5736`) and the default `_matherr` (`00ee0a3a`): in
/// `math.rs`.
const CTRLFP: u32 = 0x00ed_5abb;
const FLOAD_WITH_FB: u32 = 0x00ed_27c5;
const HANDLE_EXC: u32 = 0x00ed_5554;
const RAISE_EXC: u32 = 0x00ed_5531;
const SET_ERRNO_FROM_MATHERR: u32 = 0x00ed_5736;
const DEFAULT_MATHERR: u32 = 0x00ee_0a3a;
/// The functions of this file the exe calls (`CALL`); jumps and fall-throughs are direct.
const EXPONENT_BITS: u32 = 0x00ed_2808;
const NAN_ERROR_TYPE: u32 = 0x00ed_27ac;
const MATH_EXIT: u32 = 0x00ed_282b;
const START_ONE_ARG_ERROR_HANDLING: u32 = 0x00ed_2737;
const EXCEPT_87: u32 = 0x00ee_1b04;
const COS_X87: u32 = 0x00ec_9fc8;
const SIN_X87: u32 = 0x00ec_a0f8;
const TAN_X87: u32 = 0x00ec_a628;
const COS_CORE: u32 = 0x00ed_979e;
const SIN_CORE: u32 = 0x00ed_994e;
const TAN_CORE: u32 = 0x00ed_b89e;

/// The x87 bodies' constants: the double 1.0 `00ed27ac` adds to a signalling NaN
/// (`010f2a90`), the control word they load when the caller's is not `0x027f` (`010f2a98`,
/// `0x027f` in the exe) and the extended modulus `FPREM1` reduces an out-of-range argument by
/// (`010f2a9a`, pi * 2^62 in the exe).
const NAN_ADDEND: u32 = 0x010f_2a90;
const BODY_CONTROL_WORD: u32 = 0x010f_2a98;
const REDUCTION_MODULUS: u32 = 0x010f_2a9a;
/// The extended indefinite NaN an infinite argument gives (`011c1d00`).
const INDEFINITE_NAN: u32 = 0x011c_1d00;
/// The names the bodies report (`"cos"`, `"sin"`, `"tan"`).
const COS_NAME: u32 = 0x011c_1950;
const SIN_NAME: u32 = 0x011c_1960;
const TAN_NAME: u32 = 0x011c_1980;
/// The global `__87except` tests before calling `_matherr`: nonzero skips it (the same word
/// `math.rs`'s `__handle_qnan1` tests; no label in the map).
const MATHERR_SKIP: u32 = 0x011c_2b60;
/// The control word every x87 path compares against (`CMP word ptr [ESP], 0x27f`).
const PLAIN_CONTROL_WORD: u16 = 0x027f;
/// The quiet bit of a double NaN and the default NaN of an invalid SSE2 operation.
const QUIET: u64 = 1 << 51;
const DEFAULT_NAN: u64 = 0xfff8_0000_0000_0000;

/// Which instruction an x87 body runs, with the operation code and name its exits report.
#[derive(Clone, Copy, PartialEq, Debug)]
enum Trig {
    Cos,
    Sin,
    Tan,
}

impl Trig {
    /// `MOV EDX, ..`: the operation code the exits give `__math_exit` and
    /// `__startOneArgErrorHandling`.
    fn operation(self) -> u32 {
        match self {
            Trig::Cos => 0x12,
            Trig::Sin => 0x1e,
            Trig::Tan => 0x20,
        }
    }

    /// `LEA ECX, [..]`: the name.
    fn name(self) -> u32 {
        match self {
            Trig::Cos => COS_NAME,
            Trig::Sin => SIN_NAME,
            Trig::Tan => TAN_NAME,
        }
    }

    /// `FCOS`, `FSIN`, or `FPTAN` then `FSTP ST0` (the 1.0 it pushes popped, the tangent left),
    /// on an operand in range.
    fn run(self, x: X87Ext) -> X87Ext {
        match self {
            Trig::Cos => x87_fcos(x),
            Trig::Sin => x87_fsin(x),
            Trig::Tan => x87_fptan(x),
        }
    }
}

/// C2 after `FSIN`/`FCOS`/`FPTAN` on a finite operand: set when it is out of range (`|x| >=
/// 2^63`, biased exponent `0x403e` or more), and the instruction then leaves `ST0` as it was
/// (`FPTAN` pushes nothing).
fn out_of_range(x: X87Ext) -> bool {
    x.sign_exponent() & 0x7fff >= 0x403e
}

/// The double's low and high words.
fn words(bits: u64) -> (u32, u32) {
    (bits as u32, (bits >> 32) as u32)
}

/// Whether the stored double's exponent is all ones: the `ZF` that `00ed2808`'s and
/// `__fload_withFB`'s `CMP EAX, 0x7ff00000` (or `TEST EAX, 0`) leave.
fn exponent_all_ones(high: u32) -> bool {
    high & 0x7ff0_0000 == 0x7ff0_0000
}

// ---- The entries -----------------------------------------------------------------------

/// The register-argument entries' x87 path (`SUB ESP, 0xc; FST double ptr [ESP]; CALL
/// 00ed2808; CALL <body>; ADD ESP, 0xc; RET`): `ST0` stored as a double (`FST` keeps `ST0` and
/// stores a signalling NaN quiet), its exponent bits and `ZF` from `00ed2808`, then the body
/// with `ST0` and the stored double, whose `ST0` comes back.
fn register_entry_x87(e: &mut Engine, body: u32, st0: f64) -> f64 {
    let stored = x87_load64(st0.to_bits()).to_bits();
    let (low, high) = words(stored);
    let eax = e.call(EXPONENT_BITS, &args![low, high]).u32();
    let zf = exponent_all_ones(high);
    e.call(body, &args![zf, eax, st0, low, high]).f64()
}

// Translated from 00ec9f70 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_CIcos` (no label in the map: named from the body, `FCOS`, and the name `"cos"` its error
/// report passes), `ST0` in, `ST0` out. With the SSE2 routines available it jumps to the SSE2
/// body (`00ed9780`); otherwise it stores `ST0` to the stack, runs `00ed2808` on it and calls
/// the x87 body `00ec9fc8` ([`register_entry_x87`]).
pub fn ci_cos(e: &mut Engine, st0: f64) -> f64 {
    if super::sse2_math_available(e) {
        return ci_cos_sse2(e, st0);
    }
    register_entry_x87(e, COS_X87, st0)
}

// Translated from 00eca0a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_CIsin` (no label in the map: `FSIN`, the name `"sin"`), `ST0` in, `ST0` out: as
/// [`ci_cos`] with the SSE2 body `00ed9930` and the x87 body `00eca0f8`.
pub fn ci_sin(e: &mut Engine, st0: f64) -> f64 {
    if super::sse2_math_available(e) {
        return ci_sin_sse2(e, st0);
    }
    register_entry_x87(e, SIN_X87, st0)
}

// Translated from 00eca5d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `_CItan` (no label in the map: `FPTAN`, the name `"tan"`; the game calls it from
/// `BSCubeMapCamera` and the shader constant set-up), `ST0` in, `ST0` out: as [`ci_cos`] with
/// the SSE2 body `00edb880` and the x87 body `00eca628`.
pub fn ci_tan(e: &mut Engine, st0: f64) -> f64 {
    if super::sse2_math_available(e) {
        return ci_tan_sse2(e, st0);
    }
    register_entry_x87(e, TAN_X87, st0)
}

// Translated from 00ec9f30 (disassembled, FalloutNV.exe 1.4.0.525)
/// `cos` (LIBCMT; no label in the map), cdecl, the double on the stack, the result in `ST0`.
/// With the SSE2 routines available it jumps to `00ed9798` ([`cos_sse2`]), otherwise to
/// `00ec9fbf` ([`fn_00ec9fbf`]), which loads the argument and falls into the x87 body.
pub fn cos(e: &mut Engine, x: f64) -> f64 {
    if super::sse2_math_available(e) {
        return cos_sse2(e, x);
    }
    fn_00ec9fbf(e, x)
}

// Translated from 00eca060 (disassembled, FalloutNV.exe 1.4.0.525)
/// `sin` (LIBCMT; no label in the map), cdecl, the double on the stack, the result in `ST0`:
/// as [`cos`] with `00ed9948` and `00eca0ef`.
pub fn sin(e: &mut Engine, x: f64) -> f64 {
    if super::sse2_math_available(e) {
        return sin_sse2(e, x);
    }
    fn_00eca0ef(e, x)
}

// Translated from 00eca590 (disassembled, FalloutNV.exe 1.4.0.525)
/// `tan` (LIBCMT; no label in the map), cdecl, the double on the stack, the result in `ST0`:
/// as [`cos`] with `00edb898` and `00eca61f`.
pub fn tan(e: &mut Engine, x: f64) -> f64 {
    if super::sse2_math_available(e) {
        return tan_sse2(e, x);
    }
    fn_00eca61f(e, x)
}

// ---- The x87 paths ----------------------------------------------------------------------

/// The stack loaders (`LEA EDX, [ESP + 4]; CALL 00ed27c5`, then the body that follows):
/// `__fload_withFB` loads the stack double into `ST0` and answers its exponent bits in `EAX`
/// (`ZF` set when they are all ones), and the body runs on it with the stack double as it is.
fn stack_entry_x87(e: &mut Engine, trig: Trig, x: f64) -> f64 {
    let bits = x.to_bits();
    let (low, high) = words(bits);
    let loaded = e.with_stack(8, |e, slot| {
        e.mem.set_u64(slot.addr(), bits);
        // ECX is not read.
        e.call(FLOAD_WITH_FB, &[0, slot.addr()])
    });
    let zf = exponent_all_ones(high);
    x87_body(e, trig, zf, loaded.eax, loaded.st0, low, high)
}

// Translated from 00ec9fbf (disassembled, FalloutNV.exe 1.4.0.525)
/// The x87 path of `cos` (Ghidra makes it a function of its own: `cos` jumps here, and so does
/// the SSE2 core `00ed979e` for an argument it does not take), cdecl, the double on the stack:
/// loads it (`__fload_withFB`) and falls into the x87 body `00ec9fc8` ([`stack_entry_x87`]).
pub fn fn_00ec9fbf(e: &mut Engine, x: f64) -> f64 {
    stack_entry_x87(e, Trig::Cos, x)
}

// Translated from 00eca0ef (disassembled, FalloutNV.exe 1.4.0.525)
/// The x87 path of `sin` (reached from `sin` and from the SSE2 core `00ed994e`): as
/// [`fn_00ec9fbf`] with the body `00eca0f8`.
pub fn fn_00eca0ef(e: &mut Engine, x: f64) -> f64 {
    stack_entry_x87(e, Trig::Sin, x)
}

// Translated from 00eca61f (disassembled, FalloutNV.exe 1.4.0.525)
/// The x87 path of `tan` (reached from `tan` and from the SSE2 core `00edb89e`): as
/// [`fn_00ec9fbf`] with the body `00eca628`.
pub fn fn_00eca61f(e: &mut Engine, x: f64) -> f64 {
    stack_entry_x87(e, Trig::Tan, x)
}

/// `00ed281e`, the exit with the error switch set: the saved control word loaded back unless
/// it is `0x027f`, then `POP EDX; RET` with `ST0` as it is.
fn restore_control_word(e: &mut Engine, saved: u16) {
    if saved != PLAIN_CONTROL_WORD {
        set_x87_control_word(e, saved);
    }
}

/// The shared x87 body (see [`cos_x87`]).
fn x87_body(e: &mut Engine, trig: Trig, zf: bool, eax: u32, st0: f64, low: u32, high: u32) -> f64 {
    // PUSH EDX; FSTCW word ptr [ESP]: the caller's control word in the pushed slot (its upper
    // half keeps EDX's, which nothing reads).
    let saved = x87_control_word(e);
    let switch_set = |e: &Engine| e.global::<u32>(super::MATH_ERROR_SWITCH) != 0;
    if !zf {
        if saved != PLAIN_CONTROL_WORD {
            let word = e.mem.u16(BODY_CONTROL_WORD);
            set_x87_control_word(e, word);
        }
        let x = X87Ext::load64(st0);
        // FCOS/FSIN/FPTAN; FSTSW AX; SAHF; JP: C2.
        let result = if out_of_range(x) {
            // FLD tbyte [010f2a9a]; FXCH; FPREM1 until C2 is clear; FSTP ST1; the instruction
            // again (no C2 test after it).
            let modulus = X87Ext(read_extended(e, REDUCTION_MODULUS));
            let mut partial = x;
            loop {
                let (remainder, status) = x87_fprem1(partial, modulus);
                partial = remainder;
                if status & X87_STATUS_C2 == 0 {
                    break;
                }
            }
            trig.run(partial)
        } else {
            trig.run(x)
        };
        let st0 = result.store64();
        if switch_set(e) {
            restore_control_word(e, saved);
            return st0;
        }
        // MOV EDX, op; LEA ECX, [name]; JMP 00ed282b.
        return e
            .call(
                MATH_EXIT,
                &args![
                    trig.operation(),
                    trig.name(),
                    st0,
                    u32::from(saved),
                    0u32,
                    low,
                    high
                ],
            )
            .f64();
    }
    // An infinity or NaN: EAX is the stored double's high word. TEST EAX, 0xfffff; JNZ; CMP
    // dword ptr [ESP + 8], 0 (the low word); JNZ: a NaN goes to 00ed27ac.
    let (error, st0) = if eax & 0xfffff != 0 || low != 0 {
        let answer = e.call(NAN_ERROR_TYPE, &args![eax, st0]);
        (answer.eax, answer.st0)
    } else {
        // FSTP ST0; FLD tbyte [011c1d00]; MOV EAX, 1.
        (1, super::fld_extended_special(e, INDEFINITE_NAN))
    };
    if switch_set(e) {
        restore_control_word(e, saved);
        return st0;
    }
    // MOV EDX, op; LEA ECX, [name]; CALL 00ed2737; POP EDX; RET.
    e.call(
        START_ONE_ARG_ERROR_HANDLING,
        &args![
            error,
            trig.name(),
            trig.operation(),
            st0,
            u32::from(saved),
            0u32,
            low,
            high
        ],
    )
    .f64()
}

/// The 10 bytes of an extended constant (`FLD tbyte ptr [..]`).
fn read_extended(e: &Engine, address: u32) -> [u8; 10] {
    let mut bytes = [0u8; 10];
    e.mem.read(address, &mut bytes);
    bytes
}

// Translated from 00ec9fc8 (disassembled, FalloutNV.exe 1.4.0.525)
/// The x87 body of `cos` (no label; `_CIcos` calls it, `cos`'s loader `00ec9fbf` falls into
/// it). Its inputs are `ZF` (set when the stored double's exponent is all ones), `EAX` (the
/// exponent bits, or the whole high word when they are all ones), `ST0` and the double on the
/// stack (`low`, `high`); `ST0` comes back. It saves the control word (`FSTCW`). A finite
/// argument: with a word other than `0x027f` it loads `010f2a98`; `FCOS`; when C2 says the
/// operand was out of range it is reduced by `FPREM1` against the modulus at `010f2a9a`
/// (repeated until C2 clears) and `FCOS` runs on the remainder. Then, with the error switch
/// (`0126fd90`) set, the saved word is put back (unless `0x027f`) and the cosine returned;
/// otherwise `__math_exit` (`00ed282b`) runs with operation 0x12 and the name `"cos"`. An
/// infinity becomes the indefinite NaN (`011c1d00`) with error type 1; a NaN gets its type and
/// `ST0` from `00ed27ac`; with the switch set the word is put back and that `ST0` returned,
/// otherwise `__startOneArgErrorHandling` (`00ed2737`) reports it.
pub fn cos_x87(e: &mut Engine, zf: bool, eax: u32, st0: f64, low: u32, high: u32) -> f64 {
    x87_body(e, Trig::Cos, zf, eax, st0, low, high)
}

// Translated from 00eca0f8 (disassembled, FalloutNV.exe 1.4.0.525)
/// The x87 body of `sin` (no label; `_CIsin` calls it, `00eca0ef` falls into it): as
/// [`cos_x87`] with `FSIN`, operation 0x1e and the name `"sin"`.
pub fn sin_x87(e: &mut Engine, zf: bool, eax: u32, st0: f64, low: u32, high: u32) -> f64 {
    x87_body(e, Trig::Sin, zf, eax, st0, low, high)
}

// Translated from 00eca628 (disassembled, FalloutNV.exe 1.4.0.525)
/// The x87 body of `tan` (no label; `_CItan` calls it, `00eca61f` falls into it): as
/// [`cos_x87`] with `FPTAN` and `FSTP ST0` (the pushed 1.0 popped; after the reduction too),
/// operation 0x20 and the name `"tan"`.
pub fn tan_x87(e: &mut Engine, zf: bool, eax: u32, st0: f64, low: u32, high: u32) -> f64 {
    x87_body(e, Trig::Tan, zf, eax, st0, low, high)
}

// ---- The x87 error machinery ------------------------------------------------------------

// Translated from 00ed2808 (disassembled, FalloutNV.exe 1.4.0.525)
/// The register entries' exponent test (no label; Ghidra gives the body no name), cdecl, the
/// stored double on the stack (low word, high word): `EAX` is `high & 0x7ff00000`, and when
/// that is all ones the whole high word instead. It leaves `ZF` set exactly then (`CMP EAX,
/// 0x7ff00000`), which the callers take from the high word ([`exponent_all_ones`]).
pub fn fn_00ed2808(_e: &mut Engine, _low: u32, high: u32) -> u32 {
    let exponent = high & 0x7ff0_0000;
    if exponent == 0x7ff0_0000 {
        // 00ed2819: MOV EAX, dword ptr [ESP + 8].
        return high;
    }
    exponent
}

// Translated from 00ed27ac (disassembled, FalloutNV.exe 1.4.0.525)
/// The NaN's error type (no label), with `EAX` the double's high word and `ST0` the NaN; `EAX`
/// and `ST0` come back. A quiet NaN (`EAX & 0x80000`) gives 7 and `ST0` as it is; a signalling
/// one gives 1 and `ST0 + 1.0` (`FADD double ptr [010f2a90]`, which quiets it).
pub fn fn_00ed27ac(e: &mut Engine, eax: u32, st0: f64) -> Ret {
    if eax & 0x8_0000 != 0 {
        return Ret {
            eax: 7,
            st0,
            ..Ret::default()
        };
    }
    let addend = f64::from_bits(e.mem.u64(NAN_ADDEND));
    Ret {
        eax: 1,
        st0: x87_add(st0, addend),
        ..Ret::default()
    }
}

/// The registered form of [`fn_00ed27ac`]: `(EAX, ST0)`.
fn fn_00ed27ac_entry(e: &mut Engine, words: &[u32]) -> Ret {
    let mut i = 0;
    let eax = u32::take(words, &mut i);
    let st0 = f64::take(words, &mut i);
    fn_00ed27ac(e, eax, st0)
}

/// The precision-exception flag (`PE`, bit 5) of the x87 status word as `__math_exit`'s
/// `FSTSW` reads it. The engine keeps no status word (see `math.rs`'s `fn_00ed5ae4`); the flag
/// is sticky, and every inexact x87 result sets it, which every sine, cosine and tangent of a
/// nonzero argument is (and the game's x87 code before it rarely leaves it clear), so the
/// engine takes it as set.
fn status_precision_flag() -> bool {
    true
}

// Translated from 00ed282b (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ed282b - the x87 status word's precision flag (FSTSW AX; AND AX, 0x20) is not modelled: taken as set (status_precision_flag)
/// `__math_exit` (Ghidra's function-ID label), jumped to by the x87 bodies with `EDX` the
/// operation, `ECX` the name and `ST0` the result; the stack holds the saved control word (the
/// body's pushed slot, of which the low word is read), the body's return address and the
/// argument (`low`, `high`). A saved word of `0x027f` returns `ST0` as it is. Otherwise, when
/// the word's precision mask (bit 5) is clear or the status word's precision flag is clear, the
/// word is loaded back (`FLDCW`) and `ST0` returned; when both are set the result is reported
/// as inexact: `__startOneArgErrorHandling` with error type 8 (`MOV EAX, 8`), whose `ST0` comes
/// back.
pub fn math_exit(
    e: &mut Engine,
    operation: u32,
    name: u32,
    st0: f64,
    saved: u32,
    return_address: u32,
    low: u32,
    high: u32,
) -> f64 {
    let word = saved as u16;
    if word == PLAIN_CONTROL_WORD {
        // 00ed2853
        return st0;
    }
    if word & 0x20 == 0 || !status_precision_flag() {
        // 00ed2850: FLDCW word ptr [ESP].
        set_x87_control_word(e, word);
        return st0;
    }
    e.call(
        START_ONE_ARG_ERROR_HANDLING,
        &args![8u32, name, operation, st0, saved, return_address, low, high],
    )
    .f64()
}

// Translated from 00ed2737 (disassembled, FalloutNV.exe 1.4.0.525)
/// `__startOneArgErrorHandling` (Ghidra's function-ID label), with `EAX` the error type, `ECX`
/// the name, `EDX` the operation and `ST0` the result; the stack holds the saved control word
/// (`[EBP+8]`), the body's return address and the argument (`[EBP+0x10]`, `[EBP+0x14]`). It
/// fills an `_exception` at `EBP-0x20` (type, name, the argument, the second argument left as
/// the stack holds it, the result `FSTP`'d as a double: a signalling NaN quiet), calls
/// `__87except(operation, &exception, &saved word)`, loads the `_exception`'s result
/// (`FLD`) and loads the saved word back unless it is `0x027f`.
pub fn start_one_arg_error_handling(
    e: &mut Engine,
    error: u32,
    name: u32,
    operation: u32,
    st0: f64,
    saved: u32,
    _return_address: u32,
    low: u32,
    high: u32,
) -> f64 {
    // The `_exception` at EBP-0x20 and, after it, the slot standing for the saved word at
    // EBP+8 (which `__87except` reads through its address).
    e.with_stack(0x24, |e, block| {
        let exception = block.addr();
        let saved_at = exception + 0x20;
        e.mem.set_u32(saved_at, saved);
        e.mem.set_u32(exception, error);
        // FSTP double ptr [EBP-8].
        e.mem
            .set_u64(exception + 0x18, x87_load64(st0.to_bits()).to_bits());
        e.mem.set_u32(exception + 4, name);
        e.mem.set_u32(exception + 8, low);
        e.mem.set_u32(exception + 0xc, high);
        e.call(EXCEPT_87, &args![operation, exception, saved_at]);
        let result = x87_load64(e.mem.u64(exception + 0x18));
        let word = e.mem.u16(saved_at);
        if word != PLAIN_CONTROL_WORD {
            set_x87_control_word(e, word);
        }
        result
    })
}

// Translated from 00ee1b04 (disassembled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00ee1b04 - the /GS stack cookie check (00ec408c) is not translated
/// `__87except` (Ghidra's function-ID label), cdecl: (operation, the `_exception`, a pointer to
/// the control word). The word (`MOVZX`, `EBP-0x88`) and the `_exception` type decide the
/// exception flags: type 1 and 5 give 8 (invalid), 2 gives 4 (zero divide), 3 gives 0x11
/// (overflow, inexact), 4 gives 0x12 (underflow, inexact), 8 gives 0x10 (inexact); type 7 is
/// changed to 1 and skips to the end, like every other type. With flags, `__handle_exc(flags,
/// &result, word)`; when that answers 0 the local block (`EBP-0x80`, 0x80 bytes) gets for the
/// operations 0x10, 0x16 and 0x1d bits 0-1 set and 2-4 cleared in its word at +0x40 and the
/// second argument (`FLD`/`FSTP`) at +0x30, for any other operation bit 0 of that word
/// cleared, and `__raise_exc(&block, &word, flags, operation, &argument, &result)` runs. Then
/// `_ctrlfp(word, 0xffff)`; unless the type is 8 or the global at `011c2b60` is nonzero, the
/// default `_matherr` (`00ee0a3a`, not the user's) gets the `_exception`; when it answers 0
/// (or was skipped) `__set_errno_from_matherr(type)`.
pub fn except_87(e: &mut Engine, operation: u32, exception: Ptr, control: Ptr) {
    let at = exception.addr();
    // The frame: the word (EBP-0x88), the flags (EBP-0x84), the block (EBP-0x80).
    e.with_stack(0x88, |e, frame| {
        let word_at = frame.addr();
        let block = word_at + 8;
        let word = u32::from(e.mem.u16(control.addr()));
        e.mem.set_u32(word_at, word);
        let flags = match e.mem.u32(at) {
            1 | 5 => Some(8u32),
            2 => Some(4),
            3 => Some(0x11),
            4 => Some(0x12),
            7 => {
                // 00ee1b64: MOV dword ptr [ESI], 1.
                e.mem.set_u32(at, 1);
                None
            }
            8 => Some(0x10),
            _ => None,
        };
        if let Some(flags) = flags {
            e.mem.set_u32(word_at + 4, flags);
            let result = at + 0x18;
            let handled = e.call(HANDLE_EXC, &args![flags, result, word]).u32();
            if handled == 0 {
                if operation == 0x10 || operation == 0x16 || operation == 0x1d {
                    // 00ee1bcc: MOV ECX, [EBP-0x40]; FLD double ptr [ESI+0x10]; AND ECX,
                    // 0xffffffe3; FSTP double ptr [EBP-0x50]; OR ECX, 3.
                    let field = e.mem.u32(block + 0x40);
                    let second = x87_load64(e.mem.u64(at + 0x10)).to_bits();
                    e.mem.set_u64(block + 0x30, second);
                    e.mem.set_u32(block + 0x40, (field & 0xffff_ffe3) | 3);
                } else {
                    // 00ee1bc6: AND dword ptr [EBP-0x40], 0xfffffffe.
                    let field = e.mem.u32(block + 0x40);
                    e.mem.set_u32(block + 0x40, field & !1);
                }
                e.call(
                    RAISE_EXC,
                    &args![block, word_at, flags, operation, at + 8, at + 0x18],
                );
            }
        }
        // 00ee1bfd
        let word = e.mem.u32(word_at);
        e.call(CTRLFP, &args![word, 0xffffu32]);
        let kind = e.mem.u32(at);
        let mut set_errno = true;
        if kind != 8 && e.global::<u32>(MATHERR_SKIP) == 0 {
            let answer = e.call(DEFAULT_MATHERR, &args![exception]).u32();
            set_errno = answer == 0;
        }
        if set_errno {
            // 00ee1c28: PUSH dword ptr [ESI].
            let kind = e.mem.u32(at);
            e.call(SET_ERRNO_FROM_MATHERR, &args![kind]);
        }
    })
}

// ---- SSE2 double-precision instructions ---------------------------------------------------

/// An XMM register as two double lanes, low first.
type Pd = [f64; 2];

/// A NaN with its quiet bit set (sign and payload kept).
fn quiet(v: f64) -> f64 {
    f64::from_bits(v.to_bits() | QUIET)
}

/// One lane of an SSE2 double arithmetic instruction (`dest op src`): a NaN in the destination
/// wins, quieted, else a NaN in the source, quieted; on two numbers the IEEE result (round to
/// nearest, denormals kept, as `MXCSR` 0x1f80 has it), and the default NaN for an invalid
/// operation (`inf - inf`, `0 * inf`, `0 / 0`, `inf / inf`).
fn sd_op(dest: f64, src: f64, op: fn(f64, f64) -> f64) -> f64 {
    if dest.is_nan() {
        return quiet(dest);
    }
    if src.is_nan() {
        return quiet(src);
    }
    let result = op(dest, src);
    if result.is_nan() {
        f64::from_bits(DEFAULT_NAN)
    } else {
        result
    }
}

/// `ADDSD`, `SUBSD`, `MULSD`, `DIVSD` (the low lane).
fn addsd(dest: f64, src: f64) -> f64 {
    sd_op(dest, src, |a, b| a + b)
}
fn subsd(dest: f64, src: f64) -> f64 {
    sd_op(dest, src, |a, b| a - b)
}
fn mulsd(dest: f64, src: f64) -> f64 {
    sd_op(dest, src, |a, b| a * b)
}
fn divsd(dest: f64, src: f64) -> f64 {
    sd_op(dest, src, |a, b| a / b)
}

/// `ADDPD`, `SUBPD`, `MULPD`: both lanes.
fn addpd(dest: Pd, src: Pd) -> Pd {
    [addsd(dest[0], src[0]), addsd(dest[1], src[1])]
}
fn subpd(dest: Pd, src: Pd) -> Pd {
    [subsd(dest[0], src[0]), subsd(dest[1], src[1])]
}
fn mulpd(dest: Pd, src: Pd) -> Pd {
    [mulsd(dest[0], src[0]), mulsd(dest[1], src[1])]
}

/// `ANDPD` on one lane: the bit patterns ANDed.
fn and_lane(dest: f64, src: f64) -> f64 {
    f64::from_bits(dest.to_bits() & src.to_bits())
}

/// `CVTSD2SI r32, xmm`: the low lane rounded to nearest-even (`MXCSR`'s mode), the integer
/// indefinite `0x80000000` for a NaN or a value out of range: what `FISTP` gives under the
/// default control word, `crate::x87::fistp`.
fn cvtsd2si(v: f64) -> u32 {
    crate::x87::fistp(v) as u32
}

/// `MOVLPD xmm, m64` / an `m64` operand: the double at `address`.
fn m64(e: &Engine, address: u32) -> f64 {
    f64::from_bits(e.mem.u64(address))
}

/// `MOVAPD xmm, m128` / an `m128` operand: the two doubles at `address`.
fn m128(e: &Engine, address: u32) -> Pd {
    [m64(e, address), m64(e, address + 8)]
}

/// `MOVLPD qword ptr [ESP + 4], xmm; FLD double ptr [ESP + 4]`: the low lane as `ST0`.
fn fld_result(v: f64) -> f64 {
    x87_load64(v.to_bits())
}

/// `PEXTRW EAX, XMM0, 3; AND AX, 0x7fff; SUB AX, bias`: the 16-bit difference the cores test.
fn top_word_less(x: f64, bias: u16) -> u16 {
    (((x.to_bits() >> 48) as u16) & 0x7fff).wrapping_sub(bias)
}

// ---- The SSE2 bodies ----------------------------------------------------------------------

/// The register entries' SSE2 bodies (`PUSH EBP; MOV EBP, ESP; SUB ESP, 8; AND ESP, -16; FSTP
/// double ptr [ESP]; MOVQ XMM0, qword ptr [ESP]; CALL <core>; LEAVE; RET`): `ST0` stored as a
/// double (a signalling NaN quiet), which is both `XMM0` (its high lane cleared) and the
/// double at the core's `[ESP + 4]`; the core's `ST0` comes back.
fn register_entry_sse2(e: &mut Engine, core: u32, st0: f64) -> f64 {
    let stored = x87_load64(st0.to_bits());
    e.call(core, &args![stored]).f64()
}

// Translated from 00ed9780 (disassembled, FalloutNV.exe 1.4.0.525)
/// The SSE2 body `_CIcos` jumps to (no label), `ST0` in, `ST0` out: the core `00ed979e` on
/// `ST0` stored as a double ([`register_entry_sse2`]).
pub fn ci_cos_sse2(e: &mut Engine, st0: f64) -> f64 {
    register_entry_sse2(e, COS_CORE, st0)
}

// Translated from 00ed9930 (disassembled, FalloutNV.exe 1.4.0.525)
/// The SSE2 body `_CIsin` jumps to (no label): the core `00ed994e`, as [`ci_cos_sse2`].
pub fn ci_sin_sse2(e: &mut Engine, st0: f64) -> f64 {
    register_entry_sse2(e, SIN_CORE, st0)
}

// Translated from 00edb880 (disassembled, FalloutNV.exe 1.4.0.525)
/// The SSE2 body `_CItan` jumps to (no label): the core `00edb89e`, as [`ci_cos_sse2`].
pub fn ci_tan_sse2(e: &mut Engine, st0: f64) -> f64 {
    register_entry_sse2(e, TAN_CORE, st0)
}

// Translated from 00ed9798 (disassembled, FalloutNV.exe 1.4.0.525)
/// The SSE2 body `cos` jumps to (no label), cdecl, the double on the stack: `MOVLPD XMM0,
/// qword ptr [ESP + 4]` (the double as it is; the high lane, which the core never reads into
/// its result, keeps what `XMM0` held) and the core `00ed979e` that follows.
pub fn cos_sse2(e: &mut Engine, x: f64) -> f64 {
    cos_sse2_core(e, x)
}

// Translated from 00ed9948 (disassembled, FalloutNV.exe 1.4.0.525)
/// The SSE2 body `sin` jumps to: `MOVLPD XMM0` of the stack double and the core `00ed994e`.
pub fn sin_sse2(e: &mut Engine, x: f64) -> f64 {
    sin_sse2_core(e, x)
}

// Translated from 00edb898 (disassembled, FalloutNV.exe 1.4.0.525)
/// The SSE2 body `tan` jumps to: `MOVLPD XMM0` of the stack double and the core `00edb89e`.
pub fn tan_sse2(e: &mut Engine, x: f64) -> f64 {
    tan_sse2_core(e, x)
}

/// The constants of the sine and cosine cores' main path, by the instruction that reads them
/// (the two cores run the same instructions on their own constants).
struct Kernel {
    /// `MOVLPD XMM1, [..]`, then `MULSD XMM1, XMM0`.
    scale: u32,
    /// `MOVLPD XMM2, [..]`, added and subtracted again (`ADDSD`/`SUBSD XMM1, XMM2`).
    shifter: u32,
    /// `MOVLPD XMM3, [..]`, times the rounded multiple (`MULSD XMM3, XMM1`).
    step_high: u32,
    /// `MOVAPD XMM2, [..]`, times both lanes of the multiple (`MULPD XMM2, XMM1`).
    step_pair: u32,
    /// `MULSD XMM1, [..]`.
    step_low: u32,
    /// `MOVAPD XMM5, [..]` and `ADDPD XMM5, [..]`.
    poly5: u32,
    poly5_add: u32,
    /// `MOVAPD XMM6, [..]` and `ADDPD XMM6, [..]`.
    poly6: u32,
    poly6_add: u32,
    /// `ADD EDX, ..` before `AND EDX, 0x3f`.
    index_bias: u32,
    /// `LEA EAX, [..]`: the table of 64 entries of 0x20 bytes.
    table: u32,
}

const COS_KERNEL: Kernel = Kernel {
    scale: 0x010f_8db0,
    shifter: 0x010f_8db8,
    step_high: 0x010f_8dd0,
    step_pair: 0x010f_8dc0,
    step_low: 0x010f_8dd8,
    poly5: 0x010f_8da0,
    poly5_add: 0x010f_8d90,
    poly6: 0x010f_8d80,
    poly6_add: 0x010f_8d70,
    index_bias: 0x001c_7610,
    table: 0x010f_8570,
};

const SIN_KERNEL: Kernel = Kernel {
    scale: 0x010f_9660,
    shifter: 0x010f_9668,
    step_high: 0x010f_9680,
    step_pair: 0x010f_9670,
    step_low: 0x010f_9688,
    poly5: 0x010f_9650,
    poly5_add: 0x010f_9640,
    poly6: 0x010f_9630,
    poly6_add: 0x010f_9620,
    index_bias: 0x001c_7600,
    table: 0x010f_8e20,
};

/// The main path of the sine and cosine cores (`00ed97b5..00ed98f6` and `00ed9965..00ed9aa6`,
/// the same instructions on each core's [`Kernel`]): `XMM0`'s low lane in, the `ST0` it loads
/// out. Registers whose high lane a `MOVLPD`/`MOVSD` keeps from before (`XMM1` until its
/// `UNPCKLPD`, `XMM3`, `XMM4`, `XMM5` after its `MOVLPD`, `XMM7`, and `XMM0` until its
/// `UNPCKLPD`) are kept as their low lane: no instruction reads those high lanes into the
/// result. Every operand here is a finite number (the argument's exponent is at most `0x40f`),
/// so only the tables can bring in a NaN, which the instructions' rules then carry.
fn sincos_kernel(e: &Engine, x: f64, k: &Kernel) -> f64 {
    let mut x0 = x; // the low lane of XMM0
    let mut x1 = m64(e, k.scale); // MOVLPD XMM1
    x1 = mulsd(x1, x0); // MULSD XMM1, XMM0
    let x2_shifter = m64(e, k.shifter); // MOVLPD XMM2
    let mut edx = cvtsd2si(x1); // CVTSD2SI EDX, XMM1
    x1 = addsd(x1, x2_shifter); // ADDSD XMM1, XMM2
    let mut x3 = m64(e, k.step_high); // MOVLPD XMM3
    x1 = subsd(x1, x2_shifter); // SUBSD XMM1, XMM2
    let mut x2: Pd = m128(e, k.step_pair); // MOVAPD XMM2
    x3 = mulsd(x3, x1); // MULSD XMM3, XMM1
    let mut x1p: Pd = [x1, x1]; // UNPCKLPD XMM1, XMM1
    edx = edx.wrapping_add(k.index_bias); // ADD EDX, ..
    let mut x4 = x0; // MOVSD XMM4, XMM0
    edx &= 0x3f; // AND EDX, 0x3f
    let mut x5: Pd = m128(e, k.poly5); // MOVAPD XMM5
                                       // LEA EAX, [table]; SHL EDX, 5; ADD EAX, EDX.
    let entry = k.table.wrapping_add(edx << 5);
    x2 = mulpd(x2, x1p); // MULPD XMM2, XMM1
    x0 = subsd(x0, x3); // SUBSD XMM0, XMM3
    x1p[0] = mulsd(x1p[0], m64(e, k.step_low)); // MULSD XMM1, [..]
    x4 = subsd(x4, x3); // SUBSD XMM4, XMM3
    let mut x7 = m64(e, entry + 8); // MOVLPD XMM7, [EAX + 8]
    let mut x0p: Pd = [x0, x0]; // UNPCKLPD XMM0, XMM0
    x3 = x4; // MOVSD XMM3, XMM4
    x4 = subsd(x4, x2[0]); // SUBSD XMM4, XMM2
    x5 = mulpd(x5, x0p); // MULPD XMM5, XMM0
    x0p = subpd(x0p, x2); // SUBPD XMM0, XMM2
    let mut x6: Pd = m128(e, k.poly6); // MOVAPD XMM6
    x7 = mulsd(x7, x4); // MULSD XMM7, XMM4
    x3 = subsd(x3, x4); // SUBSD XMM3, XMM4
    x5 = mulpd(x5, x0p); // MULPD XMM5, XMM0
    x0p = mulpd(x0p, x0p); // MULPD XMM0, XMM0
    x3 = subsd(x3, x2[0]); // SUBSD XMM3, XMM2
    x2 = m128(e, entry); // MOVAPD XMM2, [EAX]
    x1p[0] = subsd(x1p[0], x3); // SUBSD XMM1, XMM3
    x3 = m64(e, entry + 0x18); // MOVLPD XMM3, [EAX + 0x18]
    x2[0] = addsd(x2[0], x3); // ADDSD XMM2, XMM3
    x7 = subsd(x7, x2[0]); // SUBSD XMM7, XMM2
    x2[0] = mulsd(x2[0], x4); // MULSD XMM2, XMM4
    x6 = mulpd(x6, x0p); // MULPD XMM6, XMM0
    x3 = mulsd(x3, x4); // MULSD XMM3, XMM4
    x2 = mulpd(x2, x0p); // MULPD XMM2, XMM0
    x0p = mulpd(x0p, x0p); // MULPD XMM0, XMM0
    x5 = addpd(x5, m128(e, k.poly5_add)); // ADDPD XMM5, [..]
    x4 = mulsd(x4, m64(e, entry)); // MULSD XMM4, [EAX]
    x6 = addpd(x6, m128(e, k.poly6_add)); // ADDPD XMM6, [..]
    x5 = mulpd(x5, x0p); // MULPD XMM5, XMM0
    x0p[0] = x3; // MOVSD XMM0, XMM3
    x3 = addsd(x3, m64(e, entry + 8)); // ADDSD XMM3, [EAX + 8]
    x1p[0] = mulsd(x1p[0], x7); // MULSD XMM1, XMM7
    x7 = x4; // MOVSD XMM7, XMM4
    x4 = addsd(x4, x3); // ADDSD XMM4, XMM3
    x6 = addpd(x6, x5); // ADDPD XMM6, XMM5
    let mut x5_low = m64(e, entry + 8); // MOVLPD XMM5, [EAX + 8]
    x5_low = subsd(x5_low, x3); // SUBSD XMM5, XMM3
    x3 = subsd(x3, x4); // SUBSD XMM3, XMM4
    x1p[0] = addsd(x1p[0], m64(e, entry + 0x10)); // ADDSD XMM1, [EAX + 0x10]
    x6 = mulpd(x6, x2); // MULPD XMM6, XMM2
    x5_low = addsd(x5_low, x0p[0]); // ADDSD XMM5, XMM0
    x3 = addsd(x3, x7); // ADDSD XMM3, XMM7
    x1p[0] = addsd(x1p[0], x5_low); // ADDSD XMM1, XMM5
    x1p[0] = addsd(x1p[0], x3); // ADDSD XMM1, XMM3
    x1p[0] = addsd(x1p[0], x6[0]); // ADDSD XMM1, XMM6
    x6 = [x6[1], x6[1]]; // UNPCKHPD XMM6, XMM6
    x1p[0] = addsd(x1p[0], x6[0]); // ADDSD XMM1, XMM6
    x4 = addsd(x4, x1p[0]); // ADDSD XMM4, XMM1
    fld_result(x4)
}

/// `cos`'s tiny-argument constant (`010f8e00`) and `sin`'s (`010f9690`, `010f9698`, and
/// `010f96a0` for a zero or denormal argument).
const COS_TINY: u32 = 0x010f_8e00;
const SIN_TINY_FACTOR: u32 = 0x010f_9690;
const SIN_TINY_SCALE: u32 = 0x010f_9698;
const SIN_DENORMAL_FACTOR: u32 = 0x010f_96a0;

// Translated from 00ed979e (disassembled, FalloutNV.exe 1.4.0.525)
/// The SSE2 cosine (no label), `XMM0` in (the double the callers also have at `[ESP + 4]`),
/// `ST0` out. Its top word without the sign, less `0x3030`, sorts the argument: up to `0x10c5`
/// (an exponent from `0x303` to `0x40f`) the main path ([`sincos_kernel`] on the cosine
/// constants); above it as a signed number (an exponent above `0x40f`, the infinities and
/// NaNs included) a jump to `cos`'s x87 path `00ec9fbf` with the stack double; below `0x3030`
/// (a smaller exponent: tiny, denormal or zero) the argument's sign is cleared (`PINSRW`) and
/// the result is `[010f8e00] - |x|` (`SUBSD`).
pub fn cos_sse2_core(e: &mut Engine, x: f64) -> f64 {
    let ax = top_word_less(x, 0x3030);
    if ax <= 0x10c5 {
        return sincos_kernel(e, x, &COS_KERNEL);
    }
    // 00ed98f7: JG 00ed9924.
    if ax as i16 > 0x10c5 {
        return fn_00ec9fbf(e, x);
    }
    // PEXTRW EAX, XMM0, 3; AND AX, 0x7fff; PINSRW XMM0, EAX, 3.
    let magnitude = f64::from_bits(x.to_bits() & !(1 << 63));
    let result = subsd(m64(e, COS_TINY), magnitude);
    fld_result(result)
}

// Translated from 00ed994e (disassembled, FalloutNV.exe 1.4.0.525)
/// The SSE2 sine (no label), `XMM0` in, `ST0` out: sorted as [`cos_sse2_core`] (main path on
/// the sine constants; a large exponent, infinity or NaN to `sin`'s x87 path `00eca0ef`). Below
/// `0x3030` the difference shifted right by 4 (`SHR AX, 4`) is `0xcfd` exactly for an exponent
/// of zero (zero or denormal): the result is `x * [010f96a0]`. For any other tiny argument
/// `([010f9690] * x - x) * [010f96a0]` is computed into `XMM3` (its flags only) and the result
/// is `x` itself.
pub fn sin_sse2_core(e: &mut Engine, x: f64) -> f64 {
    let ax = top_word_less(x, 0x3030);
    if ax <= 0x10c5 {
        return sincos_kernel(e, x, &SIN_KERNEL);
    }
    // 00ed9aa7: JG 00ed9af2.
    if ax as i16 > 0x10c5 {
        return fn_00eca0ef(e, x);
    }
    // SUB ESP, 0x10; SHR AX, 4; CMP AX, 0xcfd; JNZ 00ed9acc.
    if ax >> 4 == 0xcfd {
        return fld_result(mulsd(x, m64(e, SIN_DENORMAL_FACTOR)));
    }
    // 00ed9acc: MOVLPD XMM3; MULSD XMM3, XMM0; SUBSD XMM3, XMM0; MULSD XMM3, [..]; the result
    // is XMM0.
    let mut x3 = m64(e, SIN_TINY_FACTOR);
    x3 = mulsd(x3, x);
    x3 = subsd(x3, x);
    let _discarded = mulsd(x3, m64(e, SIN_TINY_SCALE));
    fld_result(x)
}

/// The tangent core's constants (`010fc4e0..010fc568`), by the instruction that reads them,
/// and its table (`010faee0`, 32 entries of 0xb0 bytes).
const TAN_SCALE: u32 = 0x010f_c4e0;
const TAN_SHIFTER: u32 = 0x010f_c4f0;
const TAN_STEP3: u32 = 0x010f_c500;
const TAN_STEP4: u32 = 0x010f_c510;
const TAN_STEP1: u32 = 0x010f_c520;
const TAN_STEP5: u32 = 0x010f_c530;
const TAN_NUMERATOR: u32 = 0x010f_c538;
const TAN_MASK: u32 = 0x010f_c540;
const TAN_TINY_FACTOR: u32 = 0x010f_c558;
const TAN_TINY_SCALE: u32 = 0x010f_c560;
const TAN_TABLE: u32 = 0x010f_aee0;

/// The main path of the tangent core (`00edb8b5..00edba8b`): `XMM0`'s low lane in, the `ST0`
/// it loads out. `XMM5` after its `MOVLPD` and `XMM6` after its `MOVLPD` are kept as their low
/// lane (their high lanes, from before, reach only `ANDPD` results whose high lane is never
/// read); every other register is set whole before it is read. The argument is finite with an
/// exponent at most `0x40c`; a zero divisor in `DIVSD` (the masked reduced argument) gives an
/// infinity, which the instructions' NaN rules then carry.
fn tan_kernel(e: &Engine, x: f64) -> f64 {
    let mut x0: Pd = [x, x]; // UNPCKLPD XMM0, XMM0
    let mut x1: Pd = m128(e, TAN_SCALE); // MOVAPD XMM1
    x1 = mulpd(x1, x0); // MULPD XMM1, XMM0
    let mut edx = cvtsd2si(x1[0]); // CVTSD2SI EDX, XMM1
    let x2_shifter: Pd = m128(e, TAN_SHIFTER); // MOVAPD XMM2
    x1 = addpd(x1, x2_shifter); // ADDPD XMM1, XMM2
    let mut x3: Pd = m128(e, TAN_STEP3); // MOVAPD XMM3
    x1 = subpd(x1, x2_shifter); // SUBPD XMM1, XMM2
    let mut x5 = m64(e, TAN_STEP5); // MOVLPD XMM5
    edx = edx.wrapping_add(0x72900); // ADD EDX, 0x72900
    let mut x4: Pd = m128(e, TAN_STEP4); // MOVAPD XMM4
    x3 = mulpd(x3, x1); // MULPD XMM3, XMM1
    edx &= 0x1f; // AND EDX, 0x1f
    x5 = mulsd(x5, x1[0]); // MULSD XMM5, XMM1
    let mut ecx = edx; // MOV ECX, EDX
    x4 = mulpd(x4, x1); // MULPD XMM4, XMM1
    ecx <<= 1; // SHL ECX, 1
    x0 = subpd(x0, x3); // SUBPD XMM0, XMM3
    x1 = mulpd(x1, m128(e, TAN_STEP1)); // MULPD XMM1, [..]
    edx = edx.wrapping_add(ecx); // ADD EDX, ECX
    ecx <<= 2; // SHL ECX, 2
    edx = edx.wrapping_add(ecx); // ADD EDX, ECX
    x5 = addsd(x5, x0[0]); // ADDSD XMM5, XMM0
    let mut x2: Pd = x0; // MOVAPD XMM2, XMM0
    x0 = subpd(x0, x4); // SUBPD XMM0, XMM4
    let mut x6 = m64(e, TAN_NUMERATOR); // MOVLPD XMM6
    edx <<= 4; // SHL EDX, 4
    x5 = and_lane(x5, m64(e, TAN_MASK)); // ANDPD XMM5, [..]
    x3 = x0; // MOVAPD XMM3, XMM0
             // LEA EAX, [010faee0]; ADD EAX, EDX.
    let entry = TAN_TABLE.wrapping_add(edx);
    x2 = subpd(x2, x0); // SUBPD XMM2, XMM0
    x0 = [x0[1], x0[1]]; // UNPCKHPD XMM0, XMM0
    x6 = divsd(x6, x5); // DIVSD XMM6, XMM5
    x2 = subpd(x2, x4); // SUBPD XMM2, XMM4
    let mut x7: Pd = m128(e, entry + 0x10); // MOVAPD XMM7, [EAX + 0x10]
    x3[0] = subsd(x3[0], x5); // SUBSD XMM3, XMM5
    x7 = mulpd(x7, x0); // MULPD XMM7, XMM0
    x2 = subpd(x2, x1); // SUBPD XMM2, XMM1
    x1 = m128(e, entry + 0x30); // MOVAPD XMM1, [EAX + 0x30]
    x1 = mulpd(x1, x0); // MULPD XMM1, XMM0
    x4 = m128(e, entry + 0x60); // MOVAPD XMM4, [EAX + 0x60]
    x4 = mulpd(x4, x0); // MULPD XMM4, XMM0
    x2[0] = addsd(x2[0], x3[0]); // ADDSD XMM2, XMM3
    x3 = x0; // MOVAPD XMM3, XMM0
    x0 = mulpd(x0, x0); // MULPD XMM0, XMM0
    x7 = addpd(x7, m128(e, entry)); // ADDPD XMM7, [EAX]
    x1 = addpd(x1, m128(e, entry + 0x20)); // ADDPD XMM1, [EAX + 0x20]
    x1 = mulpd(x1, x0); // MULPD XMM1, XMM0
    x4 = addpd(x4, m128(e, entry + 0x50)); // ADDPD XMM4, [EAX + 0x50]
    x7 = addpd(x7, x1); // ADDPD XMM7, XMM1
    x1 = m128(e, entry + 0x70); // MOVAPD XMM1, [EAX + 0x70]
    x1 = mulpd(x1, x0); // MULPD XMM1, XMM0
    x0 = mulpd(x0, x0); // MULPD XMM0, XMM0
    x4 = addpd(x4, x1); // ADDPD XMM4, XMM1
    x1 = m128(e, entry + 0x40); // MOVAPD XMM1, [EAX + 0x40]
    x1 = mulpd(x1, x0); // MULPD XMM1, XMM0
    x7 = addpd(x7, x1); // ADDPD XMM7, XMM1
    x1 = x3; // MOVAPD XMM1, XMM3
    x3 = mulpd(x3, x0); // MULPD XMM3, XMM0
    x0[0] = mulsd(x0[0], x0[0]); // MULSD XMM0, XMM0
    x1 = mulpd(x1, m128(e, entry + 0x90)); // MULPD XMM1, [EAX + 0x90]
    x4 = mulpd(x4, x3); // MULPD XMM4, XMM3
    x3[0] = x1[0]; // MOVSD XMM3, XMM1
    x7 = addpd(x7, x4); // ADDPD XMM7, XMM4
    x4[0] = x1[0]; // MOVSD XMM4, XMM1
    x0[0] = mulsd(x0[0], x7[0]); // MULSD XMM0, XMM7
    x7 = [x7[1], x7[1]]; // UNPCKHPD XMM7, XMM7
    x0[0] = addsd(x0[0], x7[0]); // ADDSD XMM0, XMM7
    x1 = [x1[1], x1[1]]; // UNPCKHPD XMM1, XMM1
    x3[0] = addsd(x3[0], x1[0]); // ADDSD XMM3, XMM1
    x4[0] = subsd(x4[0], x3[0]); // SUBSD XMM4, XMM3
    x1[0] = addsd(x1[0], x4[0]); // ADDSD XMM1, XMM4
    x4[0] = x2[0]; // MOVSD XMM4, XMM2
    x7[0] = m64(e, entry + 0x90); // MOVLPD XMM7, [EAX + 0x90]
    x2 = [x2[1], x2[1]]; // UNPCKHPD XMM2, XMM2
    x7[0] = addsd(x7[0], m64(e, entry + 0x98)); // ADDSD XMM7, [EAX + 0x98]
    x7[0] = mulsd(x7[0], x2[0]); // MULSD XMM7, XMM2
    x7[0] = addsd(x7[0], m64(e, entry + 0x88)); // ADDSD XMM7, [EAX + 0x88]
    x7[0] = addsd(x7[0], x1[0]); // ADDSD XMM7, XMM1
    x0[0] = addsd(x0[0], x7[0]); // ADDSD XMM0, XMM7
    x7[0] = m64(e, TAN_NUMERATOR); // MOVLPD XMM7, [010fc538]
    x4[0] = mulsd(x4[0], x6); // MULSD XMM4, XMM6
    x2[0] = m64(e, entry + 0xa8); // MOVLPD XMM2, [EAX + 0xa8]
    x2[0] = and_lane(x2[0], x6); // ANDPD XMM2, XMM6
    x5 = mulsd(x5, x2[0]); // MULSD XMM5, XMM2
    x6 = mulsd(x6, m64(e, entry + 0xa0)); // MULSD XMM6, [EAX + 0xa0]
    x7[0] = subsd(x7[0], x5); // SUBSD XMM7, XMM5
    x2[0] = subsd(x2[0], m64(e, entry + 0x80)); // SUBSD XMM2, [EAX + 0x80]
    x7[0] = subsd(x7[0], x4[0]); // SUBSD XMM7, XMM4
    x7[0] = mulsd(x7[0], x6); // MULSD XMM7, XMM6
    x4[0] = x3[0]; // MOVSD XMM4, XMM3
    x3[0] = subsd(x3[0], x2[0]); // SUBSD XMM3, XMM2
    x2[0] = addsd(x2[0], x3[0]); // ADDSD XMM2, XMM3
    x4[0] = subsd(x4[0], x2[0]); // SUBSD XMM4, XMM2
    x0[0] = addsd(x0[0], x4[0]); // ADDSD XMM0, XMM4
    x0[0] = subsd(x0[0], x7[0]); // SUBSD XMM0, XMM7
    x0[0] = addsd(x0[0], x3[0]); // ADDSD XMM0, XMM3
    fld_result(x0[0])
}

// Translated from 00edb89e (disassembled, FalloutNV.exe 1.4.0.525)
/// The SSE2 tangent (no label), `XMM0` in, `ST0` out. Its top word without the sign, less
/// `0x3820`: up to `0x8a8` (an exponent from `0x382` to `0x40c`) the main path
/// ([`tan_kernel`]); above it as a signed number (a larger exponent, the infinities and NaNs)
/// a jump to `tan`'s x87 path `00eca61f`; below `0x3820` the result is `([010fc558] * x + x) *
/// [010fc560]`. For an exponent of zero (the difference shifted right by 4 is `0xc7e`) the
/// code first computes `x * [010fc560]` into `XMM3`, which the next instruction overwrites.
pub fn tan_sse2_core(e: &mut Engine, x: f64) -> f64 {
    let ax = top_word_less(x, 0x3820);
    if ax <= 0x8a8 {
        return tan_kernel(e, x);
    }
    // 00edba8c: JG 00edbacd.
    if ax as i16 > 0x8a8 {
        return fn_00eca61f(e, x);
    }
    // SUB ESP, 0x10; SHR AX, 4; CMP AX, 0xc7e; JNZ 00edbaa7.
    if ax >> 4 == 0xc7e {
        // MOVSD XMM3, XMM0; MULSD XMM3, [010fc560]: overwritten below.
        let _overwritten = mulsd(x, m64(e, TAN_TINY_SCALE));
    }
    // 00edbaa7
    let mut x3 = m64(e, TAN_TINY_FACTOR);
    x3 = mulsd(x3, x);
    x3 = addsd(x3, x);
    x3 = mulsd(x3, m64(e, TAN_TINY_SCALE));
    fld_result(x3)
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ec_9f70, ci_cos(f64) -> f64),
        entry!(0x00ec_a0a0, ci_sin(f64) -> f64),
        entry!(0x00ec_a5d0, ci_tan(f64) -> f64),
        entry!(0x00ec_9f30, cos(f64) -> f64),
        entry!(0x00ec_a060, sin(f64) -> f64),
        entry!(0x00ec_a590, tan(f64) -> f64),
        entry!(0x00ec_9fbf, fn_00ec9fbf(f64) -> f64),
        entry!(0x00ec_a0ef, fn_00eca0ef(f64) -> f64),
        entry!(0x00ec_a61f, fn_00eca61f(f64) -> f64),
        entry!(0x00ec_9fc8, cos_x87(bool, u32, f64, u32, u32) -> f64),
        entry!(0x00ec_a0f8, sin_x87(bool, u32, f64, u32, u32) -> f64),
        entry!(0x00ec_a628, tan_x87(bool, u32, f64, u32, u32) -> f64),
        entry!(0x00ed_2808, fn_00ed2808(u32, u32) -> u32),
        (0x00ed_27ac, fn_00ed27ac_entry as AbiFn),
        entry!(0x00ed_282b, math_exit(u32, u32, f64, u32, u32, u32, u32) -> f64),
        entry!(
            0x00ed_2737,
            start_one_arg_error_handling(u32, u32, u32, f64, u32, u32, u32, u32) -> f64
        ),
        entry!(0x00ee_1b04, except_87(u32, Ptr, Ptr)),
        entry!(0x00ed_9780, ci_cos_sse2(f64) -> f64),
        entry!(0x00ed_9930, ci_sin_sse2(f64) -> f64),
        entry!(0x00ed_b880, ci_tan_sse2(f64) -> f64),
        entry!(0x00ed_9798, cos_sse2(f64) -> f64),
        entry!(0x00ed_9948, sin_sse2(f64) -> f64),
        entry!(0x00ed_b898, tan_sse2(f64) -> f64),
        entry!(0x00ed_979e, cos_sse2_core(f64) -> f64),
        entry!(0x00ed_994e, sin_sse2_core(f64) -> f64),
        entry!(0x00ed_b89e, tan_sse2_core(f64) -> f64),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    const SSE2_FLAG: u32 = 0x0127_0a64;
    const ERROR_SWITCH: u32 = 0x0126_fd90;
    const ERRNO_ADDR: u32 = 0x00ec_85e3;
    const EDOM: u32 = 0x21;
    /// The status-flag helper's extended constants (`math.rs`'s `fn_00ed5ae4` reads them).
    const STATFP_CONSTANTS: u32 = 0x011c_2520;
    /// The SSE2 cores' constants and tables, all inside this range.
    const SSE_DATA: u32 = 0x010f_8000;
    const SSE_DATA_LEN: u32 = 0x5000;
    /// The modulus the tests give `FPREM1` (not the exe's: any large extended value).
    const TEST_MODULUS: (u16, u64) = (0x403d, 0xb000_0000_0000_0001);

    type Calls = Vec<Vec<u32>>;
    type Shared<T> = Rc<RefCell<Vec<T>>>;

    fn calls(e: &Engine, addr: u32) -> Calls {
        let log = e.call_log.as_ref().unwrap();
        log.iter()
            .filter(|c| c.0 == addr)
            .map(|c| c.1.clone())
            .collect()
    }

    fn clear_log(e: &mut Engine) {
        e.call_log = Some(vec![]);
    }

    fn pair(bits: u64) -> [u32; 2] {
        [bits as u32, (bits >> 32) as u32]
    }

    /// A xorshift generator for spread test values.
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> u64 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            self.0
        }

        /// A double with a biased exponent in `exponents`, a random fraction and sign.
        fn double(&mut self, exponents: std::ops::RangeInclusive<u64>) -> f64 {
            let span = exponents.end() - exponents.start() + 1;
            let exponent = exponents.start() + self.next() % span;
            let fraction = self.next() & 0x000f_ffff_ffff_ffff;
            let sign = self.next() & (1 << 63);
            f64::from_bits(sign | (exponent << 52) | fraction)
        }
    }

    /// An engine with what the paths read: the flag, the error switch (clear, as in the exe),
    /// the x87 bodies' constants (1.0, `0x027f`, a test modulus), the indefinite NaN, the
    /// `_matherr` skip word as the exe has it (nonzero), the status helper's constants, the
    /// SSE2 data filled from `seed`, `_errno` doubled to one slot and the call log on.
    fn engine(sse2: bool, seed: u64) -> (Engine, u32) {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.register_double(ERRNO_ADDR, move |_, _| slot.into_ret());
        e.map(SSE2_FLAG, 4);
        e.set_global(SSE2_FLAG, u32::from(sse2));
        e.map(ERROR_SWITCH, 4);
        e.map(NAN_ADDEND, 0x14);
        e.mem.set_u64(NAN_ADDEND, 1.0f64.to_bits());
        e.mem.set_u16(BODY_CONTROL_WORD, 0x027f);
        let modulus = X87Ext::from_parts(TEST_MODULUS.0, TEST_MODULUS.1);
        e.mem.write(REDUCTION_MODULUS, &modulus.0);
        e.map(INDEFINITE_NAN, 0x10);
        e.mem.set_u64(INDEFINITE_NAN, 0xc000_0000_0000_0000);
        e.mem.set_u16(INDEFINITE_NAN + 8, 0xffff);
        e.map(MATHERR_SKIP, 4);
        e.set_global(MATHERR_SKIP, 0x2694u32);
        e.map(STATFP_CONSTANTS, 0x18);
        fill_sse_data(&mut e, seed);
        e.call_log = Some(vec![]);
        (e, slot)
    }

    /// Moderate doubles (exponents 2^-4 to 2^3) of both signs in every SSE2 constant and table
    /// slot.
    fn fill_sse_data(e: &mut Engine, seed: u64) {
        e.map(SSE_DATA, SSE_DATA_LEN);
        let mut rng = Rng(seed);
        for at in (SSE_DATA..SSE_DATA + SSE_DATA_LEN).step_by(8) {
            e.mem.set_u64(at, rng.double(0x3fb..=0x402).to_bits());
        }
    }

    /// The six entries with the instruction their x87 body runs and whether they take `ST0`.
    const ENTRIES: [(u32, Trig, bool); 6] = [
        (0x00ec_9f70, Trig::Cos, true),
        (0x00ec_a0a0, Trig::Sin, true),
        (0x00ec_a5d0, Trig::Tan, true),
        (0x00ec_9f30, Trig::Cos, false),
        (0x00ec_a060, Trig::Sin, false),
        (0x00ec_a590, Trig::Tan, false),
    ];

    fn body_of(trig: Trig) -> u32 {
        match trig {
            Trig::Cos => COS_X87,
            Trig::Sin => SIN_X87,
            Trig::Tan => TAN_X87,
        }
    }

    fn core_of(trig: Trig) -> u32 {
        match trig {
            Trig::Cos => COS_CORE,
            Trig::Sin => SIN_CORE,
            Trig::Tan => TAN_CORE,
        }
    }

    /// The x87 instruction sequence of a finite argument through the helpers, under the
    /// control word the body runs with (`0x027f` here): `FLD`, then for an argument of 2^63 or
    /// more `FPREM1` against the modulus until C2 clears, the instruction; the result as `FSTP
    /// qword` gives it.
    fn x87_reference(trig: Trig, x: f64) -> f64 {
        let saved = crate::x87::control_word();
        crate::x87::set_control_word(0x027f);
        let mut v = X87Ext::load64(x);
        if x.abs() >= 2f64.powi(63) {
            let modulus = X87Ext::from_parts(TEST_MODULUS.0, TEST_MODULUS.1);
            loop {
                let (r, status) = x87_fprem1(v, modulus);
                v = r;
                if status & X87_STATUS_C2 == 0 {
                    break;
                }
            }
        }
        let r = match trig {
            Trig::Cos => x87_fcos(v),
            Trig::Sin => x87_fsin(v),
            Trig::Tan => x87_fptan(v),
        };
        let out = r.store64();
        crate::x87::set_control_word(saved);
        out
    }

    /// Finite arguments over the whole double range: zeros, denormals, every exponent band,
    /// the edges around 2^63 where the reduction starts, and the largest doubles.
    fn x87_arguments() -> Vec<f64> {
        let mut out = vec![
            0.0,
            -0.0,
            f64::from_bits(1),
            -f64::from_bits(0x000f_ffff_ffff_ffff),
            f64::MIN_POSITIVE,
            0.5,
            1.0,
            -1.0,
            std::f64::consts::FRAC_PI_2,
            3.0,
            -100.25,
            1.0e10,
            f64::from_bits(0x43df_ffff_ffff_ffff),
            2f64.powi(63),
            -(2f64.powi(63)),
            2f64.powi(64) + 4096.0,
            1.0e30,
            -1.0e200,
            f64::MAX,
            f64::MIN,
        ];
        let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
        for band in [0x001..=0x3ff, 0x3f0..=0x40f, 0x400..=0x43d, 0x43e..=0x7fe] {
            for _ in 0..60 {
                out.push(rng.double(band.clone()));
            }
        }
        out
    }

    #[test]
    fn out_of_range_is_where_the_fpu_leaves_the_operand_alone() {
        for x in [
            2f64.powi(63),
            -(2f64.powi(63)),
            1.0e19,
            1.0e300,
            f64::MAX,
            f64::from_bits(0x43df_ffff_ffff_ffff),
            1.0e18,
            1.0,
        ] {
            let v = X87Ext::load64(x);
            assert_eq!(
                x87_fsin(v) == v,
                out_of_range(v),
                "{x:e}: FSIN leaves it alone exactly when C2 is set"
            );
            assert_eq!(x87_fcos(v) == v, out_of_range(v), "{x:e}: FCOS");
        }
    }

    #[test]
    fn the_entries_take_the_sse2_body_only_with_the_flag_and_every_exception_masked() {
        for (entry, trig, register) in ENTRIES {
            // The flag clear: the x87 body.
            let (mut e, _) = engine(false, 1);
            let x87 = e.call(entry, &args![0.75f64]).f64();
            assert_eq!(x87.to_bits(), x87_reference(trig, 0.75).to_bits());
            if register {
                assert_eq!(calls(&e, EXPONENT_BITS).len(), 1);
                assert_eq!(calls(&e, body_of(trig)).len(), 1);
            } else {
                assert_eq!(calls(&e, FLOAD_WITH_FB).len(), 1);
            }
            assert!(calls(&e, core_of(trig)).is_empty());
            // The flag set (any nonzero value), every exception masked: the SSE2 body, no x87
            // code.
            let (mut e, _) = engine(true, 1);
            e.set_global(SSE2_FLAG, 2u32);
            let sse2 = e.call(entry, &args![0.75f64]).f64();
            let core = match trig {
                Trig::Cos => cos_sse2_core(&mut e, 0.75),
                Trig::Sin => sin_sse2_core(&mut e, 0.75),
                Trig::Tan => tan_sse2_core(&mut e, 0.75),
            };
            assert_eq!(sse2.to_bits(), core.to_bits());
            assert!(calls(&e, EXPONENT_BITS).is_empty());
            assert!(calls(&e, FLOAD_WITH_FB).is_empty());
            assert_eq!(calls(&e, core_of(trig)).len(), usize::from(register));
            // Also at 24-bit precision (0x007f masks every exception).
            set_x87_control_word(&mut e, 0x007f);
            clear_log(&mut e);
            e.call(entry, &args![0.75f64]);
            assert!(calls(&e, FLOAD_WITH_FB).is_empty());
            assert!(calls(&e, EXPONENT_BITS).is_empty());
            // One x87 exception unmasked: the x87 body.
            set_x87_control_word(&mut e, 0x027e);
            clear_log(&mut e);
            let unmasked = e.call(entry, &args![0.75f64]).f64();
            assert_eq!(unmasked.to_bits(), x87_reference(trig, 0.75).to_bits());
            assert_eq!(
                x87_control_word(&mut e),
                0x027e,
                "the caller's word is back"
            );
            assert_eq!(
                calls(&e, EXPONENT_BITS).len() + calls(&e, FLOAD_WITH_FB).len(),
                1
            );
            set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
        }
    }

    #[test]
    fn the_x87_paths_match_the_instructions_bit_for_bit() {
        let arguments = x87_arguments();
        for (entry, trig, _) in ENTRIES {
            for (word, switch) in [(0x027fu16, 0u32), (0x027f, 1), (0x007f, 0), (0x0e7f, 1)] {
                let (mut e, errno) = engine(false, 2);
                e.set_global(ERROR_SWITCH, switch);
                set_x87_control_word(&mut e, word);
                e.mem.set_u32(errno, 0);
                for &x in &arguments {
                    let got = e.call(entry, &args![x]).f64();
                    let want = x87_reference(trig, x);
                    assert_eq!(
                        got.to_bits(),
                        want.to_bits(),
                        "{entry:08x}({x:e}) under {word:04x}, switch {switch}"
                    );
                    assert_eq!(x87_control_word(&mut e), word, "the caller's word is back");
                }
                // A finite argument never sets errno.
                assert_eq!(e.mem.u32(errno), 0);
                set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
            }
        }
    }

    #[test]
    fn the_x87_body_reduces_large_arguments_and_reports_through_the_exit() {
        let (mut e, _) = engine(false, 3);
        let x = 1.0e22;
        let got = cos(&mut e, x);
        assert_eq!(got.to_bits(), x87_reference(Trig::Cos, x).to_bits());
        // The exit: operation 0x12, "cos", ST0, the saved word, the return slot, the double.
        let w = pair(x.to_bits());
        let r = pair(got.to_bits());
        assert_eq!(
            calls(&e, MATH_EXIT),
            vec![vec![0x12, COS_NAME, r[0], r[1], 0x027f, 0, w[0], w[1]]]
        );
        assert!(calls(&e, START_ONE_ARG_ERROR_HANDLING).is_empty());
        // Under a word with the precision exception masked and other than 0x027f, the exit
        // reports the (sticky) inexact result: type 8, handled by __handle_exc (flag 0x10), no
        // errno; the result is the double the report stored.
        for (entry, trig, _) in ENTRIES {
            set_x87_control_word(&mut e, 0x007f);
            clear_log(&mut e);
            let got = e.call(entry, &args![0.5f64]).f64();
            assert_eq!(got.to_bits(), x87_reference(trig, 0.5).to_bits());
            let start = calls(&e, START_ONE_ARG_ERROR_HANDLING);
            assert_eq!(start.len(), 1);
            assert_eq!(start[0][..3], [8, trig.name(), trig.operation()]);
            assert_eq!(start[0][5], 0x007f);
            let handled = calls(&e, HANDLE_EXC);
            assert_eq!(handled.len(), 1);
            assert_eq!(handled[0][0], 0x10);
            assert_eq!(handled[0][2], 0x007f);
            assert_eq!(calls(&e, SET_ERRNO_FROM_MATHERR), vec![vec![8]]);
            assert!(calls(&e, RAISE_EXC).is_empty());
            assert_eq!(x87_control_word(&mut e), 0x007f);
        }
        // The precision exception unmasked: the word is loaded back, nothing reported.
        set_x87_control_word(&mut e, 0x025f);
        clear_log(&mut e);
        sin(&mut e, 0.5);
        assert!(calls(&e, START_ONE_ARG_ERROR_HANDLING).is_empty());
        assert_eq!(x87_control_word(&mut e), 0x025f);
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn the_x87_bodies_report_infinities_and_nans() {
        let indefinite = 0xfff8_0000_0000_0000u64;
        for (entry, trig, register) in ENTRIES {
            let (mut e, errno) = engine(false, 4);
            // An infinity: the indefinite NaN, error type 1 (invalid: handled, the exception
            // masked), errno EDOM.
            for inf in [f64::INFINITY, f64::NEG_INFINITY] {
                e.mem.set_u32(errno, 0);
                clear_log(&mut e);
                let got = e.call(entry, &args![inf]).f64();
                assert_eq!(got.to_bits(), indefinite, "{entry:08x}({inf})");
                assert_eq!(e.mem.u32(errno), EDOM);
                let w = pair(inf.to_bits());
                let n = pair(indefinite);
                assert_eq!(
                    calls(&e, START_ONE_ARG_ERROR_HANDLING),
                    vec![vec![
                        1,
                        trig.name(),
                        trig.operation(),
                        n[0],
                        n[1],
                        0x027f,
                        0,
                        w[0],
                        w[1]
                    ]]
                );
                assert_eq!(calls(&e, HANDLE_EXC)[0][0], 8);
                assert!(calls(&e, NAN_ERROR_TYPE).is_empty());
            }
            // A quiet NaN: type 7 (rewritten to 1, no __handle_exc), errno EDOM, the NaN back.
            let nan = f64::from_bits(0xfff8_0000_0000_0123);
            e.mem.set_u32(errno, 0);
            clear_log(&mut e);
            assert_eq!(
                e.call(entry, &args![nan]).f64().to_bits(),
                0xfff8_0000_0000_0123
            );
            assert_eq!(e.mem.u32(errno), EDOM);
            assert_eq!(calls(&e, START_ONE_ARG_ERROR_HANDLING)[0][0], 7);
            assert!(calls(&e, HANDLE_EXC).is_empty());
            // A signalling NaN comes back quiet. Through the stack form it reaches 00ed27ac
            // signalling (type 1, ST0 + 1.0); through the register form `FST` stored it quiet
            // first (type 7).
            let signalling = f64::from_bits(0x7ff0_0000_0000_0123);
            clear_log(&mut e);
            assert_eq!(
                e.call(entry, &args![signalling]).f64().to_bits(),
                0x7ff8_0000_0000_0123
            );
            let kind = if register { 7 } else { 1 };
            assert_eq!(calls(&e, START_ONE_ARG_ERROR_HANDLING)[0][0], kind);
            // With the error switch set: no report, the control word back, errno untouched.
            e.set_global(ERROR_SWITCH, 1u32);
            set_x87_control_word(&mut e, 0x0e7f);
            e.mem.set_u32(errno, 0);
            clear_log(&mut e);
            assert_eq!(
                e.call(entry, &args![f64::INFINITY]).f64().to_bits(),
                indefinite
            );
            assert_eq!(
                e.call(entry, &args![nan]).f64().to_bits(),
                0xfff8_0000_0000_0123
            );
            assert!(calls(&e, START_ONE_ARG_ERROR_HANDLING).is_empty());
            assert!(calls(&e, MATH_EXIT).is_empty());
            assert_eq!(e.mem.u32(errno), 0);
            assert_eq!(x87_control_word(&mut e), 0x0e7f);
            set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
        }
    }

    #[test]
    fn the_exponent_test_and_the_nan_type() {
        let (mut e, _) = engine(false, 5);
        assert_eq!(fn_00ed2808(&mut e, 0, 0x3ff8_1234), 0x3ff0_0000);
        assert_eq!(fn_00ed2808(&mut e, 5, 0x8000_0000), 0);
        assert_eq!(fn_00ed2808(&mut e, 0, 0xfff0_0000), 0xfff0_0000);
        assert_eq!(fn_00ed2808(&mut e, 1, 0x7ff8_0001), 0x7ff8_0001);
        let signalling = f64::from_bits(0x7ff0_0000_0000_0001);
        // Quiet: 7 and ST0 as it is.
        let q = e.call(NAN_ERROR_TYPE, &args![0x7ff8_0000u32, signalling]);
        assert_eq!((q.eax, q.st0.to_bits()), (7, 0x7ff0_0000_0000_0001));
        // Signalling: 1 and ST0 + 1.0, quiet.
        let s = e.call(NAN_ERROR_TYPE, &args![0x7ff0_0000u32, signalling]);
        assert_eq!((s.eax, s.st0.to_bits()), (1, 0x7ff8_0000_0000_0001));
    }

    #[test]
    fn math_exit_returns_restores_or_reports() {
        let (mut e, _) = engine(false, 6);
        let doubled: Shared<Vec<u32>> = Rc::default();
        let seen = doubled.clone();
        e.register_double(START_ONE_ARG_ERROR_HANDLING, move |_, a| {
            seen.borrow_mut().push(a.to_vec());
            9.0f64.into_ret()
        });
        let w = pair(2.5f64.to_bits());
        // 0x027f: ST0 as it is, the word untouched.
        set_x87_control_word(&mut e, 0x0c7f);
        let r = math_exit(&mut e, 0x12, COS_NAME, 0.25, 0xabcd_027f, 7, w[0], w[1]);
        assert_eq!(r, 0.25);
        assert_eq!(x87_control_word(&mut e), 0x0c7f);
        // The precision exception unmasked: the saved word loaded.
        let r = math_exit(&mut e, 0x12, COS_NAME, 0.25, 0x025f, 7, w[0], w[1]);
        assert_eq!(r, 0.25);
        assert_eq!(x87_control_word(&mut e), 0x025f);
        assert!(doubled.borrow().is_empty());
        // Masked: reported as inexact (type 8) with everything passed on.
        let r = math_exit(&mut e, 0x1e, SIN_NAME, 0.25, 0x1234_007f, 7, w[0], w[1]);
        assert_eq!(r, 9.0);
        let q = pair(0.25f64.to_bits());
        assert_eq!(
            *doubled.borrow(),
            vec![vec![
                8,
                SIN_NAME,
                0x1e,
                q[0],
                q[1],
                0x1234_007f,
                7,
                w[0],
                w[1]
            ]]
        );
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn start_one_arg_error_handling_fills_the_exception_and_loads_its_result() {
        let (mut e, _) = engine(false, 7);
        let seen: Shared<(u32, Vec<u8>, u16)> = Rc::default();
        let record = seen.clone();
        e.register_double(EXCEPT_87, move |e, a| {
            record
                .borrow_mut()
                .push((a[0], e.mem.bytes(a[1], 0x20), e.mem.u16(a[2])));
            // A handler that changes the result.
            e.mem.set_u64(a[1] + 0x18, 0x7ff0_0000_0000_0042);
            Ret::default()
        });
        set_x87_control_word(&mut e, 0x027f);
        let st0 = f64::from_bits(0x7ff0_0000_0000_0099);
        let x = pair(1.0e300f64.to_bits());
        let r = start_one_arg_error_handling(&mut e, 3, TAN_NAME, 0x20, st0, 0x0e7f, 0, x[0], x[1]);
        // FLD of the changed result: quiet.
        assert_eq!(r.to_bits(), 0x7ff8_0000_0000_0042);
        assert_eq!(
            x87_control_word(&mut e),
            0x0e7f,
            "the saved word loaded back"
        );
        let seen = seen.borrow();
        assert_eq!(seen.len(), 1);
        let (operation, bytes, word) = &seen[0];
        assert_eq!((*operation, *word), (0x20, 0x0e7f));
        let word_at =
            |i: usize| u32::from_le_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
        assert_eq!(word_at(0), 3);
        assert_eq!(word_at(4), TAN_NAME);
        assert_eq!([word_at(8), word_at(0xc)], x);
        // The result FSTP'd: quiet.
        assert_eq!([word_at(0x18), word_at(0x1c)], pair(0x7ff8_0000_0000_0099));
        set_x87_control_word(&mut e, crate::x87::DEFAULT_CONTROL_WORD);
    }

    #[test]
    fn except_87_maps_the_types_to_exception_flags() {
        let (mut e, errno) = engine(false, 8);
        let handled = Rc::new(Cell::new(1u32));
        let answer = handled.clone();
        e.register_double(HANDLE_EXC, move |_, _| answer.get().into_ret());
        e.register(RAISE_EXC, |_, _| Ret::default());
        let exception = e.mem.alloc(0x20);
        let word = e.mem.alloc(4);
        e.mem.set_u32(word, 0xffff_0e7f);
        for (kind, flags) in [
            (1u32, Some(8u32)),
            (2, Some(4)),
            (3, Some(0x11)),
            (4, Some(0x12)),
            (5, Some(8)),
            (6, None),
            (7, None),
            (8, Some(0x10)),
            (0, None),
            (9, None),
        ] {
            e.mem.set_u32(exception, kind);
            e.mem.set_u32(errno, 0);
            clear_log(&mut e);
            except_87(&mut e, 0x12, Ptr::new(exception), Ptr::new(word));
            let seen = calls(&e, HANDLE_EXC);
            match flags {
                Some(flags) => assert_eq!(seen, vec![vec![flags, exception + 0x18, 0x0e7f]]),
                None => assert!(seen.is_empty(), "type {kind}"),
            }
            assert_eq!(calls(&e, CTRLFP), vec![vec![0x0e7f, 0xffff]]);
            // Type 7 becomes 1 (domain).
            let after = if kind == 7 { 1 } else { kind };
            assert_eq!(e.mem.u32(exception), after);
            assert_eq!(calls(&e, SET_ERRNO_FROM_MATHERR), vec![vec![after]]);
            assert!(
                calls(&e, DEFAULT_MATHERR).is_empty(),
                "the skip word is set"
            );
            assert!(calls(&e, RAISE_EXC).is_empty());
        }
        assert_eq!(e.mem.u32(errno), 0, "type 9: no errno");
        // Without the skip word the default _matherr answers 0: errno is set; type 8 skips it.
        e.set_global(MATHERR_SKIP, 0u32);
        e.mem.set_u32(exception, 1);
        clear_log(&mut e);
        except_87(&mut e, 0x12, Ptr::new(exception), Ptr::new(word));
        assert_eq!(calls(&e, DEFAULT_MATHERR), vec![vec![exception]]);
        assert_eq!(e.mem.u32(errno), EDOM);
        e.mem.set_u32(exception, 8);
        clear_log(&mut e);
        except_87(&mut e, 0x12, Ptr::new(exception), Ptr::new(word));
        assert!(calls(&e, DEFAULT_MATHERR).is_empty());
        // Not handled: __raise_exc with the block, the word's slot, the flags, the operation and
        // the argument and result addresses; operations 0x10, 0x16 and 0x1d store the second
        // argument in the block and set its word's low bits.
        handled.set(0);
        for operation in [0x12u32, 0x10, 0x16, 0x1d] {
            e.mem.set_u32(exception, 2);
            e.mem.set_u64(exception + 0x10, 0x7ff0_0000_0000_0005);
            let blocks: Shared<(u32, u64)> = Rc::default();
            let record = blocks.clone();
            e.register_double(RAISE_EXC, move |e, a| {
                record
                    .borrow_mut()
                    .push((e.mem.u32(a[0] + 0x40), e.mem.u64(a[0] + 0x30)));
                assert_eq!(e.mem.u32(a[1]), 0x0e7f, "the word's slot");
                Ret::default()
            });
            clear_log(&mut e);
            except_87(&mut e, operation, Ptr::new(exception), Ptr::new(word));
            let raised = calls(&e, RAISE_EXC);
            assert_eq!(raised.len(), 1);
            assert_eq!(
                raised[0][2..],
                [4, operation, exception + 8, exception + 0x18]
            );
            let (field, second) = blocks.borrow()[0];
            if operation == 0x12 {
                assert_eq!((field, second), (0, 0));
            } else {
                assert_eq!((field, second), (3, 0x7ff8_0000_0000_0005));
            }
        }
    }

    // ---- SSE2 -----------------------------------------------------------------------------

    /// The cores' main paths as the disassembly lists them (from the first constant load to the
    /// last instruction before the result is stored).
    const COS_LISTING: &str = "MOVLPD XMM1,qword ptr [0x010f8db0];MULSD XMM1,XMM0;\
        MOVLPD XMM2,qword ptr [0x010f8db8];CVTSD2SI EDX,XMM1;ADDSD XMM1,XMM2;\
        MOVLPD XMM3,qword ptr [0x010f8dd0];SUBSD XMM1,XMM2;MOVAPD XMM2,xmmword ptr [0x010f8dc0];\
        MULSD XMM3,XMM1;UNPCKLPD XMM1,XMM1;ADD EDX,0x1c7610;MOVSD XMM4,XMM0;AND EDX,0x3f;\
        MOVAPD XMM5,xmmword ptr [0x010f8da0];LEA EAX,[0x10f8570];SHL EDX,0x5;ADD EAX,EDX;\
        MULPD XMM2,XMM1;SUBSD XMM0,XMM3;MULSD XMM1,qword ptr [0x010f8dd8];SUBSD XMM4,XMM3;\
        MOVLPD XMM7,qword ptr [EAX + 0x8];UNPCKLPD XMM0,XMM0;MOVSD XMM3,XMM4;SUBSD XMM4,XMM2;\
        MULPD XMM5,XMM0;SUBPD XMM0,XMM2;MOVAPD XMM6,xmmword ptr [0x010f8d80];MULSD XMM7,XMM4;\
        SUBSD XMM3,XMM4;MULPD XMM5,XMM0;MULPD XMM0,XMM0;SUBSD XMM3,XMM2;\
        MOVAPD XMM2,xmmword ptr [EAX];SUBSD XMM1,XMM3;MOVLPD XMM3,qword ptr [EAX + 0x18];\
        ADDSD XMM2,XMM3;SUBSD XMM7,XMM2;MULSD XMM2,XMM4;MULPD XMM6,XMM0;MULSD XMM3,XMM4;\
        MULPD XMM2,XMM0;MULPD XMM0,XMM0;ADDPD XMM5,xmmword ptr [0x010f8d90];\
        MULSD XMM4,qword ptr [EAX];ADDPD XMM6,xmmword ptr [0x010f8d70];MULPD XMM5,XMM0;\
        MOVSD XMM0,XMM3;ADDSD XMM3,qword ptr [EAX + 0x8];MULSD XMM1,XMM7;MOVSD XMM7,XMM4;\
        ADDSD XMM4,XMM3;ADDPD XMM6,XMM5;MOVLPD XMM5,qword ptr [EAX + 0x8];SUBSD XMM5,XMM3;\
        SUBSD XMM3,XMM4;ADDSD XMM1,qword ptr [EAX + 0x10];MULPD XMM6,XMM2;ADDSD XMM5,XMM0;\
        ADDSD XMM3,XMM7;ADDSD XMM1,XMM5;ADDSD XMM1,XMM3;ADDSD XMM1,XMM6;UNPCKHPD XMM6,XMM6;\
        ADDSD XMM1,XMM6;ADDSD XMM4,XMM1";

    const SIN_LISTING: &str = "MOVLPD XMM1,qword ptr [0x010f9660];MULSD XMM1,XMM0;\
        MOVLPD XMM2,qword ptr [0x010f9668];CVTSD2SI EDX,XMM1;ADDSD XMM1,XMM2;\
        MOVLPD XMM3,qword ptr [0x010f9680];SUBSD XMM1,XMM2;MOVAPD XMM2,xmmword ptr [0x010f9670];\
        MULSD XMM3,XMM1;UNPCKLPD XMM1,XMM1;ADD EDX,0x1c7600;MOVSD XMM4,XMM0;AND EDX,0x3f;\
        MOVAPD XMM5,xmmword ptr [0x010f9650];LEA EAX,[0x10f8e20];SHL EDX,0x5;ADD EAX,EDX;\
        MULPD XMM2,XMM1;SUBSD XMM0,XMM3;MULSD XMM1,qword ptr [0x010f9688];SUBSD XMM4,XMM3;\
        MOVLPD XMM7,qword ptr [EAX + 0x8];UNPCKLPD XMM0,XMM0;MOVSD XMM3,XMM4;SUBSD XMM4,XMM2;\
        MULPD XMM5,XMM0;SUBPD XMM0,XMM2;MOVAPD XMM6,xmmword ptr [0x010f9630];MULSD XMM7,XMM4;\
        SUBSD XMM3,XMM4;MULPD XMM5,XMM0;MULPD XMM0,XMM0;SUBSD XMM3,XMM2;\
        MOVAPD XMM2,xmmword ptr [EAX];SUBSD XMM1,XMM3;MOVLPD XMM3,qword ptr [EAX + 0x18];\
        ADDSD XMM2,XMM3;SUBSD XMM7,XMM2;MULSD XMM2,XMM4;MULPD XMM6,XMM0;MULSD XMM3,XMM4;\
        MULPD XMM2,XMM0;MULPD XMM0,XMM0;ADDPD XMM5,xmmword ptr [0x010f9640];\
        MULSD XMM4,qword ptr [EAX];ADDPD XMM6,xmmword ptr [0x010f9620];MULPD XMM5,XMM0;\
        MOVSD XMM0,XMM3;ADDSD XMM3,qword ptr [EAX + 0x8];MULSD XMM1,XMM7;MOVSD XMM7,XMM4;\
        ADDSD XMM4,XMM3;ADDPD XMM6,XMM5;MOVLPD XMM5,qword ptr [EAX + 0x8];SUBSD XMM5,XMM3;\
        SUBSD XMM3,XMM4;ADDSD XMM1,qword ptr [EAX + 0x10];MULPD XMM6,XMM2;ADDSD XMM5,XMM0;\
        ADDSD XMM3,XMM7;ADDSD XMM1,XMM5;ADDSD XMM1,XMM3;ADDSD XMM1,XMM6;UNPCKHPD XMM6,XMM6;\
        ADDSD XMM1,XMM6;ADDSD XMM4,XMM1";

    const TAN_LISTING: &str = "UNPCKLPD XMM0,XMM0;MOVAPD XMM1,xmmword ptr [0x010fc4e0];\
        MULPD XMM1,XMM0;CVTSD2SI EDX,XMM1;MOVAPD XMM2,xmmword ptr [0x010fc4f0];ADDPD XMM1,XMM2;\
        MOVAPD XMM3,xmmword ptr [0x010fc500];SUBPD XMM1,XMM2;MOVLPD XMM5,qword ptr [0x010fc530];\
        ADD EDX,0x72900;MOVAPD XMM4,xmmword ptr [0x010fc510];MULPD XMM3,XMM1;AND EDX,0x1f;\
        MULSD XMM5,XMM1;MOV ECX,EDX;MULPD XMM4,XMM1;SHL ECX,0x1;SUBPD XMM0,XMM3;\
        MULPD XMM1,xmmword ptr [0x010fc520];ADD EDX,ECX;SHL ECX,0x2;ADD EDX,ECX;ADDSD XMM5,XMM0;\
        MOVAPD XMM2,XMM0;SUBPD XMM0,XMM4;MOVLPD XMM6,qword ptr [0x010fc538];SHL EDX,0x4;\
        LEA EAX,[0x10faee0];ANDPD XMM5,xmmword ptr [0x010fc540];MOVAPD XMM3,XMM0;ADD EAX,EDX;\
        SUBPD XMM2,XMM0;UNPCKHPD XMM0,XMM0;DIVSD XMM6,XMM5;SUBPD XMM2,XMM4;\
        MOVAPD XMM7,xmmword ptr [EAX + 0x10];SUBSD XMM3,XMM5;MULPD XMM7,XMM0;SUBPD XMM2,XMM1;\
        MOVAPD XMM1,xmmword ptr [EAX + 0x30];MULPD XMM1,XMM0;MOVAPD XMM4,xmmword ptr [EAX + 0x60];\
        MULPD XMM4,XMM0;ADDSD XMM2,XMM3;MOVAPD XMM3,XMM0;MULPD XMM0,XMM0;\
        ADDPD XMM7,xmmword ptr [EAX];ADDPD XMM1,xmmword ptr [EAX + 0x20];MULPD XMM1,XMM0;\
        ADDPD XMM4,xmmword ptr [EAX + 0x50];ADDPD XMM7,XMM1;MOVAPD XMM1,xmmword ptr [EAX + 0x70];\
        MULPD XMM1,XMM0;MULPD XMM0,XMM0;ADDPD XMM4,XMM1;MOVAPD XMM1,xmmword ptr [EAX + 0x40];\
        MULPD XMM1,XMM0;ADDPD XMM7,XMM1;MOVAPD XMM1,XMM3;MULPD XMM3,XMM0;MULSD XMM0,XMM0;\
        MULPD XMM1,xmmword ptr [EAX + 0x90];MULPD XMM4,XMM3;MOVSD XMM3,XMM1;ADDPD XMM7,XMM4;\
        MOVSD XMM4,XMM1;MULSD XMM0,XMM7;UNPCKHPD XMM7,XMM7;ADDSD XMM0,XMM7;UNPCKHPD XMM1,XMM1;\
        ADDSD XMM3,XMM1;SUBSD XMM4,XMM3;ADDSD XMM1,XMM4;MOVSD XMM4,XMM2;\
        MOVLPD XMM7,qword ptr [EAX + 0x90];UNPCKHPD XMM2,XMM2;ADDSD XMM7,qword ptr [EAX + 0x98];\
        MULSD XMM7,XMM2;ADDSD XMM7,qword ptr [EAX + 0x88];ADDSD XMM7,XMM1;ADDSD XMM0,XMM7;\
        MOVLPD XMM7,qword ptr [0x010fc538];MULSD XMM4,XMM6;MOVLPD XMM2,qword ptr [EAX + 0xa8];\
        ANDPD XMM2,XMM6;MULSD XMM5,XMM2;MULSD XMM6,qword ptr [EAX + 0xa0];SUBSD XMM7,XMM5;\
        SUBSD XMM2,qword ptr [EAX + 0x80];SUBSD XMM7,XMM4;MULSD XMM7,XMM6;MOVSD XMM4,XMM3;\
        SUBSD XMM3,XMM2;ADDSD XMM2,XMM3;SUBSD XMM4,XMM2;ADDSD XMM0,XMM4;SUBSD XMM0,XMM7;\
        ADDSD XMM0,XMM3";

    /// A machine that runs those listings one instruction at a time: eight XMM registers of
    /// two lanes (as bits) and the three general registers they use.
    struct Machine {
        xmm: [[u64; 2]; 8],
        eax: u32,
        ecx: u32,
        edx: u32,
    }

    #[derive(Clone, Copy)]
    enum Operand {
        Xmm(usize),
        Gpr(u8),
        Imm(u32),
        Mem(u32),
    }

    impl Machine {
        fn gpr(&mut self, name: u8) -> &mut u32 {
            match name {
                b'A' => &mut self.eax,
                b'C' => &mut self.ecx,
                _ => &mut self.edx,
            }
        }

        fn operand(&self, text: &str) -> Operand {
            let text = text.trim();
            if let Some(n) = text.strip_prefix("XMM") {
                return Operand::Xmm(n.parse().unwrap());
            }
            if let Some(hex) = text.strip_prefix("0x") {
                return Operand::Imm(u32::from_str_radix(hex, 16).unwrap());
            }
            if let Some(open) = text.find('[') {
                let inner = &text[open + 1..text.len() - 1];
                let hex = |s: &str| u32::from_str_radix(&s[2..], 16).unwrap();
                let address = match inner.split_once(" + ") {
                    Some((_, offset)) => self.eax.wrapping_add(hex(offset)),
                    None if inner == "EAX" => self.eax,
                    None => hex(inner),
                };
                return Operand::Mem(address);
            }
            Operand::Gpr(text.as_bytes()[1])
        }

        /// One lane of an arithmetic instruction, by the SSE2 rules.
        fn lane(op: &str, dest: u64, src: u64) -> u64 {
            let (a, b) = (f64::from_bits(dest), f64::from_bits(src));
            if a.is_nan() {
                return dest | QUIET;
            }
            if b.is_nan() {
                return src | QUIET;
            }
            let r = match op {
                "ADD" => a + b,
                "SUB" => a - b,
                "MUL" => a * b,
                _ => a / b,
            };
            if r.is_nan() {
                DEFAULT_NAN
            } else {
                r.to_bits()
            }
        }

        fn source(&self, e: &Engine, src: Operand) -> [u64; 2] {
            match src {
                Operand::Xmm(r) => self.xmm[r],
                Operand::Mem(a) => [e.mem.u64(a), e.mem.u64(a + 8)],
                _ => panic!("not a vector operand"),
            }
        }

        fn run(&mut self, e: &Engine, listing: &str) {
            for line in listing.split(';') {
                let line = line.trim();
                let (mnemonic, rest) = line.split_once(' ').unwrap();
                let (dest, src) = rest.split_once(',').unwrap();
                let dest = self.operand(dest);
                let src = self.operand(src);
                match (mnemonic, dest) {
                    ("MOVLPD", Operand::Xmm(d)) => self.xmm[d][0] = self.source(e, src)[0],
                    ("MOVAPD", Operand::Xmm(d)) => self.xmm[d] = self.source(e, src),
                    ("MOVSD", Operand::Xmm(d)) => self.xmm[d][0] = self.source(e, src)[0],
                    ("UNPCKLPD", Operand::Xmm(d)) => {
                        self.xmm[d] = [self.xmm[d][0], self.source(e, src)[0]];
                    }
                    ("UNPCKHPD", Operand::Xmm(d)) => {
                        self.xmm[d] = [self.xmm[d][1], self.source(e, src)[1]];
                    }
                    ("ANDPD", Operand::Xmm(d)) => {
                        let s = self.source(e, src);
                        self.xmm[d] = [self.xmm[d][0] & s[0], self.xmm[d][1] & s[1]];
                    }
                    ("CVTSD2SI", Operand::Gpr(r)) => {
                        let value = f64::from_bits(self.source(e, src)[0]);
                        *self.gpr(r) = crate::x87::fistp(value) as u32;
                    }
                    (m, Operand::Xmm(d)) if m.ends_with("SD") => {
                        let s = self.source(e, src);
                        self.xmm[d][0] = Self::lane(&m[..3], self.xmm[d][0], s[0]);
                    }
                    (m, Operand::Xmm(d)) if m.ends_with("PD") => {
                        let s = self.source(e, src);
                        for lane in 0..2 {
                            self.xmm[d][lane] = Self::lane(&m[..3], self.xmm[d][lane], s[lane]);
                        }
                    }
                    (m, Operand::Gpr(r)) => {
                        let value = match src {
                            Operand::Imm(v) | Operand::Mem(v) => v,
                            Operand::Gpr(g) => *self.gpr(g),
                            Operand::Xmm(_) => panic!("{line}"),
                        };
                        let reg = self.gpr(r);
                        *reg = match m {
                            "ADD" => reg.wrapping_add(value),
                            "AND" => *reg & value,
                            "SHL" => *reg << value,
                            "MOV" | "LEA" => value,
                            _ => panic!("{line}"),
                        };
                    }
                    _ => panic!("{line}"),
                }
            }
        }
    }

    /// The listing run on `x` with every other register lane holding noise: the low lane of
    /// the result register as `FLD` loads it.
    fn listing_result(e: &Engine, listing: &str, result: usize, x: f64, rng: &mut Rng) -> u64 {
        let mut m = Machine {
            xmm: [[0; 2]; 8],
            eax: rng.next() as u32,
            ecx: rng.next() as u32,
            edx: rng.next() as u32,
        };
        for r in m.xmm.iter_mut() {
            *r = [rng.next(), rng.next()];
        }
        m.xmm[0][0] = x.to_bits();
        m.run(e, listing);
        x87_load64(m.xmm[result][0]).to_bits()
    }

    fn top_word(x: f64) -> u16 {
        (x.to_bits() >> 48) as u16 & 0x7fff
    }

    #[test]
    fn the_sse2_cores_follow_their_listings_bit_for_bit() {
        let mut rng = Rng(0x0123_4567_89ab_cdef);
        for seed in [11u64, 12, 13] {
            let (mut e, _) = engine(true, seed);
            let mut arguments = vec![
                // The ends of the main paths: top words 0x3030 to 0x40f5 for the sine and
                // cosine, 0x3820 to 0x40c8 for the tangent.
                f64::from_bits(0x3030_0000_0000_0000),
                f64::from_bits(0x40f5_ffff_ffff_ffff),
                f64::from_bits(0xc0f5_0000_0000_0001),
                f64::from_bits(0x3820_0000_0000_0000),
                f64::from_bits(0x40c8_ffff_ffff_ffff),
                0.5,
                -1.25,
                100.0,
            ];
            for _ in 0..300 {
                arguments.push(rng.double(0x382..=0x40c));
            }
            for _ in 0..100 {
                arguments.push(rng.double(0x303..=0x40f));
            }
            for &x in &arguments {
                let top = top_word(x);
                if (0x3030..=0x40f5).contains(&top) {
                    let want = listing_result(&e, COS_LISTING, 4, x, &mut rng);
                    assert_eq!(cos_sse2_core(&mut e, x).to_bits(), want, "cos({x:e})");
                    let want = listing_result(&e, SIN_LISTING, 4, x, &mut rng);
                    assert_eq!(sin_sse2_core(&mut e, x).to_bits(), want, "sin({x:e})");
                }
                if (0x3820..=0x40c8).contains(&top) {
                    let want = listing_result(&e, TAN_LISTING, 0, x, &mut rng);
                    assert_eq!(tan_sse2_core(&mut e, x).to_bits(), want, "tan({x:e})");
                }
            }
            // The main paths call nothing.
            assert!(calls(&e, FLOAD_WITH_FB).is_empty());
        }
    }

    #[test]
    fn the_sse2_cores_carry_nans_and_infinities_from_their_tables() {
        let mut rng = Rng(0xfeed_beef_0bad_f00d);
        let (mut e, _) = engine(true, 21);
        // Some slots poisoned with NaNs (quiet and signalling), infinities, zeros and a
        // denormal; then the tangent's mask cleared so that its division is by zero.
        let specials = [
            0x7ff8_0000_0000_0011u64,
            0xfff0_0000_0000_0022,
            0x7ff0_0000_0000_0000,
            0xfff0_0000_0000_0000,
            0,
            1 << 63,
            1,
        ];
        for at in (SSE_DATA..SSE_DATA + SSE_DATA_LEN).step_by(8) {
            if rng.next() % 7 == 3 {
                let pick = specials[(rng.next() % specials.len() as u64) as usize];
                e.mem.set_u64(at, pick);
            }
        }
        for clear_mask in [false, true] {
            if clear_mask {
                e.mem.set_u64(TAN_MASK, 0);
            }
            for _ in 0..200 {
                let x = rng.double(0x382..=0x40c);
                let top = top_word(x);
                if top <= 0x40c8 {
                    let want = listing_result(&e, TAN_LISTING, 0, x, &mut rng);
                    assert_eq!(tan_sse2_core(&mut e, x).to_bits(), want, "tan({x:e})");
                }
                if top <= 0x40f5 {
                    let want = listing_result(&e, COS_LISTING, 4, x, &mut rng);
                    assert_eq!(cos_sse2_core(&mut e, x).to_bits(), want, "cos({x:e})");
                    let want = listing_result(&e, SIN_LISTING, 4, x, &mut rng);
                    assert_eq!(sin_sse2_core(&mut e, x).to_bits(), want, "sin({x:e})");
                }
            }
        }
        // The default NaN of an invalid operation and the NaN rules themselves.
        assert_eq!(subsd(f64::INFINITY, f64::INFINITY).to_bits(), DEFAULT_NAN);
        assert_eq!(mulsd(0.0, f64::NEG_INFINITY).to_bits(), DEFAULT_NAN);
        assert_eq!(divsd(0.0, -0.0).to_bits(), DEFAULT_NAN);
        assert_eq!(divsd(1.0, -0.0), f64::NEG_INFINITY);
        let a = f64::from_bits(0x7ff0_0000_0000_0001);
        let b = f64::from_bits(0xfff8_0000_0000_0002);
        assert_eq!(addsd(a, b).to_bits(), 0x7ff8_0000_0000_0001);
        assert_eq!(addsd(b, a).to_bits(), 0xfff8_0000_0000_0002);
        assert_eq!(mulsd(2.0, a).to_bits(), 0x7ff8_0000_0000_0001);
        assert_eq!(cvtsd2si(2.5), 2);
        assert_eq!(cvtsd2si(-3.5), (-4i32) as u32);
        assert_eq!(cvtsd2si(f64::NAN), 0x8000_0000);
        assert_eq!(cvtsd2si(3.0e9), 0x8000_0000);
    }

    #[test]
    fn the_sse2_cores_take_tiny_arguments_themselves() {
        let (mut e, _) = engine(true, 31);
        let cos_tiny = m64(&e, COS_TINY);
        let sin_denormal = m64(&e, SIN_DENORMAL_FACTOR);
        let tan_factor = m64(&e, TAN_TINY_FACTOR);
        let tan_scale = m64(&e, TAN_TINY_SCALE);
        let tiny = [
            0.0,
            -0.0,
            f64::from_bits(1),
            -f64::from_bits(0x000f_ffff_ffff_ffff),
            f64::MIN_POSITIVE,
            -1.0e-200,
            f64::from_bits(0x302f_ffff_ffff_ffff),
            f64::from_bits(0xb81f_ffff_ffff_ffff),
        ];
        for x in tiny {
            // Every value is below the tangent's edge (top word 0x3820); the last is above the
            // sine's and cosine's (0x3030).
            if top_word(x) < 0x3030 {
                // cos: the constant less |x|.
                assert_eq!(
                    cos_sse2_core(&mut e, x).to_bits(),
                    (cos_tiny - x.abs()).to_bits(),
                    "cos({x:e})"
                );
                // sin: x times the constant for a zero or denormal, else x itself.
                let want = if top_word(x) < 0x10 {
                    x * sin_denormal
                } else {
                    x
                };
                assert_eq!(
                    sin_sse2_core(&mut e, x).to_bits(),
                    want.to_bits(),
                    "sin({x:e})"
                );
            }
            // tan: (factor * x + x) * scale.
            let want = (tan_factor * x + x) * tan_scale;
            assert_eq!(
                tan_sse2_core(&mut e, x).to_bits(),
                want.to_bits(),
                "tan({x:e})"
            );
        }
        // Between the tangent's and the sine's lower edges the tangent is tiny and the sine in
        // its main path.
        let x = f64::from_bits(0x3810_0000_0000_0000);
        assert_eq!(
            tan_sse2_core(&mut e, x).to_bits(),
            ((tan_factor * x + x) * tan_scale).to_bits()
        );
        assert!(calls(&e, FLOAD_WITH_FB).is_empty());
    }

    #[test]
    fn the_sse2_cores_hand_large_and_special_arguments_to_the_x87_path() {
        for (core, entry, register, trig, edge) in [
            (
                COS_CORE,
                0x00ec_9f30u32,
                0x00ec_9f70u32,
                Trig::Cos,
                0x40f6_0000_0000_0000u64,
            ),
            (
                SIN_CORE,
                0x00ec_a060,
                0x00ec_a0a0,
                Trig::Sin,
                0x40f6_0000_0000_0000,
            ),
            (
                TAN_CORE,
                0x00ec_a590,
                0x00ec_a5d0,
                Trig::Tan,
                0x40c9_0000_0000_0000,
            ),
        ] {
            let (mut e, errno) = engine(true, 41);
            for x in [
                f64::from_bits(edge),
                -f64::from_bits(edge),
                1.0e10,
                1.0e300,
                f64::MAX,
            ] {
                clear_log(&mut e);
                let got = e.call(core, &args![x]).f64();
                let want = x87_reference(trig, x);
                assert_eq!(got.to_bits(), want.to_bits(), "{core:08x}({x:e})");
                // __fload_withFB got the double where it lies; the exit returns at 0x027f.
                let loads = calls(&e, FLOAD_WITH_FB);
                assert_eq!(loads.len(), 1);
                assert_eq!(e.mem.u64(loads[0][1]), x.to_bits());
                assert_eq!(calls(&e, MATH_EXIT).len(), 1);
                // The stack entry and the register entry reach the same.
                assert_eq!(e.call(entry, &args![x]).f64().to_bits(), got.to_bits());
                assert_eq!(e.call(register, &args![x]).f64().to_bits(), got.to_bits());
            }
            // An infinity and a NaN: the x87 path's error report, errno EDOM.
            e.mem.set_u32(errno, 0);
            assert_eq!(
                e.call(core, &args![f64::NEG_INFINITY]).f64().to_bits(),
                0xfff8_0000_0000_0000
            );
            assert_eq!(e.mem.u32(errno), EDOM);
            let signalling = f64::from_bits(0x7ff0_0000_0000_0077);
            assert_eq!(
                e.call(entry, &args![signalling]).f64().to_bits(),
                0x7ff8_0000_0000_0077
            );
            // The register entry stores ST0 first (quiet), then the same.
            assert_eq!(
                e.call(register, &args![signalling]).f64().to_bits(),
                0x7ff8_0000_0000_0077
            );
        }
    }

    /// With `NV_EXE` set to the installed FalloutNV.exe 1.4.0.525, on the exe's own tables:
    /// the SSE2 and the x87 path of each entry agree to within rounding (a check of the table
    /// reads and the index arithmetic on the real constants against the FPU's instructions).
    #[test]
    fn with_the_installed_exe_the_sse2_and_x87_paths_agree() {
        let Ok(path) = std::env::var("NV_EXE") else {
            return;
        };
        let mut e = Engine::with_exe(std::path::Path::new(&path)).unwrap();
        let mut rng = Rng(0x5151_5151_5151_5151);
        let mut arguments: Vec<f64> = (0..2000).map(|_| rng.double(0x3c0..=0x408)).collect();
        arguments.extend((0..200).map(|_| rng.double(0x300..=0x3c0)));
        for (entry, _, _) in ENTRIES {
            for &x in &arguments {
                e.set_global(SSE2_FLAG, 1u32);
                let sse2 = e.call(entry, &args![x]).f64();
                e.set_global(SSE2_FLAG, 0u32);
                let x87 = e.call(entry, &args![x]).f64();
                let scale = sse2.abs().max(x87.abs()).max(1.0);
                assert!(
                    (sse2 - x87).abs() <= 1.0e-14 * scale,
                    "{entry:08x}({x:e}): SSE2 {sse2:e}, x87 {x87:e}"
                );
            }
        }
    }
}
