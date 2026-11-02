//! `fallout shared/facegen/run_0064fcd0` (Xbox PDB source unit), subsystem `fallout shared/facegen`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

/// The word at +0x10 of an object (`0044edb0`, `thiscall`).
const WORD_AT_0X10: u32 = 0x0044_edb0;
/// The other object's float `index` (`005dd4e0`, `thiscall`, one word; result in ST0) and
/// this object's store of a float `index` (`0064a4d0`, `thiscall`, two words).
const FLOAT_AT_INDEX_005DD4E0: u32 = 0x005d_d4e0;
const SET_FLOAT_AT_INDEX_0064A4D0: u32 = 0x0064_a4d0;

// Translated from 0064fcd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Unnamed in the engine map: an assignment from `other` (`thiscall`, one word). Only when
/// both objects' virtual slot +4 answer the same: this object's slot +0x10 gets `other`'s
/// slot +0xc (a float, stored as a `float`), slot +0x50 gets `other`'s word +0x10
/// (`0044edb0`) and 1, and each of this object's `+0x10` entries gets `other`'s float of
/// the same index (`005dd4e0`, then `0064a4d0`). Answers `this`.
pub fn fn_0064fcd0(e: &mut Engine, this: Ptr, other: Ptr) -> Ptr {
    let mine = e.vcall(this.addr(), 4, &args![]).u32();
    let theirs = e.vcall(other.addr(), 4, &args![]).u32();
    if mine != theirs {
        return this;
    }
    let value = e.vcall(other.addr(), 0xc, &args![]).f64();
    e.vcall(this.addr(), 0x10, &args![x87_store32(value)]);
    let word = e.call(WORD_AT_0X10, &args![other]).u32();
    e.vcall(this.addr(), 0x50, &args![word, 1u32]);
    let mut index = 0u32;
    while index < e.mem.u32(this.addr() + 0x10) {
        let value = e.call(FLOAT_AT_INDEX_005DD4E0, &args![other, index]).f64();
        e.call(
            SET_FLOAT_AT_INDEX_0064A4D0,
            &args![this, index, x87_store32(value)],
        );
        index += 1;
    }
    this
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![entry!(0x0064_fcd0, fn_0064fcd0(Ptr, Ptr) -> Ptr)]
}

#[cfg(test)]
mod n063_tests {
    use super::*;

    fn calls(e: &Engine, addr: u32) -> Vec<Vec<u32>> {
        e.call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(a, _)| *a == addr)
            .map(|(_, w)| w.clone())
            .collect()
    }

    /// An object whose vtable slots are `base + slot`, with `count` at +0x10.
    fn object(e: &mut Engine, base: u32, count: u32) -> u32 {
        let vtable = e.mem.alloc(0x60);
        for slot in 0..0x18 {
            e.mem.set_u32(vtable + 4 * slot, base + 4 * slot);
        }
        let object = e.mem.alloc(0x20);
        e.mem.set_u32(object, vtable);
        e.mem.set_u32(object + 0x10, count);
        object
    }

    #[test]
    fn assignment_copies_only_between_matching_kinds() {
        let mut e = Engine::new();
        let this = object(&mut e, 0x00e0_0000, 2);
        let other = object(&mut e, 0x00e1_0000, 5);
        for (base, kind) in [(0x00e0_0000u32, 7u32), (0x00e1_0000, 7)] {
            e.register_double(base + 4, move |_, _| kind.into_ret());
        }
        e.register(0x00e1_000c, |_, _| Ret {
            st0: 0.25,
            ..Ret::default()
        });
        for address in [0x00e0_0010u32, 0x00e0_0050, SET_FLOAT_AT_INDEX_0064A4D0] {
            e.register(address, |_, _| Ret::default());
        }
        e.register(WORD_AT_0X10, |e, a| e.mem.u32(a[0] + 0x10).into_ret());
        e.register(FLOAT_AT_INDEX_005DD4E0, |_, a| Ret {
            st0: f64::from(a[1]) + 0.5,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_fcd0, &args![this, other]).u32(), this);
        assert_eq!(calls(&e, 0x00e0_0010), vec![vec![this, 0x3e80_0000]]);
        assert_eq!(calls(&e, 0x00e0_0050), vec![vec![this, 5, 1]]);
        assert_eq!(
            calls(&e, SET_FLOAT_AT_INDEX_0064A4D0),
            vec![vec![this, 0, 0x3f00_0000], vec![this, 1, 0x3fc0_0000]]
        );
        // Different kinds: nothing is copied.
        e.register(0x00e1_0004, |_, _| 8u32.into_ret());
        e.call_log = Some(vec![]);
        assert_eq!(e.call(0x0064_fcd0, &args![this, other]).u32(), this);
        assert!(calls(&e, 0x00e0_0010).is_empty());
        assert!(calls(&e, SET_FLOAT_AT_INDEX_0064A4D0).is_empty());
    }
}
