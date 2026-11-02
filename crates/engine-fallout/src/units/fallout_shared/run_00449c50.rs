//! `fallout shared/run_00449c50` (Xbox PDB source unit), subsystem `fallout shared`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00449d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scalar deleting destructor shape (the engine map gives it no name; the class is unconfirmed): runs the destructor body `00449c50` on `this`, then frees `this` with `00401030` when bit 0 of `flags` is set. Returns `this`.
pub fn fn_00449d10(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    e.call(0x0044_9c50, &args![this]);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

/// `0044c480` (`thiscall`, `RET 4`): run with 1 first.
const CLEAR_ENTRIES: u32 = 0x0044_c480;
/// `0044c1e0` (`thiscall`, no stack word): the base destructor.
const BASE_DESTRUCTOR_0044C1E0: u32 = 0x0044_c1e0;

// Translated from 00449c50 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00449c50 - the C++ exception-unwinding frame (FS:[0] chain, handler at 00eecdf8, state variable) is not translated
/// Unnamed in the engine map (`thiscall`, no stack word): a destructor. Sets the vtable
/// `010172dc`, calls `0044c480(1)` on `this`, then the base destructor `0044c1e0`.
pub fn fn_00449c50(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0101_72dc);
    e.call(CLEAR_ENTRIES, &args![this, 1u32]);
    e.call(BASE_DESTRUCTOR_0044C1E0, &args![this]);
}

/// `0044ddc0` (`thiscall`, no stack word): the word at `this` (here the
/// table's bucket count).
const BUCKET_COUNT: u32 = 0x0044_ddc0;

// Translated from 00449cb0 (decompiled, FalloutNV.exe 1.4.0.525)
/// The bucket of a string key (`thiscall`, `RET 4`; the engine map gives it
/// no name): the hash `h = h * 33 + c` over the key's characters (each a
/// signed byte, in 32-bit wrapping arithmetic) modulo the bucket count
/// (`0044ddc0` on `this`; an unsigned `DIV`, which faults on a count of 0).
pub fn fn_00449cb0(e: &mut Engine, this: Ptr, key: u32) -> u32 {
    let mut hash = 0u32;
    let mut at = key;
    loop {
        let c = e.mem.i8(at);
        if c == 0 {
            break;
        }
        hash = (hash << 5).wrapping_add(hash).wrapping_add(c as i32 as u32);
        at = at.wrapping_add(1);
    }
    let count = e.call(BUCKET_COUNT, &args![this]).u32();
    hash % count
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0044_9cb0, fn_00449cb0(Ptr, u32) -> u32),
        entry!(0x0044_9d10, fn_00449d10(Ptr, u32) -> Ptr),
        entry!(0x0044_9c50, fn_00449c50(Ptr)),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_00449d10_runs_destructor_body_then_deletes_on_flag_one() {
        use crate::prelude::*;

        let mut e = Engine::new();
        let this: Ptr = Ptr::new(e.mem.alloc(16));
        e.register(0x0044_9c50, |_, _| Ret::default());
        e.register(0x0040_1030, |_, _| Ret::default());
        for (flags, deletes) in [(0u32, false), (2, false), (1, true), (0xffff_ffff, true)] {
            e.call_log = Some(vec![]);
            let result = e.call(0x0044_9d10, &args![this, flags]);
            assert_eq!(result.u32(), this.addr());
            let log = e.call_log.take().unwrap();
            let calls: Vec<(u32, Vec<u32>)> = log
                .into_iter()
                .filter(|(address, _)| *address != 0x0044_9d10)
                .collect();
            let mut expected = vec![(0x0044_9c50, vec![this.addr()])];
            if deletes {
                expected.push((0x0040_1030, vec![this.addr()]));
            }
            assert_eq!(calls, expected);
        }
    }
}

#[cfg(test)]
mod tests_b043 {
    use super::*;

    #[test]
    fn fn_00449c50_clears_then_runs_the_base() {
        let mut e = Engine::new();
        e.register(CLEAR_ENTRIES, |_, _| Ret::default());
        e.register(BASE_DESTRUCTOR_0044C1E0, |_, _| Ret::default());
        let this = e.mem.alloc(0x10);
        e.call_log = Some(vec![]);
        e.call(0x0044_9c50, &args![this]);
        let log = e.call_log.take().unwrap();
        assert_eq!(
            log[1..],
            [
                (CLEAR_ENTRIES, vec![this, 1]),
                (BASE_DESTRUCTOR_0044C1E0, vec![this])
            ]
        );
        assert_eq!(e.mem.u32(this), 0x0101_72dc);
    }
}

#[cfg(test)]
mod n038_tests {
    use super::*;

    #[test]
    fn at_00449cb0_hashes_signed_characters_modulo_the_count() {
        let mut e = Engine::new();
        e.register(BUCKET_COUNT, |e, a| Ret {
            eax: e.mem.u32(a[0]),
            ..Ret::default()
        });
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 1000);
        let key = e.mem.alloc(8);
        e.mem.set_cstr(key, b"ab");
        // ((0 * 33 + 97) * 33 + 98) % 1000 = 3299 % 1000.
        assert_eq!(e.call(0x0044_9cb0, &args![this, key]).u32(), 299);
        // A byte above 0x7f counts as negative: 0 * 33 - 1 wraps.
        e.mem.set_cstr(key, &[0xff]);
        e.mem.set_u32(this, 7);
        assert_eq!(
            e.call(0x0044_9cb0, &args![this, key]).u32(),
            0xffff_ffff % 7
        );
        // The empty key is bucket 0.
        e.mem.set_cstr(key, b"");
        assert_eq!(e.call(0x0044_9cb0, &args![this, key]).u32(), 0);
    }
}
