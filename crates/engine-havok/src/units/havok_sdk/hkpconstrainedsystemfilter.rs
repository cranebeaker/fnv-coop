//! `Havok SDK/hkpconstrainedsystemfilter.cpp` (Xbox PDB source unit), subsystem `Havok SDK`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

// Translated from 00dffb30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x8` (`SUB ECX,0x8`) and jumps to `00dffb00`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dffb30(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_fb00, &args![this.addr().wrapping_sub(0x8), flags])
        .u32()
}

// Translated from 00dffb50 (decompiled, FalloutNV.exe 1.4.0.525)
/// Thunk: moves `this` back by `0x10` (`SUB ECX,0x10`) and jumps to `00dffb00`, which (`RET 4`) has the shape of a scalar deleting destructor: it reads bit 0 of the one stack word. The caller gets its `EAX`; the class is unconfirmed.
pub fn fn_00dffb50(e: &mut Engine, this: Ptr, flags: u32) -> u32 {
    e.call(0x00df_fb00, &args![this.addr().wrapping_sub(0x10), flags])
        .u32()
}

// Translated from 00e46cf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstrainedSystemFilter::isCollisionEnabled` overload `ov3` of the engine map (Xbox
/// PDB class; which overload it is stays unconfirmed): `RET 0x18`, the result byte pointer
/// `out` and five words. When the filter at `[this + 0x28]` is set, asks it (slot `4` of the
/// object at filter `+ 0xC`, with a result buffer and the five words) and stores 0 to `*out`
/// when the byte its answer points to is 0; otherwise stores 1. Returns `out` (`EAX`).
pub fn hkp_constrained_system_filter_is_collision_enabled_ov3(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
) -> Ptr {
    let filter = e.mem.u32(this.addr() + 0x28);
    if filter != 0 {
        let enabled = e.with_stack(4, |e, result| {
            let answer = e
                .vcall(
                    filter + 0xc,
                    4,
                    &args![result, word_1, word_2, word_3, word_4, word_5],
                )
                .u32();
            e.mem.u8(answer)
        });
        if enabled == 0 {
            e.mem.set_u8(out.addr(), 0);
            return out;
        }
    }
    e.mem.set_u8(out.addr(), 1);
    out
}

// Translated from 00e46d40 (decompiled, FalloutNV.exe 1.4.0.525)
/// `hkpConstrainedSystemFilter::isCollisionEnabled` overload `ov4` of the engine map (Xbox
/// PDB class; which overload it is stays unconfirmed): `RET 0x20`, the result byte pointer
/// `out` and seven words. When the filter at `[this + 0x28]` is set, asks it (slot `0` of
/// the object at filter `+ 0xC`, with a result buffer and the seven words) and stores 0 to
/// `*out` when the byte its answer points to is 0; otherwise stores 1. Returns `out`
/// (`EAX`).
pub fn hkp_constrained_system_filter_is_collision_enabled_ov4(
    e: &mut Engine,
    this: Ptr,
    out: Ptr,
    word_1: u32,
    word_2: u32,
    word_3: u32,
    word_4: u32,
    word_5: u32,
    word_6: u32,
    word_7: u32,
) -> Ptr {
    let filter = e.mem.u32(this.addr() + 0x28);
    if filter != 0 {
        let enabled = e.with_stack(4, |e, result| {
            let answer = e
                .vcall(
                    filter + 0xc,
                    0,
                    &args![result, word_1, word_2, word_3, word_4, word_5, word_6, word_7],
                )
                .u32();
            e.mem.u8(answer)
        });
        if enabled == 0 {
            e.mem.set_u8(out.addr(), 0);
            return out;
        }
    }
    e.mem.set_u8(out.addr(), 1);
    out
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x00df_fb30, fn_00dffb30(Ptr, u32) -> u32),
        entry!(0x00df_fb50, fn_00dffb50(Ptr, u32) -> u32),
        entry!(
            0x00e4_6cf0,
            hkp_constrained_system_filter_is_collision_enabled_ov3(Ptr, Ptr, u32, u32, u32, u32, u32) -> Ptr
        ),
        entry!(
            0x00e4_6d40,
            hkp_constrained_system_filter_is_collision_enabled_ov4(
                Ptr, Ptr, u32, u32, u32, u32, u32, u32, u32
            ) -> Ptr
        ),
    ]
}

#[cfg(test)]
mod a07_thunk_tests {
    use crate::prelude::*;

    /// Calls `thunk` with `this = 0x2000` and one flags word, with a double
    /// over `target` that returns `0xdeadbeef`; checks that the double ran
    /// once, with `this` moved back by `back` and the flags passed on, and
    /// returns the thunk's `EAX`.
    fn check(thunk: u32, target: u32, back: u32) -> u32 {
        let mut e = Engine::new();
        e.register(target, |_, _| Ret {
            eax: 0xdead_beef,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let result = e.call(thunk, &[0x2000u32, 0x1]);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log.iter().filter(|(a, _)| *a == target).collect();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].1, vec![0x2000 - back, 0x1]);
        result.u32()
    }

    #[test]
    fn thunk_00dffb30() {
        assert_eq!(check(0x00df_fb30, 0x00df_fb00, 0x8), 0xdead_beef);
    }

    #[test]
    fn thunk_00dffb50() {
        assert_eq!(check(0x00df_fb50, 0x00df_fb00, 0x10), 0xdead_beef);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::units::havok_sdk::hkpbvagent::test_support::{
        calls, engine, log_double, new_log, Log,
    };

    const SLOT_0: u32 = 0x0900_0000;
    const SLOT_4: u32 = 0x0900_0004;

    /// A filter whose object at `+0xC` has slots 0 and 4 that log their calls and answer a
    /// pointer to a byte holding `answer`. Returns the filter and the log.
    fn filter(e: &mut Engine, answer: u8) -> (u32, Log) {
        let log = new_log();
        let byte = e.mem.alloc(4);
        e.mem.set_u8(byte, answer);
        log_double(e, &log, SLOT_0, byte);
        log_double(e, &log, SLOT_4, byte);
        let vtable = e.mem.alloc(8);
        e.mem.set_u32(vtable, SLOT_0);
        e.mem.set_u32(vtable + 4, SLOT_4);
        let filter = e.mem.alloc(0x20);
        e.mem.set_u32(filter + 0xc, vtable);
        (filter, log)
    }

    /// A filter owner with the filter at `+0x28`.
    fn owner(e: &mut Engine, filter: u32) -> u32 {
        let this = e.mem.alloc(0x40);
        e.mem.set_u32(this + 0x28, filter);
        this
    }

    #[test]
    fn ov3_asks_the_filter_slot_4_00e46cf0() {
        for answer in [0u8, 1, 7] {
            let mut e = engine();
            let (filter, log) = filter(&mut e, answer);
            let this = owner(&mut e, filter);
            let out = e.mem.alloc(4);
            e.mem.set_u8(out, 0x55);
            let result = e.call(0x00e4_6cf0, &[this, out, 1, 2, 3, 4, 5]);
            assert_eq!(result.u32(), out);
            assert_eq!(e.mem.u8(out), u8::from(answer != 0));
            let asked = calls(&log, SLOT_4);
            assert_eq!(asked.len(), 1);
            assert_eq!(asked[0][0], filter + 0xc);
            assert_eq!(asked[0][2..], [1, 2, 3, 4, 5]);
            assert!(calls(&log, SLOT_0).is_empty());
        }
    }

    #[test]
    fn ov3_without_a_filter_enables_00e46cf0() {
        let mut e = engine();
        let this = owner(&mut e, 0);
        let out = e.mem.alloc(4);
        let result = e.call(0x00e4_6cf0, &[this, out, 1, 2, 3, 4, 5]);
        assert_eq!(result.u32(), out);
        assert_eq!(e.mem.u8(out), 1);
    }

    #[test]
    fn ov4_asks_the_filter_slot_0_00e46d40() {
        for answer in [0u8, 1, 7] {
            let mut e = engine();
            let (filter, log) = filter(&mut e, answer);
            let this = owner(&mut e, filter);
            let out = e.mem.alloc(4);
            e.mem.set_u8(out, 0x55);
            let result = e.call(0x00e4_6d40, &[this, out, 1, 2, 3, 4, 5, 6, 7]);
            assert_eq!(result.u32(), out);
            assert_eq!(e.mem.u8(out), u8::from(answer != 0));
            let asked = calls(&log, SLOT_0);
            assert_eq!(asked.len(), 1);
            assert_eq!(asked[0][0], filter + 0xc);
            assert_eq!(asked[0][2..], [1, 2, 3, 4, 5, 6, 7]);
            assert!(calls(&log, SLOT_4).is_empty());
        }
    }

    #[test]
    fn ov4_without_a_filter_enables_00e46d40() {
        let mut e = engine();
        let this = owner(&mut e, 0);
        let out = e.mem.alloc(4);
        let result = e.call(0x00e4_6d40, &[this, out, 1, 2, 3, 4, 5, 6, 7]);
        assert_eq!(result.u32(), out);
        assert_eq!(e.mem.u8(out), 1);
    }
}
