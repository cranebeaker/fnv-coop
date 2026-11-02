//! `(unplaced)/atexit_irregular_c` (no source unit: the compiler's `atexit` destructors),
//! subsystem `compiler: static init/exit`: ten `atexit` destructor functions of FalloutNV.exe
//! 1.4.0.525 that neither the generated table `data/atexit_destructors.tsv` nor parts 1 and 2
//! (`atexit_irregular.rs`, `atexit_irregular_b.rs`) have, each translated from its own
//! disassembly (`POST /disassemble_bytes`; Ghidra made no function of them). Part 3 of the list.
//!
//! The exit sequence (`units::crt::doexit`) runs them last-registered first. They are cdecl
//! functions with no parameters; what they leave in `EAX` nobody reads, so they return
//! nothing. The shapes:
//!
//! - the release of a reference-counted object held by a global (`InterlockedDecrement` of the
//!   count at +4, the virtual destructor in vtable slot +4 reached by `JMP EDX` at zero):
//!   `00fdad30`, `00fdada0`, `00fdae10`, `00fdb110`, `00fdb140`, `00fdb230`;
//! - the release of a string held by a global (the count 8 bytes before the pointer, skipped
//!   for the word at `0109b220`): `00fde1b0`, `00fde1d0`;
//! - a vtable stored in a global, then a tail jump to `00cbf1a0` with the global in `ECX`:
//!   `00fddd40`;
//! - a vtable stored in a global: `00fde2c0`.
//!
//! None has an exception frame or a `/GS` cookie check. Callees are called by address and never
//! re-implemented; imports through their import slot.

#[allow(unused_imports)]
use crate::prelude::*;

/// Import slot of KERNEL32 `InterlockedDecrement` (stdcall, one argument).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The word the string releases compare a string pointer with (`CMP EAX,dword ptr [0109b220]`).
const STRING_SENTINEL: u32 = 0x0109_b220;
/// `00cbf1a0` (thiscall, plain `RET`: stores vtable `010c3b30` in `this`).
const FN_00CBF1A0: u32 = 0x00cb_f1a0;

/// Releases the reference-counted object whose pointer is in `global`: nothing when the global
/// is null (`TEST EAX,EAX; JZ`); `InterlockedDecrement` of the count at +4 of the object; when
/// that gives 0 (`TEST EAX,EAX; JNZ`), the virtual function in vtable slot +4 (the destructor)
/// with the object as `this` (`MOV EAX,[ESI]; MOV EDX,[EAX+4]; MOV ECX,ESI; JMP EDX`).
fn release_ref(e: &mut Engine, global: u32) {
    let object = e.global::<u32>(global);
    if object == 0 {
        return;
    }
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
    if pointer != e.global::<u32>(STRING_SENTINEL) {
        e.call(INTERLOCKED_DECREMENT, &args![pointer.wrapping_sub(8)]);
    }
}

// Translated from 00fdad30 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the reference-counted object held by `011f36a4`: [`release_ref`].
pub fn fn_00fdad30(e: &mut Engine) {
    release_ref(e, 0x011f_36a4);
}

// Translated from 00fdada0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the reference-counted object held by `011f36d4`: [`release_ref`].
pub fn fn_00fdada0(e: &mut Engine) {
    release_ref(e, 0x011f_36d4);
}

// Translated from 00fdae10 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the reference-counted object held by `011f3f40`: [`release_ref`].
pub fn fn_00fdae10(e: &mut Engine) {
    release_ref(e, 0x011f_3f40);
}

// Translated from 00fdb110 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the reference-counted object held by `011f5e7c`: [`release_ref`].
pub fn fn_00fdb110(e: &mut Engine) {
    release_ref(e, 0x011f_5e7c);
}

// Translated from 00fdb140 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the reference-counted object held by `011f5e74`: [`release_ref`].
pub fn fn_00fdb140(e: &mut Engine) {
    release_ref(e, 0x011f_5e74);
}

// Translated from 00fdb230 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the reference-counted object held by `011f6004`: [`release_ref`].
pub fn fn_00fdb230(e: &mut Engine) {
    release_ref(e, 0x011f_6004);
}

// Translated from 00fddd40 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01267b54`: its vtable `010c3af8` stored, then
/// `MOV ECX,01267b54; JMP 00cbf1a0`.
pub fn fn_00fddd40(e: &mut Engine) {
    e.set_global::<u32>(0x0126_7b54, 0x010c_3af8);
    e.call(FN_00CBF1A0, &args![0x0126_7b54_u32]);
}

// Translated from 00fde1b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the string held by `0126826c`: [`release_string`].
pub fn fn_00fde1b0(e: &mut Engine) {
    release_string(e, 0x0126_826c);
}

// Translated from 00fde1d0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the string held by `01268290`: [`release_string`].
pub fn fn_00fde1d0(e: &mut Engine) {
    release_string(e, 0x0126_8290);
}

// Translated from 00fde2c0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `0126a420`: only its vtable `010d9fc4` stored.
pub fn fn_00fde2c0(e: &mut Engine) {
    e.set_global::<u32>(0x0126_a420, 0x010d_9fc4);
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00fd_ad30, fn_00fdad30()),
        entry!(0x00fd_ada0, fn_00fdada0()),
        entry!(0x00fd_ae10, fn_00fdae10()),
        entry!(0x00fd_b110, fn_00fdb110()),
        entry!(0x00fd_b140, fn_00fdb140()),
        entry!(0x00fd_b230, fn_00fdb230()),
        entry!(0x00fd_dd40, fn_00fddd40()),
        entry!(0x00fd_e1b0, fn_00fde1b0()),
        entry!(0x00fd_e1d0, fn_00fde1d0()),
        entry!(0x00fd_e2c0, fn_00fde2c0()),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    type Calls = Vec<(u32, Vec<u32>)>;

    /// A stand-in object address, its vtable and a virtual destructor.
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

    /// `InterlockedDecrement` as the platform does it: decrement in memory, return the new value.
    fn interlocked_decrement_double(e: &mut Engine, a: &[u32]) -> Ret {
        let v = e.mem.i32(a[0]).wrapping_sub(1);
        e.mem.set_i32(a[0], v);
        Ret {
            eax: v as u32,
            ..Ret::default()
        }
    }

    #[test]
    fn reference_releases_destroy_at_zero() {
        for (entry, global) in [
            (0x00fd_ad30_u32, 0x011f_36a4_u32),
            (0x00fd_ada0, 0x011f_36d4),
            (0x00fd_ae10, 0x011f_3f40),
            (0x00fd_b110, 0x011f_5e7c),
            (0x00fd_b140, 0x011f_5e74),
            (0x00fd_b230, 0x011f_6004),
        ] {
            let mut e = Engine::new();
            e.register(INTERLOCKED_DECREMENT, interlocked_decrement_double);
            quiet(&mut e, &[VIRTUAL_TARGET]);
            e.put_vtable(VTABLE, &[0, VIRTUAL_TARGET, VIRTUAL_TARGET + 4]);
            e.map(OBJECT, 0x1000);
            e.map(global, 4);
            assert!(run(&mut e, entry).is_empty(), "null: {entry:08x}");
            e.set_global::<u32>(global, OBJECT);
            e.set_global::<u32>(OBJECT, VTABLE);
            e.set_global::<u32>(OBJECT + 4, 2);
            assert_eq!(
                run(&mut e, entry),
                vec![(INTERLOCKED_DECREMENT, vec![OBJECT + 4])],
                "{entry:08x}"
            );
            assert_eq!(
                run(&mut e, entry),
                vec![
                    (INTERLOCKED_DECREMENT, vec![OBJECT + 4]),
                    (VIRTUAL_TARGET, vec![OBJECT]),
                ],
                "{entry:08x}"
            );
        }
    }

    #[test]
    fn string_releases_skip_the_sentinel() {
        for (entry, global) in [
            (0x00fd_e1b0_u32, 0x0126_826c_u32),
            (0x00fd_e1d0, 0x0126_8290),
        ] {
            let mut e = Engine::new();
            e.map(global, 4);
            e.map(STRING_SENTINEL, 4);
            e.set_global::<u32>(STRING_SENTINEL, 0x0700_0000);
            quiet(&mut e, &[INTERLOCKED_DECREMENT]);
            e.set_global::<u32>(global, 0x0700_0000);
            assert!(run(&mut e, entry).is_empty(), "{entry:08x}");
            e.set_global::<u32>(global, 0x0700_0010);
            assert_eq!(
                run(&mut e, entry),
                vec![(INTERLOCKED_DECREMENT, vec![0x0700_0008])],
                "{entry:08x}"
            );
        }
    }

    #[test]
    fn fn_00fddd40_stores_the_vtable_then_runs_00cbf1a0() {
        let mut e = Engine::new();
        e.map(0x0126_7b54, 4);
        quiet(&mut e, &[FN_00CBF1A0]);
        assert_eq!(
            run(&mut e, 0x00fd_dd40),
            vec![(FN_00CBF1A0, vec![0x0126_7b54])]
        );
        assert_eq!(e.global::<u32>(0x0126_7b54), 0x010c_3af8);
    }

    #[test]
    fn fn_00fde2c0_stores_the_vtable() {
        let mut e = Engine::new();
        e.map(0x0126_a420, 4);
        assert!(run(&mut e, 0x00fd_e2c0).is_empty());
        assert_eq!(e.global::<u32>(0x0126_a420), 0x010d_9fc4);
    }

    #[test]
    fn every_function_is_registered_once() {
        let funcs = funcs();
        let addrs: Vec<u32> = funcs.iter().map(|(a, _)| *a).collect();
        assert_eq!(addrs.len(), 10);
        assert!(
            addrs.windows(2).all(|w| w[0] < w[1]),
            "ascending and unique"
        );
        let e = Engine::new();
        for a in addrs {
            assert!(e.is_translated(a), "{a:08x}");
        }
    }
}
