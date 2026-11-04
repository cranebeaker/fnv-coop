//! `(unplaced)/atexit_irregular_b` (no source unit: the compiler's `atexit` destructors),
//! subsystem `compiler: static init/exit`: thirteen `atexit` destructor functions of
//! FalloutNV.exe 1.4.0.525 (the engine map's "compiler: atexit destructors") that the recorded
//! game reached and neither the generated table `data/atexit_destructors.tsv` nor part 1
//! (`atexit_irregular.rs`) has, each translated from its own disassembly. Part 2 of the list.
//!
//! The exit sequence (`units::crt::doexit`) runs them last-registered first. They are cdecl
//! functions with no parameters; what they leave in `EAX` nobody reads, so they return
//! nothing. The shapes:
//!
//! - the release of a string held by a global (the count 8 bytes before the pointer, skipped
//!   for the word at `0109b220`): `00fdad60`, `00fdad80`, `00fde170`;
//! - a destructor reached by a tail jump with the global in `ECX` (`MOV ECX,global; JMP
//!   destructor`): `00fdcc30`, `00fdcc40`, `00fdcc80`, `00fdcc90`;
//! - the release of a reference-counted object held by a global (`InterlockedDecrement` of the
//!   count at +4, the virtual destructor in vtable slot +4 reached by `JMP EDX` at zero):
//!   `00fdce10`, `00fdd3b0`, `00fdd3e0`, `00fde390`;
//! - a vtable stored in a global: `00fde2a0`;
//! - a plain count at +0x60 counted down, with `00e7ecd0` at zero: `00fde350`.
//!
//! None has an exception frame or a `/GS` cookie check. Callees are called by address and never
//! re-implemented; imports through their import slot.

#[allow(unused_imports)]
use crate::prelude::*;

/// Import slot of KERNEL32 `InterlockedDecrement` (stdcall, one argument).
const INTERLOCKED_DECREMENT: u32 = 0x00fd_f1c4;
/// The word the string releases compare a string pointer with (`CMP EAX,dword ptr [0109b220]`).
/// The executable holds 0 there; the translation reads it as the game does.
const STRING_SENTINEL: u32 = 0x0109_b220;

/// Releases the reference-counted object whose pointer is in `global`: nothing when the global
/// is null; `InterlockedDecrement` of the count at +4 of the object; when that gives 0 (`TEST
/// EAX,EAX; JNZ`), the virtual function in vtable slot +4 (the destructor) with the object as
/// `this` (`MOV EAX,[ESI]; MOV EDX,[EAX+4]; MOV ECX,ESI; JMP EDX`).
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

/// The destructors the four tail-jump functions below run (`thiscall`, no stack arguments) and
/// the function `00fde350` jumps to (`thiscall`, no stack arguments).
const DESTRUCTOR_00B580A0: u32 = 0x00b5_80a0;
const DESTRUCTOR_00B7FBE0: u32 = 0x00b7_fbe0;
const DESTRUCTOR_00B60420: u32 = 0x00b6_0420;
const FN_00E7ECD0: u32 = 0x00e7_ecd0;

// Translated from 00fdad60 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the string held by `011f36c4`: [`release_string`].
pub fn fn_00fdad60(e: &mut Engine) {
    release_string(e, 0x011f_36c4);
}

// Translated from 00fdad80 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the string held by `011f36cc`: [`release_string`].
pub fn fn_00fdad80(e: &mut Engine) {
    release_string(e, 0x011f_36cc);
}

// Translated from 00fdcc30 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f9670`: `MOV ECX,011f9670; JMP 00b580a0`.
pub fn fn_00fdcc30(e: &mut Engine) {
    e.call(DESTRUCTOR_00B580A0, &args![0x011f_9670_u32]);
}

// Translated from 00fdcc40 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f965c`: `MOV ECX,011f965c; JMP 00b7fbe0`.
pub fn fn_00fdcc40(e: &mut Engine) {
    e.call(DESTRUCTOR_00B7FBE0, &args![0x011f_965c_u32]);
}

// Translated from 00fdcc80 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f9f08`: `MOV ECX,011f9f08; JMP 00b7fbe0`.
pub fn fn_00fdcc80(e: &mut Engine) {
    e.call(DESTRUCTOR_00B7FBE0, &args![0x011f_9f08_u32]);
}

// Translated from 00fdcc90 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `011f9f1c`: `MOV ECX,011f9f1c; JMP 00b60420`.
pub fn fn_00fdcc90(e: &mut Engine) {
    e.call(DESTRUCTOR_00B60420, &args![0x011f_9f1c_u32]);
}

// Translated from 00fdce10 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the reference-counted object held by `011fffe0`: [`release_ref`]
/// (the virtual destructor reached by `JMP EDX`).
pub fn fn_00fdce10(e: &mut Engine) {
    release_ref(e, 0x011f_ffe0);
}

// Translated from 00fdd3b0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the reference-counted object held by `012024f8`: [`release_ref`]
/// (the virtual destructor reached by `JMP EDX`).
pub fn fn_00fdd3b0(e: &mut Engine) {
    release_ref(e, 0x0120_24f8);
}

// Translated from 00fdd3e0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the reference-counted object held by `012024f0`: [`release_ref`]
/// (the virtual destructor reached by `JMP EDX`).
pub fn fn_00fdd3e0(e: &mut Engine) {
    release_ref(e, 0x0120_24f0);
}

// Translated from 00fde170 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the string held by `01268174`: [`release_string`].
pub fn fn_00fde170(e: &mut Engine) {
    release_string(e, 0x0126_8174);
}

// Translated from 00fde2a0 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the global at `01269b78`: only its vtable `0102e388` stored.
pub fn fn_00fde2a0(e: &mut Engine) {
    e.set_global::<u32>(0x0126_9b78, 0x0102_e388);
}

// Translated from 00fde350 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the object held by `0126f730`: nothing when the pointer is null;
/// otherwise its count at +0x60 is decremented (`DEC`, not interlocked) and, when it reaches 0,
/// `00e7ecd0` runs with the object as `this` (`JMP`).
pub fn fn_00fde350(e: &mut Engine) {
    let object = e.global::<u32>(0x0126_f730);
    if object == 0 {
        return;
    }
    let count = object.wrapping_add(0x60);
    let left = e.mem.u32(count).wrapping_sub(1);
    e.mem.set_u32(count, left);
    if e.mem.u32(count) == 0 {
        e.call(FN_00E7ECD0, &args![object]);
    }
}

// Translated from 00fde390 (disassembled, FalloutNV.exe 1.4.0.525)
/// The `atexit` destructor of the reference-counted object held by `0126f910`: [`release_ref`]
/// (the virtual destructor reached by `JMP EDX`).
pub fn fn_00fde390(e: &mut Engine) {
    release_ref(e, 0x0126_f910);
}

pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00fd_ad60, fn_00fdad60()),
        entry!(0x00fd_ad80, fn_00fdad80()),
        entry!(0x00fd_cc30, fn_00fdcc30()),
        entry!(0x00fd_cc40, fn_00fdcc40()),
        entry!(0x00fd_cc80, fn_00fdcc80()),
        entry!(0x00fd_cc90, fn_00fdcc90()),
        entry!(0x00fd_ce10, fn_00fdce10()),
        entry!(0x00fd_d3b0, fn_00fdd3b0()),
        entry!(0x00fd_d3e0, fn_00fdd3e0()),
        entry!(0x00fd_e170, fn_00fde170()),
        entry!(0x00fd_e2a0, fn_00fde2a0()),
        entry!(0x00fd_e350, fn_00fde350()),
        entry!(0x00fd_e390, fn_00fde390()),
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

    #[test]
    fn later_string_releases_skip_the_sentinel() {
        for (entry, global) in [
            (0x00fd_ad60_u32, 0x011f_36c4_u32),
            (0x00fd_ad80, 0x011f_36cc),
            (0x00fd_e170, 0x0126_8174),
        ] {
            let mut e = Engine::new();
            map_word(&mut e, global);
            map_word(&mut e, STRING_SENTINEL);
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
    fn later_tail_destructors_run_on_their_globals() {
        for (entry, destructor, global) in [
            (0x00fd_cc30_u32, DESTRUCTOR_00B580A0, 0x011f_9670_u32),
            (0x00fd_cc40, DESTRUCTOR_00B7FBE0, 0x011f_965c),
            (0x00fd_cc80, DESTRUCTOR_00B7FBE0, 0x011f_9f08),
            (0x00fd_cc90, DESTRUCTOR_00B60420, 0x011f_9f1c),
        ] {
            let mut e = Engine::new();
            quiet(&mut e, &[destructor]);
            assert_eq!(run(&mut e, entry), vec![(destructor, vec![global])]);
        }
    }

    #[test]
    fn later_reference_releases_destroy_at_zero() {
        for (entry, global) in [
            (0x00fd_ce10_u32, 0x011f_ffe0_u32),
            (0x00fd_d3b0, 0x0120_24f8),
            (0x00fd_d3e0, 0x0120_24f0),
            (0x00fd_e390, 0x0126_f910),
        ] {
            let mut e = counted_engine();
            map_word(&mut e, global);
            assert!(run(&mut e, entry).is_empty(), "null: {entry:08x}");
            e.set_global::<u32>(global, OBJECT);
            e.set_global::<u32>(OBJECT, VTABLE);
            e.set_global::<u32>(OBJECT + 4, 2);
            assert_eq!(
                run(&mut e, entry),
                vec![(INTERLOCKED_DECREMENT, vec![OBJECT + 4])]
            );
            assert_eq!(
                run(&mut e, entry),
                vec![
                    (INTERLOCKED_DECREMENT, vec![OBJECT + 4]),
                    (VIRTUAL_TARGET, vec![OBJECT]),
                ]
            );
        }
    }

    #[test]
    fn fn_00fde2a0_stores_the_vtable() {
        let mut e = Engine::new();
        map_word(&mut e, 0x0126_9b78);
        assert!(run(&mut e, 0x00fd_e2a0).is_empty());
        assert_eq!(e.global::<u32>(0x0126_9b78), 0x0102_e388);
    }

    #[test]
    fn fn_00fde350_counts_down_and_runs_00e7ecd0_at_zero() {
        let mut e = Engine::new();
        map_word(&mut e, 0x0126_f730);
        quiet(&mut e, &[FN_00E7ECD0]);
        assert!(run(&mut e, 0x00fd_e350).is_empty());
        e.map(OBJECT, 0x100);
        e.set_global::<u32>(0x0126_f730, OBJECT);
        e.set_global::<u32>(OBJECT + 0x60, 2);
        assert!(run(&mut e, 0x00fd_e350).is_empty());
        assert_eq!(e.global::<u32>(OBJECT + 0x60), 1);
        assert_eq!(run(&mut e, 0x00fd_e350), vec![(FN_00E7ECD0, vec![OBJECT])]);
        assert_eq!(e.global::<u32>(OBJECT + 0x60), 0);
        // From 0 the count wraps and nothing more runs.
        assert!(run(&mut e, 0x00fd_e350).is_empty());
        assert_eq!(e.global::<u32>(OBJECT + 0x60), 0xffff_ffff);
    }

    #[test]
    fn every_function_is_registered_once() {
        let funcs = funcs();
        let addrs: Vec<u32> = funcs.iter().map(|(a, _)| *a).collect();
        assert_eq!(addrs.len(), 13);
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
