//! `(unplaced)/static_init_irregular_j` (no source unit: the compiler's dynamic initializers),
//! subsystem `compiler: static init/exit`: the last 31 initializers of FalloutNV.exe 1.4.0.525
//! that `research/initgen` refused and start-up skipped, each of its own shape. Part 10 of the
//! list. Decoded from `read_memory` bytes (Ghidra made no function of them; `00fbffc0` and
//! `00fc72e0` read by hand: `SETZ AL` and `REP STOSD`), every instruction translated in order.
//!
//! Two are C initializers (`_initterm_e`, `010113c4..010113e8`) and answer `EAX`, which
//! `__cinit` tests: `___initmbctable` (`00ed417e`) and `00edd4e7` both answer 0. The rest are
//! cdecl C++ initializers with no parameters whose `EAX` `_initterm` ignores. None has an
//! `FS:[0]` frame or a `/GS` check.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_atexit` (`00ec658f`, cdecl, one word: the function to run at exit).
const ATEXIT: u32 = 0x00ec_658f;
/// The float `Setting` constructor (`0044f440`, thiscall, `RET 8`: `this`, name, float bits).
const FLOAT_SETTING_CONSTRUCT: u32 = 0x0044_f440;
/// `__setmbcp` (`00ed3fe4`, cdecl, one word: the code page).
const SETMBCP: u32 = 0x00ed_3fe4;
/// The `SetUnhandledExceptionFilter` import slot (`00fdf180`, stdcall, one word).
const SET_UNHANDLED_EXCEPTION_FILTER: u32 = 0x00fd_f180;
/// The `CreateSemaphoreA` import slot (`00fdf0b4`, stdcall, four words).
const CREATE_SEMAPHORE_A: u32 = 0x00fd_f0b4;
/// The `InterlockedIncrement` import slot (`00fdf1c0`, stdcall, one word).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// `__iob_func` (`00ecdf44`, cdecl, no arguments: the CRT's stream table).
const IOB_FUNC: u32 = 0x00ec_df44;
/// `00ec3a4f` (thiscall, `RET 4`: `this`, a stream).
const FN_00EC3A4F: u32 = 0x00ec_3a4f;
/// `00a6db50` (thiscall, `RET 0x10`: `this`, four float words).
const FN_00A6DB50: u32 = 0x00a6_db50;
/// `00eab8a0` (thiscall, `RET 8`: `this`, a float word, a word).
const FN_00EAB8A0: u32 = 0x00ea_b8a0;
/// `00b00e40` (thiscall, `RET 4`: `this`, a word).
const FN_00B00E40: u32 = 0x00b0_0e40;
/// `00ca9360` (Ghidra `operator_`, thiscall, `RET 4`: `this`, a pointer to a float).
const FN_00CA9360: u32 = 0x00ca_9360;
/// `00aa1070` (cdecl, one word: a size; answers a block).
const FN_00AA1070: u32 = 0x00aa_1070;
/// `_memset` (`00ec61c0`, cdecl: destination, byte, count).
const MEMSET: u32 = 0x00ec_61c0;
/// The empty string's data word the copies below compare with and copy (`0109b220`).
const EMPTY: u32 = 0x0109_b220;

// Translated from 00ed417e (disassembled, FalloutNV.exe 1.4.0.525)
/// `___initmbctable`: when the word at `01271a90` is 0, `__setmbcp(-3)` and the word set to 1.
/// Answers 0.
pub fn fn_00ed417e(e: &mut Engine) -> u32 {
    if e.global::<u32>(0x0127_1a90) == 0 {
        e.call(SETMBCP, &args![(-3i32) as u32]);
        e.set_global::<u32>(0x0127_1a90, 1);
    }
    0
}

// Translated from 00edd4e7 (disassembled, FalloutNV.exe 1.4.0.525)
/// `SetUnhandledExceptionFilter(00edd4a5)`; answers 0.
pub fn fn_00edd4e7(e: &mut Engine) -> u32 {
    e.call(SET_UNHANDLED_EXCEPTION_FILTER, &args![0x00ed_d4a5u32]);
    0
}

// Translated from 00fc78de (disassembled, FalloutNV.exe 1.4.0.525)
/// `00ec3a4f` on `0126fbc0` with the third stream (`__iob_func() + 0x40`); `_atexit(00fde56d)`.
pub fn fn_00fc78de(e: &mut Engine) {
    let streams = e.call(IOB_FUNC, &args![]).u32();
    e.call(
        FN_00EC3A4F,
        &args![0x0126_fbc0u32, streams.wrapping_add(0x40)],
    );
    e.call(ATEXIT, &args![0x00fd_e56du32]);
}

// Translated from 00fb49d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float at `011f3658`: 1 divided by the integer at `011a7ed4` (`FILD; FLD1; FDIVRP
/// ST(1),ST(0)`, bytes `de f1`: `ST1 = ST0 / ST1`, popped).
pub fn fn_00fb49d0(e: &mut Engine) {
    let n = f64::from(e.global::<i32>(0x011a_7ed4));
    e.set_global::<u32>(0x011f_3658, x87_store32(x87_div(1.0, n)));
}

// Translated from 00fb49f0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Copies the three words at `011f426c` to `011f365c` (all three loaded first).
pub fn fn_00fb49f0(e: &mut Engine) {
    let a = e.global::<u32>(0x011f_426c);
    let b = e.global::<u32>(0x011f_4270);
    let c = e.global::<u32>(0x011f_4274);
    e.set_global::<u32>(0x011f_365c, a);
    e.set_global::<u32>(0x011f_3660, b);
    e.set_global::<u32>(0x011f_3664, c);
}

/// `FLD float ptr [value]; SUB ESP,0x10; FST` it to the four words; `MOV ECX,this; CALL
/// 00a6db50`: the float (a signalling NaN quieted) four times.
fn four_floats(e: &mut Engine, value: u32, this: u32) {
    let f = x87_quiet_bits(e.global::<u32>(value));
    e.call(FN_00A6DB50, &args![this, f, f, f, f]);
}

// Translated from 00fb4b60 (disassembled, FalloutNV.exe 1.4.0.525)
/// `00a6db50` on `011f3704` with the float at `01096b8c` four times.
pub fn fn_00fb4b60(e: &mut Engine) {
    four_floats(e, 0x0109_6b8c, 0x011f_3704);
}

// Translated from 00fb5530 (disassembled, FalloutNV.exe 1.4.0.525)
/// `00a6db50` on `011f4208` with the float at `01015f5c` four times.
pub fn fn_00fb5530(e: &mut Engine) {
    four_floats(e, 0x0101_5f5c, 0x011f_4208);
}

// Translated from 00fb5820 (disassembled, FalloutNV.exe 1.4.0.525)
/// Ten zero words from `011f4514`; `_atexit(00fdaf10)`.
pub fn fn_00fb5820(e: &mut Engine) {
    for k in 0..10u32 {
        e.set_global::<u32>(0x011f_4514 + 4 * k, 0);
    }
    e.call(ATEXIT, &args![0x00fd_af10u32]);
}

// Translated from 00fb58b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011f4790` = 0, `011f4794` = 1, `011f4798` = `CreateSemaphoreA(0, 0, 1, 0)`;
/// `_atexit(00fdaf60)`.
pub fn fn_00fb58b0(e: &mut Engine) {
    e.set_global::<u32>(0x011f_4790, 0);
    e.set_global::<u32>(0x011f_4794, 1);
    let handle = e
        .call(CREATE_SEMAPHORE_A, &args![0u32, 0u32, 1u32, 0u32])
        .u32();
    e.set_global::<u32>(0x011f_4798, handle);
    e.call(ATEXIT, &args![0x00fd_af60u32]);
}

// Translated from 00fb59a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The word at `0109b220` in the ten words from `011f49b0`; `_atexit(00fdaf80)`.
pub fn fn_00fb59a0(e: &mut Engine) {
    let v = e.global::<u32>(EMPTY);
    for k in 0..10u32 {
        e.set_global::<u32>(0x011f_49b0 + 4 * k, v);
    }
    e.call(ATEXIT, &args![0x00fd_af80u32]);
}

// Translated from 00fb6270 (disassembled, FalloutNV.exe 1.4.0.525)
/// The word at `0109b220` in the four words from `011f5e9c`; `_atexit(00fdb170)`.
pub fn fn_00fb6270(e: &mut Engine) {
    let v = e.global::<u32>(EMPTY);
    for k in 0..4u32 {
        e.set_global::<u32>(0x011f_5e9c + 4 * k, v);
    }
    e.call(ATEXIT, &args![0x00fd_b170u32]);
}

/// `00fba2e0`'s divisors and the floats each quotient goes to, in the code's order.
const ZERO_QUOTIENTS: [(u32, &[u32]); 6] = [
    (
        0x010b_8cd8,
        &[
            0x011a_df20,
            0x011a_df30,
            0x011a_df40,
            0x011a_df50,
            0x011a_df60,
            0x011a_df70,
            0x011a_dfb0,
            0x011a_dfc0,
            0x011a_dfd0,
            0x011a_dfe0,
            0x011a_dff0,
            0x011a_e000,
        ],
    ),
    (
        0x010b_8cd0,
        &[
            0x011a_e010,
            0x011a_e020,
            0x011a_e030,
            0x011a_e040,
            0x011a_e050,
            0x011a_e0b0,
            0x011a_e0c0,
            0x011a_e0d0,
            0x011a_e0e0,
            0x011a_e0f0,
        ],
    ),
    (
        0x010b_8cc8,
        &[
            0x011a_e100,
            0x011a_e110,
            0x011a_e120,
            0x011a_e130,
            0x011a_e1b0,
            0x011a_e1c0,
            0x011a_e1d0,
            0x011a_e1e0,
        ],
    ),
    (
        0x010b_8cc0,
        &[
            0x011a_e1f0,
            0x011a_e200,
            0x011a_e210,
            0x011a_e2b0,
            0x011a_e2c0,
            0x011a_e2d0,
        ],
    ),
    (
        0x010b_8cb8,
        &[0x011a_e2e0, 0x011a_e2f0, 0x011a_e3b0, 0x011a_e3c0],
    ),
    (0x010b_8cb0, &[0x011a_e3d0, 0x011a_e4b0]),
];

// Translated from 00fba2e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// Zero divided by each of six doubles, each quotient rounded through the local and stored in
/// its floats ([`ZERO_QUOTIENTS`]). `FLDZ` stays in `ST1` for the first five (`FLD double;
/// FDIVR ST(0),ST(1)`, bytes `d8 f9`: `ST0 = ST1 / ST0`); the sixth divides the zero itself
/// (`FDIV double ptr [010b8cb0]`). Every quotient is `0 / d` with zero the dividend.
pub fn fn_00fba2e0(e: &mut Engine) {
    for (k, &(divisor, floats)) in ZERO_QUOTIENTS.iter().enumerate() {
        let quotient = if k < 5 {
            // FLD double ptr [divisor]; FDIVR ST1.
            x87_div(0.0, x87_load64(e.global::<u64>(divisor)))
        } else {
            // FDIV double ptr [divisor] on the FLDZ.
            x87_div(0.0, e.global::<f64>(divisor))
        };
        // FSTP float ptr [ESP]; FLD float ptr [ESP].
        let bits = x87_store32(x87_load32(x87_store32(quotient)));
        for &g in floats {
            e.set_global::<u32>(g, bits);
        }
    }
}

// Translated from 00fbab30 (disassembled, FalloutNV.exe 1.4.0.525)
/// The byte at `011ad80e` copied to `01201fd4`.
pub fn fn_00fbab30(e: &mut Engine) {
    let b = e.global::<u8>(0x011a_d80e);
    e.set_global::<u8>(0x0120_1fd4, b);
}

/// `PUSH word; MOV ECX,this; CALL 00b00e40; PUSH exit_fn; MOV [this],vtable; CALL _atexit`.
fn object_with_vtable(e: &mut Engine, this: u32, word: u32, vtable: u32, exit_fn: u32) {
    e.call(FN_00B00E40, &args![this, word]);
    e.set_global::<u32>(this, vtable);
    e.call(ATEXIT, &args![exit_fn]);
}

// Translated from 00fbcb40 (disassembled, FalloutNV.exe 1.4.0.525)
/// `00b00e40(0)` on `0126782c`, its vtable `010c3b68`; `_atexit(00fddd20)`.
pub fn fn_00fbcb40(e: &mut Engine) {
    object_with_vtable(e, 0x0126_782c, 0, 0x010c_3b68, 0x00fd_dd20);
}

// Translated from 00fbcb70 (disassembled, FalloutNV.exe 1.4.0.525)
/// `00b00e40(1)` on `01267b0c`, its vtable `010c3b74`; `_atexit(00fddd30)`.
pub fn fn_00fbcb70(e: &mut Engine) {
    object_with_vtable(e, 0x0126_7b0c, 1, 0x010c_3b74, 0x00fd_dd30);
}

// Translated from 00fbce90 (disassembled, FalloutNV.exe 1.4.0.525)
/// The float setting at `01267c84` (name `010c4ec4`) with 0.0 (`FLDZ`); `_atexit(00fddf10)`.
pub fn fn_00fbce90(e: &mut Engine) {
    e.call(
        FLOAT_SETTING_CONSTRUCT,
        &args![0x0126_7c84u32, 0x010c_4ec4u32, 0u32],
    );
    e.call(ATEXIT, &args![0x00fd_df10u32]);
}

/// `PUSH 0x94; CALL 00aa1070`; the block's words cleared (`_memset(block, 0, [count] * 4)`,
/// the block stored at `+8` before the call), the vtable at `+0`; `_atexit(exit_fn)`.
fn cleared_table(e: &mut Engine, this: u32, count: u32, vtable: u32, exit_fn: u32) {
    let block = e.call(FN_00AA1070, &args![0x94u32]).u32();
    let n = e.global::<u32>(count);
    e.set_global::<u32>(this + 8, block);
    e.call(MEMSET, &args![block, 0u32, n.wrapping_mul(4)]);
    e.set_global::<u32>(this, vtable);
    e.call(ATEXIT, &args![exit_fn]);
}

// Translated from 00fbd4e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The table at `011b02f0` (count `011b02f4`, vtable `010c4c84`); `_atexit(00fdde70)`.
pub fn fn_00fbd4e0(e: &mut Engine) {
    cleared_table(e, 0x011b_02f0, 0x011b_02f4, 0x010c_4c84, 0x00fd_de70);
}

// Translated from 00fbd520 (disassembled, FalloutNV.exe 1.4.0.525)
/// The table at `011b0300` (count `011b0304`, vtable `010c4cc4`); `_atexit(00fdde80)`.
pub fn fn_00fbd520(e: &mut Engine) {
    cleared_table(e, 0x011b_0300, 0x011b_0304, 0x010c_4cc4, 0x00fd_de80);
}

// Translated from 00fbdff0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The table at `011b2190` (count `011b2194`, vtable `010c91c8`); `_atexit(00fde260)`.
pub fn fn_00fbdff0(e: &mut Engine) {
    cleared_table(e, 0x011b_2190, 0x011b_2194, 0x010c_91c8, 0x00fd_e260);
}

/// `MOVAPS XMM0,[from]; MOVAPS [to],XMM0`: 16 bytes copied.
fn copy16(e: &mut Engine, from: u32, to: u32) {
    for k in 0..4u32 {
        let w = e.global::<u32>(from + 4 * k);
        e.set_global::<u32>(to + 4 * k, w);
    }
}

// Translated from 00fbd560 (disassembled, FalloutNV.exe 1.4.0.525)
/// The 16 bytes at `010c7160` copied to `01267e30`.
pub fn fn_00fbd560(e: &mut Engine) {
    copy16(e, 0x010c_7160, 0x0126_7e30);
}

// Translated from 00fbde20 (disassembled, FalloutNV.exe 1.4.0.525)
/// The vector at `012683b0`: `(a, a, a, b)` with `a` the float at `01011d78` and `b` the one at
/// `01012068` (`MOVSS`, three `UNPCKLPS`; the lanes the registers held before are not used).
pub fn fn_00fbde20(e: &mut Engine) {
    let a = e.global::<u32>(0x0101_1d78);
    let b = e.global::<u32>(0x0101_2068);
    for (k, w) in [a, a, a, b].into_iter().enumerate() {
        e.set_global::<u32>(0x0126_83b0 + 4 * k as u32, w);
    }
}

// Translated from 00fbe6e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The 48 bytes at `011b2e60` copied to `01268700` (three `MOVAPS` pairs).
pub fn fn_00fbe6e0(e: &mut Engine) {
    copy16(e, 0x011b_2e60, 0x0126_8700);
    copy16(e, 0x011b_2e70, 0x0126_8710);
    copy16(e, 0x011b_2e80, 0x0126_8720);
}

// Translated from 00fbffc0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The byte at `011b7b89`: whether the word at `010d6ae4` is 1 (`CMP [..],1; SETZ AL`).
pub fn fn_00fbffc0(e: &mut Engine) {
    let one = e.global::<u32>(0x010d_6ae4) == 1;
    e.set_global::<u8>(0x011b_7b89, u8::from(one));
}

// Translated from 00fc02b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// `00ca9360` on `011b8898` with the address of a local float 1.0; then the float at
/// `0104da48` copied to `011b889c` (`FLD`/`FSTP`).
pub fn fn_00fc02b0(e: &mut Engine) {
    e.with_stack(4, |e, local| {
        e.mem.set_u32(local.addr(), 0x3f80_0000);
        e.call(FN_00CA9360, &args![0x011b_8898u32, local.addr()]);
    });
    let f = x87_quiet_bits(e.global::<u32>(0x0104_da48));
    e.set_global::<u32>(0x011b_889c, f);
}

// Translated from 00fc09a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The 64 bytes at `011b9240` copied to `0126a3e0` (four `MOVAPS` pairs).
pub fn fn_00fc09a0(e: &mut Engine) {
    copy16(e, 0x011b_9240, 0x0126_a3e0);
    copy16(e, 0x011b_9250, 0x0126_a3f0);
    copy16(e, 0x011b_9260, 0x0126_a400);
    copy16(e, 0x011b_9270, 0x0126_a410);
}

// Translated from 00fc1530 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011b9f78` = `[011b9e78]`; `011b9f90`, `011b9fa8`, `011b9fc0` = `[011b9e7c]`.
pub fn fn_00fc1530(e: &mut Engine) {
    let a = e.global::<u32>(0x011b_9e78);
    e.set_global::<u32>(0x011b_9f78, a);
    let b = e.global::<u32>(0x011b_9e7c);
    e.set_global::<u32>(0x011b_9f90, b);
    e.set_global::<u32>(0x011b_9fa8, b);
    e.set_global::<u32>(0x011b_9fc0, b);
}

// Translated from 00fc3b30 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011bc270`, `011bc288` = `[011bc1f8]`; `011bc2b8` = `[011bc1fc]`.
pub fn fn_00fc3b30(e: &mut Engine) {
    let a = e.global::<u32>(0x011b_c1f8);
    e.set_global::<u32>(0x011b_c270, a);
    e.set_global::<u32>(0x011b_c288, a);
    let b = e.global::<u32>(0x011b_c1fc);
    e.set_global::<u32>(0x011b_c2b8, b);
}

// Translated from 00fc7200 (disassembled, FalloutNV.exe 1.4.0.525)
/// `011be650`, `011be668` = `[011be5f0]`.
pub fn fn_00fc7200(e: &mut Engine) {
    let a = e.global::<u32>(0x011b_e5f0);
    e.set_global::<u32>(0x011b_e650, a);
    e.set_global::<u32>(0x011b_e668, a);
}

// Translated from 00fc72e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// 0x100 zero words from `0126f108` (`REP STOSD`); `_atexit(00fde2e0)`.
pub fn fn_00fc72e0(e: &mut Engine) {
    for k in 0..0x100u32 {
        e.set_global::<u32>(0x0126_f108 + 4 * k, 0);
    }
    e.call(ATEXIT, &args![0x00fd_e2e0u32]);
}

// Translated from 00fc7490 (disassembled, FalloutNV.exe 1.4.0.525)
/// A string copy: when the data word at `011f49e0` is not the empty string's (`0109b220`),
/// `InterlockedIncrement` on its count (8 bytes before the data); the word (loaded again) to
/// `0126f750`; `_atexit(00fde370)`.
pub fn fn_00fc7490(e: &mut Engine) {
    let data = e.global::<u32>(0x011f_49e0);
    if data != e.global::<u32>(EMPTY) {
        e.call(INTERLOCKED_INCREMENT, &args![data.wrapping_sub(8)]);
    }
    let data = e.global::<u32>(0x011f_49e0);
    e.set_global::<u32>(0x0126_f750, data);
    e.call(ATEXIT, &args![0x00fd_e370u32]);
}

// Translated from 00fc7680 (disassembled, FalloutNV.exe 1.4.0.525)
/// `00eab8a0` on `0126fa68` with 1.0 (`FLD1`) and `011f426c`; `_atexit(00fde3c0)`.
pub fn fn_00fc7680(e: &mut Engine) {
    e.call(
        FN_00EAB8A0,
        &args![0x0126_fa68u32, 0x3f80_0000u32, 0x011f_426cu32],
    );
    e.call(ATEXIT, &args![0x00fd_e3c0u32]);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00ed_417e, fn_00ed417e() -> u32),
        entry!(0x00ed_d4e7, fn_00edd4e7() -> u32),
        entry!(0x00fb_49d0, fn_00fb49d0()),
        entry!(0x00fb_49f0, fn_00fb49f0()),
        entry!(0x00fb_4b60, fn_00fb4b60()),
        entry!(0x00fb_5530, fn_00fb5530()),
        entry!(0x00fb_5820, fn_00fb5820()),
        entry!(0x00fb_58b0, fn_00fb58b0()),
        entry!(0x00fb_59a0, fn_00fb59a0()),
        entry!(0x00fb_6270, fn_00fb6270()),
        entry!(0x00fb_a2e0, fn_00fba2e0()),
        entry!(0x00fb_ab30, fn_00fbab30()),
        entry!(0x00fb_cb40, fn_00fbcb40()),
        entry!(0x00fb_cb70, fn_00fbcb70()),
        entry!(0x00fb_ce90, fn_00fbce90()),
        entry!(0x00fb_d4e0, fn_00fbd4e0()),
        entry!(0x00fb_d520, fn_00fbd520()),
        entry!(0x00fb_d560, fn_00fbd560()),
        entry!(0x00fb_de20, fn_00fbde20()),
        entry!(0x00fb_dff0, fn_00fbdff0()),
        entry!(0x00fb_e6e0, fn_00fbe6e0()),
        entry!(0x00fb_ffc0, fn_00fbffc0()),
        entry!(0x00fc_02b0, fn_00fc02b0()),
        entry!(0x00fc_09a0, fn_00fc09a0()),
        entry!(0x00fc_1530, fn_00fc1530()),
        entry!(0x00fc_3b30, fn_00fc3b30()),
        entry!(0x00fc_7200, fn_00fc7200()),
        entry!(0x00fc_72e0, fn_00fc72e0()),
        entry!(0x00fc_7490, fn_00fc7490()),
        entry!(0x00fc_7680, fn_00fc7680()),
        entry!(0x00fc_78de, fn_00fc78de()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    /// An engine with `pages` mapped and every callee of this unit doubled (answering
    /// `0x0bad_0000 + low bits of the address`).
    fn engine(pages: &[u32]) -> Engine {
        let mut e = Engine::new();
        for &p in pages {
            e.map(p, 0x1000);
        }
        for f in [
            ATEXIT,
            FLOAT_SETTING_CONSTRUCT,
            SETMBCP,
            SET_UNHANDLED_EXCEPTION_FILTER,
            CREATE_SEMAPHORE_A,
            INTERLOCKED_INCREMENT,
            IOB_FUNC,
            FN_00EC3A4F,
            FN_00A6DB50,
            FN_00EAB8A0,
            FN_00B00E40,
            FN_00CA9360,
            FN_00AA1070,
            MEMSET,
        ] {
            e.register_double(f, move |_, _| Ret {
                eax: 0x0bad_0000 | (f & 0xffff),
                ..Ret::default()
            });
        }
        e
    }

    fn run(e: &mut Engine, entry: u32) -> (Calls, u32) {
        e.call_log = Some(vec![]);
        let eax = e.call(entry, &args![]).u32();
        let calls = e
            .call_log
            .take()
            .unwrap()
            .into_iter()
            .filter(|(a, _)| *a != entry)
            .collect();
        (calls, eax)
    }

    #[test]
    fn the_c_initializers_answer_zero() {
        let mut e = engine(&[0x0127_1000]);
        let (calls, eax) = run(&mut e, 0x00ed_417e);
        assert_eq!((calls, eax), (vec![(SETMBCP, vec![0xffff_fffd])], 0));
        assert_eq!(e.mem.u32(0x0127_1a90), 1);
        // Once set, nothing is called.
        assert_eq!(run(&mut e, 0x00ed_417e), (vec![], 0));
        let (calls, eax) = run(&mut e, 0x00ed_d4e7);
        assert_eq!(
            (calls, eax),
            (vec![(SET_UNHANDLED_EXCEPTION_FILTER, vec![0x00ed_d4a5])], 0)
        );
    }

    #[test]
    fn calls_get_their_words_in_order() {
        let mut e = engine(&[
            0x0109_6000,
            0x0101_5000,
            0x011f_4000,
            0x0104_d000,
            0x011b_8000,
        ]);
        e.mem.set_u32(0x0109_6b8c, 0x7f80_0001);
        assert_eq!(
            run(&mut e, 0x00fb_4b60).0,
            vec![(
                FN_00A6DB50,
                vec![
                    0x011f_3704,
                    0x7fc0_0001,
                    0x7fc0_0001,
                    0x7fc0_0001,
                    0x7fc0_0001
                ]
            )]
        );
        assert_eq!(
            run(&mut e, 0x00fc_78de).0,
            vec![
                (IOB_FUNC, vec![]),
                (FN_00EC3A4F, vec![0x0126_fbc0, 0x0bad_df44 + 0x40]),
                (ATEXIT, vec![0x00fd_e56d]),
            ]
        );
        assert_eq!(
            run(&mut e, 0x00fc_7680).0,
            vec![
                (FN_00EAB8A0, vec![0x0126_fa68, 0x3f80_0000, 0x011f_426c]),
                (ATEXIT, vec![0x00fd_e3c0]),
            ]
        );
        assert_eq!(
            run(&mut e, 0x00fb_ce90).0,
            vec![
                (FLOAT_SETTING_CONSTRUCT, vec![0x0126_7c84, 0x010c_4ec4, 0]),
                (ATEXIT, vec![0x00fd_df10]),
            ]
        );
        let (calls, _) = run(&mut e, 0x00fb_58b0);
        assert_eq!(
            calls,
            vec![
                (CREATE_SEMAPHORE_A, vec![0, 0, 1, 0]),
                (ATEXIT, vec![0x00fd_af60]),
            ]
        );
        assert_eq!(
            [
                e.mem.u32(0x011f_4790),
                e.mem.u32(0x011f_4794),
                e.mem.u32(0x011f_4798)
            ],
            [0, 1, 0x0bad_f0b4]
        );
        e.mem.set_u32(0x0104_da48, 0x4000_0000);
        let (calls, _) = run(&mut e, 0x00fc_02b0);
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, FN_00CA9360);
        assert_eq!(calls[0].1[0], 0x011b_8898);
        assert_eq!(e.mem.u32(0x011b_889c), 0x4000_0000);
    }

    #[test]
    fn tables_and_objects() {
        let mut e = engine(&[0x011b_0000, 0x0126_7000]);
        e.mem.set_u32(0x011b_02f4, 5);
        assert_eq!(
            run(&mut e, 0x00fb_d4e0).0,
            vec![
                (FN_00AA1070, vec![0x94]),
                (MEMSET, vec![0x0bad_1070, 0, 20]),
                (ATEXIT, vec![0x00fd_de70]),
            ]
        );
        assert_eq!(e.mem.u32(0x011b_02f8), 0x0bad_1070);
        assert_eq!(e.mem.u32(0x011b_02f0), 0x010c_4c84);
        assert_eq!(
            run(&mut e, 0x00fb_cb70).0,
            vec![
                (FN_00B00E40, vec![0x0126_7b0c, 1]),
                (ATEXIT, vec![0x00fd_dd30]),
            ]
        );
        assert_eq!(e.mem.u32(0x0126_7b0c), 0x010c_3b74);
    }

    #[test]
    fn the_string_copy_counts_a_reference_unless_empty() {
        let mut e = engine(&[0x0109_b000, 0x011f_4000, 0x0126_f000]);
        e.mem.set_u32(0x0109_b220, 0x0200_0010);
        e.mem.set_u32(0x011f_49e0, 0x0200_0010);
        assert_eq!(
            run(&mut e, 0x00fc_7490).0,
            vec![(ATEXIT, vec![0x00fd_e370])]
        );
        e.mem.set_u32(0x011f_49e0, 0x0300_0010);
        assert_eq!(
            run(&mut e, 0x00fc_7490).0,
            vec![
                (INTERLOCKED_INCREMENT, vec![0x0300_0008]),
                (ATEXIT, vec![0x00fd_e370])
            ]
        );
        assert_eq!(e.mem.u32(0x0126_f750), 0x0300_0010);
    }

    #[test]
    fn floats_words_and_vectors() {
        let mut e = engine(&[
            0x011a_7000,
            0x011f_3000,
            0x0101_1000,
            0x0101_2000,
            0x0126_8000,
            0x010d_6000,
            0x011b_7000,
            0x010b_8000,
            0x011a_d000,
            0x011a_e000,
            0x0126_f000,
            0x011f_4000,
        ]);
        e.mem.set_i32(0x011a_7ed4, 4);
        run(&mut e, 0x00fb_49d0);
        assert_eq!(e.mem.u32(0x011f_3658), 0.25f32.to_bits());
        e.mem.set_u32(0x0101_1d78, 0x3f80_0000);
        e.mem.set_u32(0x0101_2068, 0x4000_0000);
        run(&mut e, 0x00fb_de20);
        let v: Vec<u32> = (0..4).map(|k| e.mem.u32(0x0126_83b0 + 4 * k)).collect();
        assert_eq!(v, [0x3f80_0000, 0x3f80_0000, 0x3f80_0000, 0x4000_0000]);
        e.mem.set_u32(0x010d_6ae4, 1);
        run(&mut e, 0x00fb_ffc0);
        assert_eq!(e.mem.u8(0x011b_7b89), 1);
        e.mem.set_u32(0x010d_6ae4, 2);
        run(&mut e, 0x00fb_ffc0);
        assert_eq!(e.mem.u8(0x011b_7b89), 0);
        // 0 / -2.0 is -0.0; 0 / 3.0 is +0.0.
        e.mem.set_f64(0x010b_8cd8, -2.0);
        e.mem.set_f64(0x010b_8cb0, 3.0);
        e.mem.set_u32(0x011a_df20, 7);
        e.mem.set_u32(0x011a_e4b0, 7);
        run(&mut e, 0x00fb_a2e0);
        assert_eq!(e.mem.u32(0x011a_df20), 0x8000_0000);
        assert_eq!(e.mem.u32(0x011a_e000), 0x8000_0000);
        assert_eq!(e.mem.u32(0x011a_e4b0), 0);
        e.mem.set_u32(0x0126_f108 + 0x3fc, 9);
        assert_eq!(
            run(&mut e, 0x00fc_72e0).0,
            vec![(ATEXIT, vec![0x00fd_e2e0])]
        );
        assert_eq!(e.mem.u32(0x0126_f108 + 0x3fc), 0);
        e.mem.set_u32(0x011f_426c, 1);
        e.mem.set_u32(0x011f_4270, 2);
        e.mem.set_u32(0x011f_4274, 3);
        run(&mut e, 0x00fb_49f0);
        assert_eq!(
            [
                e.mem.u32(0x011f_365c),
                e.mem.u32(0x011f_3660),
                e.mem.u32(0x011f_3664)
            ],
            [1, 2, 3]
        );
    }

    #[test]
    fn every_function_is_registered() {
        let e = Engine::new();
        let list = funcs();
        assert_eq!(list.len(), 31);
        for (a, _) in list {
            assert!(e.is_translated(a), "{a:08x}");
        }
    }
}
