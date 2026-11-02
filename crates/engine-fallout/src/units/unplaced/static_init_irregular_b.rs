//! `(unplaced)/static_init_irregular_b` (no source unit: the compiler's dynamic initializers),
//! subsystem `compiler: static init/exit`: 256 dynamic initializers of FalloutNV.exe 1.4.0.525
//! (the entries `00f9b040` to `00fc7810` of the `_initterm` table `00fdf4a4..010113c0`, the
//! engine map's "compiler: dynamic initializers") that `research/initgen` refused because their
//! bodies match none of the table's shapes (docs/ENGINE_CRATE.md, `research/initgen/README.md`),
//! each translated from its own disassembly. This is part 2 of the list; part 1 is
//! `static_init_irregular.rs`.
//!
//! The CRT runs every initializer once at start-up, before `main`, in table order. They are
//! cdecl functions with no parameters; what they leave in `EAX` is whatever their last call left
//! and `_initterm` ignores it, so they return nothing (`00fb5f40`, which ends in a `JMP` to
//! another function, returns that function's `EAX`). The shapes, with the number of functions:
//!
//! - 120 call `00a2e180` on a global (`MOV ECX,global; CALL 00a2e180`, the one call
//!   `initgen` refuses: its answer is the incoming `ECX`, which `_initterm` leaves from the
//!   previous initializer, and it counts the calls in the word at `011f36a0`).
//! - 16 compute `float / root` and 16 `double / (float * root) * float`, where the root is the
//!   CRT `sqrt` of a float constant, rounded to a float and loaded back (x87 arithmetic).
//! - 34 build a global object with an `FS:[0]` exception frame: its vtable words and fields
//!   stored (`0x01012120`, a vtable, a float, byte or word), then the base vtable `01017720`, the
//!   singleton getter `0044f570`, the singleton's virtual function at vtable slot +4 with the
//!   object, and the object's destructor given to `_atexit`.
//! - 12 write bit-fields into two adjacent global words (read-modify-write with masks).
//! - 4 store a `TlsAlloc` index in a global and give its `TlsFree` destructor to `_atexit`
//!   (Havok's thread-local indexes).
//! - 46 are straight-line code of constructor calls on globals (with stack arguments, an
//!   answer kept, or locals' addresses), stores of vtables, flags and words, and `_atexit`.
//! - single cases: the 16-byte vector table `00fbe090`, the products with a double table
//!   `00fb8d00`, a cosine-based double `00fb6bd0`, six floats then a tail jump `00fb5f40`, a
//!   9-word copy `00fa9220`, a pointer with a dead guarded increment (`00fb85e0`, and `00fb8630`
//!   with a frame and two array constructions), and an empty body `00fb3240`.
//!
//! 35 of them (the `FS:[0]` ones) are marked `// Translation incomplete`: the exception
//! frame (a handler address and the state numbers) is the compiler's unwinding on an exception
//! and is not translated; every call in their bodies is. No function of this file calls the
//! `/GS` cookie check `00ec408c`.
//!
//! Callees are called by address and never re-implemented; imports through their import slot.
//! Float work follows the exe's x87 order with the prelude's helpers.

#[allow(unused_imports)]
use crate::prelude::*;

/// `_atexit` (LIBCMT, cdecl, one argument: the function to run at exit).
const ATEXIT: u32 = 0x00ec_658f;
/// The CRT `sqrt` (`00ec6054`; `units/crt.rs` names it so; Ghidra shows only the address):
/// cdecl, one double on the stack, the root in `ST0`.
const SQRT: u32 = 0x00ec_6054;
/// `eh vector constructor iterator` (`00ec782f`, stdcall `RET 0x14`): array, element size, count,
/// element constructor, destructor for the unwinding.
const EH_VECTOR_CONSTRUCTOR_ITERATOR: u32 = 0x00ec_782f;
/// Import slot of KERNEL32 `InterlockedIncrement` (stdcall, one argument).
const INTERLOCKED_INCREMENT: u32 = 0x00fd_f1c0;
/// The singleton getter `0044f570` (cdecl, no arguments: it makes the singleton kept at
/// [`SINGLETON`] when it is absent).
const SINGLETON_GETTER: u32 = 0x0044_f570;
/// The global holding the singleton the getter makes.
const SINGLETON: u32 = 0x011f_96a0;
/// The base vtable the FS-frame objects store before registering with the singleton.
const BASE_VTABLE: u32 = 0x0101_7720;

/// `FLD double ptr [address]`: the double the x87 stack holds after the load (a signalling NaN
/// quieted: the quiet bit set, the rest kept).
fn load64(e: &Engine, address: u32) -> f64 {
    let bits = e.global::<u64>(address);
    let nan =
        bits & 0x7ff0_0000_0000_0000 == 0x7ff0_0000_0000_0000 && bits & 0x000f_ffff_ffff_ffff != 0;
    f64::from_bits(if nan {
        bits | 0x0008_0000_0000_0000
    } else {
        bits
    })
}

/// The first half of the square-root initializers: `FLD float ptr [address]; FSTP float ptr
/// [EBP-4]; FLD float ptr [EBP-4]; SUB ESP,8; FSTP double ptr [ESP]; CALL sqrt; ADD ESP,8; FSTP
/// float ptr [EBP-0xc]; FLD float ptr [EBP-0xc]; FSTP float ptr [EBP-8]`. The float goes through
/// the x87 stack twice and is pushed as a double; the root comes back in `ST0`, is stored as a
/// float and loaded back. Returns the bits of the local at `EBP-8`.
fn float_sqrt(e: &mut Engine, address: u32) -> u32 {
    let loaded = x87_quiet_bits(e.global::<u32>(address));
    let root = e.call(SQRT, &args![x87_load32(loaded)]).f64();
    x87_quiet_bits(x87_store32(root))
}

/// The tail of the FS-frame objects (frame state 0 onwards): `MOV [global],01017720`; `CALL
/// 0044f570`; `MOV EAX,[011f96a0]`; `PUSH global`; the singleton's virtual function at vtable
/// slot +4 (`RET 4`); `PUSH destructor; CALL _atexit; ADD ESP,4`.
fn register_with_singleton(e: &mut Engine, global: u32, destructor: u32) {
    e.set_global::<u32>(global, BASE_VTABLE);
    e.call(SINGLETON_GETTER, &args![]);
    let singleton = e.global::<u32>(SINGLETON);
    e.vcall(singleton, 4, &args![global]);
    e.call(ATEXIT, &args![destructor]);
}

/// `*global = (*global & keep) | value`: one `MOV r,[global]; AND r,keep; OR r,value; MOV
/// [global],r` of a bit-field write.
fn bits_or(e: &mut Engine, global: u32, keep: u32, value: u32) {
    let old = e.global::<u32>(global);
    e.set_global::<u32>(global, (old & keep) | value);
}

/// `(value & mask) << shift`: the value of a bit-field, as the compiler builds it (`AND r,mask;
/// SHL r,shift`).
fn field(value: u32, mask: u32, shift: u32) -> u32 {
    (value & mask) << shift
}

/// `CMP dword ptr [global],0; JZ`; otherwise `MOV EAX,[global]; ADD EAX,0x158; MOV [EBP-4],EAX;
/// MOV ECX,[EBP-4]; ADD ECX,4; PUSH ECX; CALL InterlockedIncrement`: the count at +4 of the
/// field at +0x158 of the object the pointer in `global` points to.
fn increment_count_of_object_at(e: &mut Engine, global: u32) {
    if e.global::<u32>(global) != 0 {
        let field = e.global::<u32>(global).wrapping_add(0x158);
        e.call(INTERLOCKED_INCREMENT, &args![field.wrapping_add(4)]);
    }
}

/// `FLD float ptr [factor]; FMUL double ptr [multiplier]; FSTP float ptr [global]`.
fn float_times_double(e: &mut Engine, factor: u32, multiplier: u32, global: u32) {
    let product = x87_mul(
        x87_load32(e.global::<u32>(factor)),
        e.global::<f64>(multiplier),
    );
    e.set_global::<u32>(global, x87_store32(product));
}

/// `FLD float ptr [value]; FADD ST0,ST0; FSTP float ptr [global]`.
fn float_doubled(e: &mut Engine, value: u32, global: u32) {
    let loaded = x87_load32(e.global::<u32>(value));
    e.set_global::<u32>(global, x87_store32(x87_add(loaded, loaded)));
}

/// `MOVAPS xmmword ptr [global],XMM0`: four words (the global is 16-byte aligned, as the
/// instruction requires).
fn store_vec(e: &mut Engine, global: u32, words: [u32; 4]) {
    for (i, word) in words.into_iter().enumerate() {
        e.set_global::<u32>(global + 4 * i as u32, word);
    }
}

// Translated from 00f9b040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dc7d8`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9b040(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_c7d8]);
}

// Translated from 00f9b2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dc858`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9b2a0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_c858]);
}

// Translated from 00f9b520 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dc8d0`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9b520(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_c8d0]);
}

// Translated from 00f9b780 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dc948`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9b780(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_c948]);
}

// Translated from 00f9b9e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dc999`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9b9e0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_c999]);
}

// Translated from 00f9b9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00403a70` with `0xf`, `008258a0`, then stores `AL` of its answer in the byte at `011dc998`.
pub fn fn_00f9b9f0(e: &mut Engine) {
    let r0 = e.call(0x0040_3a70, &args![0xf, 0x0082_58a0]).u32();
    e.set_global::<u8>(0x011d_c998, r0 as u8);
}

// Translated from 00f9bc60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dca38`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9bc60(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_ca38]);
}

// Translated from 00f9bf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dcac0`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9bf60(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_cac0]);
}

// Translated from 00f9bf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00403a70` with `0x11`, `00826260`, then stores `AL` of its answer in the byte at `011dcac1`.
pub fn fn_00f9bf70(e: &mut Engine) {
    let r0 = e.call(0x0040_3a70, &args![0x11, 0x0082_6260]).u32();
    e.set_global::<u8>(0x011d_cac1, r0 as u8);
}

// Translated from 00f9c1e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dcb0d`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9c1e0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_cb0d]);
}

// Translated from 00f9c1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00403a70` with `0x18`, `008264d0`, then stores `AL` of its answer in the byte at `011dcb0c`.
pub fn fn_00f9c1f0(e: &mut Engine) {
    let r0 = e.call(0x0040_3a70, &args![0x18, 0x0082_64d0]).u32();
    e.set_global::<u8>(0x011d_cb0c, r0 as u8);
}

// Translated from 00f9c460 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dcb89`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9c460(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_cb89]);
}

// Translated from 00f9c470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00403a70` with `0x19`, `008273c0`, then stores `AL` of its answer in the byte at `011dcb88`.
pub fn fn_00f9c470(e: &mut Engine) {
    let r0 = e.call(0x0040_3a70, &args![0x19, 0x0082_73c0]).u32();
    e.set_global::<u8>(0x011d_cb88, r0 as u8);
}

// Translated from 00f9c530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00403a70` with `0x1`, `00827a40`, then stores `AL` of its answer in the byte at `011dcbcc`.
pub fn fn_00f9c530(e: &mut Engine) {
    let r0 = e.call(0x0040_3a70, &args![1, 0x0082_7a40]).u32();
    e.set_global::<u8>(0x011d_cbcc, r0 as u8);
}

// Translated from 00f9c7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dcc3d`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9c7a0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_cc3d]);
}

// Translated from 00f9c7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00403a70` with `0x5`, `008280a0`, then stores `AL` of its answer in the byte at `011dcc3c`.
pub fn fn_00f9c7b0(e: &mut Engine) {
    let r0 = e.call(0x0040_3a70, &args![5, 0x0082_80a0]).u32();
    e.set_global::<u8>(0x011d_cc3c, r0 as u8);
}

// Translated from 00f9ca20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dcca5`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9ca20(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_cca5]);
}

// Translated from 00f9ca30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00403a70` with `0x13`, `00828760`, then stores `AL` of its answer in the byte at `011dcca4`.
pub fn fn_00f9ca30(e: &mut Engine) {
    let r0 = e.call(0x0040_3a70, &args![0x13, 0x0082_8760]).u32();
    e.set_global::<u8>(0x011d_cca4, r0 as u8);
}

// Translated from 00f9cca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dcd01`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9cca0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_cd01]);
}

// Translated from 00f9ccb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00403a70` with `0x1c`, `0082a0d0`, then stores `AL` of its answer in the byte at `011dcd00`.
pub fn fn_00f9ccb0(e: &mut Engine) {
    let r0 = e.call(0x0040_3a70, &args![0x1c, 0x0082_a0d0]).u32();
    e.set_global::<u8>(0x011d_cd00, r0 as u8);
}

// Translated from 00f9cf20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dcd81`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9cf20(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_cd81]);
}

// Translated from 00f9cf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00403a70` with `0x15`, `0082b110`, then stores `AL` of its answer in the byte at `011dcd80`.
pub fn fn_00f9cf30(e: &mut Engine) {
    let r0 = e.call(0x0040_3a70, &args![0x15, 0x0082_b110]).u32();
    e.set_global::<u8>(0x011d_cd80, r0 as u8);
}

// Translated from 00f9d1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dce20`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9d1a0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_ce20]);
}

// Translated from 00f9d1b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00403a70` with `0x1b`, `0082b6d0`, then stores `AL` of its answer in the byte at `011dce21`.
pub fn fn_00f9d1b0(e: &mut Engine) {
    let r0 = e.call(0x0040_3a70, &args![0x1b, 0x0082_b6d0]).u32();
    e.set_global::<u8>(0x011d_ce21, r0 as u8);
}

// Translated from 00f9d420 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dce51`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9d420(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_ce51]);
}

// Translated from 00f9d430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00403a70` with `0x22`, `0082b8f0`, then stores `AL` of its answer in the byte at `011dce50`.
pub fn fn_00f9d430(e: &mut Engine) {
    let r0 = e.call(0x0040_3a70, &args![0x22, 0x0082_b8f0]).u32();
    e.set_global::<u8>(0x011d_ce50, r0 as u8);
}

// Translated from 00f9d6a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dcee5`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9d6a0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_cee5]);
}

// Translated from 00f9d6b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00403a70` with `0x0`, `0082bc30`, then stores `AL` of its answer in the byte at `011dcee4`.
pub fn fn_00f9d6b0(e: &mut Engine) {
    let r0 = e.call(0x0040_3a70, &args![0, 0x0082_bc30]).u32();
    e.set_global::<u8>(0x011d_cee4, r0 as u8);
}

// Translated from 00f9d920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dcf75`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9d920(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_cf75]);
}

// Translated from 00f9d930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00403a70` with `0x1d`, `0082d090`, then stores `AL` of its answer in the byte at `011dcf74`.
pub fn fn_00f9d930(e: &mut Engine) {
    let r0 = e.call(0x0040_3a70, &args![0x1d, 0x0082_d090]).u32();
    e.set_global::<u8>(0x011d_cf74, r0 as u8);
}

// Translated from 00f9db40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dd0cc`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9db40(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_d0cc]);
}

// Translated from 00f9e0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dd404`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9e0a0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_d404]);
}

// Translated from 00f9e0e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `CreateEventA` (import slot `00fdf0cc`, with `0x0`, `0x0`, `0x0`, `0x0`), then stores `EAX` of its answer in the word at `011dd3b0`.
pub fn fn_00f9e0e0(e: &mut Engine) {
    let r0 = e.call(0x00fd_f0cc, &args![0, 0, 0, 0]).u32();
    e.set_global::<u32>(0x011d_d3b0, r0);
}

// Translated from 00f9e350 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dd620`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9e350(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_d620]);
}

// Translated from 00f9e730 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011ddd00`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9e730(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_dd00]);
}

// Translated from 00f9ec20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00ec782f` (`eh vector constructor iterator`) with `011dd9e8`, `0x24`, `0x14`, `0083b770`, `0083b7d0`, then registers `00fd8c20` with `_atexit` (`00ec658f`).
pub fn fn_00f9ec20(e: &mut Engine) {
    e.call(
        0x00ec_782f,
        &args![0x011d_d9e8, 0x24, 0x14, 0x0083_b770, 0x0083_b7d0],
    );
    e.call(0x00ec_658f, &args![0x00fd_8c20]);
}

// Translated from 00f9efa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dddc0`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9efa0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_ddc0]);
}

// Translated from 00f9f240 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dde44`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9f240(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_de44]);
}

// Translated from 00f9f5a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011ddee8`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9f5a0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_dee8]);
}

// Translated from 00f9fa20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011ddf98`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9fa20(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_df98]);
}

// Translated from 00f9fc80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011de010`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9fc80(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_e010]);
}

// Translated from 00f9fee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011de088`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00f9fee0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_e088]);
}

// Translated from 00fa01b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011de114`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa01b0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_e114]);
}

// Translated from 00fa0410 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011de23d`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa0410(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_e23d]);
}

// Translated from 00fa09e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011de440`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa09e0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_e440]);
}

// Translated from 00fa0c40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011de4dc`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa0c40(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_e4dc]);
}

// Translated from 00fa0ef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011de564`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa0ef0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_e564]);
}

// Translated from 00fa11f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011de5ec`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa11f0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_e5ec]);
}

// Translated from 00fa1450 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011de664`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa1450(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_e664]);
}

// Translated from 00fa1790 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011de6f4`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa1790(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_e6f4]);
}

// Translated from 00fa1b20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011de7ac`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa1b20(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_e7ac]);
}

// Translated from 00fa1d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011de940`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa1d40(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_e940]);
}

// Translated from 00fa2260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dee18`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa2260(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_ee18]);
}

// Translated from 00fa3610 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011df050`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa3610(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_f050]);
}

// Translated from 00fa38c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011df0d0`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa38c0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_f0d0]);
}

// Translated from 00fa3b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011df160`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa3b40(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_f160]);
}

// Translated from 00fa3f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011df23c`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa3f30(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_f23c]);
}

// Translated from 00fa4290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011df71e`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa4290(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_f71e]);
}

// Translated from 00fa4860 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `00401050` (`vector constructor iterator`) with `011df7a0`, `0xc`, `0x5`, `006815c0`.
pub fn fn_00fa4860(e: &mut Engine) {
    e.call(0x0040_1050, &args![0x011d_f7a0, 0xc, 5, 0x0068_15c0]);
}

// Translated from 00fa4ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011df8e4`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa4ad0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_f8e4]);
}

// Translated from 00fa4d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011df968`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa4d30(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_f968]);
}

// Translated from 00fa4fc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011df9e0`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa4fc0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_f9e0]);
}

// Translated from 00fa52e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dfa1a`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa52e0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_fa1a]);
}

// Translated from 00fa5640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dfc7c`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa5640(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_fc7c]);
}

// Translated from 00fa59c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dfcfc`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa59c0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_fcfc]);
}

// Translated from 00fa5bc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dfd74`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa5bc0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_fd74]);
}

// Translated from 00fa5e80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dfdec`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa5e80(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_fdec]);
}

// Translated from 00fa6240 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dfeb4`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa6240(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_feb4]);
}

// Translated from 00fa6560 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dff4c`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa6560(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_ff4c]);
}

// Translated from 00fa6800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011dffcc`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa6800(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011d_ffcc]);
}

// Translated from 00fa6a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011e0048`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa6a60(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011e_0048]);
}

// Translated from 00fa6cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011e00c8`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa6cc0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011e_00c8]);
}

// Translated from 00fa7020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011e01bc`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa7020(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011e_01bc]);
}

// Translated from 00fa7220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011e0247`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa7220(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011e_0247]);
}

// Translated from 00fa76e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011e0334`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa76e0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011e_0334]);
}

// Translated from 00fa7940 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011e0362`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa7940(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011e_0362]);
}

// Translated from 00fa7c60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011e0461`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa7c60(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011e_0461]);
}

// Translated from 00fa7fb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011e0529`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa7fb0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011e_0529]);
}

// Translated from 00fa8290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011e0551`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa8290(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011e_0551]);
}

// Translated from 00fa8670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011e0b34`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa8670(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011e_0b34]);
}

// Translated from 00fa9220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the 9 words at `011e09ec`: copies them from `011a9448` (`REP MOVSD` with `ECX` = 9, `ESI` the source, `EDI` the destination, forward).
pub fn fn_00fa9220(e: &mut Engine) {
    let (source, destination) = (0x011a_9448, 0x011e_09ec);
    for i in 0..9 {
        let word = e.global::<u32>(source + 4 * i);
        e.set_global::<u32>(destination + 4 * i, word);
    }
}

// Translated from 00fa94d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011e0dec`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa94d0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011e_0dec]);
}

// Translated from 00fa9730 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f12c0`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa9730(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_12c0]);
}

// Translated from 00fa9b00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f133c`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa9b00(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_133c]);
}

// Translated from 00fa9d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1389`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa9d70(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1389]);
}

// Translated from 00fa9fd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f14ec`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fa9fd0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_14ec]);
}

// Translated from 00faa230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1570`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00faa230(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1570]);
}

// Translated from 00faa4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f15cd`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00faa4f0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_15cd]);
}

// Translated from 00faa8f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f16ec`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00faa8f0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_16ec]);
}

// Translated from 00faac40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1784`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00faac40(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1784]);
}

// Translated from 00fab030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1898`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fab030(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1898]);
}

// Translated from 00fab380 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f193c`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fab380(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_193c]);
}

// Translated from 00fab670 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f19d0`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fab670(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_19d0]);
}

// Translated from 00fabba0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1a62`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fabba0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1a62]);
}

// Translated from 00fabf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1b54`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fabf80(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1b54]);
}

// Translated from 00fac210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1bd8`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fac210(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1bd8]);
}

// Translated from 00fac4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1c7d`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fac4e0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1c7d]);
}

// Translated from 00fac850 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1d20`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fac850(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1d20]);
}

// Translated from 00facb20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1d6d`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00facb20(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1d6d]);
}

// Translated from 00facd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1e20`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00facd80(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1e20]);
}

// Translated from 00facfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1e98`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00facfe0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1e98]);
}

// Translated from 00fad240 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1f10`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fad240(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1f10]);
}

// Translated from 00fad4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f1f88`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fad4a0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_1f88]);
}

// Translated from 00fad700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2000`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fad700(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2000]);
}

// Translated from 00fad960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2088`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fad960(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2088]);
}

// Translated from 00fadb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f20a5`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fadb60(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_20a5]);
}

// Translated from 00fadeb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f219c`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fadeb0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_219c]);
}

// Translated from 00fae110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f22f8`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fae110(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_22f8]);
}

// Translated from 00fae510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2384`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fae510(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2384]);
}

// Translated from 00fae7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f23fc`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fae7d0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_23fc]);
}

// Translated from 00faea30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2474`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00faea30(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2474]);
}

// Translated from 00faec30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f24e5`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00faec30(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_24e5]);
}

// Translated from 00faef10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f257c`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00faef10(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_257c]);
}

// Translated from 00faf230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2600`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00faf230(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2600]);
}

// Translated from 00faf4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2684`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00faf4f0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2684]);
}

// Translated from 00faf810 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2708`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00faf810(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2708]);
}

// Translated from 00fafa70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f278c`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fafa70(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_278c]);
}

// Translated from 00fafd90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2810`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fafd90(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2810]);
}

// Translated from 00fb0050 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2894`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb0050(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2894]);
}

// Translated from 00fb0250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f290c`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb0250(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_290c]);
}

// Translated from 00fb04b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2984`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb04b0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2984]);
}

// Translated from 00fb07d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2a08`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb07d0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2a08]);
}

// Translated from 00fb0a30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2a80`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb0a30(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2a80]);
}

// Translated from 00fb0c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2bf4`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb0c90(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2bf4]);
}

// Translated from 00fb1330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2c88`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb1330(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2c88]);
}

// Translated from 00fb1590 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2d00`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb1590(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2d00]);
}

// Translated from 00fb17f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2d84`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb17f0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2d84]);
}

// Translated from 00fb1a50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2e08`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb1a50(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2e08]);
}

// Translated from 00fb1d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2e80`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb1d10(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2e80]);
}

// Translated from 00fb1f70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2ef8`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb1f70(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2ef8]);
}

// Translated from 00fb21d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f2f88`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb21d0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_2f88]);
}

// Translated from 00fb2490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f3000`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb2490(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_3000]);
}

// Translated from 00fb26f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f3078`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb26f0(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_3078]);
}

// Translated from 00fb2950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f30fc`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb2950(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_30fc]);
}

// Translated from 00fb2c10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f3180`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb2c10(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_3180]);
}

// Translated from 00fb2e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f3204`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb2e70(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_3204]);
}

// Translated from 00fb3160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the global at `011f327c`: runs `00a2e180` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fb3160(e: &mut Engine) {
    e.call(0x00a2_e180, &args![0x011f_327c]);
}

// Translated from 00fb31b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f32e8`: the float at `01093f3c` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `01093f40` divided by it (`FDIV`, x87) is stored at `011f32e8`.
pub fn fn_00fb31b0(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_3f3c);
    let numerator = x87_load32(e.global::<u32>(0x0109_3f40));
    e.set_global::<u32>(0x011f_32e8, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb31f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f32ec`: the float at `01093f3c` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `01093f3c` times it is divided into the double at `01011598` (`FDIVR`), times the float at `01093f40` (`FMUL`), and stored at `011f32ec` (all x87).
pub fn fn_00fb31f0(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_3f3c);
    let product = x87_mul(x87_load32(e.global::<u32>(0x0109_3f3c)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x0109_3f40)));
    e.set_global::<u32>(0x011f_32ec, x87_store32(scaled));
}

// Translated from 00fb3240 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer with an empty body (`PUSH EBP; MOV EBP,ESP; POP EBP; RET`): nothing happens.
pub fn fn_00fb3240(_e: &mut Engine) {}

// Translated from 00fb3250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: runs `00a0d5f0` on `011f32f4` (`ECX`; stack arguments `00000101`), then stores `01094dfc` in the word at `011f32f4`, then stores `01094ddc` in the word at `011f32f4`, then stores `0x1` in the byte at `011f3304`, then stores `01094dbc` in the word at `011f32f4`, then stores `01094d9c` in the word at `011f32f4`, then registers `00fda000` with `_atexit` (`00ec658f`).
pub fn fn_00fb3250(e: &mut Engine) {
    e.call(0x00a0_d5f0, &args![0x011f_32f4, 0x101]);
    e.set_global::<u32>(0x011f_32f4, 0x0109_4dfc);
    e.set_global::<u32>(0x011f_32f4, 0x0109_4ddc);
    e.set_global::<u8>(0x011f_3304, 1);
    e.set_global::<u32>(0x011f_32f4, 0x0109_4dbc);
    e.set_global::<u32>(0x011f_32f4, 0x0109_4d9c);
    e.call(0x00ec_658f, &args![0x00fd_a000]);
}

// Translated from 00fb32b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: runs `00a0d710` on `011f3308` (`ECX`; stack arguments `0x25`), then stores `01094e3c` in the word at `011f3308`, then registers `00fda020` with `_atexit` (`00ec658f`).
pub fn fn_00fb32b0(e: &mut Engine) {
    e.call(0x00a0_d710, &args![0x011f_3308, 0x25]);
    e.set_global::<u32>(0x011f_3308, 0x0109_4e3c);
    e.call(0x00ec_658f, &args![0x00fd_a020]);
}

// Translated from 00fb32e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: stores `0x0` in the word at `011f3320`, then stores `0x0` in the word at `011f3318`, then stores `0x0` in the word at `011f331c`, then registers `00fda0a0` with `_atexit` (`00ec658f`).
pub fn fn_00fb32e0(e: &mut Engine) {
    e.set_global::<u32>(0x011f_3320, 0);
    e.set_global::<u32>(0x011f_3318, 0);
    e.set_global::<u32>(0x011f_331c, 0);
    e.call(0x00ec_658f, &args![0x00fd_a0a0]);
}

// Translated from 00fb3310 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: stores `0x0` in the word at `011f332c`, then stores `0x0` in the word at `011f3324`, then stores `0x0` in the word at `011f3328`, then registers `00fda0f0` with `_atexit` (`00ec658f`).
pub fn fn_00fb3310(e: &mut Engine) {
    e.set_global::<u32>(0x011f_332c, 0);
    e.set_global::<u32>(0x011f_3324, 0);
    e.set_global::<u32>(0x011f_3328, 0);
    e.call(0x00ec_658f, &args![0x00fd_a0f0]);
}

// Translated from 00fb3340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `InitializeCriticalSection` (import slot `00fdf054`, with `011f3330`), then registers `00fda140` with `_atexit` (`00ec658f`).
pub fn fn_00fb3340(e: &mut Engine) {
    e.call(0x00fd_f054, &args![0x011f_3330]);
    e.call(0x00ec_658f, &args![0x00fd_a140]);
}

// Translated from 00fb3360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: stores `0x0` in the word at `011f3348`, then stores `0x0` in the word at `011f334c`, then registers `00fda150` with `_atexit` (`00ec658f`).
pub fn fn_00fb3360(e: &mut Engine) {
    e.set_global::<u32>(0x011f_3348, 0);
    e.set_global::<u32>(0x011f_334c, 0);
    e.call(0x00ec_658f, &args![0x00fd_a150]);
}

// Translated from 00fb3390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: stores `0x0` in the word at `011f3350`, then stores `0x0` in the word at `011f3354`, then registers `00fda170` with `_atexit` (`00ec658f`).
pub fn fn_00fb3390(e: &mut Engine) {
    e.set_global::<u32>(0x011f_3350, 0);
    e.set_global::<u32>(0x011f_3354, 0);
    e.call(0x00ec_658f, &args![0x00fd_a170]);
}

// Translated from 00fb33c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: runs `00a0d8e0` on `011f3358` (`ECX`; stack arguments `0x25`), then stores `01094e7c` in the word at `011f3358`, then registers `00fda190` with `_atexit` (`00ec658f`).
pub fn fn_00fb33c0(e: &mut Engine) {
    e.call(0x00a0_d8e0, &args![0x011f_3358, 0x25]);
    e.set_global::<u32>(0x011f_3358, 0x0109_4e7c);
    e.call(0x00ec_658f, &args![0x00fd_a190]);
}

// Translated from 00fb34d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f33b4`: the float at `01094ef0` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `01094ef4` divided by it (`FDIV`, x87) is stored at `011f33b4`.
pub fn fn_00fb34d0(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_4ef0);
    let numerator = x87_load32(e.global::<u32>(0x0109_4ef4));
    e.set_global::<u32>(0x011f_33b4, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb3510 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f33b8`: the float at `01094ef0` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `01094ef0` times it is divided into the double at `01011598` (`FDIVR`), times the float at `01094ef4` (`FMUL`), and stored at `011f33b8` (all x87).
pub fn fn_00fb3510(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_4ef0);
    let product = x87_mul(x87_load32(e.global::<u32>(0x0109_4ef0)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x0109_4ef4)));
    e.set_global::<u32>(0x011f_33b8, x87_store32(scaled));
}

// Translated from 00fb3700 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `InitializeCriticalSection` (import slot `00fdf054`, with `011f33dc`), then registers `00fda3b0` with `_atexit` (`00ec658f`).
pub fn fn_00fb3700(e: &mut Engine) {
    e.call(0x00fd_f054, &args![0x011f_33dc]);
    e.call(0x00ec_658f, &args![0x00fd_a3b0]);
}

// Translated from 00fb3760 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f340c`: the float at `010950a0` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `010950a4` divided by it (`FDIV`, x87) is stored at `011f340c`.
pub fn fn_00fb3760(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_50a0);
    let numerator = x87_load32(e.global::<u32>(0x0109_50a4));
    e.set_global::<u32>(0x011f_340c, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb37a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f3410`: the float at `010950a0` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `010950a0` times it is divided into the double at `01011598` (`FDIVR`), times the float at `010950a4` (`FMUL`), and stored at `011f3410` (all x87).
pub fn fn_00fb37a0(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_50a0);
    let product = x87_mul(x87_load32(e.global::<u32>(0x0109_50a0)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x0109_50a4)));
    e.set_global::<u32>(0x011f_3410, x87_store32(scaled));
}

// Translated from 00fb3d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f34d8`: the float at `01095474` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `01095478` divided by it (`FDIV`, x87) is stored at `011f34d8`.
pub fn fn_00fb3d30(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_5474);
    let numerator = x87_load32(e.global::<u32>(0x0109_5478));
    e.set_global::<u32>(0x011f_34d8, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb3d70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f34dc`: the float at `01095474` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `01095474` times it is divided into the double at `01011598` (`FDIVR`), times the float at `01095478` (`FMUL`), and stored at `011f34dc` (all x87).
pub fn fn_00fb3d70(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_5474);
    let product = x87_mul(x87_load32(e.global::<u32>(0x0109_5474)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x0109_5478)));
    e.set_global::<u32>(0x011f_34dc, x87_store32(scaled));
}

// Translated from 00fb3dc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: runs `00a1eaa0` on `011f34e0` (`ECX`; stack arguments `0x25`), then stores `010955c4` in the word at `011f34e0`, then stores `010955a4` in the word at `011f34e0`, then stores `0x1` in the byte at `011f34f0`, then stores `01095584` in the word at `011f34e0`, then registers `00fda7c0` with `_atexit` (`00ec658f`).
pub fn fn_00fb3dc0(e: &mut Engine) {
    e.call(0x00a1_eaa0, &args![0x011f_34e0, 0x25]);
    e.set_global::<u32>(0x011f_34e0, 0x0109_55c4);
    e.set_global::<u32>(0x011f_34e0, 0x0109_55a4);
    e.set_global::<u8>(0x011f_34f0, 1);
    e.set_global::<u32>(0x011f_34e0, 0x0109_5584);
    e.call(0x00ec_658f, &args![0x00fd_a7c0]);
}

// Translated from 00fb3e10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: runs `00a1ede0` on `011f34f4` (`ECX`; stack arguments `0x25`), then stores `01095644` in the word at `011f34f4`, then stores `01095624` in the word at `011f34f4`, then stores `0x1` in the byte at `011f3504`, then stores `01095604` in the word at `011f34f4`, then registers `00fda7e0` with `_atexit` (`00ec658f`).
pub fn fn_00fb3e10(e: &mut Engine) {
    e.call(0x00a1_ede0, &args![0x011f_34f4, 0x25]);
    e.set_global::<u32>(0x011f_34f4, 0x0109_5644);
    e.set_global::<u32>(0x011f_34f4, 0x0109_5624);
    e.set_global::<u8>(0x011f_3504, 1);
    e.set_global::<u32>(0x011f_34f4, 0x0109_5604);
    e.call(0x00ec_658f, &args![0x00fd_a7e0]);
}

// Translated from 00fb3e60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: runs `00a1eeb0` on `011f3508` (`ECX`; stack arguments `0x0`, `0x1`), then stores `01095684` in the word at `011f3508`, then registers `00fda800` with `_atexit` (`00ec658f`).
pub fn fn_00fb3e60(e: &mut Engine) {
    e.call(0x00a1_eeb0, &args![0x011f_3508, 0, 1]);
    e.set_global::<u32>(0x011f_3508, 0x0109_5684);
    e.call(0x00ec_658f, &args![0x00fd_a800]);
}

// Translated from 00fb3ed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f3520`: the float at `01095694` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `01095698` divided by it (`FDIV`, x87) is stored at `011f3520`.
pub fn fn_00fb3ed0(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_5694);
    let numerator = x87_load32(e.global::<u32>(0x0109_5698));
    e.set_global::<u32>(0x011f_3520, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb3f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f3524`: the float at `01095694` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `01095694` times it is divided into the double at `01011598` (`FDIVR`), times the float at `01095698` (`FMUL`), and stored at `011f3524` (all x87).
pub fn fn_00fb3f10(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_5694);
    let product = x87_mul(x87_load32(e.global::<u32>(0x0109_5694)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x0109_5698)));
    e.set_global::<u32>(0x011f_3524, x87_store32(scaled));
}

// Translated from 00fb3fa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f3530`: the float at `010956c0` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `010956c4` divided by it (`FDIV`, x87) is stored at `011f3530`.
pub fn fn_00fb3fa0(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_56c0);
    let numerator = x87_load32(e.global::<u32>(0x0109_56c4));
    e.set_global::<u32>(0x011f_3530, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb3fe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f3534`: the float at `010956c0` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `010956c0` times it is divided into the double at `01011598` (`FDIVR`), times the float at `010956c4` (`FMUL`), and stored at `011f3534` (all x87).
pub fn fn_00fb3fe0(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_56c0);
    let product = x87_mul(x87_load32(e.global::<u32>(0x0109_56c0)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x0109_56c4)));
    e.set_global::<u32>(0x011f_3534, x87_store32(scaled));
}

// Translated from 00fb4070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f3544`: the float at `010956cc` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `010956d0` divided by it (`FDIV`, x87) is stored at `011f3544`.
pub fn fn_00fb4070(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_56cc);
    let numerator = x87_load32(e.global::<u32>(0x0109_56d0));
    e.set_global::<u32>(0x011f_3544, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb40b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f3548`: the float at `010956cc` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `010956cc` times it is divided into the double at `01011598` (`FDIVR`), times the float at `010956d0` (`FMUL`), and stored at `011f3548` (all x87).
pub fn fn_00fb40b0(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_56cc);
    let product = x87_mul(x87_load32(e.global::<u32>(0x0109_56cc)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x0109_56d0)));
    e.set_global::<u32>(0x011f_3548, x87_store32(scaled));
}

// Translated from 00fb4140 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f3558`: the float at `01095730` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `01095734` divided by it (`FDIV`, x87) is stored at `011f3558`.
pub fn fn_00fb4140(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_5730);
    let numerator = x87_load32(e.global::<u32>(0x0109_5734));
    e.set_global::<u32>(0x011f_3558, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb4180 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f355c`: the float at `01095730` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `01095730` times it is divided into the double at `01011598` (`FDIVR`), times the float at `01095734` (`FMUL`), and stored at `011f355c` (all x87).
pub fn fn_00fb4180(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_5730);
    let product = x87_mul(x87_load32(e.global::<u32>(0x0109_5730)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x0109_5734)));
    e.set_global::<u32>(0x011f_355c, x87_store32(scaled));
}

// Translated from 00fb41d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `GetTickCount` (import slot `00fdf060`), then runs `00aa50f0` on `011f3560` (`ECX`; stack arguments its answer).
pub fn fn_00fb41d0(e: &mut Engine) {
    let r0 = e.call(0x00fd_f060, &args![]).u32();
    e.call(0x00aa_50f0, &args![0x011f_3560, r0]);
}

// Translated from 00fb4230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f3588`: the float at `01095744` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `01095748` divided by it (`FDIV`, x87) is stored at `011f3588`.
pub fn fn_00fb4230(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_5744);
    let numerator = x87_load32(e.global::<u32>(0x0109_5748));
    e.set_global::<u32>(0x011f_3588, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb4270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f358c`: the float at `01095744` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `01095744` times it is divided into the double at `01011598` (`FDIVR`), times the float at `01095748` (`FMUL`), and stored at `011f358c` (all x87).
pub fn fn_00fb4270(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_5744);
    let product = x87_mul(x87_load32(e.global::<u32>(0x0109_5744)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x0109_5748)));
    e.set_global::<u32>(0x011f_358c, x87_store32(scaled));
}

// Translated from 00fb4300 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f3598`: the float at `010957ac` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `010957b0` divided by it (`FDIV`, x87) is stored at `011f3598`.
pub fn fn_00fb4300(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_57ac);
    let numerator = x87_load32(e.global::<u32>(0x0109_57b0));
    e.set_global::<u32>(0x011f_3598, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb4340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f359c`: the float at `010957ac` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `010957ac` times it is divided into the double at `01011598` (`FDIVR`), times the float at `010957b0` (`FMUL`), and stored at `011f359c` (all x87).
pub fn fn_00fb4340(e: &mut Engine) {
    let root = float_sqrt(e, 0x0109_57ac);
    let product = x87_mul(x87_load32(e.global::<u32>(0x0109_57ac)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x0109_57b0)));
    e.set_global::<u32>(0x011f_359c, x87_store32(scaled));
}

// Translated from 00fb4390 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f35d0`: stores `01012120` in the word at `011f35d0`, stores `0109589c` in the word at `011f35d8`, stores `0x1` in the byte at `011f35d4`, then stores the vtable `01017720` in `011f35d0`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f35d0` (`RET 4`) and registers `00fda830` with `_atexit`.
// Translation incomplete: 00fb4390 - FS:[0] exception frame (handler 00f1e7da) not translated.
pub fn fn_00fb4390(e: &mut Engine) {
    e.set_global::<u32>(0x011f_35d0, 0x0101_2120);
    e.set_global::<u32>(0x011f_35d8, 0x0109_589c);
    e.set_global::<u8>(0x011f_35d4, 1);
    register_with_singleton(e, 0x011f_35d0, 0x00fd_a830);
}

// Translated from 00fb4430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f35dc`: stores `01012120` in the word at `011f35dc`, stores `010958b4` in the word at `011f35e4`, stores `0x1` in the byte at `011f35e0`, then stores the vtable `01017720` in `011f35dc`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f35dc` (`RET 4`) and registers `00fda8b0` with `_atexit`.
// Translation incomplete: 00fb4430 - FS:[0] exception frame (handler 00f1e80a) not translated.
pub fn fn_00fb4430(e: &mut Engine) {
    e.set_global::<u32>(0x011f_35dc, 0x0101_2120);
    e.set_global::<u32>(0x011f_35e4, 0x0109_58b4);
    e.set_global::<u8>(0x011f_35e0, 1);
    register_with_singleton(e, 0x011f_35dc, 0x00fd_a8b0);
}

// Translated from 00fb44d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f35e8`: stores `01012120` in the word at `011f35e8`, stores `010958d4` in the word at `011f35f0`, stores `0x0` in the byte at `011f35ec`, then stores the vtable `01017720` in `011f35e8`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f35e8` (`RET 4`) and registers `00fda930` with `_atexit`.
// Translation incomplete: 00fb44d0 - FS:[0] exception frame (handler 00f1e83a) not translated.
pub fn fn_00fb44d0(e: &mut Engine) {
    e.set_global::<u32>(0x011f_35e8, 0x0101_2120);
    e.set_global::<u32>(0x011f_35f0, 0x0109_58d4);
    e.set_global::<u8>(0x011f_35ec, 0);
    register_with_singleton(e, 0x011f_35e8, 0x00fd_a930);
}

// Translated from 00fb4570 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f35f4`: stores `01012120` in the word at `011f35f4`, stores `010958f0` in the word at `011f35fc`, copies the float at `0101e570` to `011f35f8` (`FLD`/`FSTP`), then stores the vtable `01017720` in `011f35f4`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f35f4` (`RET 4`) and registers `00fda9b0` with `_atexit`.
// Translation incomplete: 00fb4570 - FS:[0] exception frame (handler 00f1e86a) not translated.
pub fn fn_00fb4570(e: &mut Engine) {
    e.set_global::<u32>(0x011f_35f4, 0x0101_2120);
    e.set_global::<u32>(0x011f_35fc, 0x0109_58f0);
    e.set_global::<u32>(0x011f_35f8, x87_quiet_bits(e.global::<u32>(0x0101_e570)));
    register_with_singleton(e, 0x011f_35f4, 0x00fd_a9b0);
}

// Translated from 00fb4610 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f3600`: stores `01012120` in the word at `011f3600`, stores `01095910` in the word at `011f3608`, copies the float at `0101df44` to `011f3604` (`FLD`/`FSTP`), then stores the vtable `01017720` in `011f3600`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f3600` (`RET 4`) and registers `00fdaa30` with `_atexit`.
// Translation incomplete: 00fb4610 - FS:[0] exception frame (handler 00f1e89a) not translated.
pub fn fn_00fb4610(e: &mut Engine) {
    e.set_global::<u32>(0x011f_3600, 0x0101_2120);
    e.set_global::<u32>(0x011f_3608, 0x0109_5910);
    e.set_global::<u32>(0x011f_3604, x87_quiet_bits(e.global::<u32>(0x0101_df44)));
    register_with_singleton(e, 0x011f_3600, 0x00fd_aa30);
}

// Translated from 00fb46b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f360c`: stores `01012120` in the word at `011f360c`, stores `01095930` in the word at `011f3614`, copies the float at `0101b268` to `011f3610` (`FLD`/`FSTP`), then stores the vtable `01017720` in `011f360c`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f360c` (`RET 4`) and registers `00fdaab0` with `_atexit`.
// Translation incomplete: 00fb46b0 - FS:[0] exception frame (handler 00f1e8ca) not translated.
pub fn fn_00fb46b0(e: &mut Engine) {
    e.set_global::<u32>(0x011f_360c, 0x0101_2120);
    e.set_global::<u32>(0x011f_3614, 0x0109_5930);
    e.set_global::<u32>(0x011f_3610, x87_quiet_bits(e.global::<u32>(0x0101_b268)));
    register_with_singleton(e, 0x011f_360c, 0x00fd_aab0);
}

// Translated from 00fb4750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f3618`: stores `01012120` in the word at `011f3618`, stores `01095954` in the word at `011f3620`, copies the float at `0101df44` to `011f361c` (`FLD`/`FSTP`), then stores the vtable `01017720` in `011f3618`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f3618` (`RET 4`) and registers `00fdab30` with `_atexit`.
// Translation incomplete: 00fb4750 - FS:[0] exception frame (handler 00f1e8fa) not translated.
pub fn fn_00fb4750(e: &mut Engine) {
    e.set_global::<u32>(0x011f_3618, 0x0101_2120);
    e.set_global::<u32>(0x011f_3620, 0x0109_5954);
    e.set_global::<u32>(0x011f_361c, x87_quiet_bits(e.global::<u32>(0x0101_df44)));
    register_with_singleton(e, 0x011f_3618, 0x00fd_ab30);
}

// Translated from 00fb47f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f3624`: stores `01012120` in the word at `011f3624`, stores `01095978` in the word at `011f362c`, stores `0x0` in the word at `011f3628`, then stores the vtable `01017720` in `011f3624`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f3624` (`RET 4`) and registers `00fdabb0` with `_atexit`.
// Translation incomplete: 00fb47f0 - FS:[0] exception frame (handler 00f1e92a) not translated.
pub fn fn_00fb47f0(e: &mut Engine) {
    e.set_global::<u32>(0x011f_3624, 0x0101_2120);
    e.set_global::<u32>(0x011f_362c, 0x0109_5978);
    e.set_global::<u32>(0x011f_3628, 0);
    register_with_singleton(e, 0x011f_3624, 0x00fd_abb0);
}

// Translated from 00fb4890 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f3638`: stores `01012120` in the word at `011f3638`, stores `010959a4` in the word at `011f3640`, stores `000927c0` in the word at `011f363c`, then stores the vtable `01017720` in `011f3638`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f3638` (`RET 4`) and registers `00fdac30` with `_atexit`.
// Translation incomplete: 00fb4890 - FS:[0] exception frame (handler 00f1eb5a) not translated.
pub fn fn_00fb4890(e: &mut Engine) {
    e.set_global::<u32>(0x011f_3638, 0x0101_2120);
    e.set_global::<u32>(0x011f_3640, 0x0109_59a4);
    e.set_global::<u32>(0x011f_363c, 0x0009_27c0);
    register_with_singleton(e, 0x011f_3638, 0x00fd_ac30);
}

// Translated from 00fb4930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f3644`: stores `01012120` in the word at `011f3644`, stores `010959c8` in the word at `011f364c`, stores `0x1` in the byte at `011f3648`, then stores the vtable `01017720` in `011f3644`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f3644` (`RET 4`) and registers `00fdacb0` with `_atexit`.
// Translation incomplete: 00fb4930 - FS:[0] exception frame (handler 00f1eb8a) not translated.
pub fn fn_00fb4930(e: &mut Engine) {
    e.set_global::<u32>(0x011f_3644, 0x0101_2120);
    e.set_global::<u32>(0x011f_364c, 0x0109_59c8);
    e.set_global::<u8>(0x011f_3648, 1);
    register_with_singleton(e, 0x011f_3644, 0x00fd_acb0);
}

// Translated from 00fb5f40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the six floats at `011ab598..011ab5b0`: the float at `01016248` goes through `FLD`/`FST`/`FSTP` (quieted if it is a signalling NaN), 1.0 (`FLD1`) and 0.0 (`FLDZ`) through `FST`/`FSTP`; the stores are `011ab598` = 0.0, `011ab59c` = 0.0, `011ab5a4` = 1.0, `011ab5a8` = 1.0, `011ab5ac` and `011ab5b0` = the float; then it jumps (`JMP`) to `00a88cd0` with `ECX` = `011ab598` (thiscall, no stack arguments), so that function's answer is its own.
pub fn fn_00fb5f40(e: &mut Engine) -> u32 {
    let a = x87_quiet_bits(e.global::<u32>(0x0101_6248)); // FLD; FST [ESP+0x10]; FSTP [ESP+0x14]
    let one = 1.0f32.to_bits(); // FLD1; FST [ESP+8]; FSTP [ESP+0xc]
    let zero = 0.0f32.to_bits(); // FLDZ; FST [ESP]; FSTP [ESP+4]
    e.set_global::<u32>(0x011a_b5a4, one);
    e.set_global::<u32>(0x011a_b5b0, a);
    e.set_global::<u32>(0x011a_b59c, zero);
    e.set_global::<u32>(0x011a_b598, zero);
    e.set_global::<u32>(0x011a_b5ac, a);
    e.set_global::<u32>(0x011a_b5a8, one);
    e.call(0x00a8_8cd0, &args![0x011a_b598]).u32()
}

// Translated from 00fb66d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `GetTickCount` (import slot `00fdf060`), then runs `00aa50f0` on `011f6394` (`ECX`; stack arguments its answer).
pub fn fn_00fb66d0(e: &mut Engine) {
    let r0 = e.call(0x00fd_f060, &args![]).u32();
    e.call(0x00aa_50f0, &args![0x011f_6394, r0]);
}

// Translated from 00fb6730 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f6c1c`: the float at `010a2c08` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `010a2c0c` divided by it (`FDIV`, x87) is stored at `011f6c1c`.
pub fn fn_00fb6730(e: &mut Engine) {
    let root = float_sqrt(e, 0x010a_2c08);
    let numerator = x87_load32(e.global::<u32>(0x010a_2c0c));
    e.set_global::<u32>(0x011f_6c1c, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb6770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f6c18`: the float at `010a2c08` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `010a2c08` times it is divided into the double at `01011598` (`FDIVR`), times the float at `010a2c0c` (`FMUL`), and stored at `011f6c18` (all x87).
pub fn fn_00fb6770(e: &mut Engine) {
    let root = float_sqrt(e, 0x010a_2c08);
    let product = x87_mul(x87_load32(e.global::<u32>(0x010a_2c08)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x010a_2c0c)));
    e.set_global::<u32>(0x011f_6c18, x87_store32(scaled));
}

// Translated from 00fb6800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f6c2c`: the float at `010a2c54` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `010a2c58` divided by it (`FDIV`, x87) is stored at `011f6c2c`.
pub fn fn_00fb6800(e: &mut Engine) {
    let root = float_sqrt(e, 0x010a_2c54);
    let numerator = x87_load32(e.global::<u32>(0x010a_2c58));
    e.set_global::<u32>(0x011f_6c2c, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb6840 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f6c28`: the float at `010a2c54` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `010a2c54` times it is divided into the double at `01011598` (`FDIVR`), times the float at `010a2c58` (`FMUL`), and stored at `011f6c28` (all x87).
pub fn fn_00fb6840(e: &mut Engine) {
    let root = float_sqrt(e, 0x010a_2c54);
    let product = x87_mul(x87_load32(e.global::<u32>(0x010a_2c54)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x010a_2c58)));
    e.set_global::<u32>(0x011f_6c28, x87_store32(scaled));
}

// Translated from 00fb68d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f6c3c`: the float at `010a2d1c` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `010a2d20` divided by it (`FDIV`, x87) is stored at `011f6c3c`.
pub fn fn_00fb68d0(e: &mut Engine) {
    let root = float_sqrt(e, 0x010a_2d1c);
    let numerator = x87_load32(e.global::<u32>(0x010a_2d20));
    e.set_global::<u32>(0x011f_6c3c, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb6910 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f6c38`: the float at `010a2d1c` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `010a2d1c` times it is divided into the double at `01011598` (`FDIVR`), times the float at `010a2d20` (`FMUL`), and stored at `011f6c38` (all x87).
pub fn fn_00fb6910(e: &mut Engine) {
    let root = float_sqrt(e, 0x010a_2d1c);
    let product = x87_mul(x87_load32(e.global::<u32>(0x010a_2d1c)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x010a_2d20)));
    e.set_global::<u32>(0x011f_6c38, x87_store32(scaled));
}

// Translated from 00fb69a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f6c4c`: the float at `010a2d34` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `010a2d38` divided by it (`FDIV`, x87) is stored at `011f6c4c`.
pub fn fn_00fb69a0(e: &mut Engine) {
    let root = float_sqrt(e, 0x010a_2d34);
    let numerator = x87_load32(e.global::<u32>(0x010a_2d38));
    e.set_global::<u32>(0x011f_6c4c, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb69e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f6c48`: the float at `010a2d34` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `010a2d34` times it is divided into the double at `01011598` (`FDIVR`), times the float at `010a2d38` (`FMUL`), and stored at `011f6c48` (all x87).
pub fn fn_00fb69e0(e: &mut Engine) {
    let root = float_sqrt(e, 0x010a_2d34);
    let product = x87_mul(x87_load32(e.global::<u32>(0x010a_2d34)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x010a_2d38)));
    e.set_global::<u32>(0x011f_6c48, x87_store32(scaled));
}

// Translated from 00fb6a70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f6c5c`: the float at `010a2d4c` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `010a2d50` divided by it (`FDIV`, x87) is stored at `011f6c5c`.
pub fn fn_00fb6a70(e: &mut Engine) {
    let root = float_sqrt(e, 0x010a_2d4c);
    let numerator = x87_load32(e.global::<u32>(0x010a_2d50));
    e.set_global::<u32>(0x011f_6c5c, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb6ab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f6c58`: the float at `010a2d4c` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `010a2d4c` times it is divided into the double at `01011598` (`FDIVR`), times the float at `010a2d50` (`FMUL`), and stored at `011f6c58` (all x87).
pub fn fn_00fb6ab0(e: &mut Engine) {
    let root = float_sqrt(e, 0x010a_2d4c);
    let product = x87_mul(x87_load32(e.global::<u32>(0x010a_2d4c)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x010a_2d50)));
    e.set_global::<u32>(0x011f_6c58, x87_store32(scaled));
}

// Translated from 00fb6b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f6c70`: the float at `010a2d90` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back, and the float at `010a2d94` divided by it (`FDIV`, x87) is stored at `011f6c70`.
pub fn fn_00fb6b40(e: &mut Engine) {
    let root = float_sqrt(e, 0x010a_2d90);
    let numerator = x87_load32(e.global::<u32>(0x010a_2d94));
    e.set_global::<u32>(0x011f_6c70, x87_store32(x87_div(numerator, x87_m32(root))));
}

// Translated from 00fb6b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the float at `011f6c6c`: the float at `010a2d90` goes through `FLD`/`FSTP` and is pushed as a double to the CRT `sqrt` (`00ec6054`), the root is stored as a float and loaded back; the float at `010a2d90` times it is divided into the double at `01011598` (`FDIVR`), times the float at `010a2d94` (`FMUL`), and stored at `011f6c6c` (all x87).
pub fn fn_00fb6b80(e: &mut Engine) {
    let root = float_sqrt(e, 0x010a_2d90);
    let product = x87_mul(x87_load32(e.global::<u32>(0x010a_2d90)), x87_m32(root));
    let quotient = x87_div(e.global::<f64>(0x0101_1598), product);
    let scaled = x87_mul(quotient, x87_m32(e.global::<u32>(0x010a_2d94)));
    e.set_global::<u32>(0x011f_6c6c, x87_store32(scaled));
}

// Translated from 00fb6bd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the double at `011f6c80`: `FLD` the double at `01011590`, `FMUL` by the double at `010a2f48`, pass the product as a double to `00eca920` (cdecl; Ghidra shows a folded CRT function named `_cos`/`_log`/`_sin`/`_tan`, identity unconfirmed), negate its result (`FCHS`), `FDIV` by the double at `01011590` and store the double at `011f6c80` (all x87).
pub fn fn_00fb6bd0(e: &mut Engine) {
    let product = x87_mul(load64(e, 0x0101_1590), e.global::<f64>(0x010a_2f48));
    let answer = e.call(0x00ec_a920, &args![product]).f64();
    let negated = f64::from_bits(answer.to_bits() ^ (1 << 63)); // FCHS
    let quotient = x87_div(negated, e.global::<f64>(0x0101_1590));
    e.set_global::<f64>(0x011f_6c80, quotient);
}

// Translated from 00fb7080 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: stores `ffffffff` in the word at `011f6df0`, then stores `0x0` in the byte at `011f6df4`, then stores `0x0` in the word at `011f6df8`, then registers `00fdb640` with `_atexit` (`00ec658f`).
pub fn fn_00fb7080(e: &mut Engine) {
    e.set_global::<u32>(0x011f_6df0, 0xffff_ffff);
    e.set_global::<u8>(0x011f_6df4, 0);
    e.set_global::<u32>(0x011f_6df8, 0);
    e.call(0x00ec_658f, &args![0x00fd_b640]);
}

// Translated from 00fb7c90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f7130`: stores `01012120` in the word at `011f7130`, stores `010a41a0` in the word at `011f7138`, stores `0x0` in the byte at `011f7134`, then stores the vtable `01017720` in `011f7130`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f7130` (`RET 4`) and registers `00fdbff0` with `_atexit`.
// Translation incomplete: 00fb7c90 - FS:[0] exception frame (handler 00f22cda) not translated.
pub fn fn_00fb7c90(e: &mut Engine) {
    e.set_global::<u32>(0x011f_7130, 0x0101_2120);
    e.set_global::<u32>(0x011f_7138, 0x010a_41a0);
    e.set_global::<u8>(0x011f_7134, 0);
    register_with_singleton(e, 0x011f_7130, 0x00fd_bff0);
}

// Translated from 00fb7d30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f713c`: stores `01012120` in the word at `011f713c`, stores `010a41c0` in the word at `011f7144`, stores `0x80` in the word at `011f7140`, then stores the vtable `01017720` in `011f713c`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f713c` (`RET 4`) and registers `00fdc070` with `_atexit`.
// Translation incomplete: 00fb7d30 - FS:[0] exception frame (handler 00f22d0a) not translated.
pub fn fn_00fb7d30(e: &mut Engine) {
    e.set_global::<u32>(0x011f_713c, 0x0101_2120);
    e.set_global::<u32>(0x011f_7144, 0x010a_41c0);
    e.set_global::<u32>(0x011f_7140, 0x80);
    register_with_singleton(e, 0x011f_713c, 0x00fd_c070);
}

// Translated from 00fb7dd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f7148`: stores `01012120` in the word at `011f7148`, stores `010a41d0` in the word at `011f7150`, stores `00008000` in the word at `011f714c`, then stores the vtable `01017720` in `011f7148`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f7148` (`RET 4`) and registers `00fdc0f0` with `_atexit`.
// Translation incomplete: 00fb7dd0 - FS:[0] exception frame (handler 00f22d3a) not translated.
pub fn fn_00fb7dd0(e: &mut Engine) {
    e.set_global::<u32>(0x011f_7148, 0x0101_2120);
    e.set_global::<u32>(0x011f_7150, 0x010a_41d0);
    e.set_global::<u32>(0x011f_714c, 0x8000);
    register_with_singleton(e, 0x011f_7148, 0x00fd_c0f0);
}

// Translated from 00fb7e70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f7154`: stores `01012120` in the word at `011f7154`, stores `010a41f4` in the word at `011f715c`, copies the float at `010a41f0` to `011f7158` (`FLD`/`FSTP`), then stores the vtable `01017720` in `011f7154`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f7154` (`RET 4`) and registers `00fdc170` with `_atexit`.
// Translation incomplete: 00fb7e70 - FS:[0] exception frame (handler 00f22d6a) not translated.
pub fn fn_00fb7e70(e: &mut Engine) {
    e.set_global::<u32>(0x011f_7154, 0x0101_2120);
    e.set_global::<u32>(0x011f_715c, 0x010a_41f4);
    e.set_global::<u32>(0x011f_7158, x87_quiet_bits(e.global::<u32>(0x010a_41f0)));
    register_with_singleton(e, 0x011f_7154, 0x00fd_c170);
}

// Translated from 00fb7f10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `011f7160`: stores `01012120` in the word at `011f7160`, stores `010a4210` in the word at `011f7168`, copies the float at `01053114` to `011f7164` (`FLD`/`FSTP`), then stores the vtable `01017720` in `011f7160`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `011f7160` (`RET 4`) and registers `00fdc1f0` with `_atexit`.
// Translation incomplete: 00fb7f10 - FS:[0] exception frame (handler 00f22d9a) not translated.
pub fn fn_00fb7f10(e: &mut Engine) {
    e.set_global::<u32>(0x011f_7160, 0x0101_2120);
    e.set_global::<u32>(0x011f_7168, 0x010a_4210);
    e.set_global::<u32>(0x011f_7164, x87_quiet_bits(e.global::<u32>(0x0105_3114)));
    register_with_singleton(e, 0x011f_7160, 0x00fd_c1f0);
}

// Translated from 00fb8060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the bit-field words at `011f7184` and `011f7188`: the word at `011f7184` becomes `0x0` (its old bits kept under the mask `0x0`); in the word at `011f7188` bits 24..31 get `0x0 & 0xff`, bits 20..23 get `0x0 & 0xf` and bits 0..19 get `0x0 & 0xfffff` (each field a read-modify-write that keeps the other bits).
pub fn fn_00fb8060(e: &mut Engine) {
    let eax = 0;
    bits_or(e, 0x011f_7184, 0, eax);
    bits_or(e, 0x011f_7188, 0x00ff_ffff, field(0, 0xff, 24));
    bits_or(e, 0x011f_7188, 0xff0f_ffff, field(0, 0xf, 20));
    bits_or(e, 0x011f_7188, 0xfff0_0000, field(0, 0x000f_ffff, 0));
}

// Translated from 00fb80d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the bit-field words at `011f718c` and `011f7190`: the word at `011f718c` becomes `0x0` (its old bits kept under the mask `0x0`); in the word at `011f7190` bits 24..31 get `0x1 & 0xff`, bits 20..23 get `0x0 & 0xf` and bits 0..19 get `0x0 & 0xfffff` (each field a read-modify-write that keeps the other bits).
pub fn fn_00fb80d0(e: &mut Engine) {
    let eax = 0;
    bits_or(e, 0x011f_718c, 0, eax);
    bits_or(e, 0x011f_7190, 0x00ff_ffff, field(1, 0xff, 24));
    bits_or(e, 0x011f_7190, 0xff0f_ffff, field(0, 0xf, 20));
    bits_or(e, 0x011f_7190, 0xfff0_0000, field(0, 0x000f_ffff, 0));
}

// Translated from 00fb8140 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the bit-field words at `011f7194` and `011f7198`: the word at `011f7194` becomes `0x1` (its old bits kept under the mask `0x0`); in the word at `011f7198` bits 24..31 get `0x0 & 0xff`, bits 20..23 get `0x0 & 0xf` and bits 0..19 get `0x0 & 0xfffff` (each field a read-modify-write that keeps the other bits).
pub fn fn_00fb8140(e: &mut Engine) {
    let eax = 1;
    bits_or(e, 0x011f_7194, 0, eax);
    bits_or(e, 0x011f_7198, 0x00ff_ffff, field(0, 0xff, 24));
    bits_or(e, 0x011f_7198, 0xff0f_ffff, field(0, 0xf, 20));
    bits_or(e, 0x011f_7198, 0xfff0_0000, field(0, 0x000f_ffff, 0));
}

// Translated from 00fb81b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the bit-field words at `011f719c` and `011f71a0`: the word at `011f719c` becomes `0x2` (its old bits kept under the mask `0x0`); in the word at `011f71a0` bits 24..31 get `0x0 & 0xff`, bits 20..23 get `0x0 & 0xf` and bits 0..19 get `0x0 & 0xfffff` (each field a read-modify-write that keeps the other bits).
pub fn fn_00fb81b0(e: &mut Engine) {
    let eax = 2;
    bits_or(e, 0x011f_719c, 0, eax);
    bits_or(e, 0x011f_71a0, 0x00ff_ffff, field(0, 0xff, 24));
    bits_or(e, 0x011f_71a0, 0xff0f_ffff, field(0, 0xf, 20));
    bits_or(e, 0x011f_71a0, 0xfff0_0000, field(0, 0x000f_ffff, 0));
}

// Translated from 00fb8230 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the bit-field words at `011f7250` and `011f7254`: the word at `011f7250` becomes `0x0` (its old bits kept under the mask `0x0`); in the word at `011f7254` bits 24..31 get `0x0 & 0xff`, bits 20..23 get `0x0 & 0xf` and bits 0..19 get `0x0 & 0xfffff` (each field a read-modify-write that keeps the other bits).
pub fn fn_00fb8230(e: &mut Engine) {
    let eax = 0;
    bits_or(e, 0x011f_7250, 0, eax);
    bits_or(e, 0x011f_7254, 0x00ff_ffff, field(0, 0xff, 24));
    bits_or(e, 0x011f_7254, 0xff0f_ffff, field(0, 0xf, 20));
    bits_or(e, 0x011f_7254, 0xfff0_0000, field(0, 0x000f_ffff, 0));
}

// Translated from 00fb82a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the bit-field words at `011f7258` and `011f725c`: the word at `011f7258` becomes `0x0` (its old bits kept under the mask `0x0`); in the word at `011f725c` bits 24..31 get `0x1 & 0xff`, bits 20..23 get `0x0 & 0xf` and bits 0..19 get `0x0 & 0xfffff` (each field a read-modify-write that keeps the other bits).
pub fn fn_00fb82a0(e: &mut Engine) {
    let eax = 0;
    bits_or(e, 0x011f_7258, 0, eax);
    bits_or(e, 0x011f_725c, 0x00ff_ffff, field(1, 0xff, 24));
    bits_or(e, 0x011f_725c, 0xff0f_ffff, field(0, 0xf, 20));
    bits_or(e, 0x011f_725c, 0xfff0_0000, field(0, 0x000f_ffff, 0));
}

// Translated from 00fb8310 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the bit-field words at `011f7260` and `011f7264`: the word at `011f7260` becomes `0x1` (its old bits kept under the mask `0x0`); in the word at `011f7264` bits 24..31 get `0x0 & 0xff`, bits 20..23 get `0x0 & 0xf` and bits 0..19 get `0x0 & 0xfffff` (each field a read-modify-write that keeps the other bits).
pub fn fn_00fb8310(e: &mut Engine) {
    let eax = 1;
    bits_or(e, 0x011f_7260, 0, eax);
    bits_or(e, 0x011f_7264, 0x00ff_ffff, field(0, 0xff, 24));
    bits_or(e, 0x011f_7264, 0xff0f_ffff, field(0, 0xf, 20));
    bits_or(e, 0x011f_7264, 0xfff0_0000, field(0, 0x000f_ffff, 0));
}

// Translated from 00fb8380 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the bit-field words at `011f7268` and `011f726c`: the word at `011f7268` becomes `0x2` (its old bits kept under the mask `0x0`); in the word at `011f726c` bits 24..31 get `0x0 & 0xff`, bits 20..23 get `0x0 & 0xf` and bits 0..19 get `0x0 & 0xfffff` (each field a read-modify-write that keeps the other bits).
pub fn fn_00fb8380(e: &mut Engine) {
    let eax = 2;
    bits_or(e, 0x011f_7268, 0, eax);
    bits_or(e, 0x011f_726c, 0x00ff_ffff, field(0, 0xff, 24));
    bits_or(e, 0x011f_726c, 0xff0f_ffff, field(0, 0xf, 20));
    bits_or(e, 0x011f_726c, 0xfff0_0000, field(0, 0x000f_ffff, 0));
}

// Translated from 00fb8400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the bit-field words at `011f727c` and `011f7280`: the word at `011f727c` becomes `0x0` (its old bits kept under the mask `0x0`); in the word at `011f7280` bits 24..31 get `0x0 & 0xff`, bits 20..23 get `0x0 & 0xf` and bits 0..19 get `0x0 & 0xfffff` (each field a read-modify-write that keeps the other bits).
pub fn fn_00fb8400(e: &mut Engine) {
    let eax = 0;
    bits_or(e, 0x011f_727c, 0, eax);
    bits_or(e, 0x011f_7280, 0x00ff_ffff, field(0, 0xff, 24));
    bits_or(e, 0x011f_7280, 0xff0f_ffff, field(0, 0xf, 20));
    bits_or(e, 0x011f_7280, 0xfff0_0000, field(0, 0x000f_ffff, 0));
}

// Translated from 00fb8470 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the bit-field words at `011f7284` and `011f7288`: the word at `011f7284` becomes `0x0` (its old bits kept under the mask `0x0`); in the word at `011f7288` bits 24..31 get `0x1 & 0xff`, bits 20..23 get `0x0 & 0xf` and bits 0..19 get `0x0 & 0xfffff` (each field a read-modify-write that keeps the other bits).
pub fn fn_00fb8470(e: &mut Engine) {
    let eax = 0;
    bits_or(e, 0x011f_7284, 0, eax);
    bits_or(e, 0x011f_7288, 0x00ff_ffff, field(1, 0xff, 24));
    bits_or(e, 0x011f_7288, 0xff0f_ffff, field(0, 0xf, 20));
    bits_or(e, 0x011f_7288, 0xfff0_0000, field(0, 0x000f_ffff, 0));
}

// Translated from 00fb84e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the bit-field words at `011f728c` and `011f7290`: the word at `011f728c` becomes `0x1` (its old bits kept under the mask `0x0`); in the word at `011f7290` bits 24..31 get `0x0 & 0xff`, bits 20..23 get `0x0 & 0xf` and bits 0..19 get `0x0 & 0xfffff` (each field a read-modify-write that keeps the other bits).
pub fn fn_00fb84e0(e: &mut Engine) {
    let eax = 1;
    bits_or(e, 0x011f_728c, 0, eax);
    bits_or(e, 0x011f_7290, 0x00ff_ffff, field(0, 0xff, 24));
    bits_or(e, 0x011f_7290, 0xff0f_ffff, field(0, 0xf, 20));
    bits_or(e, 0x011f_7290, 0xfff0_0000, field(0, 0x000f_ffff, 0));
}

// Translated from 00fb8550 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the bit-field words at `011f7294` and `011f7298`: the word at `011f7294` becomes `0x2` (its old bits kept under the mask `0x0`); in the word at `011f7298` bits 24..31 get `0x0 & 0xff`, bits 20..23 get `0x0 & 0xf` and bits 0..19 get `0x0 & 0xfffff` (each field a read-modify-write that keeps the other bits).
pub fn fn_00fb8550(e: &mut Engine) {
    let eax = 2;
    bits_or(e, 0x011f_7294, 0, eax);
    bits_or(e, 0x011f_7298, 0x00ff_ffff, field(0, 0xff, 24));
    bits_or(e, 0x011f_7298, 0xff0f_ffff, field(0, 0xf, 20));
    bits_or(e, 0x011f_7298, 0xfff0_0000, field(0, 0x000f_ffff, 0));
}

// Translated from 00fb85c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `InitializeCriticalSection` (import slot `00fdf054`, with `011f8170`), then registers `00fdc370` with `_atexit` (`00ec658f`).
pub fn fn_00fb85c0(e: &mut Engine) {
    e.call(0x00fd_f054, &args![0x011f_8170]);
    e.call(0x00ec_658f, &args![0x00fd_c370]);
}

// Translated from 00fb85e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the pointer at `011f8188`: stores 0 in it; if the word is not 0 (the test follows the store, so the block is dead, translated anyway) `InterlockedIncrement` (import slot `00fdf1c0`) of the count at +4 of the field at +0x158 of the pointed-to object; registers `00fdc380` with `_atexit`.
pub fn fn_00fb85e0(e: &mut Engine) {
    e.set_global::<u32>(0x011f_8188, 0);
    increment_count_of_object_at(e, 0x011f_8188);
    e.call(ATEXIT, &args![0x00fd_c380]);
}

// Translated from 00fb8630 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the pointer at `011f8190` and two arrays: stores 0 in the pointer; if the word is not 0 (the test follows the store, so the block is dead, translated anyway) `InterlockedIncrement` (import slot `00fdf1c0`) of the count at +4 of the field at +0x158 of the pointed-to object; `eh vector constructor iterator` (`00ec782f`) over 8 elements of 4 bytes at `011f8194` and over 9 at `011f81b4`, constructor `00afd230`, destructor `00afc720` (frame states 0 and 1); registers `00fdc3c0` with `_atexit`.
// Translation incomplete: 00fb8630 - FS:[0] exception frame (handler 00f236ee) not translated.
pub fn fn_00fb8630(e: &mut Engine) {
    e.set_global::<u32>(0x011f_8190, 0);
    increment_count_of_object_at(e, 0x011f_8190);
    // state 0 of the frame
    e.call(
        EH_VECTOR_CONSTRUCTOR_ITERATOR,
        &args![0x011f_8194, 4, 8, 0x00af_d230, 0x00af_c720],
    );
    // state 1
    e.call(
        EH_VECTOR_CONSTRUCTOR_ITERATOR,
        &args![0x011f_81b4, 4, 9, 0x00af_d230, 0x00af_c720],
    );
    // state -1
    e.call(ATEXIT, &args![0x00fd_c3c0]);
}

// Translated from 00fb8740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: runs `00b0dce0` on `011f8bdc` (`ECX`; no stack arguments), then registers `00fdc410` with `_atexit` (`00ec658f`).
pub fn fn_00fb8740(e: &mut Engine) {
    e.call(0x00b0_dce0, &args![0x011f_8bdc]);
    e.call(0x00ec_658f, &args![0x00fd_c410]);
}

// Translated from 00fb8770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `InitializeCriticalSection` (import slot `00fdf054`, with `011f8bc4`), then registers `00fdc480` with `_atexit` (`00ec658f`).
pub fn fn_00fb8770(e: &mut Engine) {
    e.call(0x00fd_f054, &args![0x011f_8bc4]);
    e.call(0x00ec_658f, &args![0x00fd_c480]);
}

// Translated from 00fb8790 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: stores the address of the local at `EBP-9` in the local at `EBP-8` (a dead store: nothing reads it), then runs `0044a730` on `011f8ba4` (`ECX`; stack arguments `0x0`, `0x0`), then registers `00fdc490` with `_atexit` (`00ec658f`).
pub fn fn_00fb8790(e: &mut Engine) {
    e.with_stack(0x20, |e, p| {
        let ebp = p.addr() + 0x20;
        e.mem.set_u32(ebp - 8, ebp - 9);
        e.call(0x0044_a730, &args![0x011f_8ba4, 0, 0]);
        e.call(0x00ec_658f, &args![0x00fd_c490]);
    });
}

// Translated from 00fb87c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: stores the address of the local at `EBP-9` in the local at `EBP-8` (a dead store: nothing reads it), then runs `0044a730` on `011f8b84` (`ECX`; stack arguments `0x0`, `0x0`), then registers `00fdc4b0` with `_atexit` (`00ec658f`).
pub fn fn_00fb87c0(e: &mut Engine) {
    e.with_stack(0x20, |e, p| {
        let ebp = p.addr() + 0x20;
        e.mem.set_u32(ebp - 8, ebp - 9);
        e.call(0x0044_a730, &args![0x011f_8b84, 0, 0]);
        e.call(0x00ec_658f, &args![0x00fd_c4b0]);
    });
}

// Translated from 00fb88c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: runs `00b17db0` on `011f8c68` (`ECX`; no stack arguments), then registers `00fdc4d0` with `_atexit` (`00ec658f`).
pub fn fn_00fb88c0(e: &mut Engine) {
    e.call(0x00b1_7db0, &args![0x011f_8c68]);
    e.call(0x00ec_658f, &args![0x00fd_c4d0]);
}

// Translated from 00fb88f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: runs `00b17db0` on `011f8c18` (`ECX`; no stack arguments), then registers `00fdc530` with `_atexit` (`00ec658f`).
pub fn fn_00fb88f0(e: &mut Engine) {
    e.call(0x00b1_7db0, &args![0x011f_8c18]);
    e.call(0x00ec_658f, &args![0x00fd_c530]);
}

// Translated from 00fb8920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: runs `00b17db0` on `011f8c50` (`ECX`; no stack arguments), then registers `00fdc590` with `_atexit` (`00ec658f`).
pub fn fn_00fb8920(e: &mut Engine) {
    e.call(0x00b1_7db0, &args![0x011f_8c50]);
    e.call(0x00ec_658f, &args![0x00fd_c590]);
}

// Translated from 00fb8950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: runs `00b17db0` on `011f8c80` (`ECX`; no stack arguments), then registers `00fdc5f0` with `_atexit` (`00ec658f`).
pub fn fn_00fb8950(e: &mut Engine) {
    e.call(0x00b1_7db0, &args![0x011f_8c80]);
    e.call(0x00ec_658f, &args![0x00fd_c5f0]);
}

// Translated from 00fb8cc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: stores `0x0` in the halfword at `011f8d84`, then stores `0x0` in the word at `011f8d88`, then runs `00b07f20` on `011f8d84` (`ECX`; stack arguments `0x4`), then registers `00fdc660` with `_atexit` (`00ec658f`).
pub fn fn_00fb8cc0(e: &mut Engine) {
    e.set_global::<u16>(0x011f_8d84, 0);
    e.set_global::<u32>(0x011f_8d88, 0);
    e.call(0x00b0_7f20, &args![0x011f_8d84, 4]);
    e.call(0x00ec_658f, &args![0x00fd_c660]);
}

// Translated from 00fb8d00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the 30 floats at `011f8d98..011f8e0c`: each is the float at `010a578c` (`FLD`) times a double constant from the executable's data (`FMUL double ptr`), stored as a float (`FSTP`), except `011f8da8`, which is the float doubled (`FADD ST0,ST0`); x87 arithmetic.
pub fn fn_00fb8d00(e: &mut Engine) {
    float_times_double(e, 0x010a_578c, 0x0103_4208, 0x011f_8d98);
    float_times_double(e, 0x010a_578c, 0x0106_5770, 0x011f_8d9c);
    float_times_double(e, 0x010a_578c, 0x0102_40b8, 0x011f_8da0);
    float_times_double(e, 0x010a_578c, 0x010a_57d0, 0x011f_8da4);
    float_doubled(e, 0x010a_578c, 0x011f_8da8);
    float_times_double(e, 0x010a_578c, 0x010a_57c8, 0x011f_8dac);
    float_times_double(e, 0x010a_578c, 0x0107_2fd0, 0x011f_8db0);
    float_times_double(e, 0x010a_578c, 0x0101_2060, 0x011f_8db4);
    float_times_double(e, 0x010a_578c, 0x010a_57c0, 0x011f_8db8);
    float_times_double(e, 0x010a_578c, 0x0102_0758, 0x011f_8dbc);
    float_times_double(e, 0x010a_578c, 0x010a_57b8, 0x011f_8dc0);
    float_times_double(e, 0x010a_578c, 0x0102_40b0, 0x011f_8dc4);
    float_times_double(e, 0x010a_578c, 0x0101_db80, 0x011f_8dc8);
    float_times_double(e, 0x010a_578c, 0x0106_d4c8, 0x011f_8dcc);
    float_times_double(e, 0x010a_578c, 0x0103_5700, 0x011f_8dd0);
    float_times_double(e, 0x010a_578c, 0x010a_57b0, 0x011f_8dd4);
    float_times_double(e, 0x010a_578c, 0x0103_5700, 0x011f_8dd8);
    float_times_double(e, 0x010a_578c, 0x010a_57a8, 0x011f_8ddc);
    float_times_double(e, 0x010a_578c, 0x0101_db80, 0x011f_8de0);
    float_times_double(e, 0x010a_578c, 0x010a_57c8, 0x011f_8de4);
    float_times_double(e, 0x010a_578c, 0x0103_5708, 0x011f_8de8);
    float_times_double(e, 0x010a_578c, 0x0102_e438, 0x011f_8dec);
    float_times_double(e, 0x010a_578c, 0x0103_56f8, 0x011f_8df0);
    float_times_double(e, 0x010a_578c, 0x010a_57a0, 0x011f_8df4);
    float_times_double(e, 0x010a_578c, 0x0102_1928, 0x011f_8df8);
    float_times_double(e, 0x010a_578c, 0x010a_5798, 0x011f_8dfc);
    float_times_double(e, 0x010a_578c, 0x010a_5790, 0x011f_8e00);
    float_times_double(e, 0x010a_578c, 0x0101_5a40, 0x011f_8e04);
    float_times_double(e, 0x010a_578c, 0x0107_2f80, 0x011f_8e08);
    float_times_double(e, 0x010a_578c, 0x0101_5a38, 0x011f_8e0c);
}

// Translated from 00fb8f80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: runs `00b3b770` on `011f8e2c` (`ECX`; stack arguments the address of a local, the address of a local), then registers `00fdc690` with `_atexit` (`00ec658f`).
pub fn fn_00fb8f80(e: &mut Engine) {
    e.with_stack(0x2c, |e, p| {
        let ebp = p.addr() + 0x2c;
        e.call(0x00b3_b770, &args![0x011f_8e2c, ebp - 2, ebp - 1]);
        e.call(0x00ec_658f, &args![0x00fd_c690]);
    });
}

// Translated from 00fb92a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: stores `010a58c8` in the word at `011f8ebc`, then stores `010a58a0` in the word at `011f8ebc`, then registers `00fdc720` with `_atexit` (`00ec658f`).
pub fn fn_00fb92a0(e: &mut Engine) {
    e.set_global::<u32>(0x011f_8ebc, 0x010a_58c8);
    e.set_global::<u32>(0x011f_8ebc, 0x010a_58a0);
    e.call(0x00ec_658f, &args![0x00fd_c720]);
}

// Translated from 00fbb7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `InitializeCriticalSection` (import slot `00fdf054`, with `01202d78`), then registers `00fdd430` with `_atexit` (`00ec658f`).
pub fn fn_00fbb7c0(e: &mut Engine) {
    e.call(0x00fd_f054, &args![0x0120_2d78]);
    e.call(0x00ec_658f, &args![0x00fd_d430]);
}

// Translated from 00fbb7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `01202da8`: stores `01012120` in the word at `01202da8`, stores `010c1774` in the word at `01202db0`, stores `0x5` in the word at `01202dac`, then stores the vtable `01017720` in `01202da8`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `01202da8` (`RET 4`) and registers `00fdd440` with `_atexit`.
// Translation incomplete: 00fbb7e0 - FS:[0] exception frame (handler 00f2e1ba) not translated.
pub fn fn_00fbb7e0(e: &mut Engine) {
    e.set_global::<u32>(0x0120_2da8, 0x0101_2120);
    e.set_global::<u32>(0x0120_2db0, 0x010c_1774);
    e.set_global::<u32>(0x0120_2dac, 5);
    register_with_singleton(e, 0x0120_2da8, 0x00fd_d440);
}

// Translated from 00fbb880 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `01202db4`: stores `01012120` in the word at `01202db4`, stores `010c179c` in the word at `01202dbc`, stores `0x14` in the word at `01202db8`, then stores the vtable `01017720` in `01202db4`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `01202db4` (`RET 4`) and registers `00fdd4c0` with `_atexit`.
// Translation incomplete: 00fbb880 - FS:[0] exception frame (handler 00f2e1ea) not translated.
pub fn fn_00fbb880(e: &mut Engine) {
    e.set_global::<u32>(0x0120_2db4, 0x0101_2120);
    e.set_global::<u32>(0x0120_2dbc, 0x010c_179c);
    e.set_global::<u32>(0x0120_2db8, 0x14);
    register_with_singleton(e, 0x0120_2db4, 0x00fd_d4c0);
}

// Translated from 00fbb920 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `01202dc0`: stores `01012120` in the word at `01202dc0`, stores `010c17dc` in the word at `01202dc8`, stores `0x32` in the word at `01202dc4`, then stores the vtable `01017720` in `01202dc0`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `01202dc0` (`RET 4`) and registers `00fdd540` with `_atexit`.
// Translation incomplete: 00fbb920 - FS:[0] exception frame (handler 00f2e21a) not translated.
pub fn fn_00fbb920(e: &mut Engine) {
    e.set_global::<u32>(0x0120_2dc0, 0x0101_2120);
    e.set_global::<u32>(0x0120_2dc8, 0x010c_17dc);
    e.set_global::<u32>(0x0120_2dc4, 0x32);
    register_with_singleton(e, 0x0120_2dc0, 0x00fd_d540);
}

// Translated from 00fbb9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `01202dcc`: stores `01012120` in the word at `01202dcc`, stores `010c180c` in the word at `01202dd4`, stores `0x14` in the word at `01202dd0`, then stores the vtable `01017720` in `01202dcc`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `01202dcc` (`RET 4`) and registers `00fdd5c0` with `_atexit`.
// Translation incomplete: 00fbb9c0 - FS:[0] exception frame (handler 00f2e24a) not translated.
pub fn fn_00fbb9c0(e: &mut Engine) {
    e.set_global::<u32>(0x0120_2dcc, 0x0101_2120);
    e.set_global::<u32>(0x0120_2dd4, 0x010c_180c);
    e.set_global::<u32>(0x0120_2dd0, 0x14);
    register_with_singleton(e, 0x0120_2dcc, 0x00fd_d5c0);
}

// Translated from 00fbbb00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: runs `00c46530` on `01202e20` (`ECX`; stack arguments `0x0`, `000003e8`), then stores `010c1c54` in the word at `01202e20`, then registers `00fdd640` with `_atexit` (`00ec658f`).
pub fn fn_00fbbb00(e: &mut Engine) {
    e.call(0x00c4_6530, &args![0x0120_2e20, 0, 0x3e8]);
    e.set_global::<u32>(0x0120_2e20, 0x010c_1c54);
    e.call(0x00ec_658f, &args![0x00fd_d640]);
}

// Translated from 00fbbda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `01203120`: stores `01012120` in the word at `01203120`, stores `010c2820` in the word at `01203128`, copies the float at `010c281c` to `01203124` (`FLD`/`FSTP`), then stores the vtable `01017720` in `01203120`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `01203120` (`RET 4`) and registers `00fdd6d0` with `_atexit`.
// Translation incomplete: 00fbbda0 - FS:[0] exception frame (handler 00f2ef5a) not translated.
pub fn fn_00fbbda0(e: &mut Engine) {
    e.set_global::<u32>(0x0120_3120, 0x0101_2120);
    e.set_global::<u32>(0x0120_3128, 0x010c_2820);
    e.set_global::<u32>(0x0120_3124, x87_quiet_bits(e.global::<u32>(0x010c_281c)));
    register_with_singleton(e, 0x0120_3120, 0x00fd_d6d0);
}

// Translated from 00fbbe40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `0120312c`: stores `01012120` in the word at `0120312c`, stores `010c2844` in the word at `01203134`, copies the float at `010c2840` to `01203130` (`FLD`/`FSTP`), then stores the vtable `01017720` in `0120312c`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `0120312c` (`RET 4`) and registers `00fdd750` with `_atexit`.
// Translation incomplete: 00fbbe40 - FS:[0] exception frame (handler 00f2ef8a) not translated.
pub fn fn_00fbbe40(e: &mut Engine) {
    e.set_global::<u32>(0x0120_312c, 0x0101_2120);
    e.set_global::<u32>(0x0120_3134, 0x010c_2844);
    e.set_global::<u32>(0x0120_3130, x87_quiet_bits(e.global::<u32>(0x010c_2840)));
    register_with_singleton(e, 0x0120_312c, 0x00fd_d750);
}

// Translated from 00fbbee0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `01203138`: stores `01012120` in the word at `01203138`, stores `010c2864` in the word at `01203140`, stores 1.0 in the float at `0120313c` (`FLD1`/`FSTP`), then stores the vtable `01017720` in `01203138`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `01203138` (`RET 4`) and registers `00fdd7d0` with `_atexit`.
// Translation incomplete: 00fbbee0 - FS:[0] exception frame (handler 00f2efba) not translated.
pub fn fn_00fbbee0(e: &mut Engine) {
    e.set_global::<u32>(0x0120_3138, 0x0101_2120);
    e.set_global::<u32>(0x0120_3140, 0x010c_2864);
    e.set_global::<f32>(0x0120_313c, 1.0);
    register_with_singleton(e, 0x0120_3138, 0x00fd_d7d0);
}

// Translated from 00fbbf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `01203144`: stores `01012120` in the word at `01203144`, stores `010c2884` in the word at `0120314c`, copies the float at `0101712c` to `01203148` (`FLD`/`FSTP`), then stores the vtable `01017720` in `01203144`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `01203144` (`RET 4`) and registers `00fdd850` with `_atexit`.
// Translation incomplete: 00fbbf80 - FS:[0] exception frame (handler 00f2efea) not translated.
pub fn fn_00fbbf80(e: &mut Engine) {
    e.set_global::<u32>(0x0120_3144, 0x0101_2120);
    e.set_global::<u32>(0x0120_314c, 0x010c_2884);
    e.set_global::<u32>(0x0120_3148, x87_quiet_bits(e.global::<u32>(0x0101_712c)));
    register_with_singleton(e, 0x0120_3144, 0x00fd_d850);
}

// Translated from 00fbc020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `01203150`: stores `01012120` in the word at `01203150`, stores `010c289c` in the word at `01203158`, copies the float at `0102f0f8` to `01203154` (`FLD`/`FSTP`), then stores the vtable `01017720` in `01203150`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `01203150` (`RET 4`) and registers `00fdd8d0` with `_atexit`.
// Translation incomplete: 00fbc020 - FS:[0] exception frame (handler 00f2f01a) not translated.
pub fn fn_00fbc020(e: &mut Engine) {
    e.set_global::<u32>(0x0120_3150, 0x0101_2120);
    e.set_global::<u32>(0x0120_3158, 0x010c_289c);
    e.set_global::<u32>(0x0120_3154, x87_quiet_bits(e.global::<u32>(0x0102_f0f8)));
    register_with_singleton(e, 0x0120_3150, 0x00fd_d8d0);
}

// Translated from 00fbc0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `0120315c`: stores `01012120` in the word at `0120315c`, stores `010c28b0` in the word at `01203164`, copies the float at `0102f0f8` to `01203160` (`FLD`/`FSTP`), then stores the vtable `01017720` in `0120315c`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `0120315c` (`RET 4`) and registers `00fdd950` with `_atexit`.
// Translation incomplete: 00fbc0c0 - FS:[0] exception frame (handler 00f2f04a) not translated.
pub fn fn_00fbc0c0(e: &mut Engine) {
    e.set_global::<u32>(0x0120_315c, 0x0101_2120);
    e.set_global::<u32>(0x0120_3164, 0x010c_28b0);
    e.set_global::<u32>(0x0120_3160, x87_quiet_bits(e.global::<u32>(0x0102_f0f8)));
    register_with_singleton(e, 0x0120_315c, 0x00fd_d950);
}

// Translated from 00fbc160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `01203168`: stores `01012120` in the word at `01203168`, stores `010c28cc` in the word at `01203170`, copies the float at `0102f0f8` to `0120316c` (`FLD`/`FSTP`), then stores the vtable `01017720` in `01203168`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `01203168` (`RET 4`) and registers `00fdd9d0` with `_atexit`.
// Translation incomplete: 00fbc160 - FS:[0] exception frame (handler 00f2f07a) not translated.
pub fn fn_00fbc160(e: &mut Engine) {
    e.set_global::<u32>(0x0120_3168, 0x0101_2120);
    e.set_global::<u32>(0x0120_3170, 0x010c_28cc);
    e.set_global::<u32>(0x0120_316c, x87_quiet_bits(e.global::<u32>(0x0102_f0f8)));
    register_with_singleton(e, 0x0120_3168, 0x00fd_d9d0);
}

// Translated from 00fbc200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `01203174`: stores `01012120` in the word at `01203174`, stores `010c28f0` in the word at `0120317c`, copies the float at `010c28ec` to `01203178` (`FLD`/`FSTP`), then stores the vtable `01017720` in `01203174`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `01203174` (`RET 4`) and registers `00fdda50` with `_atexit`.
// Translation incomplete: 00fbc200 - FS:[0] exception frame (handler 00f2f0aa) not translated.
pub fn fn_00fbc200(e: &mut Engine) {
    e.set_global::<u32>(0x0120_3174, 0x0101_2120);
    e.set_global::<u32>(0x0120_317c, 0x010c_28f0);
    e.set_global::<u32>(0x0120_3178, x87_quiet_bits(e.global::<u32>(0x010c_28ec)));
    register_with_singleton(e, 0x0120_3174, 0x00fd_da50);
}

// Translated from 00fbc2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `01203188`: stores `01012120` in the word at `01203188`, stores `010c291c` in the word at `01203190`, stores `0x0` in the byte at `0120318c`, then stores the vtable `01017720` in `01203188`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `01203188` (`RET 4`) and registers `00fddad0` with `_atexit`.
// Translation incomplete: 00fbc2c0 - FS:[0] exception frame (handler 00f2f0da) not translated.
pub fn fn_00fbc2c0(e: &mut Engine) {
    e.set_global::<u32>(0x0120_3188, 0x0101_2120);
    e.set_global::<u32>(0x0120_3190, 0x010c_291c);
    e.set_global::<u8>(0x0120_318c, 0);
    register_with_singleton(e, 0x0120_3188, 0x00fd_dad0);
}

// Translated from 00fbc360 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `01203194`: stores `01012120` in the word at `01203194`, stores `010c2938` in the word at `0120319c`, stores `0x14` in the word at `01203198`, then stores the vtable `01017720` in `01203194`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `01203194` (`RET 4`) and registers `00fddb50` with `_atexit`.
// Translation incomplete: 00fbc360 - FS:[0] exception frame (handler 00f2f10a) not translated.
pub fn fn_00fbc360(e: &mut Engine) {
    e.set_global::<u32>(0x0120_3194, 0x0101_2120);
    e.set_global::<u32>(0x0120_319c, 0x010c_2938);
    e.set_global::<u32>(0x0120_3198, 0x14);
    register_with_singleton(e, 0x0120_3194, 0x00fd_db50);
}

// Translated from 00fbc400 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `012031a0`: stores `01012120` in the word at `012031a0`, stores `010c2958` in the word at `012031a8`, stores `0x28` in the word at `012031a4`, then stores the vtable `01017720` in `012031a0`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `012031a0` (`RET 4`) and registers `00fddbd0` with `_atexit`.
// Translation incomplete: 00fbc400 - FS:[0] exception frame (handler 00f2f13a) not translated.
pub fn fn_00fbc400(e: &mut Engine) {
    e.set_global::<u32>(0x0120_31a0, 0x0101_2120);
    e.set_global::<u32>(0x0120_31a8, 0x010c_2958);
    e.set_global::<u32>(0x0120_31a4, 0x28);
    register_with_singleton(e, 0x0120_31a0, 0x00fd_dbd0);
}

// Translated from 00fbc4a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `012031ac`: stores `01012120` in the word at `012031ac`, stores `010c2978` in the word at `012031b4`, stores `000007d0` in the word at `012031b0`, then stores the vtable `01017720` in `012031ac`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `012031ac` (`RET 4`) and registers `00fddc50` with `_atexit`.
// Translation incomplete: 00fbc4a0 - FS:[0] exception frame (handler 00f2f16a) not translated.
pub fn fn_00fbc4a0(e: &mut Engine) {
    e.set_global::<u32>(0x0120_31ac, 0x0101_2120);
    e.set_global::<u32>(0x0120_31b4, 0x010c_2978);
    e.set_global::<u32>(0x0120_31b0, 0x7d0);
    register_with_singleton(e, 0x0120_31ac, 0x00fd_dc50);
}

// Translated from 00fbc7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer: calls `InitializeCriticalSection` (import slot `00fdf054`, with `01203328`), then registers `00fddcd0` with `_atexit` (`00ec658f`).
pub fn fn_00fbc7c0(e: &mut Engine) {
    e.call(0x00fd_f054, &args![0x0120_3328]);
    e.call(0x00ec_658f, &args![0x00fd_dcd0]);
}

// Translated from 00fbe090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the 16 16-byte vectors at `01268460..01268550`: each is a vector of four words (0 or -1, one-hot, then the combinations) built word by word in an aligned local, loaded into `XMM0` (`MOVAPS`) and stored to its global the next time round (`MOVAPS`; the globals are 16-byte aligned, as that instruction requires). Each store takes the local's previous contents.
pub fn fn_00fbe090(e: &mut Engine) {
    let mut stack = [0u32; 4]; // the 16-byte aligned local at ESP after AND ESP,-16; SUB ESP,0x10
    let ecx = 0u32;
    let eax = 0xffff_ffffu32; // OR EAX,-1
    stack[0] = ecx;
    stack[1] = ecx;
    stack[2] = ecx;
    stack[3] = ecx;
    let xmm0 = stack;
    stack[0] = eax;
    stack[1] = ecx;
    stack[2] = ecx;
    stack[3] = ecx;
    store_vec(e, 0x0126_8460, xmm0);
    let xmm0 = stack;
    stack[0] = ecx;
    stack[1] = eax;
    stack[2] = ecx;
    stack[3] = ecx;
    store_vec(e, 0x0126_8470, xmm0);
    let xmm0 = stack;
    stack[0] = eax;
    stack[1] = eax;
    stack[2] = ecx;
    stack[3] = ecx;
    store_vec(e, 0x0126_8480, xmm0);
    let xmm0 = stack;
    stack[0] = ecx;
    stack[1] = ecx;
    stack[2] = eax;
    stack[3] = ecx;
    store_vec(e, 0x0126_8490, xmm0);
    let xmm0 = stack;
    stack[0] = eax;
    stack[1] = ecx;
    stack[2] = eax;
    stack[3] = ecx;
    store_vec(e, 0x0126_84a0, xmm0);
    let xmm0 = stack;
    stack[0] = ecx;
    stack[1] = eax;
    stack[2] = eax;
    stack[3] = ecx;
    store_vec(e, 0x0126_84b0, xmm0);
    let xmm0 = stack;
    stack[0] = eax;
    stack[1] = eax;
    stack[2] = eax;
    stack[3] = ecx;
    store_vec(e, 0x0126_84c0, xmm0);
    let xmm0 = stack;
    stack[0] = ecx;
    stack[1] = ecx;
    stack[2] = ecx;
    stack[3] = eax;
    store_vec(e, 0x0126_84d0, xmm0);
    let xmm0 = stack;
    stack[0] = eax;
    stack[1] = ecx;
    stack[2] = ecx;
    stack[3] = eax;
    store_vec(e, 0x0126_84e0, xmm0);
    let xmm0 = stack;
    stack[0] = ecx;
    stack[1] = eax;
    stack[2] = ecx;
    stack[3] = eax;
    store_vec(e, 0x0126_84f0, xmm0);
    let xmm0 = stack;
    stack[0] = eax;
    stack[1] = eax;
    stack[2] = ecx;
    stack[3] = eax;
    store_vec(e, 0x0126_8500, xmm0);
    let xmm0 = stack;
    stack[0] = ecx;
    stack[1] = ecx;
    stack[2] = eax;
    stack[3] = eax;
    store_vec(e, 0x0126_8510, xmm0);
    let xmm0 = stack;
    store_vec(e, 0x0126_8520, xmm0);
    stack[0] = eax;
    stack[1] = ecx;
    stack[2] = eax;
    stack[3] = eax;
    let xmm0 = stack;
    stack[0] = ecx;
    stack[1] = eax;
    stack[2] = eax;
    stack[3] = eax;
    store_vec(e, 0x0126_8530, xmm0);
    let xmm0 = stack;
    stack[0] = eax;
    stack[1] = eax;
    stack[2] = eax;
    stack[3] = eax;
    store_vec(e, 0x0126_8540, xmm0);
    let xmm0 = stack;
    store_vec(e, 0x0126_8550, xmm0);
}

// Translated from 00fc76d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `0126fac8`: stores `01012120` in the word at `0126fac8`, stores `010f1dbc` in the word at `0126fad0`, stores `0x1` in the byte at `0126facc`, then stores the vtable `01017720` in `0126fac8`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `0126fac8` (`RET 4`) and registers `00fde3d0` with `_atexit`.
// Translation incomplete: 00fc76d0 - FS:[0] exception frame (handler 00f34c8a) not translated.
pub fn fn_00fc76d0(e: &mut Engine) {
    e.set_global::<u32>(0x0126_fac8, 0x0101_2120);
    e.set_global::<u32>(0x0126_fad0, 0x010f_1dbc);
    e.set_global::<u8>(0x0126_facc, 1);
    register_with_singleton(e, 0x0126_fac8, 0x00fd_e3d0);
}

// Translated from 00fc7770 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `0126fad4`: stores `01012120` in the word at `0126fad4`, stores `010f1ddc` in the word at `0126fadc`, stores `01900000` in the word at `0126fad8`, then stores the vtable `01017720` in `0126fad4`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `0126fad4` (`RET 4`) and registers `00fde450` with `_atexit`.
// Translation incomplete: 00fc7770 - FS:[0] exception frame (handler 00f34cba) not translated.
pub fn fn_00fc7770(e: &mut Engine) {
    e.set_global::<u32>(0x0126_fad4, 0x0101_2120);
    e.set_global::<u32>(0x0126_fadc, 0x010f_1ddc);
    e.set_global::<u32>(0x0126_fad8, 0x0190_0000);
    register_with_singleton(e, 0x0126_fad4, 0x00fd_e450);
}

// Translated from 00fc7810 (decompiled, FalloutNV.exe 1.4.0.525)
/// The start-up initializer of the object at `0126fae0`: stores `01012120` in the word at `0126fae0`, stores `010f1df8` in the word at `0126fae8`, stores `0x3` in the word at `0126fae4`, then stores the vtable `01017720` in `0126fae0`, calls `0044f570` (the singleton's getter), calls the singleton's virtual function at vtable slot +4 with `0126fae0` (`RET 4`) and registers `00fde4d0` with `_atexit`.
// Translation incomplete: 00fc7810 - FS:[0] exception frame (handler 00f34cea) not translated.
pub fn fn_00fc7810(e: &mut Engine) {
    e.set_global::<u32>(0x0126_fae0, 0x0101_2120);
    e.set_global::<u32>(0x0126_fae8, 0x010f_1df8);
    e.set_global::<u32>(0x0126_fae4, 3);
    register_with_singleton(e, 0x0126_fae0, 0x00fd_e4d0);
}

/// Import slot of KERNEL32 `TlsAlloc` (stdcall, no arguments, the new index in `EAX`).
const TLS_ALLOC: u32 = 0x00fd_f048;

/// The shape of Havok's four thread-local index initializers: `CALL [TlsAlloc]`, `PUSH
/// destructor`, `MOV [global],EAX`, `CALL _atexit`, `POP ECX`, `RET`. The destructor (the
/// `atexit_irregular` unit) passes the global to `TlsFree`.
fn tls_index(e: &mut Engine, global: u32, destructor: u32) {
    let index = e.call(TLS_ALLOC, &args![]).u32();
    e.set_global::<u32>(global, index);
    e.call(ATEXIT, &args![destructor]);
}

// Translated from 00fbd610 (disassembled, FalloutNV.exe 1.4.0.525)
/// The thread-local index of Havok's memory router (`hkMemoryRouter::getInstance`, `00c85750`,
/// reads it): `TlsAlloc` into `01268108`, its `TlsFree` destructor `00fde150` registered with
/// `_atexit`.
pub fn fn_00fbd610(e: &mut Engine) {
    tls_index(e, 0x0126_8108, 0x00fd_e150);
}

// Translated from 00fbd8c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// A Havok thread-local index: `TlsAlloc` into `012681d8`, its `TlsFree` destructor `00fde190`
/// registered with `_atexit`.
pub fn fn_00fbd8c0(e: &mut Engine) {
    tls_index(e, 0x0126_81d8, 0x00fd_e190);
}

// Translated from 00fbdd80 (disassembled, FalloutNV.exe 1.4.0.525)
/// The thread-local index of Havok's monitor stream (`hkMonitorStream::init`, `00cb0db0`, sets
/// it): `TlsAlloc` into `0126835c`, its `TlsFree` destructor `00fde250` registered with
/// `_atexit`.
pub fn fn_00fbdd80(e: &mut Engine) {
    tls_index(e, 0x0126_835c, 0x00fd_e250);
}

// Translated from 00fbe810 (disassembled, FalloutNV.exe 1.4.0.525)
/// The thread-local index Havok's job queue and worker threads use (`hkWorkerThreadFunc`,
/// `00d2cfb0`, sets it): `TlsAlloc` into `012687c8`, its `TlsFree` destructor `00fde270`
/// registered with `_atexit`.
pub fn fn_00fbe810(e: &mut Engine) {
    tls_index(e, 0x0126_87c8, 0x00fd_e270);
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00f9_b040, fn_00f9b040()),
        entry!(0x00f9_b2a0, fn_00f9b2a0()),
        entry!(0x00f9_b520, fn_00f9b520()),
        entry!(0x00f9_b780, fn_00f9b780()),
        entry!(0x00f9_b9e0, fn_00f9b9e0()),
        entry!(0x00f9_b9f0, fn_00f9b9f0()),
        entry!(0x00f9_bc60, fn_00f9bc60()),
        entry!(0x00f9_bf60, fn_00f9bf60()),
        entry!(0x00f9_bf70, fn_00f9bf70()),
        entry!(0x00f9_c1e0, fn_00f9c1e0()),
        entry!(0x00f9_c1f0, fn_00f9c1f0()),
        entry!(0x00f9_c460, fn_00f9c460()),
        entry!(0x00f9_c470, fn_00f9c470()),
        entry!(0x00f9_c530, fn_00f9c530()),
        entry!(0x00f9_c7a0, fn_00f9c7a0()),
        entry!(0x00f9_c7b0, fn_00f9c7b0()),
        entry!(0x00f9_ca20, fn_00f9ca20()),
        entry!(0x00f9_ca30, fn_00f9ca30()),
        entry!(0x00f9_cca0, fn_00f9cca0()),
        entry!(0x00f9_ccb0, fn_00f9ccb0()),
        entry!(0x00f9_cf20, fn_00f9cf20()),
        entry!(0x00f9_cf30, fn_00f9cf30()),
        entry!(0x00f9_d1a0, fn_00f9d1a0()),
        entry!(0x00f9_d1b0, fn_00f9d1b0()),
        entry!(0x00f9_d420, fn_00f9d420()),
        entry!(0x00f9_d430, fn_00f9d430()),
        entry!(0x00f9_d6a0, fn_00f9d6a0()),
        entry!(0x00f9_d6b0, fn_00f9d6b0()),
        entry!(0x00f9_d920, fn_00f9d920()),
        entry!(0x00f9_d930, fn_00f9d930()),
        entry!(0x00f9_db40, fn_00f9db40()),
        entry!(0x00f9_e0a0, fn_00f9e0a0()),
        entry!(0x00f9_e0e0, fn_00f9e0e0()),
        entry!(0x00f9_e350, fn_00f9e350()),
        entry!(0x00f9_e730, fn_00f9e730()),
        entry!(0x00f9_ec20, fn_00f9ec20()),
        entry!(0x00f9_efa0, fn_00f9efa0()),
        entry!(0x00f9_f240, fn_00f9f240()),
        entry!(0x00f9_f5a0, fn_00f9f5a0()),
        entry!(0x00f9_fa20, fn_00f9fa20()),
        entry!(0x00f9_fc80, fn_00f9fc80()),
        entry!(0x00f9_fee0, fn_00f9fee0()),
        entry!(0x00fa_01b0, fn_00fa01b0()),
        entry!(0x00fa_0410, fn_00fa0410()),
        entry!(0x00fa_09e0, fn_00fa09e0()),
        entry!(0x00fa_0c40, fn_00fa0c40()),
        entry!(0x00fa_0ef0, fn_00fa0ef0()),
        entry!(0x00fa_11f0, fn_00fa11f0()),
        entry!(0x00fa_1450, fn_00fa1450()),
        entry!(0x00fa_1790, fn_00fa1790()),
        entry!(0x00fa_1b20, fn_00fa1b20()),
        entry!(0x00fa_1d40, fn_00fa1d40()),
        entry!(0x00fa_2260, fn_00fa2260()),
        entry!(0x00fa_3610, fn_00fa3610()),
        entry!(0x00fa_38c0, fn_00fa38c0()),
        entry!(0x00fa_3b40, fn_00fa3b40()),
        entry!(0x00fa_3f30, fn_00fa3f30()),
        entry!(0x00fa_4290, fn_00fa4290()),
        entry!(0x00fa_4860, fn_00fa4860()),
        entry!(0x00fa_4ad0, fn_00fa4ad0()),
        entry!(0x00fa_4d30, fn_00fa4d30()),
        entry!(0x00fa_4fc0, fn_00fa4fc0()),
        entry!(0x00fa_52e0, fn_00fa52e0()),
        entry!(0x00fa_5640, fn_00fa5640()),
        entry!(0x00fa_59c0, fn_00fa59c0()),
        entry!(0x00fa_5bc0, fn_00fa5bc0()),
        entry!(0x00fa_5e80, fn_00fa5e80()),
        entry!(0x00fa_6240, fn_00fa6240()),
        entry!(0x00fa_6560, fn_00fa6560()),
        entry!(0x00fa_6800, fn_00fa6800()),
        entry!(0x00fa_6a60, fn_00fa6a60()),
        entry!(0x00fa_6cc0, fn_00fa6cc0()),
        entry!(0x00fa_7020, fn_00fa7020()),
        entry!(0x00fa_7220, fn_00fa7220()),
        entry!(0x00fa_76e0, fn_00fa76e0()),
        entry!(0x00fa_7940, fn_00fa7940()),
        entry!(0x00fa_7c60, fn_00fa7c60()),
        entry!(0x00fa_7fb0, fn_00fa7fb0()),
        entry!(0x00fa_8290, fn_00fa8290()),
        entry!(0x00fa_8670, fn_00fa8670()),
        entry!(0x00fa_9220, fn_00fa9220()),
        entry!(0x00fa_94d0, fn_00fa94d0()),
        entry!(0x00fa_9730, fn_00fa9730()),
        entry!(0x00fa_9b00, fn_00fa9b00()),
        entry!(0x00fa_9d70, fn_00fa9d70()),
        entry!(0x00fa_9fd0, fn_00fa9fd0()),
        entry!(0x00fa_a230, fn_00faa230()),
        entry!(0x00fa_a4f0, fn_00faa4f0()),
        entry!(0x00fa_a8f0, fn_00faa8f0()),
        entry!(0x00fa_ac40, fn_00faac40()),
        entry!(0x00fa_b030, fn_00fab030()),
        entry!(0x00fa_b380, fn_00fab380()),
        entry!(0x00fa_b670, fn_00fab670()),
        entry!(0x00fa_bba0, fn_00fabba0()),
        entry!(0x00fa_bf80, fn_00fabf80()),
        entry!(0x00fa_c210, fn_00fac210()),
        entry!(0x00fa_c4e0, fn_00fac4e0()),
        entry!(0x00fa_c850, fn_00fac850()),
        entry!(0x00fa_cb20, fn_00facb20()),
        entry!(0x00fa_cd80, fn_00facd80()),
        entry!(0x00fa_cfe0, fn_00facfe0()),
        entry!(0x00fa_d240, fn_00fad240()),
        entry!(0x00fa_d4a0, fn_00fad4a0()),
        entry!(0x00fa_d700, fn_00fad700()),
        entry!(0x00fa_d960, fn_00fad960()),
        entry!(0x00fa_db60, fn_00fadb60()),
        entry!(0x00fa_deb0, fn_00fadeb0()),
        entry!(0x00fa_e110, fn_00fae110()),
        entry!(0x00fa_e510, fn_00fae510()),
        entry!(0x00fa_e7d0, fn_00fae7d0()),
        entry!(0x00fa_ea30, fn_00faea30()),
        entry!(0x00fa_ec30, fn_00faec30()),
        entry!(0x00fa_ef10, fn_00faef10()),
        entry!(0x00fa_f230, fn_00faf230()),
        entry!(0x00fa_f4f0, fn_00faf4f0()),
        entry!(0x00fa_f810, fn_00faf810()),
        entry!(0x00fa_fa70, fn_00fafa70()),
        entry!(0x00fa_fd90, fn_00fafd90()),
        entry!(0x00fb_0050, fn_00fb0050()),
        entry!(0x00fb_0250, fn_00fb0250()),
        entry!(0x00fb_04b0, fn_00fb04b0()),
        entry!(0x00fb_07d0, fn_00fb07d0()),
        entry!(0x00fb_0a30, fn_00fb0a30()),
        entry!(0x00fb_0c90, fn_00fb0c90()),
        entry!(0x00fb_1330, fn_00fb1330()),
        entry!(0x00fb_1590, fn_00fb1590()),
        entry!(0x00fb_17f0, fn_00fb17f0()),
        entry!(0x00fb_1a50, fn_00fb1a50()),
        entry!(0x00fb_1d10, fn_00fb1d10()),
        entry!(0x00fb_1f70, fn_00fb1f70()),
        entry!(0x00fb_21d0, fn_00fb21d0()),
        entry!(0x00fb_2490, fn_00fb2490()),
        entry!(0x00fb_26f0, fn_00fb26f0()),
        entry!(0x00fb_2950, fn_00fb2950()),
        entry!(0x00fb_2c10, fn_00fb2c10()),
        entry!(0x00fb_2e70, fn_00fb2e70()),
        entry!(0x00fb_3160, fn_00fb3160()),
        entry!(0x00fb_31b0, fn_00fb31b0()),
        entry!(0x00fb_31f0, fn_00fb31f0()),
        entry!(0x00fb_3240, fn_00fb3240()),
        entry!(0x00fb_3250, fn_00fb3250()),
        entry!(0x00fb_32b0, fn_00fb32b0()),
        entry!(0x00fb_32e0, fn_00fb32e0()),
        entry!(0x00fb_3310, fn_00fb3310()),
        entry!(0x00fb_3340, fn_00fb3340()),
        entry!(0x00fb_3360, fn_00fb3360()),
        entry!(0x00fb_3390, fn_00fb3390()),
        entry!(0x00fb_33c0, fn_00fb33c0()),
        entry!(0x00fb_34d0, fn_00fb34d0()),
        entry!(0x00fb_3510, fn_00fb3510()),
        entry!(0x00fb_3700, fn_00fb3700()),
        entry!(0x00fb_3760, fn_00fb3760()),
        entry!(0x00fb_37a0, fn_00fb37a0()),
        entry!(0x00fb_3d30, fn_00fb3d30()),
        entry!(0x00fb_3d70, fn_00fb3d70()),
        entry!(0x00fb_3dc0, fn_00fb3dc0()),
        entry!(0x00fb_3e10, fn_00fb3e10()),
        entry!(0x00fb_3e60, fn_00fb3e60()),
        entry!(0x00fb_3ed0, fn_00fb3ed0()),
        entry!(0x00fb_3f10, fn_00fb3f10()),
        entry!(0x00fb_3fa0, fn_00fb3fa0()),
        entry!(0x00fb_3fe0, fn_00fb3fe0()),
        entry!(0x00fb_4070, fn_00fb4070()),
        entry!(0x00fb_40b0, fn_00fb40b0()),
        entry!(0x00fb_4140, fn_00fb4140()),
        entry!(0x00fb_4180, fn_00fb4180()),
        entry!(0x00fb_41d0, fn_00fb41d0()),
        entry!(0x00fb_4230, fn_00fb4230()),
        entry!(0x00fb_4270, fn_00fb4270()),
        entry!(0x00fb_4300, fn_00fb4300()),
        entry!(0x00fb_4340, fn_00fb4340()),
        entry!(0x00fb_4390, fn_00fb4390()),
        entry!(0x00fb_4430, fn_00fb4430()),
        entry!(0x00fb_44d0, fn_00fb44d0()),
        entry!(0x00fb_4570, fn_00fb4570()),
        entry!(0x00fb_4610, fn_00fb4610()),
        entry!(0x00fb_46b0, fn_00fb46b0()),
        entry!(0x00fb_4750, fn_00fb4750()),
        entry!(0x00fb_47f0, fn_00fb47f0()),
        entry!(0x00fb_4890, fn_00fb4890()),
        entry!(0x00fb_4930, fn_00fb4930()),
        entry!(0x00fb_5f40, fn_00fb5f40() -> u32),
        entry!(0x00fb_66d0, fn_00fb66d0()),
        entry!(0x00fb_6730, fn_00fb6730()),
        entry!(0x00fb_6770, fn_00fb6770()),
        entry!(0x00fb_6800, fn_00fb6800()),
        entry!(0x00fb_6840, fn_00fb6840()),
        entry!(0x00fb_68d0, fn_00fb68d0()),
        entry!(0x00fb_6910, fn_00fb6910()),
        entry!(0x00fb_69a0, fn_00fb69a0()),
        entry!(0x00fb_69e0, fn_00fb69e0()),
        entry!(0x00fb_6a70, fn_00fb6a70()),
        entry!(0x00fb_6ab0, fn_00fb6ab0()),
        entry!(0x00fb_6b40, fn_00fb6b40()),
        entry!(0x00fb_6b80, fn_00fb6b80()),
        entry!(0x00fb_6bd0, fn_00fb6bd0()),
        entry!(0x00fb_7080, fn_00fb7080()),
        entry!(0x00fb_7c90, fn_00fb7c90()),
        entry!(0x00fb_7d30, fn_00fb7d30()),
        entry!(0x00fb_7dd0, fn_00fb7dd0()),
        entry!(0x00fb_7e70, fn_00fb7e70()),
        entry!(0x00fb_7f10, fn_00fb7f10()),
        entry!(0x00fb_8060, fn_00fb8060()),
        entry!(0x00fb_80d0, fn_00fb80d0()),
        entry!(0x00fb_8140, fn_00fb8140()),
        entry!(0x00fb_81b0, fn_00fb81b0()),
        entry!(0x00fb_8230, fn_00fb8230()),
        entry!(0x00fb_82a0, fn_00fb82a0()),
        entry!(0x00fb_8310, fn_00fb8310()),
        entry!(0x00fb_8380, fn_00fb8380()),
        entry!(0x00fb_8400, fn_00fb8400()),
        entry!(0x00fb_8470, fn_00fb8470()),
        entry!(0x00fb_84e0, fn_00fb84e0()),
        entry!(0x00fb_8550, fn_00fb8550()),
        entry!(0x00fb_85c0, fn_00fb85c0()),
        entry!(0x00fb_85e0, fn_00fb85e0()),
        entry!(0x00fb_8630, fn_00fb8630()),
        entry!(0x00fb_8740, fn_00fb8740()),
        entry!(0x00fb_8770, fn_00fb8770()),
        entry!(0x00fb_8790, fn_00fb8790()),
        entry!(0x00fb_87c0, fn_00fb87c0()),
        entry!(0x00fb_88c0, fn_00fb88c0()),
        entry!(0x00fb_88f0, fn_00fb88f0()),
        entry!(0x00fb_8920, fn_00fb8920()),
        entry!(0x00fb_8950, fn_00fb8950()),
        entry!(0x00fb_8cc0, fn_00fb8cc0()),
        entry!(0x00fb_8d00, fn_00fb8d00()),
        entry!(0x00fb_8f80, fn_00fb8f80()),
        entry!(0x00fb_92a0, fn_00fb92a0()),
        entry!(0x00fb_b7c0, fn_00fbb7c0()),
        entry!(0x00fb_b7e0, fn_00fbb7e0()),
        entry!(0x00fb_b880, fn_00fbb880()),
        entry!(0x00fb_b920, fn_00fbb920()),
        entry!(0x00fb_b9c0, fn_00fbb9c0()),
        entry!(0x00fb_bb00, fn_00fbbb00()),
        entry!(0x00fb_bda0, fn_00fbbda0()),
        entry!(0x00fb_be40, fn_00fbbe40()),
        entry!(0x00fb_bee0, fn_00fbbee0()),
        entry!(0x00fb_bf80, fn_00fbbf80()),
        entry!(0x00fb_c020, fn_00fbc020()),
        entry!(0x00fb_c0c0, fn_00fbc0c0()),
        entry!(0x00fb_c160, fn_00fbc160()),
        entry!(0x00fb_c200, fn_00fbc200()),
        entry!(0x00fb_c2c0, fn_00fbc2c0()),
        entry!(0x00fb_c360, fn_00fbc360()),
        entry!(0x00fb_c400, fn_00fbc400()),
        entry!(0x00fb_c4a0, fn_00fbc4a0()),
        entry!(0x00fb_c7c0, fn_00fbc7c0()),
        entry!(0x00fb_d610, fn_00fbd610()),
        entry!(0x00fb_d8c0, fn_00fbd8c0()),
        entry!(0x00fb_dd80, fn_00fbdd80()),
        entry!(0x00fb_e090, fn_00fbe090()),
        entry!(0x00fb_e810, fn_00fbe810()),
        entry!(0x00fc_76d0, fn_00fc76d0()),
        entry!(0x00fc_7770, fn_00fc7770()),
        entry!(0x00fc_7810, fn_00fc7810()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Vec<(u32, Vec<u32>)>;

    /// One run of a function against doubles: the words in memory before, the answers the
    /// doubled callees give (`EAX`, and `ST0` for the doubles), the calls expected in order
    /// (the argument words, `this` first) and the memory expected afterwards as (address, width
    /// in bytes, value).
    struct Case {
        entry: u32,
        preset: &'static [(u32, u32)],
        rets: &'static [(u32, u32)],
        st0s: &'static [(u32, f64)],
        calls: &'static [(u32, &'static [u32])],
        mem: &'static [(u32, u32, u32)],
    }

    /// The calls the run of `entry` made, in order, the top-level call itself filtered out by its
    /// address.
    fn run(e: &mut Engine, entry: u32) -> Calls {
        e.call_log = Some(vec![]);
        e.call(entry, &args![]);
        let log = e.call_log.take().unwrap();
        log.into_iter().filter(|(a, _)| *a != entry).collect()
    }

    /// A word or a byte or a halfword of memory as a `u32`.
    fn read(e: &Engine, address: u32, width: u32) -> u32 {
        match width {
            1 => u32::from(e.mem.u8(address)),
            2 => u32::from(e.mem.u16(address)),
            _ => e.mem.u32(address),
        }
    }

    fn check(c: &Case) {
        let mut e = Engine::new();
        for &(a, _) in c.preset {
            e.map(a, 4);
        }
        for &(a, _, _) in c.mem {
            e.map(a, 4);
        }
        for &(a, w) in c.preset {
            e.mem.set_u32(a, w);
        }
        for &(callee, _) in c.calls {
            let eax = c.rets.iter().find(|r| r.0 == callee).map_or(0, |r| r.1);
            let st0 = c.st0s.iter().find(|r| r.0 == callee).map_or(0.0, |r| r.1);
            e.register_double(callee, move |_, _| Ret {
                eax,
                st0,
                ..Ret::default()
            });
        }
        let got = run(&mut e, c.entry);
        let want: Calls = c.calls.iter().map(|(a, w)| (*a, w.to_vec())).collect();
        assert_eq!(got, want, "{:08x}: calls", c.entry);
        for &(a, width, value) in c.mem {
            assert_eq!(
                read(&e, a, width),
                value,
                "{:08x}: memory at {a:08x}",
                c.entry
            );
        }
    }

    /// (function, global): `MOV ECX,global; CALL 00a2e180`.
    const COUNTER_CALLS: &[(u32, u32)] = &[
        (0x00f9_b040, 0x011d_c7d8),
        (0x00f9_b2a0, 0x011d_c858),
        (0x00f9_b520, 0x011d_c8d0),
        (0x00f9_b780, 0x011d_c948),
        (0x00f9_b9e0, 0x011d_c999),
        (0x00f9_bc60, 0x011d_ca38),
        (0x00f9_bf60, 0x011d_cac0),
        (0x00f9_c1e0, 0x011d_cb0d),
        (0x00f9_c460, 0x011d_cb89),
        (0x00f9_c7a0, 0x011d_cc3d),
        (0x00f9_ca20, 0x011d_cca5),
        (0x00f9_cca0, 0x011d_cd01),
        (0x00f9_cf20, 0x011d_cd81),
        (0x00f9_d1a0, 0x011d_ce20),
        (0x00f9_d420, 0x011d_ce51),
        (0x00f9_d6a0, 0x011d_cee5),
        (0x00f9_d920, 0x011d_cf75),
        (0x00f9_db40, 0x011d_d0cc),
        (0x00f9_e0a0, 0x011d_d404),
        (0x00f9_e350, 0x011d_d620),
        (0x00f9_e730, 0x011d_dd00),
        (0x00f9_efa0, 0x011d_ddc0),
        (0x00f9_f240, 0x011d_de44),
        (0x00f9_f5a0, 0x011d_dee8),
        (0x00f9_fa20, 0x011d_df98),
        (0x00f9_fc80, 0x011d_e010),
        (0x00f9_fee0, 0x011d_e088),
        (0x00fa_01b0, 0x011d_e114),
        (0x00fa_0410, 0x011d_e23d),
        (0x00fa_09e0, 0x011d_e440),
        (0x00fa_0c40, 0x011d_e4dc),
        (0x00fa_0ef0, 0x011d_e564),
        (0x00fa_11f0, 0x011d_e5ec),
        (0x00fa_1450, 0x011d_e664),
        (0x00fa_1790, 0x011d_e6f4),
        (0x00fa_1b20, 0x011d_e7ac),
        (0x00fa_1d40, 0x011d_e940),
        (0x00fa_2260, 0x011d_ee18),
        (0x00fa_3610, 0x011d_f050),
        (0x00fa_38c0, 0x011d_f0d0),
        (0x00fa_3b40, 0x011d_f160),
        (0x00fa_3f30, 0x011d_f23c),
        (0x00fa_4290, 0x011d_f71e),
        (0x00fa_4ad0, 0x011d_f8e4),
        (0x00fa_4d30, 0x011d_f968),
        (0x00fa_4fc0, 0x011d_f9e0),
        (0x00fa_52e0, 0x011d_fa1a),
        (0x00fa_5640, 0x011d_fc7c),
        (0x00fa_59c0, 0x011d_fcfc),
        (0x00fa_5bc0, 0x011d_fd74),
        (0x00fa_5e80, 0x011d_fdec),
        (0x00fa_6240, 0x011d_feb4),
        (0x00fa_6560, 0x011d_ff4c),
        (0x00fa_6800, 0x011d_ffcc),
        (0x00fa_6a60, 0x011e_0048),
        (0x00fa_6cc0, 0x011e_00c8),
        (0x00fa_7020, 0x011e_01bc),
        (0x00fa_7220, 0x011e_0247),
        (0x00fa_76e0, 0x011e_0334),
        (0x00fa_7940, 0x011e_0362),
        (0x00fa_7c60, 0x011e_0461),
        (0x00fa_7fb0, 0x011e_0529),
        (0x00fa_8290, 0x011e_0551),
        (0x00fa_8670, 0x011e_0b34),
        (0x00fa_94d0, 0x011e_0dec),
        (0x00fa_9730, 0x011f_12c0),
        (0x00fa_9b00, 0x011f_133c),
        (0x00fa_9d70, 0x011f_1389),
        (0x00fa_9fd0, 0x011f_14ec),
        (0x00fa_a230, 0x011f_1570),
        (0x00fa_a4f0, 0x011f_15cd),
        (0x00fa_a8f0, 0x011f_16ec),
        (0x00fa_ac40, 0x011f_1784),
        (0x00fa_b030, 0x011f_1898),
        (0x00fa_b380, 0x011f_193c),
        (0x00fa_b670, 0x011f_19d0),
        (0x00fa_bba0, 0x011f_1a62),
        (0x00fa_bf80, 0x011f_1b54),
        (0x00fa_c210, 0x011f_1bd8),
        (0x00fa_c4e0, 0x011f_1c7d),
        (0x00fa_c850, 0x011f_1d20),
        (0x00fa_cb20, 0x011f_1d6d),
        (0x00fa_cd80, 0x011f_1e20),
        (0x00fa_cfe0, 0x011f_1e98),
        (0x00fa_d240, 0x011f_1f10),
        (0x00fa_d4a0, 0x011f_1f88),
        (0x00fa_d700, 0x011f_2000),
        (0x00fa_d960, 0x011f_2088),
        (0x00fa_db60, 0x011f_20a5),
        (0x00fa_deb0, 0x011f_219c),
        (0x00fa_e110, 0x011f_22f8),
        (0x00fa_e510, 0x011f_2384),
        (0x00fa_e7d0, 0x011f_23fc),
        (0x00fa_ea30, 0x011f_2474),
        (0x00fa_ec30, 0x011f_24e5),
        (0x00fa_ef10, 0x011f_257c),
        (0x00fa_f230, 0x011f_2600),
        (0x00fa_f4f0, 0x011f_2684),
        (0x00fa_f810, 0x011f_2708),
        (0x00fa_fa70, 0x011f_278c),
        (0x00fa_fd90, 0x011f_2810),
        (0x00fb_0050, 0x011f_2894),
        (0x00fb_0250, 0x011f_290c),
        (0x00fb_04b0, 0x011f_2984),
        (0x00fb_07d0, 0x011f_2a08),
        (0x00fb_0a30, 0x011f_2a80),
        (0x00fb_0c90, 0x011f_2bf4),
        (0x00fb_1330, 0x011f_2c88),
        (0x00fb_1590, 0x011f_2d00),
        (0x00fb_17f0, 0x011f_2d84),
        (0x00fb_1a50, 0x011f_2e08),
        (0x00fb_1d10, 0x011f_2e80),
        (0x00fb_1f70, 0x011f_2ef8),
        (0x00fb_21d0, 0x011f_2f88),
        (0x00fb_2490, 0x011f_3000),
        (0x00fb_26f0, 0x011f_3078),
        (0x00fb_2950, 0x011f_30fc),
        (0x00fb_2c10, 0x011f_3180),
        (0x00fb_2e70, 0x011f_3204),
        (0x00fb_3160, 0x011f_327c),
    ];

    #[test]
    fn every_function_is_registered_once() {
        let funcs = funcs();
        assert_eq!(funcs.len(), 256);
        let mut addrs: Vec<u32> = funcs.iter().map(|(a, _)| *a).collect();
        let sorted = addrs.windows(2).all(|w| w[0] < w[1]);
        assert!(sorted, "ascending and unique");
        addrs.dedup();
        assert_eq!(addrs.len(), 256);
        let e = Engine::new();
        for a in addrs {
            assert!(e.is_translated(a), "{a:08x}");
        }
    }

    #[test]
    fn tls_index_initializers_store_the_index_and_register_tls_free() {
        for (f, global, destructor) in [
            (0x00fb_d610, 0x0126_8108, 0x00fd_e150),
            (0x00fb_d8c0, 0x0126_81d8, 0x00fd_e190),
            (0x00fb_dd80, 0x0126_835c, 0x00fd_e250),
            (0x00fb_e810, 0x0126_87c8, 0x00fd_e270),
        ] {
            let mut e = Engine::new();
            e.map(global, 4);
            e.register(TLS_ALLOC, |_, _| Ret {
                eax: 7,
                ..Ret::default()
            });
            e.register(ATEXIT, |_, _| Ret::default());
            let calls = run(&mut e, f);
            assert_eq!(
                calls,
                vec![(TLS_ALLOC, vec![]), (ATEXIT, vec![destructor])],
                "{f:08x}"
            );
            assert_eq!(e.mem.u32(global), 7, "{f:08x}");
        }
    }

    #[test]
    fn counter_initializers_run_00a2e180_on_their_global() {
        assert_eq!(COUNTER_CALLS.len(), 120);
        let mut e = Engine::new();
        e.register(0x00a2_e180, |_, _| Ret::default());
        for &(f, global) in COUNTER_CALLS {
            assert_eq!(run(&mut e, f), vec![(0x00a2_e180, vec![global])], "{f:08x}");
        }
    }

    #[test]
    fn short_functions_make_their_calls_and_stores() {
        let cases: &[Case] = &[
            Case {
                entry: 0x00f9_b9f0,
                preset: &[],
                rets: &[(0x0040_3a70, 0x1234_56a5)],
                st0s: &[],
                calls: &[(0x0040_3a70, &[0xf, 0x0082_58a0])],
                mem: &[(0x011d_c998, 1, 0xa5)],
            },
            Case {
                entry: 0x00f9_bf70,
                preset: &[],
                rets: &[(0x0040_3a70, 0x1234_56a5)],
                st0s: &[],
                calls: &[(0x0040_3a70, &[0x11, 0x0082_6260])],
                mem: &[(0x011d_cac1, 1, 0xa5)],
            },
            Case {
                entry: 0x00f9_c1f0,
                preset: &[],
                rets: &[(0x0040_3a70, 0x1234_56a5)],
                st0s: &[],
                calls: &[(0x0040_3a70, &[0x18, 0x0082_64d0])],
                mem: &[(0x011d_cb0c, 1, 0xa5)],
            },
            Case {
                entry: 0x00f9_c470,
                preset: &[],
                rets: &[(0x0040_3a70, 0x1234_56a5)],
                st0s: &[],
                calls: &[(0x0040_3a70, &[0x19, 0x0082_73c0])],
                mem: &[(0x011d_cb88, 1, 0xa5)],
            },
            Case {
                entry: 0x00f9_c530,
                preset: &[],
                rets: &[(0x0040_3a70, 0x1234_56a5)],
                st0s: &[],
                calls: &[(0x0040_3a70, &[1, 0x0082_7a40])],
                mem: &[(0x011d_cbcc, 1, 0xa5)],
            },
            Case {
                entry: 0x00f9_c7b0,
                preset: &[],
                rets: &[(0x0040_3a70, 0x1234_56a5)],
                st0s: &[],
                calls: &[(0x0040_3a70, &[5, 0x0082_80a0])],
                mem: &[(0x011d_cc3c, 1, 0xa5)],
            },
            Case {
                entry: 0x00f9_ca30,
                preset: &[],
                rets: &[(0x0040_3a70, 0x1234_56a5)],
                st0s: &[],
                calls: &[(0x0040_3a70, &[0x13, 0x0082_8760])],
                mem: &[(0x011d_cca4, 1, 0xa5)],
            },
            Case {
                entry: 0x00f9_ccb0,
                preset: &[],
                rets: &[(0x0040_3a70, 0x1234_56a5)],
                st0s: &[],
                calls: &[(0x0040_3a70, &[0x1c, 0x0082_a0d0])],
                mem: &[(0x011d_cd00, 1, 0xa5)],
            },
            Case {
                entry: 0x00f9_cf30,
                preset: &[],
                rets: &[(0x0040_3a70, 0x1234_56a5)],
                st0s: &[],
                calls: &[(0x0040_3a70, &[0x15, 0x0082_b110])],
                mem: &[(0x011d_cd80, 1, 0xa5)],
            },
            Case {
                entry: 0x00f9_d1b0,
                preset: &[],
                rets: &[(0x0040_3a70, 0x1234_56a5)],
                st0s: &[],
                calls: &[(0x0040_3a70, &[0x1b, 0x0082_b6d0])],
                mem: &[(0x011d_ce21, 1, 0xa5)],
            },
            Case {
                entry: 0x00f9_d430,
                preset: &[],
                rets: &[(0x0040_3a70, 0x1234_56a5)],
                st0s: &[],
                calls: &[(0x0040_3a70, &[0x22, 0x0082_b8f0])],
                mem: &[(0x011d_ce50, 1, 0xa5)],
            },
            Case {
                entry: 0x00f9_d6b0,
                preset: &[],
                rets: &[(0x0040_3a70, 0x1234_56a5)],
                st0s: &[],
                calls: &[(0x0040_3a70, &[0, 0x0082_bc30])],
                mem: &[(0x011d_cee4, 1, 0xa5)],
            },
            Case {
                entry: 0x00f9_d930,
                preset: &[],
                rets: &[(0x0040_3a70, 0x1234_56a5)],
                st0s: &[],
                calls: &[(0x0040_3a70, &[0x1d, 0x0082_d090])],
                mem: &[(0x011d_cf74, 1, 0xa5)],
            },
            Case {
                entry: 0x00f9_e0e0,
                preset: &[],
                rets: &[(0x00fd_f0cc, 0x1111_2222)],
                st0s: &[],
                calls: &[(0x00fd_f0cc, &[0, 0, 0, 0])],
                mem: &[(0x011d_d3b0, 4, 0x1111_2222)],
            },
            Case {
                entry: 0x00f9_ec20,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[
                    (
                        0x00ec_782f,
                        &[0x011d_d9e8, 0x24, 0x14, 0x0083_b770, 0x0083_b7d0],
                    ),
                    (0x00ec_658f, &[0x00fd_8c20]),
                ],
                mem: &[],
            },
            Case {
                entry: 0x00fa_4860,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x0040_1050, &[0x011d_f7a0, 0xc, 5, 0x0068_15c0])],
                mem: &[],
            },
            Case {
                entry: 0x00fa_9220,
                preset: &[
                    (0x011a_9448, 0x1000_0001),
                    (0x011a_944c, 0x2000_0002),
                    (0x011a_9450, 0x3000_0003),
                    (0x011a_9454, 0x4000_0004),
                    (0x011a_9458, 0x5000_0005),
                    (0x011a_945c, 0x6000_0006),
                    (0x011a_9460, 0x7000_0007),
                    (0x011a_9464, 0x8000_0008),
                    (0x011a_9468, 0x9000_0009),
                ],
                rets: &[],
                st0s: &[],
                calls: &[],
                mem: &[
                    (0x011e_09ec, 4, 0x1000_0001),
                    (0x011e_09f0, 4, 0x2000_0002),
                    (0x011e_09f4, 4, 0x3000_0003),
                    (0x011e_09f8, 4, 0x4000_0004),
                    (0x011e_09fc, 4, 0x5000_0005),
                    (0x011e_0a00, 4, 0x6000_0006),
                    (0x011e_0a04, 4, 0x7000_0007),
                    (0x011e_0a08, 4, 0x8000_0008),
                    (0x011e_0a0c, 4, 0x9000_0009),
                ],
            },
            Case {
                entry: 0x00fb_31b0,
                preset: &[(0x0109_3f3c, 0x3fa0_0000), (0x0109_3f40, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_32e8, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_32b0,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[
                    (0x00a0_d710, &[0x011f_3308, 0x25]),
                    (0x00ec_658f, &[0x00fd_a020]),
                ],
                mem: &[(0x011f_3308, 4, 0x0109_4e3c)],
            },
            Case {
                entry: 0x00fb_32e0,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00ec_658f, &[0x00fd_a0a0])],
                mem: &[
                    (0x011f_3320, 4, 0),
                    (0x011f_3318, 4, 0),
                    (0x011f_331c, 4, 0),
                ],
            },
            Case {
                entry: 0x00fb_3310,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00ec_658f, &[0x00fd_a0f0])],
                mem: &[
                    (0x011f_332c, 4, 0),
                    (0x011f_3324, 4, 0),
                    (0x011f_3328, 4, 0),
                ],
            },
            Case {
                entry: 0x00fb_3340,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00fd_f054, &[0x011f_3330]), (0x00ec_658f, &[0x00fd_a140])],
                mem: &[],
            },
            Case {
                entry: 0x00fb_3360,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00ec_658f, &[0x00fd_a150])],
                mem: &[(0x011f_3348, 4, 0), (0x011f_334c, 4, 0)],
            },
            Case {
                entry: 0x00fb_3390,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00ec_658f, &[0x00fd_a170])],
                mem: &[(0x011f_3350, 4, 0), (0x011f_3354, 4, 0)],
            },
            Case {
                entry: 0x00fb_33c0,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[
                    (0x00a0_d8e0, &[0x011f_3358, 0x25]),
                    (0x00ec_658f, &[0x00fd_a190]),
                ],
                mem: &[(0x011f_3358, 4, 0x0109_4e7c)],
            },
            Case {
                entry: 0x00fb_34d0,
                preset: &[(0x0109_4ef0, 0x3fa0_0000), (0x0109_4ef4, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_33b4, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_3700,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00fd_f054, &[0x011f_33dc]), (0x00ec_658f, &[0x00fd_a3b0])],
                mem: &[],
            },
            Case {
                entry: 0x00fb_3760,
                preset: &[(0x0109_50a0, 0x3fa0_0000), (0x0109_50a4, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_340c, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_3d30,
                preset: &[(0x0109_5474, 0x3fa0_0000), (0x0109_5478, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_34d8, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_3e60,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[
                    (0x00a1_eeb0, &[0x011f_3508, 0, 1]),
                    (0x00ec_658f, &[0x00fd_a800]),
                ],
                mem: &[(0x011f_3508, 4, 0x0109_5684)],
            },
            Case {
                entry: 0x00fb_3ed0,
                preset: &[(0x0109_5694, 0x3fa0_0000), (0x0109_5698, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_3520, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_3fa0,
                preset: &[(0x0109_56c0, 0x3fa0_0000), (0x0109_56c4, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_3530, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_4070,
                preset: &[(0x0109_56cc, 0x3fa0_0000), (0x0109_56d0, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_3544, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_4140,
                preset: &[(0x0109_5730, 0x3fa0_0000), (0x0109_5734, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_3558, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_4230,
                preset: &[(0x0109_5744, 0x3fa0_0000), (0x0109_5748, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_3588, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_4300,
                preset: &[(0x0109_57ac, 0x3fa0_0000), (0x0109_57b0, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_3598, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_6730,
                preset: &[(0x010a_2c08, 0x3fa0_0000), (0x010a_2c0c, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_6c1c, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_6800,
                preset: &[(0x010a_2c54, 0x3fa0_0000), (0x010a_2c58, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_6c2c, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_68d0,
                preset: &[(0x010a_2d1c, 0x3fa0_0000), (0x010a_2d20, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_6c3c, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_69a0,
                preset: &[(0x010a_2d34, 0x3fa0_0000), (0x010a_2d38, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_6c4c, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_6a70,
                preset: &[(0x010a_2d4c, 0x3fa0_0000), (0x010a_2d50, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_6c5c, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_6b40,
                preset: &[(0x010a_2d90, 0x3fa0_0000), (0x010a_2d94, 0x4040_0000)],
                rets: &[],
                st0s: &[(0x00ec_6054, 1.25)],
                calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
                mem: &[(0x011f_6c70, 4, 0x4019_999a)],
            },
            Case {
                entry: 0x00fb_7080,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00ec_658f, &[0x00fd_b640])],
                mem: &[
                    (0x011f_6df0, 4, 0xffff_ffff),
                    (0x011f_6df4, 1, 0),
                    (0x011f_6df8, 4, 0),
                ],
            },
            Case {
                entry: 0x00fb_85c0,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00fd_f054, &[0x011f_8170]), (0x00ec_658f, &[0x00fd_c370])],
                mem: &[],
            },
            Case {
                entry: 0x00fb_8740,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00b0_dce0, &[0x011f_8bdc]), (0x00ec_658f, &[0x00fd_c410])],
                mem: &[],
            },
            Case {
                entry: 0x00fb_8770,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00fd_f054, &[0x011f_8bc4]), (0x00ec_658f, &[0x00fd_c480])],
                mem: &[],
            },
            Case {
                entry: 0x00fb_8790,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[
                    (0x0044_a730, &[0x011f_8ba4, 0, 0]),
                    (0x00ec_658f, &[0x00fd_c490]),
                ],
                mem: &[],
            },
            Case {
                entry: 0x00fb_87c0,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[
                    (0x0044_a730, &[0x011f_8b84, 0, 0]),
                    (0x00ec_658f, &[0x00fd_c4b0]),
                ],
                mem: &[],
            },
            Case {
                entry: 0x00fb_88c0,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00b1_7db0, &[0x011f_8c68]), (0x00ec_658f, &[0x00fd_c4d0])],
                mem: &[],
            },
            Case {
                entry: 0x00fb_88f0,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00b1_7db0, &[0x011f_8c18]), (0x00ec_658f, &[0x00fd_c530])],
                mem: &[],
            },
            Case {
                entry: 0x00fb_8920,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00b1_7db0, &[0x011f_8c50]), (0x00ec_658f, &[0x00fd_c590])],
                mem: &[],
            },
            Case {
                entry: 0x00fb_8950,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00b1_7db0, &[0x011f_8c80]), (0x00ec_658f, &[0x00fd_c5f0])],
                mem: &[],
            },
            Case {
                entry: 0x00fb_8cc0,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[
                    (0x00b0_7f20, &[0x011f_8d84, 4]),
                    (0x00ec_658f, &[0x00fd_c660]),
                ],
                mem: &[(0x011f_8d84, 2, 0), (0x011f_8d88, 4, 0)],
            },
            Case {
                entry: 0x00fb_92a0,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00ec_658f, &[0x00fd_c720])],
                mem: &[(0x011f_8ebc, 4, 0x010a_58a0)],
            },
            Case {
                entry: 0x00fb_b7c0,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00fd_f054, &[0x0120_2d78]), (0x00ec_658f, &[0x00fd_d430])],
                mem: &[],
            },
            Case {
                entry: 0x00fb_bb00,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[
                    (0x00c4_6530, &[0x0120_2e20, 0, 0x3e8]),
                    (0x00ec_658f, &[0x00fd_d640]),
                ],
                mem: &[(0x0120_2e20, 4, 0x010c_1c54)],
            },
            Case {
                entry: 0x00fb_c7c0,
                preset: &[],
                rets: &[],
                st0s: &[],
                calls: &[(0x00fd_f054, &[0x0120_3328]), (0x00ec_658f, &[0x00fd_dcd0])],
                mem: &[],
            },
        ];
        assert_eq!(cases.len(), 57);
        for c in cases {
            check(c);
        }
    }

    #[test]
    fn t_00fb31f0() {
        check(&Case {
            entry: 0x00fb_31f0,
            preset: &[
                (0x0109_3f3c, 0x3fa0_0000),
                (0x0109_3f40, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_32ec, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb3250() {
        check(&Case {
            entry: 0x00fb_3250,
            preset: &[],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x00a0_d5f0, &[0x011f_32f4, 0x101]),
                (0x00ec_658f, &[0x00fd_a000]),
            ],
            mem: &[(0x011f_32f4, 4, 0x0109_4d9c), (0x011f_3304, 1, 1)],
        });
    }

    #[test]
    fn t_00fb3510() {
        check(&Case {
            entry: 0x00fb_3510,
            preset: &[
                (0x0109_4ef0, 0x3fa0_0000),
                (0x0109_4ef4, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_33b8, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb37a0() {
        check(&Case {
            entry: 0x00fb_37a0,
            preset: &[
                (0x0109_50a0, 0x3fa0_0000),
                (0x0109_50a4, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_3410, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb3d70() {
        check(&Case {
            entry: 0x00fb_3d70,
            preset: &[
                (0x0109_5474, 0x3fa0_0000),
                (0x0109_5478, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_34dc, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb3dc0() {
        check(&Case {
            entry: 0x00fb_3dc0,
            preset: &[],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x00a1_eaa0, &[0x011f_34e0, 0x25]),
                (0x00ec_658f, &[0x00fd_a7c0]),
            ],
            mem: &[(0x011f_34e0, 4, 0x0109_5584), (0x011f_34f0, 1, 1)],
        });
    }

    #[test]
    fn t_00fb3e10() {
        check(&Case {
            entry: 0x00fb_3e10,
            preset: &[],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x00a1_ede0, &[0x011f_34f4, 0x25]),
                (0x00ec_658f, &[0x00fd_a7e0]),
            ],
            mem: &[(0x011f_34f4, 4, 0x0109_5604), (0x011f_3504, 1, 1)],
        });
    }

    #[test]
    fn t_00fb3f10() {
        check(&Case {
            entry: 0x00fb_3f10,
            preset: &[
                (0x0109_5694, 0x3fa0_0000),
                (0x0109_5698, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_3524, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb3fe0() {
        check(&Case {
            entry: 0x00fb_3fe0,
            preset: &[
                (0x0109_56c0, 0x3fa0_0000),
                (0x0109_56c4, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_3534, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb40b0() {
        check(&Case {
            entry: 0x00fb_40b0,
            preset: &[
                (0x0109_56cc, 0x3fa0_0000),
                (0x0109_56d0, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_3548, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb4180() {
        check(&Case {
            entry: 0x00fb_4180,
            preset: &[
                (0x0109_5730, 0x3fa0_0000),
                (0x0109_5734, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_355c, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb4270() {
        check(&Case {
            entry: 0x00fb_4270,
            preset: &[
                (0x0109_5744, 0x3fa0_0000),
                (0x0109_5748, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_358c, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb4340() {
        check(&Case {
            entry: 0x00fb_4340,
            preset: &[
                (0x0109_57ac, 0x3fa0_0000),
                (0x0109_57b0, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_359c, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb4390() {
        check(&Case {
            entry: 0x00fb_4390,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_35d0]),
                (0x00ec_658f, &[0x00fd_a830]),
            ],
            mem: &[
                (0x011f_35d0, 4, 0x0101_7720),
                (0x011f_35d4, 1, 1),
                (0x011f_35d8, 4, 0x0109_589c),
            ],
        });
    }

    #[test]
    fn t_00fb4430() {
        check(&Case {
            entry: 0x00fb_4430,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_35dc]),
                (0x00ec_658f, &[0x00fd_a8b0]),
            ],
            mem: &[
                (0x011f_35dc, 4, 0x0101_7720),
                (0x011f_35e0, 1, 1),
                (0x011f_35e4, 4, 0x0109_58b4),
            ],
        });
    }

    #[test]
    fn t_00fb44d0() {
        check(&Case {
            entry: 0x00fb_44d0,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_35e8]),
                (0x00ec_658f, &[0x00fd_a930]),
            ],
            mem: &[
                (0x011f_35e8, 4, 0x0101_7720),
                (0x011f_35ec, 1, 0),
                (0x011f_35f0, 4, 0x0109_58d4),
            ],
        });
    }

    #[test]
    fn t_00fb4570() {
        check(&Case {
            entry: 0x00fb_4570,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
                (0x0101_e570, 0x7fa0_0001),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_35f4]),
                (0x00ec_658f, &[0x00fd_a9b0]),
            ],
            mem: &[
                (0x011f_35f4, 4, 0x0101_7720),
                (0x011f_35fc, 4, 0x0109_58f0),
                (0x011f_35f8, 4, 0x7fe0_0001),
            ],
        });
    }

    #[test]
    fn t_00fb4610() {
        check(&Case {
            entry: 0x00fb_4610,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
                (0x0101_df44, 0x7fa0_0001),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_3600]),
                (0x00ec_658f, &[0x00fd_aa30]),
            ],
            mem: &[
                (0x011f_3600, 4, 0x0101_7720),
                (0x011f_3608, 4, 0x0109_5910),
                (0x011f_3604, 4, 0x7fe0_0001),
            ],
        });
    }

    #[test]
    fn t_00fb46b0() {
        check(&Case {
            entry: 0x00fb_46b0,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
                (0x0101_b268, 0x7fa0_0001),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_360c]),
                (0x00ec_658f, &[0x00fd_aab0]),
            ],
            mem: &[
                (0x011f_360c, 4, 0x0101_7720),
                (0x011f_3614, 4, 0x0109_5930),
                (0x011f_3610, 4, 0x7fe0_0001),
            ],
        });
    }

    #[test]
    fn t_00fb4750() {
        check(&Case {
            entry: 0x00fb_4750,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
                (0x0101_df44, 0x7fa0_0001),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_3618]),
                (0x00ec_658f, &[0x00fd_ab30]),
            ],
            mem: &[
                (0x011f_3618, 4, 0x0101_7720),
                (0x011f_3620, 4, 0x0109_5954),
                (0x011f_361c, 4, 0x7fe0_0001),
            ],
        });
    }

    #[test]
    fn t_00fb47f0() {
        check(&Case {
            entry: 0x00fb_47f0,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_3624]),
                (0x00ec_658f, &[0x00fd_abb0]),
            ],
            mem: &[
                (0x011f_3624, 4, 0x0101_7720),
                (0x011f_3628, 4, 0),
                (0x011f_362c, 4, 0x0109_5978),
            ],
        });
    }

    #[test]
    fn t_00fb4890() {
        check(&Case {
            entry: 0x00fb_4890,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_3638]),
                (0x00ec_658f, &[0x00fd_ac30]),
            ],
            mem: &[
                (0x011f_3638, 4, 0x0101_7720),
                (0x011f_363c, 4, 0x0009_27c0),
                (0x011f_3640, 4, 0x0109_59a4),
            ],
        });
    }

    #[test]
    fn t_00fb4930() {
        check(&Case {
            entry: 0x00fb_4930,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_3644]),
                (0x00ec_658f, &[0x00fd_acb0]),
            ],
            mem: &[
                (0x011f_3644, 4, 0x0101_7720),
                (0x011f_3648, 1, 1),
                (0x011f_364c, 4, 0x0109_59c8),
            ],
        });
    }

    #[test]
    fn t_00fb6770() {
        check(&Case {
            entry: 0x00fb_6770,
            preset: &[
                (0x010a_2c08, 0x3fa0_0000),
                (0x010a_2c0c, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_6c18, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb6840() {
        check(&Case {
            entry: 0x00fb_6840,
            preset: &[
                (0x010a_2c54, 0x3fa0_0000),
                (0x010a_2c58, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_6c28, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb6910() {
        check(&Case {
            entry: 0x00fb_6910,
            preset: &[
                (0x010a_2d1c, 0x3fa0_0000),
                (0x010a_2d20, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_6c38, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb69e0() {
        check(&Case {
            entry: 0x00fb_69e0,
            preset: &[
                (0x010a_2d34, 0x3fa0_0000),
                (0x010a_2d38, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_6c48, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb6ab0() {
        check(&Case {
            entry: 0x00fb_6ab0,
            preset: &[
                (0x010a_2d4c, 0x3fa0_0000),
                (0x010a_2d50, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_6c58, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb6b80() {
        check(&Case {
            entry: 0x00fb_6b80,
            preset: &[
                (0x010a_2d90, 0x3fa0_0000),
                (0x010a_2d94, 0x4040_0000),
                (0x0101_1598, 0),
                (0x0101_159c, 0x4000_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_6054, 1.25)],
            calls: &[(0x00ec_6054, &[0, 0x3ff4_0000])],
            mem: &[(0x011f_6c6c, 4, 0x4075_c28f)],
        });
    }

    #[test]
    fn t_00fb6bd0() {
        check(&Case {
            entry: 0x00fb_6bd0,
            preset: &[
                (0x0101_1590, 0),
                (0x0101_1594, 0x4000_0000),
                (0x010a_2f48, 0),
                (0x010a_2f4c, 0x3fe0_0000),
            ],
            rets: &[],
            st0s: &[(0x00ec_a920, 0.25)],
            calls: &[(0x00ec_a920, &[0, 0x3ff0_0000])],
            mem: &[(0x011f_6c80, 4, 0), (0x011f_6c84, 4, 0xbfc0_0000)],
        });
    }

    #[test]
    fn t_00fb7c90() {
        check(&Case {
            entry: 0x00fb_7c90,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_7130]),
                (0x00ec_658f, &[0x00fd_bff0]),
            ],
            mem: &[
                (0x011f_7130, 4, 0x0101_7720),
                (0x011f_7134, 1, 0),
                (0x011f_7138, 4, 0x010a_41a0),
            ],
        });
    }

    #[test]
    fn t_00fb7d30() {
        check(&Case {
            entry: 0x00fb_7d30,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_713c]),
                (0x00ec_658f, &[0x00fd_c070]),
            ],
            mem: &[
                (0x011f_713c, 4, 0x0101_7720),
                (0x011f_7140, 4, 0x80),
                (0x011f_7144, 4, 0x010a_41c0),
            ],
        });
    }

    #[test]
    fn t_00fb7dd0() {
        check(&Case {
            entry: 0x00fb_7dd0,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_7148]),
                (0x00ec_658f, &[0x00fd_c0f0]),
            ],
            mem: &[
                (0x011f_7148, 4, 0x0101_7720),
                (0x011f_714c, 4, 0x8000),
                (0x011f_7150, 4, 0x010a_41d0),
            ],
        });
    }

    #[test]
    fn t_00fb7e70() {
        check(&Case {
            entry: 0x00fb_7e70,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
                (0x010a_41f0, 0x7fa0_0001),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_7154]),
                (0x00ec_658f, &[0x00fd_c170]),
            ],
            mem: &[
                (0x011f_7154, 4, 0x0101_7720),
                (0x011f_715c, 4, 0x010a_41f4),
                (0x011f_7158, 4, 0x7fe0_0001),
            ],
        });
    }

    #[test]
    fn t_00fb7f10() {
        check(&Case {
            entry: 0x00fb_7f10,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
                (0x0105_3114, 0x7fa0_0001),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x011f_7160]),
                (0x00ec_658f, &[0x00fd_c1f0]),
            ],
            mem: &[
                (0x011f_7160, 4, 0x0101_7720),
                (0x011f_7168, 4, 0x010a_4210),
                (0x011f_7164, 4, 0x7fe0_0001),
            ],
        });
    }

    #[test]
    fn t_00fb8060() {
        check(&Case {
            entry: 0x00fb_8060,
            preset: &[(0x011f_7184, 0xa5a5_a5a5), (0x011f_7188, 0xa5a5_a5a5)],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[(0x011f_7184, 4, 0), (0x011f_7188, 4, 0)],
        });
    }

    #[test]
    fn t_00fb80d0() {
        check(&Case {
            entry: 0x00fb_80d0,
            preset: &[(0x011f_718c, 0xa5a5_a5a5), (0x011f_7190, 0xa5a5_a5a5)],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[(0x011f_718c, 4, 0), (0x011f_7190, 4, 0x0100_0000)],
        });
    }

    #[test]
    fn t_00fb8140() {
        check(&Case {
            entry: 0x00fb_8140,
            preset: &[(0x011f_7194, 0xa5a5_a5a5), (0x011f_7198, 0xa5a5_a5a5)],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[(0x011f_7194, 4, 1), (0x011f_7198, 4, 0)],
        });
    }

    #[test]
    fn t_00fb81b0() {
        check(&Case {
            entry: 0x00fb_81b0,
            preset: &[(0x011f_719c, 0xa5a5_a5a5), (0x011f_71a0, 0xa5a5_a5a5)],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[(0x011f_719c, 4, 2), (0x011f_71a0, 4, 0)],
        });
    }

    #[test]
    fn t_00fb8230() {
        check(&Case {
            entry: 0x00fb_8230,
            preset: &[(0x011f_7250, 0xa5a5_a5a5), (0x011f_7254, 0xa5a5_a5a5)],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[(0x011f_7250, 4, 0), (0x011f_7254, 4, 0)],
        });
    }

    #[test]
    fn t_00fb82a0() {
        check(&Case {
            entry: 0x00fb_82a0,
            preset: &[(0x011f_7258, 0xa5a5_a5a5), (0x011f_725c, 0xa5a5_a5a5)],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[(0x011f_7258, 4, 0), (0x011f_725c, 4, 0x0100_0000)],
        });
    }

    #[test]
    fn t_00fb8310() {
        check(&Case {
            entry: 0x00fb_8310,
            preset: &[(0x011f_7260, 0xa5a5_a5a5), (0x011f_7264, 0xa5a5_a5a5)],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[(0x011f_7260, 4, 1), (0x011f_7264, 4, 0)],
        });
    }

    #[test]
    fn t_00fb8380() {
        check(&Case {
            entry: 0x00fb_8380,
            preset: &[(0x011f_7268, 0xa5a5_a5a5), (0x011f_726c, 0xa5a5_a5a5)],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[(0x011f_7268, 4, 2), (0x011f_726c, 4, 0)],
        });
    }

    #[test]
    fn t_00fb8400() {
        check(&Case {
            entry: 0x00fb_8400,
            preset: &[(0x011f_727c, 0xa5a5_a5a5), (0x011f_7280, 0xa5a5_a5a5)],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[(0x011f_727c, 4, 0), (0x011f_7280, 4, 0)],
        });
    }

    #[test]
    fn t_00fb8470() {
        check(&Case {
            entry: 0x00fb_8470,
            preset: &[(0x011f_7284, 0xa5a5_a5a5), (0x011f_7288, 0xa5a5_a5a5)],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[(0x011f_7284, 4, 0), (0x011f_7288, 4, 0x0100_0000)],
        });
    }

    #[test]
    fn t_00fb84e0() {
        check(&Case {
            entry: 0x00fb_84e0,
            preset: &[(0x011f_728c, 0xa5a5_a5a5), (0x011f_7290, 0xa5a5_a5a5)],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[(0x011f_728c, 4, 1), (0x011f_7290, 4, 0)],
        });
    }

    #[test]
    fn t_00fb8550() {
        check(&Case {
            entry: 0x00fb_8550,
            preset: &[(0x011f_7294, 0xa5a5_a5a5), (0x011f_7298, 0xa5a5_a5a5)],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[(0x011f_7294, 4, 2), (0x011f_7298, 4, 0)],
        });
    }

    #[test]
    fn t_00fb85e0() {
        check(&Case {
            entry: 0x00fb_85e0,
            preset: &[],
            rets: &[],
            st0s: &[],
            calls: &[(0x00ec_658f, &[0x00fd_c380])],
            mem: &[(0x011f_8188, 4, 0)],
        });
    }

    #[test]
    fn t_00fb8630() {
        check(&Case {
            entry: 0x00fb_8630,
            preset: &[],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x00ec_782f, &[0x011f_8194, 4, 8, 0x00af_d230, 0x00af_c720]),
                (0x00ec_782f, &[0x011f_81b4, 4, 9, 0x00af_d230, 0x00af_c720]),
                (0x00ec_658f, &[0x00fd_c3c0]),
            ],
            mem: &[(0x011f_8190, 4, 0)],
        });
    }

    #[test]
    fn t_00fb8d00() {
        check(&Case {
            entry: 0x00fb_8d00,
            preset: &[
                (0x010a_578c, 0x3fc0_0000),
                (0x0101_2060, 0),
                (0x0101_2064, 0x3ffc_0000),
                (0x0101_5a38, 0),
                (0x0101_5a3c, 0x401a_0000),
                (0x0101_5a40, 0),
                (0x0101_5a44, 0x4018_0000),
                (0x0101_db80, 0),
                (0x0101_db84, 0x4008_0000),
                (0x0102_0758, 0),
                (0x0102_075c, 0x4002_0000),
                (0x0102_1928, 0),
                (0x0102_192c, 0x4015_0000),
                (0x0102_40b0, 0),
                (0x0102_40b4, 0x4006_0000),
                (0x0102_40b8, 0),
                (0x0102_40bc, 0x3fe8_0000),
                (0x0102_e438, 0),
                (0x0102_e43c, 0x4012_0000),
                (0x0103_4208, 0),
                (0x0103_420c, 0x3fd0_0000),
                (0x0103_56f8, 0),
                (0x0103_56fc, 0x4013_0000),
                (0x0103_5700, 0),
                (0x0103_5704, 0x400c_0000),
                (0x0103_5708, 0),
                (0x0103_570c, 0x4011_0000),
                (0x0106_5770, 0),
                (0x0106_5774, 0x3fe0_0000),
                (0x0106_d4c8, 0),
                (0x0106_d4cc, 0x400a_0000),
                (0x0107_2f80, 0),
                (0x0107_2f84, 0x4019_0000),
                (0x0107_2fd0, 0),
                (0x0107_2fd4, 0x3ff8_0000),
                (0x010a_5790, 0),
                (0x010a_5794, 0x4017_0000),
                (0x010a_5798, 0),
                (0x010a_579c, 0x4016_0000),
                (0x010a_57a0, 0),
                (0x010a_57a4, 0x4014_0000),
                (0x010a_57a8, 0),
                (0x010a_57ac, 0x4010_0000),
                (0x010a_57b0, 0),
                (0x010a_57b4, 0x400e_0000),
                (0x010a_57b8, 0),
                (0x010a_57bc, 0x4004_0000),
                (0x010a_57c0, 0),
                (0x010a_57c4, 0x4000_0000),
                (0x010a_57c8, 0),
                (0x010a_57cc, 0x3ff4_0000),
                (0x010a_57d0, 0),
                (0x010a_57d4, 0x3ff0_0000),
            ],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[
                (0x011f_8d98, 4, 0x3ec0_0000),
                (0x011f_8d9c, 4, 0x3f40_0000),
                (0x011f_8da0, 4, 0x3f90_0000),
                (0x011f_8da4, 4, 0x3fc0_0000),
                (0x011f_8da8, 4, 0x4040_0000),
                (0x011f_8dac, 4, 0x3ff0_0000),
                (0x011f_8db0, 4, 0x4010_0000),
                (0x011f_8db4, 4, 0x4028_0000),
                (0x011f_8db8, 4, 0x4040_0000),
                (0x011f_8dbc, 4, 0x4058_0000),
                (0x011f_8dc0, 4, 0x4070_0000),
                (0x011f_8dc4, 4, 0x4084_0000),
                (0x011f_8dc8, 4, 0x4090_0000),
                (0x011f_8dcc, 4, 0x409c_0000),
                (0x011f_8dd0, 4, 0x40a8_0000),
                (0x011f_8dd4, 4, 0x40b4_0000),
                (0x011f_8dd8, 4, 0x40a8_0000),
                (0x011f_8ddc, 4, 0x40c0_0000),
                (0x011f_8de0, 4, 0x4090_0000),
                (0x011f_8de4, 4, 0x3ff0_0000),
                (0x011f_8de8, 4, 0x40cc_0000),
                (0x011f_8dec, 4, 0x40d8_0000),
                (0x011f_8df0, 4, 0x40e4_0000),
                (0x011f_8df4, 4, 0x40f0_0000),
                (0x011f_8df8, 4, 0x40fc_0000),
                (0x011f_8dfc, 4, 0x4104_0000),
                (0x011f_8e00, 4, 0x410a_0000),
                (0x011f_8e04, 4, 0x4110_0000),
                (0x011f_8e08, 4, 0x4116_0000),
                (0x011f_8e0c, 4, 0x411c_0000),
            ],
        });
    }

    #[test]
    fn t_00fbb7e0() {
        check(&Case {
            entry: 0x00fb_b7e0,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_2da8]),
                (0x00ec_658f, &[0x00fd_d440]),
            ],
            mem: &[
                (0x0120_2da8, 4, 0x0101_7720),
                (0x0120_2dac, 4, 5),
                (0x0120_2db0, 4, 0x010c_1774),
            ],
        });
    }

    #[test]
    fn t_00fbb880() {
        check(&Case {
            entry: 0x00fb_b880,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_2db4]),
                (0x00ec_658f, &[0x00fd_d4c0]),
            ],
            mem: &[
                (0x0120_2db4, 4, 0x0101_7720),
                (0x0120_2db8, 4, 0x14),
                (0x0120_2dbc, 4, 0x010c_179c),
            ],
        });
    }

    #[test]
    fn t_00fbb920() {
        check(&Case {
            entry: 0x00fb_b920,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_2dc0]),
                (0x00ec_658f, &[0x00fd_d540]),
            ],
            mem: &[
                (0x0120_2dc0, 4, 0x0101_7720),
                (0x0120_2dc4, 4, 0x32),
                (0x0120_2dc8, 4, 0x010c_17dc),
            ],
        });
    }

    #[test]
    fn t_00fbb9c0() {
        check(&Case {
            entry: 0x00fb_b9c0,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_2dcc]),
                (0x00ec_658f, &[0x00fd_d5c0]),
            ],
            mem: &[
                (0x0120_2dcc, 4, 0x0101_7720),
                (0x0120_2dd0, 4, 0x14),
                (0x0120_2dd4, 4, 0x010c_180c),
            ],
        });
    }

    #[test]
    fn t_00fbbda0() {
        check(&Case {
            entry: 0x00fb_bda0,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
                (0x010c_281c, 0x7fa0_0001),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_3120]),
                (0x00ec_658f, &[0x00fd_d6d0]),
            ],
            mem: &[
                (0x0120_3120, 4, 0x0101_7720),
                (0x0120_3128, 4, 0x010c_2820),
                (0x0120_3124, 4, 0x7fe0_0001),
            ],
        });
    }

    #[test]
    fn t_00fbbe40() {
        check(&Case {
            entry: 0x00fb_be40,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
                (0x010c_2840, 0x7fa0_0001),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_312c]),
                (0x00ec_658f, &[0x00fd_d750]),
            ],
            mem: &[
                (0x0120_312c, 4, 0x0101_7720),
                (0x0120_3134, 4, 0x010c_2844),
                (0x0120_3130, 4, 0x7fe0_0001),
            ],
        });
    }

    #[test]
    fn t_00fbbee0() {
        check(&Case {
            entry: 0x00fb_bee0,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_3138]),
                (0x00ec_658f, &[0x00fd_d7d0]),
            ],
            mem: &[
                (0x0120_3138, 4, 0x0101_7720),
                (0x0120_313c, 4, 0x3f80_0000),
                (0x0120_3140, 4, 0x010c_2864),
            ],
        });
    }

    #[test]
    fn t_00fbbf80() {
        check(&Case {
            entry: 0x00fb_bf80,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
                (0x0101_712c, 0x7fa0_0001),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_3144]),
                (0x00ec_658f, &[0x00fd_d850]),
            ],
            mem: &[
                (0x0120_3144, 4, 0x0101_7720),
                (0x0120_314c, 4, 0x010c_2884),
                (0x0120_3148, 4, 0x7fe0_0001),
            ],
        });
    }

    #[test]
    fn t_00fbc020() {
        check(&Case {
            entry: 0x00fb_c020,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
                (0x0102_f0f8, 0x7fa0_0001),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_3150]),
                (0x00ec_658f, &[0x00fd_d8d0]),
            ],
            mem: &[
                (0x0120_3150, 4, 0x0101_7720),
                (0x0120_3158, 4, 0x010c_289c),
                (0x0120_3154, 4, 0x7fe0_0001),
            ],
        });
    }

    #[test]
    fn t_00fbc0c0() {
        check(&Case {
            entry: 0x00fb_c0c0,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
                (0x0102_f0f8, 0x7fa0_0001),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_315c]),
                (0x00ec_658f, &[0x00fd_d950]),
            ],
            mem: &[
                (0x0120_315c, 4, 0x0101_7720),
                (0x0120_3164, 4, 0x010c_28b0),
                (0x0120_3160, 4, 0x7fe0_0001),
            ],
        });
    }

    #[test]
    fn t_00fbc160() {
        check(&Case {
            entry: 0x00fb_c160,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
                (0x0102_f0f8, 0x7fa0_0001),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_3168]),
                (0x00ec_658f, &[0x00fd_d9d0]),
            ],
            mem: &[
                (0x0120_3168, 4, 0x0101_7720),
                (0x0120_3170, 4, 0x010c_28cc),
                (0x0120_316c, 4, 0x7fe0_0001),
            ],
        });
    }

    #[test]
    fn t_00fbc200() {
        check(&Case {
            entry: 0x00fb_c200,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
                (0x010c_28ec, 0x7fa0_0001),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_3174]),
                (0x00ec_658f, &[0x00fd_da50]),
            ],
            mem: &[
                (0x0120_3174, 4, 0x0101_7720),
                (0x0120_317c, 4, 0x010c_28f0),
                (0x0120_3178, 4, 0x7fe0_0001),
            ],
        });
    }

    #[test]
    fn t_00fbc2c0() {
        check(&Case {
            entry: 0x00fb_c2c0,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_3188]),
                (0x00ec_658f, &[0x00fd_dad0]),
            ],
            mem: &[
                (0x0120_3188, 4, 0x0101_7720),
                (0x0120_318c, 1, 0),
                (0x0120_3190, 4, 0x010c_291c),
            ],
        });
    }

    #[test]
    fn t_00fbc360() {
        check(&Case {
            entry: 0x00fb_c360,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_3194]),
                (0x00ec_658f, &[0x00fd_db50]),
            ],
            mem: &[
                (0x0120_3194, 4, 0x0101_7720),
                (0x0120_3198, 4, 0x14),
                (0x0120_319c, 4, 0x010c_2938),
            ],
        });
    }

    #[test]
    fn t_00fbc400() {
        check(&Case {
            entry: 0x00fb_c400,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_31a0]),
                (0x00ec_658f, &[0x00fd_dbd0]),
            ],
            mem: &[
                (0x0120_31a0, 4, 0x0101_7720),
                (0x0120_31a4, 4, 0x28),
                (0x0120_31a8, 4, 0x010c_2958),
            ],
        });
    }

    #[test]
    fn t_00fbc4a0() {
        check(&Case {
            entry: 0x00fb_c4a0,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0120_31ac]),
                (0x00ec_658f, &[0x00fd_dc50]),
            ],
            mem: &[
                (0x0120_31ac, 4, 0x0101_7720),
                (0x0120_31b0, 4, 0x7d0),
                (0x0120_31b4, 4, 0x010c_2978),
            ],
        });
    }

    #[test]
    fn t_00fbe090() {
        check(&Case {
            entry: 0x00fb_e090,
            preset: &[],
            rets: &[],
            st0s: &[],
            calls: &[],
            mem: &[
                (0x0126_8460, 4, 0),
                (0x0126_8464, 4, 0),
                (0x0126_8468, 4, 0),
                (0x0126_846c, 4, 0),
                (0x0126_8470, 4, 0xffff_ffff),
                (0x0126_8474, 4, 0),
                (0x0126_8478, 4, 0),
                (0x0126_847c, 4, 0),
                (0x0126_8480, 4, 0),
                (0x0126_8484, 4, 0xffff_ffff),
                (0x0126_8488, 4, 0),
                (0x0126_848c, 4, 0),
                (0x0126_8490, 4, 0xffff_ffff),
                (0x0126_8494, 4, 0xffff_ffff),
                (0x0126_8498, 4, 0),
                (0x0126_849c, 4, 0),
                (0x0126_84a0, 4, 0),
                (0x0126_84a4, 4, 0),
                (0x0126_84a8, 4, 0xffff_ffff),
                (0x0126_84ac, 4, 0),
                (0x0126_84b0, 4, 0xffff_ffff),
                (0x0126_84b4, 4, 0),
                (0x0126_84b8, 4, 0xffff_ffff),
                (0x0126_84bc, 4, 0),
                (0x0126_84c0, 4, 0),
                (0x0126_84c4, 4, 0xffff_ffff),
                (0x0126_84c8, 4, 0xffff_ffff),
                (0x0126_84cc, 4, 0),
                (0x0126_84d0, 4, 0xffff_ffff),
                (0x0126_84d4, 4, 0xffff_ffff),
                (0x0126_84d8, 4, 0xffff_ffff),
                (0x0126_84dc, 4, 0),
                (0x0126_84e0, 4, 0),
                (0x0126_84e4, 4, 0),
                (0x0126_84e8, 4, 0),
                (0x0126_84ec, 4, 0xffff_ffff),
                (0x0126_84f0, 4, 0xffff_ffff),
                (0x0126_84f4, 4, 0),
                (0x0126_84f8, 4, 0),
                (0x0126_84fc, 4, 0xffff_ffff),
                (0x0126_8500, 4, 0),
                (0x0126_8504, 4, 0xffff_ffff),
                (0x0126_8508, 4, 0),
                (0x0126_850c, 4, 0xffff_ffff),
                (0x0126_8510, 4, 0xffff_ffff),
                (0x0126_8514, 4, 0xffff_ffff),
                (0x0126_8518, 4, 0),
                (0x0126_851c, 4, 0xffff_ffff),
                (0x0126_8520, 4, 0),
                (0x0126_8524, 4, 0),
                (0x0126_8528, 4, 0xffff_ffff),
                (0x0126_852c, 4, 0xffff_ffff),
                (0x0126_8530, 4, 0xffff_ffff),
                (0x0126_8534, 4, 0),
                (0x0126_8538, 4, 0xffff_ffff),
                (0x0126_853c, 4, 0xffff_ffff),
                (0x0126_8540, 4, 0),
                (0x0126_8544, 4, 0xffff_ffff),
                (0x0126_8548, 4, 0xffff_ffff),
                (0x0126_854c, 4, 0xffff_ffff),
                (0x0126_8550, 4, 0xffff_ffff),
                (0x0126_8554, 4, 0xffff_ffff),
                (0x0126_8558, 4, 0xffff_ffff),
                (0x0126_855c, 4, 0xffff_ffff),
            ],
        });
    }

    #[test]
    fn t_00fc76d0() {
        check(&Case {
            entry: 0x00fc_76d0,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0126_fac8]),
                (0x00ec_658f, &[0x00fd_e3d0]),
            ],
            mem: &[
                (0x0126_fac8, 4, 0x0101_7720),
                (0x0126_facc, 1, 1),
                (0x0126_fad0, 4, 0x010f_1dbc),
            ],
        });
    }

    #[test]
    fn t_00fc7770() {
        check(&Case {
            entry: 0x00fc_7770,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0126_fad4]),
                (0x00ec_658f, &[0x00fd_e450]),
            ],
            mem: &[
                (0x0126_fad4, 4, 0x0101_7720),
                (0x0126_fad8, 4, 0x0190_0000),
                (0x0126_fadc, 4, 0x010f_1ddc),
            ],
        });
    }

    #[test]
    fn t_00fc7810() {
        check(&Case {
            entry: 0x00fc_7810,
            preset: &[
                (0x011f_96a0, 0x0800_0000),
                (0x0800_0000, 0x0900_0000),
                (0x0900_0004, 0x0a00_0010),
            ],
            rets: &[],
            st0s: &[],
            calls: &[
                (0x0044_f570, &[]),
                (0x0a00_0010, &[0x0800_0000, 0x0126_fae0]),
                (0x00ec_658f, &[0x00fd_e4d0]),
            ],
            mem: &[
                (0x0126_fae0, 4, 0x0101_7720),
                (0x0126_fae4, 4, 3),
                (0x0126_fae8, 4, 0x010f_1df8),
            ],
        });
    }

    /// The words of every call to `callee` (arguments, `this` first) a double recorded.
    type Seen = Rc<RefCell<Vec<u32>>>;

    /// A double for `callee` that records its argument words and answers `answer`.
    fn recording(e: &mut Engine, callee: u32, answer: Ret) -> Seen {
        let seen: Seen = Rc::new(RefCell::new(Vec::new()));
        let log = Rc::clone(&seen);
        e.register_double(callee, move |_, a| {
            log.borrow_mut().extend_from_slice(a);
            answer
        });
        seen
    }

    #[test]
    fn t_00fb5f40() {
        // Six floats and a tail jump: the function's answer is the callee's.
        let mut e = Engine::new();
        e.map(0x0101_6248, 4);
        e.map(0x011a_b598, 0x20);
        // A signalling NaN: FLD/FSTP quiets it (the quiet bit set, the payload kept).
        e.mem.set_u32(0x0101_6248, 0x7f80_0001);
        e.mem.set_u32(0x011a_b5a0, 0xdead_beef);
        let seen = recording(
            &mut e,
            0x00a8_8cd0,
            Ret {
                eax: 0x77,
                ..Ret::default()
            },
        );
        assert_eq!(e.call(0x00fb_5f40, &args![]).u32(), 0x77);
        assert_eq!(*seen.borrow(), vec![0x011a_b598]);
        let words: Vec<u32> = (0..6).map(|i| e.mem.u32(0x011a_b598 + 4 * i)).collect();
        assert_eq!(
            words,
            vec![0, 0, 0xdead_beef, 0x3f80_0000, 0x3f80_0000, 0x7fc0_0001]
        );
        assert_eq!(e.mem.u32(0x011a_b5b0), 0x7fc0_0001);
    }

    #[test]
    fn t_00fb8f80() {
        // The addresses of two locals (EBP-2 and EBP-1) go to 00b3b770, then _atexit.
        let mut e = Engine::new();
        quiet_atexit(&mut e);
        let seen = recording(&mut e, 0x00b3_b770, Ret::default());
        e.call(0x00fb_8f80, &args![]);
        let words = seen.borrow().clone();
        assert_eq!(words.len(), 3);
        assert_eq!(words[0], 0x011f_8e2c);
        assert_eq!(words[2], words[1] + 1);
    }

    fn quiet_atexit(e: &mut Engine) {
        e.register(ATEXIT, |_, _| Ret::default());
    }

    #[test]
    fn t_00fb85e0_guarded_increment() {
        // The guard follows a store of 0, so the entry never reaches the increment; the helper
        // does it for a non-null pointer: the count at +4 of the field at +0x158.
        let mut e = Engine::new();
        e.map(0x011f_8188, 4);
        e.mem.set_u32(0x011f_8188, 0x0800_0000);
        let seen = recording(&mut e, INTERLOCKED_INCREMENT, Ret::default());
        increment_count_of_object_at(&mut e, 0x011f_8188);
        assert_eq!(*seen.borrow(), vec![0x0800_015c]);
        e.mem.set_u32(0x011f_8188, 0);
        increment_count_of_object_at(&mut e, 0x011f_8188);
        assert_eq!(seen.borrow().len(), 1);
    }

    #[test]
    fn square_root_initializers_quiet_a_signalling_nan() {
        // 00fb31b0: the float goes through FLD/FSTP, so its signalling NaN reaches sqrt quieted,
        // widened exactly; the root's NaN comes back through a float and divides.
        let mut e = Engine::new();
        for a in [0x0109_3f3c, 0x0109_3f40, 0x011f_32e8] {
            e.map(a, 4);
        }
        e.mem.set_u32(0x0109_3f3c, 0x7f80_0001);
        e.mem.set_u32(0x0109_3f40, 0x3f80_0000);
        let seen = recording(
            &mut e,
            SQRT,
            Ret {
                st0: f64::from_bits(0x7ff8_0000_2000_0000),
                ..Ret::default()
            },
        );
        e.call(0x00fb_31b0, &args![]);
        assert_eq!(*seen.borrow(), vec![0x2000_0000, 0x7ff8_0000]);
        assert_eq!(e.mem.u32(0x011f_32e8), 0x7fc0_0001);
    }

    #[test]
    fn t_00fb6bd0_signalling_nan_constants() {
        // FLD of a signalling NaN double quiets it; the FDIV memory operand stays signalling
        // for the NaN choice and the result is that NaN, quieted.
        let mut e = Engine::new();
        for a in [0x0101_1590, 0x010a_2f48, 0x011f_6c80] {
            e.map(a, 8);
        }
        e.mem.set_u64(0x0101_1590, 0x7ff0_0000_0000_0001);
        e.mem.set_u64(0x010a_2f48, 0x3fe0_0000_0000_0000);
        let seen = recording(
            &mut e,
            0x00ec_a920,
            Ret {
                st0: 0.25,
                ..Ret::default()
            },
        );
        e.call(0x00fb_6bd0, &args![]);
        assert_eq!(*seen.borrow(), vec![1, 0x7ff8_0000]);
        assert_eq!(e.mem.u64(0x011f_6c80), 0x7ff8_0000_0000_0001);
    }

    #[test]
    fn t_00fb8d00_signalling_nan_factor() {
        // FLD float quiets the factor; x87 multiplication keeps that NaN.
        let mut e = Engine::new();
        e.map(0x010a_578c, 4);
        e.map(0x0103_4208, 8);
        e.map(0x011f_8d98, 4);
        e.mem.set_u32(0x010a_578c, 0x7f80_0001);
        float_times_double(&mut e, 0x010a_578c, 0x0103_4208, 0x011f_8d98);
        assert_eq!(e.mem.u32(0x011f_8d98), 0x7fc0_0001);
        float_doubled(&mut e, 0x010a_578c, 0x011f_8d98);
        assert_eq!(e.mem.u32(0x011f_8d98), 0x7fc0_0001);
    }

    #[test]
    fn t_00fb3240_empty() {
        let mut e = Engine::new();
        assert!(run(&mut e, 0x00fb_3240).is_empty());
    }
}
