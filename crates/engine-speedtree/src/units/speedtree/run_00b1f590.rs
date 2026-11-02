//! `SpeedTree/run_00b1f590` (Xbox PDB source unit), subsystem `SpeedTree`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The element allocation (`00b23aa0`, cdecl: count, 0) and the
/// too-long error (`00b0fb80`, no arguments).
const ALLOCATE_ELEMENTS: u32 = 0x00b2_3aa0;
const LENGTH_ERROR: u32 = 0x00b0_fb80;
/// The vector's first, last and end pointers (+0xc, +0x10, +0x14).
const FIRST: u32 = 0x0c;
const LAST: u32 = 0x10;
const END: u32 = 0x14;
/// The largest element count (`0xaaaaaaa`, the code's constant for
/// 0x18-byte elements).
const MAX_COUNT: u32 = 0x0aaa_aaaa;
/// The element size (0x18 bytes).
const ELEMENT_SIZE: u32 = 0x18;

// Translated from 00b1f590 (decompiled, FalloutNV.exe 1.4.0.525)
/// A vector's `_Buy` (thiscall, `RET 4`; unnamed in the engine map): the
/// three pointers cleared; false for a zero count. A count above the
/// maximum gives the length error (`00b0fb80`, which throws in the exe);
/// otherwise the storage for `count` elements of 0x18 bytes is allocated
/// (`00b23aa0`) and the pointers set to it (end at `count * 0x18`). Then
/// true.
pub fn fn_00b1f590(e: &mut Engine, this: Ptr, count: u32) -> bool {
    let base = this.addr();
    e.mem.set_u32(base + FIRST, 0);
    e.mem.set_u32(base + LAST, 0);
    e.mem.set_u32(base + END, 0);
    if count == 0 {
        return false;
    }
    // The code takes the maximum, or 1 when it is 0 (it never is).
    let limit = MAX_COUNT;
    let maximum = if limit > 0 { limit } else { 1 };
    if maximum < count {
        e.call(LENGTH_ERROR, &args![]);
    } else {
        let storage = e.call(ALLOCATE_ELEMENTS, &args![count, 0u32]).u32();
        e.mem.set_u32(base + FIRST, storage);
        let first = e.mem.u32(base + FIRST);
        e.mem.set_u32(base + LAST, first);
        let end = count
            .wrapping_mul(ELEMENT_SIZE)
            .wrapping_add(e.mem.u32(base + FIRST));
        e.mem.set_u32(base + END, end);
    }
    true
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x00b1_f590, fn_00b1f590(Ptr, u32) -> bool)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buys_storage_for_the_count() {
        let mut e = Engine::new();
        e.register(ALLOCATE_ELEMENTS, |_, _| 0x0600_0000u32.into_ret());
        e.register(LENGTH_ERROR, |_, _| Ret::default());
        let this = e.mem.alloc(0x18);
        for offset in [FIRST, LAST, END] {
            e.mem.set_u32(this + offset, 0x1111);
        }
        assert_eq!(e.call(0x00b1_f590, &args![this, 0u32]).u32() & 0xff, 0);
        assert_eq!(e.mem.u32(this + FIRST), 0);
        assert_eq!(e.mem.u32(this + END), 0);
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x00b1_f590, &args![this, 3u32]).u32() & 0xff, 1);
        assert_eq!(e.mem.u32(this + FIRST), 0x0600_0000);
        assert_eq!(e.mem.u32(this + LAST), 0x0600_0000);
        assert_eq!(e.mem.u32(this + END), 0x0600_0048);
        let log = e.call_log.take().unwrap();
        let calls: Vec<_> = log
            .iter()
            .filter(|(a, _)| *a == ALLOCATE_ELEMENTS)
            .collect();
        assert_eq!(calls[0].1, vec![3, 0]);
        // Too many: the length error, the pointers left cleared.
        e.call_log = Some(vec![]);
        assert_eq!(
            e.call(0x00b1_f590, &args![this, MAX_COUNT + 1]).u32() & 0xff,
            1
        );
        let log = e.call_log.take().unwrap();
        assert!(log.iter().any(|(a, _)| *a == LENGTH_ERROR));
        assert!(!log.iter().any(|(a, _)| *a == ALLOCATE_ELEMENTS));
        assert_eq!(e.mem.u32(this + FIRST), 0);
    }
}
