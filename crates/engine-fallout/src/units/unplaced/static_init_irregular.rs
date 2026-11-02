//! `(unplaced)/static_init_irregular` (no source unit: the compiler's dynamic initializers),
//! subsystem `compiler: static init/exit`: 252 of the "dynamic initializer" functions of
//! FalloutNV.exe 1.4.0.525 (`00f35420` .. `00f9adc0`, the engine map's "compiler: dynamic
//! initializers") whose bodies match none of the shapes of `data/static_initializers.tsv`
//! (docs/ENGINE_CRATE.md, `research/initgen/README.md`), each translated from its own
//! disassembly. The CRT runs each once at start-up, before `main`. They are cdecl functions with
//! no parameters; what they leave in `EAX` nobody reads, so they return nothing. The shapes:
//!
//! - 209 functions `MOV ECX,global; CALL 00a2e180`: the callee answers `ECX`, which `_initterm`
//!   leaves from the previous initializer, so the table refused them. `00a2e180` is called by
//!   address with the global as its `this`.
//! - 22 functions `PUSH second; PUSH first; CALL 00403a70; ADD ESP,8; MOV [global],AL`: a cdecl
//!   call with two words whose low byte is stored in a byte global.
//! - 6 functions that run the compiler's `eh vector constructor iterator` (`00ec782f`, stdcall,
//!   five words) on a global array, then give an `atexit` function to `_atexit` (`00ec658f`).
//! - 6 functions that call `00403e20` (it answers a pointer to a float), then run `0045d060` on a
//!   global object with a constant word and that float (a quiet copy, `FLD`/`FSTP`), then give an
//!   `atexit` function to `_atexit`.
//! - 9 single cases (`00f38300`, `00f48900`, `00f4b250`, `00f587a0`, `00f5cb40`, `00f79710`,
//!   `00f8fa80`, `00f90990`, `00f92d50`).
//!
//! `00f79710` (2,686 bytes) sits in an `FS:[0]` exception frame (handler `00efd6fe`, states 0 to
//! 0x1b): it is marked `// Translation incomplete` because the frame (the compiler's unwinding on
//! an exception) is not translated; every call in its body is. No other function here has a frame
//! or a `/GS` cookie check.
//!
//! Callees are called by address and never re-implemented.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_atexit` (`00ec658f`, cdecl, one word: the function to run at exit).
const ATEXIT: u32 = 0x00ec_658f;
/// `00a2e180` (`thiscall`, no stack argument; answers `ECX`): bumps the word at `011f36a0`.
const FN_00A2E180: u32 = 0x00a2_e180;
/// `00403a70` (cdecl, two words, answers `AL`).
const FN_00403A70: u32 = 0x0040_3a70;
/// The compiler's `eh vector constructor iterator` (`00ec782f`, stdcall `RET 0x14`):
/// `(array, element size, count, constructor, destructor)`, pushed in reverse. It calls the
/// thiscall constructor on each element.
const EH_VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// `00403e20` (`thiscall`, no stack argument): answers `ECX + 4` (a pointer to the float inside
/// the object), or `01202800` (a zeroed float it stores first) when `ECX` is null.
const FLOAT_GETTER_00403E20: u32 = 0x0040_3e20;
/// `0045d060` (`thiscall`, `RET 8`): `(this, word, float)`.
const FN_0045D060: u32 = 0x0045_d060;
/// `0043d4d0` (`thiscall`, no stack argument): answers `ECX + 4` (a pointer to the word inside
/// the object), or `01202800` (a zeroed word it stores first) when `ECX` is null.
const WORD_GETTER_0043D4D0: u32 = 0x0043_d4d0;
/// `0045cfe0` (`thiscall`, `RET 8`): `(this, word, word)`.
const FN_0045CFE0: u32 = 0x0045_cfe0;
/// `006444e0` (`thiscall`, `RET 0x10`): `(this, word, word, word, word)`.
const FN_006444E0: u32 = 0x0064_44e0;
/// `00718e90` (cdecl, four words, answers `EAX`).
const FN_00718E90: u32 = 0x0071_8e90;
/// `00430bc0` (cdecl, one word, answers `EAX`).
const FN_00430BC0: u32 = 0x0043_0bc0;
/// `0050e6c0` (cdecl, no words, answers `AL`).
const FN_0050E6C0: u32 = 0x0050_e6c0;
/// `00401000` (cdecl, one word: a size; answers `EAX`).
const FN_00401000: u32 = 0x0040_1000;
/// `0040c150` (`thiscall`, `RET 8`): `(this, word, word)`, answers `EAX`.
const FN_0040C150: u32 = 0x0040_c150;

/// `MOV ECX,global; CALL 00a2e180`.
fn run_00a2e180(e: &mut Engine, global: u32) {
    e.call(FN_00A2E180, &args![global]);
}

/// `PUSH second; PUSH first; CALL 00403a70; ADD ESP,8; MOV [global],AL`: the call's low byte is
/// stored in the byte global.
fn store_byte_00403a70(e: &mut Engine, global: u32, first: u32, second: u32) {
    let result = e.call(FN_00403A70, &args![first, second]).u8();
    e.set_global::<u8>(global, result);
}

/// `PUSH exit_fn; CALL _atexit; ADD ESP,4` (the result is not used).
fn register_at_exit(e: &mut Engine, exit_fn: u32) {
    e.call(ATEXIT, &args![exit_fn]);
}

/// `PUSH destructor; PUSH constructor; PUSH count; PUSH size; PUSH array; CALL 00ec782f`, then
/// `_atexit` of `exit_fn`.
fn vector_construct_at_exit(
    e: &mut Engine,
    array: u32,
    size: u32,
    count: u32,
    constructor: u32,
    destructor: u32,
    exit_fn: u32,
) {
    e.call(
        EH_VECTOR_CONSTRUCTOR_ITERATOR,
        &args![array, size, count, constructor, destructor],
    );
    register_at_exit(e, exit_fn);
}

/// `MOV ECX,getter_global; CALL 00403e20; PUSH ECX; FLD float ptr [EAX]; FSTP float ptr [ESP];
/// PUSH word; MOV ECX,object; CALL 0045d060`, then `_atexit` of `exit_fn`. The float is copied
/// through the x87 stack (a signalling NaN comes out quiet) and passed as its bit pattern, after
/// the constant word.
fn float_object_at_exit(e: &mut Engine, getter_global: u32, object: u32, word: u32, exit_fn: u32) {
    let pointer = e.call(FLOAT_GETTER_00403E20, &args![getter_global]).u32();
    let float = x87_quiet_bits(e.global::<u32>(pointer));
    e.call(FN_0045D060, &args![object, word, float]);
    register_at_exit(e, exit_fn);
}

/// `PUSH args; CALL callee; ADD ESP,..; MOV [global],EAX` for a cdecl callee.
fn store_result(e: &mut Engine, global: u32, callee: u32, words: &[u32]) {
    let result = e.call(callee, words).u32();
    e.set_global::<u32>(global, result);
}

// Translated from 00f35420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c32f0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f35420(e: &mut Engine) {
    run_00a2e180(e, 0x011c_32f0);
}

// Translated from 00f359e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c33b0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f359e0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_33b0);
}

// Translated from 00f35fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c34ec`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f35fd0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_34ec);
}

// Translated from 00f364f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c37cc`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f364f0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_37cc);
}

// Translated from 00f36fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c3880`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f36fc0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_3880);
}

// Translated from 00f373c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c3970`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f373c0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_3970);
}

// Translated from 00f37690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c3aa8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f37690(e: &mut Engine) {
    run_00a2e180(e, 0x011c_3aa8);
}

// Translated from 00f37ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c3bd8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f37ec0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_3bd8);
}

// Translated from 00f38220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c3e34`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f38220(e: &mut Engine) {
    run_00a2e180(e, 0x011c_3e34);
}

// Translated from 00f38300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the object at `011c3c90`: `0043d4d0` on `011c63cc` answers a pointer to
/// a word, four times (the first two pointers are read after the second call, the third and
/// fourth after their own calls); the object's constructor `0045cfe0` takes `01017ffc` and
/// `a * b + c + d + 1` (32-bit, `IMUL`/`ADD`/`LEA`, `a`..`d` the four words), then `_atexit` of
/// `00fc8090`.
pub fn fn_00f38300(e: &mut Engine) {
    const GLOBAL: u32 = 0x011c_63cc;
    let first = e.call(WORD_GETTER_0043D4D0, &args![GLOBAL]).u32();
    let second = e.call(WORD_GETTER_0043D4D0, &args![GLOBAL]).u32();
    let mut sum = e.global::<u32>(first).wrapping_mul(e.global::<u32>(second));
    let third = e.call(WORD_GETTER_0043D4D0, &args![GLOBAL]).u32();
    sum = sum.wrapping_add(e.global::<u32>(third));
    let fourth = e.call(WORD_GETTER_0043D4D0, &args![GLOBAL]).u32();
    let last = e.global::<u32>(fourth);
    let word = sum.wrapping_add(last).wrapping_add(1);
    e.call(FN_0045CFE0, &args![0x011c_3c90_u32, 0x0101_7ffc_u32, word]);
    register_at_exit(e, 0x00fc_8090);
}

// Translated from 00f38e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c403c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f38e00(e: &mut Engine) {
    run_00a2e180(e, 0x011c_403c);
}

// Translated from 00f39530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c4168`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f39530(e: &mut Engine) {
    run_00a2e180(e, 0x011c_4168);
}

// Translated from 00f398b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c4c04`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f398b0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_4c04);
}

// Translated from 00f39c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c4ca4`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f39c10(e: &mut Engine) {
    run_00a2e180(e, 0x011c_4ca4);
}

// Translated from 00f39ee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c4d24`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f39ee0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_4d24);
}

// Translated from 00f3a280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c5194`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3a280(e: &mut Engine) {
    run_00a2e180(e, 0x011c_5194);
}

// Translated from 00f3b7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c52c1`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3b7a0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_52c1);
}

// Translated from 00f3bc40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c5400`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3bc40(e: &mut Engine) {
    run_00a2e180(e, 0x011c_5400);
}

// Translated from 00f3bea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c5478`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3bea0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_5478);
}

// Translated from 00f3cba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c567c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3cba0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_567c);
}

// Translated from 00f3d020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c5790`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3d020(e: &mut Engine) {
    run_00a2e180(e, 0x011c_5790);
}

// Translated from 00f3d560 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c5910`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3d560(e: &mut Engine) {
    run_00a2e180(e, 0x011c_5910);
}

// Translated from 00f3db50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c5c1c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3db50(e: &mut Engine) {
    run_00a2e180(e, 0x011c_5c1c);
}

// Translated from 00f3e8e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c5cb5`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3e8e0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_5cb5);
}

// Translated from 00f3e9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the array at `011c5d10`: `eh vector constructor iterator` (`00ec782f`) over 20 elements of 24 bytes with constructor `00488f50` and destructor `00489070`, then `_atexit` of `00fc9090`.
pub fn fn_00f3e9b0(e: &mut Engine) {
    vector_construct_at_exit(
        e,
        0x011c_5d10,
        0x18,
        0x14,
        0x0048_8f50,
        0x0048_9070,
        0x00fc_9090,
    );
}

// Translated from 00f3edc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c6308`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3edc0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_6308);
}

// Translated from 00f3f180 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c6414`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3f180(e: &mut Engine) {
    run_00a2e180(e, 0x011c_6414);
}

// Translated from 00f3f410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c64c0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3f410(e: &mut Engine) {
    run_00a2e180(e, 0x011c_64c0);
}

// Translated from 00f3f8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c6f78`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3f8d0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_6f78);
}

// Translated from 00f3fc30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c70f9`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f3fc30(e: &mut Engine) {
    run_00a2e180(e, 0x011c_70f9);
}

// Translated from 00f40b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the object at `011c7740`: `00403e20` on `011c744c` answers a pointer to a float; the object's constructor `0045d060` takes `01022218` and that float (a quiet copy), then `_atexit` of `00fc9680`.
pub fn fn_00f40b10(e: &mut Engine) {
    float_object_at_exit(e, 0x011c_744c, 0x011c_7740, 0x0102_2218, 0x00fc_9680);
}

// Translated from 00f40c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the object at `011c7568`: `00403e20` on `011c7278` answers a pointer to a float; the object's constructor `0045d060` takes `010222b8` and that float (a quiet copy), then `_atexit` of `00fc96d0`.
pub fn fn_00f40c10(e: &mut Engine) {
    float_object_at_exit(e, 0x011c_7278, 0x011c_7568, 0x0102_22b8, 0x00fc_96d0);
}

// Translated from 00f40d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the object at `011c7598`: `00403e20` on `011c7494` answers a pointer to a float; the object's constructor `0045d060` takes `01022354` and that float (a quiet copy), then `_atexit` of `00fc9720`.
pub fn fn_00f40d10(e: &mut Engine) {
    float_object_at_exit(e, 0x011c_7494, 0x011c_7598, 0x0102_2354, 0x00fc_9720);
}

// Translated from 00f416c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the array at `011c7804`: `eh vector constructor iterator` (`00ec782f`) over 3 elements of 4 bytes with constructor `006694e0` and destructor `0045cec0`, then `_atexit` of `00fc9a10`.
pub fn fn_00f416c0(e: &mut Engine) {
    vector_construct_at_exit(
        e,
        0x011c_7804,
        0x4,
        0x3,
        0x0066_94e0,
        0x0045_cec0,
        0x00fc_9a10,
    );
}

// Translated from 00f41820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the array at `011c7860`: `eh vector constructor iterator` (`00ec782f`) over 3 elements of 4 bytes with constructor `006694e0` and destructor `0045cec0`, then `_atexit` of `00fc9a60`.
pub fn fn_00f41820(e: &mut Engine) {
    vector_construct_at_exit(
        e,
        0x011c_7860,
        0x4,
        0x3,
        0x0066_94e0,
        0x0045_cec0,
        0x00fc_9a60,
    );
}

// Translated from 00f42230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c7b79`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f42230(e: &mut Engine) {
    run_00a2e180(e, 0x011c_7b79);
}

// Translated from 00f42e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c7da8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f42e10(e: &mut Engine) {
    run_00a2e180(e, 0x011c_7da8);
}

// Translated from 00f43190 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c7e44`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f43190(e: &mut Engine) {
    run_00a2e180(e, 0x011c_7e44);
}

// Translated from 00f43470 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c7ed0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f43470(e: &mut Engine) {
    run_00a2e180(e, 0x011c_7ed0);
}

// Translated from 00f44170 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c80f4`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f44170(e: &mut Engine) {
    run_00a2e180(e, 0x011c_80f4);
}

// Translated from 00f44530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c819c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f44530(e: &mut Engine) {
    run_00a2e180(e, 0x011c_819c);
}

// Translated from 00f44790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c8214`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f44790(e: &mut Engine) {
    run_00a2e180(e, 0x011c_8214);
}

// Translated from 00f44a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c82a8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f44a30(e: &mut Engine) {
    run_00a2e180(e, 0x011c_82a8);
}

// Translated from 00f44d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c838c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f44d10(e: &mut Engine) {
    run_00a2e180(e, 0x011c_838c);
}

// Translated from 00f451e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c8438`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f451e0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_8438);
}

// Translated from 00f45770 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c851c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f45770(e: &mut Engine) {
    run_00a2e180(e, 0x011c_851c);
}

// Translated from 00f459d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c85ac`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f459d0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_85ac);
}

// Translated from 00f45c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c8ccc`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f45c90(e: &mut Engine) {
    run_00a2e180(e, 0x011c_8ccc);
}

// Translated from 00f47e00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c8e4c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f47e00(e: &mut Engine) {
    run_00a2e180(e, 0x011c_8e4c);
}

// Translated from 00f48090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c8ec8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f48090(e: &mut Engine) {
    run_00a2e180(e, 0x011c_8ec8);
}

// Translated from 00f48450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c8f80`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f48450(e: &mut Engine) {
    run_00a2e180(e, 0x011c_8f80);
}

// Translated from 00f486b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c8fa2`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f486b0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_8fa2);
}

// Translated from 00f48900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011c8fa1`: the low byte of `0050e6c0` (cdecl, no words).
pub fn fn_00f48900(e: &mut Engine) {
    let result = e.call(FN_0050E6C0, &args![]).u8();
    e.set_global::<u8>(0x011c_8fa1, result);
}

// Translated from 00f48b60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c910c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f48b60(e: &mut Engine) {
    run_00a2e180(e, 0x011c_910c);
}

// Translated from 00f48dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c9190`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f48dc0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_9190);
}

// Translated from 00f492f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c92a8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f492f0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_92a8);
}

// Translated from 00f495c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c9330`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f495c0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_9330);
}

// Translated from 00f49820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c9355`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f49820(e: &mut Engine) {
    run_00a2e180(e, 0x011c_9355);
}

// Translated from 00f49f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c94f0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f49f10(e: &mut Engine) {
    run_00a2e180(e, 0x011c_94f0);
}

// Translated from 00f4a200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c958c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4a200(e: &mut Engine) {
    run_00a2e180(e, 0x011c_958c);
}

// Translated from 00f4a7f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c9684`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4a7f0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_9684);
}

// Translated from 00f4ad00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011c9e9c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4ad00(e: &mut Engine) {
    run_00a2e180(e, 0x011c_9e9c);
}

// Translated from 00f4af80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011ca010`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4af80(e: &mut Engine) {
    run_00a2e180(e, 0x011c_a010);
}

// Translated from 00f4b250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the word at `011c9f6c`: the word `0043d4d0` on `011c9fb8` points to.
pub fn fn_00f4b250(e: &mut Engine) {
    let pointer = e.call(WORD_GETTER_0043D4D0, &args![0x011c_9fb8_u32]).u32();
    let word = e.global::<u32>(pointer);
    e.set_global::<u32>(0x011c_9f6c, word);
}

// Translated from 00f4b6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011ca091`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4b6c0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_a091);
}

// Translated from 00f4bb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011ca3e0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4bb40(e: &mut Engine) {
    run_00a2e180(e, 0x011c_a3e0);
}

// Translated from 00f4c1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011ca504`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4c1b0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_a504);
}

// Translated from 00f4c2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the object at `011ca450`: `00403e20` on `011ca4cc` answers a pointer to a float; the object's constructor `0045d060` takes `01031218` and that float (a quiet copy), then `_atexit` of `00fcb040`.
pub fn fn_00f4c2e0(e: &mut Engine) {
    float_object_at_exit(e, 0x011c_a4cc, 0x011c_a450, 0x0103_1218, 0x00fc_b040);
}

// Translated from 00f4c600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011ca5a8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4c600(e: &mut Engine) {
    run_00a2e180(e, 0x011c_a5a8);
}

// Translated from 00f4cb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011ca6c4`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4cb90(e: &mut Engine) {
    run_00a2e180(e, 0x011c_a6c4);
}

// Translated from 00f4d030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011ca7f4`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4d030(e: &mut Engine) {
    run_00a2e180(e, 0x011c_a7f4);
}

// Translated from 00f4d990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011ca958`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4d990(e: &mut Engine) {
    run_00a2e180(e, 0x011c_a958);
}

// Translated from 00f4dbf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011ca9d0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4dbf0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_a9d0);
}

// Translated from 00f4de50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011caa5c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4de50(e: &mut Engine) {
    run_00a2e180(e, 0x011c_aa5c);
}

// Translated from 00f4e310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cab25`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4e310(e: &mut Engine) {
    run_00a2e180(e, 0x011c_ab25);
}

// Translated from 00f4e790 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cac48`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4e790(e: &mut Engine) {
    run_00a2e180(e, 0x011c_ac48);
}

// Translated from 00f4e9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cacf0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4e9f0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_acf0);
}

// Translated from 00f4ed20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cada4`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4ed20(e: &mut Engine) {
    run_00a2e180(e, 0x011c_ada4);
}

// Translated from 00f4ef80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cae2c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4ef80(e: &mut Engine) {
    run_00a2e180(e, 0x011c_ae2c);
}

// Translated from 00f4f240 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011caeb0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4f240(e: &mut Engine) {
    run_00a2e180(e, 0x011c_aeb0);
}

// Translated from 00f4f5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011caf37`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4f5a0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_af37);
}

// Translated from 00f4fa50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb06c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f4fa50(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b06c);
}

// Translated from 00f50830 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb0e4`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f50830(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b0e4);
}

// Translated from 00f50b70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb178`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f50b70(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b178);
}

// Translated from 00f50dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb1f0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f50dd0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b1f0);
}

// Translated from 00f51030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb268`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f51030(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b268);
}

// Translated from 00f51450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb318`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f51450(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b318);
}

// Translated from 00f516b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb390`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f516b0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b390);
}

// Translated from 00f51950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb514`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f51950(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b514);
}

// Translated from 00f519f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the array at `011cb3e0`: `eh vector constructor iterator` (`00ec782f`) over 14 elements of 8 bytes with constructor `0096a2d0` and destructor `0046ffb0`, then `_atexit` of `00fcb260`.
pub fn fn_00f519f0(e: &mut Engine) {
    vector_construct_at_exit(
        e,
        0x011c_b3e0,
        0x8,
        0xe,
        0x0096_a2d0,
        0x0046_ffb0,
        0x00fc_b260,
    );
}

// Translated from 00f51a20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the array at `011cb450`: `eh vector constructor iterator` (`00ec782f`) over 14 elements of 8 bytes with constructor `0096a2d0` and destructor `0046ffb0`, then `_atexit` of `00fcb280`.
pub fn fn_00f51a20(e: &mut Engine) {
    vector_construct_at_exit(
        e,
        0x011c_b450,
        0x8,
        0xe,
        0x0096_a2d0,
        0x0046_ffb0,
        0x00fc_b280,
    );
}

// Translated from 00f51da0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb5e0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f51da0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b5e0);
}

// Translated from 00f521e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb684`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f521e0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b684);
}

// Translated from 00f52440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb6a9`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f52440(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b6a9);
}

// Translated from 00f526a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb79c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f526a0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b79c);
}

// Translated from 00f529e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb850`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f529e0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b850);
}

// Translated from 00f52c70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb8d4`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f52c70(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b8d4);
}

// Translated from 00f52ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb950`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f52ef0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b950);
}

// Translated from 00f53160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cb9e5`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f53160(e: &mut Engine) {
    run_00a2e180(e, 0x011c_b9e5);
}

// Translated from 00f53420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cbd14`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f53420(e: &mut Engine) {
    run_00a2e180(e, 0x011c_bd14);
}

// Translated from 00f54340 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cbe40`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f54340(e: &mut Engine) {
    run_00a2e180(e, 0x011c_be40);
}

// Translated from 00f54850 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cbf20`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f54850(e: &mut Engine) {
    run_00a2e180(e, 0x011c_bf20);
}

// Translated from 00f54af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cc088`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f54af0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_c088);
}

// Translated from 00f54bf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the array at `011cbf60`: `eh vector constructor iterator` (`00ec782f`) over 2 elements of 84 bytes with constructor `005aa0f0` and destructor `005aa1a0`, then `_atexit` of `00fcb8c0`.
pub fn fn_00f54bf0(e: &mut Engine) {
    vector_construct_at_exit(
        e,
        0x011c_bf60,
        0x54,
        0x2,
        0x005a_a0f0,
        0x005a_a1a0,
        0x00fc_b8c0,
    );
}

// Translated from 00f54e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cc108`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f54e90(e: &mut Engine) {
    run_00a2e180(e, 0x011c_c108);
}

// Translated from 00f551f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cc1ac`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f551f0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_c1ac);
}

// Translated from 00f556b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cc290`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f556b0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_c290);
}

// Translated from 00f55d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cc3a0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f55d30(e: &mut Engine) {
    run_00a2e180(e, 0x011c_c3a0);
}

// Translated from 00f56090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cc440`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f56090(e: &mut Engine) {
    run_00a2e180(e, 0x011c_c440);
}

// Translated from 00f56310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cc4b8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f56310(e: &mut Engine) {
    run_00a2e180(e, 0x011c_c4b8);
}

// Translated from 00f56570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cc530`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f56570(e: &mut Engine) {
    run_00a2e180(e, 0x011c_c530);
}

// Translated from 00f567d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cc5c8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f567d0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_c5c8);
}

// Translated from 00f56a80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cc67c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f56a80(e: &mut Engine) {
    run_00a2e180(e, 0x011c_c67c);
}

// Translated from 00f579c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cc9e0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f579c0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_c9e0);
}

// Translated from 00f57c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cca58`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f57c20(e: &mut Engine) {
    run_00a2e180(e, 0x011c_ca58);
}

// Translated from 00f57eb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011ccae0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f57eb0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_cae0);
}

// Translated from 00f58110 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011ccb58`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f58110(e: &mut Engine) {
    run_00a2e180(e, 0x011c_cb58);
}

// Translated from 00f58370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cccd8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f58370(e: &mut Engine) {
    run_00a2e180(e, 0x011c_ccd8);
}

// Translated from 00f587a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the float at `011ccc60`: `1.0` divided by the float `00403e20` on
/// `011ccc40` points to (`FLD float ptr [EAX]; FLD1; FDIVRP`: `ST1 = ST0 / ST1`, bytes `DE F1`),
/// stored as a float.
pub fn fn_00f587a0(e: &mut Engine) {
    let pointer = e.call(FLOAT_GETTER_00403E20, &args![0x011c_cc40_u32]).u32();
    let value = x87_load32(e.global::<u32>(pointer));
    let result = x87_div(1.0, value);
    e.set_global::<u32>(0x011c_cc60, x87_store32(result));
}

// Translated from 00f58a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011ccd68`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f58a10(e: &mut Engine) {
    run_00a2e180(e, 0x011c_cd68);
}

// Translated from 00f58d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cce18`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f58d10(e: &mut Engine) {
    run_00a2e180(e, 0x011c_ce18);
}

// Translated from 00f59030 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011ccf00`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f59030(e: &mut Engine) {
    run_00a2e180(e, 0x011c_cf00);
}

// Translated from 00f59410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cddb4`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f59410(e: &mut Engine) {
    run_00a2e180(e, 0x011c_ddb4);
}

// Translated from 00f5cb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializers of four objects (`011cd680`, `011cd690`, `011cd6a0`, `011cd6b0`): each is
/// `006444e0` (`thiscall`, four words) on the object, the words being addresses of globals.
pub fn fn_00f5cb40(e: &mut Engine) {
    e.call(
        FN_006444E0,
        &args![
            0x011c_d680_u32,
            0x011c_d708_u32,
            0x011c_d58c_u32,
            0x011c_d36c_u32,
            0x011c_cf70_u32
        ],
    );
    e.call(
        FN_006444E0,
        &args![
            0x011c_d690_u32,
            0x011c_deb0_u32,
            0x011c_d99c_u32,
            0x011c_d9c4_u32,
            0x011c_db18_u32
        ],
    );
    e.call(
        FN_006444E0,
        &args![
            0x011c_d6a0_u32,
            0x011c_d9a8_u32,
            0x011c_de44_u32,
            0x011c_d9c4_u32,
            0x011c_db18_u32
        ],
    );
    e.call(
        FN_006444E0,
        &args![
            0x011c_d6b0_u32,
            0x011c_d0d8_u32,
            0x011c_d2a0_u32,
            0x011c_d9c4_u32,
            0x011c_db18_u32
        ],
    );
}

// Translated from 00f5d7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011cfaac`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f5d7d0(e: &mut Engine) {
    run_00a2e180(e, 0x011c_faac);
}

/// The 28 allocate-and-construct blocks of `00f79710`: (global, first word, second word).
const OBJECT_BLOCKS_00F79710: [(u32, u32, u32); 28] = [
    (0x011d_51d0, 0x0106_367c, 0x0104_8458),
    (0x011d_51d4, 0x0106_3670, 0x0105_9d60),
    (0x011d_51d8, 0x0106_3654, 0x0106_3664),
    (0x011d_51dc, 0x0106_3634, 0x0106_3644),
    (0x011d_51e0, 0x0106_3628, 0x0104_7514),
    (0x011d_51e4, 0x0106_3618, 0x0104_1730),
    (0x011d_51e8, 0x0106_3604, 0x0106_3610),
    (0x011d_51ec, 0x0106_35e4, 0x0106_35f4),
    (0x011d_51f0, 0x0106_35d4, 0x0104_4fd8),
    (0x011d_51f4, 0x0106_35c8, 0x0104_4fd4),
    (0x011d_51f8, 0x0106_35ac, 0x0106_35bc),
    (0x011d_51fc, 0x0106_3590, 0x0106_35a0),
    (0x011d_5200, 0x0106_3584, 0x0104_7484),
    (0x011d_5204, 0x0106_3568, 0x0106_3578),
    (0x011d_5208, 0x0106_3550, 0x0106_3560),
    (0x011d_520c, 0x0106_3540, 0x0104_0ad0),
    (0x011d_5210, 0x0106_352c, 0x0106_3538),
    (0x011d_5214, 0x0106_3510, 0x0106_3520),
    (0x011d_5218, 0x0106_34f4, 0x0106_3504),
    (0x011d_521c, 0x0106_34d8, 0x0106_34e8),
    (0x011d_5220, 0x0106_34bc, 0x0106_34cc),
    (0x011d_5224, 0x0106_34a0, 0x0106_34b0),
    (0x011d_5228, 0x0106_3484, 0x0106_3494),
    (0x011d_522c, 0x0106_3468, 0x0106_3478),
    (0x011d_5230, 0x0106_344c, 0x0106_345c),
    (0x011d_5234, 0x0106_343c, 0x0106_1670),
    (0x011d_5238, 0x0106_3420, 0x0106_3430),
    (0x011d_523c, 0x0106_340c, 0x0106_3418),
];

// Translated from 00f79710 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of 28 pointers (`011d51d0` .. `011d523c`, in steps of 4): for each,
/// `00401000(0xc)` allocates twelve bytes and, when it answers a pointer, `0040c150` runs on the
/// block with two addresses (the block's own pair, from the table [`OBJECT_BLOCKS_00F79710`]);
/// its answer, or 0 when the allocation answered null, is stored in the global.
// Translation incomplete: 00f79710 - FS:[0] exception frame (handler 00efd6fe, states 0 to 0x1b, -1 between the blocks) not translated.
pub fn fn_00f79710(e: &mut Engine) {
    for &(global, first, second) in &OBJECT_BLOCKS_00F79710 {
        let block = e.call(FN_00401000, &args![0xc_u32]).u32();
        let object = if block != 0 {
            e.call(FN_0040C150, &args![block, first, second]).u32()
        } else {
            0
        };
        e.set_global::<u32>(global, object);
    }
}

// Translated from 00f80d80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d5920`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f80d80(e: &mut Engine) {
    run_00a2e180(e, 0x011d_5920);
}

// Translated from 00f81920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d5b1c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f81920(e: &mut Engine) {
    run_00a2e180(e, 0x011d_5b1c);
}

// Translated from 00f82160 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d5c28`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f82160(e: &mut Engine) {
    run_00a2e180(e, 0x011d_5c28);
}

// Translated from 00f823e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d5d80`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f823e0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_5d80);
}

// Translated from 00f82e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d6080`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f82e30(e: &mut Engine) {
    run_00a2e180(e, 0x011d_6080);
}

// Translated from 00f83720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d6134`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f83720(e: &mut Engine) {
    run_00a2e180(e, 0x011d_6134);
}

// Translated from 00f839a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d6155`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f839a0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_6155);
}

// Translated from 00f846b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d6628`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f846b0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_6628);
}

// Translated from 00f84df0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d6718`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f84df0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_6718);
}

// Translated from 00f85050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d6790`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f85050(e: &mut Engine) {
    run_00a2e180(e, 0x011d_6790);
}

// Translated from 00f855a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d689c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f855a0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_689c);
}

// Translated from 00f85800 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d6914`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f85800(e: &mut Engine) {
    run_00a2e180(e, 0x011d_6914);
}

// Translated from 00f85d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d6ac8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f85d00(e: &mut Engine) {
    run_00a2e180(e, 0x011d_6ac8);
}

// Translated from 00f85fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d6ae5`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f85fa0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_6ae5);
}

// Translated from 00f86220 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d6bc8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f86220(e: &mut Engine) {
    run_00a2e180(e, 0x011d_6bc8);
}

// Translated from 00f864a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d6c50`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f864a0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_6c50);
}

// Translated from 00f875b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d6ea8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f875b0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_6ea8);
}

// Translated from 00f89500 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d74c8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f89500(e: &mut Engine) {
    run_00a2e180(e, 0x011d_74c8);
}

// Translated from 00f89990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d7590`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f89990(e: &mut Engine) {
    run_00a2e180(e, 0x011d_7590);
}

// Translated from 00f8a070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d7698`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8a070(e: &mut Engine) {
    run_00a2e180(e, 0x011d_7698);
}

// Translated from 00f8ac10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d8444`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8ac10(e: &mut Engine) {
    run_00a2e180(e, 0x011d_8444);
}

// Translated from 00f8b410 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d857c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8b410(e: &mut Engine) {
    run_00a2e180(e, 0x011d_857c);
}

// Translated from 00f8b840 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d8654`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8b840(e: &mut Engine) {
    run_00a2e180(e, 0x011d_8654);
}

// Translated from 00f8bcb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the object at `011d8724`: `00403e20` on `011d870c` answers a pointer to a float; the object's constructor `0045d060` takes `0106e660` and that float (a quiet copy), then `_atexit` of `00fd7a30`.
pub fn fn_00f8bcb0(e: &mut Engine) {
    float_object_at_exit(e, 0x011d_870c, 0x011d_8724, 0x0106_e660, 0x00fd_7a30);
}

// Translated from 00f8bd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the object at `011d8788`: `00403e20` on `011d8770` answers a pointer to a float; the object's constructor `0045d060` takes `0106e700` and that float (a quiet copy), then `_atexit` of `00fd7a70`.
pub fn fn_00f8bd80(e: &mut Engine) {
    float_object_at_exit(e, 0x011d_8770, 0x011d_8788, 0x0106_e700, 0x00fd_7a70);
}

// Translated from 00f8c4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d88dc`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8c4d0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_88dc);
}

// Translated from 00f8c730 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d8a18`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8c730(e: &mut Engine) {
    run_00a2e180(e, 0x011d_8a18);
}

// Translated from 00f8cb70 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d8c34`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8cb70(e: &mut Engine) {
    run_00a2e180(e, 0x011d_8c34);
}

// Translated from 00f8d720 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d8db4`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8d720(e: &mut Engine) {
    run_00a2e180(e, 0x011d_8db4);
}

// Translated from 00f8da60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d8f2c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8da60(e: &mut Engine) {
    run_00a2e180(e, 0x011d_8f2c);
}

// Translated from 00f8dce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d8fc1`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8dce0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_8fc1);
}

// Translated from 00f8df40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d90c0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8df40(e: &mut Engine) {
    run_00a2e180(e, 0x011d_90c0);
}

// Translated from 00f8e1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d9154`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8e1e0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_9154);
}

// Translated from 00f8e440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d9185`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8e440(e: &mut Engine) {
    run_00a2e180(e, 0x011d_9185);
}

// Translated from 00f8e6e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d9288`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8e6e0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_9288);
}

// Translated from 00f8e940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d9318`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8e940(e: &mut Engine) {
    run_00a2e180(e, 0x011d_9318);
}

// Translated from 00f8eba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d93a8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8eba0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_93a8);
}

// Translated from 00f8ee30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d945c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8ee30(e: &mut Engine) {
    run_00a2e180(e, 0x011d_945c);
}

// Translated from 00f8f090 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d947d`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8f090(e: &mut Engine) {
    run_00a2e180(e, 0x011d_947d);
}

// Translated from 00f8f320 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d957c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8f320(e: &mut Engine) {
    run_00a2e180(e, 0x011d_957c);
}

// Translated from 00f8f580 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d9654`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8f580(e: &mut Engine) {
    run_00a2e180(e, 0x011d_9654);
}

// Translated from 00f8f920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d96c6`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8f920(e: &mut Engine) {
    run_00a2e180(e, 0x011d_96c6);
}

// Translated from 00f8fa80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the float at `011d979c`: the float `00403e20` on `011d9774` points to
/// times the double at `0101e568` (`FLD float; FMUL double`, x87 NaN rules), stored as a float.
pub fn fn_00f8fa80(e: &mut Engine) {
    let pointer = e.call(FLOAT_GETTER_00403E20, &args![0x011d_9774_u32]).u32();
    let value = x87_load32(e.global::<u32>(pointer));
    let factor = f64::from_bits(e.global::<u64>(0x0101_e568));
    let mut product = x87_mul(value, factor);
    // An invalid product of two non-NaN operands (infinity times zero) is the x87's default NaN.
    if product.is_nan() && !value.is_nan() && !factor.is_nan() {
        product = f64::from_bits(0xfff8_0000_0000_0000);
    }
    e.set_global::<u32>(0x011d_979c, x87_store32(product));
}

// Translated from 00f8fc90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d9f1c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8fc90(e: &mut Engine) {
    run_00a2e180(e, 0x011d_9f1c);
}

// Translated from 00f8ff50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011d9fb8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f8ff50(e: &mut Engine) {
    run_00a2e180(e, 0x011d_9fb8);
}

// Translated from 00f901b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011da078`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f901b0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_a078);
}

// Translated from 00f90460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011da0ca`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f90460(e: &mut Engine) {
    run_00a2e180(e, 0x011d_a0ca);
}

// Translated from 00f90940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011da274`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f90940(e: &mut Engine) {
    run_00a2e180(e, 0x011d_a274);
}

// Translated from 00f90990 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the words `011da28c` .. `011da29c`: `00430bc0(i)` (cdecl, one word) for
/// `i` = 0 to 4, each answer stored in turn; then 0x65 is stored in `011da2a0`.
pub fn fn_00f90990(e: &mut Engine) {
    store_result(e, 0x011d_a28c, FN_00430BC0, &args![0_u32]);
    store_result(e, 0x011d_a290, FN_00430BC0, &args![1_u32]);
    store_result(e, 0x011d_a294, FN_00430BC0, &args![2_u32]);
    store_result(e, 0x011d_a298, FN_00430BC0, &args![3_u32]);
    store_result(e, 0x011d_a29c, FN_00430BC0, &args![4_u32]);
    e.set_global::<u32>(0x011d_a2a0, 0x65);
}

// Translated from 00f90c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011da32c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f90c40(e: &mut Engine) {
    run_00a2e180(e, 0x011d_a32c);
}

// Translated from 00f90ec0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011da3e8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f90ec0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_a3e8);
}

// Translated from 00f911b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011da574`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f911b0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_a574);
}

// Translated from 00f91450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011da5f8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f91450(e: &mut Engine) {
    run_00a2e180(e, 0x011d_a5f8);
}

// Translated from 00f91750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011da689`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f91750(e: &mut Engine) {
    run_00a2e180(e, 0x011d_a689);
}

// Translated from 00f91aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011da7c0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f91aa0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_a7c0);
}

// Translated from 00f91d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011da854`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f91d00(e: &mut Engine) {
    run_00a2e180(e, 0x011d_a854);
}

// Translated from 00f91f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011da8e8`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f91f60(e: &mut Engine) {
    run_00a2e180(e, 0x011d_a8e8);
}

// Translated from 00f92200 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011da980`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f92200(e: &mut Engine) {
    run_00a2e180(e, 0x011d_a980);
}

// Translated from 00f92460 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011daa14`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f92460(e: &mut Engine) {
    run_00a2e180(e, 0x011d_aa14);
}

// Translated from 00f92700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011daa98`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f92700(e: &mut Engine) {
    run_00a2e180(e, 0x011d_aa98);
}

// Translated from 00f92980 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011daacf`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f92980(e: &mut Engine) {
    run_00a2e180(e, 0x011d_aacf);
}

// Translated from 00f92d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the words `011dac50` .. `011dac5c`: `00718e90` (cdecl, four words) with
/// four different sets of constants, each answer stored in turn.
pub fn fn_00f92d50(e: &mut Engine) {
    store_result(
        e,
        0x011d_ac50,
        FN_00718E90,
        &args![0x1a_u32, 0xff_u32, 0x80_u32, 0xff_u32],
    );
    store_result(
        e,
        0x011d_ac54,
        FN_00718E90,
        &args![0x2e_u32, 0xcf_u32, 0xff_u32, 0xff_u32],
    );
    store_result(
        e,
        0x011d_ac58,
        FN_00718E90,
        &args![0xff_u32, 0xb6_u32, 0x42_u32, 0xff_u32],
    );
    store_result(
        e,
        0x011d_ac5c,
        FN_00718E90,
        &args![0xc5_u32, 0xff_u32, 0xff_u32, 0xff_u32],
    );
}

// Translated from 00f93020 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dad44`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f93020(e: &mut Engine) {
    run_00a2e180(e, 0x011d_ad44);
}

// Translated from 00f93280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dae60`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f93280(e: &mut Engine) {
    run_00a2e180(e, 0x011d_ae60);
}

// Translated from 00f937a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011daf40`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f937a0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_af40);
}

// Translated from 00f93a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dafdc`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f93a00(e: &mut Engine) {
    run_00a2e180(e, 0x011d_afdc);
}

// Translated from 00f93c80 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011db06c`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f93c80(e: &mut Engine) {
    run_00a2e180(e, 0x011d_b06c);
}

// Translated from 00f93fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011db164`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f93fb0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_b164);
}

// Translated from 00f94590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011db334`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f94590(e: &mut Engine) {
    run_00a2e180(e, 0x011d_b334);
}

// Translated from 00f94a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011db379`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f94a30(e: &mut Engine) {
    run_00a2e180(e, 0x011d_b379);
}

// Translated from 00f951a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011db5c0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f951a0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_b5c0);
}

// Translated from 00f95760 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011db7ac`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f95760(e: &mut Engine) {
    run_00a2e180(e, 0x011d_b7ac);
}

// Translated from 00f95f60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011db874`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f95f60(e: &mut Engine) {
    run_00a2e180(e, 0x011d_b874);
}

// Translated from 00f961c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011db8ec`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f961c0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_b8ec);
}

// Translated from 00f96420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011db964`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f96420(e: &mut Engine) {
    run_00a2e180(e, 0x011d_b964);
}

// Translated from 00f96680 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011db9bd`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f96680(e: &mut Engine) {
    run_00a2e180(e, 0x011d_b9bd);
}

// Translated from 00f96690 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011db9bc`: `00403a70(0x4, 00802ff0)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f96690(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_b9bc, 0x4, 0x0080_2ff0);
}

// Translated from 00f96900 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dba58`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f96900(e: &mut Engine) {
    run_00a2e180(e, 0x011d_ba58);
}

// Translated from 00f96c00 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dbaec`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f96c00(e: &mut Engine) {
    run_00a2e180(e, 0x011d_baec);
}

// Translated from 00f96e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dbb64`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f96e90(e: &mut Engine) {
    run_00a2e180(e, 0x011d_bb64);
}

// Translated from 00f970f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dbbc9`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f970f0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_bbc9);
}

// Translated from 00f97100 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dbbc8`: `00403a70(0x12, 008085f0)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f97100(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_bbc8, 0x12, 0x0080_85f0);
}

// Translated from 00f97370 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dbc35`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f97370(e: &mut Engine) {
    run_00a2e180(e, 0x011d_bc35);
}

// Translated from 00f97380 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dbc34`: `00403a70(0x6, 0080ac70)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f97380(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_bc34, 0x6, 0x0080_ac70);
}

// Translated from 00f975f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dbc9d`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f975f0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_bc9d);
}

// Translated from 00f97600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dbc9c`: `00403a70(0xc, 0080b040)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f97600(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_bc9c, 0xc, 0x0080_b040);
}

// Translated from 00f97870 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dbd15`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f97870(e: &mut Engine) {
    run_00a2e180(e, 0x011d_bd15);
}

// Translated from 00f97880 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dbd14`: `00403a70(0x9, 0080b4e0)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f97880(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_bd14, 0x9, 0x0080_b4e0);
}

// Translated from 00f97af0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dbdbc`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f97af0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_bdbc);
}

// Translated from 00f97d50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dbe11`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f97d50(e: &mut Engine) {
    run_00a2e180(e, 0x011d_be11);
}

// Translated from 00f97d60 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dbe10`: `00403a70(0xa, 0080bbf0)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f97d60(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_be10, 0xa, 0x0080_bbf0);
}

// Translated from 00f97fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dbe52`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f97fd0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_be52);
}

// Translated from 00f97fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dbe51`: `00403a70(0x21, 0080bff0)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f97fe0(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_be51, 0x21, 0x0080_bff0);
}

// Translated from 00f98250 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dbedb`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f98250(e: &mut Engine) {
    run_00a2e180(e, 0x011d_bedb);
}

// Translated from 00f98260 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dbed8`: `00403a70(0x3, 0080c260)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f98260(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_bed8, 0x3, 0x0080_c260);
}

// Translated from 00f98280 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dbeda`: `00403a70(0x1e, 0080c2e0)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f98280(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_beda, 0x1e, 0x0080_c2e0);
}

// Translated from 00f982a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dbf38`: `00403a70(0x1f, 0080c360)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f982a0(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_bf38, 0x1f, 0x0080_c360);
}

// Translated from 00f982c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dbed9`: `00403a70(0x20, 0080c3e0)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f982c0(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_bed9, 0x20, 0x0080_c3e0);
}

// Translated from 00f98530 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dbf7d`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f98530(e: &mut Engine) {
    run_00a2e180(e, 0x011d_bf7d);
}

// Translated from 00f98540 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dbf7c`: `00403a70(0xe, 0080c730)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f98540(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_bf7c, 0xe, 0x0080_c730);
}

// Translated from 00f987b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc005`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f987b0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c005);
}

// Translated from 00f987c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dc004`: `00403a70(0x7, 0080cc70)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f987c0(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_c004, 0x7, 0x0080_cc70);
}

// Translated from 00f98a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc036`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f98a30(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c036);
}

// Translated from 00f98a40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dc035`: `00403a70(0x14, 0080d120)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f98a40(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_c035, 0x14, 0x0080_d120);
}

// Translated from 00f98cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc109`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f98cb0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c109);
}

// Translated from 00f98cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dc108`: `00403a70(0x16, 0080d430)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f98cc0(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_c108, 0x16, 0x0080_d430);
}

// Translated from 00f98f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc145`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f98f30(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c145);
}

// Translated from 00f98f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dc144`: `00403a70(0x17, 0080d880)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f98f40(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_c144, 0x17, 0x0080_d880);
}

// Translated from 00f991b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc1e9`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f991b0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c1e9);
}

// Translated from 00f991c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dc1e8`: `00403a70(0x2, 0080db30)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f991c0(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_c1e8, 0x2, 0x0080_db30);
}

// Translated from 00f99430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc21d`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f99430(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c21d);
}

// Translated from 00f99440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dc21c`: `00403a70(0x8, 0080df10)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f99440(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_c21c, 0x8, 0x0080_df10);
}

// Translated from 00f996b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc2e9`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f996b0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c2e9);
}

// Translated from 00f996c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dc2e8`: `00403a70(0xb, 0080e4c0)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f996c0(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_c2e8, 0xb, 0x0080_e4c0);
}

// Translated from 00f99930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc33d`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f99930(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c33d);
}

// Translated from 00f99940 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dc33c`: `00403a70(0xd, 0080ef50)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f99940(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_c33c, 0xd, 0x0080_ef50);
}

// Translated from 00f99bb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc3dc`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f99bb0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c3dc);
}

// Translated from 00f99bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dc3dd`: `00403a70(0x23, 0080f1a0)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f99bc0(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_c3dd, 0x23, 0x0080_f1a0);
}

// Translated from 00f99e30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc454`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f99e30(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c454);
}

// Translated from 00f99e40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the byte at `011dc455`: `00403a70(0x10, 0080f660)` (cdecl, two words), its low byte stored in the global.
pub fn fn_00f99e40(e: &mut Engine) {
    store_byte_00403a70(e, 0x011d_c455, 0x10, 0x0080_f660);
}

// Translated from 00f9a0b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc4cc`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f9a0b0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c4cc);
}

// Translated from 00f9a310 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc544`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f9a310(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c544);
}

// Translated from 00f9a570 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc5c0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f9a570(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c5c0);
}

// Translated from 00f9a7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc60d`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f9a7d0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c60d);
}

// Translated from 00f9aa40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc6b0`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f9aa40(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c6b0);
}

// Translated from 00f9adc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Dynamic initializer of the global at `011dc758`: runs `00a2e180` with `ECX` = the global.
pub fn fn_00f9adc0(e: &mut Engine) {
    run_00a2e180(e, 0x011d_c758);
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00f3_5420, fn_00f35420()),
        entry!(0x00f3_59e0, fn_00f359e0()),
        entry!(0x00f3_5fd0, fn_00f35fd0()),
        entry!(0x00f3_64f0, fn_00f364f0()),
        entry!(0x00f3_6fc0, fn_00f36fc0()),
        entry!(0x00f3_73c0, fn_00f373c0()),
        entry!(0x00f3_7690, fn_00f37690()),
        entry!(0x00f3_7ec0, fn_00f37ec0()),
        entry!(0x00f3_8220, fn_00f38220()),
        entry!(0x00f3_8300, fn_00f38300()),
        entry!(0x00f3_8e00, fn_00f38e00()),
        entry!(0x00f3_9530, fn_00f39530()),
        entry!(0x00f3_98b0, fn_00f398b0()),
        entry!(0x00f3_9c10, fn_00f39c10()),
        entry!(0x00f3_9ee0, fn_00f39ee0()),
        entry!(0x00f3_a280, fn_00f3a280()),
        entry!(0x00f3_b7a0, fn_00f3b7a0()),
        entry!(0x00f3_bc40, fn_00f3bc40()),
        entry!(0x00f3_bea0, fn_00f3bea0()),
        entry!(0x00f3_cba0, fn_00f3cba0()),
        entry!(0x00f3_d020, fn_00f3d020()),
        entry!(0x00f3_d560, fn_00f3d560()),
        entry!(0x00f3_db50, fn_00f3db50()),
        entry!(0x00f3_e8e0, fn_00f3e8e0()),
        entry!(0x00f3_e9b0, fn_00f3e9b0()),
        entry!(0x00f3_edc0, fn_00f3edc0()),
        entry!(0x00f3_f180, fn_00f3f180()),
        entry!(0x00f3_f410, fn_00f3f410()),
        entry!(0x00f3_f8d0, fn_00f3f8d0()),
        entry!(0x00f3_fc30, fn_00f3fc30()),
        entry!(0x00f4_0b10, fn_00f40b10()),
        entry!(0x00f4_0c10, fn_00f40c10()),
        entry!(0x00f4_0d10, fn_00f40d10()),
        entry!(0x00f4_16c0, fn_00f416c0()),
        entry!(0x00f4_1820, fn_00f41820()),
        entry!(0x00f4_2230, fn_00f42230()),
        entry!(0x00f4_2e10, fn_00f42e10()),
        entry!(0x00f4_3190, fn_00f43190()),
        entry!(0x00f4_3470, fn_00f43470()),
        entry!(0x00f4_4170, fn_00f44170()),
        entry!(0x00f4_4530, fn_00f44530()),
        entry!(0x00f4_4790, fn_00f44790()),
        entry!(0x00f4_4a30, fn_00f44a30()),
        entry!(0x00f4_4d10, fn_00f44d10()),
        entry!(0x00f4_51e0, fn_00f451e0()),
        entry!(0x00f4_5770, fn_00f45770()),
        entry!(0x00f4_59d0, fn_00f459d0()),
        entry!(0x00f4_5c90, fn_00f45c90()),
        entry!(0x00f4_7e00, fn_00f47e00()),
        entry!(0x00f4_8090, fn_00f48090()),
        entry!(0x00f4_8450, fn_00f48450()),
        entry!(0x00f4_86b0, fn_00f486b0()),
        entry!(0x00f4_8900, fn_00f48900()),
        entry!(0x00f4_8b60, fn_00f48b60()),
        entry!(0x00f4_8dc0, fn_00f48dc0()),
        entry!(0x00f4_92f0, fn_00f492f0()),
        entry!(0x00f4_95c0, fn_00f495c0()),
        entry!(0x00f4_9820, fn_00f49820()),
        entry!(0x00f4_9f10, fn_00f49f10()),
        entry!(0x00f4_a200, fn_00f4a200()),
        entry!(0x00f4_a7f0, fn_00f4a7f0()),
        entry!(0x00f4_ad00, fn_00f4ad00()),
        entry!(0x00f4_af80, fn_00f4af80()),
        entry!(0x00f4_b250, fn_00f4b250()),
        entry!(0x00f4_b6c0, fn_00f4b6c0()),
        entry!(0x00f4_bb40, fn_00f4bb40()),
        entry!(0x00f4_c1b0, fn_00f4c1b0()),
        entry!(0x00f4_c2e0, fn_00f4c2e0()),
        entry!(0x00f4_c600, fn_00f4c600()),
        entry!(0x00f4_cb90, fn_00f4cb90()),
        entry!(0x00f4_d030, fn_00f4d030()),
        entry!(0x00f4_d990, fn_00f4d990()),
        entry!(0x00f4_dbf0, fn_00f4dbf0()),
        entry!(0x00f4_de50, fn_00f4de50()),
        entry!(0x00f4_e310, fn_00f4e310()),
        entry!(0x00f4_e790, fn_00f4e790()),
        entry!(0x00f4_e9f0, fn_00f4e9f0()),
        entry!(0x00f4_ed20, fn_00f4ed20()),
        entry!(0x00f4_ef80, fn_00f4ef80()),
        entry!(0x00f4_f240, fn_00f4f240()),
        entry!(0x00f4_f5a0, fn_00f4f5a0()),
        entry!(0x00f4_fa50, fn_00f4fa50()),
        entry!(0x00f5_0830, fn_00f50830()),
        entry!(0x00f5_0b70, fn_00f50b70()),
        entry!(0x00f5_0dd0, fn_00f50dd0()),
        entry!(0x00f5_1030, fn_00f51030()),
        entry!(0x00f5_1450, fn_00f51450()),
        entry!(0x00f5_16b0, fn_00f516b0()),
        entry!(0x00f5_1950, fn_00f51950()),
        entry!(0x00f5_19f0, fn_00f519f0()),
        entry!(0x00f5_1a20, fn_00f51a20()),
        entry!(0x00f5_1da0, fn_00f51da0()),
        entry!(0x00f5_21e0, fn_00f521e0()),
        entry!(0x00f5_2440, fn_00f52440()),
        entry!(0x00f5_26a0, fn_00f526a0()),
        entry!(0x00f5_29e0, fn_00f529e0()),
        entry!(0x00f5_2c70, fn_00f52c70()),
        entry!(0x00f5_2ef0, fn_00f52ef0()),
        entry!(0x00f5_3160, fn_00f53160()),
        entry!(0x00f5_3420, fn_00f53420()),
        entry!(0x00f5_4340, fn_00f54340()),
        entry!(0x00f5_4850, fn_00f54850()),
        entry!(0x00f5_4af0, fn_00f54af0()),
        entry!(0x00f5_4bf0, fn_00f54bf0()),
        entry!(0x00f5_4e90, fn_00f54e90()),
        entry!(0x00f5_51f0, fn_00f551f0()),
        entry!(0x00f5_56b0, fn_00f556b0()),
        entry!(0x00f5_5d30, fn_00f55d30()),
        entry!(0x00f5_6090, fn_00f56090()),
        entry!(0x00f5_6310, fn_00f56310()),
        entry!(0x00f5_6570, fn_00f56570()),
        entry!(0x00f5_67d0, fn_00f567d0()),
        entry!(0x00f5_6a80, fn_00f56a80()),
        entry!(0x00f5_79c0, fn_00f579c0()),
        entry!(0x00f5_7c20, fn_00f57c20()),
        entry!(0x00f5_7eb0, fn_00f57eb0()),
        entry!(0x00f5_8110, fn_00f58110()),
        entry!(0x00f5_8370, fn_00f58370()),
        entry!(0x00f5_87a0, fn_00f587a0()),
        entry!(0x00f5_8a10, fn_00f58a10()),
        entry!(0x00f5_8d10, fn_00f58d10()),
        entry!(0x00f5_9030, fn_00f59030()),
        entry!(0x00f5_9410, fn_00f59410()),
        entry!(0x00f5_cb40, fn_00f5cb40()),
        entry!(0x00f5_d7d0, fn_00f5d7d0()),
        entry!(0x00f7_9710, fn_00f79710()),
        entry!(0x00f8_0d80, fn_00f80d80()),
        entry!(0x00f8_1920, fn_00f81920()),
        entry!(0x00f8_2160, fn_00f82160()),
        entry!(0x00f8_23e0, fn_00f823e0()),
        entry!(0x00f8_2e30, fn_00f82e30()),
        entry!(0x00f8_3720, fn_00f83720()),
        entry!(0x00f8_39a0, fn_00f839a0()),
        entry!(0x00f8_46b0, fn_00f846b0()),
        entry!(0x00f8_4df0, fn_00f84df0()),
        entry!(0x00f8_5050, fn_00f85050()),
        entry!(0x00f8_55a0, fn_00f855a0()),
        entry!(0x00f8_5800, fn_00f85800()),
        entry!(0x00f8_5d00, fn_00f85d00()),
        entry!(0x00f8_5fa0, fn_00f85fa0()),
        entry!(0x00f8_6220, fn_00f86220()),
        entry!(0x00f8_64a0, fn_00f864a0()),
        entry!(0x00f8_75b0, fn_00f875b0()),
        entry!(0x00f8_9500, fn_00f89500()),
        entry!(0x00f8_9990, fn_00f89990()),
        entry!(0x00f8_a070, fn_00f8a070()),
        entry!(0x00f8_ac10, fn_00f8ac10()),
        entry!(0x00f8_b410, fn_00f8b410()),
        entry!(0x00f8_b840, fn_00f8b840()),
        entry!(0x00f8_bcb0, fn_00f8bcb0()),
        entry!(0x00f8_bd80, fn_00f8bd80()),
        entry!(0x00f8_c4d0, fn_00f8c4d0()),
        entry!(0x00f8_c730, fn_00f8c730()),
        entry!(0x00f8_cb70, fn_00f8cb70()),
        entry!(0x00f8_d720, fn_00f8d720()),
        entry!(0x00f8_da60, fn_00f8da60()),
        entry!(0x00f8_dce0, fn_00f8dce0()),
        entry!(0x00f8_df40, fn_00f8df40()),
        entry!(0x00f8_e1e0, fn_00f8e1e0()),
        entry!(0x00f8_e440, fn_00f8e440()),
        entry!(0x00f8_e6e0, fn_00f8e6e0()),
        entry!(0x00f8_e940, fn_00f8e940()),
        entry!(0x00f8_eba0, fn_00f8eba0()),
        entry!(0x00f8_ee30, fn_00f8ee30()),
        entry!(0x00f8_f090, fn_00f8f090()),
        entry!(0x00f8_f320, fn_00f8f320()),
        entry!(0x00f8_f580, fn_00f8f580()),
        entry!(0x00f8_f920, fn_00f8f920()),
        entry!(0x00f8_fa80, fn_00f8fa80()),
        entry!(0x00f8_fc90, fn_00f8fc90()),
        entry!(0x00f8_ff50, fn_00f8ff50()),
        entry!(0x00f9_01b0, fn_00f901b0()),
        entry!(0x00f9_0460, fn_00f90460()),
        entry!(0x00f9_0940, fn_00f90940()),
        entry!(0x00f9_0990, fn_00f90990()),
        entry!(0x00f9_0c40, fn_00f90c40()),
        entry!(0x00f9_0ec0, fn_00f90ec0()),
        entry!(0x00f9_11b0, fn_00f911b0()),
        entry!(0x00f9_1450, fn_00f91450()),
        entry!(0x00f9_1750, fn_00f91750()),
        entry!(0x00f9_1aa0, fn_00f91aa0()),
        entry!(0x00f9_1d00, fn_00f91d00()),
        entry!(0x00f9_1f60, fn_00f91f60()),
        entry!(0x00f9_2200, fn_00f92200()),
        entry!(0x00f9_2460, fn_00f92460()),
        entry!(0x00f9_2700, fn_00f92700()),
        entry!(0x00f9_2980, fn_00f92980()),
        entry!(0x00f9_2d50, fn_00f92d50()),
        entry!(0x00f9_3020, fn_00f93020()),
        entry!(0x00f9_3280, fn_00f93280()),
        entry!(0x00f9_37a0, fn_00f937a0()),
        entry!(0x00f9_3a00, fn_00f93a00()),
        entry!(0x00f9_3c80, fn_00f93c80()),
        entry!(0x00f9_3fb0, fn_00f93fb0()),
        entry!(0x00f9_4590, fn_00f94590()),
        entry!(0x00f9_4a30, fn_00f94a30()),
        entry!(0x00f9_51a0, fn_00f951a0()),
        entry!(0x00f9_5760, fn_00f95760()),
        entry!(0x00f9_5f60, fn_00f95f60()),
        entry!(0x00f9_61c0, fn_00f961c0()),
        entry!(0x00f9_6420, fn_00f96420()),
        entry!(0x00f9_6680, fn_00f96680()),
        entry!(0x00f9_6690, fn_00f96690()),
        entry!(0x00f9_6900, fn_00f96900()),
        entry!(0x00f9_6c00, fn_00f96c00()),
        entry!(0x00f9_6e90, fn_00f96e90()),
        entry!(0x00f9_70f0, fn_00f970f0()),
        entry!(0x00f9_7100, fn_00f97100()),
        entry!(0x00f9_7370, fn_00f97370()),
        entry!(0x00f9_7380, fn_00f97380()),
        entry!(0x00f9_75f0, fn_00f975f0()),
        entry!(0x00f9_7600, fn_00f97600()),
        entry!(0x00f9_7870, fn_00f97870()),
        entry!(0x00f9_7880, fn_00f97880()),
        entry!(0x00f9_7af0, fn_00f97af0()),
        entry!(0x00f9_7d50, fn_00f97d50()),
        entry!(0x00f9_7d60, fn_00f97d60()),
        entry!(0x00f9_7fd0, fn_00f97fd0()),
        entry!(0x00f9_7fe0, fn_00f97fe0()),
        entry!(0x00f9_8250, fn_00f98250()),
        entry!(0x00f9_8260, fn_00f98260()),
        entry!(0x00f9_8280, fn_00f98280()),
        entry!(0x00f9_82a0, fn_00f982a0()),
        entry!(0x00f9_82c0, fn_00f982c0()),
        entry!(0x00f9_8530, fn_00f98530()),
        entry!(0x00f9_8540, fn_00f98540()),
        entry!(0x00f9_87b0, fn_00f987b0()),
        entry!(0x00f9_87c0, fn_00f987c0()),
        entry!(0x00f9_8a30, fn_00f98a30()),
        entry!(0x00f9_8a40, fn_00f98a40()),
        entry!(0x00f9_8cb0, fn_00f98cb0()),
        entry!(0x00f9_8cc0, fn_00f98cc0()),
        entry!(0x00f9_8f30, fn_00f98f30()),
        entry!(0x00f9_8f40, fn_00f98f40()),
        entry!(0x00f9_91b0, fn_00f991b0()),
        entry!(0x00f9_91c0, fn_00f991c0()),
        entry!(0x00f9_9430, fn_00f99430()),
        entry!(0x00f9_9440, fn_00f99440()),
        entry!(0x00f9_96b0, fn_00f996b0()),
        entry!(0x00f9_96c0, fn_00f996c0()),
        entry!(0x00f9_9930, fn_00f99930()),
        entry!(0x00f9_9940, fn_00f99940()),
        entry!(0x00f9_9bb0, fn_00f99bb0()),
        entry!(0x00f9_9bc0, fn_00f99bc0()),
        entry!(0x00f9_9e30, fn_00f99e30()),
        entry!(0x00f9_9e40, fn_00f99e40()),
        entry!(0x00f9_a0b0, fn_00f9a0b0()),
        entry!(0x00f9_a310, fn_00f9a310()),
        entry!(0x00f9_a570, fn_00f9a570()),
        entry!(0x00f9_a7d0, fn_00f9a7d0()),
        entry!(0x00f9_aa40, fn_00f9aa40()),
        entry!(0x00f9_adc0, fn_00f9adc0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::rc::Rc;

    type Calls = Vec<(u32, Vec<u32>)>;

    /// Where the float getter double keeps its float.
    const FLOAT_AT: u32 = 0x0800_0000;

    /// Runs the function at `entry` and returns the calls it made, in order (the top-level call
    /// itself is filtered out by its address).
    fn run(e: &mut Engine, entry: u32) -> Calls {
        e.call_log = Some(vec![]);
        e.call(entry, &args![]);
        let log = e.call_log.take().unwrap();
        log.into_iter().filter(|(a, _)| *a != entry).collect()
    }

    /// Doubles that do nothing for each address.
    fn quiet(e: &mut Engine, addrs: &[u32]) {
        for a in addrs {
            e.register(*a, |_, _| Ret::default());
        }
    }

    /// A double for `00403e20` that answers a pointer to the float with the bits `bits`.
    fn float_getter(e: &mut Engine, bits: u32) {
        e.map(FLOAT_AT, 4);
        e.set_global::<u32>(FLOAT_AT, bits);
        e.register(FLOAT_GETTER_00403E20, |_, _| Ret {
            eax: FLOAT_AT,
            ..Ret::default()
        });
    }

    /// The calls of `callee` in `calls`.
    fn only(calls: &Calls, callee: u32) -> Calls {
        calls
            .iter()
            .filter(|(a, _)| *a == callee)
            .cloned()
            .collect()
    }

    /// (function, global)
    const SHAPE_A: &[(u32, u32)] = &[
        (0x00f3_5420, 0x011c_32f0),
        (0x00f3_59e0, 0x011c_33b0),
        (0x00f3_5fd0, 0x011c_34ec),
        (0x00f3_64f0, 0x011c_37cc),
        (0x00f3_6fc0, 0x011c_3880),
        (0x00f3_73c0, 0x011c_3970),
        (0x00f3_7690, 0x011c_3aa8),
        (0x00f3_7ec0, 0x011c_3bd8),
        (0x00f3_8220, 0x011c_3e34),
        (0x00f3_8e00, 0x011c_403c),
        (0x00f3_9530, 0x011c_4168),
        (0x00f3_98b0, 0x011c_4c04),
        (0x00f3_9c10, 0x011c_4ca4),
        (0x00f3_9ee0, 0x011c_4d24),
        (0x00f3_a280, 0x011c_5194),
        (0x00f3_b7a0, 0x011c_52c1),
        (0x00f3_bc40, 0x011c_5400),
        (0x00f3_bea0, 0x011c_5478),
        (0x00f3_cba0, 0x011c_567c),
        (0x00f3_d020, 0x011c_5790),
        (0x00f3_d560, 0x011c_5910),
        (0x00f3_db50, 0x011c_5c1c),
        (0x00f3_e8e0, 0x011c_5cb5),
        (0x00f3_edc0, 0x011c_6308),
        (0x00f3_f180, 0x011c_6414),
        (0x00f3_f410, 0x011c_64c0),
        (0x00f3_f8d0, 0x011c_6f78),
        (0x00f3_fc30, 0x011c_70f9),
        (0x00f4_2230, 0x011c_7b79),
        (0x00f4_2e10, 0x011c_7da8),
        (0x00f4_3190, 0x011c_7e44),
        (0x00f4_3470, 0x011c_7ed0),
        (0x00f4_4170, 0x011c_80f4),
        (0x00f4_4530, 0x011c_819c),
        (0x00f4_4790, 0x011c_8214),
        (0x00f4_4a30, 0x011c_82a8),
        (0x00f4_4d10, 0x011c_838c),
        (0x00f4_51e0, 0x011c_8438),
        (0x00f4_5770, 0x011c_851c),
        (0x00f4_59d0, 0x011c_85ac),
        (0x00f4_5c90, 0x011c_8ccc),
        (0x00f4_7e00, 0x011c_8e4c),
        (0x00f4_8090, 0x011c_8ec8),
        (0x00f4_8450, 0x011c_8f80),
        (0x00f4_86b0, 0x011c_8fa2),
        (0x00f4_8b60, 0x011c_910c),
        (0x00f4_8dc0, 0x011c_9190),
        (0x00f4_92f0, 0x011c_92a8),
        (0x00f4_95c0, 0x011c_9330),
        (0x00f4_9820, 0x011c_9355),
        (0x00f4_9f10, 0x011c_94f0),
        (0x00f4_a200, 0x011c_958c),
        (0x00f4_a7f0, 0x011c_9684),
        (0x00f4_ad00, 0x011c_9e9c),
        (0x00f4_af80, 0x011c_a010),
        (0x00f4_b6c0, 0x011c_a091),
        (0x00f4_bb40, 0x011c_a3e0),
        (0x00f4_c1b0, 0x011c_a504),
        (0x00f4_c600, 0x011c_a5a8),
        (0x00f4_cb90, 0x011c_a6c4),
        (0x00f4_d030, 0x011c_a7f4),
        (0x00f4_d990, 0x011c_a958),
        (0x00f4_dbf0, 0x011c_a9d0),
        (0x00f4_de50, 0x011c_aa5c),
        (0x00f4_e310, 0x011c_ab25),
        (0x00f4_e790, 0x011c_ac48),
        (0x00f4_e9f0, 0x011c_acf0),
        (0x00f4_ed20, 0x011c_ada4),
        (0x00f4_ef80, 0x011c_ae2c),
        (0x00f4_f240, 0x011c_aeb0),
        (0x00f4_f5a0, 0x011c_af37),
        (0x00f4_fa50, 0x011c_b06c),
        (0x00f5_0830, 0x011c_b0e4),
        (0x00f5_0b70, 0x011c_b178),
        (0x00f5_0dd0, 0x011c_b1f0),
        (0x00f5_1030, 0x011c_b268),
        (0x00f5_1450, 0x011c_b318),
        (0x00f5_16b0, 0x011c_b390),
        (0x00f5_1950, 0x011c_b514),
        (0x00f5_1da0, 0x011c_b5e0),
        (0x00f5_21e0, 0x011c_b684),
        (0x00f5_2440, 0x011c_b6a9),
        (0x00f5_26a0, 0x011c_b79c),
        (0x00f5_29e0, 0x011c_b850),
        (0x00f5_2c70, 0x011c_b8d4),
        (0x00f5_2ef0, 0x011c_b950),
        (0x00f5_3160, 0x011c_b9e5),
        (0x00f5_3420, 0x011c_bd14),
        (0x00f5_4340, 0x011c_be40),
        (0x00f5_4850, 0x011c_bf20),
        (0x00f5_4af0, 0x011c_c088),
        (0x00f5_4e90, 0x011c_c108),
        (0x00f5_51f0, 0x011c_c1ac),
        (0x00f5_56b0, 0x011c_c290),
        (0x00f5_5d30, 0x011c_c3a0),
        (0x00f5_6090, 0x011c_c440),
        (0x00f5_6310, 0x011c_c4b8),
        (0x00f5_6570, 0x011c_c530),
        (0x00f5_67d0, 0x011c_c5c8),
        (0x00f5_6a80, 0x011c_c67c),
        (0x00f5_79c0, 0x011c_c9e0),
        (0x00f5_7c20, 0x011c_ca58),
        (0x00f5_7eb0, 0x011c_cae0),
        (0x00f5_8110, 0x011c_cb58),
        (0x00f5_8370, 0x011c_ccd8),
        (0x00f5_8a10, 0x011c_cd68),
        (0x00f5_8d10, 0x011c_ce18),
        (0x00f5_9030, 0x011c_cf00),
        (0x00f5_9410, 0x011c_ddb4),
        (0x00f5_d7d0, 0x011c_faac),
        (0x00f8_0d80, 0x011d_5920),
        (0x00f8_1920, 0x011d_5b1c),
        (0x00f8_2160, 0x011d_5c28),
        (0x00f8_23e0, 0x011d_5d80),
        (0x00f8_2e30, 0x011d_6080),
        (0x00f8_3720, 0x011d_6134),
        (0x00f8_39a0, 0x011d_6155),
        (0x00f8_46b0, 0x011d_6628),
        (0x00f8_4df0, 0x011d_6718),
        (0x00f8_5050, 0x011d_6790),
        (0x00f8_55a0, 0x011d_689c),
        (0x00f8_5800, 0x011d_6914),
        (0x00f8_5d00, 0x011d_6ac8),
        (0x00f8_5fa0, 0x011d_6ae5),
        (0x00f8_6220, 0x011d_6bc8),
        (0x00f8_64a0, 0x011d_6c50),
        (0x00f8_75b0, 0x011d_6ea8),
        (0x00f8_9500, 0x011d_74c8),
        (0x00f8_9990, 0x011d_7590),
        (0x00f8_a070, 0x011d_7698),
        (0x00f8_ac10, 0x011d_8444),
        (0x00f8_b410, 0x011d_857c),
        (0x00f8_b840, 0x011d_8654),
        (0x00f8_c4d0, 0x011d_88dc),
        (0x00f8_c730, 0x011d_8a18),
        (0x00f8_cb70, 0x011d_8c34),
        (0x00f8_d720, 0x011d_8db4),
        (0x00f8_da60, 0x011d_8f2c),
        (0x00f8_dce0, 0x011d_8fc1),
        (0x00f8_df40, 0x011d_90c0),
        (0x00f8_e1e0, 0x011d_9154),
        (0x00f8_e440, 0x011d_9185),
        (0x00f8_e6e0, 0x011d_9288),
        (0x00f8_e940, 0x011d_9318),
        (0x00f8_eba0, 0x011d_93a8),
        (0x00f8_ee30, 0x011d_945c),
        (0x00f8_f090, 0x011d_947d),
        (0x00f8_f320, 0x011d_957c),
        (0x00f8_f580, 0x011d_9654),
        (0x00f8_f920, 0x011d_96c6),
        (0x00f8_fc90, 0x011d_9f1c),
        (0x00f8_ff50, 0x011d_9fb8),
        (0x00f9_01b0, 0x011d_a078),
        (0x00f9_0460, 0x011d_a0ca),
        (0x00f9_0940, 0x011d_a274),
        (0x00f9_0c40, 0x011d_a32c),
        (0x00f9_0ec0, 0x011d_a3e8),
        (0x00f9_11b0, 0x011d_a574),
        (0x00f9_1450, 0x011d_a5f8),
        (0x00f9_1750, 0x011d_a689),
        (0x00f9_1aa0, 0x011d_a7c0),
        (0x00f9_1d00, 0x011d_a854),
        (0x00f9_1f60, 0x011d_a8e8),
        (0x00f9_2200, 0x011d_a980),
        (0x00f9_2460, 0x011d_aa14),
        (0x00f9_2700, 0x011d_aa98),
        (0x00f9_2980, 0x011d_aacf),
        (0x00f9_3020, 0x011d_ad44),
        (0x00f9_3280, 0x011d_ae60),
        (0x00f9_37a0, 0x011d_af40),
        (0x00f9_3a00, 0x011d_afdc),
        (0x00f9_3c80, 0x011d_b06c),
        (0x00f9_3fb0, 0x011d_b164),
        (0x00f9_4590, 0x011d_b334),
        (0x00f9_4a30, 0x011d_b379),
        (0x00f9_51a0, 0x011d_b5c0),
        (0x00f9_5760, 0x011d_b7ac),
        (0x00f9_5f60, 0x011d_b874),
        (0x00f9_61c0, 0x011d_b8ec),
        (0x00f9_6420, 0x011d_b964),
        (0x00f9_6680, 0x011d_b9bd),
        (0x00f9_6900, 0x011d_ba58),
        (0x00f9_6c00, 0x011d_baec),
        (0x00f9_6e90, 0x011d_bb64),
        (0x00f9_70f0, 0x011d_bbc9),
        (0x00f9_7370, 0x011d_bc35),
        (0x00f9_75f0, 0x011d_bc9d),
        (0x00f9_7870, 0x011d_bd15),
        (0x00f9_7af0, 0x011d_bdbc),
        (0x00f9_7d50, 0x011d_be11),
        (0x00f9_7fd0, 0x011d_be52),
        (0x00f9_8250, 0x011d_bedb),
        (0x00f9_8530, 0x011d_bf7d),
        (0x00f9_87b0, 0x011d_c005),
        (0x00f9_8a30, 0x011d_c036),
        (0x00f9_8cb0, 0x011d_c109),
        (0x00f9_8f30, 0x011d_c145),
        (0x00f9_91b0, 0x011d_c1e9),
        (0x00f9_9430, 0x011d_c21d),
        (0x00f9_96b0, 0x011d_c2e9),
        (0x00f9_9930, 0x011d_c33d),
        (0x00f9_9bb0, 0x011d_c3dc),
        (0x00f9_9e30, 0x011d_c454),
        (0x00f9_a0b0, 0x011d_c4cc),
        (0x00f9_a310, 0x011d_c544),
        (0x00f9_a570, 0x011d_c5c0),
        (0x00f9_a7d0, 0x011d_c60d),
        (0x00f9_aa40, 0x011d_c6b0),
        (0x00f9_adc0, 0x011d_c758),
    ];

    /// (function, byte global, first word, second word)
    const SHAPE_B: &[(u32, u32, u32, u32)] = &[
        (0x00f9_6690, 0x011d_b9bc, 0x4, 0x0080_2ff0),
        (0x00f9_7100, 0x011d_bbc8, 0x12, 0x0080_85f0),
        (0x00f9_7380, 0x011d_bc34, 0x6, 0x0080_ac70),
        (0x00f9_7600, 0x011d_bc9c, 0xc, 0x0080_b040),
        (0x00f9_7880, 0x011d_bd14, 0x9, 0x0080_b4e0),
        (0x00f9_7d60, 0x011d_be10, 0xa, 0x0080_bbf0),
        (0x00f9_7fe0, 0x011d_be51, 0x21, 0x0080_bff0),
        (0x00f9_8260, 0x011d_bed8, 0x3, 0x0080_c260),
        (0x00f9_8280, 0x011d_beda, 0x1e, 0x0080_c2e0),
        (0x00f9_82a0, 0x011d_bf38, 0x1f, 0x0080_c360),
        (0x00f9_82c0, 0x011d_bed9, 0x20, 0x0080_c3e0),
        (0x00f9_8540, 0x011d_bf7c, 0xe, 0x0080_c730),
        (0x00f9_87c0, 0x011d_c004, 0x7, 0x0080_cc70),
        (0x00f9_8a40, 0x011d_c035, 0x14, 0x0080_d120),
        (0x00f9_8cc0, 0x011d_c108, 0x16, 0x0080_d430),
        (0x00f9_8f40, 0x011d_c144, 0x17, 0x0080_d880),
        (0x00f9_91c0, 0x011d_c1e8, 0x2, 0x0080_db30),
        (0x00f9_9440, 0x011d_c21c, 0x8, 0x0080_df10),
        (0x00f9_96c0, 0x011d_c2e8, 0xb, 0x0080_e4c0),
        (0x00f9_9940, 0x011d_c33c, 0xd, 0x0080_ef50),
        (0x00f9_9bc0, 0x011d_c3dd, 0x23, 0x0080_f1a0),
        (0x00f9_9e40, 0x011d_c455, 0x10, 0x0080_f660),
    ];

    /// (function, array, size, count, constructor, destructor, exit function)
    const SHAPE_C: &[(u32, u32, u32, u32, u32, u32, u32)] = &[
        (
            0x00f3_e9b0,
            0x011c_5d10,
            0x18,
            0x14,
            0x0048_8f50,
            0x0048_9070,
            0x00fc_9090,
        ),
        (
            0x00f4_16c0,
            0x011c_7804,
            0x4,
            0x3,
            0x0066_94e0,
            0x0045_cec0,
            0x00fc_9a10,
        ),
        (
            0x00f4_1820,
            0x011c_7860,
            0x4,
            0x3,
            0x0066_94e0,
            0x0045_cec0,
            0x00fc_9a60,
        ),
        (
            0x00f5_19f0,
            0x011c_b3e0,
            0x8,
            0xe,
            0x0096_a2d0,
            0x0046_ffb0,
            0x00fc_b260,
        ),
        (
            0x00f5_1a20,
            0x011c_b450,
            0x8,
            0xe,
            0x0096_a2d0,
            0x0046_ffb0,
            0x00fc_b280,
        ),
        (
            0x00f5_4bf0,
            0x011c_bf60,
            0x54,
            0x2,
            0x005a_a0f0,
            0x005a_a1a0,
            0x00fc_b8c0,
        ),
    ];

    /// (function, getter global, object, constant word, exit function)
    const SHAPE_D: &[(u32, u32, u32, u32, u32)] = &[
        (
            0x00f4_0b10,
            0x011c_744c,
            0x011c_7740,
            0x0102_2218,
            0x00fc_9680,
        ),
        (
            0x00f4_0c10,
            0x011c_7278,
            0x011c_7568,
            0x0102_22b8,
            0x00fc_96d0,
        ),
        (
            0x00f4_0d10,
            0x011c_7494,
            0x011c_7598,
            0x0102_2354,
            0x00fc_9720,
        ),
        (
            0x00f4_c2e0,
            0x011c_a4cc,
            0x011c_a450,
            0x0103_1218,
            0x00fc_b040,
        ),
        (
            0x00f8_bcb0,
            0x011d_870c,
            0x011d_8724,
            0x0106_e660,
            0x00fd_7a30,
        ),
        (
            0x00f8_bd80,
            0x011d_8770,
            0x011d_8788,
            0x0106_e700,
            0x00fd_7a70,
        ),
    ];

    /// Every address of the file is registered exactly once.
    #[test]
    fn every_function_is_registered_once() {
        let funcs = funcs();
        assert_eq!(funcs.len(), 252);
        assert_eq!(
            SHAPE_A.len() + SHAPE_B.len() + SHAPE_C.len() + SHAPE_D.len() + 9,
            252
        );
        let mut addrs: Vec<u32> = funcs.iter().map(|(a, _)| *a).collect();
        let sorted = addrs.windows(2).all(|w| w[0] < w[1]);
        assert!(sorted, "ascending and unique");
        addrs.dedup();
        assert_eq!(addrs.len(), 252);
        let e = Engine::new();
        for a in addrs {
            assert!(e.is_translated(a), "{a:08x}");
        }
    }

    #[test]
    fn shape_a_runs_00a2e180_on_the_global() {
        let mut e = Engine::new();
        quiet(&mut e, &[FN_00A2E180]);
        assert_eq!(SHAPE_A.len(), 209);
        for &(f, global) in SHAPE_A {
            assert_eq!(run(&mut e, f), vec![(FN_00A2E180, vec![global])], "{f:08x}");
        }
    }

    #[test]
    fn shape_b_stores_the_low_byte_of_the_answer() {
        let mut e = Engine::new();
        e.map(0x011d_b000, 0x2000);
        assert_eq!(SHAPE_B.len(), 22);
        for answer in [0x1234_5678_u32, 0xffff_ff00] {
            e.register_double(FN_00403A70, move |_, _| Ret {
                eax: answer,
                ..Ret::default()
            });
            for addr in 0x011d_b900..0x011d_c500 {
                e.set_global::<u8>(addr, 0xaa);
            }
            for &(f, _, first, second) in SHAPE_B {
                assert_eq!(
                    run(&mut e, f),
                    vec![(FN_00403A70, vec![first, second])],
                    "{f:08x}"
                );
            }
            for addr in 0x011d_b900..0x011d_c500 {
                let stored = SHAPE_B.iter().any(|b| b.1 == addr);
                let expected = if stored { answer as u8 } else { 0xaa };
                assert_eq!(e.global::<u8>(addr), expected, "{addr:08x}");
            }
        }
    }

    #[test]
    fn shape_c_constructs_the_array_then_registers_the_exit_function() {
        let mut e = Engine::new();
        quiet(&mut e, &[EH_VECTOR_CONSTRUCTOR_ITERATOR, ATEXIT]);
        assert_eq!(SHAPE_C.len(), 6);
        for &(f, array, size, count, constructor, destructor, exit_fn) in SHAPE_C {
            assert_eq!(
                run(&mut e, f),
                vec![
                    (
                        EH_VECTOR_CONSTRUCTOR_ITERATOR,
                        vec![array, size, count, constructor, destructor]
                    ),
                    (ATEXIT, vec![exit_fn])
                ],
                "{f:08x}"
            );
        }
    }

    #[test]
    fn shape_d_passes_the_word_then_a_quiet_copy_of_the_float() {
        let mut e = Engine::new();
        quiet(&mut e, &[FN_0045D060, ATEXIT]);
        assert_eq!(SHAPE_D.len(), 6);
        for (bits, passed) in [
            (0x3fc0_0000_u32, 0x3fc0_0000_u32),
            (0x7f80_0001, 0x7fc0_0001),
            (0xff80_0001, 0xffc0_0001),
            (0x7fc0_0000, 0x7fc0_0000),
            (0x8000_0000, 0x8000_0000),
            (0x7f80_0000, 0x7f80_0000),
        ] {
            float_getter(&mut e, bits);
            for &(f, getter_global, object, word, exit_fn) in SHAPE_D {
                assert_eq!(
                    run(&mut e, f),
                    vec![
                        (FLOAT_GETTER_00403E20, vec![getter_global]),
                        (FN_0045D060, vec![object, word, passed]),
                        (ATEXIT, vec![exit_fn])
                    ],
                    "{f:08x} {bits:08x}"
                );
            }
        }
    }

    #[test]
    fn fn_00f38300_combines_the_four_words() {
        const POINTERS: [u32; 4] = [0x0800_0100, 0x0800_0200, 0x0800_0300, 0x0800_0400];
        let mut e = Engine::new();
        quiet(&mut e, &[FN_0045CFE0, ATEXIT]);
        e.map(0x0800_0000, 0x1000);
        for (i, v) in [7_u32, 6, 5, 4].into_iter().enumerate() {
            e.set_global::<u32>(POINTERS[i], v);
        }
        let calls = Rc::new(Cell::new(0_usize));
        let counter = calls.clone();
        // The getter answers the four pointers in turn; its second call changes the first word,
        // which the exe reads only after that call.
        e.register_double(WORD_GETTER_0043D4D0, move |e, _| {
            let n = counter.get();
            counter.set(n + 1);
            if n == 1 {
                e.set_global::<u32>(POINTERS[0], 9);
            }
            Ret {
                eax: POINTERS[n],
                ..Ret::default()
            }
        });
        assert_eq!(
            run(&mut e, 0x00f3_8300),
            vec![
                (WORD_GETTER_0043D4D0, vec![0x011c_63cc]),
                (WORD_GETTER_0043D4D0, vec![0x011c_63cc]),
                (WORD_GETTER_0043D4D0, vec![0x011c_63cc]),
                (WORD_GETTER_0043D4D0, vec![0x011c_63cc]),
                (
                    FN_0045CFE0,
                    vec![0x011c_3c90, 0x0101_7ffc, 9 * 6 + 5 + 4 + 1]
                ),
                (ATEXIT, vec![0x00fc_8090])
            ]
        );
    }

    #[test]
    fn fn_00f38300_wraps_to_32_bits() {
        let mut e = Engine::new();
        quiet(&mut e, &[FN_0045CFE0, ATEXIT]);
        e.map(0x0800_0000, 0x1000);
        e.set_global::<u32>(0x0800_0000, 0x0001_0001);
        e.set_global::<u32>(0x0800_0004, 0xffff_ffff);
        let calls = Rc::new(Cell::new(0_u32));
        let counter = calls.clone();
        // The first two answers point at the first word, the last two at the second.
        e.register_double(WORD_GETTER_0043D4D0, move |_, _| {
            counter.set(counter.get() + 1);
            Ret {
                eax: if counter.get() <= 2 {
                    0x0800_0000
                } else {
                    0x0800_0004
                },
                ..Ret::default()
            }
        });
        let calls = run(&mut e, 0x00f3_8300);
        // 0x10001 * 0x10001 + 0xffff_ffff + 0xffff_ffff + 1, modulo 2^32.
        assert_eq!(
            only(&calls, FN_0045CFE0),
            vec![(FN_0045CFE0, vec![0x011c_3c90, 0x0101_7ffc, 0x0002_0000])]
        );
    }

    #[test]
    fn fn_00f48900_stores_the_low_byte() {
        let mut e = Engine::new();
        e.map(0x011c_8000, 0x1000);
        for addr in 0x011c_8fa0..0x011c_8fa3 {
            e.set_global::<u8>(addr, 0xaa);
        }
        e.register(FN_0050E6C0, |_, _| Ret {
            eax: 0x1234_56ff,
            ..Ret::default()
        });
        assert_eq!(run(&mut e, 0x00f4_8900), vec![(FN_0050E6C0, vec![])]);
        assert_eq!(e.global::<u8>(0x011c_8fa0), 0xaa);
        assert_eq!(e.global::<u8>(0x011c_8fa1), 0xff);
        assert_eq!(e.global::<u8>(0x011c_8fa2), 0xaa);
    }

    #[test]
    fn fn_00f4b250_copies_the_word_the_getter_points_to() {
        let mut e = Engine::new();
        e.map(0x0800_0000, 0x1000);
        e.map(0x011c_9000, 0x1000);
        e.set_global::<u32>(0x0800_0010, 0xcafe_f00d);
        e.register(WORD_GETTER_0043D4D0, |_, _| Ret {
            eax: 0x0800_0010,
            ..Ret::default()
        });
        assert_eq!(
            run(&mut e, 0x00f4_b250),
            vec![(WORD_GETTER_0043D4D0, vec![0x011c_9fb8])]
        );
        assert_eq!(e.global::<u32>(0x011c_9f6c), 0xcafe_f00d);
    }

    #[test]
    fn fn_00f587a0_stores_one_over_the_float() {
        for (bits, expected) in [
            (0x4080_0000_u32, 0x3e80_0000_u32),
            (0x4040_0000, 0x3eaa_aaab),
            (0x3f80_0000, 0x3f80_0000),
            (0x0000_0000, 0x7f80_0000),
            (0x8000_0000, 0xff80_0000),
            (0x7f80_0000, 0x0000_0000),
            (0xff80_0000, 0x8000_0000),
            (0x0000_0001, 0x7f80_0000),
            (0x7f80_0001, 0x7fc0_0001),
            (0xffc0_1234, 0xffc0_1234),
        ] {
            let mut e = Engine::new();
            e.map(0x011c_c000, 0x1000);
            float_getter(&mut e, bits);
            assert_eq!(
                run(&mut e, 0x00f5_87a0),
                vec![(FLOAT_GETTER_00403E20, vec![0x011c_cc40])]
            );
            assert_eq!(e.global::<u32>(0x011c_cc60), expected, "{bits:08x}");
        }
    }

    #[test]
    fn fn_00f5cb40_runs_006444e0_on_four_objects() {
        let mut e = Engine::new();
        quiet(&mut e, &[FN_006444E0]);
        assert_eq!(
            run(&mut e, 0x00f5_cb40),
            vec![
                (
                    FN_006444E0,
                    vec![
                        0x011c_d680,
                        0x011c_d708,
                        0x011c_d58c,
                        0x011c_d36c,
                        0x011c_cf70
                    ]
                ),
                (
                    FN_006444E0,
                    vec![
                        0x011c_d690,
                        0x011c_deb0,
                        0x011c_d99c,
                        0x011c_d9c4,
                        0x011c_db18
                    ]
                ),
                (
                    FN_006444E0,
                    vec![
                        0x011c_d6a0,
                        0x011c_d9a8,
                        0x011c_de44,
                        0x011c_d9c4,
                        0x011c_db18
                    ]
                ),
                (
                    FN_006444E0,
                    vec![
                        0x011c_d6b0,
                        0x011c_d0d8,
                        0x011c_d2a0,
                        0x011c_d9c4,
                        0x011c_db18
                    ]
                ),
            ]
        );
    }

    /// The block table of `00f79710`: 28 globals from `011d51d0` in steps of 4, and the pairs of
    /// the first and last blocks.
    #[test]
    fn fn_00f79710_table_has_28_blocks() {
        assert_eq!(OBJECT_BLOCKS_00F79710.len(), 28);
        for (i, block) in OBJECT_BLOCKS_00F79710.iter().enumerate() {
            assert_eq!(block.0, 0x011d_51d0 + 4 * i as u32);
        }
        assert_eq!(
            OBJECT_BLOCKS_00F79710[0],
            (0x011d_51d0, 0x0106_367c, 0x0104_8458)
        );
        assert_eq!(
            OBJECT_BLOCKS_00F79710[27],
            (0x011d_523c, 0x0106_340c, 0x0106_3418)
        );
    }

    #[test]
    fn fn_00f79710_allocates_and_constructs_each_block() {
        let mut e = Engine::new();
        e.map(0x011d_5000, 0x1000);
        let next = Rc::new(Cell::new(0_u32));
        let counter = next.clone();
        e.register_double(FN_00401000, move |_, _| {
            counter.set(counter.get() + 1);
            Ret {
                eax: 0x0800_0000 + 0x10 * counter.get(),
                ..Ret::default()
            }
        });
        // The constructor answers a pointer one past its `this`.
        e.register(FN_0040C150, |_, a| Ret {
            eax: a[0] + 1,
            ..Ret::default()
        });
        let calls = run(&mut e, 0x00f7_9710);
        assert_eq!(calls.len(), 56);
        for (i, pair) in calls.chunks(2).enumerate() {
            let block = 0x0800_0000 + 0x10 * (i as u32 + 1);
            let (global, first, second) = OBJECT_BLOCKS_00F79710[i];
            assert_eq!(pair[0], (FN_00401000, vec![0xc]));
            assert_eq!(pair[1], (FN_0040C150, vec![block, first, second]));
            assert_eq!(e.global::<u32>(global), block + 1, "block {i}");
        }
        // Spot checks against the disassembly.
        assert_eq!(
            calls[1],
            (FN_0040C150, vec![0x0800_0010, 0x0106_367c, 0x0104_8458])
        );
        assert_eq!(
            calls[55],
            (FN_0040C150, vec![0x0800_01c0, 0x0106_340c, 0x0106_3418])
        );
    }

    #[test]
    fn fn_00f79710_stores_null_when_the_allocation_fails() {
        let mut e = Engine::new();
        e.map(0x011d_5000, 0x1000);
        for i in 0..28 {
            e.set_global::<u32>(0x011d_51d0 + 4 * i, 0xdead_beef);
        }
        e.register(FN_00401000, |_, _| Ret::default());
        e.register(FN_0040C150, |_, _| {
            panic!("constructor ran on a null block")
        });
        let calls = run(&mut e, 0x00f7_9710);
        assert_eq!(calls.len(), 28);
        assert!(calls.iter().all(|c| *c == (FN_00401000, vec![0xc])));
        for i in 0..28 {
            assert_eq!(e.global::<u32>(0x011d_51d0 + 4 * i), 0);
        }
    }

    #[test]
    fn fn_00f79710_mixes_failed_and_successful_allocations() {
        let mut e = Engine::new();
        e.map(0x011d_5000, 0x1000);
        let next = Rc::new(Cell::new(0_u32));
        let counter = next.clone();
        // Every other allocation fails.
        e.register_double(FN_00401000, move |_, _| {
            counter.set(counter.get() + 1);
            Ret {
                eax: if counter.get() & 1 == 0 {
                    0
                } else {
                    0x0800_0000 + counter.get()
                },
                ..Ret::default()
            }
        });
        e.register(FN_0040C150, |_, a| Ret {
            eax: a[0] ^ 0x5000_0000,
            ..Ret::default()
        });
        run(&mut e, 0x00f7_9710);
        for i in 0..28_u32 {
            let expected = if (i + 1) & 1 == 0 {
                0
            } else {
                (0x0800_0000 + i + 1) ^ 0x5000_0000
            };
            assert_eq!(e.global::<u32>(0x011d_51d0 + 4 * i), expected, "block {i}");
        }
    }

    #[test]
    fn fn_00f8fa80_multiplies_the_float_by_the_double() {
        // (float bits, double bits, stored float bits)
        let cases: [(u32, u64, u32); 8] = [
            (0x3fc0_0000, 2.0_f64.to_bits(), 0x4040_0000),
            (0x4040_0000, 0.1_f64.to_bits(), 0x3e99_999a),
            (0x3f80_0000, 0.5_f64.to_bits(), 0x3f00_0000),
            (0x7f80_0001, 2.0_f64.to_bits(), 0x7fc0_0001),
            (0x7f80_0000, 0.0_f64.to_bits(), 0xffc0_0000),
            (0xffc0_0000, 2.0_f64.to_bits(), 0xffc0_0000),
            (0x3fc0_0000, 0x7ff0_0000_0000_0001, 0x7fc0_0000),
            (0x3f80_0000, 1e300_f64.to_bits(), 0x7f80_0000),
        ];
        for (bits, factor, expected) in cases {
            let mut e = Engine::new();
            e.map(0x011d_9000, 0x1000);
            e.map(0x0101_e568, 8);
            e.set_global::<u64>(0x0101_e568, factor);
            float_getter(&mut e, bits);
            assert_eq!(
                run(&mut e, 0x00f8_fa80),
                vec![(FLOAT_GETTER_00403E20, vec![0x011d_9774])]
            );
            assert_eq!(
                e.global::<u32>(0x011d_979c),
                expected,
                "{bits:08x} * {factor:016x}"
            );
        }
    }

    #[test]
    fn fn_00f90990_stores_five_answers_and_a_constant() {
        let mut e = Engine::new();
        e.map(0x011d_a000, 0x1000);
        e.register(FN_00430BC0, |_, a| Ret {
            eax: 0x100 + a[0],
            ..Ret::default()
        });
        assert_eq!(
            run(&mut e, 0x00f9_0990),
            (0..5).map(|i| (FN_00430BC0, vec![i])).collect::<Calls>()
        );
        for i in 0..5_u32 {
            assert_eq!(e.global::<u32>(0x011d_a28c + 4 * i), 0x100 + i);
        }
        assert_eq!(e.global::<u32>(0x011d_a2a0), 0x65);
    }

    #[test]
    fn fn_00f92d50_stores_four_answers_in_turn() {
        let mut e = Engine::new();
        e.map(0x011d_a000, 0x1000);
        let next = Rc::new(Cell::new(0_u32));
        let counter = next.clone();
        e.register_double(FN_00718E90, move |_, _| {
            counter.set(counter.get() + 1);
            Ret {
                eax: 0xaa00_0000 + counter.get(),
                ..Ret::default()
            }
        });
        assert_eq!(
            run(&mut e, 0x00f9_2d50),
            vec![
                (FN_00718E90, vec![0x1a, 0xff, 0x80, 0xff]),
                (FN_00718E90, vec![0x2e, 0xcf, 0xff, 0xff]),
                (FN_00718E90, vec![0xff, 0xb6, 0x42, 0xff]),
                (FN_00718E90, vec![0xc5, 0xff, 0xff, 0xff]),
            ]
        );
        for i in 0..4_u32 {
            assert_eq!(e.global::<u32>(0x011d_ac50 + 4 * i), 0xaa00_0001 + i);
        }
    }
}
