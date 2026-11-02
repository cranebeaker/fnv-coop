//! `fallout shared/havok/run_00626560` (Xbox PDB source unit), subsystem `fallout shared/havok`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// `__thiscall` initializer the copy constructor runs first (`00626630`, no arguments).
const FUNCTION_INIT: u32 = 0x0062_6630;
/// `__thiscall` copy from the source (`00626600`, one argument).
const FUNCTION_COPY_FROM: u32 = 0x0062_6600;
/// `__thiscall` (`004a4840`, one argument) called on a one-byte local.
const CHECK_HOLDER: u32 = 0x004a_4840;
/// `__thiscall` (`00cb1870`, `hkmap.cpp`, three arguments).
const MAP_STORE: u32 = 0x00cb_1870;

// Translated from 00626560 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00626560 - the compiler's exception frame (FS:[0] handler 00efc1f8, state -1 then 0 around the copy) is not translated.
/// A `std::function<...>` copy constructor (the library's multiple-match
/// name): initializes `this` (`00626630`), then copies `source` into it
/// (`00626600`); returns `this`.
pub fn function_copy_constructor(e: &mut Engine, this: Ptr, source: u32) -> Ptr {
    e.call(FUNCTION_INIT, &args![this]);
    e.call(FUNCTION_COPY_FROM, &args![this, source]);
    this
}

// Translated from 006265c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__thiscall`: runs `004a4840` on a zeroed one-byte local with
/// `this`, then `00cb1870` on `this` with that result and the two arguments.
pub fn fn_006265c0(e: &mut Engine, this: Ptr, first: u32, second: u32) {
    let checked = e.with_stack(1, |e, local| {
        e.call(CHECK_HOLDER, &args![local, this]).u32()
    });
    e.call(MAP_STORE, &args![this, checked, first, second]);
}

// Translated from 00626660 (decompiled, FalloutNV.exe 1.4.0.525)
/// No map name. `__thiscall`: the address of element `index` of the 8-byte
/// elements at the pointer stored in `this`.
pub fn fn_00626660(e: &mut Engine, this: Ptr, index: u32) -> u32 {
    e.mem.u32(this.addr()).wrapping_add(index.wrapping_mul(8))
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00626560, function_copy_constructor(Ptr, u32) -> Ptr),
        entry!(0x006265c0, fn_006265c0(Ptr, u32, u32)),
        entry!(0x00626660, fn_00626660(Ptr, u32) -> u32),
        entry!(0x0062_6600, fn_00626600(Ptr, u32)),
        entry!(0x0062_6630, fn_00626630(Ptr) -> Ptr),
    ]
}

/// `hkMapBase<unsigned long,unsigned long,hkMapOperations<unsigned long>>::reserve` (engine
/// map name of `00cb1fe0`): `thiscall`, `RET 8`, the allocator and the count.
const MAP_RESERVE: u32 = 0x00cb_1fe0;

// Translated from 00626600 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `thiscall`, `RET 4`. Asks `004a4840` (on a zeroed stack byte,
/// with `this`) for the allocator and reserves `count` entries in the map `this` (`00cb1fe0`;
/// `count` is the word the exe pushed before `004a4840`, left on the stack for it).
pub fn fn_00626600(e: &mut Engine, this: Ptr, count: u32) {
    let allocator = e.with_stack(1, |e, byte| {
        e.mem.set_u8(byte.addr(), 0);
        e.call(CHECK_HOLDER, &args![byte, this]).u32()
    });
    e.call(MAP_RESERVE, &args![this, allocator, count]);
}

// Translated from 00626630 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map. `fastcall`: an empty map, 0 at `+0` and `+4` and `0xffffffff`
/// at `+8`. Answers `this`.
pub fn fn_00626630(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0);
    e.mem.set_u32(this.addr().wrapping_add(4), 0);
    e.mem.set_u32(this.addr().wrapping_add(8), 0xffff_ffff);
    this
}

#[cfg(test)]
mod map_tests {
    use super::*;

    #[test]
    fn at_00626600_reserves_through_the_allocator() {
        let mut e = Engine::new();
        e.call_log = Some(vec![]);
        e.register(CHECK_HOLDER, |e, a| {
            assert_eq!(e.mem.u8(a[0]), 0);
            0x7700u32.into_ret()
        });
        e.register(MAP_RESERVE, |_, _| Ret::default());
        e.call(0x0062_6600, &args![0x4000u32, 9u32]);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(MAP_RESERVE, vec![0x4000, 0x7700, 9])));
        assert!(log
            .iter()
            .any(|(a, w)| *a == CHECK_HOLDER && w[1] == 0x4000));
    }

    #[test]
    fn at_00626630_makes_an_empty_map() {
        let mut e = Engine::new();
        let this = e.mem.alloc(0xc);
        e.mem.set_u32(this, 5);
        e.mem.set_u32(this + 4, 5);
        assert_eq!(e.call(0x0062_6630, &args![this]).u32(), this);
        assert_eq!(e.mem.u32(this), 0);
        assert_eq!(e.mem.u32(this + 4), 0);
        assert_eq!(e.mem.u32(this + 8), 0xffff_ffff);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_constructor_initializes_then_copies_and_returns_this() {
        let mut e = Engine::new();
        e.register(FUNCTION_INIT, |_, _| Ret::default());
        e.register(FUNCTION_COPY_FROM, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let result = e.call(0x0062_6560, &args![Ptr::<()>::new(0x4000), 0x5000u32]);
        assert_eq!(result.u32(), 0x4000);
        let log = e.call_log.clone().unwrap();
        let calls: Vec<_> = log.iter().filter(|c| c.0 != 0x0062_6560).collect();
        assert_eq!(calls[0], &(FUNCTION_INIT, vec![0x4000]));
        assert_eq!(calls[1], &(FUNCTION_COPY_FROM, vec![0x4000, 0x5000]));
    }

    #[test]
    fn fn_006265c0_passes_the_checked_result_and_arguments_on() {
        let mut e = Engine::new();
        e.register(CHECK_HOLDER, |e, a| {
            // The local byte is zeroed memory.
            assert_eq!(e.mem.u8(a[0]), 0);
            Ret {
                eax: a[1] + 1,
                ..Ret::default()
            }
        });
        e.register(MAP_STORE, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        e.call(0x0062_65c0, &args![Ptr::<()>::new(0x4000), 7u32, 9u32]);
        let log = e.call_log.clone().unwrap();
        assert!(log.contains(&(MAP_STORE, vec![0x4000, 0x4001, 7, 9])));
    }

    #[test]
    fn fn_00626660_indexes_eight_byte_elements() {
        let mut e = Engine::new();
        let this = e.mem.alloc(4);
        e.mem.set_u32(this, 0x1000);
        assert_eq!(
            e.call(0x0062_6660, &args![Ptr::<()>::new(this), 3u32])
                .u32(),
            0x1018
        );
    }
}
