//! `(unplaced)/atexit_irregular` (no source unit: the compiler's `atexit` destructors), subsystem
//! `compiler: static init/exit`: the 229 `atexit` destructor functions of FalloutNV.exe 1.4.0.525
//! (`00fc7a70` onwards, the engine map's "compiler: atexit destructors") that match neither of
//! the two fixed shapes of `data/atexit_destructors.tsv` (docs/ENGINE_CRATE.md,
//! `research/initgen/README.md`), each translated from its own disassembly.
//!
//! The exit sequence (`units::crt::doexit`) runs them last-registered first. They are cdecl
//! functions with no parameters; what they leave in `EAX` is whatever their last call left, and
//! nobody reads it, so they return nothing. A "vtable" below is an address in the executable's
//! data whose first words are code addresses (checked in Ghidra for each one stored). The shapes:
//!
//! - `eh vector destructor iterator` (`00ec5fce`, `RET 0x10`) on a global array: 31 functions.
//! - `DeleteCriticalSection` on a global critical section: 21; `CloseHandle` or `TlsFree` of a
//!   global handle: 5.
//! - the release of a reference-counted object held by a global (`InterlockedDecrement` of the
//!   count at +4, the virtual destructor in vtable slot +4 at zero): 57; of a string held by a
//!   global (the count 8 bytes before the pointer, skipped for the word at `0109b220`): 4; of
//!   an array of strings: 2, of an array of such objects: 2.
//! - a thiscall on a global (6; with two stack arguments 2), with a vtable stored first (1), or
//!   only the vtable stored (8), or nothing at all (2).
//! - 72 destructors that store a vtable, call a singleton getter, call the singleton's virtual
//!   function at vtable slot +8 and run `00404950` on the global.
//! - single cases: cdecl calls with a global's word (4), an array object freeing its block
//!   through a vtable slot (2), a scalar delete (1), a halfword and pointer cleared around a
//!   free (1), a free after a destructor (5), two vtables stored in turn (2), a release to the
//!   pool object `011f6238` (1).
//!
//! 82 of them sit in an `FS:[0]` exception frame (a handler address and a state number): each
//! is marked `// Translation incomplete` because the frame (the compiler's unwinding on an
//! exception) is not translated; every call in their bodies is.
//!
//! Callees are called by address and never re-implemented; imports through their import slot.

#[allow(unused_imports)]
use crate::prelude::*;

/// `eh vector destructor iterator` (the engine map's name for `00ec5fce`, stdcall `RET 0x10`):
/// `(array, element size, count, element destructor)`, pushed in reverse.
const EH_VECTOR_DESTRUCTOR_ITERATOR: u32 = 0x00ec_5fce;
/// Import slot of KERNEL32 `InterlockedDecrement` (stdcall, one argument).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// Import slot of KERNEL32 `DeleteCriticalSection` (stdcall, one argument).
const DELETE_CRITICAL_SECTION: u32 = 0x00fd_f058;
/// The word the string releases compare a string pointer with (`CMP EAX,dword ptr [0109b220]`).
/// The executable holds 0 there; the translation reads it as the game does.
const STRING_SENTINEL: u32 = 0x0109_b220;
/// Destructor `00404950` the singleton destructors run on their global last (`thiscall`,
/// no stack arguments).
const BASE_DESTRUCTOR_00404950: u32 = 0x0040_4950;

/// `PUSH destructor; PUSH count; PUSH size; PUSH array; CALL 00ec5fce`: runs the element
/// destructor over `count` elements of `size` bytes (`RET 0x10` pops the four words).
fn vector_destructor_iterator(e: &mut Engine, array: u32, size: u32, count: u32, destructor: u32) {
    e.call(
        EH_VECTOR_DESTRUCTOR_ITERATOR,
        &args![array, size, count, destructor],
    );
}

/// `PUSH section; CALL dword ptr [DeleteCriticalSection]`.
fn delete_critical_section(e: &mut Engine, section: u32) {
    e.call(DELETE_CRITICAL_SECTION, &args![section]);
}

/// `MOV EAX,[global]; PUSH EAX; CALL dword ptr [import]`: the import takes the global's value.
fn import_on_global(e: &mut Engine, import: u32, global: u32) {
    let value = e.global::<u32>(global);
    e.call(import, &args![value]);
}

/// Releases the reference-counted object at the pointer in `global` plus `offset`: nothing when
/// the global is null (the test is on the global itself, before `offset` is added);
/// `InterlockedDecrement` of the count at +4 of the object; when that gives 0 (`TEST EAX,EAX;
/// JNZ`), the virtual function in vtable slot +4 (the destructor) with the object as `this`.
fn release_ref(e: &mut Engine, global: u32, offset: u32) {
    let pointer = e.global::<u32>(global);
    if pointer == 0 {
        return;
    }
    let object = pointer.wrapping_add(offset);
    if e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
        .u32()
        == 0
    {
        e.vcall(object, 4, &args![]);
    }
}

/// `MOV EAX,[global]; CMP EAX,[0109b220]; JZ done; ADD EAX,-8; PUSH EAX; CALL InterlockedDecrement`:
/// a string pointer other than the sentinel word has its count 8 bytes before it.
fn release_string(e: &mut Engine, global: u32) {
    let pointer = e.global::<u32>(global);
    release_string_pointer(e, pointer);
}

/// The compare and the decrement of [`release_string`] on a pointer already in a register.
fn release_string_pointer(e: &mut Engine, pointer: u32) {
    if pointer != e.global::<u32>(STRING_SENTINEL) {
        e.call(INTERLOCKED_DECREMENT, &args![pointer.wrapping_sub(8)]);
    }
}

/// The loop over an array of string pointers that ends at `end`: `ESI = end`, `EBX = last`; each
/// round reads the word before `ESI`, moves `ESI` down by 4, releases it as [`release_string`]
/// does, and `SUB EBX,1; JNS` repeats while `EBX` is not negative (`last + 1` rounds).
fn release_strings(e: &mut Engine, end: u32, last: i32) {
    let mut esi = end;
    let mut ebx = last;
    loop {
        let pointer = e.global::<u32>(esi.wrapping_sub(4));
        esi = esi.wrapping_sub(4);
        release_string_pointer(e, pointer);
        ebx = ebx.wrapping_sub(1);
        if ebx < 0 {
            break;
        }
    }
}

/// The loop over an array of reference-counted object pointers that ends at `end` (`EDI = end`,
/// `EBX = last`, `last + 1` rounds, last element first): a non-null pointer gets
/// `InterlockedDecrement` of its count at +4, and at zero its virtual function in vtable slot
/// +4 (the destructor).
fn release_objects(e: &mut Engine, end: u32, last: i32) {
    let mut edi = end;
    let mut ebx = last;
    loop {
        let object = e.global::<u32>(edi.wrapping_sub(4));
        edi = edi.wrapping_sub(4);
        if object != 0
            && e.call(INTERLOCKED_DECREMENT, &args![object.wrapping_add(4)])
                .u32()
                == 0
        {
            e.vcall(object, 4, &args![]);
        }
        ebx = ebx.wrapping_sub(1);
        if ebx < 0 {
            break;
        }
    }
}

/// The body of the 72 destructors of globals with a vtable stored first: `MOV [global],vtable`;
/// `CALL getter` (cdecl, no arguments, result unused: it makes the singleton kept at `singleton`
/// when it is absent); `MOV EAX,[singleton]`; the singleton's virtual function in vtable slot
/// +8 with the global as its one stack argument (`RET 4`); then `00404950` on the global.
/// The `FS:[0]` frame around it (state 0 from the store to the virtual call, -1 after) is the
/// compiler's exception unwinding and is not translated.
fn fs_singleton_destruct(e: &mut Engine, global: u32, vtable: u32, getter: u32, singleton: u32) {
    e.set_global::<u32>(global, vtable);
    e.call(getter, &args![]);
    let loader = e.global::<u32>(singleton);
    e.vcall(loader, 8, &args![global]);
    e.call(BASE_DESTRUCTOR_00404950, &args![global]);
}

// Translated from 00fc9090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011c5d10`: `eh vector destructor iterator` (`00ec5fce`) over 20 elements of 24 bytes with the element destructor `00489070`.
pub fn fn_00fc9090(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011c_5d10, 0x18, 0x14, 0x0048_9070);
}

// Translated from 00fc9a10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011c7804`: `eh vector destructor iterator` (`00ec5fce`) over 3 elements of 4 bytes with the element destructor `0045cec0`.
pub fn fn_00fc9a10(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011c_7804, 0x4, 0x3, 0x0045_cec0);
}

// Translated from 00fc9a60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011c7860`: `eh vector destructor iterator` (`00ec5fce`) over 3 elements of 4 bytes with the element destructor `0045cec0`.
pub fn fn_00fc9a60(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011c_7860, 0x4, 0x3, 0x0045_cec0);
}

// Translated from 00fcb260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011cb3e0`: `eh vector destructor iterator` (`00ec5fce`) over 14 elements of 8 bytes with the element destructor `0046ffb0`.
pub fn fn_00fcb260(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011c_b3e0, 0x8, 0xe, 0x0046_ffb0);
}

// Translated from 00fcb280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011cb450`: `eh vector destructor iterator` (`00ec5fce`) over 14 elements of 8 bytes with the element destructor `0046ffb0`.
pub fn fn_00fcb280(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011c_b450, 0x8, 0xe, 0x0046_ffb0);
}

// Translated from 00fcb8c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011cbf60`: `eh vector destructor iterator` (`00ec5fce`) over 2 elements of 84 bytes with the element destructor `005aa1a0`.
pub fn fn_00fcb8c0(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011c_bf60, 0x54, 0x2, 0x005a_a1a0);
}

// Translated from 00fd7ad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011d8828`: `eh vector destructor iterator` (`00ec5fce`) over 3 elements of 28 bytes with the element destructor `00701500`.
pub fn fn_00fd7ad0(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011d_8828, 0x1c, 0x3, 0x0070_1500);
}

// Translated from 00fd7f30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011da1b0`: `eh vector destructor iterator` (`00ec5fce`) over 5 elements of 12 bytes with the element destructor `0044f4f0`.
pub fn fn_00fd7f30(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011d_a1b0, 0xc, 0x5, 0x0044_f4f0);
}

// Translated from 00fd8c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011dd9e8`: `eh vector destructor iterator` (`00ec5fce`) over 20 elements of 36 bytes with the element destructor `0083b7d0`.
pub fn fn_00fd8c20(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011d_d9e8, 0x24, 0x14, 0x0083_b7d0);
}

// Translated from 00fda000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f32f4`: stores the vtable `01094d9c` in it, then runs `00a0d2a0` on it.
pub fn fn_00fda000(e: &mut Engine) {
    e.set_global::<u32>(0x011f_32f4, 0x0109_4d9c);
    e.call(0x00a0_d2a0, &args![0x011f_32f4]);
}

// Translated from 00fda020 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3308`: stores the vtable `01094e3c` and runs `00ae91f0` on it, stores the vtable `01094e5c` and runs `00ae91f0` on it again, then calls `00aa10f0` (cdecl, one argument) with the word at `011f3310`.
// Translation incomplete: 00fda020 - FS:[0] exception frame (handler 00f1d43a) not translated.
pub fn fn_00fda020(e: &mut Engine) {
    e.set_global::<u32>(0x011f_3308, 0x0109_4e3c);
    e.call(0x00ae_91f0, &args![0x011f_3308]);
    e.set_global::<u32>(0x011f_3308, 0x0109_4e5c);
    e.call(0x00ae_91f0, &args![0x011f_3308]);
    let p = e.global::<u32>(0x011f_3310);
    e.call(0x00aa_10f0, &args![p]);
}

// Translated from 00fda0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3318`: runs `004ed900` on it (`ECX` = the global; no stack arguments).
// Translation incomplete: 00fda0a0 - FS:[0] exception frame (handler 00f1d46a) not translated.
pub fn fn_00fda0a0(e: &mut Engine) {
    e.call(0x004e_d900, &args![0x011f_3318]);
}

// Translated from 00fda0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3324`: runs `004ed900` on it (`ECX` = the global; no stack arguments).
// Translation incomplete: 00fda0f0 - FS:[0] exception frame (handler 00f1d49a) not translated.
pub fn fn_00fda0f0(e: &mut Engine) {
    e.call(0x004e_d900, &args![0x011f_3324]);
}

// Translated from 00fda140 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f3330`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fda140(e: &mut Engine) {
    delete_critical_section(e, 0x011f_3330);
}

// Translated from 00fda150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3348`: runs `00ae83e0` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fda150(e: &mut Engine) {
    e.call(0x00ae_83e0, &args![0x011f_3348]);
}

// Translated from 00fda170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3350`: runs `00ae83e0` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fda170(e: &mut Engine) {
    e.call(0x00ae_83e0, &args![0x011f_3350]);
}

// Translated from 00fda190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3358`: stores the vtable `01094e7c` and runs `00ae91f0` on it, stores the vtable `01094e9c` and runs `00ae91f0` on it again, then calls `00aa10f0` (cdecl, one argument) with the word at `011f3360`.
// Translation incomplete: 00fda190 - FS:[0] exception frame (handler 00f1d4ca) not translated.
pub fn fn_00fda190(e: &mut Engine) {
    e.set_global::<u32>(0x011f_3358, 0x0109_4e7c);
    e.call(0x00ae_91f0, &args![0x011f_3358]);
    e.set_global::<u32>(0x011f_3358, 0x0109_4e9c);
    e.call(0x00ae_91f0, &args![0x011f_3358]);
    let p = e.global::<u32>(0x011f_3360);
    e.call(0x00aa_10f0, &args![p]);
}

// Translated from 00fda210 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3368`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda210 - FS:[0] exception frame (handler 00f1d4fa) not translated.
pub fn fn_00fda210(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_3368, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f33bc`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda2b0 - FS:[0] exception frame (handler 00f1d8da) not translated.
pub fn fn_00fda2b0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_33bc, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f33c8`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda330 - FS:[0] exception frame (handler 00f1d90a) not translated.
pub fn fn_00fda330(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_33c8, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f33dc`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fda3b0(e: &mut Engine) {
    delete_critical_section(e, 0x011f_33dc);
}

// Translated from 00fda3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3414`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda3c0 - FS:[0] exception frame (handler 00f1e16a) not translated.
pub fn fn_00fda3c0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_3414, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3420`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda440 - FS:[0] exception frame (handler 00f1e19a) not translated.
pub fn fn_00fda440(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_3420, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f342c`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda4c0 - FS:[0] exception frame (handler 00f1e1ca) not translated.
pub fn fn_00fda4c0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_342c, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3438`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda540 - FS:[0] exception frame (handler 00f1e1fa) not translated.
pub fn fn_00fda540(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_3438, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3444`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda5c0 - FS:[0] exception frame (handler 00f1e22a) not translated.
pub fn fn_00fda5c0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_3444, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3450`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda640 - FS:[0] exception frame (handler 00f1e25a) not translated.
pub fn fn_00fda640(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_3450, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda6c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f345c`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda6c0 - FS:[0] exception frame (handler 00f1e28a) not translated.
pub fn fn_00fda6c0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_345c, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda740 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3468`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda740 - FS:[0] exception frame (handler 00f1e2ba) not translated.
pub fn fn_00fda740(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_3468, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda7c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f34e0`: runs `00a1e900` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fda7c0(e: &mut Engine) {
    e.call(0x00a1_e900, &args![0x011f_34e0]);
}

// Translated from 00fda7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f34f4`: runs `00a1ec70` on it (`ECX` = the global; no stack arguments).
pub fn fn_00fda7e0(e: &mut Engine) {
    e.call(0x00a1_ec70, &args![0x011f_34f4]);
}

// Translated from 00fda800 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3508`: stores the vtable `0109568c` in the global, then calls `00aa10f0` (cdecl, one argument) with the word at `011f350c`.
pub fn fn_00fda800(e: &mut Engine) {
    e.set_global::<u32>(0x011f_3508, 0x0109_568c);
    let p = e.global::<u32>(0x011f_350c);
    e.call(0x00aa_10f0, &args![p]);
}

// Translated from 00fda830 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f35d0`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda830 - FS:[0] exception frame (handler 00f1e95a) not translated.
pub fn fn_00fda830(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_35d0, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda8b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f35dc`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda8b0 - FS:[0] exception frame (handler 00f1e98a) not translated.
pub fn fn_00fda8b0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_35dc, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f35e8`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda930 - FS:[0] exception frame (handler 00f1e9ba) not translated.
pub fn fn_00fda930(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_35e8, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fda9b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f35f4`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fda9b0 - FS:[0] exception frame (handler 00f1e9ea) not translated.
pub fn fn_00fda9b0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_35f4, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdaa30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3600`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdaa30 - FS:[0] exception frame (handler 00f1ea1a) not translated.
pub fn fn_00fdaa30(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_3600, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdaab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f360c`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdaab0 - FS:[0] exception frame (handler 00f1ea4a) not translated.
pub fn fn_00fdaab0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_360c, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdab30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3618`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdab30 - FS:[0] exception frame (handler 00f1ea7a) not translated.
pub fn fn_00fdab30(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_3618, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdabb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3624`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdabb0 - FS:[0] exception frame (handler 00f1eaaa) not translated.
pub fn fn_00fdabb0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_3624, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdac30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3638`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdac30 - FS:[0] exception frame (handler 00f1ebba) not translated.
pub fn fn_00fdac30(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_3638, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdacb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f3644`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdacb0 - FS:[0] exception frame (handler 00f1ebea) not translated.
pub fn fn_00fdacb0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_3644, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdadd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the string whose pointer is the global `011f3ec0`: unless it equals the word at `0109b220`, `InterlockedDecrement` of the word 8 bytes before it (import slot `00fdf1c4`).
pub fn fn_00fdadd0(e: &mut Engine) {
    release_string(e, 0x011f_3ec0);
}

// Translated from 00fdadf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the string whose pointer is the global `011f3ec4`: unless it equals the word at `0109b220`, `InterlockedDecrement` of the word 8 bytes before it (import slot `00fdf1c4`).
pub fn fn_00fdadf0(e: &mut Engine) {
    release_string(e, 0x011f_3ec4);
}

// Translated from 00fdae40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f4180`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdae40(e: &mut Engine) {
    delete_critical_section(e, 0x011f_4180);
}

// Translated from 00fdae60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f4380`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdae60(e: &mut Engine) {
    delete_critical_section(e, 0x011f_4380);
}

// Translated from 00fdae70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f4410`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdae70(e: &mut Engine) {
    release_ref(e, 0x011f_4410, 0);
}

// Translated from 00fdaea0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f4438`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdaea0(e: &mut Engine) {
    release_ref(e, 0x011f_4438, 0);
}

// Translated from 00fdaed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f4444`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdaed0(e: &mut Engine) {
    release_ref(e, 0x011f_4444, 0);
}

// Translated from 00fdaf00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f4580`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdaf00(e: &mut Engine) {
    delete_critical_section(e, 0x011f_4580);
}

// Translated from 00fdaf10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the 10 reference-counted objects whose pointers are the array that ends at `011f453c`, last to first: each non-null one gets `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`), and at zero its virtual destructor in vtable slot +4.
pub fn fn_00fdaf10(e: &mut Engine) {
    release_objects(e, 0x011f_453c, 0x9);
}

// Translated from 00fdaf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f4800`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdaf50(e: &mut Engine) {
    delete_critical_section(e, 0x011f_4800);
}

// Translated from 00fdaf60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that passes the value of the global `011f4798` to `CloseHandle` (import slot `00fdf07c`).
pub fn fn_00fdaf60(e: &mut Engine) {
    import_on_global(e, 0x00fd_f07c, 0x011f_4798);
}

// Translated from 00fdaf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f4900`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdaf70(e: &mut Engine) {
    delete_critical_section(e, 0x011f_4900);
}

// Translated from 00fdaf80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the 10 strings in the array that ends at `011f49d8`, last to first: each one that is not the word at `0109b220` gets `InterlockedDecrement` on the word 8 bytes before it (import slot `00fdf1c4`).
pub fn fn_00fdaf80(e: &mut Engine) {
    release_strings(e, 0x011f_49d8, 0x9);
}

// Translated from 00fdafb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f49ac`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdafb0(e: &mut Engine) {
    release_ref(e, 0x011f_49ac, 0);
}

// Translated from 00fdafe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the string whose pointer is the global `011f49e0`: unless it equals the word at `0109b220`, `InterlockedDecrement` of the word 8 bytes before it (import slot `00fdf1c4`).
pub fn fn_00fdafe0(e: &mut Engine) {
    release_string(e, 0x011f_49e0);
}

// Translated from 00fdb000 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f4a14`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdb000(e: &mut Engine) {
    release_ref(e, 0x011f_4a14, 0);
}

// Translated from 00fdb030 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f4a64`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdb030(e: &mut Engine) {
    release_ref(e, 0x011f_4a64, 0);
}

// Translated from 00fdb060 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f4a7c`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdb060(e: &mut Engine) {
    release_ref(e, 0x011f_4a7c, 0);
}

// Translated from 00fdb090 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f5ae8`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdb090(e: &mut Engine) {
    release_ref(e, 0x011f_5ae8, 0);
}

// Translated from 00fdb0c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f5c80`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdb0c0(e: &mut Engine) {
    delete_critical_section(e, 0x011f_5c80);
}

// Translated from 00fdb0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f5d28`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdb0d0(e: &mut Engine) {
    release_ref(e, 0x011f_5d28, 0);
}

// Translated from 00fdb100 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that calls `00aa10f0` (cdecl, one argument) with the word at `011f5e44`.
pub fn fn_00fdb100(e: &mut Engine) {
    let p = e.global::<u32>(0x011f_5e44);
    e.call(0x00aa_10f0, &args![p]);
}

// Translated from 00fdb170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the 4 strings in the array that ends at `011f5eac`, last to first: each one that is not the word at `0109b220` gets `InterlockedDecrement` on the word 8 bytes before it (import slot `00fdf1c4`).
pub fn fn_00fdb170(e: &mut Engine) {
    release_strings(e, 0x011f_5eac, 0x3);
}

// Translated from 00fdb1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f5ed4`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdb1a0(e: &mut Engine) {
    release_ref(e, 0x011f_5ed4, 0);
}

// Translated from 00fdb1d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f5f3c`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdb1d0(e: &mut Engine) {
    release_ref(e, 0x011f_5f3c, 0);
}

// Translated from 00fdb200 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f5f50`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdb200(e: &mut Engine) {
    release_ref(e, 0x011f_5f50, 0);
}

// Translated from 00fdb260 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f600c`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdb260(e: &mut Engine) {
    delete_critical_section(e, 0x011f_600c);
}

// Translated from 00fdb270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011ac040`: loads the word at `011ac044`, stores the vtable `010a1458` in the global, then calls `00a47600` (cdecl, one argument) with the word.
pub fn fn_00fdb270(e: &mut Engine) {
    let p = e.global::<u32>(0x011a_c044);
    e.set_global::<u32>(0x011a_c040, 0x010a_1458);
    e.call(0x00a4_7600, &args![p]);
}

// Translated from 00fdb2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f6bb8`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdb2a0(e: &mut Engine) {
    delete_critical_section(e, 0x011f_6bb8);
}

// Translated from 00fdb2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f6bd4`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdb2b0(e: &mut Engine) {
    delete_critical_section(e, 0x011f_6bd4);
}

// Translated from 00fdb2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6d9c`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb2c0 - FS:[0] exception frame (handler 00f21bba) not translated.
pub fn fn_00fdb2c0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6d9c, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdb340 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6da8`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb340 - FS:[0] exception frame (handler 00f21bea) not translated.
pub fn fn_00fdb340(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6da8, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdb3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6db4`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb3c0 - FS:[0] exception frame (handler 00f21c1a) not translated.
pub fn fn_00fdb3c0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6db4, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdb440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6dc0`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb440 - FS:[0] exception frame (handler 00f21c4a) not translated.
pub fn fn_00fdb440(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6dc0, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdb4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6dcc`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb4c0 - FS:[0] exception frame (handler 00f21c7a) not translated.
pub fn fn_00fdb4c0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6dcc, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdb540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6dd8`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb540 - FS:[0] exception frame (handler 00f21caa) not translated.
pub fn fn_00fdb540(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6dd8, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdb5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6de4`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb5c0 - FS:[0] exception frame (handler 00f21cda) not translated.
pub fn fn_00fdb5c0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6de4, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdb640 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `atexit` function whose body is empty (a bare `RET`, or the frame set-up and tear-down around nothing): it does nothing.
pub fn fn_00fdb640(_e: &mut Engine) {}

// Translated from 00fdb650 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6dfc`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb650 - FS:[0] exception frame (handler 00f21d0a) not translated.
pub fn fn_00fdb650(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6dfc, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdb6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6e14`: stores the vtable `01022738` in it, calls `004de4a0` (cdecl, no arguments: it makes the singleton kept at `011f35a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb6d0 - FS:[0] exception frame (handler 00f2278a) not translated.
pub fn fn_00fdb6d0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6e14, 0x0102_2738, 0x004d_e4a0, 0x011f_35a0);
}

// Translated from 00fdb750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6e20`: stores the vtable `01022738` in it, calls `004de4a0` (cdecl, no arguments: it makes the singleton kept at `011f35a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb750 - FS:[0] exception frame (handler 00f227ba) not translated.
pub fn fn_00fdb750(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6e20, 0x0102_2738, 0x004d_e4a0, 0x011f_35a0);
}

// Translated from 00fdb7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6e2c`: stores the vtable `01022738` in it, calls `004de4a0` (cdecl, no arguments: it makes the singleton kept at `011f35a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb7d0 - FS:[0] exception frame (handler 00f227ea) not translated.
pub fn fn_00fdb7d0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6e2c, 0x0102_2738, 0x004d_e4a0, 0x011f_35a0);
}

// Translated from 00fdb850 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6e38`: stores the vtable `01022738` in it, calls `004de4a0` (cdecl, no arguments: it makes the singleton kept at `011f35a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb850 - FS:[0] exception frame (handler 00f2281a) not translated.
pub fn fn_00fdb850(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6e38, 0x0102_2738, 0x004d_e4a0, 0x011f_35a0);
}

// Translated from 00fdb8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6e44`: stores the vtable `01022738` in it, calls `004de4a0` (cdecl, no arguments: it makes the singleton kept at `011f35a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb8d0 - FS:[0] exception frame (handler 00f2284a) not translated.
pub fn fn_00fdb8d0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6e44, 0x0102_2738, 0x004d_e4a0, 0x011f_35a0);
}

// Translated from 00fdb950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6e50`: stores the vtable `01022738` in it, calls `004de4a0` (cdecl, no arguments: it makes the singleton kept at `011f35a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb950 - FS:[0] exception frame (handler 00f2287a) not translated.
pub fn fn_00fdb950(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6e50, 0x0102_2738, 0x004d_e4a0, 0x011f_35a0);
}

// Translated from 00fdb9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6e5c`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdb9d0 - FS:[0] exception frame (handler 00f228aa) not translated.
pub fn fn_00fdb9d0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6e5c, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdba50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6e68`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdba50 - FS:[0] exception frame (handler 00f228da) not translated.
pub fn fn_00fdba50(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6e68, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdbad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6e74`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdbad0 - FS:[0] exception frame (handler 00f2290a) not translated.
pub fn fn_00fdbad0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6e74, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdbb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6e80`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdbb50 - FS:[0] exception frame (handler 00f2293a) not translated.
pub fn fn_00fdbb50(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6e80, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdbbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6e8c`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdbbd0 - FS:[0] exception frame (handler 00f2296a) not translated.
pub fn fn_00fdbbd0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6e8c, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdbc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6e98`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdbc50 - FS:[0] exception frame (handler 00f2299a) not translated.
pub fn fn_00fdbc50(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6e98, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdbcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6ea4`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdbcd0 - FS:[0] exception frame (handler 00f229ca) not translated.
pub fn fn_00fdbcd0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6ea4, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdbd50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6eb0`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdbd50 - FS:[0] exception frame (handler 00f229fa) not translated.
pub fn fn_00fdbd50(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6eb0, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdbdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6ebc`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdbdd0 - FS:[0] exception frame (handler 00f22a2a) not translated.
pub fn fn_00fdbdd0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6ebc, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdbe50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6ec8`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdbe50 - FS:[0] exception frame (handler 00f22a5a) not translated.
pub fn fn_00fdbe50(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6ec8, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdbed0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6ed4`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdbed0 - FS:[0] exception frame (handler 00f22a8a) not translated.
pub fn fn_00fdbed0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6ed4, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdbf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f6ee0`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdbf50 - FS:[0] exception frame (handler 00f22aba) not translated.
pub fn fn_00fdbf50(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_6ee0, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdbff0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f7130`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdbff0 - FS:[0] exception frame (handler 00f22dfa) not translated.
pub fn fn_00fdbff0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_7130, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdc070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f713c`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdc070 - FS:[0] exception frame (handler 00f22e2a) not translated.
pub fn fn_00fdc070(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_713c, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdc0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f7148`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdc0f0 - FS:[0] exception frame (handler 00f22e5a) not translated.
pub fn fn_00fdc0f0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_7148, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdc170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f7154`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdc170 - FS:[0] exception frame (handler 00f22e8a) not translated.
pub fn fn_00fdc170(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_7154, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdc1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f7160`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdc1f0 - FS:[0] exception frame (handler 00f22eba) not translated.
pub fn fn_00fdc1f0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_7160, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdc270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f716c`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdc270 - FS:[0] exception frame (handler 00f22eea) not translated.
pub fn fn_00fdc270(e: &mut Engine) {
    fs_singleton_destruct(e, 0x011f_716c, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdc370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f8170`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdc370(e: &mut Engine) {
    delete_critical_section(e, 0x011f_8170);
}

// Translated from 00fdc380 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object at the pointer in the global `011f8188` plus 0x158: if the global is not null, `InterlockedDecrement` of the count at +4 of that address (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4.
pub fn fn_00fdc380(e: &mut Engine) {
    release_ref(e, 0x011f_8188, 0x158);
}

// Translated from 00fdc3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011f8190`: `eh vector destructor iterator` (`00ec5fce`) over 18 elements of 4 bytes with the element destructor `00afc720`.
pub fn fn_00fdc3c0(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011f_8190, 0x4, 0x12, 0x00af_c720);
}

// Translated from 00fdc3f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f8254`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdc3f0(e: &mut Engine) {
    delete_critical_section(e, 0x011f_8254);
}

// Translated from 00fdc410 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f8bdc`: runs `00b0e7f0` on it, then calls `00aa4060` on the object at `011f6238` (`ECX` = that object, `RET 4`) with the pointer in the global at `011f8bdc`.
// Translation incomplete: 00fdc410 - FS:[0] exception frame (handler 00f24544) not translated.
pub fn fn_00fdc410(e: &mut Engine) {
    e.call(0x00b0_e7f0, &args![0x011f_8bdc]);
    let p = e.global::<u32>(0x011f_8bdc);
    e.call(0x00aa_4060, &args![0x011f_6238, p]);
}

// Translated from 00fdc480 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `011f8bc4`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdc480(e: &mut Engine) {
    delete_critical_section(e, 0x011f_8bc4);
}

// Translated from 00fdc490 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that runs `0044a730` on the global at `011f8ba4` with the two stack arguments 0 and 1 (pushed in that order, so the first parameter is 1).
pub fn fn_00fdc490(e: &mut Engine) {
    e.call(0x0044_a730, &args![0x011f_8ba4, 0x1, 0x0]);
}

// Translated from 00fdc4b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that runs `0044a730` on the global at `011f8b84` with the two stack arguments 0 and 1 (pushed in that order, so the first parameter is 1).
pub fn fn_00fdc4b0(e: &mut Engine) {
    e.call(0x0044_a730, &args![0x011f_8b84, 0x1, 0x0]);
}

// Translated from 00fdc4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f8c68`: runs `00b17f50` on it, then passes the pointer now at `011f8c68` to `00401030` (cdecl, one argument).
// Translation incomplete: 00fdc4d0 - FS:[0] exception frame (handler 00f2476a) not translated.
pub fn fn_00fdc4d0(e: &mut Engine) {
    e.call(0x00b1_7f50, &args![0x011f_8c68]);
    let p = e.global::<u32>(0x011f_8c68);
    e.call(0x0040_1030, &args![p]);
}

// Translated from 00fdc530 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f8c18`: runs `00b17f50` on it, then passes the pointer now at `011f8c18` to `00401030` (cdecl, one argument).
// Translation incomplete: 00fdc530 - FS:[0] exception frame (handler 00f2479a) not translated.
pub fn fn_00fdc530(e: &mut Engine) {
    e.call(0x00b1_7f50, &args![0x011f_8c18]);
    let p = e.global::<u32>(0x011f_8c18);
    e.call(0x0040_1030, &args![p]);
}

// Translated from 00fdc590 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f8c50`: runs `00b17f50` on it, then passes the pointer now at `011f8c50` to `00401030` (cdecl, one argument).
// Translation incomplete: 00fdc590 - FS:[0] exception frame (handler 00f247ca) not translated.
pub fn fn_00fdc590(e: &mut Engine) {
    e.call(0x00b1_7f50, &args![0x011f_8c50]);
    let p = e.global::<u32>(0x011f_8c50);
    e.call(0x0040_1030, &args![p]);
}

// Translated from 00fdc5f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f8c80`: runs `00b17f50` on it, then passes the pointer now at `011f8c80` to `00401030` (cdecl, one argument).
// Translation incomplete: 00fdc5f0 - FS:[0] exception frame (handler 00f247fa) not translated.
pub fn fn_00fdc5f0(e: &mut Engine) {
    e.call(0x00b1_7f50, &args![0x011f_8c80]);
    let p = e.global::<u32>(0x011f_8c80);
    e.call(0x0040_1030, &args![p]);
}

// Translated from 00fdc660 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that zeroes the halfword at `011f8d84`, passes the pointer at `011f8d88` to `00401030` (cdecl, one argument), then zeroes the pointer at `011f8d88`.
pub fn fn_00fdc660(e: &mut Engine) {
    e.set_global::<u16>(0x011f_8d84, 0);
    let p = e.global::<u32>(0x011f_8d88);
    e.call(0x0040_1030, &args![p]);
    e.set_global::<u32>(0x011f_8d88, 0);
}

// Translated from 00fdc690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f8e2c`: runs `00b3ba90` on it, then passes the pointer now at `011f8e2c` to `00401030` (cdecl, one argument).
// Translation incomplete: 00fdc690 - FS:[0] exception frame (handler 00f264ba) not translated.
pub fn fn_00fdc690(e: &mut Engine) {
    e.call(0x00b3_ba90, &args![0x011f_8e2c]);
    let p = e.global::<u32>(0x011f_8e2c);
    e.call(0x0040_1030, &args![p]);
}

// Translated from 00fdc720 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that stores the vtable `010a58c8` in the global at `011f8ebc`.
pub fn fn_00fdc720(e: &mut Engine) {
    e.set_global::<u32>(0x011f_8ebc, 0x010a_58c8);
}

// Translated from 00fdc750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f9154`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc750(e: &mut Engine) {
    release_ref(e, 0x011f_9154, 0);
}

// Translated from 00fdc7a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// An `atexit` function whose body is empty (a bare `RET`, or the frame set-up and tear-down around nothing): it does nothing.
pub fn fn_00fdc7a0(_e: &mut Engine) {}

// Translated from 00fdc7b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f950c`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc7b0(e: &mut Engine) {
    release_ref(e, 0x011f_950c, 0);
}

// Translated from 00fdc7e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f9510`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc7e0(e: &mut Engine) {
    release_ref(e, 0x011f_9510, 0);
}

// Translated from 00fdc810 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f9514`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc810(e: &mut Engine) {
    release_ref(e, 0x011f_9514, 0);
}

// Translated from 00fdc840 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f9518`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc840(e: &mut Engine) {
    release_ref(e, 0x011f_9518, 0);
}

// Translated from 00fdc870 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f951c`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc870(e: &mut Engine) {
    release_ref(e, 0x011f_951c, 0);
}

// Translated from 00fdc8a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f9520`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc8a0(e: &mut Engine) {
    release_ref(e, 0x011f_9520, 0);
}

// Translated from 00fdc8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f9524`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc8d0(e: &mut Engine) {
    release_ref(e, 0x011f_9524, 0);
}

// Translated from 00fdc900 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f9528`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc900(e: &mut Engine) {
    release_ref(e, 0x011f_9528, 0);
}

// Translated from 00fdc930 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f952c`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc930(e: &mut Engine) {
    release_ref(e, 0x011f_952c, 0);
}

// Translated from 00fdc960 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f9530`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc960(e: &mut Engine) {
    release_ref(e, 0x011f_9530, 0);
}

// Translated from 00fdc990 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f9534`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc990(e: &mut Engine) {
    release_ref(e, 0x011f_9534, 0);
}

// Translated from 00fdc9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f9538`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc9c0(e: &mut Engine) {
    release_ref(e, 0x011f_9538, 0);
}

// Translated from 00fdc9f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f953c`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdc9f0(e: &mut Engine) {
    release_ref(e, 0x011f_953c, 0);
}

// Translated from 00fdca20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f9540`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdca20(e: &mut Engine) {
    release_ref(e, 0x011f_9540, 0);
}

// Translated from 00fdca50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f9544`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdca50(e: &mut Engine) {
    release_ref(e, 0x011f_9544, 0);
}

// Translated from 00fdca80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f95d4`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdca80(e: &mut Engine) {
    release_ref(e, 0x011f_95d4, 0);
}

// Translated from 00fdcab0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f95e4`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdcab0(e: &mut Engine) {
    release_ref(e, 0x011f_95e4, 0);
}

// Translated from 00fdcae0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f95e8`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdcae0(e: &mut Engine) {
    release_ref(e, 0x011f_95e8, 0);
}

// Translated from 00fdcb10 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f9508`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdcb10(e: &mut Engine) {
    release_ref(e, 0x011f_9508, 0);
}

// Translated from 00fdcb40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011f9548`: `eh vector destructor iterator` (`00ec5fce`) over 35 elements of 4 bytes with the element destructor `00b66f20`.
pub fn fn_00fdcb40(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011f_9548, 0x4, 0x23, 0x00b6_6f20);
}

// Translated from 00fdcb60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f95d8`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdcb60(e: &mut Engine) {
    release_ref(e, 0x011f_95d8, 0);
}

// Translated from 00fdcb90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f95ec`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdcb90(e: &mut Engine) {
    release_ref(e, 0x011f_95ec, 0);
}

// Translated from 00fdcbc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011f95f0`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdcbc0(e: &mut Engine) {
    release_ref(e, 0x011f_95f0, 0);
}

// Translated from 00fdcca0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011fa008`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdcca0(e: &mut Engine) {
    release_ref(e, 0x011f_a008, 0);
}

// Translated from 00fdccd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011fa024`: `eh vector destructor iterator` (`00ec5fce`) over 10 elements of 4 bytes with the element destructor `00b66f20`.
pub fn fn_00fdccd0(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011f_a024, 0x4, 0xa, 0x00b6_6f20);
}

// Translated from 00fdcd00 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011fda48`: `eh vector destructor iterator` (`00ec5fce`) over 208 elements of 4 bytes with the element destructor `00b66f20`.
pub fn fn_00fdcd00(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011f_da48, 0x4, 0xd0, 0x00b6_6f20);
}

// Translated from 00fdcd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011fdd88`: `eh vector destructor iterator` (`00ec5fce`) over 156 elements of 4 bytes with the element destructor `00b66f20`.
pub fn fn_00fdcd20(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011f_dd88, 0x4, 0x9c, 0x00b6_6f20);
}

// Translated from 00fdcd40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011febd8`: `eh vector destructor iterator` (`00ec5fce`) over 22 elements of 4 bytes with the element destructor `0045cec0`.
pub fn fn_00fdcd40(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011f_ebd8, 0x4, 0x16, 0x0045_cec0);
}

// Translated from 00fdcd60 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011fec30`: `eh vector destructor iterator` (`00ec5fce`) over 22 elements of 4 bytes with the element destructor `0045cec0`.
pub fn fn_00fdcd60(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011f_ec30, 0x4, 0x16, 0x0045_cec0);
}

// Translated from 00fdcd80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011fdff8`: `eh vector destructor iterator` (`00ec5fce`) over 759 elements of 4 bytes with the element destructor `00b79930`.
pub fn fn_00fdcd80(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011f_dff8, 0x4, 0x2f7, 0x00b7_9930);
}

// Translated from 00fdcda0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `011fedc4`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdcda0(e: &mut Engine) {
    release_ref(e, 0x011f_edc4, 0);
}

// Translated from 00fdcdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011ff4a0`: `eh vector destructor iterator` (`00ec5fce`) over 3 elements of 4 bytes with the element destructor `0045cec0`.
pub fn fn_00fdcdd0(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011f_f4a0, 0x4, 0x3, 0x0045_cec0);
}

// Translated from 00fdcdf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `011ff4ac`: `eh vector destructor iterator` (`00ec5fce`) over 6 elements of 4 bytes with the element destructor `0045cec0`.
pub fn fn_00fdcdf0(e: &mut Engine) {
    vector_destructor_iterator(e, 0x011f_f4ac, 0x4, 0x6, 0x0045_cec0);
}

// Translated from 00fdce40 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array object at `011ada00`: loads its block pointer (`011ada04`), stores the vtable `010b7f00`, and when the block is not null calls the function in the vtable's slot `010b7f08` (`00cb9cb0`: it reads one stack argument, the block, and ignores `ECX`), then clears the words at `011ada04`, `011ada08` and `011ada0c`.
pub fn fn_00fdce40(e: &mut Engine) {
    let block = e.global::<u32>(0x011a_da04);
    e.set_global::<u32>(0x011a_da00, 0x010b_7f00);
    if block != 0 {
        // `CALL dword ptr [slot]`: the slot is in the vtable just stored; its function reads one
        // stack argument and does not use ECX, so the block is its only word.
        let target = e.global::<u32>(0x010b_7f08);
        e.call(target, &args![block]);
        e.set_global::<u32>(0x011a_da04, 0);
        e.set_global::<u32>(0x011a_da0c, 0);
        e.set_global::<u32>(0x011a_da08, 0);
    }
}

// Translated from 00fdce80 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array object at `011ada10`: loads its block pointer (`011ada14`), stores the vtable `010b7f00`, and when the block is not null calls the function in the vtable's slot `010b7f08` (`00cb9cb0`: it reads one stack argument, the block, and ignores `ECX`), then clears the words at `011ada14`, `011ada18` and `011ada1c`.
pub fn fn_00fdce80(e: &mut Engine) {
    let block = e.global::<u32>(0x011a_da14);
    e.set_global::<u32>(0x011a_da10, 0x010b_7f00);
    if block != 0 {
        // `CALL dword ptr [slot]`: the slot is in the vtable just stored; its function reads one
        // stack argument and does not use ECX, so the block is its only word.
        let target = e.global::<u32>(0x010b_7f08);
        e.call(target, &args![block]);
        e.set_global::<u32>(0x011a_da14, 0);
        e.set_global::<u32>(0x011a_da1c, 0);
        e.set_global::<u32>(0x011a_da18, 0);
    }
}

// Translated from 00fdcef0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `012003f4`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdcef0(e: &mut Engine) {
    release_ref(e, 0x0120_03f4, 0);
}

// Translated from 00fdcf30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `012005c8`: `eh vector destructor iterator` (`00ec5fce`) over 2 elements of 4 bytes with the element destructor `00b66f20`.
pub fn fn_00fdcf30(e: &mut Engine) {
    vector_destructor_iterator(e, 0x0120_05c8, 0x4, 0x2, 0x00b6_6f20);
}

// Translated from 00fdcf50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `012005d0`: `eh vector destructor iterator` (`00ec5fce`) over 4 elements of 4 bytes with the element destructor `00b66f20`.
pub fn fn_00fdcf50(e: &mut Engine) {
    vector_destructor_iterator(e, 0x0120_05d0, 0x4, 0x4, 0x00b6_6f20);
}

// Translated from 00fdcf70 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `012005e0`: `eh vector destructor iterator` (`00ec5fce`) over 1 elements of 4 bytes with the element destructor `00b79930`.
pub fn fn_00fdcf70(e: &mut Engine) {
    vector_destructor_iterator(e, 0x0120_05e0, 0x4, 0x1, 0x00b7_9930);
}

// Translated from 00fdcf90 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `012007c4`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdcf90(e: &mut Engine) {
    release_ref(e, 0x0120_07c4, 0);
}

// Translated from 00fdcfc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011ae5d8`: loads the word at `011ae5dc`, stores the vtable `010b9d24` in the global, then calls `00aa10f0` (cdecl, one argument) with the word.
pub fn fn_00fdcfc0(e: &mut Engine) {
    let p = e.global::<u32>(0x011a_e5dc);
    e.set_global::<u32>(0x011a_e5d8, 0x010b_9d24);
    e.call(0x00aa_10f0, &args![p]);
}

// Translated from 00fdcfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `01200810`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdcfe0(e: &mut Engine) {
    release_ref(e, 0x0120_0810, 0);
}

// Translated from 00fdd010 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `01200814`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdd010(e: &mut Engine) {
    release_ref(e, 0x0120_0814, 0);
}

// Translated from 00fdd040 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `01200818`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdd040(e: &mut Engine) {
    release_ref(e, 0x0120_0818, 0);
}

// Translated from 00fdd070 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `0120083c`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdd070(e: &mut Engine) {
    release_ref(e, 0x0120_083c, 0);
}

// Translated from 00fdd0a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `01200840`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdd0a0(e: &mut Engine) {
    release_ref(e, 0x0120_0840, 0);
}

// Translated from 00fdd0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `01200c48`: `eh vector destructor iterator` (`00ec5fce`) over 50 elements of 4 bytes with the element destructor `00b66f20`.
pub fn fn_00fdd0d0(e: &mut Engine) {
    vector_destructor_iterator(e, 0x0120_0c48, 0x4, 0x32, 0x00b6_6f20);
}

// Translated from 00fdd0f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `01200d10`: `eh vector destructor iterator` (`00ec5fce`) over 50 elements of 4 bytes with the element destructor `00b66f20`.
pub fn fn_00fdd0f0(e: &mut Engine) {
    vector_destructor_iterator(e, 0x0120_0d10, 0x4, 0x32, 0x00b6_6f20);
}

// Translated from 00fdd110 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `01200dd8`: `eh vector destructor iterator` (`00ec5fce`) over 57 elements of 4 bytes with the element destructor `00b66f20`.
pub fn fn_00fdd110(e: &mut Engine) {
    vector_destructor_iterator(e, 0x0120_0dd8, 0x4, 0x39, 0x00b6_6f20);
}

// Translated from 00fdd130 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `01201080`: `eh vector destructor iterator` (`00ec5fce`) over 17 elements of 4 bytes with the element destructor `0045cec0`.
pub fn fn_00fdd130(e: &mut Engine) {
    vector_destructor_iterator(e, 0x0120_1080, 0x4, 0x11, 0x0045_cec0);
}

// Translated from 00fdd150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `012010c4`: `eh vector destructor iterator` (`00ec5fce`) over 10 elements of 4 bytes with the element destructor `0045cec0`.
pub fn fn_00fdd150(e: &mut Engine) {
    vector_destructor_iterator(e, 0x0120_10c4, 0x4, 0xa, 0x0045_cec0);
}

// Translated from 00fdd170 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `01200ec0`: `eh vector destructor iterator` (`00ec5fce`) over 112 elements of 4 bytes with the element destructor `00b79930`.
pub fn fn_00fdd170(e: &mut Engine) {
    vector_destructor_iterator(e, 0x0120_0ec0, 0x4, 0x70, 0x00b7_9930);
}

// Translated from 00fdd190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `01201f40`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdd190(e: &mut Engine) {
    release_ref(e, 0x0120_1f40, 0);
}

// Translated from 00fdd1c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `01201f44`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdd1c0(e: &mut Engine) {
    release_ref(e, 0x0120_1f44, 0);
}

// Translated from 00fdd1f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `01201f48`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdd1f0(e: &mut Engine) {
    release_ref(e, 0x0120_1f48, 0);
}

// Translated from 00fdd220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `01201f58`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdd220(e: &mut Engine) {
    release_ref(e, 0x0120_1f58, 0);
}

// Translated from 00fdd250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `01201fc4`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdd250(e: &mut Engine) {
    release_ref(e, 0x0120_1fc4, 0);
}

// Translated from 00fdd280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `01201fd0`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdd280(e: &mut Engine) {
    release_ref(e, 0x0120_1fd0, 0);
}

// Translated from 00fdd2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `012021e0`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdd2b0(e: &mut Engine) {
    release_ref(e, 0x0120_21e0, 0);
}

// Translated from 00fdd2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `012021e4`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdd2e0(e: &mut Engine) {
    release_ref(e, 0x0120_21e4, 0);
}

// Translated from 00fdd310 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `01202230`: `eh vector destructor iterator` (`00ec5fce`) over 33 elements of 4 bytes with the element destructor `00b79930`.
pub fn fn_00fdd310(e: &mut Engine) {
    vector_destructor_iterator(e, 0x0120_2230, 0x4, 0x21, 0x00b7_9930);
}

// Translated from 00fdd330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `012022c0`: `eh vector destructor iterator` (`00ec5fce`) over 26 elements of 4 bytes with the element destructor `00b79930`.
pub fn fn_00fdd330(e: &mut Engine) {
    vector_destructor_iterator(e, 0x0120_22c0, 0x4, 0x1a, 0x00b7_9930);
}

// Translated from 00fdd350 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the array at `01202350`: `eh vector destructor iterator` (`00ec5fce`) over 30 elements of 4 bytes with the element destructor `00b79930`.
pub fn fn_00fdd350(e: &mut Engine) {
    vector_destructor_iterator(e, 0x0120_2350, 0x4, 0x1e, 0x00b7_9930);
}

// Translated from 00fdd410 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `01202600`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdd410(e: &mut Engine) {
    delete_critical_section(e, 0x0120_2600);
}

// Translated from 00fdd430 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `01202d78`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fdd430(e: &mut Engine) {
    delete_critical_section(e, 0x0120_2d78);
}

// Translated from 00fdd440 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01202da8`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdd440 - FS:[0] exception frame (handler 00f2e27a) not translated.
pub fn fn_00fdd440(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_2da8, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdd4c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01202db4`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdd4c0 - FS:[0] exception frame (handler 00f2e2aa) not translated.
pub fn fn_00fdd4c0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_2db4, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdd540 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01202dc0`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdd540 - FS:[0] exception frame (handler 00f2e2da) not translated.
pub fn fn_00fdd540(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_2dc0, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdd5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01202dcc`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdd5c0 - FS:[0] exception frame (handler 00f2e30a) not translated.
pub fn fn_00fdd5c0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_2dcc, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdd640 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01202e20`: stores the vtable `010c1c5c`, and when the pointer at `01202e24` is not null runs `00c46420` on it with the argument 3 (`RET 4`).
pub fn fn_00fdd640(e: &mut Engine) {
    e.set_global::<u32>(0x0120_2e20, 0x010c_1c5c);
    let p = e.global::<u32>(0x0120_2e24);
    if p != 0 {
        e.call(0x00c4_6420, &args![p, 3]);
    }
}

// Translated from 00fdd690 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `01202e14` (the shape of the `relref` group, in a frame): if the global is not null, `InterlockedDecrement` of the count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4.
pub fn fn_00fdd690(e: &mut Engine) {
    release_ref(e, 0x0120_2e14, 0);
}

// Translated from 00fdd6d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01203120`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdd6d0 - FS:[0] exception frame (handler 00f2f19a) not translated.
pub fn fn_00fdd6d0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_3120, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdd750 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `0120312c`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdd750 - FS:[0] exception frame (handler 00f2f1ca) not translated.
pub fn fn_00fdd750(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_312c, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdd7d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01203138`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdd7d0 - FS:[0] exception frame (handler 00f2f1fa) not translated.
pub fn fn_00fdd7d0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_3138, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdd850 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01203144`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdd850 - FS:[0] exception frame (handler 00f2f22a) not translated.
pub fn fn_00fdd850(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_3144, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdd8d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01203150`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdd8d0 - FS:[0] exception frame (handler 00f2f25a) not translated.
pub fn fn_00fdd8d0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_3150, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdd950 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `0120315c`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdd950 - FS:[0] exception frame (handler 00f2f28a) not translated.
pub fn fn_00fdd950(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_315c, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdd9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01203168`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdd9d0 - FS:[0] exception frame (handler 00f2f2ba) not translated.
pub fn fn_00fdd9d0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_3168, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fdda50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01203174`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fdda50 - FS:[0] exception frame (handler 00f2f2ea) not translated.
pub fn fn_00fdda50(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_3174, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fddad0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01203188`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fddad0 - FS:[0] exception frame (handler 00f2f31a) not translated.
pub fn fn_00fddad0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_3188, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fddb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01203194`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fddb50 - FS:[0] exception frame (handler 00f2f34a) not translated.
pub fn fn_00fddb50(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_3194, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fddbd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `012031a0`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fddbd0 - FS:[0] exception frame (handler 00f2f37a) not translated.
pub fn fn_00fddbd0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_31a0, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fddc50 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `012031ac`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fddc50 - FS:[0] exception frame (handler 00f2f3aa) not translated.
pub fn fn_00fddc50(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0120_31ac, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fddcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `01203328`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fddcd0(e: &mut Engine) {
    delete_critical_section(e, 0x0120_3328);
}

// Translated from 00fddce0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `012043c0` (the shape of the `relref` group, in a frame): if the global is not null, `InterlockedDecrement` of the count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4.
pub fn fn_00fddce0(e: &mut Engine) {
    release_ref(e, 0x0120_43c0, 0);
}

// Translated from 00fddd20 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that stores the vtable `010a47b8` in the global at `0126782c`.
pub fn fn_00fddd20(e: &mut Engine) {
    e.set_global::<u32>(0x0126_782c, 0x010a_47b8);
}

// Translated from 00fddd30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that stores the vtable `010a47b8` in the global at `01267b0c`.
pub fn fn_00fddd30(e: &mut Engine) {
    e.set_global::<u32>(0x0126_7b0c, 0x010a_47b8);
}

// Translated from 00fdde30 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `01267be0`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fdde30(e: &mut Engine) {
    release_ref(e, 0x0126_7be0, 0);
}

// Translated from 00fde140 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `01267e48`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fde140(e: &mut Engine) {
    delete_critical_section(e, 0x0126_7e48);
}

// Translated from 00fde150 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that passes the value of the global `01268108` to `TlsFree` (import slot `00fdf050`).
pub fn fn_00fde150(e: &mut Engine) {
    import_on_global(e, 0x00fd_f050, 0x0126_8108);
}

// Translated from 00fde160 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `0126813c`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fde160(e: &mut Engine) {
    delete_critical_section(e, 0x0126_813c);
}

// Translated from 00fde190 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that passes the value of the global `012681d8` to `TlsFree` (import slot `00fdf050`).
pub fn fn_00fde190(e: &mut Engine) {
    import_on_global(e, 0x00fd_f050, 0x0126_81d8);
}

// Translated from 00fde1a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `0126820c`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fde1a0(e: &mut Engine) {
    delete_critical_section(e, 0x0126_820c);
}

// Translated from 00fde220 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the reference-counted object whose pointer is the global `012682f0`: if it is not null, `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`); at zero, the virtual destructor in vtable slot +4 (`EDX` jump: a tail call).
pub fn fn_00fde220(e: &mut Engine) {
    release_ref(e, 0x0126_82f0, 0);
}

// Translated from 00fde250 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that passes the value of the global `0126835c` to `TlsFree` (import slot `00fdf050`).
pub fn fn_00fde250(e: &mut Engine) {
    import_on_global(e, 0x00fd_f050, 0x0126_835c);
}

// Translated from 00fde270 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that passes the value of the global `012687c8` to `TlsFree` (import slot `00fdf050`).
pub fn fn_00fde270(e: &mut Engine) {
    import_on_global(e, 0x00fd_f050, 0x0126_87c8);
}

// Translated from 00fde280 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that stores the vtable `0102e388` in the global at `012687fc`.
pub fn fn_00fde280(e: &mut Engine) {
    e.set_global::<u32>(0x0126_87fc, 0x0102_e388);
}

// Translated from 00fde290 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that stores the vtable `0102e388` in the global at `01268b0c`.
pub fn fn_00fde290(e: &mut Engine) {
    e.set_global::<u32>(0x0126_8b0c, 0x0102_e388);
}

// Translated from 00fde2b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that stores the vtable `0102e388` in the global at `011b7c00`.
pub fn fn_00fde2b0(e: &mut Engine) {
    e.set_global::<u32>(0x011b_7c00, 0x0102_e388);
}

// Translated from 00fde2d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that stores the vtable `010ead08` in the global at `011bdee0`.
pub fn fn_00fde2d0(e: &mut Engine) {
    e.set_global::<u32>(0x011b_dee0, 0x010e_ad08);
}

// Translated from 00fde2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the 256 reference-counted objects whose pointers are the array that ends at `0126f508`, last to first: each non-null one gets `InterlockedDecrement` of its count at +4 (import slot `00fdf1c4`), and at zero its virtual destructor in vtable slot +4.
pub fn fn_00fde2e0(e: &mut Engine) {
    release_objects(e, 0x0126_f508, 0xff);
}

// Translated from 00fde330 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the critical section at `0126f600`: `DeleteCriticalSection` (import slot `00fdf058`).
pub fn fn_00fde330(e: &mut Engine) {
    delete_critical_section(e, 0x0126_f600);
}

// Translated from 00fde370 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` release of the string whose pointer is the global `0126f750`: unless it equals the word at `0109b220`, `InterlockedDecrement` of the word 8 bytes before it (import slot `00fdf1c4`).
pub fn fn_00fde370(e: &mut Engine) {
    release_string(e, 0x0126_f750);
}

// Translated from 00fde3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` function that stores the vtable `010f16f4` in the global at `0126fa68`.
pub fn fn_00fde3c0(e: &mut Engine) {
    e.set_global::<u32>(0x0126_fa68, 0x010f_16f4);
}

// Translated from 00fde3d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `0126fac8`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fde3d0 - FS:[0] exception frame (handler 00f34d1a) not translated.
pub fn fn_00fde3d0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0126_fac8, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fde450 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `0126fad4`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fde450 - FS:[0] exception frame (handler 00f34d4a) not translated.
pub fn fn_00fde450(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0126_fad4, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

// Translated from 00fde4d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `0126fae0`: stores the vtable `01017720` in it, calls `0044f570` (cdecl, no arguments: it makes the singleton kept at `011f96a0` when it is absent), then calls the singleton's virtual function at vtable slot +8 with the global (`RET 4`), and finally runs `00404950` on the global.
// Translation incomplete: 00fde4d0 - FS:[0] exception frame (handler 00f34d7a) not translated.
pub fn fn_00fde4d0(e: &mut Engine) {
    fs_singleton_destruct(e, 0x0126_fae0, 0x0101_7720, 0x0044_f570, 0x011f_96a0);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00fc_9090, fn_00fc9090()),
        entry!(0x00fc_9a10, fn_00fc9a10()),
        entry!(0x00fc_9a60, fn_00fc9a60()),
        entry!(0x00fc_b260, fn_00fcb260()),
        entry!(0x00fc_b280, fn_00fcb280()),
        entry!(0x00fc_b8c0, fn_00fcb8c0()),
        entry!(0x00fd_7ad0, fn_00fd7ad0()),
        entry!(0x00fd_7f30, fn_00fd7f30()),
        entry!(0x00fd_8c20, fn_00fd8c20()),
        entry!(0x00fd_a000, fn_00fda000()),
        entry!(0x00fd_a020, fn_00fda020()),
        entry!(0x00fd_a0a0, fn_00fda0a0()),
        entry!(0x00fd_a0f0, fn_00fda0f0()),
        entry!(0x00fd_a140, fn_00fda140()),
        entry!(0x00fd_a150, fn_00fda150()),
        entry!(0x00fd_a170, fn_00fda170()),
        entry!(0x00fd_a190, fn_00fda190()),
        entry!(0x00fd_a210, fn_00fda210()),
        entry!(0x00fd_a2b0, fn_00fda2b0()),
        entry!(0x00fd_a330, fn_00fda330()),
        entry!(0x00fd_a3b0, fn_00fda3b0()),
        entry!(0x00fd_a3c0, fn_00fda3c0()),
        entry!(0x00fd_a440, fn_00fda440()),
        entry!(0x00fd_a4c0, fn_00fda4c0()),
        entry!(0x00fd_a540, fn_00fda540()),
        entry!(0x00fd_a5c0, fn_00fda5c0()),
        entry!(0x00fd_a640, fn_00fda640()),
        entry!(0x00fd_a6c0, fn_00fda6c0()),
        entry!(0x00fd_a740, fn_00fda740()),
        entry!(0x00fd_a7c0, fn_00fda7c0()),
        entry!(0x00fd_a7e0, fn_00fda7e0()),
        entry!(0x00fd_a800, fn_00fda800()),
        entry!(0x00fd_a830, fn_00fda830()),
        entry!(0x00fd_a8b0, fn_00fda8b0()),
        entry!(0x00fd_a930, fn_00fda930()),
        entry!(0x00fd_a9b0, fn_00fda9b0()),
        entry!(0x00fd_aa30, fn_00fdaa30()),
        entry!(0x00fd_aab0, fn_00fdaab0()),
        entry!(0x00fd_ab30, fn_00fdab30()),
        entry!(0x00fd_abb0, fn_00fdabb0()),
        entry!(0x00fd_ac30, fn_00fdac30()),
        entry!(0x00fd_acb0, fn_00fdacb0()),
        entry!(0x00fd_add0, fn_00fdadd0()),
        entry!(0x00fd_adf0, fn_00fdadf0()),
        entry!(0x00fd_ae40, fn_00fdae40()),
        entry!(0x00fd_ae60, fn_00fdae60()),
        entry!(0x00fd_ae70, fn_00fdae70()),
        entry!(0x00fd_aea0, fn_00fdaea0()),
        entry!(0x00fd_aed0, fn_00fdaed0()),
        entry!(0x00fd_af00, fn_00fdaf00()),
        entry!(0x00fd_af10, fn_00fdaf10()),
        entry!(0x00fd_af50, fn_00fdaf50()),
        entry!(0x00fd_af60, fn_00fdaf60()),
        entry!(0x00fd_af70, fn_00fdaf70()),
        entry!(0x00fd_af80, fn_00fdaf80()),
        entry!(0x00fd_afb0, fn_00fdafb0()),
        entry!(0x00fd_afe0, fn_00fdafe0()),
        entry!(0x00fd_b000, fn_00fdb000()),
        entry!(0x00fd_b030, fn_00fdb030()),
        entry!(0x00fd_b060, fn_00fdb060()),
        entry!(0x00fd_b090, fn_00fdb090()),
        entry!(0x00fd_b0c0, fn_00fdb0c0()),
        entry!(0x00fd_b0d0, fn_00fdb0d0()),
        entry!(0x00fd_b100, fn_00fdb100()),
        entry!(0x00fd_b170, fn_00fdb170()),
        entry!(0x00fd_b1a0, fn_00fdb1a0()),
        entry!(0x00fd_b1d0, fn_00fdb1d0()),
        entry!(0x00fd_b200, fn_00fdb200()),
        entry!(0x00fd_b260, fn_00fdb260()),
        entry!(0x00fd_b270, fn_00fdb270()),
        entry!(0x00fd_b2a0, fn_00fdb2a0()),
        entry!(0x00fd_b2b0, fn_00fdb2b0()),
        entry!(0x00fd_b2c0, fn_00fdb2c0()),
        entry!(0x00fd_b340, fn_00fdb340()),
        entry!(0x00fd_b3c0, fn_00fdb3c0()),
        entry!(0x00fd_b440, fn_00fdb440()),
        entry!(0x00fd_b4c0, fn_00fdb4c0()),
        entry!(0x00fd_b540, fn_00fdb540()),
        entry!(0x00fd_b5c0, fn_00fdb5c0()),
        entry!(0x00fd_b640, fn_00fdb640()),
        entry!(0x00fd_b650, fn_00fdb650()),
        entry!(0x00fd_b6d0, fn_00fdb6d0()),
        entry!(0x00fd_b750, fn_00fdb750()),
        entry!(0x00fd_b7d0, fn_00fdb7d0()),
        entry!(0x00fd_b850, fn_00fdb850()),
        entry!(0x00fd_b8d0, fn_00fdb8d0()),
        entry!(0x00fd_b950, fn_00fdb950()),
        entry!(0x00fd_b9d0, fn_00fdb9d0()),
        entry!(0x00fd_ba50, fn_00fdba50()),
        entry!(0x00fd_bad0, fn_00fdbad0()),
        entry!(0x00fd_bb50, fn_00fdbb50()),
        entry!(0x00fd_bbd0, fn_00fdbbd0()),
        entry!(0x00fd_bc50, fn_00fdbc50()),
        entry!(0x00fd_bcd0, fn_00fdbcd0()),
        entry!(0x00fd_bd50, fn_00fdbd50()),
        entry!(0x00fd_bdd0, fn_00fdbdd0()),
        entry!(0x00fd_be50, fn_00fdbe50()),
        entry!(0x00fd_bed0, fn_00fdbed0()),
        entry!(0x00fd_bf50, fn_00fdbf50()),
        entry!(0x00fd_bff0, fn_00fdbff0()),
        entry!(0x00fd_c070, fn_00fdc070()),
        entry!(0x00fd_c0f0, fn_00fdc0f0()),
        entry!(0x00fd_c170, fn_00fdc170()),
        entry!(0x00fd_c1f0, fn_00fdc1f0()),
        entry!(0x00fd_c270, fn_00fdc270()),
        entry!(0x00fd_c370, fn_00fdc370()),
        entry!(0x00fd_c380, fn_00fdc380()),
        entry!(0x00fd_c3c0, fn_00fdc3c0()),
        entry!(0x00fd_c3f0, fn_00fdc3f0()),
        entry!(0x00fd_c410, fn_00fdc410()),
        entry!(0x00fd_c480, fn_00fdc480()),
        entry!(0x00fd_c490, fn_00fdc490()),
        entry!(0x00fd_c4b0, fn_00fdc4b0()),
        entry!(0x00fd_c4d0, fn_00fdc4d0()),
        entry!(0x00fd_c530, fn_00fdc530()),
        entry!(0x00fd_c590, fn_00fdc590()),
        entry!(0x00fd_c5f0, fn_00fdc5f0()),
        entry!(0x00fd_c660, fn_00fdc660()),
        entry!(0x00fd_c690, fn_00fdc690()),
        entry!(0x00fd_c720, fn_00fdc720()),
        entry!(0x00fd_c750, fn_00fdc750()),
        entry!(0x00fd_c7a0, fn_00fdc7a0()),
        entry!(0x00fd_c7b0, fn_00fdc7b0()),
        entry!(0x00fd_c7e0, fn_00fdc7e0()),
        entry!(0x00fd_c810, fn_00fdc810()),
        entry!(0x00fd_c840, fn_00fdc840()),
        entry!(0x00fd_c870, fn_00fdc870()),
        entry!(0x00fd_c8a0, fn_00fdc8a0()),
        entry!(0x00fd_c8d0, fn_00fdc8d0()),
        entry!(0x00fd_c900, fn_00fdc900()),
        entry!(0x00fd_c930, fn_00fdc930()),
        entry!(0x00fd_c960, fn_00fdc960()),
        entry!(0x00fd_c990, fn_00fdc990()),
        entry!(0x00fd_c9c0, fn_00fdc9c0()),
        entry!(0x00fd_c9f0, fn_00fdc9f0()),
        entry!(0x00fd_ca20, fn_00fdca20()),
        entry!(0x00fd_ca50, fn_00fdca50()),
        entry!(0x00fd_ca80, fn_00fdca80()),
        entry!(0x00fd_cab0, fn_00fdcab0()),
        entry!(0x00fd_cae0, fn_00fdcae0()),
        entry!(0x00fd_cb10, fn_00fdcb10()),
        entry!(0x00fd_cb40, fn_00fdcb40()),
        entry!(0x00fd_cb60, fn_00fdcb60()),
        entry!(0x00fd_cb90, fn_00fdcb90()),
        entry!(0x00fd_cbc0, fn_00fdcbc0()),
        entry!(0x00fd_cca0, fn_00fdcca0()),
        entry!(0x00fd_ccd0, fn_00fdccd0()),
        entry!(0x00fd_cd00, fn_00fdcd00()),
        entry!(0x00fd_cd20, fn_00fdcd20()),
        entry!(0x00fd_cd40, fn_00fdcd40()),
        entry!(0x00fd_cd60, fn_00fdcd60()),
        entry!(0x00fd_cd80, fn_00fdcd80()),
        entry!(0x00fd_cda0, fn_00fdcda0()),
        entry!(0x00fd_cdd0, fn_00fdcdd0()),
        entry!(0x00fd_cdf0, fn_00fdcdf0()),
        entry!(0x00fd_ce40, fn_00fdce40()),
        entry!(0x00fd_ce80, fn_00fdce80()),
        entry!(0x00fd_cef0, fn_00fdcef0()),
        entry!(0x00fd_cf30, fn_00fdcf30()),
        entry!(0x00fd_cf50, fn_00fdcf50()),
        entry!(0x00fd_cf70, fn_00fdcf70()),
        entry!(0x00fd_cf90, fn_00fdcf90()),
        entry!(0x00fd_cfc0, fn_00fdcfc0()),
        entry!(0x00fd_cfe0, fn_00fdcfe0()),
        entry!(0x00fd_d010, fn_00fdd010()),
        entry!(0x00fd_d040, fn_00fdd040()),
        entry!(0x00fd_d070, fn_00fdd070()),
        entry!(0x00fd_d0a0, fn_00fdd0a0()),
        entry!(0x00fd_d0d0, fn_00fdd0d0()),
        entry!(0x00fd_d0f0, fn_00fdd0f0()),
        entry!(0x00fd_d110, fn_00fdd110()),
        entry!(0x00fd_d130, fn_00fdd130()),
        entry!(0x00fd_d150, fn_00fdd150()),
        entry!(0x00fd_d170, fn_00fdd170()),
        entry!(0x00fd_d190, fn_00fdd190()),
        entry!(0x00fd_d1c0, fn_00fdd1c0()),
        entry!(0x00fd_d1f0, fn_00fdd1f0()),
        entry!(0x00fd_d220, fn_00fdd220()),
        entry!(0x00fd_d250, fn_00fdd250()),
        entry!(0x00fd_d280, fn_00fdd280()),
        entry!(0x00fd_d2b0, fn_00fdd2b0()),
        entry!(0x00fd_d2e0, fn_00fdd2e0()),
        entry!(0x00fd_d310, fn_00fdd310()),
        entry!(0x00fd_d330, fn_00fdd330()),
        entry!(0x00fd_d350, fn_00fdd350()),
        entry!(0x00fd_d410, fn_00fdd410()),
        entry!(0x00fd_d430, fn_00fdd430()),
        entry!(0x00fd_d440, fn_00fdd440()),
        entry!(0x00fd_d4c0, fn_00fdd4c0()),
        entry!(0x00fd_d540, fn_00fdd540()),
        entry!(0x00fd_d5c0, fn_00fdd5c0()),
        entry!(0x00fd_d640, fn_00fdd640()),
        entry!(0x00fd_d690, fn_00fdd690()),
        entry!(0x00fd_d6d0, fn_00fdd6d0()),
        entry!(0x00fd_d750, fn_00fdd750()),
        entry!(0x00fd_d7d0, fn_00fdd7d0()),
        entry!(0x00fd_d850, fn_00fdd850()),
        entry!(0x00fd_d8d0, fn_00fdd8d0()),
        entry!(0x00fd_d950, fn_00fdd950()),
        entry!(0x00fd_d9d0, fn_00fdd9d0()),
        entry!(0x00fd_da50, fn_00fdda50()),
        entry!(0x00fd_dad0, fn_00fddad0()),
        entry!(0x00fd_db50, fn_00fddb50()),
        entry!(0x00fd_dbd0, fn_00fddbd0()),
        entry!(0x00fd_dc50, fn_00fddc50()),
        entry!(0x00fd_dcd0, fn_00fddcd0()),
        entry!(0x00fd_dce0, fn_00fddce0()),
        entry!(0x00fd_dd20, fn_00fddd20()),
        entry!(0x00fd_dd30, fn_00fddd30()),
        entry!(0x00fd_de30, fn_00fdde30()),
        entry!(0x00fd_e140, fn_00fde140()),
        entry!(0x00fd_e150, fn_00fde150()),
        entry!(0x00fd_e160, fn_00fde160()),
        entry!(0x00fd_e190, fn_00fde190()),
        entry!(0x00fd_e1a0, fn_00fde1a0()),
        entry!(0x00fd_e220, fn_00fde220()),
        entry!(0x00fd_e250, fn_00fde250()),
        entry!(0x00fd_e270, fn_00fde270()),
        entry!(0x00fd_e280, fn_00fde280()),
        entry!(0x00fd_e290, fn_00fde290()),
        entry!(0x00fd_e2b0, fn_00fde2b0()),
        entry!(0x00fd_e2d0, fn_00fde2d0()),
        entry!(0x00fd_e2e0, fn_00fde2e0()),
        entry!(0x00fd_e330, fn_00fde330()),
        entry!(0x00fd_e370, fn_00fde370()),
        entry!(0x00fd_e3c0, fn_00fde3c0()),
        entry!(0x00fd_e3d0, fn_00fde3d0()),
        entry!(0x00fd_e450, fn_00fde450()),
        entry!(0x00fd_e4d0, fn_00fde4d0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    type Calls = Vec<(u32, Vec<u32>)>;

    /// A heap-less stand-in object address, its vtable and a virtual destructor.
    const OBJECT: u32 = 0x0800_0000;
    const VTABLE: u32 = 0x0900_0000;
    const VIRTUAL_TARGET: u32 = 0x0a00_0010;

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

    /// `InterlockedDecrement` as the platform stand-in does it: decrement in memory, return the
    /// new value.
    fn interlocked_decrement_double(e: &mut Engine, a: &[u32]) -> Ret {
        let v = e.mem.i32(a[0]).wrapping_sub(1);
        e.mem.set_i32(a[0], v);
        Ret {
            eax: v as u32,
            ..Ret::default()
        }
    }

    /// An engine with `InterlockedDecrement` and the virtual destructor (vtable slot +4) doubled.
    fn counted_engine() -> Engine {
        let mut e = Engine::new();
        e.register(INTERLOCKED_DECREMENT, interlocked_decrement_double);
        quiet(&mut e, &[VIRTUAL_TARGET]);
        e.put_vtable(VTABLE, &[0, VIRTUAL_TARGET, VIRTUAL_TARGET + 4]);
        e.map(OBJECT, 0x1000);
        e
    }

    fn map_word(e: &mut Engine, addr: u32) {
        e.map(addr, 4);
    }

    /// (function, global of the word, callee)
    const CALL_WITH_WORD: &[(u32, u32, u32)] = &[(0x00fd_b100, 0x011f_5e44, 0x00aa_10f0)];

    /// (function, halfword zeroed, global of the pointer, callee, pointer zeroed)
    const CLEAR_AND_FREE: &[(u32, u32, u32, u32, u32)] = &[(
        0x00fd_c660,
        0x011f_8d84,
        0x011f_8d88,
        0x0040_1030,
        0x011f_8d88,
    )];

    /// (function, critical section)
    const CRITICAL_SECTIONS: &[(u32, u32)] = &[
        (0x00fd_a140, 0x011f_3330),
        (0x00fd_a3b0, 0x011f_33dc),
        (0x00fd_ae40, 0x011f_4180),
        (0x00fd_ae60, 0x011f_4380),
        (0x00fd_af00, 0x011f_4580),
        (0x00fd_af50, 0x011f_4800),
        (0x00fd_af70, 0x011f_4900),
        (0x00fd_b0c0, 0x011f_5c80),
        (0x00fd_b260, 0x011f_600c),
        (0x00fd_b2a0, 0x011f_6bb8),
        (0x00fd_b2b0, 0x011f_6bd4),
        (0x00fd_c370, 0x011f_8170),
        (0x00fd_c3f0, 0x011f_8254),
        (0x00fd_c480, 0x011f_8bc4),
        (0x00fd_d410, 0x0120_2600),
        (0x00fd_d430, 0x0120_2d78),
        (0x00fd_dcd0, 0x0120_3328),
        (0x00fd_e140, 0x0126_7e48),
        (0x00fd_e160, 0x0126_813c),
        (0x00fd_e1a0, 0x0126_820c),
        (0x00fd_e330, 0x0126_f600),
    ];

    /// (function, global, destructor, free)
    const DESTRUCT_THEN_FREE: &[(u32, u32, u32, u32)] = &[
        (0x00fd_c4d0, 0x011f_8c68, 0x00b1_7f50, 0x0040_1030),
        (0x00fd_c530, 0x011f_8c18, 0x00b1_7f50, 0x0040_1030),
        (0x00fd_c590, 0x011f_8c50, 0x00b1_7f50, 0x0040_1030),
        (0x00fd_c5f0, 0x011f_8c80, 0x00b1_7f50, 0x0040_1030),
        (0x00fd_c690, 0x011f_8e2c, 0x00b3_ba90, 0x0040_1030),
    ];

    /// (function, global, first callee, global of the pointer, object, second callee)
    const POOL_RELEASES: &[(u32, u32, u32, u32, u32, u32)] = &[(
        0x00fd_c410,
        0x011f_8bdc,
        0x00b0_e7f0,
        0x011f_8bdc,
        0x011f_6238,
        0x00aa_4060,
    )];

    /// (function, global, vtable stored, singleton getter, singleton global)
    const SINGLETON_DESTRUCTORS: &[(u32, u32, u32, u32, u32)] = &[
        (
            0x00fd_a210,
            0x011f_3368,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a2b0,
            0x011f_33bc,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a330,
            0x011f_33c8,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a3c0,
            0x011f_3414,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a440,
            0x011f_3420,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a4c0,
            0x011f_342c,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a540,
            0x011f_3438,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a5c0,
            0x011f_3444,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a640,
            0x011f_3450,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a6c0,
            0x011f_345c,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a740,
            0x011f_3468,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a830,
            0x011f_35d0,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a8b0,
            0x011f_35dc,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a930,
            0x011f_35e8,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_a9b0,
            0x011f_35f4,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_aa30,
            0x011f_3600,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_aab0,
            0x011f_360c,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_ab30,
            0x011f_3618,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_abb0,
            0x011f_3624,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_ac30,
            0x011f_3638,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_acb0,
            0x011f_3644,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_b2c0,
            0x011f_6d9c,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_b340,
            0x011f_6da8,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_b3c0,
            0x011f_6db4,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_b440,
            0x011f_6dc0,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_b4c0,
            0x011f_6dcc,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_b540,
            0x011f_6dd8,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_b5c0,
            0x011f_6de4,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_b650,
            0x011f_6dfc,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_b6d0,
            0x011f_6e14,
            0x0102_2738,
            0x004d_e4a0,
            0x011f_35a0,
        ),
        (
            0x00fd_b750,
            0x011f_6e20,
            0x0102_2738,
            0x004d_e4a0,
            0x011f_35a0,
        ),
        (
            0x00fd_b7d0,
            0x011f_6e2c,
            0x0102_2738,
            0x004d_e4a0,
            0x011f_35a0,
        ),
        (
            0x00fd_b850,
            0x011f_6e38,
            0x0102_2738,
            0x004d_e4a0,
            0x011f_35a0,
        ),
        (
            0x00fd_b8d0,
            0x011f_6e44,
            0x0102_2738,
            0x004d_e4a0,
            0x011f_35a0,
        ),
        (
            0x00fd_b950,
            0x011f_6e50,
            0x0102_2738,
            0x004d_e4a0,
            0x011f_35a0,
        ),
        (
            0x00fd_b9d0,
            0x011f_6e5c,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_ba50,
            0x011f_6e68,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_bad0,
            0x011f_6e74,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_bb50,
            0x011f_6e80,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_bbd0,
            0x011f_6e8c,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_bc50,
            0x011f_6e98,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_bcd0,
            0x011f_6ea4,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_bd50,
            0x011f_6eb0,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_bdd0,
            0x011f_6ebc,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_be50,
            0x011f_6ec8,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_bed0,
            0x011f_6ed4,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_bf50,
            0x011f_6ee0,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_bff0,
            0x011f_7130,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_c070,
            0x011f_713c,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_c0f0,
            0x011f_7148,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_c170,
            0x011f_7154,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_c1f0,
            0x011f_7160,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_c270,
            0x011f_716c,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_d440,
            0x0120_2da8,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_d4c0,
            0x0120_2db4,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_d540,
            0x0120_2dc0,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_d5c0,
            0x0120_2dcc,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_d6d0,
            0x0120_3120,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_d750,
            0x0120_312c,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_d7d0,
            0x0120_3138,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_d850,
            0x0120_3144,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_d8d0,
            0x0120_3150,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_d950,
            0x0120_315c,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_d9d0,
            0x0120_3168,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_da50,
            0x0120_3174,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_dad0,
            0x0120_3188,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_db50,
            0x0120_3194,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_dbd0,
            0x0120_31a0,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_dc50,
            0x0120_31ac,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_e3d0,
            0x0126_fac8,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_e450,
            0x0126_fad4,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
        (
            0x00fd_e4d0,
            0x0126_fae0,
            0x0101_7720,
            0x0044_f570,
            0x011f_96a0,
        ),
    ];

    /// (function, global, first vtable, second vtable, callee, global of the word, last callee)
    const TWO_VTABLE_DESTRUCTORS: &[(u32, u32, u32, u32, u32, u32, u32)] = &[
        (
            0x00fd_a020,
            0x011f_3308,
            0x0109_4e3c,
            0x0109_4e5c,
            0x00ae_91f0,
            0x011f_3310,
            0x00aa_10f0,
        ),
        (
            0x00fd_a190,
            0x011f_3358,
            0x0109_4e7c,
            0x0109_4e9c,
            0x00ae_91f0,
            0x011f_3360,
            0x00aa_10f0,
        ),
    ];

    /// (function, import slot, global)
    const IMPORT_ON_GLOBAL: &[(u32, u32, u32)] = &[
        (0x00fd_af60, 0x00fd_f07c, 0x011f_4798),
        (0x00fd_e150, 0x00fd_f050, 0x0126_8108),
        (0x00fd_e190, 0x00fd_f050, 0x0126_81d8),
        (0x00fd_e250, 0x00fd_f050, 0x0126_835c),
        (0x00fd_e270, 0x00fd_f050, 0x0126_87c8),
    ];

    /// (function)
    const NO_OPS: &[u32] = &[0x00fd_b640, 0x00fd_c7a0];

    /// (function, end of the array, loop counter start)
    const RELEASE_OBJECT_ARRAYS: &[(u32, u32, u32)] = &[
        (0x00fd_af10, 0x011f_453c, 0x9),
        (0x00fd_e2e0, 0x0126_f508, 0xff),
    ];

    /// (function, global holding the pointer, offset added to it)
    const RELEASE_REFS: &[(u32, u32, u32)] = &[
        (0x00fd_ae70, 0x011f_4410, 0),
        (0x00fd_aea0, 0x011f_4438, 0),
        (0x00fd_aed0, 0x011f_4444, 0),
        (0x00fd_afb0, 0x011f_49ac, 0),
        (0x00fd_b000, 0x011f_4a14, 0),
        (0x00fd_b030, 0x011f_4a64, 0),
        (0x00fd_b060, 0x011f_4a7c, 0),
        (0x00fd_b090, 0x011f_5ae8, 0),
        (0x00fd_b0d0, 0x011f_5d28, 0),
        (0x00fd_b1a0, 0x011f_5ed4, 0),
        (0x00fd_b1d0, 0x011f_5f3c, 0),
        (0x00fd_b200, 0x011f_5f50, 0),
        (0x00fd_c380, 0x011f_8188, 0x158),
        (0x00fd_c750, 0x011f_9154, 0),
        (0x00fd_c7b0, 0x011f_950c, 0),
        (0x00fd_c7e0, 0x011f_9510, 0),
        (0x00fd_c810, 0x011f_9514, 0),
        (0x00fd_c840, 0x011f_9518, 0),
        (0x00fd_c870, 0x011f_951c, 0),
        (0x00fd_c8a0, 0x011f_9520, 0),
        (0x00fd_c8d0, 0x011f_9524, 0),
        (0x00fd_c900, 0x011f_9528, 0),
        (0x00fd_c930, 0x011f_952c, 0),
        (0x00fd_c960, 0x011f_9530, 0),
        (0x00fd_c990, 0x011f_9534, 0),
        (0x00fd_c9c0, 0x011f_9538, 0),
        (0x00fd_c9f0, 0x011f_953c, 0),
        (0x00fd_ca20, 0x011f_9540, 0),
        (0x00fd_ca50, 0x011f_9544, 0),
        (0x00fd_ca80, 0x011f_95d4, 0),
        (0x00fd_cab0, 0x011f_95e4, 0),
        (0x00fd_cae0, 0x011f_95e8, 0),
        (0x00fd_cb10, 0x011f_9508, 0),
        (0x00fd_cb60, 0x011f_95d8, 0),
        (0x00fd_cb90, 0x011f_95ec, 0),
        (0x00fd_cbc0, 0x011f_95f0, 0),
        (0x00fd_cca0, 0x011f_a008, 0),
        (0x00fd_cda0, 0x011f_edc4, 0),
        (0x00fd_cef0, 0x0120_03f4, 0),
        (0x00fd_cf90, 0x0120_07c4, 0),
        (0x00fd_cfe0, 0x0120_0810, 0),
        (0x00fd_d010, 0x0120_0814, 0),
        (0x00fd_d040, 0x0120_0818, 0),
        (0x00fd_d070, 0x0120_083c, 0),
        (0x00fd_d0a0, 0x0120_0840, 0),
        (0x00fd_d190, 0x0120_1f40, 0),
        (0x00fd_d1c0, 0x0120_1f44, 0),
        (0x00fd_d1f0, 0x0120_1f48, 0),
        (0x00fd_d220, 0x0120_1f58, 0),
        (0x00fd_d250, 0x0120_1fc4, 0),
        (0x00fd_d280, 0x0120_1fd0, 0),
        (0x00fd_d2b0, 0x0120_21e0, 0),
        (0x00fd_d2e0, 0x0120_21e4, 0),
        (0x00fd_d690, 0x0120_2e14, 0),
        (0x00fd_dce0, 0x0120_43c0, 0),
        (0x00fd_de30, 0x0126_7be0, 0),
        (0x00fd_e220, 0x0126_82f0, 0),
    ];

    /// (function, global, vtable stored, global of the pointer, callee)
    const SCALAR_DELETES: &[(u32, u32, u32, u32, u32)] = &[(
        0x00fd_d640,
        0x0120_2e20,
        0x010c_1c5c,
        0x0120_2e24,
        0x00c4_6420,
    )];

    /// (function, global of the block (cleared first), destructed global, vtable stored, slot, two more cleared words)
    const SLOT_CLEARS: &[(u32, u32, u32, u32, u32, u32, u32)] = &[
        (
            0x00fd_ce40,
            0x011a_da04,
            0x011a_da00,
            0x010b_7f00,
            0x010b_7f08,
            0x011a_da0c,
            0x011a_da08,
        ),
        (
            0x00fd_ce80,
            0x011a_da14,
            0x011a_da10,
            0x010b_7f00,
            0x010b_7f08,
            0x011a_da1c,
            0x011a_da18,
        ),
    ];

    /// (function, global, value)
    const STORES: &[(u32, u32, u32)] = &[
        (0x00fd_c720, 0x011f_8ebc, 0x010a_58c8),
        (0x00fd_dd20, 0x0126_782c, 0x010a_47b8),
        (0x00fd_dd30, 0x0126_7b0c, 0x010a_47b8),
        (0x00fd_e280, 0x0126_87fc, 0x0102_e388),
        (0x00fd_e290, 0x0126_8b0c, 0x0102_e388),
        (0x00fd_e2b0, 0x011b_7c00, 0x0102_e388),
        (0x00fd_e2d0, 0x011b_dee0, 0x010e_ad08),
        (0x00fd_e3c0, 0x0126_fa68, 0x010f_16f4),
    ];

    /// (function, global of the word, destructed global, vtable stored, callee)
    const STORE_THEN_CALL_WITH_WORD: &[(u32, u32, u32, u32, u32)] = &[
        (
            0x00fd_a800,
            0x011f_350c,
            0x011f_3508,
            0x0109_568c,
            0x00aa_10f0,
        ),
        (
            0x00fd_b270,
            0x011a_c044,
            0x011a_c040,
            0x010a_1458,
            0x00a4_7600,
        ),
        (
            0x00fd_cfc0,
            0x011a_e5dc,
            0x011a_e5d8,
            0x010b_9d24,
            0x00aa_10f0,
        ),
    ];

    /// (function, global, vtable stored, callee)
    const STORE_THEN_THISCALL: &[(u32, u32, u32, u32)] =
        &[(0x00fd_a000, 0x011f_32f4, 0x0109_4d9c, 0x00a0_d2a0)];

    /// (function, global holding the string pointer)
    const RELEASE_STRINGS: &[(u32, u32)] = &[
        (0x00fd_add0, 0x011f_3ec0),
        (0x00fd_adf0, 0x011f_3ec4),
        (0x00fd_afe0, 0x011f_49e0),
        (0x00fd_e370, 0x0126_f750),
    ];

    /// (function, end of the array, loop counter start)
    const RELEASE_STRING_ARRAYS: &[(u32, u32, u32)] = &[
        (0x00fd_af80, 0x011f_49d8, 0x9),
        (0x00fd_b170, 0x011f_5eac, 0x3),
    ];

    /// (function, global, callee, first parameter, second parameter)
    const THISCALLS_WITH_TWO_ARGUMENTS: &[(u32, u32, u32, u32, u32)] = &[
        (0x00fd_c490, 0x011f_8ba4, 0x0044_a730, 0x1, 0x0),
        (0x00fd_c4b0, 0x011f_8b84, 0x0044_a730, 0x1, 0x0),
    ];

    /// (function, global, callee)
    const THISCALLS: &[(u32, u32, u32)] = &[
        (0x00fd_a0a0, 0x011f_3318, 0x004e_d900),
        (0x00fd_a0f0, 0x011f_3324, 0x004e_d900),
        (0x00fd_a150, 0x011f_3348, 0x00ae_83e0),
        (0x00fd_a170, 0x011f_3350, 0x00ae_83e0),
        (0x00fd_a7c0, 0x011f_34e0, 0x00a1_e900),
        (0x00fd_a7e0, 0x011f_34f4, 0x00a1_ec70),
    ];

    /// (function, array, element size, count, element destructor)
    const VECTOR_DESTRUCTORS: &[(u32, u32, u32, u32, u32)] = &[
        (0x00fc_9090, 0x011c_5d10, 0x18, 0x14, 0x0048_9070),
        (0x00fc_9a10, 0x011c_7804, 0x4, 0x3, 0x0045_cec0),
        (0x00fc_9a60, 0x011c_7860, 0x4, 0x3, 0x0045_cec0),
        (0x00fc_b260, 0x011c_b3e0, 0x8, 0xe, 0x0046_ffb0),
        (0x00fc_b280, 0x011c_b450, 0x8, 0xe, 0x0046_ffb0),
        (0x00fc_b8c0, 0x011c_bf60, 0x54, 0x2, 0x005a_a1a0),
        (0x00fd_7ad0, 0x011d_8828, 0x1c, 0x3, 0x0070_1500),
        (0x00fd_7f30, 0x011d_a1b0, 0xc, 0x5, 0x0044_f4f0),
        (0x00fd_8c20, 0x011d_d9e8, 0x24, 0x14, 0x0083_b7d0),
        (0x00fd_c3c0, 0x011f_8190, 0x4, 0x12, 0x00af_c720),
        (0x00fd_cb40, 0x011f_9548, 0x4, 0x23, 0x00b6_6f20),
        (0x00fd_ccd0, 0x011f_a024, 0x4, 0xa, 0x00b6_6f20),
        (0x00fd_cd00, 0x011f_da48, 0x4, 0xd0, 0x00b6_6f20),
        (0x00fd_cd20, 0x011f_dd88, 0x4, 0x9c, 0x00b6_6f20),
        (0x00fd_cd40, 0x011f_ebd8, 0x4, 0x16, 0x0045_cec0),
        (0x00fd_cd60, 0x011f_ec30, 0x4, 0x16, 0x0045_cec0),
        (0x00fd_cd80, 0x011f_dff8, 0x4, 0x2f7, 0x00b7_9930),
        (0x00fd_cdd0, 0x011f_f4a0, 0x4, 0x3, 0x0045_cec0),
        (0x00fd_cdf0, 0x011f_f4ac, 0x4, 0x6, 0x0045_cec0),
        (0x00fd_cf30, 0x0120_05c8, 0x4, 0x2, 0x00b6_6f20),
        (0x00fd_cf50, 0x0120_05d0, 0x4, 0x4, 0x00b6_6f20),
        (0x00fd_cf70, 0x0120_05e0, 0x4, 0x1, 0x00b7_9930),
        (0x00fd_d0d0, 0x0120_0c48, 0x4, 0x32, 0x00b6_6f20),
        (0x00fd_d0f0, 0x0120_0d10, 0x4, 0x32, 0x00b6_6f20),
        (0x00fd_d110, 0x0120_0dd8, 0x4, 0x39, 0x00b6_6f20),
        (0x00fd_d130, 0x0120_1080, 0x4, 0x11, 0x0045_cec0),
        (0x00fd_d150, 0x0120_10c4, 0x4, 0xa, 0x0045_cec0),
        (0x00fd_d170, 0x0120_0ec0, 0x4, 0x70, 0x00b7_9930),
        (0x00fd_d310, 0x0120_2230, 0x4, 0x21, 0x00b7_9930),
        (0x00fd_d330, 0x0120_22c0, 0x4, 0x1a, 0x00b7_9930),
        (0x00fd_d350, 0x0120_2350, 0x4, 0x1e, 0x00b7_9930),
    ];

    /// Every address of the file is registered exactly once.
    #[test]
    fn every_function_is_registered_once() {
        let funcs = funcs();
        assert_eq!(funcs.len(), 229);
        let mut addrs: Vec<u32> = funcs.iter().map(|(a, _)| *a).collect();
        let sorted = addrs.windows(2).all(|w| w[0] < w[1]);
        assert!(sorted, "ascending and unique");
        addrs.dedup();
        assert_eq!(addrs.len(), 229);
        let e = Engine::new();
        for a in addrs {
            assert!(e.is_translated(a), "{a:08x}");
        }
    }

    #[test]
    fn vector_destructors_pass_array_size_count_and_destructor() {
        let mut e = Engine::new();
        quiet(&mut e, &[EH_VECTOR_DESTRUCTOR_ITERATOR]);
        assert_eq!(VECTOR_DESTRUCTORS.len(), 31);
        for &(f, array, size, count, destructor) in VECTOR_DESTRUCTORS {
            assert_eq!(
                run(&mut e, f),
                vec![(
                    EH_VECTOR_DESTRUCTOR_ITERATOR,
                    vec![array, size, count, destructor]
                )],
                "{f:08x}"
            );
        }
    }

    #[test]
    fn critical_sections_are_deleted() {
        let mut e = Engine::new();
        quiet(&mut e, &[DELETE_CRITICAL_SECTION]);
        assert_eq!(CRITICAL_SECTIONS.len(), 21);
        for &(f, section) in CRITICAL_SECTIONS {
            assert_eq!(
                run(&mut e, f),
                vec![(DELETE_CRITICAL_SECTION, vec![section])],
                "{f:08x}"
            );
        }
    }

    #[test]
    fn handles_go_to_their_import() {
        let mut e = Engine::new();
        assert_eq!(IMPORT_ON_GLOBAL.len(), 5);
        for &(f, import, global) in IMPORT_ON_GLOBAL {
            quiet(&mut e, &[import]);
            map_word(&mut e, global);
            e.set_global::<u32>(global, 0x1234_5678);
            assert_eq!(run(&mut e, f), vec![(import, vec![0x1234_5678])], "{f:08x}");
        }
        let imports: Vec<u32> = IMPORT_ON_GLOBAL.iter().map(|r| r.1).collect();
        assert!(imports
            .iter()
            .all(|i| *i == 0x00fd_f07c || *i == 0x00fd_f050));
    }

    #[test]
    fn vtable_stores_write_the_global_and_call_nothing() {
        let mut e = Engine::new();
        assert_eq!(STORES.len(), 8);
        for &(f, global, value) in STORES {
            map_word(&mut e, global);
            e.set_global::<u32>(global, 0);
            assert_eq!(run(&mut e, f), vec![], "{f:08x}");
            assert_eq!(e.global::<u32>(global), value, "{f:08x}");
        }
    }

    #[test]
    fn empty_functions_do_nothing() {
        let mut e = Engine::new();
        for &f in NO_OPS {
            assert_eq!(run(&mut e, f), vec![], "{f:08x}");
        }
    }

    /// A thiscall on a global (`ECX` = the global, nothing on the stack), with and without the
    /// `FS:[0]` frame.
    #[test]
    fn destructors_run_on_their_global() {
        let mut e = Engine::new();
        assert_eq!(THISCALLS.len(), 6);
        for &(f, global, callee) in THISCALLS {
            quiet(&mut e, &[callee]);
            assert_eq!(run(&mut e, f), vec![(callee, vec![global])], "{f:08x}");
        }
    }

    /// `PUSH 0; PUSH 1; MOV ECX,global; CALL 0044a730` (`RET 8`): the last push is the first
    /// parameter.
    #[test]
    fn two_stack_arguments_are_in_parameter_order() {
        let mut e = Engine::new();
        assert_eq!(THISCALLS_WITH_TWO_ARGUMENTS.len(), 2);
        for &(f, global, callee, first, second) in THISCALLS_WITH_TWO_ARGUMENTS {
            quiet(&mut e, &[callee]);
            assert_eq!((first, second), (1, 0));
            assert_eq!(
                run(&mut e, f),
                vec![(callee, vec![global, 1, 0])],
                "{f:08x}"
            );
        }
    }

    #[test]
    fn vtable_is_stored_before_the_destructor_runs() {
        let mut e = Engine::new();
        let seen = Rc::new(RefCell::new(vec![]));
        assert_eq!(STORE_THEN_THISCALL.len(), 1);
        for &(f, global, vtable, callee) in STORE_THEN_THISCALL {
            map_word(&mut e, global);
            e.set_global::<u32>(global, 0);
            let log = seen.clone();
            e.register_double(callee, move |e, a| {
                log.borrow_mut().push(e.global::<u32>(a[0]));
                Ret::default()
            });
            assert_eq!(run(&mut e, f), vec![(callee, vec![global])], "{f:08x}");
            assert_eq!(*seen.borrow(), vec![vtable]);
        }
    }

    /// The release of a reference-counted object: decrement the count at +4, and the virtual
    /// destructor only when the count reaches 0.
    #[test]
    fn reference_releases_destruct_at_zero_only() {
        let mut e = counted_engine();
        assert_eq!(RELEASE_REFS.len(), 57);
        for &(f, global, offset) in RELEASE_REFS {
            map_word(&mut e, global);
            let object = OBJECT + offset;
            e.set_global::<u32>(object, VTABLE);
            // A null global: nothing at all.
            e.set_global::<u32>(global, 0);
            assert_eq!(run(&mut e, f), vec![], "{f:08x} null");
            // The count 2 becomes 1: decremented, not destroyed.
            e.set_global::<u32>(global, OBJECT);
            e.set_global::<i32>(object + 4, 2);
            assert_eq!(
                run(&mut e, f),
                vec![(INTERLOCKED_DECREMENT, vec![object + 4])],
                "{f:08x} 2"
            );
            // 1 becomes 0: the virtual function of slot +4 runs with the object.
            e.set_global::<i32>(object + 4, 1);
            assert_eq!(
                run(&mut e, f),
                vec![
                    (INTERLOCKED_DECREMENT, vec![object + 4]),
                    (VIRTUAL_TARGET, vec![object])
                ],
                "{f:08x} 1"
            );
            // 0 becomes -1: not zero, so no destructor (the test is JNZ on the result).
            e.set_global::<i32>(object + 4, 0);
            assert_eq!(
                run(&mut e, f),
                vec![(INTERLOCKED_DECREMENT, vec![object + 4])],
                "{f:08x} 0"
            );
        }
    }

    /// The offset variant tests the global, not the pointer plus the offset.
    #[test]
    fn offset_release_adds_the_offset_after_the_null_test() {
        let offsets: Vec<u32> = RELEASE_REFS
            .iter()
            .map(|r| r.2)
            .filter(|o| *o != 0)
            .collect();
        assert_eq!(offsets, vec![0x158]);
        let mut e = counted_engine();
        let &(f, global, offset) = RELEASE_REFS.iter().find(|r| r.2 != 0).unwrap();
        map_word(&mut e, global);
        e.set_global::<u32>(global, OBJECT);
        e.set_global::<u32>(OBJECT + offset, VTABLE);
        e.set_global::<i32>(OBJECT + offset + 4, 1);
        let calls = run(&mut e, f);
        assert_eq!(calls[0], (INTERLOCKED_DECREMENT, vec![OBJECT + 0x158 + 4]));
        assert_eq!(calls[1], (VIRTUAL_TARGET, vec![OBJECT + 0x158]));
    }

    #[test]
    fn string_releases_skip_the_sentinel() {
        let mut e = Engine::new();
        quiet(&mut e, &[INTERLOCKED_DECREMENT]);
        map_word(&mut e, STRING_SENTINEL);
        assert_eq!(RELEASE_STRINGS.len(), 4);
        for &(f, global) in RELEASE_STRINGS {
            map_word(&mut e, global);
            // The sentinel word is 0 in the executable, and so is an unset global: equal, skipped.
            e.set_global::<u32>(STRING_SENTINEL, 0);
            e.set_global::<u32>(global, 0);
            assert_eq!(run(&mut e, f), vec![], "{f:08x} equal");
            // A string: the count is 8 bytes before the pointer.
            e.set_global::<u32>(global, 0x0900_0100);
            assert_eq!(
                run(&mut e, f),
                vec![(INTERLOCKED_DECREMENT, vec![0x0900_00f8])],
                "{f:08x} string"
            );
            // The compare is with the word in memory, whatever it holds.
            e.set_global::<u32>(STRING_SENTINEL, 0x0900_0100);
            assert_eq!(run(&mut e, f), vec![], "{f:08x} sentinel");
            e.set_global::<u32>(global, 0);
            assert_eq!(
                run(&mut e, f),
                vec![(INTERLOCKED_DECREMENT, vec![0xffff_fff8])],
                "{f:08x} null"
            );
            e.set_global::<u32>(STRING_SENTINEL, 0);
        }
    }

    /// The loops walk down from the end of the array, `last + 1` rounds.
    #[test]
    fn string_array_releases_walk_from_the_last_element() {
        let mut e = Engine::new();
        quiet(&mut e, &[INTERLOCKED_DECREMENT]);
        map_word(&mut e, STRING_SENTINEL);
        e.set_global::<u32>(STRING_SENTINEL, 0x0900_0000);
        assert_eq!(RELEASE_STRING_ARRAYS.len(), 2);
        for &(f, end, last) in RELEASE_STRING_ARRAYS {
            let rounds = last + 1;
            e.map(end - 4 * rounds, 4 * rounds + 4);
            // Element k (counted from the end) is the sentinel when k % 3 == 0, else a string.
            let mut expect = vec![];
            for k in 0..rounds {
                let at = end - 4 - 4 * k;
                let value = if k % 3 == 0 {
                    0x0900_0000
                } else {
                    0x0900_1000 + 0x10 * k
                };
                e.set_global::<u32>(at, value);
                if k % 3 != 0 {
                    expect.push((INTERLOCKED_DECREMENT, vec![value - 8]));
                }
            }
            // The word at the end of the array is never read.
            e.set_global::<u32>(end, 0xdead_beef);
            assert_eq!(run(&mut e, f), expect, "{f:08x}");
        }
        let rounds: Vec<u32> = RELEASE_STRING_ARRAYS.iter().map(|r| r.2 + 1).collect();
        assert_eq!(rounds, vec![10, 4]);
    }

    #[test]
    fn object_array_releases_walk_from_the_last_element() {
        let mut e = counted_engine();
        assert_eq!(RELEASE_OBJECT_ARRAYS.len(), 2);
        for &(f, end, last) in RELEASE_OBJECT_ARRAYS {
            let rounds = last + 1;
            e.map(end - 4 * rounds, 4 * rounds + 4);
            let mut expect = vec![];
            for k in 0..rounds {
                let at = end - 4 - 4 * k;
                let object = OBJECT + 0x10 * k;
                let (pointer, count) = match k % 4 {
                    0 => (0, 0),
                    1 => (object, 1),
                    2 => (object, 2),
                    _ => (object, 0),
                };
                e.set_global::<u32>(at, pointer);
                if pointer != 0 {
                    e.set_global::<u32>(object, VTABLE);
                    e.set_global::<i32>(object + 4, count);
                    expect.push((INTERLOCKED_DECREMENT, vec![object + 4]));
                    if count == 1 {
                        expect.push((VIRTUAL_TARGET, vec![object]));
                    }
                }
            }
            assert_eq!(run(&mut e, f), expect, "{f:08x}");
        }
        let rounds: Vec<u32> = RELEASE_OBJECT_ARRAYS.iter().map(|r| r.2 + 1).collect();
        assert_eq!(rounds, vec![10, 256]);
    }

    #[test]
    fn stored_vtable_then_call_with_the_word() {
        let mut e = Engine::new();
        assert_eq!(STORE_THEN_CALL_WITH_WORD.len(), 3);
        for &(f, word, global, vtable, callee) in STORE_THEN_CALL_WITH_WORD {
            quiet(&mut e, &[callee]);
            map_word(&mut e, word);
            map_word(&mut e, global);
            e.set_global::<u32>(word, 0x1111_2222);
            e.set_global::<u32>(global, 0);
            assert_eq!(run(&mut e, f), vec![(callee, vec![0x1111_2222])], "{f:08x}");
            assert_eq!(e.global::<u32>(global), vtable, "{f:08x}");
        }
    }

    #[test]
    fn call_with_the_word() {
        let mut e = Engine::new();
        assert_eq!(CALL_WITH_WORD.len(), 1);
        for &(f, word, callee) in CALL_WITH_WORD {
            quiet(&mut e, &[callee]);
            map_word(&mut e, word);
            e.set_global::<u32>(word, 0x1111_2222);
            assert_eq!(run(&mut e, f), vec![(callee, vec![0x1111_2222])], "{f:08x}");
        }
    }

    /// The array objects: the vtable is stored, and a block is passed to the function in the
    /// vtable's slot (which reads one stack argument and no `ECX`) and the three words cleared.
    #[test]
    fn array_object_destructors_free_the_block_and_clear_the_words() {
        let mut e = Engine::new();
        assert_eq!(SLOT_CLEARS.len(), 2);
        for &(f, block, global, vtable, slot, w2, w3) in SLOT_CLEARS {
            let w1 = block;
            assert_eq!(block, global + 4);
            assert_eq!((w2, w3), (global + 0xc, global + 8));
            e.map(global, 0x10);
            e.map(slot, 4);
            e.set_global::<u32>(slot, VIRTUAL_TARGET);
            quiet(&mut e, &[VIRTUAL_TARGET]);
            for w in [global, w1, w2, w3] {
                e.set_global::<u32>(w, 0xaaaa_aaaa);
            }
            // No block: only the vtable is stored.
            e.set_global::<u32>(block, 0);
            assert_eq!(run(&mut e, f), vec![], "{f:08x} no block");
            assert_eq!(e.global::<u32>(global), vtable);
            assert_eq!(e.global::<u32>(w2), 0xaaaa_aaaa);
            assert_eq!(e.global::<u32>(w3), 0xaaaa_aaaa);
            // A block: freed through the slot, then the words are zero.
            e.set_global::<u32>(block, 0x0900_4000);
            assert_eq!(
                run(&mut e, f),
                vec![(VIRTUAL_TARGET, vec![0x0900_4000])],
                "{f:08x} block"
            );
            assert_eq!(
                [w1, w2, w3].map(|w| e.global::<u32>(w)),
                [0, 0, 0],
                "{f:08x}"
            );
        }
        assert_eq!(SLOT_CLEARS[0].3 + 8, SLOT_CLEARS[0].4);
        assert_eq!(SLOT_CLEARS[1].3 + 8, SLOT_CLEARS[1].4);
    }

    #[test]
    fn scalar_delete_runs_only_for_a_pointer() {
        let mut e = Engine::new();
        assert_eq!(SCALAR_DELETES.len(), 1);
        for &(f, global, vtable, pointer, callee) in SCALAR_DELETES {
            quiet(&mut e, &[callee]);
            e.map(global, 4);
            e.map(pointer, 4);
            e.set_global::<u32>(pointer, 0);
            assert_eq!(run(&mut e, f), vec![], "{f:08x} null");
            assert_eq!(e.global::<u32>(global), vtable);
            e.set_global::<u32>(global, 0);
            e.set_global::<u32>(pointer, 0x0900_5000);
            assert_eq!(
                run(&mut e, f),
                vec![(callee, vec![0x0900_5000, 3])],
                "{f:08x}"
            );
            assert_eq!(e.global::<u32>(global), vtable);
        }
    }

    #[test]
    fn clear_and_free_zeroes_a_halfword_and_the_pointer() {
        let mut e = Engine::new();
        assert_eq!(CLEAR_AND_FREE.len(), 1);
        for &(f, halfword, pointer, callee, cleared) in CLEAR_AND_FREE {
            quiet(&mut e, &[callee]);
            e.map(halfword, 8);
            e.map(pointer, 4);
            e.set_global::<u32>(halfword, 0xffff_ffff);
            e.set_global::<u32>(pointer, 0x1234_0000);
            assert_eq!(pointer, cleared);
            assert_eq!(run(&mut e, f), vec![(callee, vec![0x1234_0000])]);
            // Only the low halfword (`MOV [x],AX` with `AX` = 0) is written.
            assert_eq!(e.global::<u32>(halfword), 0xffff_0000);
            assert_eq!(e.global::<u32>(cleared), 0);
        }
    }

    /// The 72 destructors with a vtable store, the singleton getter, the singleton's virtual
    /// function at slot +8 and `00404950`.
    #[test]
    fn singleton_destructors_store_the_vtable_and_call_in_order() {
        let mut e = Engine::new();
        assert_eq!(SINGLETON_DESTRUCTORS.len(), 72);
        e.put_vtable(VTABLE, &[0, 0, VIRTUAL_TARGET]);
        e.map(OBJECT, 0x10);
        e.set_global::<u32>(OBJECT, VTABLE);
        quiet(&mut e, &[VIRTUAL_TARGET, BASE_DESTRUCTOR_00404950]);
        let mut getters = vec![];
        for &(f, global, vtable, getter, singleton) in SINGLETON_DESTRUCTORS {
            quiet(&mut e, &[getter]);
            getters.push((getter, singleton));
            map_word(&mut e, global);
            map_word(&mut e, singleton);
            e.set_global::<u32>(global, 0);
            e.set_global::<u32>(singleton, OBJECT);
            assert_eq!(
                run(&mut e, f),
                vec![
                    (getter, vec![]),
                    (VIRTUAL_TARGET, vec![OBJECT, global]),
                    (BASE_DESTRUCTOR_00404950, vec![global]),
                ],
                "{f:08x}"
            );
            assert_eq!(e.global::<u32>(global), vtable, "{f:08x}");
        }
        getters.sort_unstable();
        getters.dedup();
        assert_eq!(
            getters,
            vec![(0x0044_f570, 0x011f_96a0), (0x004d_e4a0, 0x011f_35a0)]
        );
    }

    #[test]
    fn destructor_then_free_reads_the_pointer_after_the_destructor() {
        let mut e = Engine::new();
        assert_eq!(DESTRUCT_THEN_FREE.len(), 5);
        for &(f, global, destructor, free) in DESTRUCT_THEN_FREE {
            quiet(&mut e, &[free]);
            map_word(&mut e, global);
            e.set_global::<u32>(global, 0x1234_5678);
            // The destructor changes the global: the free sees the value after it.
            e.register(destructor, |e, a| {
                e.set_global::<u32>(a[0], 0xdead_0001);
                Ret::default()
            });
            assert_eq!(
                run(&mut e, f),
                vec![(destructor, vec![global]), (free, vec![0xdead_0001])],
                "{f:08x}"
            );
        }
    }

    #[test]
    fn two_vtables_are_stored_in_turn() {
        let mut e = Engine::new();
        assert_eq!(TWO_VTABLE_DESTRUCTORS.len(), 2);
        for &(f, global, vtable1, vtable2, callee, word, last) in TWO_VTABLE_DESTRUCTORS {
            let seen = Rc::new(RefCell::new(vec![]));
            let log = seen.clone();
            e.register_double(callee, move |e, a| {
                log.borrow_mut().push(e.global::<u32>(a[0]));
                Ret::default()
            });
            quiet(&mut e, &[last]);
            map_word(&mut e, global);
            map_word(&mut e, word);
            e.set_global::<u32>(word, 0x1111_2222);
            assert_eq!(
                run(&mut e, f),
                vec![
                    (callee, vec![global]),
                    (callee, vec![global]),
                    (last, vec![0x1111_2222])
                ],
                "{f:08x}"
            );
            assert_eq!(*seen.borrow(), vec![vtable1, vtable2], "{f:08x}");
        }
    }

    #[test]
    fn pool_release_passes_the_pointer_to_the_object() {
        let mut e = Engine::new();
        assert_eq!(POOL_RELEASES.len(), 1);
        for &(f, global, first, pointer, object, second) in POOL_RELEASES {
            quiet(&mut e, &[first, second]);
            map_word(&mut e, pointer);
            e.set_global::<u32>(pointer, 0x1234_0000);
            assert_eq!(
                run(&mut e, f),
                vec![(first, vec![global]), (second, vec![object, 0x1234_0000])],
                "{f:08x}"
            );
        }
    }
}
