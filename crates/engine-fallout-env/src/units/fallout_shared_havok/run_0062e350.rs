//! `fallout shared/havok/run_0062e350` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 0062e3a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x1c (`SUB ECX,0x1c`) and jumps to `0062d870`. `0062d870` (`RET 4`, which the Xbox PDB names `TESTrapListener::_scalar_deleting_destructor_`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0062e3a0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0062_d870, &args![this.addr().wrapping_sub(0x1c), flags])
        .u32()
}

// Translated from 0062e3b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x8 (`SUB ECX,0x8`) and jumps to `0062d870`. `0062d870` (`RET 4`, which the Xbox PDB names `TESTrapListener::_scalar_deleting_destructor_`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0062e3b0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0062_d870, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 0062e3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by 0x14 (`SUB ECX,0x14`) and jumps to `0062d6c0`. `0062d6c0` (`RET 4`) has the shape of a deleting destructor: it tests bit 0 of the flags word and returns the object pointer in `EAX`. The return type is `u32` from that `EAX`; the parameter count is from the `RET 4`. The stack word is passed on unchanged.
pub fn fn_0062e3c0(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x0062_d6c0, &args![this.addr().wrapping_sub(0x14), flags])
        .u32()
}

/// `00626870(this)`: clears the array (calls `00626790` on its data and
/// size, then stores 0 as the size at `+4`).
const ARRAY_CLEAR: u32 = 0x0062_6870;
/// `0062a100(this)`: the array's capacity, `word at +8 & 0x3fffffff`.
const ARRAY_CAPACITY: u32 = 0x0062_a100;
/// `0066db70(allocator, data, capacity)` (`RET 8`): calls the allocator's
/// virtual function at `+0x10` with `data` and `capacity << 4` (the
/// elements are 16 bytes).
const ARRAY_DEALLOCATE: u32 = 0x0066_db70;
/// The top bit of the capacity word at `+8`: the storage is not owned by
/// the array and is not freed.
const DONT_DEALLOCATE: u32 = 0x8000_0000;

// Translated from 0062e350 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears an array of 16-byte elements and gives its storage back to
/// `allocator` (the layout is `hkArray`'s: data pointer at `+0`, size at
/// `+4`, capacity and flags at `+8`; the exe does not name this function):
/// clears it (`00626870`), and when the storage is owned (bit 31 of `+8`
/// clear) hands the data and its capacity to `0066db70` with `allocator`.
/// Then sets the data pointer to 0 and `+8` to `0x80000000`.
pub fn fn_0062e350(e: &mut Engine, this: Ptr, allocator: u32) {
    e.call(ARRAY_CLEAR, &args![this]);
    if e.mem.u32(this.addr() + 8) & DONT_DEALLOCATE == 0 {
        let capacity = e.call(ARRAY_CAPACITY, &args![this]).u32();
        let data = e.mem.u32(this.addr());
        e.call(ARRAY_DEALLOCATE, &args![allocator, data, capacity]);
    }
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr() + 8, DONT_DEALLOCATE);
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0062_e350, fn_0062e350(Ptr, u32)),
        entry!(0x0062_e3a0, fn_0062e3a0(Ptr, u32) -> u32),
        entry!(0x0062_e3b0, fn_0062e3b0(Ptr, u32) -> u32),
        entry!(0x0062_e3c0, fn_0062e3c0(Ptr, u32) -> u32),
    ]
}

#[cfg(test)]
mod array_release_tests {
    use super::*;

    fn engine() -> Engine {
        let mut e = Engine::new();
        e.register(ARRAY_CLEAR, |_, _| Ret::default());
        e.register(ARRAY_CAPACITY, |e, a| {
            (e.mem.u32(a[0] + 8) & 0x3fff_ffff).into_ret()
        });
        e.register(ARRAY_DEALLOCATE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e
    }

    fn logged(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    #[test]
    fn owned_storage_goes_back_to_the_allocator() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x8000);
        e.mem.set_u32(array + 4, 3);
        // Capacity 6; bit 30 (a flag the capacity mask drops) is set.
        e.mem.set_u32(array + 8, 0x4000_0006);
        e.call(0x0062_e350, &args![array, 0x5000u32]);
        assert_eq!(logged(&e, ARRAY_CLEAR), vec![vec![array]]);
        assert_eq!(logged(&e, ARRAY_CAPACITY), vec![vec![array]]);
        assert_eq!(logged(&e, ARRAY_DEALLOCATE), vec![vec![0x5000, 0x8000, 6]]);
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
    }

    #[test]
    fn storage_the_array_does_not_own_is_not_freed() {
        let mut e = engine();
        let array = e.mem.alloc(0x10);
        e.mem.set_u32(array, 0x8000);
        e.mem.set_u32(array + 8, 0x8000_0008);
        e.call(0x0062_e350, &args![array, 0x5000u32]);
        assert_eq!(logged(&e, ARRAY_CLEAR).len(), 1);
        assert!(logged(&e, ARRAY_CAPACITY).is_empty());
        assert!(logged(&e, ARRAY_DEALLOCATE).is_empty());
        assert_eq!(e.mem.u32(array), 0);
        assert_eq!(e.mem.u32(array + 8), 0x8000_0000);
    }
}

#[cfg(test)]
mod le81_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and `extra` stack words, with a
    /// double over `target` that returns `0xdeadbeef`; checks that the
    /// double ran once, with `this` moved back by `back` and `extra` passed on.
    fn check(thunk: u32, target: u32, back: u32, extra: &[u32]) -> Ret {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let mut words = vec![0x2000u32];
        words.extend_from_slice(extra);
        let result = e.call(thunk, &words);
        let log = e.call_log.take().unwrap();
        let mut expected = vec![0x2000 - back];
        expected.extend_from_slice(extra);
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, expected);
        result
    }

    #[test]
    fn le81_thunk_0062e3a0() {
        assert_eq!(
            check(0x0062_e3a0, 0x0062_d870, 0x1c, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0062e3b0() {
        assert_eq!(
            check(0x0062_e3b0, 0x0062_d870, 0x8, &[0x1]).u32(),
            0xdead_beef
        );
    }

    #[test]
    fn le81_thunk_0062e3c0() {
        assert_eq!(
            check(0x0062_e3c0, 0x0062_d6c0, 0x14, &[0x1]).u32(),
            0xdead_beef
        );
    }
}
